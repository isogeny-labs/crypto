# p61 terminal closure successor protocol

- **Status:** frozen before terminal generation
- **Date:** 2026-10-23
- **Curve:** `p61-i0025`, over `p = 2^61 - 1`, registered model
  `y^2 = x^3 + 1055x + 16361`
- **Class:** fundamental discriminant `-8726545188371316595`, unconditionally
  certified class number `358094304`
- **Task:** Conductor `T-4942`

## Requirement-to-evidence contract

| Requirement | Acceptance evidence |
| --- | --- |
| Preserve the admitted census prefix | The exact predecessor checkpoint at `x=19890496` remains unchanged and is bound by SHA-256. |
| Correct the registered-root closure | The selected degree-13 successor has the seed j-invariant and the Velu codomain links over `F_p` to the model stored at `x=0`. |
| Preserve source identity | The terminal header, checkpoint, and generation receipt name the successor commit; the predecessor checkpoint retains its earlier source commit. |
| Finish the final range | One segment covers `[19890496,19894128)`, has `65,376` node records and `18` closure records, and emits a checkpoint with `next_x=19894128`. |
| Independently replay the terminal range | `verify-segment` recomputes every selected neighbor, kernel, target link, feature audit, record-chain digest, terminal state, and all closures with audit seed 7 and two audit points. |
| Fail closed | Missing or mismatched hashes, a non-closing j-invariant, a failed model link, an invalid kernel, a replay difference, or a resource-floor violation rejects admission. |

## Frozen predecessor and failure evidence

- Predecessor producer source:
  `c9bee1b28eda33b643ecf8b9a8b74724267d1cdf`.
- Predecessor verifier source:
  `d064729541f9c9cea2dc50b4a36572971cd2b099`.
- Predecessor checkpoint:
  `campaign-v1-checkpoint-000019890496.json`.
- Failed terminal partial SHA-256:
  `6b29341fd85d046decfcc4830b2b793d049f19895b712ec9923399f35edbaa5d`;
  byte count `8,304,984`.
- Repeated observed error: the producer advanced all lanes through
  `x=19894128`, then rejected degree-13 lane `(0,0)` because it compared the
  generic canonical representative with the separately registered root model.

The old partial and failed-attempt receipts remain immutable. They are not
eligible for admission.

## Hypothesis and controls

Hypothesis: the degree-13 action returns to the root j-invariant after
`19,894,128` steps, and the observed rejection is solely a representative
mismatch. The terminal certificate must link the computed Velu codomain to the
stored seed model directly.

Controls:

1. A unit regression proves that the registered root and its generic canonical
   representative differ as coefficient pairs, share the same j-invariant,
   and are `F_p`-isomorphic.
2. A checkpoint regression permits a source transition only when the complete
   predecessor checkpoint digest is the configured digest.
3. Generation starts from the existing `x=19890496` checkpoint; it does not
   regenerate or rewrite admitted prefix segments.
4. The replay path independently reconstructs all terminal records and rejects
   any closure certificate that differs.
5. Existing global uniqueness and adjacency gates remain separate and must not
   infer completion from this terminal-segment result alone.

## Execution and resource envelope

- Build the producer/verifier from a committed successor revision and record
  the binary SHA-256 and a Git bundle SHA-256.
- Run with 8 Rayon threads, `batch_steps=256`, construction audit seed 0 with
  one point, and replay audit seed 7 with two points.
- Require `/Volumes/data` to remain mounted, preserve at least
  `1,500,000,000,000` free bytes, and dispatch only when one-minute host load
  is at most 12.
- Preserve stdout, stderr, exit codes, elapsed times, host/runtime metadata,
  input/output hashes, the unsuccessful predecessor attempts, and every
  admission decision.

## Decision and stop conditions

- **Pass:** generation, replay, exact range/count checks, digest bindings, and
  terminal checkpoint checks all pass. Admit the segment with a new immutable
  manifest and resolve the implementation obstruction.
- **j-invariant mismatch:** reject the hypothesis and retain the terminal row
  order as unresolved.
- **target-link or kernel failure:** reject admission and investigate the
  isomorphism/kernel construction; do not weaken the closure gate.
- **replay or digest mismatch:** reject admission and preserve both artifacts.
- **resource or infrastructure interruption:** classify it as operational,
  retain partial output, and resume only through the hash-pinned gate.

This run is a correctness and accounting correction. It does not measure an
ECDLP method comparison or change any end-to-end hardness conclusion.
