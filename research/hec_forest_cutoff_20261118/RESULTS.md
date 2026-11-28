# Matching the small factor base to the pencil sieve

All five arms completed their scheduled calls. Arms 0–3 verified the scalar on all 44 targets. The minimal prefix solved 17/44 within 512 anchors. Every capped case remains in CONTROL.txt and COMPARISON.tsv. Counts below use complete solver calls. Ratios are ranges of per-target median wall times on jointly solved inputs.

| Genus | Prefix | Verified | Faster than deferred | Original / candidate | Deferred / candidate | Finite rows rejected at forest gate |
|---|---|---:|---:|---:|---:|---:|
| 2 | half | 24/24 | 4 | 1.308–2.085× | 0.790–1.065× | 2,803 |
| 2 | double | 24/24 | 17 | 1.363–2.423× | 0.957–1.153× | 0 |
| 2 | minimal | 13/24 | 0 | 0.381–1.395× | 0.238–0.817× | 26,705 |
| 3 | half | 12/12 | 0 | 1.642–2.179× | 0.724–0.952× | 1,210 |
| 3 | double | 12/12 | 0 | 1.689–2.221× | 0.648–0.981× | 0 |
| 3 | minimal | 4/12 | 0 | 0.458–0.903× | 0.242–0.506× | 8,981 |
| 4 | half | 8/8 | 1 | 1.620–4.023× | 0.454–1.017× | 436 |
| 4 | double | 8/8 | 8 | 2.641–5.232× | 1.023–1.213× | 0 |
| 4 | minimal | 0/8 | 0 | — | — | 1,748 |

The unchanged deferred prefix rejected 703, 0 and 45 finite rows in genera two, three and four respectively. Matching the prefix to the scanned range gives a structural admission guarantee: all known hits are small. The remaining quotient has degree at most one in the genus-two and genus-four schedules and at most two in genus three. Thus finite rows from these schedules contain at most one or two distinct large places respectively. This statement concerns finite pencil rows; initial public rows can have other support patterns.

Genus three already covers the entire scan with the original prefix. Expanding the dense system lowered median performance on all twelve inputs. Genus four covered the full scan with the doubled prefix and improved median time across all eight. The strongest measured gain over the original was 5.232× on one input. The full-panel tenfold gate remains open.

RUN.txt identifies the remotely verified experiment commit. The counter vectors and ten rotating observations per arm per fixture are in CONTROL.txt. summarize.rb reproduces COMPARISON.tsv.
