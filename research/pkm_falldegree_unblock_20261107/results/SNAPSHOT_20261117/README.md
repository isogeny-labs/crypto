# Snapshot of the PKM fall-degree pods, 2026-11-17 (taken by another session at the owner's request)

Ten RunPod pods of this study had been running since 2026-11-15. This directory holds what
each pod's runner reported and served at 05:54 to 06:00 UTC on 2026-11-17. Nothing was started,
stopped or changed on any pod. Every reading below is `[pcc:unverified]`: it is a pod's own
record, not scored by `prime_fall_degree_tools score` and not gated.

Taken from a session that did not launch these pods. The owner asked for the status and a
snapshot. Each pod's runner token was read from the pod's environment through the RunPod API,
held in memory, used for `GET /status`, `GET /manifest` and `GET /results.tar.gz` only, and is
in no file here.

## Per pod

| pod | stage | runner state | started | ended | what the results hold |
|:--|:--|:--|:--|:--|:--|
| `pfd-m2c-t12-k0-rp` | `m2c:12:0:7` | done, exit 0 | 11-15 23:41 | 11-16 01:35 | closure without 1 at c = 2..5, with 1 at c = 6 (190,051 columns); grid zeros 0 |
| `pfd-m2c-t12-k1-rp` | `m2c:12:1:7` | done, exit 0 | 11-15 23:41 | 11-16 01:42 | the same: 1 first at c = 6; grid zeros 0 |
| `pfd-m2c-t13-k0-rp` | `m2c:13:0:7` | done, exit "deadline" | 11-15 23:41 | 11-16 03:41 | no result row (stopped at the 14,400 s limit) |
| `pfd-m2c-t13-k1-rp` | `m2c:13:1:7` | done, exit 1 | 11-15 23:41 | 11-16 01:52 | no result row; the binary aborted (exit 134, core dumped) |
| `pfd-m2c-t14-k0-rp` | `m2c:14:0:7` | done, exit "deadline" | 11-15 23:41 | 11-16 23:41 | no result row (stopped at the 86,400 s limit) |
| `pfd-m2c-t14-k1-rp` | `m2c:14:1:7` | done, exit "deadline" | 11-15 23:41 | 11-16 23:41 | empty result file |
| `pfd-x-t14-k0-c7-m112-rp` | `cap:14:0:7` | done, exit 0 | 11-15 23:41 | 11-16 05:58 | capped F4 at 7: `inconsistent false`, `oversize true`, `timed_out true`, `degree_reached 7`; grid zeros 0 |
| `pfd-x-t14-k1-c7-m112-rp` | `cap:14:1:7` | done, exit 0 | 11-15 23:41 | 11-16 05:41 | the same: not refuted, oversize, degree reached 7; grid zeros 0 |
| `bdr-growthlaw` | six null-control cells (`nullw1` t = 12, `nullw2` t = 14, `nullw3` t = 13, two draws each) | nine `exec` jobs, all exit 0 | 11-15 19:51 | last log 11-16 | a different runner (file server on `/work`); every file except the 12 MB study binary is copied to `bdr-growthlaw/work/`, listed with digests in `bdr-growthlaw/files.json`. The `t = 13` and `t = 14` null closures are censored at degree 6 by `max_entries` |
| `pkm-dense-rerun-20261115` | not read | not read | 11-15 18:19 | | a supervisor whose state is on RunPod network volume `yuzuh9rmej` mounted at `/workspace`; only the RunPod API record is here |

`pfd-m2c-t14-k1-rp` answered 404 on the first `status` and `results` requests and 200 on a retry
a few minutes later.

## Readings, as far as these records go

- `t = 12`, `m = 2`, Kummer tower, both draws: the closure contains 1 first at degree 6, the
  value already recorded for `t = 12`.
- `t = 14`, cap 7, both draws: the run stopped oversize at `--max-dense 27900000000` without
  refuting. It decides nothing about `d_F` at `t = 14`; in particular it gives no upper bound 7.
- `t = 13` and `t = 14` closures (`m2c`): no row on any of the four pods.

## State of the pods at the snapshot

All ten were `RUNNING` in the RunPod API with CPU and memory utilisation 0 at two readings a
minute apart (`snapshot.json`, field `api.runtime`), and every runner that answered reported
`state: done`. The eight `pfd-*` pods had been idle since their jobs ended: between 6 and 28
hours. Combined price of the ten: US$14.24 per hour.

## Files

Per pod: `snapshot.json` (what this session recorded, with the HTTP codes and the SHA-256 of
the fetched archive), `status.json` and `manifest.json` (the runner's answers), and
`results.tar.gz` (the runner's archive, unmodified). The archives were not checked against the
manifests file by file here.

## Shutdown (2026-11-17, 06:02 UTC, on the owner's instruction "shut them down if idle")

Each pod was checked again immediately before deletion: name matched, RunPod API CPU and memory
utilisation 0, and for the eight `pfd-*` pods the runner's `state` was `done`. All ten were then
deleted through the REST v2 API (HTTP 204 each, 06:02:37 to 06:02:53 UTC); a following listing
no longer shows them.

- The eight `pfd-*` pods and `bdr-growthlaw`: everything they held, except the study binaries, is
  in this directory.
- `pkm-dense-rerun-20261115`: not read before deletion. Its supervisor keeps its state on network
  volume `yuzuh9rmej`, which is not deleted with the pod and still exists (and is billed as storage).

Left running, not part of this shutdown: `replead-r1-20261116` (another session asks the owner
to copy `/work/out/` before terminating it) and `bdr-traceline-20261117b` (started by a session
that is active now).
