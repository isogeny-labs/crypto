"""Independently check the shape and standard-monomial count of emitted bases."""

import argparse
import json
from pathlib import Path
import re

PRODUCT = re.compile(r"v(\d+)\*v(\d+)(?: \+ v(\d+))?")
SQUARE = re.compile(r"v(\d+)\^2 \+ v\1")
GRAPH = re.compile(r"x(\d+)(?: \+ (?:1|v\d+))*")


def verify(record, directory):
    width = record["auxiliary_variables"]
    variables = (1 << width) - 1
    allowed = [variables ^ (1 << i) for i in range(width)]
    graph = set()
    squares = set()
    gates = {}
    products = 0
    target = record["target"]
    with (directory / f"target-{target}.basis.txt").open() as file:
        for line in file:
            row = line.rstrip("\n")
            if match := PRODUCT.fullmatch(row):
                a, b = (int(value) for value in match.groups()[:2])
                assert 0 <= a < b < width and allowed[a] >> b & 1
                allowed[a] ^= 1 << b
                allowed[b] ^= 1 << a
                products += 1
                if match[3] is not None:
                    assert int(match[3]) == b and b not in gates
                    gates[b] = a
            elif match := SQUARE.fullmatch(row):
                j = int(match[1])
                assert j not in squares
                squares.add(j)
            else:
                match = GRAPH.fullmatch(row)
                assert match is not None
                i = int(match[1])
                terms = row.split(" + ")[1:]
                assert i not in graph and len(set(terms)) == len(terms)
                assert all(term == "1" or int(term[1:]) < width for term in terms)
                graph.add(i)
    assert graph == set(range(20))
    assert squares == set(range(width))
    assert products == record["quadratic_rows"]
    assert products + len(graph) + len(squares) == record["basis_rows"]
    assert all(not allowed[parent] and parent not in gates for parent in gates.values())

    # A squarefree standard monomial is a clique of allowed pairs.
    # The allowed-pair graph is a disjoint union of cliques.
    standards = 1
    unvisited = variables
    while unvisited:
        start = unvisited & -unvisited
        group = start | allowed[start.bit_length() - 1]
        free = group
        parents = set()
        while free:
            bit = free & -free
            i = bit.bit_length() - 1
            assert allowed[i] == group ^ bit
            parents.add(gates.get(i))
            free ^= bit
        assert len(parents) == 1
        standards += (1 << group.bit_count()) - 1
        unvisited ^= group
    assert standards == record["roots"]
    print(f"Target {target}: {products:,} quadratic products, {standards:,} standard monomials")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("records", type=Path)
    parser.add_argument("basis_directory", type=Path)
    args = parser.parse_args()
    for record in map(json.loads, args.records.read_text().splitlines()):
        verify(record, args.basis_directory)
