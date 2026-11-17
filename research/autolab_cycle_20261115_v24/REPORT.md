# Degree-stratified common-dual study

Hypothesis: a matrix pencil whose global common annihilator is zero can nevertheless admit useful common duals on bounded degree strata. Test the previously prepared variant, preserving every feasible direction, rather than repeat the failed global constant-dual test.

A frozen one-core, 4-GiB, 90-second component experiment completed on the exposed GF(2^83) coefficient-span pencils. The worker checked all 4,096 tiny two-parameter pencils and 8,192 degree strata against independent exhaustive inner solutions. It used 0.224349807 seconds and 17,048 KiB peak process RSS. RSS is a process measurement, not a cgroup aggregate or release-memory comparison.

Both pre-Gold73 and Gold69 families gave the following candidate/actually feasible counts for degrees 0 through 8:

| Degree | Candidates | Feasible | Exact filter |
| --- | ---: | ---: | --- |
| 0 | 0 | 0 | yes |
| 1 | 0 | 0 | yes |
| 2 | 0 | 0 | yes |
| 3 | 1 | 1 | yes |
| 4 | 2 | 2 | yes |
| 5 | 8 | 4 | no |
| 6 | 32 | 24 | no |
| 7 | 128 | 94 | no |
| 8 | 256 | 246 | no |

The smallest compatible nonzero direction is binary 1010, or T^3+T. The conjecture that compatibility is polynomial divisibility by this direction is false: 20 is compatible, its T-multiple 40 is incompatible, and 35 is compatible. The complete low-six-bit feasible set including zero is {0,10,20,30,35,41,55,61}, exactly span{10,20,35}. Its parity masks are 20,33,42. Current shared duals supply only two independent guards, leaving 16 candidates instead of eight. The third parity is observed from the complete finite truth table, not yet derived as a parametric certificate.

REPLAY_RECORDED.py uses independent standard-library elimination to reconstruct both complete 512-direction systems and verify every stratum dual, candidate set and exactness flag. It passed. An initial replay draft used the wrong syndrome bit ordering and failed; its exact draft is retained. Correcting only that verifier ordering resolved the failure. Frozen worker and raw evidence were unchanged.

Changed next hypothesis: iterative common-dual refinement may recover the missing parity after restricting to the first two guards. A fresh implementation and all-characteristic soundness proof are prepared in ../dual_refinement_20261115_v24. It was syntax-checked but not executed or registered because the scientific cutoff left insufficient review time. The next cycle must freeze its sources, input and budgets, test exhaustive tiny affine domains, then measure whether the predicted 6-to-4-to-3 path holds. This is a falsifiable prediction, not a result.

The sole-model preparation review is not independent source admission. Independent algorithmic oracles and recorded replay passed. No primary-source novelty clearance is claimed. Continue the odd-characteristic and sparse block-triangular streams separately; this packed binary implementation does not establish them.

Scope: fixed exposed coefficient-span pencils, not all 35 direction bits, outer h/charts, supported norms, original 339-variable root sets, or cross-field hard comparisons. Neither 8x requested gain nor the unchanged 2x release floor has been demonstrated. Genuine matched all-roots F4/F5/F6 baselines, all requested fields, independent original root-set verification, repeated end-to-end timings, qualified isolation and memory remain required.

Raw evidence SHA256: ec4a93f784e91160aa8e68438cc67e5b83e90b785d558258a54dc2574039b46d (18,360 bytes). Replay also requires v23 raw SHA256 8d52c93409bc65b5030aea2ad5985984723ffd75da207b5f639591ed2980e9d7. If repository artifact maintenance moves raw JSON, restore the exact bytes from its artifact reference before replay. Prior attempts remain intact; internal experiment-management provenance remains separate from this public scientific checkpoint.
