# Prime F4 wide-arithmetic source inventory

Status: planning and source audit only, against source
`69e4c9763bed508f0ab0d2f424873d2e748a7229`, 2026-10-29. This document adds no
implementation, executed correctness evidence, performance result or generated
dataset coverage. No tests were run for this inventory. Source pointers below
refer to this commit; line numbers may change in later revisions.

The frozen [prime-chain contract](PRIME-F4-CHAIN-CONTRACT.md) still admits native
prime F4 only below `2^32`. Its Semaev/Gaudry references establish the equation
and decomposition context, not the correctness or speed of a future arithmetic
backend. Wide execution requires a separately frozen extension and validation.

## Candidate implementation boundary

A separate eager `u128` arithmetic path with canonical `u64` coefficients is
the smallest credible extension through `p=2^61-1`. Reuse the symbolic F4
algorithm; preserve existing narrow entry points, guards and strategy behavior.
Changing `mulmod` alone or removing the modulus guard would leave overflowing
native elimination expressions. Arithmetic eligibility is not tractability at
every requested field size and arity.

| Current source pointer | Assumption or existing capability | Wide-path obligation |
|---|---|---|
| `src/cryptanalysis/f4_fp.rs:66`, `Poly` | Exponents are `Vec<u32>`; coefficients are `u64` | Public polynomial/basis coefficient storage already fits the requested moduli |
| `f4_fp.rs:249-276`, `Barrett` | Constructor requires `2 <= p < 2^32`; `reduce` consumes a `u64` | Its word-sized input cannot hold a 122-bit product; relaxing the guard is insufficient |
| `f4_fp.rs:2135-2138`, native dispatch | Unconditionally constructs Barrett, even for arithmetic mode zero | Existing wide-row mode is not a wide-modulus backend; add explicit backend selection |
| `f4_fp.rs:1303-1316`, `eliminate` | Evaluates `x + (p-f)*y` in `u64` | Widen the complete expression before multiplication/addition |
| `f4_fp.rs:1921-1977`, `rref` | Pivot normalization evaluates `x*inv` in `u64` | Use eager wide modular multiplication and wide-safe elimination |
| `f4_fp.rs:2820-2868`, pair-row reduction | Sparse preprocessing also uses `x + neg*v` in `u64` | Fix this stage as well as dense RREF |
| `f4_fp.rs:1320-1915`, narrow/deferred reducers | `u32` coefficient rows, SIMD products and lazy `u64` accumulation | Keep these paths inaccessible to wide moduli regardless of environment modes |
| `f4_fp.rs:3985-4011`, visitor validation | Rejects `p >= 2^32` | Add deliberate wide entry points and validated dispatch rather than silently widening the existing API |

For canonical residues and `p <= 2^61-1`, each eager update has
`x + (p-f)*y <= (p-1) + (p-1)^2 = p*(p-1) < 2^122`.
Thus a `u128` product-plus-accumulator suffices when reduced immediately.
Unbounded lazy accumulation requires a different proof and is outside this
initial proposal. An explicit 61-bit ceiling also leaves sums of two canonical
residues below `2^62`; this does not justify arbitrary `u64` prime support.

Reusable helpers already contain relevant wide arithmetic:

- `f4_fp.rs:208-241`: `mulmod` uses `u128`; inversion has a non-Barrett branch.
- `f4_fp.rs:316-331`: monic scaling uses `mulmod` outside the narrow bound.
- `f4_fp.rs:345-370`: duplicate-term merging adds canonical residues. Its
  `u64` addition fits at the proposed ceiling, but needs separate review if
  that ceiling changes.
- `f4_fp.rs:3221-3272` and `3451-3505`: vector and packed polynomial reduction
  have modular-product fallbacks. Packed values describe monomials, not
  coefficient width.
- `f4_fp.rs:4091-4100` and `4277-4288`: specialization and evaluation use
  modular products; evaluation's final residue addition fits at this ceiling.

## Roots, traces and exact serialization

`src/cryptanalysis/glv_invariant_base.rs:1164-1240` implements counted,
deadline-aware rational-root extraction using `gcd(x^p-x,f)` and seeded
splitting. Its polynomial helpers at lines 1078-1146 call the `u128` modular
multiply/add helpers at lines 96-106. The directly used power/inverse routines
in `src/cryptanalysis/residual_walk.rs:193-237` do not impose a narrow-modulus
backend. These are reuse candidates, not new wide-prime correctness evidence.

Validate prime modulus, dimensions and canonical coefficients at the public
wide entry points. Zero, composite or out-of-range moduli must fail before
inversion, normalization or root extraction. Preserve exact domain maps and
the original-equation candidate check. Root deadlines remain cooperative:
polynomial exponentiation/GCD work can occur between checks. A timeout must
never become an empty-root result; polynomial degree, memory and worker time
limits remain necessary.

`f4_fp.rs:541-595`, the useful-row contributor calculation, already uses
`u128` product-add expressions. `F4Trace` holds monomials, basis indices and
multipliers; its `u32` values are exponents. `F4Report` stores basis coefficients
as `u64`. Retain these exact representations in recursive events and replay.
Contributor calculation materializes additional matrices and performs extra
elimination; its full cost is not the ordinary elimination counter, and it
does not check a deadline within every operation. Bounded event serialization
does not bound native or trace-recording matrix memory.

There is no Montgomery representation in this F4/root path. A first eager
reference backend need not introduce one. Adding Montgomery arithmetic would
require explicit conversion contracts at normalization, trace, evaluation,
root and witness boundaries. A specialized Mersenne reducer can be a later
independently checked optimization; the requested corpus includes other primes.

JSON can encode exact `u64` integers, but 61-bit coefficients exceed binary64's
exact-integer range. Require exact-integer parsing or decimal strings throughout
future ML conversion. A JavaScript-number round trip would lose information.
Raw solver events remain restricted artifacts; widening arithmetic does not
create an admissible public equation graph or decision-prefix feature view.

## Minimal separate path and counted units

1. Add explicit wide native entry points for ordinary, recording/replay and
   traced-visitor use, with a documented prime range through `2^61-1`.
2. Share symbolic preprocessing and pair scheduling through an immutable
   per-call arithmetic choice. Preserve narrow defaults and their guards.
3. Use canonical `u64` rows with eager `u128` sparse reduction, normalization
   and RREF. Force narrow/deferred/SIMD coefficient reducers off for this path.
4. Retain independent group lifts, equation/domain reconstruction, trace caps,
   sticky sink failures and distinct timeout/positive/encoding-complete states.
5. Record a separate backend identity and counter definition. Keep elimination,
   contributor tracing, root work, domain tests and group additions separate.
   Existing stage counts are not retired instructions; a 128-bit remainder may
   have a different machine cost without changing a mathematical operation count.

The affine-chain exception remains: intermediate infinity branches are not all
encoded. Wide arithmetic therefore does not authorize general negative PDP
claims, completed ECDLP recovery or exact first/last fall-degree labels.

## Independent acceptance checks before enabling native wide cells

- Compare modular multiply, full product-add updates, powers, inverses and
  normalization with independent `BigUint` arithmetic. Include `0`, `1`,
  `p-1`, cancellation, repeated updates and values above `2^32`.
- Compare sparse reduction and RREF against an independent dense reference
  on empty, rectangular, rank-deficient, inconsistent and randomized matrices.
- On small primes, compare narrow and wide backends on identical systems:
  canonical bases, independently enumerated solutions, useful-row replay,
  degree truncation and every stop state.
- At actual 47-61-bit primes and `2^61-1`, use independently constructed
  known-root and refutation systems. Where completeness is claimed, check
  input reduction and S-pair reduction as well as returned witnesses.
- Check linear, split, repeated-root, irreducible and mixed-factor root
  problems against independent evaluation; retain cancellation as timeout.
- Check exact coefficient round trips above `2^53`, trace tampering, sink
  failures and forced environment arithmetic modes. No wide call may enter
  a coefficient-truncating or unchecked product path.
- Exercise actual curve/twist fixtures with private witnesses absent; replay
  positive point lifts independently and retain all unsupported/censored cells.

Only executed, source-bound acceptance evidence can replace the current
unsupported-wide-prime outcomes. No performance comparison or broad corpus
completion claim follows from this inventory.
