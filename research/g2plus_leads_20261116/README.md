# Genus >= 2 index-calculus leads L01-L12 (2026-11-16)

Twelve leads for index calculus on genus >= 2 curves, each run as its own
preregistered small-scale test. Per lead: an Opus coordinator froze
`PROTOCOL.md` and `SPEC.md` before any data, a Sonnet executor implemented and
ran it, a Haiku runner replayed the recorded commands, and an independent Opus
assessor applied the frozen decision rule and wrote `ASSESSMENT.md`.

**Standing.** Every number below is `[pcc:unverified]`: one container, one
producer per lead, a same-code replay and one assessor. No pcc claim is filed,
no validator report or replay certificate exists, and no matched rho was run
under the §5a band. Nothing here is a leaderboard row, a `benchmarks/` row or a
speed claim. Metrics are counted operations (FPM = field multiplications
+ 10 x inversions) on small primes, never wall time.

## Verdicts

Verdicts are the assessors', from the frozen rule as written. "Reading" is the
main session's one-line interpretation and is labelled as such.

| Lead | Frozen-rule verdict | Deciding numbers | Reading |
|---|---|---|---|
| L01 anchored pencil sieve | supported | Relation-phase FPM per relation, g = 2: 30-70x cheaper than the generic Cantor walk; 5-12x against an explicit-formula baseline; 3.3-3.9x once the pencil is charged for its own table set-up | Real, about 3-5x. Not a solve: descent and linear algebra are not charged, and pencil rows cost 16-465x more to eliminate |
| L02 automorphism orbits | g = 2 supported, g = 3 killed | y^2 = x^5 + a: IC/rho improves 2.46x [2.23, 2.75]. y^2 = x^7 + a: 1.10x [0.91, 1.34] | g = 2 result is at the threshold, not above it: with fixed set-up cost removed from both sides it is 1.6-2.0x |
| L03 base re-balance | supported | Re-tuning the 2LP base constant from 2 to 1 at genus 3 cuts counted cost 36-40% with no cheaper relation engine at all | The useful part is not the synthetic discount but that the lab's current constant is past its optimum |
| L04 batch smooth-part | killed | Batch arms cost 9-44x more per candidate than the per-divisor test at g = 3 | Closed for schoolbook arithmetic |
| L05 even-degree models | supported (parity) | Even-model sieve costs 0.995-1.022 of the odd-model sieve | Gain is reach (curves with no odd model), not cost |
| L06 odd-l isogeny census | void (instrument) on the rerun | Kernel criterion fires on 167/600 (27.8%), 161/600 (26.8%) and 144/600 (24.0%) prime-order curves at l = 3, 5, 7; kill bar was 1%. One gate failed: 3.75% ambiguous draws against a 2% limit | The assessor recomputed all 600 fire flags independently with 0 mismatches and judges the gate miscalibrated, not the instrument broken. Needs a re-freeze, not new mathematics. See `L06_odd_ell_isogeny_census/ASSESSMENT_RERUN.md` |
| L07 Prym index calculus | killed | 2LP exponent 16/9 (dim 4) and 20/11 (dim 5), above the hyperelliptic 3/2 and 8/5 | As predicted |
| L08 non-hyperelliptic ladder | inconclusive (instrument) | Duplicate lines were counted as useful relations in the 2LP arms | The apparent sub-4/3 slope is an artefact. Needs a corrected rerun |
| L09 gonal pencil seeds | inconclusive | A correctness gate was unreachable as frozen | Proxy saving falls from 31% to 7% as p grows; even if valid, a shrinking constant |
| L10 Picard targets, matched rho | orbit gain inconclusive; rho accounting supported | Orbit fold 9.0x on counted cost but two tiny-curve gates failed. Lab's quartic negation-walk rho is overpriced by 1.73x | The accounting point affects existing quartic rows and should go to their owners |
| L11 Laine-Lauter memory | inconclusive | Halving memory costs 2.1-2.4x the line trials, against a 2.0x bar | Memory bounding never reduced time |
| L12 bielliptic orbits | void (instrument) | One run ended with a rank-deficient system: the stop rule is defective | The lab's existing fold already beats the orbit arm by 1.4-1.6x; no reason to rerun |

## What moved the baseline, and what did not

- **Exponent.** No lead changed an exponent. L06, the only lead that could,
  is a necessary first step only: it counts curves that have a rational
  (l,l,l) kernel for odd l. The isogeny, and the proof that its codomain is a
  plane quartic Jacobian, are not computed. About a quarter of prime-order
  curves pass the criterion per l, on numbers the frozen rule voids.
- **Constants with a verified-looking mechanism.** L01 (cheaper relations at
  genus 2), L03 (base constant re-tuning at genus 3) and L02 at genus 2. Each
  is a relation-phase or component measurement, not an end-to-end solve
  against matched rho. Under the repository's success criterion they are
  candidates for an improvement, not improvements, until re-measured in the
  lab instrument with a replay certificate and the §5a band.
- **Distance to rho, as the assessors estimate it.** Genus 2: the lab 2LP arm
  is 3-5x above rho; L01's saving would put it at roughly 1-3x, unmeasured.
  Genus 3: the lab arm is 0.54-0.74 of rho; L03's re-tuning would give about
  0.33-0.47. Both figures rest on unverified lab numbers.

## Corrections this pass makes to earlier statements

- The lead brief for L01 predicted 25-30 FPM per genus-2 relation; the
  coordinator's count with forward differences is about 15.
- L07's protocol misstates Diem's exponent for genus 2g+1; the assessor notes
  it does not affect the verdict.
- L10: the lab's matched rho for generic quartics appears 1.73x too expensive,
  which makes the reported IC/rho ratios for those rows 1.73x too favourable.

## Known problems with this pass

- **L06 was skipped silently in the first pass.** The coordinator returned
  `proceed = true` without writing files; the executor correctly refused to
  invent a spec. It was rerun separately. The rerun's replay reused the
  producer's binary, so it shows determinism, not correctness.
- **L06, l = 2.** The rerun's derivation states that every odd-order genus-3
  hyperelliptic Jacobian has two rational (2,2,2) Lagrangians of non-pair
  type (1583 of 1583 in the census), which Smith's construction does not
  cover. The lab's closure of Smith's route for prime order therefore holds
  for pair-type kernels only. Whether the non-pair isogeny is computable is
  open and is not claimed here.
- **A `git stash` by the main session** briefly reverted tracked files while
  the L03 replay was writing. Checked afterwards: the affected replay cell is
  byte-identical to its raw original and `replay/REPLAY.md` lists no differing
  file.
- **Instrument defects found by assessors:** L08 (duplicate counting), L09
  (unreachable gate), L10 (gate infeasible on tiny curves), L12 (stop rule).
  Three of four are protocol design errors by the coordinator, caught only at
  assessment.
- **Data custody.** Raw outputs are in Git because this session has no S3
  credentials. Two L05 gate dumps (11.5 MB, 8.6 MB) and replay duplicates are
  untracked and exist only on the session's disk; hashes are in each lead's
  `raw/data.sha256` or `raw/COMMANDS.log`.
- **Prior art.** The research-kb tool was not installed; novelty checks were
  by grep. L01's sieve mechanism is prior art in the repository
  (`research/more_leads_20261019`); only the fixed-anchor variant and the
  genus-3 measurement are new here.

## Next steps, in order

1. L06: re-freeze with the ambiguity gate measured at the census primes, or
   with ambiguous draws excluded by a pre-stated rule, and rerun. Then decide
   whether to fund a theta-coordinate (l,l,l)-isogeny implementation.
2. L03: re-measure base constant 1 against 2 in `src/bin/genus_ladder` on the
   same nine genus-3 curves, under a frozen paired spec.
3. L01: end-to-end 2LP at genus 2 and 3 with descent, table set-up and
   elimination charged, against matched rho.
4. L10: hand the 1.73x rho correction to the owners of the quartic rows.
5. L08: corrected rerun (count each line once; charge linear algebra after
   large-prime elimination).
