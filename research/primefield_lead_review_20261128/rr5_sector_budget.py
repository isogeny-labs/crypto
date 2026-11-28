"""Exact B677 four-distinct-point sector counts for public RR5 planning.

For uniform R in a prime-order group, each signed tuple matches R with
probability 1/n. These are expected raw relation multiplicities, not solver
success probabilities, independent ranks, or cold costs.
"""

from math import comb

D = 509
Y = 168
ORDER = 2_147_446_831
ARITY = 4
SIGNS = 1 << ARITY


def main() -> None:
    total = comb(D + Y, ARITY)
    print("k_D, k_Y, unordered_tuples, tuple_fraction, expected_signed_matches_per_uniform_R")
    counted = 0
    for k in range(ARITY + 1):
        tuples = comb(D, k) * comb(Y, ARITY - k)
        counted += tuples
        print(f"{k}, {ARITY-k}, {tuples}, {tuples/total:.9f}, {SIGNS*tuples/ORDER:.9f}")
    assert counted == total
    print(f"total tuples={total}, expected signed matches={SIGNS*total/ORDER:.9f}")
    print(f"k=3 or 4 fraction={(comb(D, 3)*comb(Y, 1)+comb(D, 4))/total:.9f}")
    print(f"optimistic query count for 677 independent rows={677/(SIGNS*total/ORDER):.3f}")


if __name__ == "__main__":
    main()
