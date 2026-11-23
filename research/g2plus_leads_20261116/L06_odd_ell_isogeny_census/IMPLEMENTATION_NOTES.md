# L06 implementation notes (executor)

Crate `l06/` follows SPEC.md section 1, Rust standard library only, one thread. Where SPEC.md was
silent the simplest correct option was taken:

1. Unit tests are in `src/tests.rs` (`#[cfg(test)]`), 11 tests, one per SPEC.md section 11 item.
2. `Poly` is a fixed array of 24 `u32` coefficients with a length; one type serves F_p and F_l
   polynomials (modulus carried by the `Fp` context). Overflow of 24 asserts.
3. Inverses and the Legendre symbol come from tables built in `Fp::new`; `Fp::legendre_pow`
   (Fermat) is checked against the table in test 3. `sqrt` is Tonelli-Shanks with the smallest
   non-residue.
4. `bsgs_all`: the SPEC.md 4.3 table is a sorted `Vec<(key, j)>` with binary search instead of a
   `HashMap`. Collection stops once more than 4096 candidates are found (so a small-order divisor
   cannot exhaust memory) and the call reports "too many"; SPEC.md 4.3 then redraws D1, at most
   three redraws (four attempts), then `status = ambiguous`. `n_candidates_J` is the candidate
   count after the 7-divisor filter (or the capped count on the too-many exit). If a twist search
   exceeds the cap, `ambiguous` is recorded with `n_candidates_pair = 17`.
5. `random_class` returns the identity after 100000 failed attempts so that tiny-p unit tests
   cannot hang. It was never reached in the census (p >= 257). Unit test 6 skips curves over F_11,
   F_13 with fewer than 4 projective points for the same reason.
6. The row key of the J[2] table is the 8-point cycle type: the partition of 7 followed by a
   trailing 1 (SPEC.md 8.4 and unit test 11 `[7, 1]`, `[1^8]`). The census looks the row up by
   its own `cycle_type`, which has the same form.
7. `Crit` has an extra `squarefree` field (needed for the reference sampler's `squarefree` count,
   which counts samples whose chi mod l is squarefree, fire or not) and `crit` asserts
   `c[0] = q^g` (SPEC.md unit test 8 rule; g = deg/2).
8. Distinct-degree factorisation stops when the remaining polynomial has degree <= 0 (SPEC.md 3).
9. The census loop writes and flushes after every chi-computed line; `run_summary.u_count` is the
   number of squarefree draws seen in the U block. A draw is in block U while fewer than `u_count`
   squarefree draws had been seen before it (non-squarefree draws take the block of their moment).
10. `gate-enum` p = 257 allocates 17 MB tables and enumerates F_{p^3}; element index is
    `a0 + a1 p + a2 p^2`; field polynomial `t^3 + a1 t + a0` with the first (a1, a0) per SPEC.md 5.
11. `analyze.py` takes G8 as `ambiguous <= 2%` of chi-requested draws (status ok, ambiguous or
    error); `error` counts are reported separately (0 observed). G4 is recomputed from the
    recorded fields (`fire == fire_fac` and `n_h == 2^n_pairs` when fire). ALL, C2, ODD use U-block
    draws with status ok; ambiguous or error draws are excluded from them and counted.
    The line `outcome_of_rules_2_to_5_regardless_of_gates_NOT_THE_VERDICT` is an addition; it
    does not feed the verdict.
12. Seeds are exactly those of SPEC.md section 2; no other randomness is used.
