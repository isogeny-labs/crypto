# Exact checks and scope

Run from this directory:

    g++ -std=c++17 -O2 -Wall -Wextra -Werror check.cpp -lgmpxx -lgmp -o /tmp/ecc2k130-prym-check
    /tmp/ecc2k130-prym-check > check.txt
    g++ -std=c++17 -O1 -g -fsanitize=undefined -fno-sanitize-recover=all -Wall -Wextra -Werror check.cpp -lgmpxx -lgmp -o /tmp/ecc2k130-prym-ubsan
    /tmp/ecc2k130-prym-ubsan > check_ubsan.txt
    cmp check.txt check_ubsan.txt
    /tmp/pari-2.19.1/gp -q -f replay.gp > replay.txt 2>&1

The native checker replays ord131(2)=130, polynomial Rabin gcds,
squarefreeness, every possible branch binomial degree1..262, exact
integer genus cases, local Euler identities and the two base-count
contradictions. The hyperelliptic-base argument is written geometry;
enumerating partition sizes only checks its dimension arithmetic.
It is not a native proof of descent or the Prym decomposition.

The initial integer loop used unsigned arithmetic with subtraction
before addition at h=0. It gave the intended output, but was replaced
with signed exact small-integer arithmetic before final validation.
Optimized and UBSan output are identical; no arithmetic mismatch.

No full Lean bridge is available and no signed verdict is claimed.
The scope is the entire Prym isogenous to A, not an arbitrary factor
in a higher-dimensional Prym. No curve search, new class-group run,
paid cloud resource, performance measurement or cryptographic solve.
