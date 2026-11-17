"""Witness verifier for groebner_compare.semaev instances: group re-addition.

Runner contract operation `verify_witnesses`. For every algebraic root the
runner has already replayed on the input equations, this worker

1. regenerates the instance from its recorded parameters (variant, m, n, l,
   a, b, seed, index, modulus) and rejects it unless the regenerated equations
   and target abscissa are identical to the instance that was solved, then
2. decodes the root to factor-base abscissae (directly, or for the e-only
   variant as factor-base roots of T^3 + e1 T^2 + e2 T + e3), lifts each to
   curve points, and accepts only if some sign choice sums to a point with
   abscissa x_R under ordinary curve addition.

Step 2 evaluates no Semaev polynomial, so a spurious algebraic root (for
example an intermediate U that is not the abscissa of a rational point, or an
e-triple that does not split over the factor base) is rejected. Step 1 shares
the generator code with the producer, so it detects tampering or drift of the
frozen input, not a generator bug; the unit tests cover the generator.
"""

import json
import sys

from .certificate import terms
from .semaev import generate, readdition


def verify(request):
    if request["operation"] != "verify_witnesses":
        return {"status": "incompatible", "reason": "only verify_witnesses is supported"}
    instance = request["instance"]
    meta = instance["semaev"]
    n = request["ring"]["nvars"]
    regenerated, _ = generate(meta["m"], meta["n"], meta["l"], variant=meta["variant"],
                              a=meta["a"], b=meta["b"], seed=meta["seed"],
                              index=meta["index"], modulus=meta["modulus"])
    if (terms(regenerated["equations"], n) != terms(instance["equations"], n)
            or regenerated["semaev"]["x_R"] != meta["x_R"]):
        return {"status": "error", "reason": "instance differs from its seeded regeneration"}
    confirmed, receipts = [], []
    for solution in request["solutions"]:
        found = readdition(meta, solution)
        if found is not None:
            confirmed.append(solution)
            receipts.append({"solution": solution, **found})
    return {"status": "ok", "verified_solutions": confirmed, "readditions": receipts,
            "method": "seeded-regeneration+curve-point-readdition"}


def main():
    for line in sys.stdin:
        request = json.loads(line)
        try:
            response = verify(request)
        except (KeyError, TypeError, ValueError, NotImplementedError) as error:
            response = {"status": "error", "reason": str(error)}
        response["request_id"] = request["request_id"]
        print(json.dumps(response, allow_nan=False), flush=True)


if __name__ == "__main__":
    main()
