# Retained structured N83 archive import check

**Run date:** 2026-10-17 UTC. **Outcome:** `PASS` from the native
[verification receipt](verify.json). This is an import and integrity check;
solver search, natural relation yield, useful rank and cold pipeline cost were
not executed.

## Inputs and execution

- Curve: `icv1-f2m83-tm6151469093347-debefd74` (a=0).
- Worker source: commit `1b00cf8a3450540aacd7778b6b2115276f46a81c`;
  the tracked worker and library files are unchanged in the later protocol
  amendment. macOS release executable SHA-256:
  `a24a7b28418e0c44ec47ab5ab03234ab9f419d9ea2e730c420ab467cc2483b32`.
- Input: S3 object named in [PROTOCOL.md](PROTOCOL.md), 10,058,074
  compressed bytes, SHA-256
  `c5de0b26e84ad80bfb7be6a214b89c9682e356effb6079c7abf28cd1151ffb03`;
  manifest SHA-256
  `bd4202ed599f2659f6279dd07dbdc6b5948594df79db969e66530c19e3cf5dca`.
  The object and manifest also passed the worker-pinned BLAKE3 checks.
- Command: `gtimeout 300 /private/tmp/n83-structured-m4-target/release/examples/koblitz_n83_structured_orbit_worker verify /private/tmp/n83-structured-m4-input/manifest.json /private/tmp/n83-structured-m4-input/object.jsonl.gz`
  on a shared arm64 macOS host (L0). Exit code 0. Standard error is empty
  in [verify.stderr](verify.stderr). No valid wall or memory interval was
  recorded, so this result supports no speed comparison.

The [import diagram](import-diagram.svg) shows the exact validation stages.
The worker authenticated the archive, checked the source-point projection,
subgroup and Frobenius identities, then replayed all point/phase/sign labels
and rebuilt the generic factor-base map. It returned **332,166 points**,
**166,083 distinct x-coordinates** and **2,001 signed-Frobenius columns**.
These are counts from one imported object, not relation or rank observations.

The first release example build failed because four tracked compile-time
files were absent from the sparse checkout; its
[failure log](example-release-sparse-failed.log) is retained. The repository's
`scripts/sparse-checkout.sh apply` materialized them, and the replacement
[example tests](example-release.log) passed 2/2. The first `verify` launch
used the test-harness path, which is not an executable example; its
[launch error](verify-launch-failed.stderr) is retained. A separate
[release example build](example-build.log) produced the executable used here.

## Limits and next step

This replay is native but uses the same repository implementation as the
producer. It is not an independent-host certificate. The bounded Linux
m=4 query and m=5 source-path control subsequently ran; their capped
outcomes are in the [run report](RUN_REPORT.md). A matched K=2,048
explicit-orbit comparison and full cold accounting remain open.
The canonical index-calculus scoreboard and progress graphs were checked;
an import integrity check adds no graphable performance or rank result, so
they remain unchanged.
