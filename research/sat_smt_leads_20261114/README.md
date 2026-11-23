# Twelve SAT/SMT index-calculus ideas: local pilots (2026-11-14)

Local toy-size pilots of the twelve proposals logged on 2026-11-13 (PR #4069), one producer and one
independent skeptic per idea, run as a 24-agent workflow in a 4-core container. These are pilots, not
claim data: no frozen protocol in Git before the runs, no isolab host, no replay certificate, no
validator report, no matched rho arm. Every number in this directory is `[pcc:unverified]`. No
leaderboard or `benchmarks/` row follows from it.

Common setup: binary curves over F_2^n with n between 7 and 31, subspace factor base of dimension
l between 1 and 10, m = 3 summands (m = 4 for idea 3), CaDiCaL 1.5.3 / Glucose 4 (python-sat),
CryptoMiniSat (pycryptosat) and Z3. Each agent wrote its own small instance generator; each skeptic
re-checked the smallest cell against its own brute-force enumeration and re-ran the producer's
smallest cell.

| # | Idea | Producer verdict | After skeptic | Smallest cell reproduced | Review |
|---|------|------------------|---------------|--------------------------|--------|
| 1 | [Proof-size slope as a go/no-go gate](01-proof-size-slope/README.md) | kill criterion met | kill criterion met | partly | [review](01-proof-size-slope/SKEPTIC.md) |
| 2 | [Linear-leaf XCNF](02-linear-leaf-xcnf/README.md) | inconclusive | kill criterion met | yes | [review](02-linear-leaf-xcnf/SKEPTIC.md) |
| 3 | [Cut-variable branching with bounded nogood memory](03-cut-variable-branching/README.md) | kill criterion met | kill criterion met | yes | [review](03-cut-variable-branching/SKEPTIC.md) |
| 4 | [XOR cubes on symmetric functions](04-xor-cubes-symmetric/README.md) | inconclusive | inconclusive | yes | [review](04-xor-cubes-symmetric/SKEPTIC.md) |
| 5 | [Conflict-directed polynomial lemmas](05-conflict-directed-poly-lemmas/README.md) | kill criterion met | kill criterion met | yes | [review](05-conflict-directed-poly-lemmas/SKEPTIC.md) |
| 6 | [BDD/ZDD bucket elimination](06-bdd-bucket-elimination/README.md) | kill criterion met | kill criterion met | yes | [review](06-bdd-bucket-elimination/SKEPTIC.md) |
| 7 | [One homogeneous AllSAT instance for factor-base-only relations](07-homogeneous-allsat/README.md) | kill criterion met | kill criterion met | yes | [review](07-homogeneous-allsat/SKEPTIC.md) |
| 8 | [Target-parametric incremental solving](08-target-parametric/README.md) | inconclusive | inconclusive | yes | [review](08-target-parametric/SKEPTIC.md) |
| 9 | [Solution-rich regime with budgets and XOR streamlining](09-solution-rich-regime/README.md) | kill criterion met | kill criterion met | partly | [review](09-solution-rich-regime/SKEPTIC.md) |
| 10 | [Bit-sliced multi-target propagation](10-bitsliced-multitarget/README.md) | kill criterion met | kill criterion met | partly | [review](10-bitsliced-multitarget/SKEPTIC.md) |
| 11 | [Solver-driven synthesis of the factor base](11-smt-factor-base-synthesis/README.md) | kill criterion met | inconclusive | yes | [review](11-smt-factor-base-synthesis/SKEPTIC.md) |
| 12 | [MaxSAT for relation filtering and structured elimination](12-maxsat-filtering/README.md) | kill criterion met | kill criterion met | yes | [review](12-maxsat-filtering/SKEPTIC.md) |

Result in one line: no idea is alive. Nine meet their pre-written kill criterion, three are
inconclusive (4, 8, 11), and none shows a SAT arm at or below the linear 2+1 scan count 2^l.

## Readings worth carrying forward

- **Idea 1 (the gate).** Backward-trimmed DRAT cores grow at 3.27 bits per l against 3.34 for
  conflicts (n = 17, l = 2..5). Trimming proofs that CDCL found does not expose short refutations.
  This does not show that no short refutation exists: the 2^l linear 2+1 certificate was not
  translated into a clausal proof, and XOR-aware or algebraic proof systems were not tried.
- **Idea 2.** A solver given the linear-leaf encoding does not find the scan by itself: the best
  solver-driven arm is over 1,000x the scan count. Only the arm where an outer loop does the scan
  reaches 2^l, and that is the scan by construction. The skeptic changed the verdict from
  inconclusive to kill because the producer had replaced the assigned kill criterion.
- **Idea 8.** Incremental solving with the target as assumptions used 0.699x the conflicts of a
  fresh solver at l = 5 (threshold 0.7; bootstrap interval 0.45 to 1.06), against the weakest of
  three fresh arms; it is 1.04x the per-target best fresh arm. Minimal target-bit cores match a
  random counting null, so no cheap decomposability filter appears. Unresolved: a mild downward
  drift of the per-position ratio on UNSAT targets.
- **Idea 11.** The lowest-defect base found is the polynomial-basis span the linear baseline
  already uses; no search arm went below it. Nothing new to synthesise at these sizes.
- **Idea 4.** With e1 fully fixed, S4 keeps e3^3 and e2^2*e3: the residual is never affine before
  k = l, so there is no sub-scan decision by this route.

## Limits

- Twelve different generators and encodings, most of them weaker than the repository's own
  (conflict slopes above 3 bits per l at m = 3), so cells are not comparable across ideas and a
  kill here covers the pilot encoding only.
- Sizes are far below anything of cryptographic interest; slopes rest on three or four sizes and
  in several pilots n grows with l.
- Several producers changed sizes after a cost probe; the reviews list each case.
- drat-trim, Kissat, cvc5, WDSat, research-kb and the lead atlas were not available.

`workflow_result.json` holds the structured producer and skeptic returns.

## Follow-ups (same day)

Two further local pilots, each with a producer and an independent skeptic. Same standing as above:
pilots, every number `[pcc:unverified]`.

| Follow-up | Decision per its pre-written rule | After skeptic | Directory |
|---|---|---|---|
| Sub-scan test: can a learning solver refute a block of x1 values for less than its leaf count? | kill | kill by the letter; the total-work leg is solid at l <= 8, the exponent leg rests on l = 4 only | [followup-subcube](followup-subcube/README.md), [review](followup-subcube/SKEPTIC.md) |
| Idea 8 powered rerun (60 targets per cell, best fresh arm as baseline, A/A floor) | inconclusive | inconclusive, knife-edge and baseline-dependent | [followup-08-powered](followup-08-powered/README.md), [review](followup-08-powered/SKEPTIC.md) |

- **Sub-scan.** (leaves + conflicts)/2^l is at least 2.48 at l = 8 against the linear 2+1 scan count;
  the lowest arm is the one that attempts no subcube refutation. In the skeptic's instrumented run a
  warm solver refuted one 64-leaf subcube for 0 marginal conflicts and a 32-leaf one for 2; neither
  amortises to the root.
- **Dual-rail encoding.** With x1 assigned, CryptoMiniSat needs about 1.6 conflicts per leaf warm
  (about 20 for a fresh solver), against 151 (l = 4) to more than 19,570 (l = 8) with plain AND
  gates. The skeptic confirmed it with identical solver options; the gain needs rails on the
  u&p, u&s and s&z gates as well as the XOR parity, not only the gate the producer named. This is
  the first SAT arm in these pilots within a small constant of the scan count.
- **Idea 8.** Incremental/best-fresh conflict ratio on non-decomposable targets: 0.590, 0.747,
  0.793 (0.773..0.812) at l = 3, 4, 5; the threshold was 0.8. The ratio rises with l (corrected
  interval for the l = 5 minus l = 4 difference: 0.014..0.106). Along a sequence it keeps falling
  but decelerates (skeptic's 510-target run at l = 4: 0.85, 0.76, 0.65, 0.60 near positions 10, 50,
  200, 450). Propagations are at least 1.14x the fresh arm and wall time 2.1 to 2.4x. Reading: a
  bounded constant in conflicts that shrinks with l and does not carry over to work done.
- Known defects, no measured row affected: the sub-scan encoding gives a false SAT at u = 0 through
  twist points; idea 8's CNF has no rationality constraint; both protocols predate the data by file
  times only.
