# Nonbranch independent precheck — 2026-11-11

**Status:** [pcc:unverified] technical precheck only. No compiler invocation, new attempt, theorem verdict, or scientific promotion. The review plan and exact input hashes were published in crypto PR #3859 before reviewer dispatch; the historical 3-of-3 attempt ceiling remains exhausted.

| Joint | Machine status | Independent finding | Boundary |
| --- | --- | --- | --- |
| Local sign/residue and false equality | not_verified | Blind reviewer derived residues 0 and -18, with the control equality false | Assumes the supplied local-ring unit criterion; no Lean execution |
| Corrected source versus archived theorem statements | not_verified | Source reviewer found six proof-script/indentation edits, no changed statement, definition, fixture, or control | Static comparison, not tactic validity |
| Thirty named axiom readbacks | not_verified | Source reviewer found 10 + 20 unique names retained | No generated axiom output |
| Selected-order/valuation identity | not_verified | Blind reviewer identified missing factor and norm premises | Requires pinned imports, compiler, axiom audit, and semantic review |

The [blind handout](blind-handout.md), [blind report](blind-review.md), and [source report](source-review.md) record separate inputs and verdicts. The editable [flow.svg](flow.svg) and inspected [report.pdf](report.pdf) visualize the boundary.

Reviewer isolation is attested by the two sessions; this packet does not include an applicable structured independence-checker run. The blind mathematical handout was sent directly and transcribed to a file after the response, so its exact session input is not established by a precommitted file hash. Those limits prevent a `validated_scoped` outcome.

The merged prospective plan fixes one local attempt of at most 40 minutes, with no retries or new EKS/instance use, but execution still requires separate owner authorization and fresh resource/budget admission. This packet supplies no such authorization. Current hosted gate batches 180 and 184 fail while compiling five archived pencil roots because their cross-directory imports are absent; the original native compiler result remains failed and the correction remains proposed_uncompiled.
