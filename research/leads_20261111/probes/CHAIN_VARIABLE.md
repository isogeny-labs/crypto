# Probe: LEAD-ICB-d4bin-chain-variable-sweep, step 1 (2026-11-12)

Result `[pcc:unverified]`: **closed, falsified on its premise.**

- **The premise was wrong.** The lead assumed the current chain unknown, P (index
  3), has k = 16 chains (inferred, not recorded). Measured, it has **k = 12**, which
  is already the value the lead hoped to reach.
- **The only other admissible choice is worse.** That choice is s4 (index 2), the
  other weight-1 unknown of `FROBENIUS_WEIGHTS = [2, 2, 1, 1]`.

| chain unknown | chains | charpoly muls | NF muls | total muls per trial |
|:--|--:|--:|--:|--:|
| P (index 3, current) | 12 | 87,417 | 21,779 | 190,239 |
| s4 (index 2) | 28 | 322,246 | 34,696 | 445,423 (2.34×) |

**Run.**
- m = 8, seed 1, 50 targets.
- `D4_BIN_MVAR={3,2} RAYON_NUM_THREADS=1 target/release/examples/d4_char2_profile --m 8 --seed 1 --trials 50`.
- Raw output: `chain_variable_m8.txt`.

**Correctness.** Both arms return the same rational solutions: 1.080 per trial,
16 trials with none, 0 disagreements between the baseline and `frob_roots` paths.

**Code.** `D4_BIN_MVAR` in `src/cryptanalysis/gaudry_quartic_bin.rs` is an opt-in
override. It is ignored unless set to 2 or 3, and the default is unchanged.

**Not tested.** Other variable orders. Those need a change to the monomial order,
not just the chain unknown. The lead gave a 10 % prior to any improvement, and the
k = 12 measurement removes its stated mechanism.
