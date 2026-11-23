# Independent monic remainder review

Reviewer `/root/monic_review`, 2026-11-01. Producer source head `f2075e7b23bcb41e28efb584a43e83c755dba150`. Scope: T-1196 Descent/Fuel and T-1198 Normalized/ZeroTest, their 16 controls, native `three_leads.rs` remainder path. No scientific execution.

## Results

The 19 main declarations and 16 producer controls freshly compiled in an isolated copy. Both readback modules compiled and produced 35 named axiom readbacks, limited to propext, Classical.choice and Quot.sound. The copied local source/binary inventory binds 124 producer inputs; all remained unchanged on post-read. Reused Mathlib dependency unions and imported compiled local helpers remain a trust boundary; this is not a clean rebuild of every dependency or an independent kernel rechecker. Commands, native exit codes and complete output are retained in receipts.json and per-module logs.

**Mathematical fidelity: faithful_scoped.** Canonicality connects a zero field coefficient to the actual zero word. Monicity makes the last coefficient cancel. The raw array preserves length; trimming removes at least its last zero. Hence each active iteration strictly decreases natural length, and induction gives termination with fuel at least the original input length (a sufficient, not tight bound). Empty initial input and constant-one divisor are included. The canonical normalized zero-list lemma needs neither prime nor positive-modulus assumptions itself: at modulus zero, canonicality prevents a nonempty input, so no contradiction is hidden there. The prime assumption supplies the field/domain properties in the main degree argument. The finite remainder is smaller in degree than its monic divisor; divisibility then forces zero, and normalized canonical encoding forces the empty list. Both directions are present and no conclusion is assumed as a hypothesis.

**Native-source fidelity: partial, with no discrepancy found in the scoped remainder-update structure.** Native pdiv lines 849-881 asserts nonempty divisor, trims initial input, detects last word one, selects last remainder word as multiplier, updates indices j+k in divisor order, and trims after each iteration. These operations match the model under odd prime-field n=1, diagnostic tables absent, canonical words and p<2^31. The Lean theorem itself also permits p=2; this is a broader mathematical model domain, not a claim that the Rust binary-field branch has been reviewed. Native code is unbounded while the model uses an explicit sufficient fuel; the descent argument explains the connection but no compiler/operational-semantics theorem is present. Native quotient allocation and q[j] writes, usize allocation/index bounds, counters, panic/OOM behavior and instrumentation are not modeled. This review does not promote model totality to unconditional Rust totality.

Native Jacobian.valid lines 972-980 checks degree<=2 and stored leading word one, then residual remainder empty. It does not itself enforce canonical coordinates or modulus assumptions; these remain caller obligations. Polynomial decoding/canonicality of native psub/pmul residual is not proved by these packets. Source-to-compiled equivalence and generalized nonmonic division are also outside scope.

## Boundary attacks and independence

Producer controls freshly replayed: insufficient fuel; nonmonic failure/rejection; canonicality counterexample [17] modulo 17; non-normal [0]; zero input and constant-one divisor. Four reviewer-added kernel controls test an internal-zero monic divisor, initially terminal input with zero fuel, p=2 in the model, and initially all-zero input with zero fuel. Their separate receipt and output carry the execution result.

This reviewer did not author the producer proofs or read sibling review reports. Parent-provided history contained producer claims; this is independent adversarial review and replay, **not blind re-derivation**. The plan was written before reading proof text, but is not a precommitted multi-reviewer plan. The repository was read-only as delegated; no merge, official research transition or independence-checker gate was performed here. Publication and full review-process completion remain the coordinator's responsibility.

## Primary references inspected

Pinned Mathlib Algebra/Polynomial/Degree/Domain.lean, lines 63-69: degree monotonicity under divisibility and zero conclusion below divisor degree. Producer reports and theorem source; current native three_leads.rs Field arithmetic, trim/pdiv/prem and Jacobian.valid. This is the standard polynomial-division argument specialized to the stored-word model; no novelty claim.

## Per-claim assessment

- Strict decrease and sufficient fuel: machine passed; faithful_scoped to stated model; no mathematical defect found.
- Empty remainder iff polynomial divisibility: machine passed; faithful_scoped to stated model; no mathematical defect found.
- Compiled Rust termination/acceptance equivalence: not_verified; partial bridge; needs further source/runtime work.
- Full independent-review workflow: incomplete (no blind derivation, no independence checker, no publication in this delegated directory).

Next useful work: canonical residual construction, index/allocation safety obligations for native quotient side effects, and caller domain enforcement. These are distinct from the proved remainder model.
