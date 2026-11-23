#!/usr/bin/env python3
"""Cross-check of prefilter_theory.py's #J = L(1) (character sums over F_{p^k}, k = 1..4) against the library's
jacobian_order (val_po 'ord', BSGS) on f = random_f(p, 4, cs), p in {13, 17, 19, 23, 29, 31}, cs = 1..40 (squarefree only).
Agreement shows the validator's point-count arithmetic is right, so the parity test rests on correct orders.
Writes out/theory_crosscheck.json."""
import json, os, random, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import OUT, dump
from jac import random_f
from polyfp import squarefree, reducible_certificate
from prefilter_theory import Fq, jac_order

H = "/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/val4po/bin/val_po"
rnd = random.Random("val4po-xcheck")
rows, bad = [], []
for p in (13, 17, 19, 23, 29, 31):
    fields = {k: Fq(p, k, rnd) for k in range(1, 5)}
    seeds = [cs for cs in range(1, 41) if squarefree(random_f(p, 4, cs), p)]
    out = subprocess.run([H], input="".join(f"ord {p} {cs}\n" for cs in seeds), capture_output=True, text=True, check=True).stdout
    for cs, line in zip(seeds, out.splitlines()):
        r = json.loads(line)
        f = random_f(p, 4, cs)
        N, _ = jac_order(fields, f, p)
        red = reducible_certificate(f, p, rnd) is not None
        e = {"p": p, "cs": cs, "validator_L1": N, "library": r.get("order"), "library_err": r.get("err"),
             "reducible": red, "parity_matches": (N % 2 == 0) == red}
        e["equal"] = r.get("order") == N
        rows.append(e)
        if not e["parity_matches"] or ("order" in r and not e["equal"]):
            bad.append(e)
res = {"curves": len(rows), "library_orders": sum(1 for e in rows if e["library"] is not None),
       "equal": sum(e["equal"] for e in rows), "library_errors": [e for e in rows if e["library"] is None],
       "bad": bad, "ok": not bad and sum(e["equal"] for e in rows) > 0, "rows": rows}
dump("theory_crosscheck.json", res)
print({k: v for k, v in res.items() if k != "rows"})
