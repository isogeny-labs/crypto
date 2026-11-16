# GHS descent with magic number two or more is not executable

- **Status:** `open`
- **Kind:** implementation gap (smooth model and norm-conorm point map); resource limit (no index calculus on a genus-`g` binary Jacobian). Not a proved obstruction.
- **Affected work:** [transfer-audit](../research/transfer_audit_20261011/PROTOCOL.md) candidate family A (every `ghs:l=<l>` row with `m >= 2` ends `arithmetic`/`not_evaluated`), [GHS transport gates](../docs/GHS_TRANSPORT.md), [`ghs_descent.rs`](../src/cryptanalysis/ghs_descent.rs) (`descend_m2_affine`, `descend_m2_abstract`)
- **Next action owner:** unassigned
- **Last updated:** 2026-10-26

## Problem

We cannot execute a Weil/GHS descent of an ordinary binary curve `E/F_(2^n)` with GHS magic number
`m >= 2` on a concrete generator, so we cannot certify the subgroup-preservation obligation
(`phi(G)` has order `r`, `ker phi ∩ <G> = 1`) or count relations for any such transfer, because the
repository stops at the abstract Artin-Schreier tower and an affine model for `m = 2`. The smooth
hyperelliptic model `C/F_(2^l)` of genus `2^(m-1)` (or `2^(m-1) - 1`) and the norm-conorm map
`E(F_(2^n)) -> Jac(C)(F_(2^l))` on points are missing, and the repository has no index calculus on a
binary Jacobian (`hcdlp_bsgs` and `hcdlp_pollard_rho` only).

The `transfer-audit` tool prices these candidates by formula and certifies what it can: the exact
magic number and genus, MMT Lemma 6 condition (8), the Hasse-Weil capacity gate (the descent
Jacobian must be large enough to hold `r`), and the class-wide set of achievable magic numbers. All
four deployed binary curves audited there (sect113r1, K-163, B-163, ECC2K-130) have prime `n`, so
their only tower is `F_2 <= F_(2^n)`, with genus `>= 2^28 - 1` (sect113r1) or the trace map killing
`<G>` (Koblitz); the missing construction does not change those verdicts. It does leave every
composite-degree binary curve unexecutable for `m >= 2`.

## Evidence and uncertainty

- **Observed:** `docs/GHS_TRANSPORT.md` ("Remaining construction gates") and
  `src/cryptanalysis/ghs_descent.rs` (module header, `descend_m2_affine`) state the gap;
  `transfer-audit` records it as an open obligation on every `m >= 2` candidate.
- **Derived or proved:** not established here. The existence of the descent follows from the GHS
  construction (Galbraith-Hess-Smart 2002; Hess 2004, as cited in `ec_trapdoor.rs`); injectivity on
  `<G>` for a given curve needs the explicit map or a theorem with checkable hypotheses.
- **Suspected explanation:** none needed; this is missing code, not an unexplained failure.
- **Missing capability:** (1) construction of `C` from the `F_2`-span of the Frobenius orbit of
  `sqrt(b)` (Hess 2004, Thm 5.3 as named in `ghs_descent.rs`); (2) the conorm-norm map on points
  with the divisor reduction in `Jac(C)`; (3) a binary-Jacobian index calculus (Gaudry / Diem /
  Gaudry-Thome-Theriault) to count relations at toy size.
- **What this evidence does not establish:** nothing about security of any curve. Formula-level
  prices for `m >= 2` rows are lower bounds without the construction cost.

## What would unblock it

- [ ] A smooth model of `C` and the norm-conorm map for one toy tower with `m = 2` (for example
  `F_(2^6)` over `F_(2^2)`), with `phi(P + Q) = phi(P) + phi(Q)` checked on all point pairs and
  `phi(G)` of exact order `r` on an enumerated toy subgroup.
- [ ] The same for one tower with `g >= 4` and a Jacobian large enough to hold `r`, replayed by a
  second implementation.
- [ ] A binary-Jacobian relation search whose total group operations are counted in the unit
  `S = ops / sqrt(r)` against rho on the same toy instances.

Meeting the first two would let `transfer-audit` mark GHS rows `verified` at toy size; the third
would put a measured destination cost beside the formula. Deployed-size claims would still need the
scaling argument.

## Next discriminating step

- **Action or proof obligation:** implement the `m = 2` smooth model and norm-conorm map over
  `F_(2^6) / F_(2^2)` and run the generic checker of `transfer-audit` (`subgroup.rs`) on it.
- **Inputs and controls:** exhaustive enumeration of `E(F_(2^6))`, the trace branch as a control
  (`m = 1` towers already pass), a Koblitz curve as a negative control (`G` killed).
- **Execution limits and stop condition:** toy size only; stop when the map is a homomorphism on all
  pairs or a counterexample is found.
- **Outcomes and next actions:** success gives the first executed `m >= 2` certificate and a target
  for index-calculus counting; a counterexample means the construction was misread and the
  affine-model note in `ghs_descent.rs` needs a correction.
- **Recheck trigger:** a new GHS construction in `ghs_descent.rs`, or a request to audit a composite
  degree binary curve.

A planned step is not a completed experiment.

## Resolution and history

- 2026-10-26: recorded while building `transfer-audit`; unresolved.
- 2026-10-26: wording correction after review of the merged tool: "three deployed binary curves"
  listed four (sect113r1, K-163, B-163, ECC2K-130); now "four". Also added to the tool: every
  candidate certificate carries an `evaluation` field (`evaluated`, `excluded` or `not_evaluable`),
  so a composite-degree `m >= 2` route whose priced part is below the baseline reads
  `not_evaluable` (the verdict `no weakness found in scope` is vacuous for it) instead of looking
  like a negative result. Status unchanged: open.
