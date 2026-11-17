#!/usr/bin/env sage -python
"""Independent finite-field census for the nonsplit quadratic-pole gate."""
import json
from pathlib import Path

from sage.all import GF

HERE = Path(__file__).resolve().parent


def census(m):
    q = 2**m
    L = GF(2**(6*m), "z")
    g = L.multiplicative_generator()
    k2g = g**((q**6-1)//(q**2-1))
    k2 = [L(0)] + [k2g**i for i in range(q*q-1)]
    W0 = next(x+x**(q*q) for x in (g**i for i in range(1, 30))
              if x+x**(q*q))
    line = {c*W0 for c in k2}
    W1 = next(x+x**(q*q) for x in (g**i for i in range(1, 50))
              if x+x**(q*q) not in line)
    assert len({W0*c+W1*h for c in k2 for h in k2}) == q**4
    norms = {w*w**(q**3) for c in k2 for h in k2
             if (w := W0*c+W1*h)}
    e = pow(q+1, -1, q**3-1)
    d = pow(2*(q+1), -1, q**3-1)
    norm_classes = {s**(q-1): s for s in norms}
    zero_classes = sum(1 for s in norm_classes.values()
                       if sum(((s**e)**(q**i) for i in range(3)), L(0)) == 0)
    # One representative of each norm class; the trace test is invariant
    # under k* scaling exactly when its relative trace vanishes.
    outside = sum(1 for s in norms
                  if sum(((s*s)**d)**(2**i) for i in range(3*m)) == 0)
    expected_norms = (q**3+q-2)//2
    expected_classes = (q*q+q+2)//2
    expected_zero_classes = 2 if m % 2 == 0 else 0
    expected_outside = (q**3-q*q+4*q-4)//4 if m % 2 == 0 else (q**3-q*q-4)//4
    hess_new = q*q*(q+1)//4
    union = q**3-1+hess_new+outside
    expected_union = (3*q**3)//2 + (q if m % 2 == 0 else 0) - 2
    assert (len(norms), len(norm_classes), zero_classes, outside, union) == (
        expected_norms, expected_classes, expected_zero_classes,
        expected_outside, expected_union)
    return {"q": q, "norm_values": len(norms),
            "projective_norm_classes": len(norm_classes),
            "relative_trace_zero_classes": zero_classes,
            "new_nonsplit_models": outside,
            "triangle_gate_models": q**3-1,
            "additional_hess_models": hess_new,
            "three_gate_union_models": union,
            "all_normalized_ordinary_models": 2*(q**3-1)}


if __name__ == "__main__":
    rows = [census(m) for m in range(1, 5)]
    output = {"schema": "ghs-nonsplit-population-v1", "rows": rows}
    (HERE/"population.json").write_text(json.dumps(output, indent=2)+"\n")
    print(json.dumps(output, indent=2))
