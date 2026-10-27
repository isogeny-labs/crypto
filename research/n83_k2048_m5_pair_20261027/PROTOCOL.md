# Primary N83 K2048 five-summand model pair

The next gate tests whether the matched 2,048-column U and D bases give a
tractable chained-S3 five-summand SAT domain on the same public subgroup
points. It measures model construction, root propagation and a bounded
natural search. It is a solver-stage diagnostic; a verified relation, rank,
factor-base logarithms and target recovery require a later worker and replay.

## Frozen inputs and algorithm

- Curve: `EC1N83Ckb0h252dacc102a4`, polynomial basis
  `z^83 + z^45 + z^2 + z + 1`, `a=0`, `b=1`, subgroup order
  `2417851639230796216685689`.
- U: `public_x_hash`, seed `2026100801`, K2048, compressed SHA-256
  `aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`,
  point-set BLAKE3
  `d528f5828a05a00add4252212bc909ece481f33093e4074ff7dd8c6a51d3c97e`.
- D: `direct_subgroup_x_d18_prefix2048`, compressed SHA-256
  `2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7`,
  point-set BLAKE3
  `2b5ac37ec3779026278f9debf1a3be6fdfdcff6ff7838f78f1eed4bee2f04b0b`.
- Public corpus: `research/koblitz_n83_factor_base_sweep_20261008/pilot-01/probe-corpus.json`,
  SHA-256 `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`.
  Target fixtures 0 and 1 are existing points. Both arms receive the same
  point in each pair.
- Five summands; chained S3 encoding; signed-Frobenius orbit domains from all
  2,048 representative x coordinates; zero split bits; model variable limit
  300,000 and orbit-domain clause limit 8,000,000. Keep the SAT implementation
  and its source/binary digest identical across arms.

## Execution and stop rules

Run preflight for U and D on fixture 0, then search in U/D/D/U order over
fixtures 0/1/1/0. Each job has one CPU, a 4 GiB cgroup memory and swap
ceiling, no network, read-only source and input mounts, and a 300-second host
watchdog. The in-process SAT search has a 5,000-conflict cap and a 240-second
deadline. Do not silently retry a failed cell. Preserve OOM, timeout, model
error, and solver `Unknown` as distinct results. Report all setup, model,
root and search wall times and cgroup peaks separately; these are shared-host
diagnostics, not a Tier 0 or Tier 1 speed comparison.

Verify object and corpus hashes before the model is built. Check the header
curve, policy, seed, closure, representative count, and point count. Compare
arm receipts only at the same fixture and resource envelope. A SAT model here
is not a group relation until every x coordinate is lifted, summed and
independently checked; no rank or IC candidate ID is assigned by this gate.
