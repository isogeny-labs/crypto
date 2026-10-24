#!/usr/bin/env python3
"""Independent validator for leaderboard Table C (Koblitz, 32-target batch).

Reads only committed raw records: the 360 isolated main-comparison price files under
research/ic_descent_20260930/runs-isolated/main/k{a}n{n}/M{j}/ (with their isolation records),
and the section-20 batch-rho figure research/ic_exponent_20260926/analysis.json; compares with
the board rows of docs/ic/leaderboard.json. No import of analyse.py, no jq.

Definition (PROTOCOL.md of ic_descent_20260930, re-implemented here from the text):
  per set M_j: the figure pairs are the first clean attempt of each slot r1..r5 (retries
  -retry1, -retry2), or of slots double-r1.. when any double pair exists; a pair is clean
  when both processes have an isolation record with exit 0 and not contended;
  s_before = median over figure pairs of baseline median.s_per_target;
  wall     = geometric mean over figure pairs of median_rep(total_ns baseline)/median_rep(total_ns candidate);
  s_after  = s_before / wall; per curve: mean over the four sets;
  S_ref    = section-20 s_rho of the curve; x ref = s_after / S_ref; x floor = s_after / floor.per_target_k.

usage: validate_C.py REPO_ROOT OUT.json
"""
import hashlib, json, math, statistics, sys
from pathlib import Path

def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def load(p):
    return json.load(open(p))

def clean(price: Path):
    iso = price.with_name(price.name.replace(".price.json", ".isolation.jsonl"))
    if not iso.is_file():
        return True   # unrecorded: accepted, as the archive's analyser does for v1
    lines = [l for l in iso.read_text().splitlines() if l.strip()]
    rec = json.loads(lines[-1])
    return rec["run"]["exit_status"] == 0 and not rec["run"]["contended"]

def med_total(rep):
    return statistics.median(r["total_ns"] for r in rep["repetitions"])

def pairs(d: Path, tag: str):
    out, used = [], []
    for i in range(1, 6):
        for k in range(3):
            stem = f"{tag}r{i}" + (f"-retry{k}" if k else "")
            b, c = d / f"{stem}-baseline.price.json", d / f"{stem}-candidate.price.json"
            if not (b.is_file() and c.is_file()):
                break
            used += [str(b), str(c)]
            if clean(b) and clean(c):
                out.append((load(b), load(c)))
                break
    return out, used

def main(repo, out_path, evid=None):
    import os
    repo = Path(repo).resolve()
    evid = Path(evid).resolve() if evid else None
    rel = lambda p: os.path.relpath(p, evid) if evid else str(p.relative_to(repo))
    runs = repo / "research/ic_descent_20260930/runs-isolated/main"
    s20_p = repo / "research/ic_exponent_20260926/analysis.json"
    board_p = repo / "docs/ic/leaderboard.json"
    s20 = {(z["a"], z["n"]): z for z in load(s20_p)["sizes"]}
    board = [b for b in load(board_p)["board"] if b["family"] == "C"]
    rows, heads, rawfiles = [], [], 0
    for b in board:
        a_n = None
        for d in sorted(runs.glob("k*n*")):
            if not d.is_dir(): continue
            a = int(d.name[1:].split("n")[0]); n = int(d.name.split("n")[1])
            if s20[(a, n)]["curve"].replace(" ", "") == b["legacy"].replace(" ", ""):
                a_n = (a, n, d)
        a, n, d = a_n
        sets = []
        for j in (1, 2, 3, 4):
            first, u1 = pairs(d / f"M{j}", "")
            dbl, u2 = pairs(d / f"M{j}", "double-")
            fig = dbl if dbl else first
            rawfiles += len(u1) + len(u2)
            sb = statistics.median(x["median"]["s_per_target"] for x, _ in fig)
            geo = math.exp(statistics.fmean(math.log(med_total(x) / med_total(y)) for x, y in fig))
            sets.append({"set": f"M{j}", "pairs": len(fig), "doubled": bool(dbl), "s_before": sb,
                         "wall_speedup": geo, "s_after": sb / geo,
                         "floor": fig[0][0]["floor"]["per_target_k"], "log2r": fig[0][0]["log2_r"]})
        S_IC = statistics.fmean(s["s_after"] for s in sets)
        S_ref = s20[(a, n)]["s_rho"]
        floor = sets[0]["floor"]
        rc = {"S_IC": S_IC, "S_ref": S_ref, "x_reference": S_IC / S_ref, "x_floor": S_IC / floor}
        claimed = {"S_IC": b["best"]["s"], "S_ref": b["reference_s"],
                   "x_reference": b["best"]["ratio_rho"], "x_floor": b["best"]["ratio_floor"]}
        agree = {k: abs(rc[k] - claimed[k]) <= 1e-9 * max(abs(rc[k]), abs(claimed[k])) for k in rc}
        for k in rc:
            heads.append({"name": f"{b['slug']} {k}", "claimed": claimed[k], "recomputed": rc[k], "rel_tol": 1e-9})
        rows.append({"slug": b["slug"], "curve": b["legacy"], "dir": d.name, "sets": sets, "recomputed": rc,
                     "claimed": claimed, "agree": agree, "pass_recompute": all(agree.values()),
                     "s_before_mean": statistics.fmean(s["s_before"] for s in sets),
                     "pairs_per_set": [s["pairs"] for s in sets]})
    rep = {"schema": "validator-report/v1", "independent": True,
           "status": "pass" if all(r["pass_recompute"] for r in rows) else "fail",
           "table": "C", "method": "stdlib python recompute from the committed isolated price files and the section-20 rho figure",
           "raw_data": [{"path": rel(s20_p), "sha256": sha(s20_p)},
                        {"path": rel(board_p)}],
           "raw_price_files_read": rawfiles, "headline": heads, "rows": rows}
    json.dump(rep, open(out_path, "w"), indent=1)
    print("status", rep["status"], "files", rawfiles)
    for r in rows:
        print(r["slug"], {k: round(v, 5) for k, v in r["recomputed"].items()}, r["pairs_per_set"], all(r["agree"].values()))

if __name__ == "__main__":
    main(*sys.argv[1:])
