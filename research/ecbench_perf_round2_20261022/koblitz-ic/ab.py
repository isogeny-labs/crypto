"""A/B of `ic price` outputs: identical counts and answers, then set-up timing."""
import json, statistics as st, sys, glob, os

def load(p): return json.load(open(p))

def counted(d):
    reps = []
    for r in d["repetitions"]:
        ic, rho = r["ic_online"], r["rho_online"]
        reps.append({"ic": {k: ic[k] for k in ("recovered", "trials", "verified")},
                     "rho": {k: rho[k] for k in ("recovered", "steps", "walks", "rounds",
                                                  "distinguished_points", "target_ops", "verified")}})
    return {"counts": d["counts"], "rho_counts": d["rho_counts"], "certificates": {
                k: {kk: vv for kk, vv in v.items()} if isinstance(v, dict) else v
                for k, v in d["certificates"].items() if not k.endswith("sha256")},
            "identity_inputs": d["identity_inputs"], "recipe": d["recipe"], "reps": reps}

def main(rdir, a, b, out=None):
    curves = sorted({os.path.basename(f).split("-")[0] for f in glob.glob(f"{rdir}/*.json")})
    summary, diffs = {}, 0
    for c in curves:
        A = [load(f) for f in sorted(glob.glob(f"{rdir}/{c}-{a}*.json"))]
        B = [load(f) for f in sorted(glob.glob(f"{rdir}/{c}-{b}*.json"))]
        ref = counted(A[0])
        for d in A[1:] + B:
            if counted(d) != ref:
                diffs += 1
                print("DIFF", c, [k for k in ref if counted(d)[k] != ref[k]])
        def setup(ds, phase=None):
            xs = [r["setup_phases_ns"].get(phase, 0) if phase else r["setup_ns"] for d in ds for r in d["repetitions"]]
            return st.median(xs) / 1e6
        phases = sorted(A[0]["repetitions"][0]["setup_phases_ns"], key=lambda p: -setup(A, p))
        row = {"setup_ms": (setup(A), setup(B)), "phases": {p: (setup(A, p), setup(B, p)) for p in phases},
               "online_ms": (st.median(r["ic_online"]["wall_ns"] for d in A for r in d["repetitions"]) / 1e6,
                             st.median(r["ic_online"]["wall_ns"] for d in B for r in d["repetitions"]) / 1e6),
               "n": (sum(len(d["repetitions"]) for d in A), sum(len(d["repetitions"]) for d in B))}
        summary[c] = row
        sa, sb = row["setup_ms"]
        print(f"{c}: setup {sa:8.2f} -> {sb:8.2f} ms ({sb/sa:.3f})  online {row['online_ms'][0]:.2f} -> {row['online_ms'][1]:.2f}  n={row['n']}")
        for p in phases[:7]:
            pa, pb = row["phases"][p]
            if pa > 0.05: print(f"     {p:18} {pa:8.2f} -> {pb:8.2f} ({pb/pa if pa else 0:.3f})")
    print("records differing:", diffs)
    if out: json.dump({"a": a, "b": b, "differing": diffs, "curves": summary}, open(out, "w"), indent=1)
    return diffs

if __name__ == "__main__":
    sys.exit(1 if main(*sys.argv[1:]) else 0)
