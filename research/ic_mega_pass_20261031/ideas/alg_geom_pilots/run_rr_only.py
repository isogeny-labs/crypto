"""Riemann-Roch star presentation only, larger cells, longer timeout, threads."""
import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from binpdp import *
TMP = os.environ.get("TMPDIR", "/tmp")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "RESULTS_rr_only.json")
cells = [tuple(int(v) for v in c.split(":")) for c in sys.argv[1:]]
TIMEOUT = int(os.environ.get("PILOT_TIMEOUT", "600"))
THREADS = os.environ.get("PILOT_THREADS", "2")
results = json.load(open(OUT)) if os.path.exists(OUT) else []
for (n, m, l) in cells:
    for want in ("unsat", "sat"):
        seed = 1
        E, bases, R = make_instance(n, m, l, seed * 7919 + n * 31 + l, want)
        sols = all_decompositions(E, bases, R)
        k = 0
        while want == "unsat" and sols and k < 20:
            k += 1
            E, bases, R = make_instance(n, m, l, seed * 7919 + n * 31 + l + 100 * k, want)
            sols = all_decompositions(E, bases, R)
        names, polys = build_rr(E, bases, R)
        path = os.path.join(TMP, f"rr_{n}_{l}_{want}.ms"); out = os.path.join(TMP, f"rr_{n}_{l}_{want}.out")
        write_msolve(path, names, polys, field_eqs=True)
        r = run_msolve(path, out, timeout=TIMEOUT, extra=("-g", "2", "-t", THREADS))
        r.update({"n": n, "m": m, "l": l, "want": want, "n_decomp_ordered": len(sols),
                  "nvars": len(names), "neqs": len(polys)})
        print(json.dumps(r), flush=True)
        results.append(r)
        json.dump(results, open(OUT, "w"), indent=1)
