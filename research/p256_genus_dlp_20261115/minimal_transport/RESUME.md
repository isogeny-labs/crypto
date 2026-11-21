# Resume: P-256 minimal transport

## Goal
Find a charged transfer advantage for the original P-256 DLP objective. This
component studies shorter coefficients for the existing genus-3 finite
eigenline; it does not replace the pending complete-cost RunPod experiment.

## Lead / hypothesis
The prior adjoint matrix suggests the Eisenstein norm a^2-a*b+b^2.
Check its relation to equivariance, polarization degree and minimal primitive
coefficient vectors; retain precise generic/finite/geometric scope. Literature:
Smith1310.5250 and the Cornacchia section in eprint2021/090. Prior code is frozen
under research/p256_genus_dlp_20261115/{transfer_search,folded_walk}.

## Spec
research/p256_genus_dlp_20261115/minimal_transport/PROTOCOL.md on branch
codex/p256-minimal-transport-20261115, base6f1bd2d9c. Planned groups: orders109,
823,4111 and P-256 n from the existing parameters(). Exact constants are reused,
not substituted. Planned qualifier checks norm, eigenvector, recovery,
unit invariance, invalid inputs, map compatibility and public scalar replay.
No timing claim or numerical panel has run; no run is admitted yet.

## State
Read current instructions, lessons and primary sources. RunPod API returned401
at16:32Z, no pod allocated. No checkpoint token/ambient AWS identity or CLI found.
Native candidate release compilation and strict Clippy pass. Its build001
digest receipt is in checks/build-001.sha256. Numerical qualification remains
unrun. Shared generic proof source f74b0e8 passed115-theorem canonical audit;
the study passed6-theorem audit (four endpoints/two constant equations), with
standard axioms only. Shared PR12 merged. Lean4.35.0 and pinned Mathlib
cfc55fc58714a59dc06e6e79edc6d1d95140d3c2 are installed for proof checking.
The separate proof-record serializer issued/replayed three already-proved
polynomial identities; it imports no new map candidate code. PCC-20261115-
1bf4c0,aa17eb,9ff011 remain unsigned. Report/PDF and diagram are inspected.

## Next steps
1. In /workspace/p256-minimal-transport, copy this saved file to .resume/RESUME.md,
   run scripts/checkpoint.sh gate and retain its receipt under checks/.
2. Once checkpoint and provider access work, use the source-frozen candidate
   and original build001 binary. A rebuild requires distinct build002 metadata
   and a fresh qualifier002; never replace001's manifest/copy.
3. The immutable wrapper and its hash already exist under qualification-001.
   Execute that copy according to RUNPOD.md, with recorded binary digest and
   authenticated allocation id. It writes a new pre-call manifest and passes
   checkpoint gate before either candidate call.
4. Run the approved immutable qualifier on an authenticated RunPod CPU pod, with
   frozen source, 120s child caps, 30min allocation cap and USD1 total cap.

## Runs in flight
No numerical experiment, cloud job or pod is active. Proof builds/audits and
supplemental artifact serialization are finished; no live native qualifier exists.
No scientific child should exist until the checkpoint and provider dependencies
are explicitly resolved. No admitted performance or count result exists.

## Stop and success conditions
Stop numerical calls on missing checkpoint/provider access; do not use
CHECKPOINT_SKIP. Reject wrong congruences, altered answers, invalid divisors,
unbound inputs or hidden work. Initial build/proof budget30CPU-minutes; each
numerical child120s. Net speedup still requires independent replay, source/A-A/
generic/folded controls and complete cold one-target accounting.

## Open questions
Does the norm-minimal vector shorten actual mapping costs under the strongest
applicable source backend? Can any destination step beat the full overhead?
Global elliptic stability and finite-subgroup stability are separate obligations.
