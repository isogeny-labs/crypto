# Pair-table performance receipt

Measured on the same Apple Silicon host, Python 3.13, campaign directory on
the external SSD, ECC2K-130 `docs/ic/params/ecc2k130-fixed.json`, stage
`pairs`. Rate includes factor-base construction and process start-up.

| Candidates | Baseline (`isogeny/main`) | Batched + field + batch inversion | Plus index-off build |
| ---: | ---: | ---: | ---: |
| 200,000 | 54.63 s, 3,661 candidates/s | 28.17 s, 7,100 candidates/s | same regime |
| 500,000 | 142.74 s, 3,503 candidates/s | 75.60 s, 6,614 candidates/s | 61.37 s, 8,147 candidates/s |

With four process workers on the same 500,000-candidate build: **19.02 s, 26,295 candidates/s** (3.27× over the one-worker 62.15 s run measured at the same time). With eight workers: **15.76 s, 31,720 candidates/s**. A paired 300,000-candidate panel gave 23.5k candidates/s at four workers and 32.3k at eight (1.37×). The parent keeps the serial hash chain and merges each worker's operation counts.

The 500,000-candidate campaign produced 499,930 pairs. With the index kept on,
the database was 160 MiB; with the bulk index-off path it was 76 MiB during
the incomplete build, and `CREATE INDEX pair_lookup` took 1.08 s and grew it
to 142 MiB. The 500k wall figures were taken sequentially and the host load
varies; a paired 50,000-candidate CPU-time test (four runs each, alternating)
gave a median of 8.04 s with the index on and 7.69 s with it off (1.05×).

The earlier in-progress run reported ~2,200 candidates/s at 1.8M candidates;
the larger table is dominated by index growth, which is why the 500k figure is
the more representative regression baseline.

Reproduce:

```sh
cd ecc2k130/codegen
./bench_pair_table.py --candidates 500000 --json
```

To compare against the pre-change engine, extract the codegen tree from the
base revision into a temporary directory and run the same command there:

```sh
git archive isogeny/main ecc2k130/codegen | tar -x -C "$TMP" --strip-components=2
(cd "$TMP" && python3 indexcalc_fixed.py \
  --params /path/to/docs/ic/params/ecc2k130-fixed.json \
  --dir "$TMP/campaign" --stage pairs --pair-budget 500000)
```

## What changed

1. **Transactions:** pair-table chunks are 4,096 candidates instead of 256,
   with one `executemany` per chunk. `synchronous=NORMAL` is used only while
   building the rebuildable pair table and restored to the previous mode
   afterwards. Other tables keep `FULL`.
2. **Field multiplication:** `Onb.mul` now walks set bits with `bit_length`
   and inlines the rotation, removing a Python call per set bit. A 3,000
   multiplication microbenchmark moved from 27.3k to 36.7k mul/s (+34%).
3. **Batch inversion:** `Curve.addBatch` uses Montgomery's trick, one field
   inversion per chunk instead of one per pair. Identical-x pairs (doubling or
   inverse) take the scalar path. Differential tests compare every result with
   `Curve.add` for 500 random pairs plus all doublings on a small curve, and
   for 64 random pairs at degree 131.
4. **Process workers:** `--pair-workers N` computes each 4,096-entry chunk's pair sums in spawned workers, then the parent writes rows and extends the rolling checksum in the original order. `ecc2k130` defaults to `min(8, CPU count)`; `ic fixed` defaults to one worker. The 20,000- and 100,000-candidate serial/parallel runs produce identical `pair_state` hashes.
5. **Index-off bulk build:** above 100,000 planned candidates, `pair_lookup`
   is dropped for the insert phase and created once when the table completes.
   This halves the database during the build and removes B-tree maintenance
   from every row. `decomposeExact` recreates the index if a completed table
   lost it. The disk preflight reserves the final index (`192` bytes per
   candidate) so a successful build cannot run out of space at index creation.

## Crash boundary

A crash can lose at most one 4,096-candidate chunk. The cursor, rolling
checksum, and rows commit together; `checkPairs` validates the committed
cursor and checksum before resuming. If a crash lands after the last row but
before index creation, the next completed build or `decomposeExact` creates
the index. The pair table is target-independent and rebuildable, so this trade
is explicit rather than silent.
