# Boolean-tensor-network relation finding (Akshima et al., v0.2) — prototype

Prototype of the relation-finding step of "A Subexponential Time Algorithm for
Elliptic Curve Discrete Logarithm" (Bochum/MPI, 2026-10-29), which is
**conditional on an unreviewed OpenAI tensor-contraction algorithm** and
asymptotic only. The authors publish no code (their repository holds the PDF
only). This directory encodes "A1 + A2 = R, A_i in a hashed factor base" on the
Koblitz curve `y^2 + xy = x^3 + 1` as a network of local predicates, counts
solutions with a plain bucket-elimination contractor (a stand-in for the
claimed algorithm), samples a witness bit by bit (paper Lemma 1), and runs the
index-calculus linear algebra.

No speed-up, no ECC2K-130 discrete logarithm, no leaderboard or benchmark row.

| File | Role |
|---|---|
| `btn.py` | GF(2^n), curve, constraint network, multiplier/squarer gadgets, contractor |
| `relation.py` | point-addition, curve-membership and factor-base predicates |
| `test_enc.py` | network count == brute-force count of decompositions |
| `run_toy.py` | full m=2 index calculus on GF(2^5), r = 11 |
| `ecc2k130_check.py` | the same encoding on the ECC2K-130 field and curve |

## Verified results (local, seeded)

* `python3 test_enc.py 5`: for 3 random R on GF(2^5), the contraction count
  equals the brute-force number of ordered pairs (A1, A2), x1 != x2, with
  A1 + A2 = R (e.g. 42 = 42, 40 = 40).
* `python3 run_toy.py 1` (`toy_seed1.log`): GF(2^5), group order 44 = 4·11,
  subgroup r = 11, |F1| = |F2| = 8, N = 16. 17 relations, each checked with
  reference curve arithmetic; the kernel mod 11 recovered the secret
  ξ = 3 and `Q == [ξ]P` holds. Contraction took about 340 s, so this checks the
  construction, not speed.
* `python3 ecc2k130_check.py`: curve `y^2+xy=x^3+1` over GF(2^131),
  f = x^131+x^13+x^2+x+1 (the ECC2K-130 curve). The addition + two curve-membership
  network has 105,062 variables, 104,801 predicates, 524,008 edges after
  replacing each variable by an equality tree, maximum predicate arity 5.
  A witness derived from reference arithmetic on random curve points satisfies
  all predicates (0 violated); flipping one bit of the output violates 1.
  Factor-base predicates are omitted at this size (a 131-bit table).

## What this does not show

* The contraction is exponential (treewidth ~23 at n = 5, ~34 at n = 7, where
  the dense tensors no longer fit). Only a working Theorem D.1 algorithm would
  change that, and the paper states it saves nothing below about 2^(2^17) edges.
* The multiplier chain here has O(n^2) edges. The paper's O(n)-edge Fourier
  multiplier is not implemented, and its integer construction does not carry
  over to GF(2)[x] without a characteristic-2 variant. The paper itself treats
  prime fields; the ECC2K-130 run is an encoding check on the binary curve.
* ECC2K-130 is not attacked. Its prime-order subgroup ℓ is 130 bits.
