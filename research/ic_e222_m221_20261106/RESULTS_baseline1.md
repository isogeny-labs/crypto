# E-222 / M-221 analog ladder: baseline 1 (local development run)

**Status:** unvalidated (pre-gate). Local, unisolated host; no replay certificate, no validator report, no matched-rho reference. Not a leaderboard or benchmarks row. Wall time unused.

**Setup.** `python3 -I gen_instances.py --bits 10,12,14,16 --per 12` gives 144 instances (E: Edwards, #E = 4r; M: Montgomery, #E = 8r; W: Weierstrass, cofactor 1), in the Weierstrass model, planted logs. Resulting prime-order bits are 10 to 17. `run_baseline.py`: `ic solve-prime-file --fb-size 8 --seed 1..3 --rho-steps 0`, 432 runs, shuffled order. Producer cap: p ≤ 10^6, so rungs stop at r ≈ 2^17 (E, M have p about 4r, 8r). Instances sha256 (MANIFEST) 8e020f2e…, rows sha256 afa76f5f….

**Verified.** 432/432 runs returned status `complete` with the scalar equal to the planted log. Seed-to-seed CV of `trials_total` within an instance: median 0.06 to 0.11 (A/A noise floor of the sampler).

**Measured.**
- Median `trials_total` against r bits, log2 slope: E 0.94, M 1.00, W 0.93 per bit (cost about doubles per bit of r: 2^(r/1) up to the cap, i.e. proportional to the group size, not its square root).
- Ratio to the cofactor-1 control at equal r: E 3.8 to 4.4, M 5.7 to 8.6 (bootstrap CIs in `results/baseline1/summary.json`).
- `trials_total / p`: E 0.56–0.58, M 0.49–0.53, W 0.55–0.58. The ratio at equal r is therefore the extra field bits that cofactor h forces (p ≈ h·r), not a structural effect.

**Reading.** The producer's relation cost is proportional to p in all three families; the cofactor torsion is not used. At equal p the E and M families cost the same as the control, so this baseline contains no cofactor-specific advantage or penalty (H4 control passes in the weak sense that form alone changes nothing here, because the producer sees only the Weierstrass image).

**Not done.** Matched rho reference and the 0/0.1/1 GAE band (the producer reports `rho` wall time only; use `ic rho` next); H1 (torsion quotient) and H2/H3 need a producer change, none exists in `solve-prime-file`; rungs above r ≈ 2^17 need `prime-orbit` or a producer without the 10^6 cap; EKS submission (no `aws` CLI in this sandbox); Conductor, KB, ecdb, resume checkpoint, benchrec record.

**Next.** (1) `ic rho` matched reference on the same 144 instances and the band. (2) Implement the H1 factor base as a quotient by E[h] in a copy of the producer and test against the equal-p control: the baseline now says what H1 must beat (cost per relation ∝ p, so a factor-h reduction in effective p would show as trials/p falling by about h).
