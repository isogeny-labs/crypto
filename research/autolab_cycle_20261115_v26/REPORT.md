# Shared-block reduction: exact, but limited at full width

Hypothesis: shared constant matrix columns would remove at least 35 of 45 equation rows. A symbolic preflight first refuted the all-cell version: the homogeneous cell has M0=0, so its constant image has rank zero. The changed nonzero-cell prediction was then frozen and tested.

The first experiment proves an exact reduction with full affine-fiber lifting. Across both exposed families, the constant image ranks for the eight high-three-bit cells are 0,1,3,3,4,4,4,4. Thus the large row-block prediction is refuted. The common domain kernel does remove 27 directions from pre-Gold73 and 23 from Gold69, leaving 46 active columns in every cell. Residual equation counts are 45,44,42,42,41,41,41,41.

Every one of 16,384 independently enumerated tiny inner fibers matched the lifted solution set. All 1,024 actual exposed affine fibers matched complete independent solves, using verified lifted particular vectors, independent basis vectors and equal dimensions. Worker time was 1.519123902 seconds, peak process RSS 23,564 KiB. Raw SHA256: 211f1fd92695d601c4d3104d9e8c2ee839156ff593eb88a669bb072a57116658, 68,449 bytes.

Changed hypothesis: before timing the 46-column representation, test whether a single-word active core survives all 35 direction coordinates. The incidence matrices are F2-linear in the direction u. The second experiment stacks all 35 unit matrices, checks its shared-kernel identities, and replays 64 previously frozen direct-field systems per family, including all units and mixed directions.

| Direction coordinates | pre-Gold73 active columns | Gold69 active columns |
| ---: | ---: | ---: |
| 6 | 46 | 46 |
| 9 | 50 | 50 |
| 18 | 59 | 59 |
| 35 | 69 | 68 |

This refutes the full-width <=64-column prediction. The full shared kernels have dimensions 4 and 1. Their exact null identities hold for every one of the 2^35 directions by linearity; this does not enumerate or solve that direction set. The 128 complete probe fibers and all small-oracle fibers matched. Worker time was 0.502891371 seconds, peak process RSS 25,480 KiB. Raw SHA256: 926aded622c60dbe84ead8c58da7e278507d25662844757c61169cd3625c0187, 89,995 bytes.

Both variants were frozen before execution and run using lab.runner resume/run-ready, with 90 seconds, 4 GiB and one core each. Two experiments ran; no third execution or unchanged retry occurred. Timing includes controls and is not a production comparison. RSS is a process peak, not an aggregate memory or release comparison. Prior frozen source, inputs and evidence remain intact.

The independent standard-library REPLAY_RECORDED.py verifies every shared-kernel basis and dimension, every complement and quotient image rank, all 1,024 local fiber ranks and consistency values, and the 128 full-width probe results. It passed. Sole-model source preparation remains separate from independent algorithmic verification and external source admission.

What changed: retain exact block reduction as a component, retire the large fixed-block and uniformly small full-width-core assumptions. NEXT.md gives a changed structured-convolution experiment and a separate generic-field implementation stream. No qualified speed ratio, full original 339-variable solve, norm/source/ghost/chart closure, new odd-field performance or novelty clearance follows. The requested 8x goal and unchanged F4/F5/F6 release gate remain open.

To replay: restore the two exact raw JSON files through repository artifact references if relocated, then run python research/autolab_cycle_20261115_v26/REPLAY_RECORDED.py. Fresh worker execution requires new output paths and a newly frozen plan. Internal management provenance remains separate from the public mathematical checkpoint.
