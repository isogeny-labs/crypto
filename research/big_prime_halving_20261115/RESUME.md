# Resume: arbitrary-precision auxiliary point halving

## Goal
Continue the user's complete general prime-field ECDLP algorithm objective via
an approach different from tensors. This component ports the qualified point-
halving/isogeny setup to multiprecision; it does not supply relation solving.

## Lead / hypothesis
MIT18.783 Fall2023 Lectures2--5 and the parent's generated setup supply the
quartic and degree-two map. A separate OpenSSL polynomial gcd can certify
complete roots without enumerating a large field. Reject the implementation
on any disagreement; fixed controls do not prove an asymptotic algorithm.

## Spec
research/big_prime_halving_20261115/PROTOCOL.md, committed bd458ef4e2 before
implementation. Exactly one qualification001: ten rejected admissions,14754
finite preimages,18 setups seed1234 (eight small legacy, ten at61..521bits)
and four adverse transcripts. Native GMP/FLINT candidate; independent OpenSSL
BN polynomial/EC_GROUP replay. Source must be committed before calls.

## State
Shared source b38b6b4712 passes canonical proof gate106theorems/0failures/
0trusted native; source7364d8dab7 adds two root-certificate laws and the local
canonical gate passes108theorems/0failures/0trusted native. Shared PR9/10 are
merged, hosted jobs did not start because of the billing lock. Native build001
failed explicit-header/strict-layout compilation; build002 passes. No native
numerical qualification has started. Builds under /workspace/scratch/
big-prime-halving-build00{1,2}; diagnostics adjacent. Old controls preserved
at /workspace/scratch/aux-halving-qualification001. Critical PR4304 has one
Copilot journal merge request14:55:21Z; do not touch its journal while pending.

## Next steps
From /workspace/scratch/big-prime-halving, commit/push all native source and
immutable wrapper (already sourcef5b318b482), restore this saved resume to
.resume/RESUME.md and pass scripts/checkpoint.sh gate after restoring its
existing upload access. Ready PR4305 records the pending scope. Run
timeout1500 bash on copied
wrapper /workspace/scratch/big-prime-halving-runner001.sh with arguments
/workspace/scratch/big-prime-halving-build002
/workspace/scratch/big-prime-halving-qualification001
/workspace/scratch/aux-halving-qualification001. Capture outer log adjacent.
Inspect every exit, retain failures, archive whole raw round, publish report/PR.

## Runs in flight
None. The shared local gate is complete. No native case or paid job is running.

## Stop and success conditions
Local functional controls only under AGENTS9a, one thread2GiB, census120s,
each setup/reference30s, outer1500s. No paid budget, cap changes, seed changes
or retries. Unexpected control/reference mismatch stops the panel. Unknown
prefixes retained. Success is full native qualification at component scope;
do not mark the complete general algorithm objective done. Preserve all raw
bytes and source/wrapper/build manifests. Publish a regular PR with auto-merge
once its dependencies/review/required gates pass.
