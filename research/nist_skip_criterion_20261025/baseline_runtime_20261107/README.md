# Native baseline runtime development

The versioned runtime now uses PARI's documented bounded stack growth. Its local
minimal container completed the existing 46 native and seven CLI controls, eight
new launcher refusals, and full-order checks against one already published curve
in each of the six frozen fields, including 521 bits. These are unsigned local
development observations; they do not establish EKS/CRIU qualification or supply
any random-curve baseline sample. [pcc:unverified]

The original fixed 256000000-byte stack failed on the 521-bit known fixture after
matching the other five fields. That raw failure is retained under
`review-evidence/container-v3/` and claim `PCC-20261107-d07e05` (unverified,
not_covered). The final `container-v4/` has zero process exits, no OOM, and all
six known-order matches. It retains the PARI warning when the stack grew to
512000000 bytes. The adapter caps growth at 1073741824 bytes. [pcc:unverified]

## Scope and remaining work

| Requested component | Current state |
| --- | --- |
| 10000 accepted curves per six fields, 60000 total | Unrun; primary nonce unset |
| All ten published-curve percentiles and cutoff trajectories 100–5000 | Retained in TARGET.json and original design; no new percentile result |
| Thin native command payload plus full-journal reader | Implemented; refusal controls complete, nominal cloud invocation unqualified |
| Minimal image and six-field known-answer control | Local development only; image unpublished |
| Command-mode CRIU/EBS integration | Maintained contract identified; actual worker-loss qualification pending |
| Funded cost pilot, canonical primary execution/export and analysis | Pending before primary freeze and nonce |

The user's request is the full accepted-curve study. A ten-accepted-per-field cost
pilot remains a separate prerequisite and never replaces or contributes to its
60000 accepted observations. All attempted seeds, rejections, duplicates and
partial failures must remain in each contiguous journal. The original statistic,
acceptance law, ten-test Holm family and exact uncertainty rules are unchanged.

`COMMAND-ADMISSION.md` records the concrete remaining gates. The numeric total
dollar cap remains unanswered. Normal-chain local STS reports Unable to locate
credentials; a local checkpoint token is absent. Existing main-only OIDC remains
a possible route, but a reviewed native workflow, authenticated image/runtime,
successful allowlisted checkpoint, actual restore receipt and current shared
budget/capacity are not supplied by this work. No paid build/job, registry push,
artifact-store upload or fresh study nonce occurred.

## Source and reproduction

The immutable parent is `fc8f438c9261a6ba5d6cb5b97af7c3bdc70c99d7`, from PR 3610.
Original November 4 native sources, cohort, old preparation/custody files and
claim proof subjects stay unchanged. New `*-runtime.cpp` adapters replace only
initialization through `pari-runtime.hpp`; they do not edit the old instruments.
The local compiled source is bound by file hashes, not mislabelled as that parent
commit. A future cloud build must come from the actually published reviewed code.

From the repository root, with the installed native libraries recorded in the
preparation receipts, use three **fresh** absolute directories:

```sh
bash research/nist_skip_criterion_20261025/pilot_qualification_20261107/prepare-layer.sh /ABS/FRESH/STATIC
bash research/nist_skip_criterion_20261025/baseline_runtime_20261107/prepare-container.sh /ABS/FRESH/STATIC /ABS/FRESH/CONTEXT
bash research/nist_skip_criterion_20261025/baseline_runtime_20261107/qualify-local.sh /ABS/FRESH/CONTEXT /ABS/FRESH/EVIDENCE
```

The staging script compares retained source/library/layer hashes and builds
the new adapters. It strips only fresh binary copies and retains pre-strip
hashes. The scratch image contains explicit shell tools, their recorded ELF
dependencies, native binaries/source and PARI data. It copies no home directory,
credential configuration or paper archive. Runtime files and native payload are
hash-bound. This userspace is tied to the recorded host ABI; no general portable
image or DNS/dlopen-path qualification is asserted. Static glibc warnings remain.
[pcc:unverified]

Controls use two CPUs, 2 GiB RAM, UID 1000, read-only root, no network or Linux
capabilities, and fresh mounted output. The journal group has a 180-second local
limit; the corrected known-order group has a separately recorded 300-second
limit. These are development stop conditions, not production estimates. The
script retains stdout/stderr, exit/OOM state, commands, image config identity,
source/payload/output hashes and fixture files, then removes only its containers.
RepoDigests is empty: the config digest is not a published registry digest.

`run-field` accepts a mode, one frozen field, a lowercase nonce and the explicit
checkpoint acknowledgement. It requires the recovery supervisor marker and its
existing owned output directory, refuses overwrites and AWS identity inputs,
runs the producer once, then the separate reader on every mapped curve. Markers
are interface checks, never checkpoint, budget or launch admission. CRIU must
restore the existing process tree; this script adds no automatic retry.

## Retained failures and evidence

The first AWS CLI based image exhausted local disk before controls started.
Its build logs and transcribed driver errors are retained as container-v1/v2;
the generated staging/this isolated checkout were moved intact to `/tmp`.
Only this task's exact image digest and ten inspected cache IDs were removed.
No unrelated worktree or cache was pruned. A minimal staging attempt then exposed
an awk dependency parser error hidden by process substitution: its incorrect
zero exit, stderr and original script are retained. Checked sequential extraction
replaced that path before any control used it. Version 3 exposed the real PARI
stack failure; version 4 contains the affected corrected control run.

`DEVELOPMENT.json` and `MANIFEST.sha256` are this version's historical subjects.
Do not rewrite them after later source changes: add versioned/current receipts.
PCC statements remain unverified with proof not_covered; no producer hash, replay
or self-written JSON is promoted to a signed execution proof. Cargo is absent
(exit 127), with no Rust pass claimed. The backlog remains blocked and unfunded.

## Sources checked

KB unavailable: no KB MCP and wrapper exit 127; Conductor check also exits 127,
with no reservation claimed. Fallback sources were the original October 29
protocol, November 4 reader/recovery source and controls, November 7 static
preparation, current AGENTS, isolab-eks/report-evidence skills, ACCESS and cost
controls, and current native command recovery examples/controller/runtime pin.
The installed PARI 2.17.3 library manual `usersch5.tex`, stack-management section,
documents `paristack_setsize(rsize,vsize)` directly after `pari_init`, before
retaining a GEN; it destroys old stack contents. That placement guided the
adapter. No mathematical mechanism or novelty claim is made.
