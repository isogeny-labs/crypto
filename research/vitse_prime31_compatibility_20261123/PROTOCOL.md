# Frozen qualification protocol: Vitse / Prime31 compatibility

Date: 2026-11-23. Lane: `lane/vitse-prime31-compatibility`, PR #5016.
This protocol is fixed before any new compatibility research binary is executed.
It is a correctness and applicability audit, with no performance measurement,
new scalar-recovery run, alternate curve, or remote compute request.

## Fixed inputs and source

- Source baseline: crypto `7d61e3b45de6e93d235345a0061dae08db1f8b44`.
- Field `p=2147483647`; curve `y^2=x^3+847514846*x+2087444724`.
- Recorded prime order `n=2147446831`; `G=(625037715,2100627251)`.
- Raw selector: `x^1302=1`, degrees `[2,3,7,31]`, coset representative 1,
  construction seed 20261123. Reuse `pkm_smooth_tower::kummer_tower`;
  do not choose a selector after seeing its support.
- Test the identity, coordinate negation, reciprocal, and negative reciprocal
  in the ordinary x-coordinate. No Vitse coordinate change is assumed.
- Source reuse: the exact unchanged complete-item prefix of
  `src/cryptanalysis/pkm_smooth_tower.rs` preceding its unique
  `// ------------------------------- Method 2 with class number > 1 (Hilbert)`
  marker. This includes the existing field arithmetic, curve law, Kummer
  constructor and S3 evaluator. No production formula is rewritten.
- The standalone checker is intentionally fixed to this prime and receives
  caller-supplied coordinate sets. It must never certify a curve action.

## Ordered checks

1. Compile and execute all contract tests in `compatibility_tests.rs`.
   These include the public B8 coordinate fixture and explicitly synthetic
   finite-set controls in the same field. B8 is not a full factor base.
2. Compile and execute `driver.rs`. Check the fixed curve is nonsingular,
   G is nonzero/on-curve, n is prime under the production bounded-u64 routine,
   and nG=O. Check the integer Hasse upper-bound condition using
   46341^2>p and p+1+2*46341<2*n. This is an arithmetic admission check,
   not an independently signed point-count certificate.
3. Construct all 1302 distinct selector roots. Check every root satisfies
   x^1302=1 and the production tower verifier. Since the defining polynomial
   has degree 1302, these distinct roots exhaust its field roots.
4. Lift every root using production square-root and curve routines.
   Retain exactly the coordinates having a rational point annihilated by n.
   Emit the entire raw and retained coordinate lists.
5. For each of the four fixed maps, audit both lists. Record every pole,
   every escaping image, closure and nonvacuous-bijection status.
   Do not hard-code a retained count or outcome before observing it.
6. Test equal retained leaf domains and a deliberately shortened fourth
   domain under explicitly supplied permutations. This checks membership
   exchangeability only; no chain or group-action certification follows.
7. Fixed-target relation control: R=G, P=2G, Q=-G. Check P+Q=R and
   S3(xP,xQ,xR)=0; swap the leaves as a positive control. Evaluate the
   proposed coordinate maps against the same fixed xR. Record values and
   witnesses. These points are a public algebraic control, not an asserted
   decomposition over the 1302-root selector.
8. Point-negation control: simultaneous point negation gives -R != R,
   while its x-coordinates are unchanged.
9. Finite-prefix-chart control: compare (G,G,-G,G) with (G,-G,G,G).
   The former has finite represented prefixes; the latter has an infinity
   first two-point subtotal. Both have total 2G. This is a group/chart
   counterexample, not a selector-constrained extracted solution.

## Evidence and execution discipline

The metadata runner records full and selected source hashes, compiler version,
exact commands and environment. It writes a distinct build request and build
result, freezes each executable by copying it, hashes the copy, and writes a
no-overwrite pre-call manifest before execution. Raw stdout/stderr and a
post-call manifest follow. Source files, protocol and binaries are rehashed
after each call; any mismatch fails qualification. A failed build or call is
retained in its own attempt directory. Later attempts cannot overwrite it.

Compilation has a 180-second cap per target; each compatibility execution has
a 60-second cap. No CPU timing comparison or operation-count benchmark is
reported. At most one initial attempt and one corrected attempt are planned;
a further attempt requires an explicit written correction explaining the
remaining risk.

## Success and limits

Qualification means all contract tests and the fixed driver complete with
their internal correctness checks, and the reports faithfully preserve both
positive and negative observations. It does not require the candidate maps to
pass. A counterexample is a successful audit outcome.

The full root `cargo test --release --lib` gate is attempted separately on
the actual source revision with its original dependency/default-feature
configuration. Its result is not substituted by this scoped qualifier.
The Dickson selector, complete extension-field pipelines, arbitrary-coordinate
compiler, high-arity P_phi,8, Cartier and higher-torsion coverage are source
audit items here; no new execution of those systems is claimed.

Signed PCC review, exact current-head CI, and any formal Lean bridge retain
their separate status. Same-team review and native self-checks are unsigned
development evidence. No performance scoreboard receives a new number.
