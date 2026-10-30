# A constant selector is a null result against fixed min-weight

Native fitting on n5/n9 selected min-weight for every input. Four small independent RREF controls passed and both n15 policy replays completed. The heldout n15 selected replay exactly matches archived min-weight counts, rank 20,490 and 20,670 rows. n17 is an archival outcome evaluation after input-only selection, not a new reducer run. The historical summary was already visible: this is a retrospective shape split, not a blind prospective test.

| Shape | Split / replay | Fixed first proxy | Fixed min-weight proxy | Selected + feature/inference proxy | Extra versus fixed min-weight |
|---|---|---:|---:|---:|---:|
| n5 m4 ell2 | train / fresh | 101,329 | 78,847 | 80,607 | 1,760 |
| n9 m4 ell3 | train / fresh | 6,062,816 | 3,522,348 | 3,525,124 | 2,776 |
| n15 m4 ell5 | heldout / fresh | 402,591,019 | 167,802,367 | 167,807,853 | 5,486 |
| n17 m4 ell6 | heldout / archived only | 1,304,928,293 | 530,723,096 | 530,729,736 | 6,640 |

The policy proxy is sparse term visits plus journal bytes divided by eight. Equal weights are a declared convention, not machine-instruction equivalence. The [native summary](../summary.json) preserves both components separately. Prefix overhead charges terms visited, input words read, two scalar feature updates per prefix row and one model output; the constant model needs zero threshold comparisons. Neither model training nor pre-pass features use heldout policy outcomes.

Training validates 91,734 historical trace records (8,984,687 uncompressed bytes). Trace weights sum exactly to each training result's sparse term visits. The 28 candidate/sample evaluations and 24 threshold comparisons return a constant model because both training labels favor min-weight. Label acquisition costs 9,765,340 proxy units; the sum of declared counted training components is 11,076,480. Training charged once to n15 plus selection/reduction is 178,884,333 units; if historical training is treated as sunk, selection/reduction is 167,807,853. These figures do not include a complete instruction price for verification, trace writing, parsing, hashing, decompression, allocations or bookkeeping. Fresh six-run audit costs are retained separately in process resource receipts.

Two independent train shapes do not permit a credible calibrated uncertainty estimate. The whole-pass choice is equal to fixed min-weight before overhead and strictly worse after overhead. The old min-weight-versus-first count difference is not an AI gain. Large fresh n15 replay reproducibility does not independently certify its row space; n17 remains archival and uncertified. The four n5/n9 fresh controls do independently certify both policies' exact row spaces.

All work is one fixed D=4 Boolean Macaulay pass on the earlier synthetic K1 [17]G fixtures, with one CPU and hard 512 MiB address space on probe-wrapped fitting, inference and reducer subprocesses. Recorded precompression disk use was 198,237,682 bytes, below the 1 GiB target. Aggregate disk and 1,200-second wall budgets were operational targets, not globally enforced by the runner. Resource timings are shared-host L0 diagnostics. No fastest-tool comparison was run because no timing/fastest claim is made. No general learned F4, closure, solving degree or end-to-end IC claim follows. **Original item 9 remains partial.**

Sources: [frozen protocol](../protocol.json), [model](../runs/frozen-v1/model.json), [attempts](../runs/frozen-v1/attempts.tsv), [n15 comparison](../runs/frozen-v1/heldout-n15-comparison.json), [source hashes](../runs/frozen-v1/sources.sha256), [binary hashes](../runs/frozen-v1/binaries.sha256), [previous immutable study](../../bounded_memory/README.md). The next test needs independently sampled shapes with different winners, prospective heldout outcomes and independently checked large row spaces.
