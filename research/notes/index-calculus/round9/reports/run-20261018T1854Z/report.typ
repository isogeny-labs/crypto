#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: hyperelliptic IC round nine, negation-map rho

*Card* `CARD-HEIC-ROUND9-5` (sha256 `78dfce16…fdcb0`), commit `b6b41402`; binary `rho_neg_bin` sha256 `92c8fc22…ffdb558`, built from `13f21480` (`examples/hyperelliptic_rho_negation.rs`). Receipt `../../run5/receipt.json`: status completed, all nine commands exit 0.

*Host and isolation.* Intel Xeon @ 2.80 GHz, 4 logical CPUs, kernel 6.18.44-fc-v139, x86-64. `tools/isolated_bench.py`, reserved CPU 3, `RAYON_NUM_THREADS=1`, preflight `--max-other-cpu 1.0`; record `contended: false`, wall 554.6 s. Wall time is not a reported value; all values are counted group operations, deterministic in the seed.

*Arms and workload.* DP Pollard rho on divisors (plain) and on ±-classes with look-ahead (folded), 16 branches, same seeds `0xC0FFEE + t`, t < 40, on the thread's 18 instances (genus 2, 3, 4; round seven's `N`). Requested 18 × 40 × 2 runs; completed 1440; every recovered `k` verified against the planted one.

*Earlier attempts* (kept beside the records, not used): 1 panicked on a constructor change; 2 was killed with its executor; 3 completed but its wrapper timed out (counts byte-identical to this run); 4 was refused by the isolation preflight.

#figure(image("swalk.svg", width: 100%), caption: [Recorded `S_walk` per row (mean over 40 seeds), plain and folded, against the ideal constants. Source: `../../run5/R9.txt`.])

*Recorded verdict lines* (printed by the binary): R1, median folded/plain walk operations over the 7 rows with N ≥ 10^5: 0.968, above the registered 0.80. R2, largest look-ahead-plus-escape share of the folded walk: 6.5%. R3: every run verified.

*Observations.* At N ≤ 6·10^4 every folded/plain ratio lies between 0.67 and 0.89. At the two largest rows the plain walk records 1.82 and 2.10 against the 1.2533 ideal, and the folded walk records 2.15 and 2.16. R2 counts escape steps only, not steps inside a fruitless cycle before its escape.

*Missing.* No cross-method comparison is made; neither arm emits its unpriced counters (hash evaluations, canonicalisations, table probes), so §5a pricing cannot be applied.
