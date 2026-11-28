# Requirement outcome

The frozen requirements remain byte-identical in `REQUIREMENTS.md`; this file
records their post-run disposition.

| ID | Requirement | Evidence | Status |
|---|---|---|---|
| R1 | Construct effective degree-4 `A` and certify `[A-4*infinity]=X` | 195,072 admitted pairs reduced back to their input classes | Pass [pcc:unverified] |
| R2 | Include nonsplit support honestly | 186,743 nonsplit candidates counted as nonsmooth | Pass [pcc:unverified] |
| R3 | Certify component and total pushforward | Zero consistency failures across 8,329 split candidates | Pass [pcc:unverified] |
| R4 | Separate image and Prym/kernel cosets | 65,024 trials in each of `K=0` and certified nonzero `K` | Pass [pcc:unverified] |
| R5 | Retain ordinary and exceptional controls | 65,024 ambient and 2,048 exceptional trials; false-kernel seed retained | Pass [pcc:unverified] |
| R6 | Measure merged projected labels and collisions | B-grid collision fields in certificate and validator | Pass [pcc:unverified] |
| R7 | Measure relation probability and useful rank | Full rank 64/64 at B=64 in both image arms | Pass [pcc:unverified] |
| R8 | Account matrix dimensions and memory | B=64 CSR: 5,068 bytes (`K=0`), 4,636 bytes (`K`) | Pass [pcc:unverified] |
| R9 | Attempt blind individual logs | 16/16 recovered in each image-compatible arm | Pass [pcc:unverified] |
| R10 | Use the actual prime order | Every n=9 equation/rank/recovery is modulo 127 | Pass [pcc:unverified] |
| R11 | Require a complete finite rho crossover before larger runs | `r^(8/5)` versus `r^(1/2)`; no finite crossover | Stop gate triggered [pcc:unverified] |
| R12 | Avoid claiming exhaustive ambient-Jacobian coverage | Scope is explicit in protocol, certificate and results | Pass |
| R13 | Independently validate emitted measurements | Separate replay reports 11 passing groups; independently signed PCC review is absent | Partial [pcc:unverified] |

The requested n=13,19,23 grids were conditional on R11.  R11 failed, so the
registered stop at n=9 completes this screen.
