# L1 — independent degree-4 closure at n=31

**Verdict: the N1 n=31 stall is real for this closure definition. It is not a gf2close artefact.** An independently written
closure (`src/macclose.c`) reproduces N1 exactly: C1 s0/s1 give (low,dim,one) = (2821, 37293, 0), D1 s0 gives (2821, 37278, 0),
and A s2 reaches 1 ∈ W_4 (low 5489).

## Why the implementation counts as independent
It is a fresh C file that does not reuse gf2close logic. It differs in several ways: degree-4 columns come first and the pivot is the
lowest bit; columns are found through a hash table rather than colex ranking; reduction is plain row by row (no M4RI); and the closure
runs in rounds (Jacobi style), logging pivot counts by leading degree each round. Only the generator (N1 `lib.system`) is shared.
Build with `clang -O3 -march=native -o src/macclose src/macclose.c`, then run `src/validate.py` and `src/main.py n arm s`.

## Validation (P1 held)
297 of 297 instances agree on (dim, low, one), with 0 mismatches:
- 162 instances at n=5,7,9 for d=3 and d=4, checked against both a pure-Python fixpoint reference and gf2close.
- 135 instances at n=13..21, d=4, all 9 arms, s=0..2, checked against gf2close.
- 54 of the 297 contain 1 in W.

Data: `raw/validation.jsonl.gz`.

## Round logs (low = dim W∩P≤3; bydeg = pivots by leading degree 0..4)
| instance | after round 1 low (deg≤2 / deg3) | after round 2 | after round 3 (degree-4 products) | final |
|---|---|---|---|---|
| n31 C1 s0, s1 | 1022 (93 / 929) | **2821**, deg4 rank 13020 | +0 low, deg4 rank 34472 < C(32,4)=35960 | stall 2821 |
| n31 D1 s0 | 1020 | 2760 | 2821 (+61), deg4 rank 34457 | stall 2821 |
| n31 A s2 | 1022 (93 / 929) | **2821** (identical to C1) | **+124** → 2945, deg4 34224 | round 4 deg4 = 35960 (full), round 5 → 1 |
| n29 C1 s0 | 898 (87 / 811) | **2465 = 29·85** | +261, deg4 = C(30,4)=27405 (full) | 4524 (SAT draw, nsol=2) |
| n29 D1 s0 | 898 | 2432 | +200, deg4 full 27405 | 1 ∈ W |

## What 2821 is
- **2821 = n(3N−5) = 31·91**: this is the size of the degree-≤3 part after round 2, before any degree-4 product produces a fall.
  For C1 the round-1 span holds exactly **3n** elements of degree ≤2 (n=31: 1 linear + 92 quadratic; n=29: 1 + 86). That is the
  n inputs plus 2n quadratic falls, which fits the rank-n mechanism of g_r = λ_r μ_r + ℓ_r giving (1+λ_r)ℓ_r and (1+μ_r)ℓ_r.
  The low count after round 1 is n(N+1)−1. The pre-registered genuine prediction for n=29, 29·85 = 2465, **held for C1**.
  It **failed for D1** (2432 at n=29; D1 has 101 quadratics at n=31 rather than 92), so the formula is not general across the
  rank-n arms. At n=31 D1 still lands on 2821, and I do not have a derivation for that coincidence.
- **The stall is not "rank-n versus Semaev" below degree 4.** A matches C1 bit for bit in its counts through round 2, so 2821 is a
  common plateau. The arms first differ in round 3, where degree-4 products are formed:
  - At n=31 the degree-4 leading rank of C1 and D1 reaches 34472 or 34457. That is below C(32,4)=35960, so no fall is forced and
    no new low element appears.
  - A gets 124 extra falls in round 3, then fills the degree-4 space and collapses.
  - At n=29 the same products fill C(30,4) completely, which forces falls (P4 held).
- So the n=29 → n=31 change for C1/C2/D1 is consistent with a **count threshold**: the degree-4 space C(N,4) outgrows the rank that
  the 2821-dimensional plateau generates. There is no evidence of a special effect from "n=2k−1 with k even". k=15 (n=29) and
  k=16 (n=31) follow the same mechanism, and only the deg-4 coverage changes.
- A's extra 124 falls in round 3 are the Semaev-specific structure that the relabelling reading does not account for at d=4, n=31.
  I did not identify what those 124 falls are.

## Pre-registration scorecard
- P1 held.
- P2 held.
- P3 mixed:
  - the C1 formula and the 2465 prediction held;
  - the D1 generalisation failed;
  - "A > 2821 before degree 4" was **falsified**.
- P4 held.

## Blockers and caveats
- `conductor check` (required by CLAUDE.md) could not run. Inside the sandbox it was denied, and outside it timed out
  ("context deadline exceeded"), so no task was claimed. All writes are confined to this new directory.
- Two background queues were launched twice by accident because the process list could not be inspected. The duplicate runs were
  stopped, and identical outputs came from the surviving copies. Wall times (300–1000 s) are inflated by contention; CPU is 60–130 s
  per instance.
- Only one instance was run per arm and seed listed above; C2 was not rerun. This is not an exhaustive check, and no security claim
  is made.
