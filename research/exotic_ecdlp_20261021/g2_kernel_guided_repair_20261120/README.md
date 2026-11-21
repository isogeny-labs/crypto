# Kernel-guided repair finds six columns per N31 target

A target-conditioned kernel computation repaired both frozen N31 S₃ projection failures **without a full-width pivot profile**. Starting from the same 15,856-column information set and rank 15,835, it selected six omitted columns for each target. Independently projecting those columns and running PLUQ gave the archived full-width rank **15,841** in both cases.

| Archived parameter | Selected original columns | Repaired width | Repaired rank | Producer peak RSS |
| --- | --- | ---: | ---: | ---: |
| `fresh_1`, q=1,558,806,792 | 13374, 13512, 13642, 13652, 13656, 13659 | 15,862 | 15,841 | 119,242,752 bytes |
| `fresh_15`, q=2,021,112,548 | 13374, 13381, 13383, 13385, 13387, 13388 | 15,862 | 15,841 | 123,895,808 bytes |

The [producer](repair.cpp) computes a basis for the left kernel of the training projection, multiplies that basis against the omitted columns, and selects pivot columns of the resulting six-rank matrix. This uses the full row stream to read omitted columns, but eliminates only the 15,856-column projection and the small quotient. The [runner](run.py) checks all frozen input/source hashes, then independently reconstructs and ranks each repaired matrix. The exact selected columns, stream hashes, exclusive producer phases, independent ranks, and memory peaks are in [result.json](result.json). The [protocol](PROTOCOL.md) and [amendment](AMENDMENT.md) record the input law, caps, failed assertion, and host memory-limit correction.

Local exploratory producer kernel phases took 0.771 and 0.772 seconds. Projection took 0.122 and 0.120 seconds; independent PLUQ rank checks took 0.603 and 0.579 seconds. These are stage diagnostics on a shared host, not a controlled solver comparison. The kernel computation still costs a large elimination and does not reconstruct polynomial solutions or check curve points. The next engineering gate is to carry the selected information set into a complete target PDP solve, with target-dependent preparation, elimination, back-substitution, and point verification charged together against the direct path on identical residuals.
