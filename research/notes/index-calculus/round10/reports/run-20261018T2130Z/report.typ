#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: hyperelliptic IC round ten, rho walk-hash mixing

*Card* `CARD-HEIC-ROUND10-1` (sha256 `9f36d950…183dba`), commit `0768e22b`; binary `mix_bin` sha256 `7e569a76…e6b8`, built from `0768e22b` (`examples/hyperelliptic_rho_hash_mixing.rs`). Receipt `../../run1/receipt.json`: status completed, all nine commands exit 0.

*Host and isolation.* Intel Xeon @ 2.80 GHz, 4 logical CPUs, kernel 6.18.44-fc-v139, x86-64. `tools/isolated_bench.py`, reserved CPU 3, preflight `--max-other-cpu 1.0`; record `contended: false`, wall 1014.5 s. Wall time is not a reported value; all values are counted group operations, deterministic in the seed.

*Arms and workload.* DP Pollard rho, 16 branches, four arms: plain and ±-folded walks, each with the old `divisor_hash` and with that hash passed through the SplitMix64 finaliser. Same seeds `0xC0FFEE + t`, t < 40, on the thread's 18 instances (round seven's `N`). Requested 18 × 40 × 4 runs; every recovered `k` verified against the planted one.

#figure(image("swalk.svg", width: 100%), caption: [Recorded `S_walk` per row (mean over 40 seeds) for the four arms, against the ideal constants. Source: `../../run1/R10.txt`.])

*Recorded verdict lines* (printed by the binary):
- H1, mixed plain `S_walk` over the 7 rows with N ≥ 10^5: median 1.406 (bound 1.45), two largest rows 2.240 and 1.696 (bound 1.55): *falsified*.
- H2, median mixed fold/plain walk operations, N ≥ 10^5: 0.960 (bound 0.80): *falsified*.
- H3, control at N ≤ 6·10^4: |median mixed − median old| plain `S_walk` = 0.023 (bound 0.15): holds.
- H4, every run verified: holds. Largest fruitless share, fold+mix: 6.0%.

*Observations.* Mixing the hash leaves the small rows where they were (H3) and does not bring the large rows to the ideal: on the largest instance (genus 4, p = 61, N = 16 790 591) the mixed plain walk records 2.240 and the mixed folded walk 3.112. The walk hash is therefore not the cause of round nine's large-`N` excess. A per-seed diagnostic (`../../diagnostic/seed_profile.txt`, not registered) shows every seed walking long with no restarts. Round eleven tests the shared walk logic with an integer-model control.

*Missing.* No cross-method comparison is made; neither arm emits its unpriced counters (hash evaluations, canonicalisations, table probes), so §5a pricing cannot be applied.
