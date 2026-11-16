# Adaptive decoding across sparse character fibers

2026-11-05. Native construction and exact finite correctness checks, not a performance panel.

## Why degree four is not the next fix for this window

The preserved `feature-fiber-repair/DERIVATION.md` establishes that the 15 signs chi(X+a), a=0,...,14, agree at 25 and173 and also at292 and440 in F479. Every function of those same signs agrees within each pair, irrespective of feature degree. Thus the proposed quartic extension cannot recover X on this narrower consecutive window. This is a fixed-input exclusion, not an exclusion for other shift sets. [pcc:unverified]

The present component instead asks whether the sixteenth sign can be deferred until the old signature is ambiguous. It does not search elliptic-point collisions or store a scalar-answer table. The pairs are supplied public coefficient-field witnesses from the preserved exclusion certificate; no unknown logarithm enters compilation.

## Exact pair-mean identity

Let delta_t(X)=1-(X-t)^(r-1). Over the caller-certified prime field this is 1 at t and 0 elsewhere. Set r=479 and

    F(X) = X + 74 delta_25(X) - 74 delta_173(X)
             + 74 delta_292(X) - 74 delta_440(X).

F equals X away from the four points, equals99 at both25 and173, and equals366 at both292 and440. It is therefore constant on the two exceptional fibers. The supplied native constructor finds F in the old cubic feature span, with471 terms and observed odd-sector rank238 among288 orbit columns. This is a solved membership instance; constant-on-fibers alone is not used as a general cubic-span sufficiency theorem. [pcc:unverified]

Let M(s) be 1 exactly when the first15-sign signature s equals either of the two supplied exceptional signatures, and 0 otherwise. The constructor exhaustively checks completeness of these two fibers over the bounded public coefficient field. Let u(X)=chi(X+15). The two pairs have u-values (+1,-1). Since405=-74 modulo479,

    X = F(X) + 405 M(s(X)) u(X).

Evaluate the first15 signs and F. When M=0, return F without querying u. When M=1, request the sign at offset15 and apply the displayed correction. The runtime object stores cubic coefficients, two sign predicates, and their correction coefficients. It does not retain the four witness scalars or a scalar-answer map. [pcc:unverified]

## General constructor and centered solve

The API accepts r, q and disjoint supplied pairs (a,b). It checks equal q-signatures within each pair, distinct pair signatures, complete fibers, and opposite nonzero extra signs at offset q. It constructs the target that replaces each pair by its field mean (a+b)/2. The correction for that signature is (a-mean)/chi(a+q). At runtime that coefficient times the extra sign restores either pair member. [pcc:unverified]

To reuse reflection-odd elimination, set c=(q-1)/2 and Z=X+c. The target is H(Z)=F(Z-c)+c. Construct each centered delta using the coefficients of (Z-t)^(r-1): binomial(r-1,k)=(-1)^k, so the coefficient at k is t^(r-1-k). Check every even coefficient of H is zero before solving its odd coefficients. The finite-field coefficient solver checks recomposition. It rejects parity failures and targets outside its cubic span rather than assuming all pair-mean targets are representable. Reconstruction subtracts c, producing F in the original offsets. [pcc:unverified]

Primality is caller-certified. r is odd, 3<=r<=u64::MAX/2; 1<=q<=63 and q<r. max_rows bounds coefficient-field enumeration and storage; max_columns bounds the feature solve. Pair membership and completeness validation are charged setup work. It enumerates public field elements but retains no field-sized answer table. Bad signs, incomplete fibers, duplicate points, absent separators and parity violations are rejected. The generator remains dense and can reject other valid-looking parameter sets.

## Public-point adapter

The adapter takes E/F_p, a public generator P, its prime order r, and Q. It obtains the first q signs through the existing native prepared source window. Only on a matching exceptional signature does it add P to the last returned shifted point and call the existing single-point sign source once. It does not recompute a longer window. It accepts no unknown target scalar. The scalar labels in targets.tsv are used only in assertions. [pcc:unverified]

The current adapter prepares the window state on each recovery call; source setup, signature comparisons, bitplane evaluation, affine point addition, and fallback direct-phase cost are separate from the sign-count identity. The original source uses E/F509 with a=2,b=10,P=(3,68),r=479. No curve or extension-field transfer is introduced.

## Recorded checks and cost scope

The optimized, overflow-checked native suite reports49 passing tests. Its complete F479 check recovers all479 inputs and requests the extra sign exactly at25,173,292,440. Eight public-point integrations, including all four exceptional inputs, pass and make four extra requests. Invalid limits/pairs/signatures/parity, invalid extra signs and propagated source errors are checked; a closure that panics confirms the extra oracle is not invoked on an ordinary signature. Twenty source/data hashes bind the run. Seven compiler warnings remain in build.log. [pcc:unverified]

The separate check.rs imports no producer, source, solver or decoder. It expands every term as an ordinary polynomial using repeated linear factors, checks the exact pair-mean polynomial, builds characters from a square bitmap, and checks the adaptive identity and trigger set at every field input. A coefficient-sign corruption is rejected at the polynomial check. This is a separate implementation in the same producing session, not a signed independent review. [pcc:unverified]

The derived sign-value count is15 for475 field inputs and16 for4 inputs. Summing over the full field gives7189 requested sign values. This counts requested character values, including cheap special values; it is not a count of field operations, direct phases or elapsed time. No distributional extrapolation or runtime speedup is asserted. No timing panel or cloud job was launched, and the original4563-case cohort remains intact. [pcc:unverified]

## Sources and next work

Prior internal evidence: `../feature-fiber-repair/DERIVATION.md`, `../feature-consistent-repair/repair-input.json`, `../feature-signature-selection/DERIVATION.md`, and the reflection/bidirectional constructor records. The quadratic-character definition and zero convention were checked in [NIST DLMF27.9](https://dlmf.nist.gov/27.9). Pair averaging, point indicators and conditional correction are derived explicitly here; no novelty-priority claim is made.

Next preserve reusable prepared source state and exploit the two retained seed pairs for an incremental exceptional sign, then freeze complete-cost validation before any performance comparison. For broader query reductions, vary the public shift set or construct other sparse residual fibers; raising polynomial degree alone cannot remove an existing signature equality.
