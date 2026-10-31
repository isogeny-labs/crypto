"""Check that run_w1's split of base.make_inputs equals the frozen function on every cell.

Run: python3 test_replication.py   (prints one line per (policy, s, seed); exit 1 on mismatch)
"""
import sys

import run_w1 as w
base, ext = w.base, w.ext

failures = 0
for policy, sizes in (("quotient", (2,)), ("normal_subspace", (2, 3, 4))):
    for s in sizes:
        for seed in w.SEEDS:
            base.S = s
            setup = (ext.archived_setup if policy == "quotient" else ext.PlainSetup)(
                w.N, s, seed, "ternary_inline", w.MODULUS)
            curve = setup.curve
            g = base.field_generator(curve)
            try:
                frozen = base.make_inputs(setup, seed, g)
            except AssertionError as error:
                frozen = ("assertion", str(error))
            used = w.factor_base_lifts(curve, setup, s)
            mine = None
            if not (frozen and frozen[0] == "assertion"):
                planted, natural = w.planted_and_natural(curve, seed, s, used, g)
                mine = (planted, natural, used)
            ok = mine == frozen
            failures += not ok
            print(policy, s, seed, "planted", len(frozen[0]) if mine else "n/a", "lifts", len(used),
                  "IDENTICAL" if ok else "MISMATCH")
sys.exit(1 if failures else 0)
