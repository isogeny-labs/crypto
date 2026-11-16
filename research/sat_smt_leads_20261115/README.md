# Open leads from the SAT/SMT pilots: six follow-up studies (2026-11-15)

Six local pilots, one per lead left open by `research/sat_smt_leads_20261114/`, each with a producer
study and an independent skeptic review (12 agents, one 4-core container). Pilots, not claim data: no
protocol frozen in Git before the runs, no isolab host, no replay certificate, no validator report,
no matched rho arm. Every number here is `[pcc:unverified]`. No leaderboard or `benchmarks/` row
follows. Report with figures: [REPORT.pdf](REPORT.pdf).

Setting: binary curves over F_2^n, point decomposition with m summands whose x-coordinates lie in an
l-dimensional F2-subspace (d for the ECC2K-130 instance). Reference for m = 3 is the linear 2+1
scan, 2^l leaf solves; for m = 4 the pair scan, 2^(2l)/2.

| Lead | Question | Decision per its rule | After skeptic | Files |
|---|---|---|---|---|
| A | Dual-rail encoding at m = 4: does learning go below the pair scan? | kill | kill, supported | [A-dualrail-m4](A-dualrail-m4/README.md), [review](A-dualrail-m4/SKEPTIC.md) |
| B | Does dual-rail transfer to the repository's ECC2K-130 S'4 instance (n = 131)? | inconclusive | inconclusive; conflict reduction at d = 8 reproduced | [B-dualrail-ecc2k130](B-dualrail-ecc2k130/README.md), [review](B-dualrail-ecc2k130/SKEPTIC.md) |
| C | Cheap block refutations, scaling exponent, budget-3 edge | "signal" by the letter; no sub-scan exponent; budget-3 edge not shown | signal is an accounting artefact; other two stand | [C-block-refutations](C-block-refutations/README.md), [review](C-block-refutations/SKEPTIC.md) |
| D | Idea 8 settling run (incremental solving, propagations, 3 seeds) | inconclusive | inconclusive; practical negative | [D-idea8-settling](D-idea8-settling/README.md), [review](D-idea8-settling/SKEPTIC.md) |
| E | Is there a short machine-checked refutation that follows the scan? | short proofs exist (with extension variables) | sound; verdict weaker than stated | [E-scan-certificate-proof](E-scan-certificate-proof/README.md), [review](E-scan-certificate-proof/SKEPTIC.md) |
| F | Close ideas 4 and 11 with noise floors | inconclusive / "nothing to synthesise" | both inconclusive (registered runs not completed) | [F-close-4-and-11](F-close-4-and-11/part1/README.md), [review](F-close-4-and-11/SKEPTIC.md) |

## Results

- **A (m = 4).** Warm CryptoMiniSat on the dual-rail encoding: 4.04, 3.35, 3.09, 2.86 conflicts per
  leaf at l = 3..6 with (x1, x2) assigned; a fresh solver per leaf is not O(1) (143 to >= 6,403).
  (leaves + conflicts)/(2^(2l)/2) for the scan-by-solver arm: 7.94, 4.41, 4.08, 3.96, 3.80; lowest
  single run 2.51; every arm that works above the leaves costs 1.3 to 2.9x that arm at l = 6. The
  skeptic found the series converging to a constant, and the per-leaf figure order-fragile (32
  conflicts per leaf at l = 5 with LSB-first assumptions).
- **B (ECC2K-130 instance, repository generator unchanged).** Mean conflicts per non-decomposable
  target as multiples of the pair count C(A,2), d = 7 / 8 / 9: repository encoding 0.37 / 29.5 /
  >= 40 (censored; the panel reports 36.7 at d = 8); with rails, free solver 0.096 / 0.59 / >= 1.2;
  linear-leaf form with rails and an x1 loop 0.087 / 0.050 / 0.026 (1.6 to 2.6 conflicts per leaf,
  i.e. 1.6 to 2.6x its own scan reference). The skeptic reproduced the counts, showed the rails are
  sound (every added XOR implied, fresh variables pure definitions), and measured 35 to 112x fewer
  conflicts for rails at d = 8 under clause-order shuffling (d = 7 about 4.3x). Wall time at d = 8:
  5.8 s (repository), 4.8 s (rails), 13.2 s (linear-leaf), against 0.82 s for plain-Python pair
  enumeration and 0.017 s for a table lookup. No SAT arm is below pair enumeration in time. The
  effect is decided at one size; with rails alone conflicts per leaf rise from 31 to 390 between
  d = 8 and 9.
- **C (block refutations, l = 6..9, 24 targets per cell).** Zero-conflict refutations are x1 values
  with no curve point, forced by finished siblings. The share of leaves under "cheap" blocks (0.247,
  0.382, 0.404, 0.471 at budget 3) falls to 0.000, 0.008, 0.001, 0.004 once failed ancestor attempts
  are charged. Budget-3 total cost is 1.09 to 1.32x the same solver's scan with no trend. Warm
  exponent at l = 9: 1.44 (1.38..1.50) and 1.26 (1.14..1.37); below 0.9 would have been the signal.
  One exploratory arm (attempts at dimensions 1 and 2 only) is at 0.83 to 0.88x at l = 9, entirely
  from fewer solver calls; conflicts are 0.98 to 1.08x.
- **D (idea 8).** Propagations, incremental over fresh, non-decomposable targets, last third of
  667-target sequences at l = 5: 0.951; per seed 0.976, 0.952, 0.924; interval on the seed means
  0.888..1.018; A/A 0.998 (0.992..1.005). Conflicts 0.601. The incremental arm costs about twice the
  time per propagation and is slower in every seed. The power-law fit is poor (R^2 0.16).
- **E (scan certificate).** Clausal refutations that follow the scan, with XOR definitions as
  extension variables, machine-checked: size slope 1.39 bits per l in clauses (1.36..1.43) and 1.48
  in literals (1.44..1.52) at n = 17, l = 2..5, against about 2.9 to 3.6 for CDCL-found proofs. The
  skeptic's own checker passed 4 proofs; 103 decomposable targets produced no proof and 103 forged
  proofs were rejected. The arm was added by protocol amendment; under the original rule the result
  is "not shown". Cross-leaf sharing: median 2 leaves per core lemma in the rule cell, 5.5 in another
  cell whose proofs are still 0.82 to 0.99 of the no-sharing size.
- **F.** Idea 4: XOR/literal conflict ratio on non-decomposable targets 0.975 (0.955..0.993) at
  l = 4 against a fair A/A band of about 0.984..1.016; worse on decomposable targets; the
  symmetrised arm 2 to 10x harder; l = 5 not run. Idea 11: minimum of dim span(V.V) is 2l-1 in every
  cell searched (verified by the skeptic in six cells); the SAT contrast ran on 1 to 2 bases per arm
  instead of 16.

## What this adds up to

- No arm, at m = 3 or m = 4, goes below its scan reference; the learning arms converge to a constant
  multiple of it.
- A sound scan-following refutation exists and is far shorter than what CDCL finds, but it needs
  extension variables (or native XOR reasoning); nothing found is shorter than the scan.
- The dual-rail post-pass lowers the conflict count of the repository's ECC2K-130 SAT row at d = 8 by
  35 to 112x. It is a SAT-row result in conflicts only: wall time moves by about 1.2x and pair
  enumeration is 6 to 780x faster.
- Idea 8 is a practical negative; ideas 4 and 11 remain formally inconclusive with no lead in either.

## Limits

- Six agents shared four cores: lead F's largest cells did not finish, and one agent's cleanup
  command killed two of lead C's runs (relaunched; the skeptic found the final data complete).
- Pre-registration rests on file times in every lead; several protocols were amended.
- One curve per size in most leads; d = 9 in lead B is censored and has 2 targets.
- drat-trim, Kissat, cvc5, WDSat, research-kb and the lead atlas were not available.
