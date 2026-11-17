#!/usr/bin/env python3
"""Validator's own record-by-record comparison of the original holdout-P20
session with its second-host re-execution (PROTOCOL.md section 13).

Written by the validator from the record schema and
src/cryptanalysis/ecbench/{band,compare,record,spec}.rs at 0e124fb1b207; it
shares no code with second-host/tools/match_records.py, which was not read.

Usage: python3 -I compare_records.py ORIGINAL.jsonl SECOND_HOST.jsonl OUT.json

Checks:
  1. every original record has exactly one partner keyed by
     (arm, workload_id, round, warmup), with the same algorithm seed;
  2. a full leaf-by-leaf diff of every pair, summarised by generalised path
     (list indices of phases replaced by the phase name), so every field that
     differs is named, not only the ones expected to differ;
  3. the fields band.rs (unpriced_counts, calibrated_native, priced_gae,
     priced_s), compare.rs (match_pairs: workload_id, round, algorithm_seed,
     counts()) and the outcome depend on are identical on every pair;
  4. priced_S(w) at w = 0, 0.1, 1 recomputed by an independent
     implementation of the band formula, compared exactly;
  5. arm means and paired ratio sums on both sessions (and on rounds 1-2 of
     the second host, which must equal the original exactly);
  6. [d]G = Q and the curve law re-checked with the validator's own affine
     arithmetic on every verified record of the second host.
"""
import collections
import hashlib
import json
import math
import sys

WEIGHTS = (0.0, 0.1, 1.0)


def load(path):
    with open(path, "rb") as f:
        data = f.read()
    recs = [json.loads(line) for line in data.decode("utf-8").splitlines() if line.strip()]
    return hashlib.sha256(data).hexdigest(), recs


def leaves(x, path=""):
    if isinstance(x, dict):
        if not x:
            yield path, {}
        for k, v in x.items():
            yield from leaves(v, f"{path}.{k}" if path else k)
    elif isinstance(x, list):
        if not x:
            yield path, []
        for i, v in enumerate(x):
            yield from leaves(v, f"{path}[{i}]")
    else:
        yield path, x


def generalise(path, rec):
    # phases[i] -> phases[<name>] so a summary line names the phase
    if path.startswith("phases["):
        i = int(path[len("phases["):path.index("]")])
        name = rec["phases"][i]["name"] if i < len(rec["phases"]) else f"#{i}"
        path = f"phases[{name}]" + path[path.index("]") + 1:]
    return path


def key(r):
    return (r["arm"], r["workload"]["workload_id"], r["round"], r["warmup"])


# --- band.rs, re-implemented -------------------------------------------------
def unpriced_counts(r):
    out = {}
    for name in r["cost"].get("unpriced") or []:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in (r.get("counters") or {}):
            out[name] = r["counters"][name]
            continue
        total, found = 0, False
        for p in r.get("phases") or []:
            nat = p.get("native") or {}
            if name in nat:
                total += nat[name]
                found = True
            elif base in nat:
                total += nat[base]
                found = True
        if not found and base.startswith("solver_") and r.get("solver"):
            out[name] = r["solver"]["ops"]
            continue
        out[name] = total if found else None
    return out


def calibration_pinned(r):
    d = r.get("detail") or {}
    v = d.get("calibration_pinned_units") if isinstance(d, dict) else None
    return list(v) if isinstance(v, list) else []


def priced_s(r, w):
    g = r["cost"].get("total_gae")
    if g is None:
        return None
    counts = unpriced_counts(r)
    missing = any(v is None for v in counts.values())
    if missing and w > 0:
        return None
    if calibration_pinned(r):
        raise SystemExit(f"{r['record_id']}: calibration-pinned record; C != 0 not handled")
    u = sum(v for v in counts.values() if v is not None)
    return (g + w * u) / math.sqrt(r["workload"]["curve"]["r"])


def counts_flag(r):
    return (not r["warmup"]) and r["outcome"]["status"] == "verified"


# --- independent curve arithmetic -------------------------------------------
def inv(a, p):
    return pow(a % p, p - 2, p)


def add(P, Q, a, p):
    if P is None:
        return Q
    if Q is None:
        return P
    (x1, y1), (x2, y2) = P, Q
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if P == Q:
        lam = (3 * x1 * x1 + a) * inv(2 * y1, p) % p
    else:
        lam = (y2 - y1) * inv(x2 - x1, p) % p
    x3 = (lam * lam - x1 - x2) % p
    return (x3, (lam * (x1 - x3) - y1) % p)


def mul(k, P, a, p):
    R = None
    while k:
        if k & 1:
            R = add(R, P, a, p)
        P = add(P, P, a, p)
        k >>= 1
    return R


def is_probable_prime(n):
    if n < 2:
        return False
    for q in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x in (1, n - 1):
            continue
        for _ in range(s - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


def scalar_checks(recs):
    c = recs[0]["workload"]["curve"]
    p = int(c["icv1"].split(":")[1].split("-")[1])
    r = c["r"]
    G = tuple(int(v, 16) for v in c["generator"])
    targets = {}
    for rec in recs:
        targets[rec["workload"]["workload_id"]] = tuple(int(v, 16) for v in rec["workload"]["target"])
    T = next(iter(targets.values()))
    (x1, y1), (x2, y2) = G, T
    a = ((y1 * y1 - x1 ** 3) - (y2 * y2 - x2 ** 3)) * inv(x1 - x2, p) % p
    b = (y1 * y1 - x1 ** 3 - a * x1) % p
    on = lambda P: (P[1] * P[1] - (P[0] ** 3 + a * P[0] + b)) % p == 0
    j = (1728 * 4 * a ** 3 * inv(4 * a ** 3 + 27 * b * b, p)) % p
    out = {
        "p": p, "a": a, "b": b, "r": r, "j_invariant": j,
        "p_prime": is_probable_prime(p), "r_prime": is_probable_prime(r),
        "trace": p + 1 - r,
        "rG_is_identity": mul(r, G, a, p) is None,
        "G_and_all_targets_on_curve": on(G) and all(on(P) for P in targets.values()),
        "targets": len(targets),
        "verified_records_checked": 0, "failures": [],
    }
    for rec in recs:
        if rec["outcome"]["status"] != "verified":
            continue
        d = int(rec["outcome"]["recovered"])
        Q = targets[rec["workload"]["workload_id"]]
        if mul(d, G, a, p) != Q:
            out["failures"].append(rec["record_id"])
        out["verified_records_checked"] += 1
    return out


# --- strict field set the band, compare and outcome depend on ---------------
def strict_view(r):
    phases = [
        {k: v for k, v in ph.items() if k not in ("wall_ns", "work_native")}
        for ph in r.get("phases") or []
    ]
    return {
        "arm": r["arm"], "role": r["role"], "round": r["round"], "warmup": r["warmup"],
        "algorithm_seed": r["algorithm_seed"], "method": r["method"], "workload": r["workload"],
        "outcome.status": r["outcome"]["status"], "outcome.recovered": r["outcome"]["recovered"],
        "outcome.matches_target": r["outcome"]["matches_target"],
        "cost": r["cost"], "boundaries": r["boundaries"], "counters": r["counters"],
        "phases(no wall_ns, no work_native)": phases, "factor_base": r.get("factor_base"),
        "field_ops": r.get("field_ops"), "solver": r.get("solver"), "detail": r.get("detail"),
        "unpriced_counts": unpriced_counts(r),
    }


def arm_stats(recs, rounds):
    by = collections.defaultdict(dict)
    for r in recs:
        if counts_flag(r) and r["round"] in rounds:
            by[r["arm"]][(r["workload"]["workload_id"], r["round"])] = r
    stats = {}
    for arm, m in sorted(by.items()):
        stats[arm] = {"n": len(m)}
        for w in WEIGHTS:
            vals = [priced_s(r, w) for r in m.values()]
            stats[arm][str(w)] = {"mean_s": sum(vals) / len(vals), "sum_s": sum(vals)}
    ratios = {}
    for num, den in (("sel", "inc"), ("sel", "rho-neg"), ("inc", "rho-neg"), ("inc-aa", "inc"),
                     ("rho-neg-aa", "rho-neg"), ("size", "inc"), ("stride", "sel")):
        keys = sorted(set(by[num]) & set(by[den]))
        seeds_equal = all(by[num][k]["algorithm_seed"] == by[den][k]["algorithm_seed"] for k in keys)
        ratios[f"{num}/{den}"] = {"pairs": len(keys), "same_seeds": seeds_equal}
        for w in WEIGHTS:
            sn = sum(priced_s(by[num][k], w) for k in keys)
            sd = sum(priced_s(by[den][k], w) for k in keys)
            ratios[f"{num}/{den}"][str(w)] = sn / sd
    return stats, ratios


def main():
    orig_path, sh_path, out_path = sys.argv[1:4]
    ha, A = load(orig_path)
    hb, B = load(sh_path)
    ia = {key(r): r for r in A}
    ib = {}
    dup_b = 0
    for r in B:
        if key(r) in ib:
            dup_b += 1
        ib[key(r)] = r
    missing = [list(k) for k in ia if k not in ib]
    seed_mismatch = [list(k) for k in ia if k in ib and ia[k]["algorithm_seed"] != ib[k]["algorithm_seed"]]

    diff_counts = collections.Counter()   # generalised path -> pairs where values differ
    only_a = collections.Counter()        # path present in original only
    only_b = collections.Counter()
    same_counts = collections.Counter()
    strict_bad = collections.Counter()
    pairs_identical_strict = 0
    priced_bad = []
    for k, ra in ia.items():
        rb = ib.get(k)
        if rb is None:
            continue
        la = dict(leaves(ra))
        lb = dict(leaves(rb))
        seen_diff, seen_a, seen_b, seen_same = set(), set(), set(), set()
        for pth in set(la) | set(lb):
            g = generalise(pth, ra if pth in la else rb)
            if pth not in lb:
                seen_a.add(g)
            elif pth not in la:
                seen_b.add(g)
            elif la[pth] != lb[pth] or type(la[pth]) is not type(lb[pth]):
                seen_diff.add(g)
            else:
                seen_same.add(g)
        for g in seen_diff:
            diff_counts[g] += 1
        for g in seen_a:
            only_a[g] += 1
        for g in seen_b:
            only_b[g] += 1
        for g in seen_same:
            same_counts[g] += 1
        sa, sb = strict_view(ra), strict_view(rb)
        ok = True
        for f in sa:
            if json.dumps(sa[f], sort_keys=True) != json.dumps(sb[f], sort_keys=True):
                strict_bad[f] += 1
                ok = False
        pairs_identical_strict += ok
        for w in WEIGHTS:
            pa, pb = priced_s(ra, w), priced_s(rb, w)
            if pa != pb:
                priced_bad.append([list(k), w, pa, pb])

    # paths that differ on some pair, grouped at a readable depth
    def depth2(c):
        agg = collections.Counter()
        for g, n in c.items():
            parts = g.split(".")
            agg[".".join(parts[:2])] = max(agg[".".join(parts[:2])], n)
        return dict(sorted(agg.items()))

    uncounted = collections.Counter()
    for r in A + B:
        for name, v in unpriced_counts(r).items():
            if v is None:
                uncounted[(r["arm"], name)] += 1
    pinned = sum(1 for r in A + B if calibration_pinned(r))

    o_stats, o_ratios = arm_stats(A, {1, 2})
    b12_stats, b12_ratios = arm_stats(B, {1, 2})
    b_stats, b_ratios = arm_stats(B, {1, 2, 3, 4, 5})
    out = {
        "schema": "validator.second-host-record-compare/v1",
        "original": {"path": orig_path, "sha256": ha, "records": len(A)},
        "second_host": {"path": sh_path, "sha256": hb, "records": len(B), "duplicate_keys": dup_b},
        "pairing_key": "(arm, workload_id, round, warmup)",
        "original_records_with_partner": len(ia) - len(missing),
        "original_records_without_partner": missing,
        "algorithm_seed_mismatches": seed_mismatch,
        "pairs_identical_on_strict_field_set": pairs_identical_strict,
        "strict_field_set": list(strict_view(A[0]).keys()),
        "strict_field_mismatch_counts": dict(strict_bad),
        "priced_s_mismatches_w0_0.1_1": priced_bad,
        "records_with_uncounted_unpriced_counter": {f"{a}:{n}": c for (a, n), c in uncounted.items()},
        "records_with_calibration_pinned_units": pinned,
        "diff_summary": {
            "differ_on_some_pair_depth2": depth2(diff_counts),
            "present_in_original_only_depth2": depth2(only_a),
            "present_in_second_host_only_depth2": depth2(only_b),
            "differ_full_paths": dict(sorted(diff_counts.items())),
            "original_only_full_paths": dict(sorted(only_a.items())),
            "second_host_only_full_paths": dict(sorted(only_b.items())),
        },
        "band_original_rounds_1_2": {"arms": o_stats, "paired_ratio_sums": o_ratios},
        "band_second_host_rounds_1_2": {"arms": b12_stats, "paired_ratio_sums": b12_ratios},
        "band_second_host_rounds_1_5": {"arms": b_stats, "paired_ratio_sums": b_ratios},
        "second_host_rounds_1_2_equal_original_exactly": (o_stats == b12_stats and o_ratios == b12_ratios),
        "scalar_check_second_host": scalar_checks(B),
        "scalar_check_original": scalar_checks(A),
    }
    with open(out_path, "w") as f:
        json.dump(out, f, indent=1, sort_keys=False)
        f.write("\n")


if __name__ == "__main__":
    main()
