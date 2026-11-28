#!/usr/bin/env python3
"""Validator: read the binary's uncounted per-attempt diagnosis (diag.sh) against the
committed rows, and test the base-point-shift mechanism on the dumped attempt cores.

Usage:
  python3 -I diag_check.py SNAP DIAG_DIR OUT.json

Checks
  D1  every diag run row equals the committed row with the same (arm, p, curve, seed) on
      every key except "diag" and the wall clock (the flag is uncounted);
  D2  per run: one diag entry per counted attempt; the last attempt certified (outcome 3)
      and t determined there (rank == rank_no_t + 1); every earlier attempt failed with t
      undetermined (rank == rank_no_t) or a too-small core;
  D3  the dumped cores (QL_CORE_DUMP, one line per counted attempt, in execution order;
      a dump line carries q, seed, attempt and r but not the arm, so dumps are matched to
      runs by r, seed and the arm order nofb then p0 of the single-thread run).  Per core:
        line rows    = rows with rhs 0 and no t entry (homogeneous: lines, line cycles)
        kappa column = the column with the most non-zeros (every relation holds -kappa)
        P0-type rows = line rows r with sum_{j != kappa} c_j + 4 c_kappa != 0 mod r,
                       i.e. rows not orthogonal to the base-point shift v_deg
                       (every lambda_P + 1, lambda_kappa + 4); a line not through P0 has
                       four points and -kappa (4 - 4 = 0), a line through P0 three (3 - 4)
        v_deg free   = no P0-type row among the line rows
        z columns    = core columns other than kappa and t that occur in no line row
        ab_rank      = the binary's rank of the core's (rhs, t coefficient) pairs; 1 means the
                       inhomogeneous rows carry one direction, so t cannot be determined
      This is the validator's own reading of the cores; the study's dev scripts are not used.
"""
import json
import sys
from collections import defaultdict

STUDY = "research/quartic_diem_p0lines_20261128"
NOFB, P0 = "Q_diem_2lp_sfb_sge_nofb", "Q_diem_2lp_sfb_sge_p0"
WALL = ("wall_ns", "search_wall_s", "wall_s")


def jl(path):
    with open(path) as fh:
        return [json.loads(l) for l in fh if l.strip()]


def core_stats(d):
    r, ct = d["r"], d["ct"]
    rows = d["rows"]
    weight = defaultdict(int)
    for ent, _rhs in rows:
        for c, v in ent:
            if v % r:
                weight[c] += 1
    cand = [c for c in weight if c != ct]
    kap = max(cand, key=lambda c: (weight[c], -c))
    line, dec = [], []
    for ent, rhs in rows:
        has_t = any(c == ct and v % r for c, v in ent)
        (dec if (rhs % r or has_t) else line).append(ent)
    p0type = 0
    in_line = set()
    for ent in line:
        s = 0
        for c, v in ent:
            s += 4 * v if c == kap else v
            if v % r:
                in_line.add(c)
        p0type += (s % r) != 0
    zc = [c for c in weight if c not in in_line and c not in (kap, ct)]
    return {"core_rows": len(rows), "line_rows": len(line), "dec_rows": len(dec),
            "kappa_col_weight": weight[kap], "max_other_weight": max([weight[c] for c in cand if c != kap] or [0]),
            "p0type_line_rows": p0type, "vdeg_free": p0type == 0, "z_cols": len(zc)}


def main():
    snap, dd, outp = sys.argv[1:4]
    st = f"{snap}/{STUDY}"
    committed = {}
    for f in ("raw/run.jsonl", "raw/run131101.jsonl"):
        for r in jl(f"{st}/{f}"):
            if r["kind"] == "run":
                committed[(r["arm"], r["p"], r["curve_seed"], r["seed"])] = r
    out = {"schema": "validator-diag-check/v1", "parts": {}, "attempts": []}
    allruns = []
    for part in ("main", "131101"):
        p_rows = f"{dd}/diag_{part}.jsonl"
        p_cores = f"{dd}/cores_{part}.jsonl"
        try:
            rows = jl(p_rows)
            cores = jl(p_cores)
        except FileNotFoundError:
            out["parts"][part] = {"present": False}
            continue
        runs = [r for r in rows if r["kind"] == "run"]
        d1 = 0
        d1_bad = []
        for r in runs:
            c = committed[(r["arm"], r["p"], r["curve_seed"], r["seed"])]
            keys = (set(r) | set(c)) - set(WALL) - {"diag"}
            diff = sorted(k for k in keys if r.get(k, object()) != c.get(k, object()))
            # every key present on both sides (index with [] semantics)
            missing = sorted(k for k in keys if k not in r or k not in c)
            ok = not diff and not missing
            d1 += ok
            if not ok:
                d1_bad.append({"key": [r["arm"], r["p"], r["curve_seed"], r["seed"]], "diff": diff, "missing": missing})
        # match dumps: group by (r, seed) in file order; split into runs by the arm order nofb, p0
        by = defaultdict(list)
        for d in cores:
            by[(d["r"], d["seed"])].append(d)
        nd = 0
        for r in runs:
            q = by[(r["r"], r["seed"])]
            k = r["attempts"]
            if r["arm"] == NOFB:
                mine = q[:k]
            else:
                kn = committed[(NOFB, r["p"], r["curve_seed"], r["seed"])]["attempts"]
                mine = q[kn:kn + k]
            assert [d["attempt"] for d in mine] == list(range(1, k + 1)), (r["arm"], r["p"], r["curve_seed"], r["seed"])
            assert all(d["q"] == r["p"] for d in mine)
            diag = r["diag"]
            assert len(diag) == k
            for i, (dg, d) in enumerate(zip(diag, mine)):
                cs = core_stats(d)
                assert cs["core_rows"] == dg["core_rows"]
                e = {"arm": r["arm"][17:], "p": r["p"], "curve_seed": r["curve_seed"], "seed": r["seed"],
                     "attempt": i + 1, "of": k, "outcome": dg["outcome"], "rank": dg["rank"],
                     "rank_no_t": dg["rank_no_t"], "core_cols": dg["core_cols"], "ab_rank": dg["ab_rank"],
                     "t_determined": dg["rank"] == dg["rank_no_t"] + 1, **cs}
                out["attempts"].append(e)
                nd += 1
            allruns.append(r)
        out["parts"][part] = {"present": True, "runs": len(runs), "D1_rows_equal_committed": d1, "D1_bad": d1_bad,
                              "core_dumps": len(cores), "core_dumps_matched": nd}
    A = out["attempts"]
    summ = {}
    for arm in ("e_nofb", "e_p0"):
        fail = [a for a in A if a["arm"] == arm and a["attempt"] < a["of"]]
        last = [a for a in A if a["arm"] == arm and a["attempt"] == a["of"]]
        per_p = defaultdict(lambda: defaultdict(int))
        for a in fail:
            per_p[a["p"]]["failed_attempts"] += 1
            per_p[a["p"]]["vdeg_free"] += a["vdeg_free"]
            per_p[a["p"]]["t_undetermined"] += not a["t_determined"]
            per_p[a["p"]]["with_z_cols"] += a["z_cols"] > 0
            per_p[a["p"]]["ab_rank_1"] += a["ab_rank"] == 1
        summ[arm] = {
            "runs": sum(1 for a in A if a["arm"] == arm and a["attempt"] == 1),
            "failed_attempts": len(fail),
            "failed_t_undetermined": sum(1 for a in fail if not a["t_determined"]),
            "failed_vdeg_free": sum(1 for a in fail if a["vdeg_free"]),
            "failed_with_z_cols": sum(1 for a in fail if a["z_cols"] > 0),
            "failed_vdeg_free_or_z": sum(1 for a in fail if a["vdeg_free"] or a["z_cols"] > 0),
            "failed_ab_rank_1": sum(1 for a in fail if a["ab_rank"] == 1),
            "failed_neither_vdeg_nor_z": [{k: a[k] for k in ("p", "curve_seed", "seed", "attempt", "dec_rows", "ab_rank",
                                                             "p0type_line_rows", "z_cols")}
                                          for a in fail if not a["vdeg_free"] and a["z_cols"] == 0],
            "last_attempts_certified": sum(1 for a in last if a["outcome"] == 3),
            "last_attempts_t_determined": sum(1 for a in last if a["t_determined"]),
            "all_attempts": sum(1 for a in A if a["arm"] == arm),
            "all_attempts_vdeg_free": sum(1 for a in A if a["arm"] == arm and a["vdeg_free"]),
            "kappa_heuristic_margin_min": min((a["kappa_col_weight"] - a["max_other_weight"] for a in A if a["arm"] == arm), default=None),
            "failed_per_prime": {str(p): dict(v) for p, v in sorted(per_p.items())},
        }
    out["summary"] = summ
    with open(outp, "w") as fh:
        json.dump(out, fh, indent=1)
        fh.write("\n")
    print(json.dumps({"parts": {k: {kk: vv for kk, vv in v.items() if kk != "D1_bad"} for k, v in out["parts"].items()},
                      "summary": summ}, indent=1))


if __name__ == "__main__":
    main()
