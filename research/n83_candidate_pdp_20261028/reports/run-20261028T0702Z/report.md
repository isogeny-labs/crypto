# Exact LFD on paired n83 m2 factor bases

Observations only. Run `n83-degree3-paired-20261028-r1` retains eight original
polynomial systems and computes ordinary-degree V3 closure with complete traces.
It measures last fall degree, not degree of regularity or intermediate solver
telemetry. Independent scientific review remains pending.

| Curve / target | Baseline LFD | Spike40 LFD | Prior FFD, both |
|---|---:|---:|---:|
| Koblitz planted | 2 | 3 | 2 |
| Koblitz natural | 2 | 3 | 2 |
| General binary planted | 2 | 3 | 2 |
| General binary natural | 2 | 3 | 2 |

All eight producers and eight fresh-process replays passed. All three runtime
controls and four standalone unit tests passed. No production timeout, failure,
retry, exclusion or resume. The 200-file manifest passes. Values are exact for
these saved inputs; there is no population estimate or sampling interval.

The baseline uses positions 0..26; spike40 uses 0..25,40. Each curve has exactly
2^27 candidate x-values, with all affine lifts saved. Both arms use the same
natural targets and common-low26 planted tuples. Planted targets are not uniform
samples over either full base. There are 54 Boolean variables per system.

Each candidate supplies a degree-at-most-two V3 row with nonzero remainder
against the completely replayed V2 space: a strict degree-three fall. Natural
cases contain one in V3. Planted cases have linear rank 53 and both remaining
affine assignments satisfy every original equation. These provide global upper
bounds of three, matching the lower certificates. Baseline retains its exact
V2 upper and lower certificates of two. V3 rank is 26288 for each planted case
and 26290 for each natural case. Spike40 V2 linear rank was 17; V3 reaches 53
or 54. A rank increase alone is not used as an LFD certificate.

The multiplication rule is deg(row)<3 before multiplying by a variable, before
Boolean normalization. Full-degree rows are never multiplied merely because
square reduction could lower their degree. Fresh replay repeats source insertion,
all reductions, all queue operations, immutable row checks, strict nonmembership
and the global upper check. Producer replay is not independent research review.

## Provenance and reproduction

The run lasted 07:02:11–07:17:39 UTC on 2026-10-28, on the recorded existing Spot
host; timestamps are diagnostic only. One native process, 8 GiB virtual memory,
600 seconds per producer or replay, 10800-second outer limit. This is the
recorded existing-host fallback after the EKS shared-bucket authorization failure.
No timing comparison or CPU-time measurement is claimed; per-case CPU/RSS is null.

Source commit: 3b4cca71a0f3df5ab0b5da202c10af1e741095b7.
Frozen protocol commit: 4955822af2f805eb6046dcfe2fb775bdf52a0293.
Protocol SHA-256: 471f53d00cdab630bb3a1ab2f1ad1e637eeceb17549ed08fad0dab21dd13031f.
Native SHA-256: fd0393eaacc36513d835abb362bf63729300088c339cd12e82cb26f41305b574.
Seed: 20261027, retained from paired inputs; no targets redrawn here.

Launch: `run-degree3.sh BINARY PRIOR_PDP_RAW FRESH_OUTPUT_ROOT` under the outer
10800-second timeout; the exact shell and all cases are frozen with the protocol.
Native per-case interface: `degree3 run INPUTS ID V2DIR OUT`, then
`degree3 replay INPUTS ID V2DIR OUT`. See `../../run-degree3.sh`.

Nine content-addressed archives reconstruct the complete raw tree. Eight archives
contain `cases/KEY/{rows.bin,operations.jsonl,progress.jsonl,result.json}`;
metadata contains original inputs, V2 snapshots, executable, controls, replay
logs, statuses, terminal, environment and manifest. Extract all nine into one
fresh root, then `sha256sum -c manifest.sha256`. The authoritative archive index
is `../../evidence/degree3-storage.json`; byte layouts are specified in
`../../DEGREE3_FORMAT.json`. The unified export is
`../../experiment-degree3.export.json`, bound to `../../protocol-degree3.json`.

Raw S3 prefix:
`s3://setec-astronomy-artifacts/runs/n83-degree3-paired-20261028-r1/local/`.
The frozen source archive is under the same run's `source/` prefix. Failed
pre-production compile output and the rejected cargo-fmt invocation are recorded
in validation/session history; no producer output was discarded.

## Scope

For these saved candidate systems, FFD=2 and LFD=3. This is not a refutation of a
universal or asymptotic FFD heuristic, a direct solving-degree measurement, an
elite-base result, or evidence of reduced regularity. Prime and m3..9 LFD remain
unresolved. The original fixed-base regularity sweep and later top-form bounds
are separate datasets; they cannot be assigned to spike40.

Definitions and closure convention follow Caminata and Gorla, *Solving degree,
last fall degree, and related invariants*, definitions 1.1 and 1.5:
https://arxiv.org/html/2112.05579. The prior FFD claims are from the exact same
source hashes in `../../evidence/pdp/paired-summary.json`.
