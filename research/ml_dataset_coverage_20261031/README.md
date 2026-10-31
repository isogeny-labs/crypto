# Requested versus retained dataset coverage

`build_coverage_v3.py` binds the prime61 audit and four custody checkpoints to
`inventory-v3.json`, preserving the immutable v2 ledger. The resulting
`coverage-v3.json` has 152 retained receipts and observed sizes `n=23` and
`n=61`. Prime enumeration now has two retained method artifacts; `pkm` remains
unobserved. The specific modulus `2^61-1` has four retained cells at `m=2`,
both twists, one generator and one key per twist. The other requested sizes
and arities remain coverage gaps. Recompute with
`python3 research/ml_dataset_inventory_20261031/build_inventory_v3.py` followed
by `python3 research/ml_dataset_coverage_20261031/build_coverage_v3.py`.

`build_coverage.py` reads the immutable five-campaign inventory, every
retained Git custody checkpoint, and `prime-twist-archive-audit.json`. It
verifies campaign denominators, plan hashes, native export flags, archive
audit bindings, and producer S3 readback receipts before emitting
`coverage-v2.json`. The original `coverage.json` remains immutable. Neither
script generates cells or contacts S3.

The default CLI axes imply 165,150,720 planned method cells: 24 field sizes,
14 arities, three families, ten methods, 16 curve seeds, eight generators,
64 target keys per generator, and two twist settings. That is planning
arithmetic. The retained local campaigns contain 148 receipts, all at `n=23`
and `m=2` or `m=4`. Eight of the ten methods have at least one retained cell;
`enumerate` and `pkm` have none. The `n=47..61` band and the specific prime
modulus `2^61-1` have no retained cell. The report includes a 3-by-10
family-method receipt grid and lists all unobserved size and arity values.

Binary and Koblitz Git checkpoints explicitly show both twist settings.
The 64 prime BSGS checkpoints omit `cell_spec`, so `audit_prime_twists.py`
hashes each retained restricted archive against its custody checkpoint and
reads the exact native manifest. It found 32 untwisted and 32 twisted cells;
`coverage-v2.json` binds that audit to all 64 checkpoint archive hashes. The
audit can be replayed from the content-addressed S3 objects after restoring
the archives to a local directory. Absent method-family intersections are
coverage gaps, not claims of mathematical impossibility.
`prime-s3-readback-sample.json` records separate fresh GET/hash checks for
shards 0 and 63; it is a two-object sample, not an independent S3 audit of all
64 archives.

Recompute without solver execution:

```sh
python3 research/ml_dataset_coverage_20261031/build_coverage.py
python3 research/ml_dataset_coverage_20261031/audit_prime_twists.py \
  --archives /var/tmp/ml-dataset-local-campaign-20261031/archives
```

# Version 4 update

`build_coverage_v4.py` binds the immutable version 3 snapshot to the checked
version 4 inventory and sixteen Koblitz47 checkpoints. `coverage-v4.json`
records **168 retained receipts** at field sizes 23, 47, and 61. Koblitz
enumeration now has eight receipts and Koblitz F6 has ten; `pkm` remains
unobserved. Both twists are represented in the new Koblitz campaign. The
size band 48 through 60 and requested arities other than 2 and 4 still have
no retained receipts. Counts are campaign receipts, not unique key identities.

```sh
python3 research/ml_dataset_inventory_20261031/build_inventory_v4.py
python3 research/ml_dataset_coverage_20261031/build_coverage_v4.py
```

# Version 5 update

`build_coverage_v5.py` binds the ordinary-binary n=47 pilot's independent audit,
plan, checkpoints, cost projection, and S3 readback sample to the checked
version 5 inventory. `coverage-v5.json` records **172 retained receipts** at
field sizes 23, 47, and 61. The binary family gains two F6 pair-domain and
two bounded enumeration receipts across both twists; one enumeration cell
timed out and remains censored. The requested n=48..60 sizes, arities other
than 2 and 4, and PKM method still have no retained receipts. Counts remain
campaign receipts rather than unique curve or key identities.

```sh
python3 research/ml_dataset_inventory_20261031/build_inventory_v5.py
python3 research/ml_dataset_coverage_20261031/build_coverage_v5.py
```
