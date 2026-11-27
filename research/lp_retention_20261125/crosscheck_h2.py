#!/usr/bin/env python3
"""H2: recount the Rust-collected FP0 pools with workspace#144's independent Python code
(docs/notes/fp-ic-triage-20261125/fptest.py, `counts`) and replay every relation."""
import json, sys, importlib.util
import sympy
ws = sys.argv[1]       # path to the workspace fptest.py
pools = json.load(open(sys.argv[2]))
rust = json.load(open(sys.argv[3]))
spec = importlib.util.spec_from_file_location("fptest", ws); fp = importlib.util.module_from_spec(spec); spec.loader.exec_module(fp)
out = []
for col, rep in zip(pools, rust["repeats"]):
    inst = col["instance"]; p, g = inst["p"], inst["g"]
    fb = list(sympy.primerange(2, col["smooth_bound"] + 1))
    edges, squares = [], 0
    for rel, k in zip(col["partials"], col["exponents"]):
        v = 1
        for j, e in rel["small"]: v = v * pow(fb[j], e, p) % p
        L = [q for q, m in rel["large"] for _ in range(m)]
        for q in L:
            assert sympy.isprime(q) and col["smooth_bound"] < q < col["lp_bound"]
            v = v * q % p
        assert v == pow(g, k, p), f"replay failed k={k}"
        if len(L) == 1: edges.append((k, 1, L[0]))
        else:
            a, b = sorted(L); edges.append((k, a, b)); squares += a == b
    b1, U = fp.counts(edges)
    r_ln, r_b1 = rep["full"]["rank"]["left_nullity"], rep["full"]["cycle_count"]
    out.append(dict(seed=col["seed"], replayed=len(edges), py_U=U, rust_left_nullity=r_ln, py_b1=b1,
                    rust_cycle_count=r_b1, squares=squares, U_equal=U == r_ln, b1_minus_rust=b1 - r_b1))
    print(out[-1])
json.dump(out, open("results/h2_crosscheck.json", "w"), indent=1)
# Frozen H2 criterion: U equal and b1 equal on every pool, every relation replayed.
ok = all(o["U_equal"] and o["b1_minus_rust"] == 0 for o in out)
print("H2", "PASS" if ok else "MISMATCH")
