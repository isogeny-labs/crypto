# Koblitz n=71 single-target online IC/rho — second-host replay (2026-10-31)

Promotes the first n=71 one-target rung (`20261031T071642Z-140c04e457`, unpinned
arm64 macOS) with an identical-candidate replay on a taskset-pinned x86_64
Linux gvisor host (`20261031T191427Z-4a20b3296e`).

- Frozen commit: `d51b908a77574e8d7ac7aa2cb3332961578cade8`
- Candidate: `IC1N71Ckb0fb85200PDP4s3rootindexRCguidedLAdenseTDsingleISO0h6367473b54e6`, manifest `6367473b…`
- Fixture: `EC1N71Ce0hac1355ef3db6` (a=0, n=71, r=5513228015079457); target law `kic-n71-single-target-public-point-v1`
- Hosts: arm64 macOS M4 Pro, unpinned (median 6.12x); x86_64 Linux 4.19 gvisor, every producer `taskset -c 2`, `cpus_allowed_list: 2` (median 7.56x, 95% CI 4.96–14.93)
- 16 one-target workloads per host, claim-check PASS 16/16 on each; both arms' scalars and replay certificates verified; identical candidate manifest on both hosts.
- Cold ratio median 141.1 pinned / 177.7 unpinned (IC setup outside the online interval).
- Supplementary charged whole-process S (wall-equivalent additions / sqrt(r), host-normalised, **not** native-operation S): 15.5 pinned, 19.3 unpinned; rho S 0.10. Total IC work still loses; class = engineering/verification, no total-work advance, no asymptotic sub-rho claim.

Files: `autolab_run/{state.json,artifacts,inputs,receipts}` (logs and the raw pair tables are excluded).
