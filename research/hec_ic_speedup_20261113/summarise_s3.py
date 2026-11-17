#!/usr/bin/env python3
"""Gates and summary for the S3 run (PLAN.md §9).

    summarise_s3.py OUT_DIR [--gates-only]

Gates: G8 (every stream process, old and new, count builds included, has the
same walk_hash, oracle_hash, smooth, frobenius and parity_rejects per row as
count-old) and G9 (every end-to-end process has the same JSON as e2e1-A
apart from *_ms fields; every run correct).  Exits non-zero if a gate fails.

Summary, one JSON line per row to OUT_DIR/summary.jsonl:
- ratio_med / ratio_min / ratio_max: per_trial_ns(mean of A, A') / per_trial_ns(B)
  over the stream repetitions (the primary statistic);
- walk_ratio_med, oracle_ratio_med: the same for each component;
- aa_med: median of A / A' (validity: in [0.95, 1.05] on every row);
- old/new per-trial, walk and oracle medians (ns);
- inversions per walk step and per oracle call, old and new (count builds);
- e2e_ratio_med: median over repetitions and seeds of total_ms(mean A, A') / total_ms(B)
  for round eight's rows (descriptive).
"""
import glob
import json
import os
import statistics as st
import sys


def lines(path):
    with open(path) as f:
        return [json.loads(l[5:]) for l in f if l.startswith("JSON ")]


def strip_ms(j):
    return {k: v for k, v in j.items() if not k.endswith("_ms")}


def main():
    out = sys.argv[1]
    gates_only = "--gates-only" in sys.argv
    ref = lines(f"{out}/count-old.txt")
    keys = ["walk_hash", "oracle_hash", "smooth", "frobenius", "parity_rejects"]
    ok = True
    streams = sorted(glob.glob(f"{out}/stream*-*.txt"))
    for path in [f"{out}/count-new.txt"] + streams:
        got = lines(path)
        if len(got) != len(ref) or any(
            {k: a[k] for k in keys} != {k: b[k] for k in keys} for a, b in zip(ref, got)
        ):
            print(f"G8 FAILED: {path}", file=sys.stderr)
            ok = False
    e2e = sorted(glob.glob(f"{out}/e2e*-*.txt"))
    eref = [strip_ms(j) for j in lines(e2e[0])] if e2e else []
    for path in e2e:
        got = lines(path)
        if [strip_ms(j) for j in got] != eref or len(got) != 55 or not all(j["correct"] for j in got):
            print(f"G9 FAILED: {path}", file=sys.stderr)
            ok = False
    print(
        f"G8 {'holds' if ok else 'see above'}: {1 + len(streams)} stream processes vs count-old, "
        f"{len(ref)} rows; G9: {len(e2e)} end-to-end processes x 55 runs",
        file=sys.stderr,
    )
    if not ok:
        sys.exit(1)
    if gates_only:
        return

    reps = sorted({os.path.basename(p).split("-")[0][len("stream"):] for p in streams}, key=int)
    arm = {(r, a): lines(f"{out}/stream{r}-{a}.txt") for r in reps for a in ("A", "B", "A2")}
    cnew = lines(f"{out}/count-new.txt")
    e2e_reps = sorted({os.path.basename(p).split("-")[0][len("e2e"):] for p in e2e}, key=int)
    earm = {(r, a): lines(f"{out}/e2e{r}-{a}.txt") for r in e2e_reps for a in ("A", "B", "A2")}
    rows = []
    for i, base in enumerate(ref):
        g, p = base["genus"], base["p"]
        rat, wr, orr, aa, oldv, newv, oldw, neww, oldo, newo = ([] for _ in range(10))
        for r in reps:
            a, b, a2 = arm[(r, "A")][i], arm[(r, "B")][i], arm[(r, "A2")][i]
            mo = (a["per_trial_ns"] + a2["per_trial_ns"]) / 2
            rat.append(mo / b["per_trial_ns"])
            wr.append((a["walk_ns"] + a2["walk_ns"]) / 2 / b["walk_ns"])
            orr.append((a["oracle_ns"] + a2["oracle_ns"]) / 2 / b["oracle_ns"])
            aa.append(a["per_trial_ns"] / a2["per_trial_ns"])
            oldv.append(mo)
            newv.append(b["per_trial_ns"])
            oldw.append((a["walk_ns"] + a2["walk_ns"]) / 2)
            neww.append(b["walk_ns"])
            oldo.append((a["oracle_ns"] + a2["oracle_ns"]) / 2)
            newo.append(b["oracle_ns"])
        e2e_rat = []
        for r in e2e_reps:
            for ja, jb, ja2 in zip(earm[(r, "A")], earm[(r, "B")], earm[(r, "A2")]):
                if ja["genus"] == g and ja["p"] == p:
                    e2e_rat.append((ja["total_ms"] + ja2["total_ms"]) / 2 / jb["total_ms"])
        rows.append(
            {
                "genus": g,
                "p": p,
                "m": base["m"],
                "reps": len(reps),
                "ratio_med": st.median(rat),
                "ratio_min": min(rat),
                "ratio_max": max(rat),
                "walk_ratio_med": st.median(wr),
                "oracle_ratio_med": st.median(orr),
                "aa_med": st.median(aa),
                "aa_min": min(aa),
                "aa_max": max(aa),
                "old_per_trial_ns": st.median(oldv),
                "new_per_trial_ns": st.median(newv),
                "old_walk_ns": st.median(oldw),
                "new_walk_ns": st.median(neww),
                "old_oracle_ns": st.median(oldo),
                "new_oracle_ns": st.median(newo),
                "old_walk_inv": base["walk_inv"],
                "new_walk_inv": cnew[i]["walk_inv"],
                "old_oracle_inv": base["oracle_inv"],
                "new_oracle_inv": cnew[i]["oracle_inv"],
                "e2e_ratio_med": st.median(e2e_rat) if e2e_rat else None,
                "e2e_pairs": len(e2e_rat),
            }
        )
    with open(f"{out}/summary.jsonl", "w") as f:
        for r in rows:
            f.write(json.dumps(r) + "\n")
    for r in rows:
        print(
            f"g{r['genus']} p={r['p']:5} ratio {r['ratio_med']:.2f} [{r['ratio_min']:.2f},{r['ratio_max']:.2f}] "
            f"walk {r['walk_ratio_med']:.2f} oracle {r['oracle_ratio_med']:.2f} A/A {r['aa_med']:.3f} "
            f"inv w {r['old_walk_inv']:.2f}->{r['new_walk_inv']:.2f} o {r['old_oracle_inv']:.2f}->{r['new_oracle_inv']:.2f} "
            f"e2e {r['e2e_ratio_med'] if r['e2e_ratio_med'] is None else round(r['e2e_ratio_med'], 2)}"
        )


main()
