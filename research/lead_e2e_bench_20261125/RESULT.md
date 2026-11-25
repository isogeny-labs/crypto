# Shared lead ECbench results

## Current K1-17 paired panel

The v2 `paired-k1-17.json` spec was frozen in commit `cd235d6a6b5` before
execution (SHA-256
`4c8b8510fab79cea67a2a92f1dd8e4bd4462824be41046c6636efc26c2058caa`).
ECbench was built from this branch's source with Rust 1.99.1 in the development
profile and `--no-default-features`; binary SHA-256
`899238f36ca1c9e4797b6d4f64089724c3d56c5939121ff1aeecce70f243f5ff`.
The executable's session metadata does not embed a Git commit, so the build
command and binary hash are the local provenance record. This is
`[pcc:unverified]` local integration evidence.

The plan admitted 16 executions across two canonical K1-17 public workloads,
`W408c022d3e16` and `W5cb35f78ac66`, with two cold rounds per arm and
alternating order. The reference rho, root-finding IC, charged two-large-prime
IC, and rho A/A control all returned the same recovered scalar within each
workload (`33413` and `36008`, respectively). All 16 outer target checks
passed. The sealed session `ECBS1h5e9c13637dc3` is in
`runs/panel-k1-17-20261125T2303Z/`; its `records.jsonl` SHA-256 is
`f4902940c4c783e3e7b0b619fc2aec615dca244b0ed3b2a739b88c7d5329b3ca`.

All four `ic.large_prime_v2` records have
`verification_performed=true`, `verified_fast=true`, and
`verified_independent=true`. Their `target_recovery_check` online phase is
nonzero in every record (127,416 to 161,542 ns), with no zero-phase reason.
This resolves the missing final online check for the charged v2 path on both
workloads. `ecbench verify --replay-all` reproduced 16/16 records identically,
regraded the session, and reported zero problems. The saved audit receipt is
`audit-paired-k1-17.json` at SHA-256
`e6eb8e60949e008f334f5f128519a0c86706d362cd028f68e2fb805a84249ff8`.

The panel uses L0 isolation. Large-prime native work remains unpriced, and
the method configurations differ from earlier lead experiments. These rows
establish a working shared end-to-end benchmark and accounting check, not a
speedup or parity claim. The replay was local with the same binary; a separate
host and independent implementation remain scientific promotion gates.

## Historical first n=11 smoke

Local integration check at `2026-11-25T22:30Z`; these observations are
`[pcc:unverified]` until the repository's independent standing gate accepts
them. The immutable input is `smoke-n11.json` at SHA-256
`2fd78b9e20fdfa7e15ef4c46274df7cc4272803f0a91848336a3ed9d6fa608c8`,
committed before execution as `45208861331`. The executed binary SHA-256 is
`f717da8ad079f5896db084ffbe4f6d23bdde7cd7435783d1af6e438121650318`;
it was a pre-existing local build, not rebuilt from the PR source.

The plan accepted all four arms on one workload, `W17d7d0a9c58d` (full hash
`17d7d0a9c58d317b68e3f0dc14532fb920b31ffa159328f4ccb096601d7b3531`).
The recorded public target is `(0x5ee, 0x7ce)`. The reference, root-finding,
double-large-prime, and A/A arms all returned scalar `18`; the session records
`matches_target=true` for each. ECbench sealed session `ECBS1h9b25152b2df9`
with four `verified` records and `ecbench verify --replay-all` reproduced all
four records with zero audit problems. This establishes a local end-to-end
correctness smoke for these exact configurations and this one target.

The raw session is in `runs/smoke-20261125T2230Z/`. The saved full-replay
receipt is `audit-smoke-n11.json` at SHA-256
`3e12fec8d25da9437ea57a2ba9ff6edf0268684b36d30a2e40c23fe388e29d0f`.
Its `records.jsonl` SHA-256
is `6edd714cd9aa1c27827cf44658304b594eb9399dfb4e540b0e760e69de4790e4`.
Rerun the command in `README.md` to reproduce the audit from the committed
session. All four records are retained.

## Accounting boundary found

The double-large-prime record reports `outcome.status=verified` and the outer
ECbench check agrees with the target, while its method detail reports
`verification_performed=false`, `verified_fast=false`, and
`verified_independent=false`. Its online phase map gives
`target_recovery_check=0` with the reason `no candidate logarithm was pinned`,
despite `recovered=18`. The method's cold `verification` phase is present, and
`matches_target=true` comes from the outer harness. The online interval cannot
be used for a complete online comparison until this discrepancy is reconciled
in the method timing/verification code and checked again. The local smoke's
correctness conclusion uses the outer check and replay only.

All costs in this session are L0 and marked `lower_bound=true` because native
work remains unpriced. One public target and one round cannot support a
performance inference. The next benchmark stage is to repair the online
recovery phase, freeze multiple same-target workloads and a strong IC baseline,
then run an isolated paired panel with A/A, positive control, price-band
accounting, and independent replay. Failure and timeout rows must remain in
that panel.
