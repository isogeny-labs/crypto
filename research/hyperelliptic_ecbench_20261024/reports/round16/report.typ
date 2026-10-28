#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: hyperelliptic IC round sixteen, the thread's instances through ecbench

*Card* `CARD-HEIC-ROUND16-1` (sha256 `f7559530…306203`), commit `575fea48`; binary `ecbench` sha256 `a267fe64…823fef`, built from `575fea48`; spec `spec-r16.json` sha256 `60fb0a9e…35afa7f`. Receipt `../../round16/run1/receipt.json`: status completed, all 28 commands exit 0.

*Host and level.* 4 vCPU Xeon @ 2.30 GHz; `--cpus none`, level L0 on every run. Only counted group operations are read; they are deterministic in the seed.

*Workload.* The thread's 18 instances (genus 2, 3, 4; prime subgroup orders `r` from 1321 to 16 790 591), 8 planted targets each, arms `rho.negation` (reference) and `rho.plain` (baseline), 2 rounds after 1 warm-up: 864 executions, all verified. Audit with `--replay-all`: `audit OK: 864 records, 864 verified, 576 replays reproduced, 0 problems`; receipt sha256 `92d88c3c…f5b8e`.

#figure(image("r16.svg", width: 100%), caption: [Left: per-curve `rho.negation / rho.plain` in `S` (w = 0) with 95% intervals, marker by genus; the vertical line marks `r = 10^5`. Right: mean `S` per curve and arm (circles genus 2, squares genus 3, triangles genus 4) against the floors. Source: `../../round16/run1/cmd3.stdout`, `cmd4.stdout`.])

*Registered tests.*
- E1, every run verified and every replay identical: *holds*.
- E2, median per-curve ratio over the 7 instances with `r ≥ 10^5`: 0.646 (bound 0.80): *holds at w = 0*.
- E3, median `S(rho.negation)` on those 7: 1.273, beside round fifteen's walk-only median 1.008 (descriptive; different accounting).

*§5a band.* `rho.negation` leaves `canonicalisations_uncharged = 166 244` unpriced; `rho.plain` none. Pooled ratio 0.840 (w = 0), 0.895 (w = 0.1), 1.394 (w = 1): unresolved inside the band, so the arms are not ranked.

*Decision.* As registered, ecbench's `rho.negation` becomes the thread's rho reference (accounting). Later IC/rho comparisons run both arms in one ecbench session, priced across the band.

*Missing.* No wall-time figure (L0). No validator report, so no benchmarks row. The workload slugs are unregistered and are cited by genus, `p` and `r`.
