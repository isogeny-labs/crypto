# Prime31 factorized S4 — executed Rust development receipt (2026-11-15)

**[pcc:unverified]** Producer-only, single-run stage timings; no independent replay, complete ECDLP success or cold-rho advantage.

Source-bound job: [GitHub Actions run 43764153465](https://github.com/isogeny-labs/crypto/actions/runs/43764153465), job 132965380902. GitHub-hosted Ubuntu 24.04, Rust 1.100.0 release, single Rayon thread. Completed **successfully** on 2026-11-15, after earlier queued reports. GitHub job stdout: 40/40 reused library tests passed; 3/3 prime31 example tests passed, including old-vs-factorized exact monic eliminant parity B4/B8/B16, the special U=target_x branch, and correct fully signed planted witness at B31.

Input: ordinary cofactor-one `E: y²=x³+847514846*x+2087444724` over `p=2147483647`, group order `n=2147446831` prime. Frozen B31 Dickson factor base, target positive `P0+P1+P2+P3`, public control `[17]G`. All group arithmetic is quarantined outside the polynomial discovery oracle.

| B | K degree | K construction (ms) | Old quotient positive (ms) | Factorized positive (ms) | Same-run ratio |
| ---:| ---: | ---: | ---: | ---: | ---: |
| 4 | 16 | 0.008232 | 0.076954 | 0.026349 | 2.92× |
| 8 | 64 | 0.042663 | 3.179931 | 0.138345 | 22.99× |
| 16 | 256 | 0.200718 | 166.965025 | 0.917729 | 181.93× |
| 31 | 961 | 1.997539 | not executed | 10.122916 | no ratio |

For the B16 public-negative `[17]G`, new 0.905791ms versus old 166.658650ms in the **same** Rust run (183.99×, target-dependent stage only). B16 special U=target_x gives new 0.890138ms vs old 164.736424ms (185.07×). B31 factorized public negative 10.172138ms, special case 10.153701ms; full B31 reference omitted deliberately. The factorized code uses 496 short S4 factors of degree ≤4 at B31.

## Interpretation

This is a reproducible, actual **Rust component improvement**: multiplicativity of classical resultants reduces the implemented schoolbook target-norm work from O(B^6) to O(B^4), and the same-process stage measurements favor it strongly. It does **not** improve the exact B31 m4 natural-target support size, 618885/2147446831 (~0.0288196%). At a representative ~10ms factorized B31 negative call, the optimistic uniform-target expected time for **one** four-sum hit is ~34.7 seconds **before** extraction, any useful-rank test, base-log solve or cold setup. This extrapolation is a *conditional cost illustration*, not a measured complete process, and assumes uniform targets and constant per-query cost.

The direct follow-on is predeclared in [M4_FACTORIZED_NATURAL_4096_PROTOCOL_20261115.md](M4_FACTORIZED_NATURAL_4096_PROTOCOL_20261115.md). The newly checked-in Rust-only `--m4-natural-4096` gate will sample 4096 deterministic independent public encodings and charge all failures; until its source-specific CI completes, natural hit measurements remain unverified. If no useful new m4 relations appear, prioritize m8 direct algebraic extraction and one-relation approaches, not additional known-sum enumeration.

## Literature and searched-lead gate

Previously searched and reassessed before this update: #4228 certified quotient/subresultant, #4275 dense F4 vs enumeration, #4077 full m8 cost, #4311/#4338 selector ideas, and x/y coordinate pilot. Outside prior art: [Petit–Kosters–Messeng, PKC 2016](https://doi.org/10.1007/978-3-662-49387-8_1) and [Amadori–Pintore–Sala, FFA 51, 2018](https://doi.org/10.1016/j.ffa.2018.01.009). These show that a prime-field Semaev IC framework and single-relation variants already exist. Novelty is limited to this implementation's concrete factorization/measurement, not a new ECDLP complexity theorem. Research-KB/Conductor/EKS unavailable in this connector; review scope limited to GitHub primary reports and published records.
