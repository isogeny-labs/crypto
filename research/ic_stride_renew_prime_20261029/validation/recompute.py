#!/usr/bin/env python3
"""Validator's independent recomputation for XP-20261029-stride-renew-prime.

Stdlib only; shares no code with ecbench.  Reads one session's records.jsonl
and plan.json and recomputes, from the raw records:

  * completeness per arm (measured, verified, censored, matches_target);
  * pairing (workload, round) across arms and equality of algorithm seeds;
  * the section 5a priced cost S(w) = (G - C + w (U + n_C)) / sqrt(r) with
    C = n_C = 0 (checked: no record names a pinned calibration unit), U the
    counts behind the record's `cost.unpriced` names: a top-level counter of
    that name, else the sum over phases of native[name] or native[name
    without "_uncharged"] (the frozen definition of the band);
  * paired ratios of totals sum S_b / sum S_a, pooled and per curve, at
    w = 0, 0.1, 1, with the validator's own cluster bootstrap over targets
    (4000 resamples, seed 29102026): one-stage over targets, one-stage
    stratified by curve, and two-stage (targets, then pairs inside each
    target, the scheme of ecbench's cluster bootstrap), with ten further
    seeds for the walk/srenew upper bound;
  * A/A exactness per pair (total GAE, U, every phase's adds/doubles/gae
    and native counts);
  * band ranks per curve (mean S per arm, order at every weight);
  * that srenew and walk differ only in `targets` (method params);
  * stride-renew accounting: scalar_mults = 4 + 2*stride_restarts, adds of
    the relations phase cover stride steps + renewals, phase gae = adds +
    doubles, total_gae = sum of phase gae, renewals = targets with relations;
  * an independent elliptic-curve check: curve coefficients derived from
    plan.json points, p and r prime, every point on the curve, [r]G = O,
    r alone in the Hasse interval (so #E = r), j-invariant, and [d]G = Q in
    affine coordinates for every measured record of every arm.

Usage: python3 -I recompute.py <session_dir> <label>   (prints JSON)
"""
import json
import math
import random
import sys
from collections import defaultdict

WEIGHTS = (0.0, 0.1, 1.0)
PAIRS = [("walk", "srenew"), ("walk", "walk-aa"), ("rho-neg", "rho-neg-aa"),
         ("rho-neg", "walk"), ("rho-neg", "srenew")]
SEED = 29102026
RESAMPLES = 4000


# ---------------------------------------------------------------- arithmetic
def is_prime(n):
    if n < 2:
        return False
    small = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for q in small:
        if n % q == 0:
            return n == q
    d, s = n - 1, 0
    while d % 2 == 0:
        d //= 2
        s += 1
    for a in small:
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


class Curve:
    def __init__(self, p, a, b):
        self.p, self.a, self.b = p, a, b

    def on(self, P):
        if P is None:
            return True
        x, y = P
        return (y * y - (x * x * x + self.a * x + self.b)) % self.p == 0

    def add(self, P, Q):
        p = self.p
        if P is None:
            return Q
        if Q is None:
            return P
        x1, y1 = P
        x2, y2 = Q
        if x1 == x2:
            if (y1 + y2) % p == 0:
                return None
            lam = (3 * x1 * x1 + self.a) * pow(2 * y1, -1, p) % p
        else:
            lam = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (lam * lam - x1 - x2) % p
        return (x3, (lam * (x1 - x3) - y1) % p)

    def mul(self, k, P):
        R = None
        A = P
        while k:
            if k & 1:
                R = self.add(R, A)
            A = self.add(A, A)
            k >>= 1
        return R


def pt(v):
    return (int(v[0], 16), int(v[1], 16))


# ---------------------------------------------------------------- records
def unpriced_count(rec):
    total = 0
    detail = {}
    for name in rec["cost"].get("unpriced", []):
        base = name[:-len("_uncharged")] if name.endswith("_uncharged") else name
        if name in rec.get("counters", {}):
            v = rec["counters"][name]
            detail[name] = v
            total += v
            continue
        found = False
        sub = 0
        for ph in rec.get("phases", []):
            nat = ph.get("native", {}) or {}
            if name in nat:
                sub += nat[name]
                found = True
            elif base in nat:
                sub += nat[base]
                found = True
        if not found:
            raise SystemExit(f"{rec['record_id']}: unpriced {name} has no count")
        detail[name] = sub
        total += sub
    return total, detail


def phase(rec, name):
    for ph in rec["phases"]:
        if ph["name"] == name:
            return ph
    return None


def main():
    sdir, label = sys.argv[1], sys.argv[2]
    plan = json.load(open(f"{sdir}/plan.json"))
    recs = [json.loads(l) for l in open(f"{sdir}/records.jsonl")]
    out = {"label": label, "session_dir": sdir, "records": len(recs)}
    out["session_ids"] = sorted({r["session_id"] for r in recs})
    out["binary_sha256"] = sorted({r.get("binary_sha256") for r in recs})
    out["isolation_levels"] = dict(sorted(
        {lvl: sum(1 for r in recs if r["isolation"]["level"] == lvl)
         for lvl in {r["isolation"]["level"] for r in recs}}.items()))

    # ---- curves from plan.json, independent EC checks
    curves = {}
    for wl in plan["workloads"]:
        c = wl["curve"]
        slug = c["slug"]
        icv = c["icv1"].split(":")
        p = int(icv[1].split("-")[1])
        t = int(icv[2])
        r = int(c["r"])
        cur = curves.setdefault(slug, {"p": p, "t": t, "r": r, "G": pt(c["generator"]),
                                       "targets": {}, "icv1": c["icv1"]})
        cur["targets"][wl["workload_id"]] = (wl["target_index"], pt(wl["target"]))
    curve_checks = {}
    ec = {}
    for slug, c in curves.items():
        p, r, G = c["p"], c["r"], c["G"]
        pts = [G] + [q for _, q in c["targets"].values()]
        (x1, y1), (x2, y2) = pts[0], next(q for q in pts if q[0] != pts[0][0])
        a = ((y1 * y1 - x1 ** 3) - (y2 * y2 - x2 ** 3)) * pow(x1 - x2, -1, p) % p
        b = (y1 * y1 - x1 ** 3 - a * x1) % p
        E = Curve(p, a, b)
        ec[slug] = E
        disc = (4 * a ** 3 + 27 * b ** 2) % p
        j = 1728 * 4 * a ** 3 * pow(disc, -1, p) % p if disc else None
        lo, hi = p + 1 - 2 * math.isqrt(p) - 2, p + 1 + 2 * math.isqrt(p) + 2
        multiples = [m for m in range(r * max(1, lo // r), hi + 1, r) if lo <= m <= hi]
        curve_checks[slug] = {
            "p": p, "a": a, "b": b, "r": r, "trace": c["t"],
            "r_equals_p_plus_1_minus_t": r == p + 1 - c["t"],
            "p_prime": is_prime(p), "r_prime": is_prime(r),
            "nonsingular": disc != 0, "j_invariant": j,
            "j_not_0_or_1728": j not in (0, 1728 % p),
            "all_points_on_curve": all(E.on(q) for q in pts),
            "rG_is_identity": E.mul(r, G) is None,
            "r_only_multiple_in_hasse_interval": multiples == [r],
            "targets": len(c["targets"]),
        }

    # ---- completeness, pairing, priced S
    out["plan"] = {"workloads": len(plan["workloads"]), "executions": len(plan["executions"]),
                   "arms": plan["arms"]}
    by_arm = defaultdict(list)
    for rec in recs:
        by_arm[rec["arm"]].append(rec)
    comp = {}
    meas = {}
    dg_fail = []
    dg_checked = 0
    calib = 0
    for arm, rs in sorted(by_arm.items()):
        m = [x for x in rs if not x["warmup"]]
        st = defaultdict(int)
        for x in m:
            st[x["outcome"]["status"]] += 1
        comp[arm] = {
            "records": len(rs), "warmup": len(rs) - len(m), "measured": len(m),
            "statuses": dict(st),
            "matches_target_true": sum(1 for x in m if x["outcome"].get("matches_target") is True),
            "censored": sum(1 for x in m if x["outcome"]["status"] != "verified"
                            or x["cost"].get("total_gae") is None),
        }
        for x in m:
            if x.get("detail") and x["detail"].get("calibration_pinned_units"):
                calib += 1
            wl = x["workload"]
            slug = wl["curve"]["slug"]
            E = ec[slug]
            G = pt(wl["curve"]["generator"])
            Q = pt(wl["target"])
            d = int(x["outcome"]["recovered"])
            dg_checked += 1
            if E.mul(d, G) != Q or Q != curves[slug]["targets"][wl["workload_id"]][1]:
                dg_fail.append(x["record_id"])
            G_ = float(x["cost"]["total_gae"])
            U, _ = unpriced_count(x)
            r = int(wl["curve"]["r"])
            key = (wl["workload_id"], x["round"])
            meas.setdefault(arm, {})[key] = {
                "G": G_, "U": U, "r": r, "slug": slug, "seed": x["algorithm_seed"],
                "s_rec": x["cost"]["s"], "rec": x,
            }
    out["completeness"] = comp
    out["records_with_pinned_calibration"] = calib
    out["dG_eq_Q"] = {"checked": dg_checked, "failures": dg_fail}
    out["curves"] = curve_checks

    # record's own s equals G / sqrt(r)
    s_dev = max(abs(v["s_rec"] - v["G"] / math.sqrt(v["r"])) / (v["G"] / math.sqrt(v["r"]))
                for a in meas.values() for v in a.values())
    out["record_s_vs_G_over_sqrt_r_max_rel_dev"] = s_dev

    def S(v, w):
        return (v["G"] + w * v["U"]) / math.sqrt(v["r"])

    # pairing: same keys and same algorithm seed across arms
    keys = set(next(iter(meas.values())).keys())
    same_keys = all(set(a.keys()) == keys for a in meas.values())
    seed_mismatch = sum(1 for k in keys if len({meas[a][k]["seed"] for a in meas}) != 1)
    out["pairing"] = {"pairs_per_arm": len(keys), "same_keys_all_arms": same_keys,
                      "algorithm_seed_mismatches": seed_mismatch,
                      "workloads": len({k[0] for k in keys}),
                      "rounds": sorted({k[1] for k in keys})}

    # only `targets` differs between srenew and walk
    diffs = set()
    for k in keys:
        pa = meas["walk"][k]["rec"]["method"]["params"]
        pb = meas["srenew"][k]["rec"]["method"]["params"]
        for kk in set(pa) | set(pb):
            if pa.get(kk) != pb.get(kk):
                diffs.add((kk, pa.get(kk), pb.get(kk)))
        if meas["walk"][k]["rec"]["method"]["id"] != meas["srenew"][k]["rec"]["method"]["id"]:
            diffs.add(("method.id",))
    out["walk_vs_srenew_param_differences"] = sorted(map(list, diffs))
    aa_params_equal = all(meas["walk"][k]["rec"]["method"]["params"]
                          == meas["walk-aa"][k]["rec"]["method"]["params"] for k in keys)
    out["walk_aa_params_equal_walk"] = aa_params_equal

    # ---- ratios and bootstrap
    slug_of = {k[0]: meas["walk"][k]["slug"] for k in keys}

    def cluster_sums(a, b, w, filt=None):
        cs = defaultdict(lambda: [0.0, 0.0])
        for k in keys:
            if filt and slug_of[k[0]] != filt:
                continue
            cs[k[0]][0] += S(meas[a][k], w)
            cs[k[0]][1] += S(meas[b][k], w)
        return cs

    def boot(cs, stratified):
        rng = random.Random(SEED)
        ids = sorted(cs)
        strata = defaultdict(list)
        for i in ids:
            strata[slug_of[i]].append(i)
        vals = []
        for _ in range(RESAMPLES):
            sa = sb = 0.0
            if stratified:
                for _, members in sorted(strata.items()):
                    for _ in range(len(members)):
                        c = cs[members[rng.randrange(len(members))]]
                        sa += c[0]
                        sb += c[1]
            else:
                for _ in range(len(ids)):
                    c = cs[ids[rng.randrange(len(ids))]]
                    sa += c[0]
                    sb += c[1]
            vals.append(sb / sa)
        vals.sort()
        return [vals[int(0.025 * RESAMPLES)], vals[int(0.975 * RESAMPLES) - 1]]

    def pairs_by_cluster(a, b, w):
        pc = defaultdict(list)
        for k in sorted(keys):
            pc[k[0]].append((S(meas[a][k], w), S(meas[b][k], w)))
        return pc

    def boot_two_stage(pc, seed):
        # Two-stage: draw workloads with replacement, then pairs with
        # replacement inside each drawn workload (the scheme ecbench's
        # cluster bootstrap uses; the RNG and code are the validator's own).
        rng = random.Random(seed)
        ids = sorted(pc)
        vals = []
        for _ in range(RESAMPLES):
            sa = sb = 0.0
            for _ in range(len(ids)):
                members = pc[ids[rng.randrange(len(ids))]]
                for _ in range(len(members)):
                    x, y = members[rng.randrange(len(members))]
                    sa += x
                    sb += y
            vals.append(sb / sa)
        vals.sort()
        return [vals[int(0.025 * RESAMPLES)], vals[int(0.975 * RESAMPLES) - 1]]

    ratios = {}
    for a, b in PAIRS:
        name = f"{a}__{b}"
        ratios[name] = {}
        for w in WEIGHTS:
            cs = cluster_sums(a, b, w)
            tot_a = sum(c[0] for c in cs.values())
            tot_b = sum(c[1] for c in cs.values())
            entry = {"ratio_b_over_a": tot_b / tot_a,
                     "mean_s_a": tot_a / len(keys), "mean_s_b": tot_b / len(keys)}
            entry["ci95_cluster_bootstrap"] = boot(cs, False)
            entry["ci95_cluster_bootstrap_stratified"] = boot(cs, True)
            entry["ci95_two_stage_bootstrap"] = boot_two_stage(pairs_by_cluster(a, b, w), SEED)
            if (a, b) == ("walk", "srenew"):
                # seed sensitivity of the decision-bearing upper bound
                ups = [boot_two_stage(pairs_by_cluster(a, b, w), SEED + i)[1] for i in range(1, 11)]
                entry["two_stage_upper_bound_over_10_more_seeds"] = {
                    "min": min(ups), "max": max(ups), "all_below_1": all(u < 1 for u in ups)}
            per_curve = {}
            for slug in sorted(curves):
                csc = cluster_sums(a, b, w, slug)
                per_curve[slug] = sum(c[1] for c in csc.values()) / sum(c[0] for c in csc.values())
            entry["per_curve"] = per_curve
            ratios[name][str(w)] = entry
    out["ratios"] = ratios

    # ---- A/A exactness
    def same(x, y):
        if x["cost"]["total_gae"] != y["cost"]["total_gae"]:
            return False
        if unpriced_count(x)[1] != unpriced_count(y)[1]:
            return False
        px = [(p["name"], p["adds"], p["doubles"], p["scalar_mults"], p["gae"], p.get("native"))
              for p in x["phases"]]
        py = [(p["name"], p["adds"], p["doubles"], p["scalar_mults"], p["gae"], p.get("native"))
              for p in y["phases"]]
        return px == py and x["outcome"]["recovered"] == y["outcome"]["recovered"]

    out["aa_exact"] = {
        "walk_aa_vs_walk_pairs_identical": sum(1 for k in keys if same(meas["walk"][k]["rec"], meas["walk-aa"][k]["rec"])),
        "rho_aa_vs_rho_pairs_identical": sum(1 for k in keys if same(meas["rho-neg"][k]["rec"], meas["rho-neg-aa"][k]["rec"])),
        "pairs": len(keys),
    }

    # ---- band ranks per curve: mean S per arm per weight
    band = {}
    for slug in sorted(curves):
        band[slug] = {}
        for arm in sorted(meas):
            vals = [v for k, v in meas[arm].items() if v["slug"] == slug]
            band[slug][arm] = {str(w): sum(S(v, w) for v in vals) / len(vals) for w in WEIGHTS}
        band[slug]["srenew_lt_walk_every_weight"] = all(
            band[slug]["srenew"][str(w)] < band[slug]["walk"][str(w)] for w in WEIGHTS)
        band[slug]["rho_lt_srenew_every_weight"] = all(
            band[slug]["rho-neg"][str(w)] < band[slug]["srenew"][str(w)] for w in WEIGHTS)
    out["band_mean_s"] = band

    # ---- stride-renew accounting and walk accounting
    acc = {"records": 0, "scalar_mults_eq_4_plus_2_restarts": 0,
           "rel_adds_ge_steps_plus_renewals_plus_restarts_plus_2": 0,
           "phase_gae_eq_adds_plus_doubles": 0, "total_gae_eq_sum_phase_gae": 0,
           "renewals_ge_1_and_le_relations": 0, "missing_stride_counters": 0}
    sums = defaultdict(float)
    for k in keys:
        x = meas["srenew"][k]["rec"]
        y = meas["walk"][k]["rec"]
        acc["records"] += 1
        rel = phase(x, "relations")
        nat = rel["native"]
        if not all(c in nat for c in ("stride_steps", "stride_restarts", "stride_renewals")):
            acc["missing_stride_counters"] += 1
            continue
        if rel["scalar_mults"] == 4 + 2 * nat["stride_restarts"]:
            acc["scalar_mults_eq_4_plus_2_restarts"] += 1
        if rel["adds"] >= nat["stride_steps"] + nat["stride_renewals"] + nat["stride_restarts"] + 2:
            acc["rel_adds_ge_steps_plus_renewals_plus_restarts_plus_2"] += 1
        if all(abs(p["gae"] - (p["adds"] + p["doubles"])) < 1e-9 for p in x["phases"]) and \
           all(abs(p["gae"] - (p["adds"] + p["doubles"])) < 1e-9 for p in y["phases"]):
            acc["phase_gae_eq_adds_plus_doubles"] += 1
        if abs(sum(p["gae"] for p in x["phases"]) - x["cost"]["total_gae"]) < 1e-9 and \
           abs(sum(p["gae"] for p in y["phases"]) - y["cost"]["total_gae"]) < 1e-9:
            acc["total_gae_eq_sum_phase_gae"] += 1
        if 1 <= nat["stride_renewals"] <= nat["relations"]:
            acc["renewals_ge_1_and_le_relations"] += 1
        ry = phase(y, "relations")
        for tag, rec_, ph in (("srenew", x, rel), ("walk", y, ry)):
            sums[f"{tag}.relations.adds"] += ph["adds"]
            sums[f"{tag}.relations.doubles"] += ph["doubles"]
            sums[f"{tag}.relations.scalar_mults"] += ph["scalar_mults"]
            sums[f"{tag}.relations.trials"] += ph["native"]["trials"]
            sums[f"{tag}.relations.relations"] += ph["native"]["relations"]
            sums[f"{tag}.total_gae"] += rec_["cost"]["total_gae"]
            for pn in ("factor_base", "oracle_setup", "linear_algebra", "verify"):
                pp = phase(rec_, pn)
                sums[f"{tag}.{pn}.gae"] += pp["gae"] if pp else 0
        sums["srenew.stride_steps"] += nat["stride_steps"]
        sums["srenew.stride_renewals"] += nat["stride_renewals"]
        sums["srenew.stride_restarts"] += nat["stride_restarts"]
        sums["walk.walk_steps"] += ry["native"].get("walk_steps", 0)
        sums["walk.walk_pool_ops"] += ry["native"].get("walk_pool_ops", 0)
        sums["walk.walk_jumps"] += ry["native"].get("walk_jumps", 0)
    acc["pairs_oracle_setup_gae_equal"] = sum(
        1 for k in keys if phase(meas["srenew"][k]["rec"], "oracle_setup")["gae"]
        == phase(meas["walk"][k]["rec"], "oracle_setup")["gae"])
    acc["pairs_factor_base_native_equal"] = sum(
        1 for k in keys if phase(meas["srenew"][k]["rec"], "factor_base")["native"]
        == phase(meas["walk"][k]["rec"], "factor_base")["native"])
    out["stride_renew_accounting"] = acc
    out["per_pair_means"] = {k: v / len(keys) for k, v in sorted(sums.items())}
    print(json.dumps(out, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
