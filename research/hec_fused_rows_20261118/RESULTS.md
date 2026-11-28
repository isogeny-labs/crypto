# Signed pencil rows on a freshly seeded panel

All four arms solved and verified all 44 targets. Direct accumulation and the generic reconstruction produced identical signed rows under audit. The matched arms agreed on anchors, cutoff, accepted rows, rejections, joins and scalar.

| Genus | Targets | Matched generic / fused wall | Previous deferred / fused wall | Original normalized / fused wall |
|---|---:|---:|---:|---:|
| 2 | 24 | 1.071–1.192× | 1.124–1.396× | 1.648–2.840× |
| 3 | 12 | 1.031–1.098× | 1.047–1.134× | 1.817–3.202× |
| 4 | 8 | 1.020–1.103× | 1.044–1.348× | 2.794–8.222× |

Direct accumulation reduced median full-solver wall time versus the matched reconstruction on all 44 inputs. Matched field reduction counts were equal in genera two and four. For genus three the generic/fused reduction-count ratio was 0.981–0.993, so the simpler reconstruction incurred extra field arithmetic while lowering elapsed time.

The 8.222× cell was genus four, p=31, case 0. The original solver consumed 351 anchors and 348,094 field reductions. The candidate consumed 87 anchors and 50,512 field reductions. With the relation stream and anchor count fixed, the contribution of direct accumulation on that cell was 1.103×. The complete peak reflects earlier algorithmic changes and target-dependent anchor counts. The full-panel tenfold goal remains open. Raw counter vectors and all 2,112 individual timings are in CONTROL.txt. summarize.rb regenerates COMPARISON.tsv.
