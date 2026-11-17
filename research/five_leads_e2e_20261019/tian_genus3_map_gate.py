"""Replay the printed F_257 genus-3 point-to-quartic divisor cuts.

Reads the SHA-checked extracted coefficient fixture. Rank six of quartic
multiplication in F_257[X,Y]/(two cubic cuts) certifies a degree-three
scheme-theoretic intersection, without relying on a numerical root finder.
This is a point-map gate, not a Jacobian homomorphism or DLP test.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

P = 257
F = [167, 104, 125, 138, 6, 13, 0, 1]
QUARTIC = {
    (4, 0): 1, (3, 1): 89, (2, 2): 244, (1, 3): 126,
    (0, 4): 113, (3, 0): 131, (2, 1): 3, (1, 2): 255,
    (0, 3): 65, (2, 0): 172, (1, 1): 139, (0, 2): 21,
    (1, 0): 201, (0, 1): 228, (0, 0): 70,
}
EXAMPLES = {
    (2, 7): ([90, 77, 239, 1], [132, 61, 101, 1]),
    (121, 5): ([107, 59, 90, 1], [192, 231, 59, 1]),
}


def eval_poly(coefficients: list[int], x: int) -> int:
    result = 0
    for coefficient in reversed(coefficients):
        result = (result * x + coefficient) % P
    return result


def mapped_cuts(fixture: dict, x: int, y: int) -> tuple[list[int], list[int]] | None:
    den = eval_poly(fixture["denominator"], x)
    if den == 0:
        return None
    inv = pow(den, -1, P)
    c = [((eval_poly(row["x_only"], x) + y * eval_poly(row["y_factor"], x)) * inv) % P
         for row in fixture["functions"]]
    return [c[2] * -1 % P, c[1], c[0] * -1 % P, 1], [c[5] * -1 % P, c[4], c[3] * -1 % P, 1]


def quotient_powers(cubic: list[int], n: int = 6) -> list[list[int]]:
    values = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    for _ in range(3, n + 1):
        previous = values[-1]
        product = [0, previous[0], previous[1], previous[2]]
        top = product.pop()
        for i in range(3):
            product[i] = (product[i] - top * cubic[i]) % P
        values.append(product)
    return values


def quartic_rank(xcut: list[int], ycut: list[int]) -> int:
    xp = quotient_powers(xcut)
    yp = quotient_powers(ycut)
    columns = []
    for xbasis in range(3):
        for ybasis in range(3):
            column = [0] * 9
            for (i, j), coefficient in QUARTIC.items():
                for a in range(3):
                    for b in range(3):
                        column[3 * a + b] = (
                            column[3 * a + b] + coefficient * xp[i + xbasis][a] * yp[j + ybasis][b]
                        ) % P
            columns.append(column)
    matrix = [[columns[j][i] for j in range(9)] for i in range(9)]
    rank = 0
    for column in range(9):
        pivot = next((i for i in range(rank, 9) if matrix[i][column]), None)
        if pivot is None:
            continue
        matrix[rank], matrix[pivot] = matrix[pivot], matrix[rank]
        inv = pow(matrix[rank][column], -1, P)
        matrix[rank] = [v * inv % P for v in matrix[rank]]
        for i in range(rank + 1, 9):
            factor = matrix[i][column]
            matrix[i] = [(a - factor * b) % P for a, b in zip(matrix[i], matrix[rank])]
        rank += 1
    return rank


def main() -> None:
    fixture = json.loads(Path(sys.argv[1]).read_text())
    if fixture["schema"] != "tian-genus3-f257-map-coefficients/v1":
        raise ValueError("unexpected fixture schema")
    stats = {"source_rational_points": 0, "denominator_zero": 0, "rank_six": 0,
             "rank_other": {}, "rank_other_points": [], "printed_examples_checked": 0}
    for x in range(P):
        rhs = eval_poly(F, x)
        for y in range(P):
            if y * y % P != rhs:
                continue
            stats["source_rational_points"] += 1
            cuts = mapped_cuts(fixture, x, y)
            if cuts is None:
                stats["denominator_zero"] += 1
                continue
            if (x, y) in EXAMPLES:
                assert cuts == EXAMPLES[x, y], ((x, y), cuts, EXAMPLES[x, y])
                stats["printed_examples_checked"] += 1
            rank = quartic_rank(*cuts)
            if rank == 6:
                stats["rank_six"] += 1
            else:
                stats["rank_other"][str(rank)] = stats["rank_other"].get(str(rank), 0) + 1
                stats["rank_other_points"].append({"x": x, "y": y, "rank": rank})
    assert stats["printed_examples_checked"] == 2
    xcut, ycut = EXAMPLES[2, 7]
    corrupt = xcut.copy()
    corrupt[2] = (corrupt[2] + 1) % P
    stats["corrupt_coefficient_rank"] = quartic_rank(corrupt, ycut)
    print(json.dumps({"schema": "tian-genus3-f257-map-gate/v1", "result": stats}, indent=2))


if __name__ == "__main__":
    main()
