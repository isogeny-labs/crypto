# Continuation readiness, 2026-10-26

The results summary is published in existing private PR 2550, with explicit
owner authorization. This file records remaining work; it is not a new run,
frozen protocol, completed export or approval to bypass a failed gate.

## Independent guessing replay

The original quartic/HKY queues stopped across the workspace restart. No new
HKY case has run and the old completion marker must not be invented. Preserve
the original producer timeout and unknown cost independently of any new verdict.

The fixed work is all 124 `(draw,k,branch)` cases for draws 0,3,5,6, k=0..4 and
every branch. The frozen native auditor is
`/workspace/ic-frozen-20261025/ic_guess_completed_audit`, SHA-256
`df8ad65646b08e1cdbd4bbf158cb31cc65adc72d1713ba34646b0e25c3cfcc64`.
The source input receipt SHA-256 is
`95ee147e1221ce4ef6c98bdd0b808398a01362343ffaa66debc217d6eeff5a9b`.
Keep one thread, `HKY_CHUNK=1024`, 8-GiB address space and 900 seconds per case.
These limits come from COMPLETED_GUESS_REPLAY_PROTOCOL.md; a fresh attempt is
not an interrupted clock resumed. No independent solver-word recount follows.

Sources reviewed for preparation: the study PROTOCOL.md's HKY literature and
original completion protocol; AGENTS.md sections on checkpointing, protocol-bound
jobs and isolation; the local experiment-protocol and queue-experiment skills;
the canonical [experiment-protocol skill](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/.claude/skills/experiment-protocol/SKILL.md),
blob `072da4999ba4ed09c5b0e82f6a65af2e5931be64`. The next execution needs a
dated recovery amendment and a validated, committed, frozen
`experiment.protocol/v1`, followed by a complete `experiment.export/v1`.
The original Markdown protocol must remain intact. Preparation is not a
retrospective claim that the historical runs used the newer schema.

Before any launch, AGENTS.md states: **“Other runtimes (Codex) run
`scripts/checkpoint.sh gate` before launching and do not launch if it fails.”**
The resume note was filled with exact inputs, source state, commands, caps,
pending decisions and no running experiment. The official gate exited 1:
no fallback AWS profile, no `CHECKPOINT_UPLOAD_TOKEN` and no local upload-token
file. Nothing was checkpointed and no experiment launched. Exact output is in
`pipeline-receipts/restart-checkpoint-gate-20261026/`.

Next: configure the documented checkpoint upload binding through the environment,
without placing bearer tokens in chat, Git or reports; validate/freeze the recovery
protocol; acquire the resource lock per case using `tools/isolated_bench.py busy`;
retain every capped, mismatched and successful result. No paid VM is allocated.
The USD100 extra-cloud ceiling includes replay; workspace billing is unmeasured.

## Source validation and publication

The first integrated source at `1d71daac7` fails before release library tests on an
unchanged upstream SAT file. Its exact blob and gate receipt are in
`problems/sat-progress-phase-base-syntax.md`. The earlier 4,635-test pass belongs
to the frozen constructor source. The later merge `69cf0253e` incorporates
remote `ac16317e5` and main `942caf7e4`; it retains the identical SAT blob.
A fresh final-source library attempt and post-merge browser receipt are
preserved separately from the first integration checks. Do not patch unrelated SAT code or transfer
that pass to an integrated head.

The new dashboard preview passes 1280/390/320, dark and JavaScript-disabled;
there are no script errors or external requests. Its native-evidence generator
and panel index check pass. The merged full-site suite cannot assemble because
a migrated backend-leaderboard source is not hydrated. These are source/metadata
checks, not experiments or admitted performance measurements.

Twenty-four original files match the remote's genuine manifests. Forty-one
remaining unchanged files total 7,508,437 bytes. The workspace has no AWS
identity. The prepared CI transport passes its local tests, but its binary
input upload failed with HTTP401; no workflow ran and no S3 object was uploaded
or read back. The owner storage-exception decision remains pending. Do not add
the exception label, push oversized data, split/delete raw evidence or fabricate
manifests. Keep the completed bytes and source commits intact while resolving
access. The broader research campaign and final-head delivery remain incomplete.
