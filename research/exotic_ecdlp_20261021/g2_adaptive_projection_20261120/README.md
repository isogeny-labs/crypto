# Six or seven witnessed columns repair two N31 S3 projections

The previously failed 15,856-column information set recovered full rank after adding **six columns** for `fresh_1` and **seven columns** for `fresh_15`. Both frozen 16,399-row matrices began at projected rank 15,835; their full-width rank is 15,841. Adding all 11 recorded outside pivots also retained full rank. The exact step-by-step results are in [result.json](result.json).

| Archived parameter | Base rank | First full-rank width | Added columns | Final rank | Row nonzeros at first full rank |
| --- | ---: | ---: | ---: | ---: | ---: |
| `fresh_1`, q=1,558,806,792 | 15,835 | 15,862 | 6 | 15,841 | 959,593 |
| `fresh_15`, q=2,021,112,548 | 15,835 | 15,863 | 7 | 15,841 | 951,902 |

The [protocol](PROTOCOL.md) fixed the training union, witness order, full-width row streams, ranker hash, and all step caps before execution. [run.py](run.py) checks the source hashes, reconstructs every projected stream, and uses the archived PLUQ ranker. A second [validator](validate.py) compiled [rank_m4ri.cpp](rank_m4ri.cpp) and recomputed the four threshold ranks with M4RI echelonization. Its [receipt](validation.json) matches the projected stream hashes and ranks immediately before and at full-rank recovery. Both methods returned rank 15,841 at the listed widths.

These witnesses were selected from the **full-width echelon profiles** of the two failed cases. Finding them therefore already incurred a full-width elimination. This experiment establishes a compact rank-preserving projection for those two parameters; it does not establish a cheaper online solver. The next implementation test needs an incremental repair rule that discovers missing columns without first eliminating the full matrix, then measures rank checking, elimination, solution extraction, and point verification on the same target-dependent interval. The local elimination times in the receipt are exploratory stage diagnostics.
