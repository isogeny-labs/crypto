# Protocol: is the isogeny base's lead a seed-0 artifact, and what does its tower search cost? (frozen before data)

2026-11-14. Follows `research/curve25519_engine_other_bases_20261114` (PR #4157), where the `isogeny` base (t = 5, seed 0) had
the highest hits per counted F4 mul-adds, resting on 12 (blind) and 99 (`lifts=1`) hits, with the tower search not counted.
Same instrument and conditions: `curve25519_family f4cost`, k = 20 miniature (p = 1048573, A = 926), G = 3, m = 2, mode full,
t = 5, 20,000 targets per arm, target seed 2026111403 (the references' seed), binary sha256
`ed99cb1653aaa35c9a87c0aa0817a63de0687dc498600e5694b3f7bc443f99bd`. All results [pcc:unverified]. No rho arm: no cross-method
claim and no distance to the matched rho.

## Cells

- Tower seeds 1 to 20 (`isogeny@SEED`), each at `lifts=0` and `lifts=1`: 40 arms. Seed 0 is the PR #4157 arm (same conditions, not rerun).
- Tower search cost for seeds 0 to 20, from the library's `isogeny_tower` (`IsogenyTowerCost`: curves tried, `search_gae`,
  `count_gae`, `build_gae`, Legendre symbols, square roots, Velu evaluations), by a scratch program built against this repository.

## Hypotheses (directional, fixed now)

- H6 (the lead is not a seed-0 artifact): the blind arm's hits per 10^9 mul-adds, pooled over seeds 1 to 20 (total hits over total
  mul-adds, exact Poisson 95% interval), has its lower bound above 1.515, the upper end of digit set 1's blind interval
  (1.205 [0.945, 1.515]), the next-best base of PR #4157.
- H7 (lifts neutral on the isogeny base): the pooled rate ratio `lifts=1` / blind over seeds 1 to 20 has a two-sided exact conditional
  binomial p of at least 0.05. If it falls below 0.05 the neutrality seen at seed 0 is not general.
- H8 (cost shape): the blind F4 mul-adds per target differ by less than 10% between the seeds' largest and smallest.
- H9 (setup is negligible): per seed, the tower's total group operations (`search_gae` + `count_gae` + `build_gae`) are below 10^-3 of
  that seed's counted blind F4 mul-adds (the threshold is about 3.5e6 group operations for a blind arm of about 3.5e9 mul-adds).
  This tests the setup against the F4 work at an equal weight of one group operation per mul-add; it does not by itself establish the
  order at every price in the 0 to 1 GAE band, and I do not claim that.

Descriptive, no prediction: per-seed hits, per-seed rate, the dispersion of the blind counts against a common-rate Poisson model
(chi-square on counts weighted by exposure), the range across seeds, and where seed 0 falls in that range.

Decision: a hypothesis is "supported" only if its stated bound holds; otherwise reported as not supported. No correction for the
four hypotheses beyond stating their number.

## Out of scope

Other t, k, G; other bases; a rho arm; timing; a claim about how the seed should be chosen (the torsion-leads study treats seed
selection by coset coverage as a lead). No Conductor task, isolab-eks run or S3 resume checkpoint (the AWS route is absent here;
counted mul-adds are deterministic); no research-kb query; no figure or PDF.
