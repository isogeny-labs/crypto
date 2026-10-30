# T22 / T23 next cards (from T21, `REPORT.typ`)

T22: reduce_fold in vector registers (u128 field, n = 73, 83).
- Why: `reduce_fold` is 16% of the collected Ir of the K = 40 profile and, unlike the u64 type's `fold_shifted`, moves its operands between general and vector registers around three carry-less products (hypothesis; first take a per-instruction profile).
- Test: same-binary scan gate as T21 (K = 600, 20,000 states, 7 rotated rounds, A/A), switch off by default, unit test equal to the fold on 20,000 random and edge inputs per modulus.
- Kill: median ratio above 0.8333 or bootstrap upper bound above 0.90 (the T21 gate); the instruction count must also fall.

T23: u128 Miller-Rabin plus a small-prime cofactor split in `factorise_u128`, so `KoblitzCurve::new` is fast at n = 83 and above (the n = 83 order is above 2^64 after the cofactor, so the T21 early stop does not apply and the trial division would not finish).
- Test: factor list equal to the old trial division on every order that terminates, plus known n = 83 and 89 orders; process-start A/B as T21 stage P with an A/A arm.
- Kill: any difference in the factor list.
