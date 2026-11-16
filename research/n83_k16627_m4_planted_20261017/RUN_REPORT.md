# N83 K=16,627 planted four-summand relation control

**Run:** 2026-10-18 UTC. **Protocol:** [PROTOCOL.md](PROTOCOL.md) committed as `035c6b7924` before implementation, with the pre-run [object-digest clarification](INPUT_ADDENDUM.md). **Status:** `UNKNOWN_wall_cap` for the pinned positive arm; the unpinned arm was not attempted under the frozen dependency rule. No IC scoreboard row is promoted.

## Exact input and controls

The exact primary curve is `icv1-f2m83-tm6151469093347-debefd74`, `a=0`, `b=1`, over `GF(2^83)` with low modulus terms `[0,1,2,45]`. The retained `public_x_hash`, seed-`2026100801` signed-Frobenius base has 16,627 columns and 2,760,082 stored points. Its compressed object's BLAKE3 is `1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`; the point-set BLAKE3 is `564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0`. The earlier [construction report](../n83_k16627_prebuild_20261017/RUN_REPORT.md) and [m=4 capacity report](../n83_k16627_m4_capacity_20261017/RUN_REPORT.md) are separate gates.

The preregistered target was the group sum of the first stored representatives of signed-orbit columns 0, 1, 2 and 3, contingent on all intermediate sums being affine and the x values distinct. The worker intended to pin those four x values only after installing the complete K=16,627 Frobenius-orbit SAT domain. Its source snapshot was bound to clean commit `1322cb5681d31849a2f33a6b47e16f9e3d598e0a`; [all five source files](source-files.sha256) and [all five staged inputs](inputs-before-run.sha256) passed SHA-256 checks. The native Linux release worker built in the pinned offline image, with binary SHA-256 `ad476db700089d5c62864fff945e5f6d573cfabbd2fe9cef617bda415632685d`. The touched exporter example passed 19/19 tests. Details and logs are in [BUILD_RECEIPT.md](BUILD_RECEIPT.md).

The pre-stack release library suite passed 4,216 tests with 117 ignored; the
stacked-head run stopped at inherited compilation errors before tests ran. Both
complete logs and the formatting checks are tracked in
[VALIDATION.md](VALIDATION.md).

## Observed run

| Measure | Pinned positive arm | Unpinned arm |
| --- | ---: | ---: |
| Frozen outer wall cap | 1,800 s | 900 s, not started |
| Container CPU / hard memory / swap | 1 / 4 GiB / 0 | not started |
| Container exit / OOM | 124 / false | not started |
| UTC start to end | 01:11:04 to 01:41:04 | not started |
| Cgroup peak | 2,660,335,616 bytes | not measured |
| Worker JSON receipt | absent | absent by protocol |
| Phase at timeout / SAT conflicts / SAT models | unknown / unknown / unknown | not measured |
| Verified group relation / rank | unknown / unknown | not measured |

The [outer outcome](pinned/outcome.json), [container state](pinned/container-state.json), [host config](pinned/host-config.json), [peak](pinned/peak.bytes), UTC timestamps and raw stdout/stderr are retained. The container used a read-only root, no network and the pinned arm64 image. The worker produced no JSON before the outer `timeout` terminated it. No phase markers existed, so the receipt does **not** distinguish base import, model construction, SAT search or group lift as the phase consuming the wall. The cgroup peak is a whole-container measure; it does not attribute memory to one phase. The earlier model-capacity arm completed, but its separate wall and memory observations cannot identify this run's stopped phase.

## Interpretation and next discriminator

The planted positive condition was **not verified** within this run's cap. Exit 124 is a bounded wall result, not a solver UNSAT result, a measured zero relation yield or evidence that the constructed four-point sum lacks a decomposition. The unpinned arm's prerequisite required a valid pinned relation; it was therefore **not attempted**. There is no natural-query yield, useful-rank, linear-algebra, target-extraction or complete IC cost result from this run. The current verified IC baseline and matched-rho gap remain unchanged. These L0 walls were observed on a shared host and are not a speed comparison; no same-output external solver timing was taken.

The next discriminating experiment is a newly frozen **phase-observable complete-witness arm** on the same object. Flush append-only receipts after import, target construction, model installation and entry into SAT search. For the positive arm, also pin the two intermediate x coordinates computed from the planted group sums, so all seven chain x blocks are fixed while the full orbit domain remains installed. A field-checked SAT model and independently replayed signed-orbit row would establish this positive control; flushed phase markers would identify the stopped phase, and completed solver checkpoints would bound conflict work if the search remained capped. A separate unpinned natural-query corpus still remains necessary for factor-base quality. The [diagram](planted-control.svg) marks the measured capacity gate, this censored positive arm and the remaining search/rank obligations; canonical IC charts are unchanged because no relation, rank or full-cost row was added.
