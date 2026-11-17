import json, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
DBG = sys.argv[2]
rows = [json.loads(l) for l in open(sys.argv[1])]
jobs = [r for r in rows if r["arm"] == "rho"]
def go(r):
    argv = ["rhoseq"] + r["argv"][1:]
    o = json.loads(subprocess.run([DBG] + argv, capture_output=True, text=True, check=True).stdout.strip().splitlines()[-1])
    o.update(pb=r["pb"], idx=r["idx"], T=r["T"], argv=argv); return o
with open(sys.argv[3], "w") as fh:
    for o in ThreadPoolExecutor(4).map(go, jobs): fh.write(json.dumps(o) + "\n")
print("rhoseq runs", len(jobs))
