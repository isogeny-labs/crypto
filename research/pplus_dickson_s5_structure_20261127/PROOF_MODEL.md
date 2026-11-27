# Proof model: direct Dickson/S5 structural gate

This study has no secret input. It uses one registered public curve, public
Dickson fibres and three deterministic public targets. Consequently its proof
object is exhaustive deterministic replay, not a zero-knowledge proof.
[pcc:unverified]

## Input and randomness custody

- The field, curve and generator are fixed in `PROTOCOL.md`.
- The Dickson roots use a fixed backwards-square-root order.
- The planted target uses the first finite nonidentity signed four-leaf
  multiset in lexicographic order; the other targets are `G` and `[17]G`.
- No random stream is used, so there is no seed to disclose or reconstruct.
- `SOURCE_MANIFEST_2.json` binds the active, prospectively qualified v2 sources
  to the source commit and their SHA-256 values. The v1 manifest and packet are
  retained as the coverage-invalid history described by `AMENDMENT_1.md`.

## Candidate evidence

The candidate emits:

1. all quotient-fold and every multiplication-channel entry through `n=64`;
2. divisor-remainder recurrence and direct-zero sets for all five multiplicity
   charts and all nine target cells through `n=16`;
3. every entry of every bounded direct `S_5` kernel, plus its ordinary and two
   pair-invariant displacement ranks;
4. every lift-filtered zero multiset and the complete recovered signed witness
   set for the three targets; and
5. the explicit four-multiset state size and absence of an exact zero oracle.

The two candidate `S_5` evaluators use Gaussian and fraction-free determinant
paths. Their agreement is a producer self-check, not independent verification.

## Independent replay

The verifier shares no Rust module with the candidate. It separately:

- constructs `D_n` in the monomial basis and reduces every basis product;
- reconstructs every root and evaluates the Dickson recurrence at that root;
- forms `S_4` with a generic polynomial Sylvester determinant and evaluates
  the final `S_5` resultant with subset dynamic programming;
- compares every emitted kernel field element, then independently recomputes
  all ranks, zeros and first-zero positions; and
- independently performs rational lifting, direct curve addition and complete
  sign enumeration, including repetitions, non-lifting roots and `y=0` lifts.

The frozen analyzer accepts algebra only when this replay has zero mismatches.
It returns `STRUCTURE_ONLY_STOP` when no exact subquadratic zero oracle is
present; low rank and compact divisor circuits cannot promote the result.

## What this does not prove

SHA-256 binds files and transport; it does not prove arithmetic. Independent
replay checks the bounded public instances; it is not a proof for every prime,
curve or Dickson degree. Neither program proves that a subquadratic zero oracle
exists or cannot exist. There is no trusted execution receipt, signed PCC
verdict, zero-knowledge transcript, timing claim or production authorization.
Every result remains `[pcc:unverified]` until the repository's independent PCC
process supplies standing.
