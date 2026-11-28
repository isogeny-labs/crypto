#!/usr/bin/env python3
"""Exhaustive finite control for unsigned relation-graph incidence rows."""

from itertools import product


P = 3


def weighted_sum(rows: list[list[int]], weights: tuple[int, ...]) -> tuple[int, ...]:
    return tuple(
        sum(weight * row[column] for weight, row in zip(weights, rows)) % P
        for column in range(len(rows[0]))
    )


def kernel(rows: list[list[int]]) -> list[tuple[int, ...]]:
    return [
        weights
        for weights in product(range(P), repeat=len(rows))
        if not any(weighted_sum(rows, weights))
    ]


def main() -> None:
    triangle = [[1, 1, 0], [0, 1, 1], [1, 0, 1]]
    loop = [[2]]
    square = [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 1, 1], [1, 0, 0, 1]]
    parallel = [[1, 1], [1, 1]]

    triangle_kernel = kernel(triangle)
    loop_kernel = kernel(loop)
    square_kernel = kernel(square)
    parallel_kernel = kernel(parallel)

    assert triangle_kernel == [(0, 0, 0)]
    assert loop_kernel == [(0,)]
    assert (1, 2, 1, 2) in square_kernel
    assert (1, 2) in parallel_kernel
    assert weighted_sum(square, (1, 2, 1, 2)) == (0, 0, 0, 0)
    assert weighted_sum(parallel, (1, 2)) == (0, 0)

    print("PASS triangle/loop kernels trivial; square/parallel controls close over F_3")


if __name__ == "__main__":
    main()
