# Requested versus retained dataset coverage

`build_coverage.py` reads the immutable five-campaign inventory and every
retained Git custody checkpoint. It verifies campaign denominators, plan
hashes, native export flags, and producer S3 readback receipts before emitting
`coverage.json`. It does not generate cells or contact S3.

The default CLI axes imply 165,150,720 planned method cells: 24 field sizes,
14 arities, three families, ten methods, 16 curve seeds, eight generators,
64 target keys per generator, and two twist settings. That is planning
arithmetic. The retained local campaigns contain 148 receipts, all at `n=23`
and `m=2` or `m=4`. Eight of the ten methods have at least one retained cell;
`enumerate` and `pkm` have none. The `n=47..61` band and the specific prime
modulus `2^61-1` have no retained cell. The report includes a 3-by-10
family-method receipt grid and lists all unobserved size and arity values.

Binary and Koblitz Git checkpoints explicitly show both twist settings.
The 64 prime BSGS checkpoints retain the plan and restricted archive but omit
`cell_spec`; this report does not infer an observed prime twist from a planned
axis. A follow-up can promote prime twist coverage only after checking the
restricted native manifests and retaining a reproducible disaggregation
receipt. Absent method-family intersections are coverage gaps, not claims of
mathematical impossibility.

Recompute without solver execution:

```sh
python3 research/ml_dataset_coverage_20261031/build_coverage.py
```
