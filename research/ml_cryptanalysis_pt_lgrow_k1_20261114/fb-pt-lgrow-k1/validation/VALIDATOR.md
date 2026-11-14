# Validator report: pair-table l*(n) vs fixed l = 7 vs strong rho, K1 n = 17, 19, 23, 29

- Report: `VAL-20261114-d9190c`.
- Verdict: **passed.** All 8 RunPod session receipts and the 4 committed sizing sessions are admissible evidence. Every number in `results.json` (177 of 177) recomputes from the raw records, and all three frozen verdicts (D, R_n23, R_n29: POSITIVE) reproduce.
- Findings: two that affect how the result should be read (F3, F4), two minor disclosure errors in the protocol's sizing summary (F1, F2) and one cosmetic change-log timing note (F5). None affects admissibility or any number.
- Validator: a Validator subagent (model id redacted per repository policy), 2026-11-14.
- Snapshot: `ml-cryptanalysis` at `a1c27e9` on branch `claude/relation-yield-index-calculus-97c347`. `git ls-remote` shows that branch and `refs/pull/310/head` both at `a1c27e9`.
  - The validated inputs (sessions, `results.json`) are unchanged since `203d8e3`.
  - `score.py`, `make_specs.py`, the specs, `sizing/` and `data/PROTOCOL.frozen.sha256` are unchanged since the freeze, `609d945`.
  - `a1c27e9` only adds `figures/` and `scripts/make_figure.py`. They were not validated.
- Machine-readable files:
  - `validator-report.json`: every check, the 177 recomputed numbers, the selection, robustness and cost analyses, and the validator's 15 scripts with their sha256 and source;
  - `replay-certificate.json`;
  - `../VALIDATION.json`.

What a pass means here: the receipts are admissible, and the reported numbers are what the frozen rule gives. It does not support an ECDLP claim, it does not show a speedup over rho, and it does not authorize promotion. Rho-strong is cheaper than every IC arm at every n and weight, by 2.7 to 11.1 bits.

## Bottom line

- **R_n23 and R_n29 hold up.** Pair-table l = 9 at n = 23 and l = 10 at n = 29 are cheaper than l = 7 on the same targets, at every band weight and in bitop. Every solve is verified and every phase is priced cold, including the larger pair table. This is a verified, measured improvement over the pair-table l = 7 configuration (class: engineering, a parameter re-selection).
- **At n = 23, l = 9 is now the cheapest validated one-target IC arm** at every band weight. Cross-target, it also beats the validated ic-ridge at w = 0, by 1.53 [1.30, 1.73] bits.
- **D adds nothing beyond R.** Because l* = 7 at n = 17 and 19, D = (R_n23 + 7 R_n29) / 84 exactly, and about 84% of it comes from n = 29. On the last interval (23 to 29) the l* gap grows *faster* than the l = 7 gap at w = 0 and 0.1 (F4).
- **n is not a problem-size axis here.** K1 n = 29 has a prime subgroup of only 2^15.4, smaller than at n = 17 (F3).

## How the numbers were checked

- I wrote my own scorer from the `ecbench.record/v1` fields and the section 5a band definition, before reading `scripts/score.py`.
  - It equals `ecbench band` at every n and arm: max relative deviation 4.5e-16, and identical charged and native sums.
  - It also matches the validated ic-ridge cell (n = 23).
  - `ecbench work` reproduces every record's bitop/v1 block (480/480).
- I reproduced every number in `results.json` with my own bootstrap: point estimates to 5e-16 relative, CIs to 1.6e-15 absolute.
  - I found the percentile convention, sorted[int(B p)], by trial before reading `score.py`. A linear-interpolation percentile differs by at most 5.5e-4.
  - Then I ran the frozen `score.py` read-only. Its stdout is byte-identical to `results.json` (sha256 `886d70da...`).
- I downloaded every S3 object myself, the pod binary included. I compared the tarball with the committed files byte for byte.
- I replayed all 8 sessions and the 4 sizing sessions in full on my own host.
- I checked the planted scalars with my own GF(2^n) arithmetic at all four n, and re-derived them from the seeds.

## Checks

| # | Check | Result |
|:--|:--|:--|
| C1 | `SHA256SUMS`; S3 tarball, binary and inputs | pass: 12/12 per session; 88/88 committed session files byte-identical to the tarball |
| C2 | Binary pin | pass: every record, all 8 `session.json`, all 8 audits, the S3 binary itself, `inputs.sha256` and `binary-check.txt` |
| C3 | Freeze before data | pass: freeze `609d945` committed 06:17:52Z and pushed 06:17:54Z; first record 06:18:44.6Z |
| C4 | Spec identity | pass: `session/spec.json` = `specs/` = S3 = a fresh `make_specs.py` run, byte for byte (8/8) |
| C4b | pt params identical to the earlier cells, apart from the dimension | pass |
| C5 | Plans | pass: `ecbench plan --doc` regenerates all 8 `plan.json` byte for byte; every record follows its plan |
| C6a | In-pod `verify --replay-all` | pass: 8/8 ok, 400/400 identical, 18/18 checks each |
| C6b | Validator `verify --replay-all` on another host | pass: 8/8 ok, 400/400 identical; sizing 4/4 ok, 64/64 identical |
| C7 | Seed, workload and target disjointness | pass |
| C8 | Scalars, own EC arithmetic, n = 17, 19, 23, 29 | pass: 32/32 targets; 480/480 records recover k |
| C9 | S against `ecbench band`, bitop against `ecbench work` | pass: exact |
| C10 | Recompute of `results.json` | pass: 177 numbers, 0 out of tolerance; verdicts POSITIVE / POSITIVE / POSITIVE |
| C11 | Selection: L_STAR from the sizing | pass, with F1 (n = 19, w = 1 is a 0.0025-bit tie the other way) |
| C12 | Robustness of D and R | D > 0 under every variant; F4 on what D means |
| C13 | Claims | see "Claims" below |

Details:

- **C1, C2.**
  - S3 `out/results.tgz` (LastModified 06:20:07Z) hashes to `6f0db5bc...`. That equals S3 `out/results.sha256` and the committed `runpod-results.sha256` of all 8 sessions.
  - S3 `in/ecbench` was downloaded and hashes to the pin `a0be1b2a...` (embedded `git_commit` `21f66351`; dirty state unknown). I deleted it after hashing.
  - S3 `in/run2.sh` (`c8eddb26...`) is byte-identical to `../fb-oracle-compare-k1-eks/runpod/run2.sh` and to the in-pod `run-sh`.
  - The tarball's inner `results.sha256` (`c6e7f0fd...`) is the hash of the previous upload, because the runner tars the directory before rewriting that file. This is harmless.
- **C3, timeline.**
  - 06:03:42Z to 06:04:00Z: sizing, run locally and committed in the freeze.
  - 06:17:52Z: freeze `609d945` committed; the push is in the local reflog at 06:17:54Z.
  - 06:18:19Z to 06:18:30Z: S3 inputs uploaded (server time).
  - 06:18:44.6Z to 06:19:57.9Z: records (pod clock).
  - 06:20:07Z: S3 results uploaded (server time).
  - 06:22:14Z: data commit `203d8e3`.
  - `data/PROTOCOL.frozen.sha256` checks for the scripts and specs. `PROTOCOL.md` fails only because two change-log entries were appended: its first 4,505 bytes hash to the frozen `1c8d6600...`, which is the file at `609d945`.
- **C4b.**
  - ic-pt-l7 carries `method_sha256` `76313edb...` (`ECM1h76313edb79c1`), with params `dimension=7`, `pdp3-koblitz:m=3,engine=rootfind-negfold`, `incremental-gauss`, `max_trials` 1e7, `targets` `walk`.
  - That is identical to `fb-pt-l7-k1-n23` and `fb-ridge-vs-pairtable-k1`. The spec arm is dict-equal to both. Only the role label changed, from candidate to baseline.
  - ic-pt-l9 (`76a8ba82...`) and ic-pt-l10 (`8eb1037c...`) differ from l = 7 only in the factor-base dimension.
  - Rho is `rho.signed_frobenius_strong` (`b6123aff...`), identical to before, and uses all 2n automorphisms (34 / 38 / 46 / 58).
- **C7.**
  - The 8 measured seeds are 20262570/71, 20262590/91, 20262630/31, 20262690/91. A parse of 1,599 JSON/JSONL files elsewhere under `experiments/` (7 unparsable, covered by the literal search) collected 1,304 seed values. None of this experiment's 392 seed values (spec, algorithm and jump seeds) occurs among them, and none lies in 20262400 to 20262999.
  - `rg -uu -z` over the repository, ignored and compressed files included, finds the spec seeds only inside this experiment. The sibling repositories have no hits. The only in-range literal elsewhere is "20262400" in `.resume/RESUME.md` and `docs/session-logs/2026-11-14.md`, both describing this run's seed formula.
  - Workload ids: 0 of 32 occur among the 309 workload ids elsewhere. No target point or planted scalar is shared. No other experiment has K1 n = 29 targets.
  - The sizing seeds (99990017/19/23/29) and targets are disjoint from the measured ones and from every other experiment: 0 shared seeds, workload ids, points or scalars.
- **C8.**
  - Moduli, found by search, each the only low-weight irreducible on which G lies on E: x^17+x^3+1, x^19+x^5+x^2+x+1, x^23+x^5+1, x^29+x^2+1.
  - The Lucas traces -101, -797, -5197 and -40309 give #E = 2 x 65587, 2 x 262543, 2 x 4196903, and 2 x 6323 x 42457.
  - r is prime and is the largest prime factor; [r]G = O; [#E]P = O for 5 random P per curve.
  - For every target, k = 1 + u64(sha256(canonical({"domain": "uniform_scalar_sha256_v1", "parts": [model, r, seed, index]}))) mod (r - 1) equals the recorded planted scalar, Q is on E, and [k]G = Q.
  - All 480 records report recovered = k.

## Primary results (8 targets x 5 rounds per n; 40 verified solves per arm and n)

Gap to rho-strong in bits, the distance remaining. l* = 7 at n = 17 and 19, so there the two columns are the same arm. 95% CIs are given at n = 23 and 29:

| n (log2 r) | unit | pt-l7 | l* | R_n = l7 - l* [95% CI] |
|:--|:--|--:|--:|:--|
| 17 (16.0) | w = 0 / 0.1 / 1 | 2.67 / 2.75 / 3.20 | same | 0 by construction |
| 19 (18.0) | w = 0 / 0.1 / 1 | 3.69 / 3.75 / 4.11 | same | 0 by construction |
| 23 (22.0) | w = 0 | 6.91 | 5.59 [5.54, 5.65] | 1.317 [1.265, 1.363] |
| | w = 0.1 | 6.94 | 5.66 [5.60, 5.73] | 1.283 [1.229, 1.331] |
| | w = 1 | 7.09 | 5.95 [5.85, 6.06] | 1.141 [1.077, 1.197] |
| | bitop | 5.96 | 5.52 | 0.438 [0.375, 0.493] |
| 29 (15.4) | w = 0 | 10.36 | 9.47 [9.38, 9.54] | 0.889 [0.669, 1.061] |
| | w = 0.1 | 10.46 | 9.52 [9.43, 9.59] | 0.941 [0.722, 1.113] |
| | w = 1 | 11.06 | 9.86 [9.74, 9.95] | 1.200 [0.979, 1.379] |
| | bitop | 10.13 | 9.23 | 0.895 [0.674, 1.069] |

| unit | slope pt-l7 | slope l* | D [95% CI] |
|:--|--:|--:|:--|
| w = 0 | 0.655 | 0.565 | 0.090 [0.071, 0.104] |
| w = 0.1 | 0.657 | 0.563 | 0.094 [0.075, 0.108] |
| w = 1 | 0.669 | 0.555 | 0.114 [0.095, 0.128] |
| bitop | 0.628 | 0.548 | 0.080 [0.061, 0.094] |

- `ecbench band --ranks` agrees across the band: "ic-pt-l9 < ic-pt-l7" at n = 23, "ic-pt-l10 < ic-pt-l7" at n = 29, and rho-strong ahead of every IC arm at every n.
- Frozen predictions:
  - D > 0 with a point estimate of +0.05 to +0.13: **held**.
  - R_n23 and R_n29 POSITIVE, around 1 to 1.5 bits: **held in sign**. R_n29 = 0.89 / 0.94 at w = 0 / 0.1 is slightly below 1, and its CIs include 1.
  - l* gap slope about +0.55: **held** (0.565 / 0.563 / 0.555).

## Selection (task check 2)

I recomputed the best l by total S on the committed sizing sessions (2 targets per n, 1 round each; seeds 99990000 + n). Margins are in bits over the runner-up:

| n | w = 0 | w = 0.1 | w = 1 | bitop (not in the rule) |
|:--|:--|:--|:--|:--|
| 17 | l7 (l6 +0.12) | l7 (l6 +0.13) | l7 (l6 +0.22) | l6 |
| 19 | l7 (l8 +0.06) | l7 (l8 +0.05) | **l8 (l7 +0.0025)** | l6 |
| 23 | l9 (l8 +0.29) | l9 (l8 +0.27) | l9 (l8 +0.19) | l8 |
| 29 | l10 (l11 +0.63) | l10 (l11 +0.55) | l10 (l11 +0.012) | l10 |

- L_STAR = {17: 7, 19: 7, 23: 9, 29: 10} follows at every weight except n = 19, w = 1. There l = 8 wins by a 0.0025-bit tie on 2 solves (F1).
- The protocol's table reproduces exactly: 2.83/2.83, 3.83/3.83, 5.61/6.97 and 9.75/11.14.
- So do its sizing D (0.13) and R (1.27 to 1.61).
- Selection shrinkage on the fresh targets is as the protocol anticipated:
  - D: 0.13 to 0.15 in sizing, 0.09 to 0.11 measured.
  - R_n29: 1.34 to 1.61 in sizing, 0.89 to 1.20 measured.

## Robustness (task check 5)

| unit | LOTO D range | LONO D: drop 17 / 19 / 23 / 29 | endpoint 17->29 | segment 23->29 [95% CI] | Theil-Sen |
|:--|:--|:--|--:|:--|--:|
| w = 0 | [0.085, 0.097] | 0.076 / 0.074 / 0.079 / 0.235 | 0.074 | **-0.071 [-0.109, -0.041]** | 0.082 |
| w = 0.1 | [0.089, 0.100] | 0.082 / 0.078 / 0.083 / 0.229 | 0.078 | **-0.057 [-0.095, -0.027]** | 0.086 |
| w = 1 | [0.108, 0.120] | 0.111 / 0.100 / 0.106 / 0.204 | 0.100 | +0.010 [-0.029, +0.041] | 0.110 |
| bitop | [0.075, 0.087] | 0.088 / 0.075 / 0.079 / 0.078 | 0.075 | +0.076 [+0.038, +0.107] | 0.075 |

- **Leave-one-target-out** (32 cases per unit): D stays positive in every case. The smallest D comes from dropping n = 29 target W87ca87d45ae9. In band, R_n23 stays in [1.12, 1.33] and R_n29 in [0.83, 1.28].
  - Per-target R_n29 at w = 0.1 runs from 0.20 to 1.34. pt-l7's per-solve CV at n = 29 is 0.58, against 0.29 for l10 and 0.07 for rho.
- **Leave-one-n-out:** every 3-point D is positive, with CIs above 0. Dropping n = 29 gives D = 0.18 x R_n23.
- **Other estimators:** the endpoint slope 17 -> 29 is R_n29 / 12. It and Theil-Sen agree with OLS.
- **Bootstrap seed:** Random(1) and Random(2) move CI ends by at most 0.0077.
- **How much of D is n = 29.** With l* = 7 at 17 and 19, D = (R_n23 + 7 R_n29) / 84 exactly. The n = 29 term is about 83% / 84% / 88% of D (bitop 93%).
- **Is "bending" an artifact?** Yes, in this sense: D is a fixed positive combination of R_n23 and R_n29, so it is positive whenever l* is cheaper at 23 and 29. Any arm that matches l = 7 at small n and beats it at large n would "bend the slope".
  - The data show an offset of about 1.1 to 1.3 bits appearing between n = 19 and 23.
  - At w = 0 and 0.1 the offset shrinks between 23 and 29: the segment estimate is negative, with the CI below 0.
  - Only at w = 1 and in bitop does the offset hold or grow.
- **Rho setup amortisation.** Setup share of rho's cost:

| unit | n = 17 | 19 | 23 | 29 | rho log2 S slope, 4 points | rho log2 S slope, 17-23 |
|:--|--:|--:|--:|--:|--:|--:|
| w = 0 | 76.1% | 70.3% | 56.8% | 79.8% | +0.016 | -0.353 |
| w = 0.1 | 73.8% | 67.3% | 52.7% | 78.0% | +0.015 | -0.342 |
| w = 1 | 58.0% | 48.5% | 32.2% | 65.0% | +0.008 | -0.281 |

- Over 17, 19, 23, rho amortising its fixed setup supplies about 0.28 to 0.35 bits per bit of the gap slope, as in VAL-20261114-1227fa.
- The 4-point slope shows almost none of this, because at n = 29 the subgroup is small (r = 42457), rho is setup-dominated again, and its S jumps back up.
- Both effects are properties of these four curves, not of a size trend (F3). D and R are rho-free, since rho cancels.
- **Pseudo-A/A** (odd vs even measured rounds): R_n23 moves by at most 0.094 bits and R_n29 by at most 0.159. Both halves stay above 0.8 bits in band.

## Claims (task check 6)

- **Is R POSITIVE a verified, measured improvement over the pair-table l = 7 baseline across the band? Yes, at n = 23 and at n = 29.**
  - Same targets, same sessions and binary, and a protocol frozen before data.
  - All 40 + 40 solves verified at each n.
  - The lower CI end is above 0 at w = 0, 0.1 and 1, and in bitop.
  - Every phase is priced cold, including the pair-table build. That build is 69% of l9's and 40% of l10's charged group operations, so the gain does not rely on amortisation.
  - Total S fell, so this is not relabelling or accounting. Class: engineering.
  - Caveats:
    - At n = 29 the l = 7 baseline is this protocol's own named configuration. No other IC arm has ever run on K1 n = 29 here.
    - At n = 23 in bitop the gain is smaller, 0.44 bits.
- **Against the validated ic-ridge at n = 23** (`fb-cost-model-k1-n23-eks`; 7 other targets, same curve, binary and S; cross-target):

| unit | ridge gap | l9 gap | ridge - l9 [95% CI] |
|:--|--:|--:|:--|
| w = 0 | 7.12 | 5.59 | 1.53 [1.30, 1.73] |
| w = 0.1 | 18.86 | 5.66 | 13.20 [12.97, 13.41] |
| w = 1 | 21.53 | 5.95 | 15.59 [15.31, 15.85] |
| bitop | 19.08 | 5.52 | 13.55 [13.28, 13.82] |

  - The mean-S ratio ridge / l9 at w = 0 is 1.42 [1.23, 1.58] bits.
  - pt-l7 on these targets against ridge at w = 0 is 0.21 [-0.02, 0.42], not separated, consistent with VAL-20261114-1227fa.
  - So pt-l9 is the cheapest validated one-target IC arm at K1 n = 23 at every band weight, including w = 0, where the protocol had noted the ridge comparison was cross-target only.
- **Distance remaining to rho** (l*, w = 0 / 0.1 / 1; bitop):
  - n = 17: 2.67 / 2.75 / 3.20; 2.85.
  - n = 19: 3.69 / 3.75 / 4.11; 3.28.
  - n = 23: 5.59 / 5.66 / 5.95; 5.52.
  - n = 29: 9.47 / 9.52 / 9.86; 9.23.
- **D POSITIVE** recomputes, but it should be reported as "the weighted sum of R_n23 and R_n29", not as evidence that growing l bends the gap's growth (F4).
- **Leaderboard.** The evidence is gate-complete: `../VALIDATION.json` names the replay certificate, this report and a matched rho reference with the section 5a band. It would support two one-target cold rows, each with the l = 7 figure as the "before" mark:
  - `icv1-f2m23-tm5197-1f85e9e1` (log2 r 22.0): pair-table rootfind-negfold l = 9. Mean S 43.90 / 49.39 / 98.88; gap 5.59 / 5.66 / 5.95 (before: 6.91 / 6.94 / 7.09). Delta 1.32 / 1.28 / 1.14 bits.
  - `icv1-f2m29-tm40309-30c52b96` (log2 r 15.37, cofactor 12646; the row must show r): l = 10. Mean S 3181.6 / 3375.5 / 5119.8; gap 9.47 / 9.52 / 9.86 (before: 10.36 / 10.46 / 11.06). Delta 0.89 / 0.94 / 1.20 bits.
  - D should not be a row.
  - The crypto IC builder reads only crypto-repository paths, and the gate requires evidence inside that repository, so a crypto board row needs a frozen summary there.
  - The crypto board's K1 n = 23 row (`research/ic_descent_20260930`, 32-target batch, pre-gate) is in another regime and accounting. It cannot be ranked against this one-target cold S.
  - `rows` is omitted from `VALIDATION.json` because no row slug exists for this directory. Whether to add the rows is the Coordinator's decision.

## Findings

- **F1 (minor, selection disclosure).** PROTOCOL.md says the best l is "the same at w = 0, 0.1 and 1". At n = 19, w = 1 the sizing's best is l = 8, by 0.0025 bits. L_STAR[19] = 7 follows at w = 0 and 0.1 only. The effect on D of choosing 8 would be at most about 0.002, but that alternative is unmeasured.
- **F2 (minor).** "pt-l5 exhausted on 1 of 2 sizing targets" at n = 29: both pt-l5 records exhausted 1e7 trials with 0 relations, so it was 2 of 2. This is not load-bearing.
- **F3 (moderate, scale binding).** n is the field degree. The prime subgroups are 2^16.0, 2^18.0, 2^22.0 and 2^15.4. K1 n = 29 is 2 x 6323 x 42457 (cofactor 12646), and PROTOCOL.md does not say so.
  - At n = 29 rho is setup-dominated, and pt-l7 needs 6.4 relations per solve against 50 to 57 at n <= 23.
  - The "bits per bit of n" slopes and the n = 29 gaps describe these four curves only.
  - The next K1 degree ecbench accepts is n = 43 (r = 2^32.1). n = 21, 25, 31, 37 and 41 are refused, which I checked with `ecbench plan`.
- **F4 (moderate, primary statistic).** D = (R_n23 + 7 R_n29) / 84 identically; about 84% of it is n = 29; the 23 -> 29 segment is negative at w = 0 and 0.1. Report R_n as the result, and D only as their weighted sum.
- **F5 (cosmetic).** The launch entry ("06:18, no data yet") was committed at 06:18:49Z, 4 s after the first record, with no data-dependent content. The "data in" entry is headed 06:25 but was committed at 06:22:14Z.

## Deviations and limitations

- **Shared binary.** Both replays and the `ecbench band` / `work` cross-checks use the producer's pinned binary, so a deterministic bug shared by run and replay would not be caught. The EC check is independent code, but it covers only the logarithms, not the cost counters.
- **Isolation.** The specs say L2, the records are L0, as PROTOCOL.md declares. Only counted cost is scored, and it replays identically on another hardware class (pod `ECBHW1h792c454b90fe`, validator `ECBHW1h8bbcecb574f5`).
- **Round order.** `alternate` with 5 measured rounds is unbalanced. That is irrelevant to deterministic counted costs.
- **No A/A arm** (by design). R is 0 by construction at n = 17 and 19, so the fresh targets carry no measured null for "l* differs from 7 where 7 is optimal". The sizing gives the next-best l at +0.12 to +0.22 bits (n = 17) and -0.003 to +0.06 bits (n = 19).
- **Memory.** Max RSS is 27,656 to 27,664 KiB for every arm except pt-l10 at n = 29 (31,460 / 32,476 KiB). The pair table holds 61,297 entries for l9 and 258,442 for l10, against about 2,900 to 3,800 for l7.
- **Success probability.** All 240 IC solves verified, with at most 37,232 trials against a cap of 1e7, so expected cost = measured cost.
- **Unverifiable remarks.** The pod id, price, HTTP checks, DELETE and runtime in the change log, and the n = 43 timeout remark, cannot be checked from committed artifacts. None is load-bearing.
- **Validator independence.** Same model family as the producer (model id redacted per repository policy), separate context. The scorer, bootstrap, EC arithmetic and robustness code were written before reading `score.py`. Then `score.py` was run read-only.

## Attestation

- No `review_plan` was on the task card. The joints I owned were: admissibility of the 8 receipts and 4 sizing sessions, the recompute, and the task's checks 1 to 6.
- `read_sibling_reports`: false. No same-round reviewer reports exist.
- I read `../fb-pt-l7-k1-n23/validation/` for format, and `../fb-cost-model-k1-n23-eks/VALIDATION.json` and its records for the ridge comparison.
- Every source is listed in `validator-report.json` under `review_attestation.sources_read`.
- I did not edit sessions, `results.json`, `score.py` or `PROTOCOL.md`, and I made no commit.
