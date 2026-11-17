# General Euclidean division with a short remainder product

T-1834 · 2026-11-07. Derived transformation and exact native producer checks;
no timing measurement or independently signed theorem.

## Sources and scope

Primary reference: [FLINT Newton quotient and remainder documentation](https://flintlib.org/doc/nmod_poly/index.html#division),
which describes quotient reversal and computing A-BQ by a short product. Read
predecessor feature-native-step-matrix/polynomial.rs, its matrix derivation,
and the earlier monic_remainder implementation. KB unavailable: no MCP and
repository wrapper missing executable. This extends a standard short-product
method already used by the monic evaluation path; no novelty claim.

Retain caller-certified prime p, 3<=p<=u64::MAX/2, and canonicalization of input
coefficient arrays. Source subgroup and prime/coprime/word-size assumptions
are unchanged. No source scalar labels enter runtime. Nonmonic divisors are
included: only their leading coefficient must be a field unit. Zero divisor
errors and lower-degree dividend returns retain prior behavior.

## Quotient and high cancellation proof

For nonzero a,b with degrees m>=d, let k=m-d+1 and define rev_j(v)=X^j v(1/X).
The unchanged quotient kernel computes

    rev_(m-d)(q) = rev_m(a) * rev_d(b)^(-1) modulo X^k.

The constant term of rev_d(b) is the nonzero leading coefficient of b, so its
truncated inverse exists. Multiplication gives

    rev_m(a) - rev_d(b) rev_(m-d)(q) = 0 modulo X^k.

The left side is rev_m(a-bq). Its lowest k coefficients vanish, so the highest
k coefficients of a-bq vanish. Hence deg(a-bq)<d. Reversal, padding and trimming
in the implementation preserve precisely the lengths in this identity.

Consequently, with q fixed, the remainder is

    r = (a mod X^d) - ((q mod X^d)(b mod X^d) mod X^d).

The implementation computes this low product and subtracts only indices0..d-1,
zero-extending omitted canonical coefficients. No full q*b is needed. If d=0,
the quotient is retained and the remainder is [0] without any residual product.
For zero a and constant b the unchanged quotient also gives zero. If deg(a)<d,
return ([0],a) before the quotient kernel, as previously. Empty/zero divisor
representations still canonicalize to [0] and return Error::ZeroDivisor.

The old full-product path also checked that the reconstructed high coefficients
vanished. Production now relies on the exact quotient identity instead of
reconstructing them solely to recheck it. Tests explicitly reconstruct every
coefficient and verify high cancellation against the predecessor. Internal
quotient-kernel correctness remains a material dependency of this optimization.

## Cost and storage accounting

The quotient/Newton work and its scalar inversions are unchanged. Only the
residual multiplication/subtraction changes. For m>=d>0, the old nonzero q*b
returns m+1 coefficients. The new residual product returns at most d coefficients
(and a canonical [0] if it vanishes), followed by a d-coefficient subtraction.
It does not materialize the high m-d+1 output coefficients of q*b or the full
subtraction vector. The quotient, reversed inputs, series inverse and recursive
mul_low temporaries remain; this is not a total peak-memory bound.

In the schoolbook residual branch, let U,V be the canonical trimmed lengths of
q mod X^d and b mod X^d, respectively. A zero operand uses zero products;
otherwise the exact field-product count is

    sum_(i=0..U-1) min(V,d-i).

It is at most the old len(q)*len(b) full schoolbook count. Degree-zero divisors
remove the residual product altogether. General recursive mul_low costs depend
on its actual split and coefficient cancellations; no universal strict product
saving or wall-time ratio is inferred from the finite panel below. Class powers,
geometry, division-value schedule and recovery costs remain separately charged.

Both Euclidean half-GCD update sites keep their exact quotient and remainder,
so the owned matrix row action, transition degrees/leading coefficients and
resultant outputs are preserved. General matrix composition is unchanged.
The earlier local matrix savings assertions remain valid. Its old full-resultant
count-difference assertion isolated matrix changes only, so this successor
retains full-resultant output/inversion comparisons and audits division counts
separately; the historical test and evidence are unchanged.

## Exact software validation

validation-01 passes:

- 2,592 division cases over p=5,17,97,257, dividend degrees0..96 and divisor
  degrees0..32 from fixed degree lists. Cases cover constants, monomials with
  zero low part, exact divisibility, trailing-zero trimming and noncanonical
  coefficients. Every pair equals the unchanged predecessor, reconstructs
  a=q*b+r and verifies the omitted high-coefficient cancellations.
- 288 constant-divisor cases;680zero remainders;1,220 exact schoolbook retained-
  pair count controls. Inversion counts agree in every division case.
- Poly::fm count comparison on that fixed panel:1,792lower,800equal,0higher.
  These are deterministic operation-count checks on the supplied inputs, not
  an external benchmark, timing result or universal distribution claim.
- 32zero-divisor controls and4empty-dividend controls.
- 288owned matrix checks and432complete resultants (all exercise half-GCD,
  including47zeros) retain exact outputs/inversion counts. Local matrix buffer
  reuse and identity-product savings controls remain.
- Inherited8320singleton cases;1108sourcepoints/24windows;160660division-value
  equalities;4100typed-lifetime plans and8prepared-source controls.

recovery-01 preserves617targets/5929calls/zero unresolved and63public comparisons
with511calls. normal-build compiles seven native dependencies without audit cfg.
Original4563cohort unchanged. No cloud, timing, external benchmark or full Cargo
rerun; prior full-suite missing-fixture/deadline evidence remains in the log.

Reproduce with `python3 generate.py`, `python3 validate.py NEW_DIRECTORY` and
`python3 validate-recovery.py ANOTHER_NEW_DIRECTORY` from this directory.
Never overwrite saved outputs. Receipts pin commands, compiler and source/output
hashes. remainder-checks.rs is a separate hand-written validation input. Normal
build command is in its receipt. Compiler warnings are retained unchanged.

Next: quotient reversal still computes and stores arrays at quotient length
m-d+1, while balanced half-GCD quotients are often short. Investigate a formally
priced small-quotient path and its crossover with Newton reversal, including
nonmonic leading inversions and all phase costs before choosing dispatch.
