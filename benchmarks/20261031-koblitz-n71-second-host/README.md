# Koblitz n=71 single-target online IC vs rho — second host

Replay of the identical candidate `IC1N71Ckb0fb85200PDP4s3rootindexRCguidedLAdenseTDsingleISO0h6367473b54e6`
on a taskset-pinned x86_64 Linux gvisor host, against the first unpinned arm64
macOS rung.

| host | workloads | median ρ/IC | 95% CI | claim-check | cold ratio |
|:--|--:|--:|:--|:--|--:|
| arm64 macOS M4 Pro, unpinned | 16 | 6.12× | 4.00–11.89 | PASS 16/16 | 177.7 |
| x86_64 Linux gvisor, `taskset -c 2` | 16 | 7.56× | 4.96–14.93 | PASS 16/16 | 141.1 |

Identical candidate manifest on both hosts; five exclusive IC online phases and
the rho walk/collision/recovery interval; both scalars and replay certificates
verified. Constant-factor online win only. Supplementary charged whole-process
S (wall-equivalent additions / sqrt(r), host-normalised) is 15.5 pinned / 19.3
unpinned against rho 0.10: total IC work still loses, so the promotion is
classed engineering/verification, not a total-work advance. No asymptotic
sub-rho, key-recovery, or deployed-curve claim.

Evidence: `research/sat_factor_base_review_20260908/autolab/evidence/20261031-n71-second-host-replay/`.
Leaderboard/browser data are unchanged: the IC leaderboard reads frozen
S-family analyses, and this vs_rho online-wall row adds no S row or curve
registry entry.

## Ratios and floor

- Matched rho reference: strong rung 3, 32 lanes, DP=4, A=142 signed-Frobenius
  classes. Online median ratio rho/IC = 7.56× pinned, 6.12× unpinned.
- Floor: ideal signed-Frobenius walk `sqrt(pi*r/(4A))` = 5.52e6 steps, so floor
  S = `sqrt(pi/(4A))` = 0.0744. Observed rho S median 0.1026 = 1.38× floor.
  IC charged S is 208× the floor (15.5/0.0744); this is the total-work gap,
  not the online claim.
- Exponent over >=4 sizes: not applicable to a single-size replay. No exponent
  is fitted and no extrapolation is made; cross-size rho projections remain
  secondary estimates.
