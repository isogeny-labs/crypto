# EXP-AUXIN-25a29a, executed outside its ledger (2026-11-12)

**Status.** `[pcc:unverified]`. Executed in a `crypto` session against the frozen specification read from the
`crypto-autoresearcher` vault (`experiments/EXP-AUXIN-25a29a/specification.yaml`, approved by DEC-20261002-c548c0). This session can
read but not push to `crypto-autoresearcher`, so these files are **not** the run package the specification names
(`experiments/EXP-AUXIN-25a29a/runs/<RUN-ID>/…`) and no RUN id was minted. A session with write access should mint the id and
copy or re-run these two scripts there.

## What the experiment is

Not a curve and not a walk. H-AUXIN-a86953 and the specification say so ("No curve is built… It does not run a walk"; invalidation
rule: "A curve is built or a walk is run"). It checks the integer arithmetic behind IDEA-20260930-704e43's memory-charged Cheon pricing
at 56 bits and divisor 65536. The atlas's next-step text for `LEAD-RHO-cheon-toy-reproduction` ("end to end with recovered scalars
certified and memory/DP costs reported beside rho") describes more than this contract does.

## Runs

| route | script | command | exit | output |
|:--|:--|:--|--:|:--|
| `power_cross` | [`route_power_cross.sh`](route_power_cross.sh) | `bash route_power_cross.sh` | 0 | [`raw-result_power_cross.json`](raw-result_power_cross.json) |
| `reduced_fraction` | [`route_reduced_fraction.sh`](route_reduced_fraction.sh) | `bash route_reduced_fraction.sh` | 0 | [`raw-result_reduced_fraction.json`](raw-result_reduced_fraction.json) |

The two scripts share no code and do not source each other. Route A uses bash 64-bit integer arithmetic with cross-multiplication.
Route B reduces each fraction by gcd in awk and subtracts the reduced fractions with bc. Tools: GNU bash 5.2.21, bc 1.07.1,
mawk 1.3.4. Both stderr logs are empty. Two runs in total, as the budget allows.

## Against the specification

| check | preregistered | route A | route B | |
|:--|:--|:--|:--|:--|
| stage-2 count, = 2⁹ | 512 | 512, `true` | 512, log₂ = 9 | pass |
| 5¹⁰ − 8·4¹⁰ | +1377017 | 1377017 | 1377017 | pass |
| 16·4¹⁰ − 5¹⁰ | +7011591 | 7011591 | 7011591 | pass |
| g₈₄ − 2 | +1234/4375 | 1234 (den 4375) | 1234/4375 | pass |
| 12/5 − g₈₄ | +516/4375 | 516 (den 4375; 12·4375/5 exact) | 516/4375 | pass |
| g₇ − 24 | +14808/4375 | 14808 (den 4375) | 14808/4375 | pass |
| g₇ − 27, 28 − g₇ | +1683/4375, +2692/4375 | 1683, 2692 | 1683/4375, 2692/4375 | pass |
| null square vs 13/100 square | +1145714375 | 1145714375 | 1145714375 | pass |
| one-half square vs null square | +17388433 | 17388433 | 17388433 | pass |

**Verdict under the specification's own rules: success.** Both routes agree on every value, and no invalidation rule fires.

## What this does and does not show

- It confirms the stage counts and signed gaps of IDEA-20260930-704e43 at one bit length. The specification's method ceiling:
  "The stage counts and the signed gaps. Not a measured walk constant and not an attack cost."
- It does not reproduce Cheon's algorithm. The measured comparison of Cheon against matched rho is
  `research/cheon_ecbench_wide_20261106/` (36–56 bits, P-224-shaped `d`), which does not charge memory for a parallel stage 1.
  No experiment in either repository yet runs the 56-bit, `d = 65536` walk with memory charged.
