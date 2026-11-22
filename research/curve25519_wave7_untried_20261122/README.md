# Wave 7: the five untried Curve25519 index-calculus leads (2026-11-22)

**Class.** Five frozen measurements on the Curve25519-family miniatures (k = 16 to 24) and, for one
lead, on the real Curve25519. Each lead has its own protocol frozen before its confirmatory run, its
own README, report PDF, figure and an independent validator report with errata applied. These are
coverage censuses, exact root counts, relation-yield counts, a counted Macaulay proxy and a derived
extrapolation. None is an end-to-end solve, an ecbench session, or a leaderboard or `benchmarks/` row.
No pcc claim was registered, so every result below is an unverified assertion [pcc:unverified].

**Origin.** The owner asked (2026-11-22) whether point hashing, auxiliary curves and twists, point
representations, coordinate types and curve equations had been tried. A repository sweep found that
most had been tried or closed by argument and listed five items never measured. Owner: "Work on the
leads above in parallel". Shared brief: [`BRIEF.md`](BRIEF.md).

## Answers

| lead | question | answer | predictions | validator |
|---|---|---|---|---|
| [L1](L1_hash_base/README.md) | Does a hash-defined factor base yield relations at the bits or random rate? | Yes, same rate: hash/random 1.0013 [0.9978, 1.0048]; a planted 3 % effect was detected. A hash base costs more and cannot be used algebraically. | 10 of 10 held | pass_with_errata (9) |
| [L2](L2_highdeg_coords/README.md) | Do degree ≥ 3 coordinate functions from projective, Jacobian or extended charts beat x-frames? | No positive: coverage per column 0.993–1.007 in the 45 qualifying cells; dense systems refute only at full degree. Mixed outcome under its frozen rule: instrument valid, positive rule not met, negative rule not met because P1x failed in one post-hoc cell. | P1x, P4 total degree, P5a, P5g failed | pass_with_errata (11) |
| [L3](L3_translated_involution/README.md) | Are coordinates invariant under P ↦ T − P (Huff, Jacobi intersection on E/<T0>) new and better? | New only when T has no rational half, and then worse (0.64–0.81× at t = 4). Jacobi-intersection coordinates have degree 4 and cover about half. Small gain for x-frames on E′ (1.21–1.24× at t = 4, 1–2 % at t = 8), not yet engine-checked. | 16 of 16 held | pass_with_errata (9) |
| [L4](L4_twist_roots/README.md) | Do mixed curve/twist pairs give F_p summation roots? | Never: 0 of 1,070,453,835 pairs at k = 16 (exhaustive). Twist points add no relation for a target on E. | 9 of 10 held as worded (P6 fails on the x = 0 pair) | pass_with_errata (10) |
| [L5](L5_selector_255/README.md) | How do the frame and seed selectors scale with t, and what remains at 255 bits? | Gain − 1 falls as about 2^(−t/2) and is a column-count effect. Derived extrapolation at t* = 128: about 4·10⁻¹⁹. | 20 of 20 held | pass_with_errata (12) |

All entries are [pcc:unverified]. The validator counts are errata counts; every validator reproduced
the measured numbers and verdicts with its own code.

## What this changes

- Four of the five themes the owner named are now closed by measurement on the miniatures: point
  hashing (L1), twists (L4), coordinate types (L2) and the translated-involution curve models (L3)
  [pcc:unverified]. The fifth, selection toward 255 bits (L5), is closed by a measured scaling law
  plus a derived extrapolation [pcc:unverified].
- One small positive remains open: x-frames on the 2-isogenous curve E′ cover 1.21–1.24× the E
  x-frames at t = 4, fading to 1–2 % at t = 8 (L3) [pcc:unverified]. It needs an F4 engine run on E′
  against E, with the isogeny transport charged, before anyone relies on it.
- No leaderboard row, `benchmarks/` row or distance to rho changes. Rho remains ahead on every track;
  the distances quoted in the lead READMEs are imported from the studies they cite [pcc:unverified].

## Disclosures

- No Conductor task, EKS run, S3 checkpoint (upload token expired), benchrec record, VALIDATION.json or
  pcc claim was reachable from this runtime. The research knowledge base and the lead atlas were
  unreachable; each lead lists the repository files it read.
- All five runs shared one 4-core host, each on one core. Wall times are indicative only; every
  verdict rests on counted or exact quantities.
- L1, L2 and L5 smoke tests used the same seed rule as their data runs, so some smoke rows coincide
  with data rows; each README lists them and the validators recomputed the verdicts without them.
  L2's design also saw some k = 16 numbers before its freeze; its README marks the post-hoc
  predictions.
- Data-suffixed outputs above 64 KB (npz, jsonl, one CSV) stay on the run host because the artifact
  store refused uploads. Each lead commits lossless JSON mirrors (exact reload checks) and SHA-256
  sums of the originals.
- The brief had two errors the leads corrected: the Montgomery S3 coefficients were swapped (L4), and
  the halves of an odd-log point lie over F_{p^4}, not F_{p^2} (L3).

## Files

- `BRIEF.md`: the shared brief. `.gitignore`: the data-file rule.
- `L1_hash_base/`, `L2_highdeg_coords/`, `L3_translated_involution/`, `L4_twist_roots/`,
  `L5_selector_255/`: each with PROTOCOL.md, code, run.log, data mirrors, README.md, REPORT.pdf, a
  figure and validator-report.json.
