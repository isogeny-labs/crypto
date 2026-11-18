# Reusing intermediate signed order sums

The extra decompositions produced 43 verified equations across 32 eligible inputs. They brought the stopping point forward on three inputs by five anchor visits in total. Median full-solver time stayed within 1% of the preceding solver in both runs for all three genera. The registered tenfold criterion failed. Retain the preceding solver as the baseline.

| Genus | Extra equations / sums tried | Prior → candidate anchor visits | Prior / candidate time, runs 1 and 2 | Original / candidate time, runs 1 and 2 |
|---|---:|---:|---:|---:|
| 2 | 36 / 60 | 984 → 982 | 1.001×, 1.004× | 4.38×, 4.35× |
| 3 | 7 / 68 | 853 → 850 | 1.003×, 1.007× | 4.07×, 4.09× |
| 4 (control) | — | 888 → 888 | 1.004×, 1.005× | 6.52×, 6.57× |

Each time ratio is the median of the 16 input-level paired medians for that genus. Each input has 24 rotating three-arm repetitions in each fresh process. Counts describe one pass across the panel. Both processes produced identical harvest and stopping records. The earlier stops were genus 2 at p=1021, target 0 (51 → 49), and genus 3 at p=73, target 2 (40 → 38) and p=127, target 1 (36 → 35). Equation counts include every admitted row, including rows that may be linearly dependent.

All 6,912 timed calls returned the planted scalar and passed group verification. Only genus 4 at p=59, target 2 exceeded 10× against the original solver in both processes. That input used the unchanged control path.

Git holds the protocol, source, binary digest, raw wall records and derived table. Run `shasum -a 256 -c OUTPUT.sha256` in this directory to verify the result files and `ruby summarize.rb | cmp - RATIOS.tsv` to reproduce the table. BINARY.sha256 identifies the locally tested release executable.
