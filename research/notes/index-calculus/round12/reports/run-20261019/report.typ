#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: hyperelliptic IC round twelve, rho branch-coefficient width

*Card* `CARD-HEIC-ROUND12-1` (sha256 `1f599093…9668b0`), commit `be98f8e7`; binary `hyperelliptic_rho_branch_width` sha256 `9ca561b5…391e63`, built from `be98f8e7` (`examples/hyperelliptic_rho_branch_width.rs`). Receipt `../../run1/receipt.json`: status completed, all nine commands exit 0.

*Host and isolation.* Intel Xeon @ 2.80 GHz, 4 logical CPUs, kernel 6.18.44-fc-v139, x86-64. `tools/isolated_bench.py`, reserved CPU 3, preflight `--max-other-cpu 1.0`; record `contended: false`, wall 529.0 s. Wall time is not a reported value; all values are counted operations, deterministic in the seed.

*Arms and workload.* Plain distinguished-point Pollard rho, 16 branches, on the Jacobian and on `Z/N` (the integer model of round eleven), each with 8-bit branch coefficients (the old reference) and with full-width coefficients `a_j, b_j` uniform in `[0, N)`. Same seeds `0xC0FFEE + t`, t < 40, on the thread's 18 instances. Requested 18 × 40 × 4 runs; every recovered `k` verified against the planted one.

#figure(image("swalk.svg", width: 100%), caption: [Recorded `S_walk` per row (mean over 40 seeds), four arms, against the random-map constant. Source: `../../run1/R12.txt`.])

*Recorded verdict lines* (printed by the binary):
- W1, integer model, full width, the 7 rows with N ≥ 10^5: median 1.303 (in [1.10, 1.40]); two largest rows 1.505 and 1.303 (bound 1.55): *holds*.
- W2, Jacobian, full width: median 1.390; two largest rows 1.481 and 1.212: *holds*.
- W3, control at N ≤ 6·10^4: Jacobian median moves by 0.025 (bound 0.15): holds.
- W4, every run verified: holds.
- Largest row (N = 16 790 591): Jacobian 8-bit / full-width `S_walk` = 1.418; full-width precomputation 884 operations = 0.216·√N, reported separately.

*Observations.* With 8-bit coefficients both the Jacobian and the integer model drift above the ideal as N grows (2.100 and 2.171 on the largest row); with full-width coefficients both stay within 1.17–1.51 on every row with N ≥ 10^5. The excess of rounds nine to eleven was therefore the reference walk's own step rule, not the group, the hash or the detection logic. This is an accounting correction against index calculus: IC/rho ratios measured against the 8-bit reference at large N flattered index calculus by up to the row's 8-bit/full factor (1.42–1.50 on the two largest rows). Those ratios were already labelled lower-bound and unranked; they are to be re-measured, not rescaled.

*Missing.* No cross-method comparison is made in this run, and no arm emits its unpriced counters (hash evaluations, table probes), so §5a pricing cannot be applied. One seed set per row; no interval is reported on `S_walk`.
