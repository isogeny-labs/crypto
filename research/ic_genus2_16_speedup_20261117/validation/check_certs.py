#!/usr/bin/env python3
"""Certificate re-check of the g16f re-runs (P4_followup/certs/*_cert.jsonl)
with the validator's own Cantor arithmetic (hyperell.py), independent of the
solver.

For every run record (fields read with [], lesson L-99ee952350b2):
  - cert.f equals the instance record's f (same file) and the committed
    instance record for (p, g, curve_seed); deg f = 2g+1, monic;
  - G = (G_u, G_v), T = (T_u, T_v) are valid reduced Mumford pairs on
    y^2 = f(x) over F_p (u monic as given, deg u <= g, deg v < deg u, u | v^2 - f);
  - if recovered_t is not null: G != 0, [r]G = 0 (r prime, so ord G = r),
    [t]G = T, 0 <= t < r;
  - ok/certified true  =>  recovered_t not null and passes;
    exhausted true     =>  recovered_t null;
    certified flag == the validator's own verdict;
  - per (g, p, curve_seed, seed): all arms share G and T and recover the same t.
Then an identity diff of every re-run record against the committed record with
the same (g, p, curve_seed, seed, arm), all fields except wall_ns, cert and
recovered_t (instance records: all fields).

usage: python3 check_certs.py
"""
import glob
import json
import os
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from hyperell import HCurve, is_prime, trim  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
CERT_GLOB = os.path.join(ROOT, "P4_followup/certs/*_cert.jsonl")
COMMITTED = ["P1/main/*.jsonl", "P1/supp/*.jsonl", "P2/crossover/*.jsonl", "P2/crossover_supp/*.jsonl",
             "P4_followup/g8decay/*.jsonl"]
NEW_FIELDS = {"cert", "recovered_t"}


def load(paths):
    runs, inst = {}, {}
    dups = []
    for fn in sorted(paths):
        with open(fn) as fh:
            for line in fh:
                if not line.strip():
                    continue
                d = json.loads(line)
                if d["kind"] == "run":
                    k = (d["g"], d["p"], d["curve_seed"], d["seed"], d["arm"])
                    if k in runs:
                        dups.append((k, fn))
                    runs[k] = (fn, d)
                elif d["kind"] == "instance":
                    inst[(d["g"], d["p"], d["curve_seed"])] = (fn, d)
    return runs, inst, dups


def main():
    files = sorted(glob.glob(CERT_GLOB))
    runs, inst, dups = load(files)
    print(f"cert files: {len(files)}; run records: {len(runs)}; instance records: {len(inst)}; duplicate run keys: {dups}")
    crun, cinst, cdups = load([f for g in COMMITTED for f in glob.glob(os.path.join(ROOT, g))])
    print(f"committed comparison set: {len(crun)} run records, {len(cinst)} instances; duplicate keys: {len(cdups)}")

    bad = []
    counts = Counter()
    curves = {}
    rG_cache, tG_cache = {}, {}
    per_target = defaultdict(set)
    for k, (fn, d) in sorted(runs.items()):
        g, p, cs, sd, arm = k
        counts["records"] += 1
        counts[f"arm:{arm}"] += 1
        ik = (g, p, cs)
        if ik not in inst:
            bad.append((k, "no instance record in the cert file set"))
            continue
        f_inst = inst[ik][1]["f"]
        cert = d["cert"]
        f = cert["f"]
        if f != f_inst:
            bad.append((k, "cert.f != instance f"))
        if ik in cinst and cinst[ik][1]["f"] != f:
            bad.append((k, "cert.f != committed instance f"))
        if len(f) != 2 * g + 2 or f[-1] != 1:
            bad.append((k, "f not monic of degree 2g+1"))
        if d["r"] != inst[ik][1]["r"]:
            bad.append((k, "record r != instance r"))
        if ik not in curves:
            curves[ik] = HCurve(f, p)
        C = curves[ik]
        r = d["r"]
        Gu, Gv, Tu, Tv = (list(cert[x]) for x in ("G_u", "G_v", "T_u", "T_v"))
        for name, u in (("G_u", Gu), ("T_u", Tu)):
            if not u or u[-1] != 1:
                bad.append((k, f"{name} not monic with leading 1"))
        G = (trim([x % p for x in Gu]), trim([x % p for x in Gv]))
        T = (trim([x % p for x in Tu]), trim([x % p for x in Tv]))
        if any(x != x % p for x in Gu + Gv + Tu + Tv):
            bad.append((k, "coefficient outside [0, p)"))
        vG, vT = C.is_valid(G), C.is_valid(T)
        if not (vG and vT):
            bad.append((k, f"invalid Mumford pair G={vG} T={vT}"))
            continue
        per_target[(g, p, cs, sd)].add((json.dumps(G), json.dumps(T), d["recovered_t"]))
        t = d["recovered_t"]
        if arm == "R_rho_neg":
            ex = None  # rho records carry no exhausted field (checked below)
            if "exhausted" in d:
                bad.append((k, "rho record has an exhausted field"))
        else:
            ex = d["exhausted"]
        passed = None
        if t is not None:
            counts["with_t"] += 1
            gk = (ik, json.dumps(G))
            if gk not in rG_cache:
                rG_cache[gk] = (not C.is_zero(G), C.is_zero(C.mul(r, G)), is_prime(r))
            nz, rz, rp = rG_cache[gk]
            tk = (gk, t)
            if tk not in tG_cache:
                tG_cache[tk] = C.mul(t, G)
            tg_ok = tG_cache[tk] == T
            in_range = isinstance(t, int) and 0 <= t < r
            passed = nz and rz and rp and tg_ok and in_range
            if passed:
                counts["cert_pass"] += 1
            else:
                counts["cert_fail"] += 1
                bad.append((k, f"certificate FAIL: G!=0 {nz}, [r]G=0 {rz}, r prime {rp}, [t]G=T {tg_ok}, 0<=t<r {in_range}"))
        else:
            counts["t_null"] += 1
        if (d["ok"] or d["certified"]) and not passed:
            bad.append((k, "ok/certified true without a passing recovered_t"))
        if d["certified"] != bool(passed):
            bad.append((k, f"certified flag {d['certified']} != validator verdict {bool(passed)}"))
        if ex:
            counts["exhausted"] += 1
            if t is not None:
                bad.append((k, "exhausted record has recovered_t"))
        if ex is False and t is None:
            bad.append((k, "non-exhausted IC record without recovered_t"))
        if d["ok"]:
            counts["ok_true"] += 1
        if d["certified"]:
            counts["certified_true"] += 1

    # one target per (curve, seed): G, T, t shared by every arm
    split = 0
    for key, s in per_target.items():
        gts = {(a, b) for a, b, _ in s}
        ts = {t for _, _, t in s if t is not None}
        if len(gts) != 1 or len(ts) > 1:
            split += 1
            bad.append((key, f"arms disagree on G/T ({len(gts)}) or t ({sorted(ts)})"))
    seeds_same_target = 0
    by_curve = defaultdict(dict)
    for (g, p, cs, sd), s in per_target.items():
        by_curve[(g, p, cs)][sd] = {(a, b) for a, b, _ in s}
    for ik, sds in by_curve.items():
        vals = list(sds.values())
        if len(vals) > 1 and all(v == vals[0] for v in vals):
            seeds_same_target += 1

    # identity diff against the committed records
    n_ov = n_fields = 0
    diffs = []
    new_only = []
    for k, (fn, d) in sorted(runs.items()):
        if k not in crun:
            new_only.append(k)
            continue
        c = crun[k][1]
        n_ov += 1
        keys = (set(d) | set(c)) - {"wall_ns"} - NEW_FIELDS
        for x in sorted(keys):
            n_fields += 1
            if x not in d or x not in c:
                diffs.append((k, x, "missing on one side"))
            elif d[x] != c[x]:
                diffs.append((k, x, d[x], c[x]))
        extra = set(c) & NEW_FIELDS
        if extra:
            diffs.append((k, "committed record already has", sorted(extra)))
    inst_diff = []
    for ik, (fn, d) in inst.items():
        if ik in cinst and cinst[ik][1] != d:
            inst_diff.append(ik)
    missing_from_rerun = sorted(set(crun) - set(runs))
    by_src = Counter(crun[k][0].split(os.sep)[-3] + "/" + crun[k][0].split(os.sep)[-2] for k in runs if k in crun)

    print("\ncounts:", dict(sorted(counts.items())))
    print(f"(curve, seed) targets: {len(per_target)}; with arms disagreeing on G/T/t: {split}; "
          f"curves whose two seeds share one target (G, T): {seeds_same_target}/{len(by_curve)}")
    print(f"identity: {n_ov} overlapping run records, {n_fields} fields compared "
          f"(wall_ns, cert, recovered_t excluded); differences: {len(diffs)}; by committed source: {dict(by_src)}")
    print(f"instance records differing from committed: {inst_diff}")
    print(f"re-run records with no committed counterpart: {len(new_only)} {sorted({k[:3] for k in new_only})}")
    print(f"committed run records not re-run: {len(missing_from_rerun)} {sorted({k[:3] for k in missing_from_rerun})}")
    for x in diffs[:40]:
        print("DIFF", x)
    for b in bad[:60]:
        print("BAD", b)
    ok = not bad and not diffs and not inst_diff
    print("CERT_CHECK", "PASS" if ok else f"FAIL ({len(bad)} certificate issues, {len(diffs)} identity diffs)")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
