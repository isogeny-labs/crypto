"""Pilot: msolve solving degree of chained x-only S3 vs Riemann-Roch star presentation
(binary Weil descent, m=3, typed affine-coset bases), plus the field-equation-free
variant of the x-only system (closure-point census). Writes RESULTS_presentations.json."""
import sys, os, json, time
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from binpdp import *

TMP = os.environ.get("TMPDIR", "/tmp")
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "RESULTS_presentations.json")
cells = [(11, 3, 2), (13, 3, 3), (15, 3, 3), (15, 3, 4), (17, 3, 4), (19, 3, 4), (19, 3, 5)]
if len(sys.argv) > 1:
    cells = [tuple(int(v) for v in c.split(":")) for c in sys.argv[1:]]
TIMEOUT = int(os.environ.get("PILOT_TIMEOUT", "240"))
results = []
if os.path.exists(OUT):
    results = json.load(open(OUT))
for (n, m, l) in cells:
    for want in ("unsat", "sat"):
        for seed in (1, 2):
            E, bases, R = make_instance(n, m, l, seed * 7919 + n * 31 + l, want)
            sols = all_decompositions(E, bases, R)
            if want == "unsat" and sols:
                # retry with other seed until no decomposition
                k = 0
                while sols and k < 20:
                    k += 1
                    E, bases, R = make_instance(n, m, l, seed * 7919 + n * 31 + l + 100 * k, want)
                    sols = all_decompositions(E, bases, R)
            row = {"n": n, "m": m, "l": l, "want": want, "seed": seed, "a2": E.a2,
                   "n_decomp_ordered": len(sols)}
            for pres in ("xonly", "rr", "xonly_nofield"):
                if pres == "rr":
                    names, polys = build_rr(E, bases, R)
                else:
                    names, polys = build_xonly(E, bases, R)
                fe = (pres != "xonly_nofield")
                path = os.path.join(TMP, f"pdp_{pres}_{n}_{l}_{want}_{seed}.ms")
                out = os.path.join(TMP, f"pdp_{pres}_{n}_{l}_{want}_{seed}.out")
                write_msolve(path, names, polys, field_eqs=fe)
                r = run_msolve(path, out, timeout=TIMEOUT)
                r.update({"nvars": len(names), "neqs": len(polys),
                          "input_maxdeg": max(len(mn) for p in polys for mn in p)})
                row[pres] = r
                print(json.dumps({"cell": (n, m, l, want, seed), "pres": pres, **r}), flush=True)
            results.append(row)
            json.dump(results, open(OUT, "w"), indent=1)
print("done")
