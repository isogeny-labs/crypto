# Errata (from the validator replay, `VALIDATION_REPORT.md`, 2026-11-22) [pcc:unverified]

The replay reproduced every row (204 rows, 0 mismatches) and every ratio. Two findings change how the
result reads; the frozen files are left as they are.

- **E1, the gain depends on the price of a linear-algebra multiplication.** RESULTS.md scores FPM_mid
  with κ = g² = 9 mod-r multiplications per field multiplication. At E = 0.2 the paired ratio to the
  plain arm is 1.11 / 1.10 / 1.09 at κ = 1 (worse), 0.86 / 0.85 / 0.83 at κ = 9, and 1.05 / 1.04 /
  1.03 in GAE at w = 1. Break-even κ is 2.5 to 4.1. The lab's measured κ is about 2 (PR #4852, F1),
  below break-even, so **at the measured price the selection is not an improvement**. It is one only
  if a mod-r multiplication costs more than about 3 field multiplications on the target machine,
  which is unmeasured at genus 3. RESULTS.md's "measured improvement" holds in the FPM_mid unit only.
- **E2, rho's draw.** Measured rho on these 12 curves ran at 1.13 / 1.17 / 0.73 of its expectation;
  against expected rho the best arm is 0.255 / 0.355 / 0.260, so the rise with p in RESULTS.md is
  partly the rho sample.
- Minor: the last line of `data.sha256` hashes `partial2/joined.jsonl.gz` but is labelled as the
  decompressed file.
