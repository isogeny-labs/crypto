import argparse, json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from order import Curve
from ic import run_ic

if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--instances", required=True)
    ap.add_argument("--bits", type=int, required=True)
    ap.add_argument("--arm", required=True)
    ap.add_argument("--seeds", required=True)
    ap.add_argument("--out", required=True)
    a = ap.parse_args()
    seeds = [int(x) for x in a.seeds.split(",")]
    curves = [Curve(json.loads(l)) for l in open(a.instances) if l.strip() and not json.loads(l).get("rejected")]
    curves = [c for c in curves if c.bits == a.bits]
    with open(a.out, "w") as fh:
        for C in curves:
            for s in seeds:
                rec = run_ic(C, s, a.arm)
                fh.write(json.dumps(rec, sort_keys=True) + "\n"); fh.flush()
                print(rec["run_id"], rec["status"], rec["certified"], rec["trials"], flush=True)
