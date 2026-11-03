# N12 held-out original executable lacks a source build binding

- **Status:** `open`
- **Kind:** evidence and implementation provenance gap
- **Affected work:** [N12 held-out study](../research/factor_base_heldout_20261025/REPORT.md), [independent-host replay](../research/factor_base_heldout_replay_20261025/PROTOCOL.md), and any claim that the original EKS executable was built from the reviewed source
- **Next action owner:** unassigned
- **Last updated:** 2026-10-25

## Problem

The original EKS evaluation returned 420 candidate and 420 matched-rho source-verified results, but its executable is not yet tied by an independently checkable build receipt to an exact tracked source tree, dependency lock, toolchain, and build command. The earlier attribution to commit `b2bd9f963b87f6919005067c810cba707b873f59` cannot provide that link because that commit does not track the study's `heldout.rs`. The saved study source and lock match later commit `a5798517feaac879bce582ff1f0d864a91daa238`; matching those files does not establish which source produced the original EKS executable.

This gap limits the implementation provenance of the original seven-model, 420-target cost study. It does not invalidate the independently verified scalar outputs or turn the recorded candidate/rho cost loss into a gain.

## Evidence and uncertainty

- **Observed:** The [study report](../research/factor_base_heldout_20261025/REPORT.md) retains 420/420 source-verified evaluation pairs, an independent validator, the raw records, graphs, and a source archive, and explicitly leaves source-to-binary provenance open. The [replay protocol](../research/factor_base_heldout_replay_20261025/PROTOCOL.md) identifies the frozen input archive and a newly built replay executable from commit `a5798517feaac879bce582ff1f0d864a91daa238`. Its Pod verified the 11 frozen input members before replay. That independently tests the recorded outputs but is not a build attestation for the original executable.
- **Derived or proved:** The finite scalar and cost checks have their documented scope. No theorem or receipt currently proves that the original EKS executable was built from the reviewed source.
- **Suspected explanation:** The source attribution may have been recorded before the study files were committed. That history does not identify the binary's exact build inputs by itself.
- **Missing capability:** An immutable original-job binary digest and a matching build receipt covering source tree, lockfile, toolchain, build image, flags, and the produced executable, or a reproducible clean build whose output matches the original digest.
- **What this evidence does not establish:** It does not establish malicious code, a wrong numerical result, a wall-speed improvement, or a general algorithmic claim.

## Exit criteria

- [ ] Recover and hash the executable bytes that the original EKS evaluation actually ran, with the original Job or bundle receipt that names them.
- [ ] Bind that digest to an exact tracked source revision, `Cargo.lock`, toolchain/build image, command, and flags through a retained build receipt, or reproduce the same executable digest from those inputs in a clean environment.
- [ ] Independently check the receipt against the archived source bundle and publish the comparison, including any mismatch or missing input, without rewriting the frozen run records.

Meeting these conditions would establish the original executable's source custody under the recorded build assumptions. The [separate wall-isolation problem](n12-heldout-wall-isolation.md), null wall ratio, and negative cost result would remain as reported.

## Next discriminating step

- **Action or proof obligation:** Audit the original EKS Job and source-bundle receipts for its executable digest and build log, then compare them with the tracked `a5798517...` source and lock. Start with read-only saved records; do not rerun the 420-target study merely to repair attribution.
- **Inputs and controls:** Use the exact original Job UID, input archive SHA-256, binary digest, source and lock hashes, and the independently recorded replay bundle. Keep the newly built replay executable distinct from the original executable.
- **Execution limits and stop condition:** Respect the existing Conductor attempt and research job budgets. Stop if the original binary or a required build input is unavailable; record the absent link instead of substituting a later binary.
- **Outcomes and next actions:** An exact build match closes this provenance gap. A mismatch requires a source-level investigation and claim correction. Missing original bytes leaves the gap open while preserving the finite replay findings.
- **Recheck trigger:** A recovered original binary, build receipt, or verified archived bundle.

## Resolution and history

2026-10-25: Opened from the independent validation deep dive. The corrected source-file match and independent-host replay are retained; original source-to-executable custody remains unresolved.
