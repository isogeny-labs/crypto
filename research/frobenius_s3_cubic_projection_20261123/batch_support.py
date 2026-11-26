"""Measure the modular product support of actual joined selector residuals."""

import argparse
import json
from pathlib import Path
from random import Random
import subprocess
from time import perf_counter

from profile import factors, geometry, span
from selector_elimination import normalized_rhs
from subspace_resultant import flint_polynomials, subspace_polynomial, target_residual
from three_summand_join import Curve, negate


def inputs(curve, basis, g):
    """Plant a target using the first three of eight signed factor-base points."""
    rng = Random(g)
    available = [signs for s in span(basis[:7]) if (signs := curve.lift(s ^ 1))]
    thirds = [rng.choice(signs) for signs in rng.sample(available, 8)]
    target = curve.add(thirds[0], curve.add(thirds[1], thirds[2]))
    assert target is not None and curve.is_double(target)
    abscissae = dict.fromkeys(w[0] for p in thirds if (w := curve.add(target, negate(p))))
    return target, list(abscissae)


def product(binary, field, basis, g, a, b):
    header = f"{field.n} {field.poly} {len(basis)} {g} {basis[-1]}\n"
    body = "".join(f"{len(t)}\n" + "".join(f"{e} {c}\n" for e, c in t.items()) for t in (a, b))
    result = subprocess.run([str(binary)], input=header + body, text=True, capture_output=True, check=True)
    return dict(tuple(map(int, line.split())) for line in result.stdout.splitlines())


def run(n, binary, max_products, control):
    for g in factors(n):
        field, basis, _ = geometry(n, g)
        curve = Curve(field)
        target, abscissae = inputs(curve, basis, g)
        k = len(basis)
        subspace = [(g >> j) & 1 for j in range(k + 1)]
        row = [target_residual(field, subspace, t, normalized_rhs(field, basis, t)[0])
               for t in abscissae]
        if n == 23:
            ring, polynomial = flint_polynomials(field)
            domain = subspace_polynomial(ring, subspace)
        print(json.dumps(dict(n=n, g=g, target=target, intermediate_abscissae=abscissae,
                              level=0, terms=[len(t) for t in row])), flush=True)

        level = 0
        while len(row) > 1:
            level += 1
            next_row = []
            outcomes = []
            for a, b in zip(row[::2], row[1::2]):
                pairs = len(a) * len(b)
                if pairs > max_products:
                    outcomes.append(dict(pair_products=pairs, capped=True))
                    continue
                start = perf_counter()
                terms = product(binary, field, basis, g, a, b)
                elapsed = perf_counter() - start
                if n == 23:
                    assert polynomial(terms) == (polynomial(a) * polynomial(b)) % domain
                if control is not None:
                    assert terms == product(control, field, basis, g, a, b)
                outcomes.append(dict(pair_products=pairs, terms=len(terms), seconds=elapsed))
                next_row.append(terms)
            print(json.dumps(dict(n=n, g=g, target=target, level=level, products=outcomes)), flush=True)
            if len(outcomes) != len(next_row):
                break
            if len(row) % 2:
                next_row.append(row[-1])
            row = next_row


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("fields", nargs="+", type=int, choices=(23, 47))
    parser.add_argument("--multiplier", type=Path, required=True)
    parser.add_argument("--control", type=Path)
    parser.add_argument("--max-products", type=int, default=100_000_000)
    args = parser.parse_args()
    for degree in args.fields:
        control = args.control.resolve() if args.control else None
        run(degree, args.multiplier.resolve(), args.max_products, control)
