# VERIFY: independent skeptic check of B_d4bin_probes (2026-11-12)

Lead checked: LEAD-ICB-d4bin-elimination-and-coset (the exponent-coset probe and step 1 of
constant-quadratic-elimination). All statements below are [pcc:unverified]. No pcc claim has been filed.

## Re-runs (single-threaded; the existing binaries were copied to the scratchpad)

The probe binary sha256 is 3edd8436...ffe1a3cb9, which matches `binary.sha256`. Both binaries
(probe at 06:04, profile at 06:17) were built after the last source edits (05:52 and 05:53).

| Command | Outcome |
|---|---|
| `d4bin_coset_quadratic_probe coset 8 100` | Identical to `coset_m8.txt`, timing lines excluded (diff empty) |
| `d4bin_coset_quadratic_probe quad 12 300` | Identical to `quad_m12.txt`, timing lines excluded (diff empty). The rank histogram is {2: 300}, the leads are [S2^2, S3^2, S2 s4^2, s4^4] on 300 of 300 targets, the staircase is {(0,0):32, (0,1):16, (1,0):12, (1,1):4}, and dim is 64 = 64 on 300 targets |
| `d4_char2_profile --m 12 --seed 1 --trials 300` | Identical to `control_profile_m12.txt`. It also equals `research/ic_d4_char2_lever_20261023/REPORT.md` line 52: TOTAL 224,196 -> 191,435, echelon 54,809, nf 21,913, charpoly 87,728, roots 55,505 -> 22,977, 0 disagreements. Chains print as 12.0 |

I did not re-run coset at m = 10 or m = 12 (40 s and 11 min). The m = 8 run is bit-identical, and
the m = 12 quad run independently shows the same 21-monomial support on 1200 components.

## Code review

- `git diff` on `gaudry_quartic_bin.rs` shows 52 insertions and 0 deletions: one new function,
  `macaulay_staircase`, which no solver path calls. The default path is unchanged, and the
  bit-identical profile reproduction above supports that.
- The probe computes the F_2-rank of the parity differences and tests all 15 nonzero mod-2 forms.
  This is broader than the two forms the lane named, so the lane's pigeonhole shortcut is not relied
  on. The synthetic positive and negative controls are asserted in code.
- The exponent reduction [e0/2, e1/2, e2, e3/2] matches FROBENIUS_WEIGHTS = [2,2,1,1]
  (S2, S3, s4, P), and the wdeg printout agrees with it.
- The absence of S2*S3 is structural. [1,1,0,0] is missing from the symbolic carrying support on
  all 300 coset instances, not only from the numeric targets. So rank <= 2 is forced, and it is
  not a numerical accident.
- The quad targets use the same curve (find_bin_curve(12,1,16)), precompute RNG (1^0x9e) and
  target RNG (77) as d4_char2_profile.

## Against the preregistered falsifiers (lane_B_binary_ic.md sections 3 and 4)

- Coset: the falsifier is "both forms fail on at least one carrying monomial of at least one
  instance". Both forms fail on every instance, so the falsifier fired. The lead is closed and the
  exponent vectors are recorded. No threshold was moved.
- Elimination step 1: the falsifier is "rank < 3 on any target". It fired on 300 of 300 targets.
  The second falsifier (staircase already {1,S3} x 32) correctly did not fire. No threshold was
  moved.

## Caveats (none overturns the verdict)

1. The lane text says that rank < 3 makes the lever "conditional"; it does not say the lever is
   dead. "Falsified" is right for the lead's premise and claim as written. But rank 2 means there
   are two equations linear in (S2, S3). That is arguably better for an elimination route, and it
   has not been tested. The RESULT already says this and proposes it as a new preregistration. It
   must not be read as evidence that elimination cannot work.
2. The module unit tests were not run. The default-unchanged evidence rests on the additive diff
   and the bit-identical profile reproduction, which I confirmed.
3. One curve was used for quad (m = 12, seed 1). Because the support is structural (the symbolic
   support is the same on 300 instances), this matters little.
4. The coset prediction's rationale (that S2 alone is carrying) was wrong in detail. The RESULT
   notes this, and it does not affect the outcome.
5. Side fact: chains = 12.0, against the lane's inferred k = 16. I confirmed it from the profile
   output.

Verdict: I agree with the report [pcc:unverified].
