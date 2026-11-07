#!/usr/bin/env python3
"""Run a real F4 (msolve 0.6.5, Berthomieu-Eder-Safey El Din; grevlex) on the
descended systems pq2_rank dumps (PQ2_DUMP=<dir>), and record per instance:

* max_step_degree: the largest degree among msolve's F4 steps (first column of
  its -v 2 step table, before the final reduction);
* linear_in_gb: linear polynomials in msolve's reduced Groebner basis;
* k: N - affdim(Z) from the dumped solution set.

Usage: run_msolve.py DUMP_DIR OUT_JSON [CENSUS_JSON]
With CENSUS_JSON (a pq2_rank JSON on the same instances), each row also gets
the census lower bound d_iter, so the two can be compared instance by instance.
"""
import json
import os
import re
import subprocess
import sys
import tempfile
import time
from collections import Counter

MSOLVE = os.environ.get("MSOLVE", "msolve")


def to_msolve(J):
    N = J["N"]

    def mono(m):
        if not m:
            return "1"
        c = Counter(m)
        return "*".join(f"x{i}^{e}" if e > 1 else f"x{i}" for i, e in sorted(c.items()))

    polys = ["+".join(mono(m) for m in e) for e in J["eqs"] if e]
    polys += [f"x{i}^2+x{i}" for i in range(N)]
    return ",".join(f"x{i}" for i in range(N)) + "\n2\n" + ",\n".join(polys) + "\n"


def affdim(Z):
    base = Z[0]
    piv = {}
    for z in Z[1:]:
        r = z ^ base
        while r:
            b = r.bit_length() - 1
            if b in piv:
                r ^= piv[b]
            else:
                piv[b] = r
                break
    return len(piv)


STEP = re.compile(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s")


def run(path):
    J = json.load(open(path))
    with tempfile.TemporaryDirectory() as td:
        fin, fout = os.path.join(td, "in.ms"), os.path.join(td, "out.ms")
        open(fin, "w").write(to_msolve(J))
        t0 = time.time()
        p = subprocess.run([MSOLVE, "-f", fin, "-g", "2", "-v", "2", "-o", fout],
                           capture_output=True, text=True, timeout=3600)
        secs = time.time() - t0
        steps = [int(m.group(1)) for m in map(STEP.match, p.stdout.splitlines()) if m]
        out = open(fout).read() if os.path.exists(fout) else ""
    body = out[out.find("["):]
    elems = [e.strip() for e in body.strip("[]:\n").split(",\n") if e.strip()]
    def term_degree(t):
        return sum(int(e or 1) for e in re.findall(r"x\d+(?:\^(\d+))?", t))

    lin = sum(1 for e in elems if max(term_degree(t) for t in e.split("+")) == 1)
    N = J["N"]
    return dict(file=os.path.basename(path), n=J["n"], N=N, k=N - affdim(J["Z"]),
                max_step_degree=max(steps) if steps else None, steps=steps,
                gb_size=len(elems), linear_in_gb=lin, msolve_seconds=round(secs, 3),
                msolve_exit=p.returncode)


def main():
    ddir, out = sys.argv[1], sys.argv[2]
    census = {}
    if len(sys.argv) > 3:
        for r in json.load(open(sys.argv[3]))["rows"]:
            census[(r["n"], r.get("instance", r.get("inst")))] = r
    rows = []
    files = sorted(os.listdir(ddir), key=lambda f: tuple(int(x) for x in re.findall(r"\d+", f)))
    for f in files:
        if not f.endswith(".json"):
            continue
        r = run(os.path.join(ddir, f))
        inst = int(re.search(r"_i(\d+)", f).group(1))
        r["instance"] = inst
        c = census.get((r["n"], inst))
        if c is not None:
            r["census_k"] = c["k"]
            r["census_d_iter"] = c["d_iter"]
        rows.append(r)
        print(f"n={r['n']:2} i={inst} k={r['k']:2} msolve max step {r['max_step_degree']} "
              f"linear {r['linear_in_gb']} census d_iter {r.get('census_d_iter')} ({r['msolve_seconds']} s)",
              flush=True)
        json.dump({"msolve": subprocess.run([MSOLVE, "-h"], capture_output=True, text=True).stdout[:0] or "0.6.5",
                   "rows": rows}, open(out, "w"), indent=1)


if __name__ == "__main__":
    main()
