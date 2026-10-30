# N83 five-base cold comparison with one shared binary pair

This frozen retest compares the same five current-source factor bases and the same online and cold selectors used by the 2026-10-19 finalist confirmation. Its 16 new N83 public points were sealed after the earlier run failed the cross-stratum executed-binary identity gate: one host built the IC and rho examples with Rust 1.99.1, the other with Rust 1.99.0. Both prior strata passed their individual Tier 1 and arithmetic replays, but the aggregate retained `VERIFICATION_FAILED` and null selections. The previous timings are diagnostics only. This retest fixes the binary allocation before measuring the new points and keeps every arm, cap, ordering rule, and pairwise selection threshold fixed.

## Frozen instance and arms

The curve is `EC1N83Ckb1a1haacb69ee9f37`: polynomial-basis GF(2^83), modulus terms `[0,2,4,7,83]`, `y² + xy = x³ + x² + 1`, subgroup order `8569786107849059`, cofactor `1128547018`. The exact IC source SHA-256 is `4a94e4e570f0cc4e39ba5c0846e411a6c43820c28119f7b9802e549023596507`, from commit `8bf07aefb3fb2a65248ce379da7284cb58a5581d`. All five original candidate manifests and the upstream selector receipts are retained byte-for-byte.

| Frozen arm | Actual usable points | Folded columns | Exact candidate ID |
| --- | ---: | ---: | --- |
| K992/root11 dynamic | 164,672 | 992 | `IC1N83Ckb1a1fb164672PDP4rootRCguidedLAgaussTDdirectISO0ha6a31a64b52b` |
| K1024/root11 dynamic | 169,984 | 1,024 | `IC1N83Ckb1a1fb169984PDP4rootRCguidedLAgaussTDdirectISO0h644bda8bfc36` |
| K3300/wide3 | 547,800 | 3,300 | `IC1N83Ckb1a1fb547800PDP4rootRCguidedLAgaussTDdirectISO0h8d1e886f8586` |
| K2304/root11 dynamic | 382,464 | 2,304 | `IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h00a0a52d463c` |
| K1107/root11 dynamic | 183,762 | 1,107 | `IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0he423fcbc5668` |

The target domain is `n83-fresh-cold-finalists16-shared-binary-20261021-v3`. For indices 0–15, use `s_i = 1 + LE(SHA256(domain || u32_le(i))) mod (r-1)` and `Q_i = s_i G`. The public target file SHA-256 is `c604d7d42468d16636f4d0a8f6fe4caa75b91104d85287743b868e04ce5098e4`; the verifier-only answer file SHA-256 is `13aad43447862c318b86b880cb0997a6df84b25803b0a1c9eccc931688687880`. The frozen exclusion audit found zero overlap with 39,600 previously recorded public points, including all 16 points from the failed first confirmation.

## Shared executable and Tier 1 admission

The workflow builds and tests the IC and rho examples exactly once with `cargo +1.99.1 --release --locked` on Linux x86-64. It uploads the two executables and a SHA-256 manifest as one build artifact. Each of two distinct overlapping `ubuntu-24.04` measurement jobs downloads that same artifact, verifies both SHA-256 values, and executes those files. The source and executed-binary hashes in every producer receipt must agree across strata. The aggregate retains null selections if either hash differs, even if both strata pass individually. No compiler installation or rebuild occurs in a measurement job.

Split the 16 points into strata 0–7 and 8–15, on separate Tier 1 cloud allocations. For each point use five fresh IC processes with empty factor-base, root-index and log caches, one independent same-point rho process per candidate, and three fixed-point rho calibrations before and after every pair. Rotate candidate order by global point index and alternate IC/rho order by point and candidate parity. Require at least four visible CPUs, sufficient RAM for the 15.25 GiB child RSS cap, cgroup and affinity telemetry, zero observed steal, and at most 10% before/after calibration median drift for every pair. The IC cap is 600 seconds for K992, K1024 and K1107, 900 seconds for K2304, and 1,200 seconds for K3300; rho/calibration children have a 120-second cap, each producer a 330-minute cap and each cloud job a 360-minute cap. Preserve failed, capped, noisy, and partial rows.

The primary metric is independently verified one-target online IC wall time after reusable setup. The separate cold-process metric charges fresh setup, base construction, relation/rank work, and the target stages. For every unordered candidate pair, compute 16 paired online log-time differences and 16 paired cold-time differences. Enumerate all `2^16` sign flips for two-sided tests at `0.05/10`. Select a unique fastest geometric-mean online or arithmetic-mean cold candidate only if it beats each other arm and all four pairwise tests pass. Any missing, unverified, resource-rejected, capped or noisy pair, or any cross-stratum binary mismatch, leaves both selected IDs null. Report same-point rho/IC ratios, stage and resource costs, and exact `IC1...W...R...` rows separately from the selectors.

`dispatch.json` freezes the fixture, manifests, code, workflow, compiler-sharing rule, pilot, upstream evidence, and source hashes before a measured child starts. The two raw provider ZIPs must be hash-checked and copied to S3 with a fresh download verification. The original failed run and this retest remain separate immutable records. This retest can select only among the five listed bases on this source and public-point population.
