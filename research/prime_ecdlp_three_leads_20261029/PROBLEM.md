# Experiment execution and qualification impediments

Raised 2026-10-29. These are missing execution capabilities and proof bridges,
not evidence against the mathematical leads. The three research tasks stay open.

## Durable checkpoint transport

The repository's `AGENTS.md`, "Resume checkpoints in S3 before every run",
requires Codex to run `scripts/checkpoint.sh gate` and not launch if it fails.
The first gate failed because no completed resume record existed. After
initializing it, `scripts/checkpoint.sh push` reported no configured AWS profile,
no `CHECKPOINT_UPLOAD_TOKEN`, and no checkpoint token file. Environment status
reported no configured AWS identity or secret bindings. GitHub authentication
is available but does not supply checkpoint/S3 transport.

Exit condition: provide the approved upload token through environment secret
configuration or an authorized AWS profile; fill and upload the resume record,
then require the unchanged gate to pass before any measured launch. No secret
value belongs in Git or in the report. No quota, IAM or infrastructure change
has been made. The absent Conductor binary was recorded; `docs/CONDUCTOR.md`
documents the warn-and-allow setup fallback, so no shared reservation is claimed.

## Five-sum independent replay

At source `42e460293269d149f4ead6929476fcc993babfea`, P20's manifest expects
`records.jsonl`, SHA256 `2e24438edf33b233c5f06f0fc0b2f9a9dac3c55ff23eda30d7bc0cf4916e7b02`,
1,622,069 bytes. It is store-backed, not present in the Git tree. An ordinary
unauthenticated GET to the configured artifact bucket returned HTTP403.
No independent replay, headline recomputation or validation admission is claimed.

Exit condition: authenticated read-only hydration of every frozen session with
size/hash verification, exact source/binary provenance, then the full native
replay and independent scalar/recomputed-cost checks under `NEXT.md`.
The original one-host study and its records remain unchanged.

## PKM qualification

The added oracle completes bounded extraction but does not establish complete
cost accounting or outperform an external solver. It exposes unrestricted
field branching; such a run may remain enumerative. Frozen future measurement
must include target-specific construction, factoring, failed/refuted calls,
point lifting, signs, verification, memory and every fallback on both arms.
No proof of bounded solving degree has been found. A plateau on the tiny
fixtures would not prove that property either.

The intended stronger target would quantify over tower depths and public
targets, define the precise symmetric ideal and elimination strategy, prove
successful point extraction, and give a bound below the complete matched-rho
cost. The current Lean dependency graph has no definition of that ideal's
filtered closure or of the Rust F4 execution/cost semantics. A constant first
fall degree does not fill either gap. The checked membership theorem is kept
as a dependency; it is not substituted for this stronger target.

## Hecke qualification

The added generic control accepts an arbitrary public target and certified
labels. The finite D79 source rows do not themselves supply a target relation.
Missing: a native importer and independently certified coordinate change for
X0(11), the exact class-label certificates, a prospectively frozen target panel,
and a non-generic target attachment to compare with the generic shift control.
The Lean recovery theorem exposes this attachment as a premise. It does not
prove it exists or that its cost is small.

Recheck these conditions at resume. Preserve their distinction from negative
experimental evidence; do not report any blocked panel as a completed null.

## Broader software-validation limit

All eight new example tests, strict scoped lint and Lean checks pass. The full
library run was capped at40minutes with5,232 unique passes, zero failures,
123 ignored and one unchanged existing JV isogeny-walk case unfinished. Cargo
exit101 reflects the explicit SIGTERM; no complete library pass is claimed.
Two initial sparse-input problems were restored and their focused checks pass.
The committed excerpts and raw-file digests are in `validation/`. Complete
outputs are NOT_UPLOADED: their required artifact-store transport is unavailable.
They remain local at the paths in `validation/unuploaded-artifacts.json`. This is a
local software-validation limit, not negative experimental evidence about PKM,
five-sum or Hecke, and the existing test was not modified or waived.
