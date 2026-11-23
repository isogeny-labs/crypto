# Standard-input multi-orbit cold comparison

The once-only frozen run at `714fa3e242b2f1571c9101c029d81eb5c8d0599e` completed with exit 0. All 7,200 calls verified their recovered scalar against the original public source and target. All 90 batch-spread gates and all 18 native A/A gates passed. The registered numerical tenfold gate failed: the equal-information Expanded arm was the fastest control in every workload, and Folded improved on it by only 1.196560–1.742197 times.

| Genus | Native / Folded, six separate workloads | Expanded / Folded, six separate workloads |
|---|---|---|
| 2 | 2.210917–2.938682 | 1.489349–1.742197 |
| 3 | 3.418988–7.363810 | 1.196560–1.333478 |
| 4 | 7.985306–10.619135 | 1.326390–1.413756 |

These ranges describe six individual source/target workloads per genus, not pooled throughput. Only two genus-4 workloads exceeded tenfold relative to default native. PlainPrefix was slower and is retained as a control, not selected as the headline baseline.

The production API receives only curve, public source, public target, subgroup order, seed and mode. It receives no cofactor, source-point index, expected scalar or known logarithm table. The preceding correctness gate qualified all 18 arbitrary-source cases and checked agreement with an expanded reference. Folded retains only 7/5/7 orbit counts and rows of length at most 8/6/8 for genera 2/3/4.

Each timed call starts with fresh algorithm state and includes factor-base construction, action/certificate construction, orbit maps, relation collection, linear algebra, final source verification and deallocation. Common public curve/order/source compatibility qualification and target construction are outside every arm equally. This is a provided-valid-public-instance comparison, not a benchmark starting from an unqualified curve or including parameter discovery. The process is warm; algorithm state is cold. No cross-call or cross-target setup cache is used.

The frozen design uses five batches of sixteen calls for each of eighteen workloads and five rotating arms. Each reported value is the median of five batch means. Shared Apple M4 Pro measurements have L0 isolation. Local noise qualification does not establish independent admission, operation-count qualification, strongest external performance or a general tenfold mathematical breakthrough. Classical automorphism information explains part of the comparison with native; row representation alone is insufficient for the requested tenfold gain.

Raw calls: CONTROL.tsv. Workload identities: WORKLOADS.tsv. Per-arm medians and spreads: SUMMARY.tsv. All per-workload ratios follow:

```text
workload=0 expanded_over_folded=1.640364 plain_over_folded=11.472449 native_over_folded=2.624441 best_over_folded=1.640364 native_aa=1.063787
workload=1 expanded_over_folded=1.558796 plain_over_folded=10.145149 native_over_folded=2.210917 best_over_folded=1.558796 native_aa=1.066155
workload=2 expanded_over_folded=1.496441 plain_over_folded=9.650482 native_over_folded=2.513503 best_over_folded=1.496441 native_aa=1.065091
workload=3 expanded_over_folded=1.742197 plain_over_folded=11.876517 native_over_folded=2.938682 best_over_folded=1.742197 native_aa=1.055055
workload=4 expanded_over_folded=1.550139 plain_over_folded=10.965355 native_over_folded=2.398302 best_over_folded=1.550139 native_aa=1.043838
workload=5 expanded_over_folded=1.489349 plain_over_folded=6.269565 native_over_folded=2.395954 best_over_folded=1.489349 native_aa=1.032561
workload=6 expanded_over_folded=1.240183 plain_over_folded=12.381067 native_over_folded=6.280054 best_over_folded=1.240183 native_aa=1.021669
workload=7 expanded_over_folded=1.321835 plain_over_folded=9.519156 native_over_folded=4.766600 best_over_folded=1.321835 native_aa=1.000124
workload=8 expanded_over_folded=1.333478 plain_over_folded=12.323233 native_over_folded=5.570961 best_over_folded=1.333478 native_aa=1.009780
workload=9 expanded_over_folded=1.196560 plain_over_folded=6.773597 native_over_folded=3.418988 best_over_folded=1.196560 native_aa=0.990236
workload=10 expanded_over_folded=1.197769 plain_over_folded=9.863524 native_over_folded=4.423286 best_over_folded=1.197769 native_aa=1.009878
workload=11 expanded_over_folded=1.255355 plain_over_folded=11.754263 native_over_folded=7.363810 best_over_folded=1.255355 native_aa=1.004421
workload=12 expanded_over_folded=1.354054 plain_over_folded=35.680598 native_over_folded=7.985306 best_over_folded=1.354054 native_aa=1.011459
workload=13 expanded_over_folded=1.374428 plain_over_folded=41.383660 native_over_folded=9.520393 best_over_folded=1.374428 native_aa=1.016265
workload=14 expanded_over_folded=1.356295 plain_over_folded=41.378989 native_over_folded=10.474132 best_over_folded=1.356295 native_aa=1.001991
workload=15 expanded_over_folded=1.386654 plain_over_folded=40.866067 native_over_folded=8.826074 best_over_folded=1.386654 native_aa=1.019693
workload=16 expanded_over_folded=1.413756 plain_over_folded=40.805486 native_over_folded=10.619135 best_over_folded=1.413756 native_aa=1.011246
workload=17 expanded_over_folded=1.326390 plain_over_folded=34.307896 native_over_folded=8.237670 best_over_folded=1.326390 native_aa=1.020380
calls=7200 correct=true spread_pass=true aa_pass=true numerical_tenfold=false local_qualified=true
```

Git custody replaces unavailable S3 under the authorized amendment. OUTPUT.sha256 preserves raw hashes; no upload is claimed. Protocol, source and binary hashes were frozen and pushed before execution. No unchanged rerun was made. The next mathematical mechanism must improve relation generation or elimination beyond this representation change; the broader objective remains open.
