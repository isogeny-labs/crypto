# Upstream solver extension inventory

This is source inventory and implementation design, not experimental evidence.
The read-only audit compared local HEAD
`d30b0ade734ba557be600e50c53c3945c57d9b76` with fetched `origin/main`
`7ffaab06bad1ceb873e3296f24fbf83a869853d6`. The inspected upstream additions
were **not integrated** into that HEAD. No tests, benchmarks or cloud jobs were
run for this audit. The full campaign and remaining acceptance criteria remain
in [REMAINING-COVERAGE.md](REMAINING-COVERAGE.md).

## What the fetched changes actually add

The delta adds `f4_fp::solve_visit` and `hky_fp_tower`; it does not change
`ic_framework/pkm_semaev.rs` or `f4_fp_tower.rs`. Paths and API observations below
refer to those exact Git objects, not a future merged checkout.

| API or adapter | Source boundary | Useful extension |
|---|---|---|
| Upstream `f4_fp::solve_visit(input, n_vars, p, opts, domains, visitor)` | Prime `p < 2^32`; exact dimensions; canonical, duplicate-free domains | Stream original-equation-checked roots, preserve domains by original variable, factor unrestricted univariates, stop at a verified witness |
| Dataset `prime_f4` | `m=2`, affine target, 1..32 distinct domain abscissae, elimination only for `p < 2^32` | Add a separate S3-chain encoding for higher arities |
| `PkmSemaevOracle` and dataset PKM | Both enforce `m=2..3` and `p < 2^31`; oracle also requires `1 <= m*t <= 64` and `2^t` dividing `p-1` | Extend only after changing and validating the oracle and extraction, not by removing the dataset guard |
| `f4_fp_tower` | Odd `p < 2^32`, 64 tower variables, 4 free variables | Representation can fit a chain through `m=6`; current oracle/extraction still rejects `m>3` |
| New `hky_fp_tower` | `p < 2^32`, 56 tower variables, 8 free variables | Separate bounded degree-closure observations; chain representation through `m=10` subject to `m*t <= 56` |

The new visitor explicitly rejects `p >= 2^32`. The ordinary F4 Barrett
reducer has the same bound. None of these additions enables prime F4 or PKM
elimination at n=47..61 or `p=2^61-1`.

## Higher-arity generic prime F4

A chain has `m` leaf coordinates and `m-2` auxiliary coordinates: at most
28 variables for the requested `m=15`. Generic F4 uses exponent vectors.
Its `n_vars <= 15` condition selects a packed optimization with an unpacked
fallback; it is not a public variable-count ceiling. This makes a bounded
chain adapter plausible below `2^32`, not proven tractable at every arity.

Build and retain the actual S3 equations, variable roles, curve coefficients,
target, monomial order and domain restrictions. Supply `Some(domain)` only
for leaf coordinates and `None` for auxiliary coordinates. Do not give an
auxiliary variable the leaf membership domain. Preserve the existing m=2
adapter as a reference lane with its own strategy identity.

The visitor checks candidates against the original equations, including after
degree-truncated elimination. Its callback must independently lift the leaf
coordinates to recorded factor-base points and verify their sum against the
target. A successful callback establishes a positive PDP witness. It must not
turn a planted construction witness into an independent ECDLP recovery claim.

Affine-chain completeness needs explicit exceptional cases. Intermediate sums
can be infinity, and the relationship between rational auxiliary solutions and
the intended PDP problem must be established. Until these branches are handled
and checked, an exhausted chain search means no roots of that encoding, not
no PDP decomposition. `VisitReport.complete` is relative to supplied domains;
`stopped_early` and `timed_out` are distinct terminal states.

## Trace and measurement contract

`VisitReport` exports F4 runs, field operations, domain evaluations, factored
univariates, root-finder multiplications, full-field branches, rejected
candidates, dimensions and termination flags. Its documentation explicitly
calls these stage diagnostics, not complete accounting or a performance claim.
These counts are not retired CPU instructions.

The visitor does not expose complete recursive pivot, critical-pair or branch
artifacts. Switching from `f4_record` must not silently discard the existing
useful-row replay schedule. Add a bounded per-call trace sink, or retain both
strategies with explicit trace gaps. A trace cap is a recorded outcome, not
permission to call aggregate logs complete intermediates.

Keep native elimination, root finding, domain testing and group lifting as
separate counter scopes. Preserve original equations and exact witness replay
even when a timed-out algebraic phase later yields a valid bounded-search
witness. No speed claim follows from source eligibility or correctness tests.

## HKY closure and fall-degree labels

The new public APIs are `HkyRing::new`, `from_raw`, `count_monomials`, `closure`
and `fall_degrees`, with `HkyLimits`, `Closure` and `FallDegrees` records.
They operate on a tower quotient ring with ordinary free variables. A PKM
chain needs `m-2` free variables: m=15 therefore exceeds the eight-variable
representation even when the tower-variable count fits.

`Closure` returns dimensions, `contains_one`, censoring, rows, stored entries,
`muladds` and elapsed milliseconds. It does not return echelon rows or
derivation certificates. Add those artifacts and an independent checker before
promoting an output to a certificate-backed mathematical label.

The first fall returned is the first observed among computed degrees. Start
at degree zero, or independently justify every skipped degree. A nonzero
arbitrary `c_min` can miss an earlier fall. The returned `last_fall` records
the first reached closure containing one; calling it the least such degree
requires exclusion of every earlier degree. For consistent systems, `None`
is not an exact general last-fall-degree result. Bounded failures remain
censored/unknown, and instance results do not prove a universal conjecture.

Admission and measurement require additional checks before dataset integration:

- Enforce primality and canonical square-rule coefficients. The constructor
  does not validate all of these conditions itself.
- Bound total degree by 255 as well as individual free exponents: the packed
  key places total degree at bit 120 of a `u128`, despite a public `u32` degree.
- Require a nonzero batch size, and cap columns, queue growth, memory and time.
  Stored-echelon `max_entries` is not a complete process-memory bound.
- Reconcile the `muladds` definition with implementation: variable-product
  accounting increments by merged output row length, whereas the documentation
  describes terms before rewriting/merging. Preserve the actual counted unit.
- Keep quotient rules and field-equation choices explicit; labels from
  different rings or encodings are not interchangeable.

## Wide-prime work remains separate

The F4 reducer relies on products and updates fitting `u64`; tower and HKY
rows use `u32` coefficients; HKY also multiplies directly in `u64`. Removing
guards would be incorrect. A wide backend needs reviewed `u128` modular
arithmetic or a justified reconstruction method, wide rows, independent
arithmetic checks and counter definitions covering the new implementation.

The existing prime root adapter already builds S3 coefficients with `u128`
modular arithmetic and calls the counted prime root finder. It is a useful
wide-prime reference, not evidence for wide F4/PKM. At `p=2^61-1`,
`v_2(p-1)=1`; the current multiplicative Kummer base therefore permits only
t=1. Widening elimination does not create larger power-of-two Kummer bases.

## Prioritized verification after implementation

1. Compare serialized chain equations and roots against independent small-field
   evaluation. Check m=2 against the existing adapter, and m=3,4 against an
   independent group PDP enumerator. Include both twists, sign choices,
   repeated coordinates and intermediate-infinity cases.
2. Check empty domains, reordered auxiliary elimination, early witness stop,
   timeout and degree truncation. Every emitted witness must replay without
   private generation data. Negative labels must match the proved scope.
3. For m=5..15, test construction, bounded/censored execution and independently
   verified planted positive cases. Invocation alone is not solved coverage.
4. Cross-check HKY against independent dense closures on tiny quotient rings;
   check degree boundaries, censoring and certificate tampering before exact
   labels. Existing example checks are source references, not newly run evidence.
5. Validate a future wide backend at boundary primes including `2^61-1`, with
   independent reduction/root checks and both twists. Keep unsupported rows
   until actual supported implementation and replay evidence exist.

Freeze the resulting per-strategy protocol before comparative measurements.
Preserve these solver obligations alongside startup-gate and durable-launch
work; neither delivery path discharges the other.
