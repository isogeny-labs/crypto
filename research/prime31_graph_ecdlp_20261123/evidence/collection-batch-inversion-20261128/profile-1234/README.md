# Collection loop after batching, profiled (levers 1234, 2026-11-28)

This profiles the 4-lever build of `mitm_four_fast.rs`, built with debug
info (`CARGO_PROFILE_RELEASE_DEBUG=true`, `--locked --release`) in the
`577d6c2c` export. Callgrind ran whole processes with caps of 1 and 2
queries, `prime31-graph-mitm-fast {1,2} 1234` (`IR.txt`). Their difference
is one query: **252.8 M instructions for 915,304 probes, 276 per probe**. The
baseline needed 1.898 G, 2,073 per probe.

Costs are summed per source line and per source file straight from the raw
profiles, with inlined standard-library code charged to its own file
(`callgrind_by_file.awk`, `by-file-per-query.tsv`,
`candidate-lines-per-query.tsv`). [pcc:unverified]

## Where one query's 252.8 M goes

| part | instructions | share | per probe |
|---|---:|---:|---:|
| Mersenne field multiplication (`fmul`: product, two folds, final compare; about 6 per probe) | 68.7 M | 27.2 % | 75 |
| other lines of the candidate (batch bookkeeping: `kind`/`num`/`den`/`prefix` stores and loads, backward pass, output write, loop control) | 40.5 M | 16.0 % | 44 |
| table lookup and per-query de-duplication set (hashbrown probing, SSE2 group match, bitmask/tag, multiply-rotate hash 3.7 M) | 38.1 M | 15.1 % | 42 |
| branch subtraction (`fsub`) | 21.9 M | 8.7 % | 24 |
| `ny = (P - y) % P`, a u64 division per probe | 11.9 M | 4.7 % | 13 |
| zeroing per-query buffers (`memset`: `vec![0; n]` for `num`/`den`, `vec![None; n]` for 24-byte output points) | 14.9 M | 5.9 % | 16 |
| slice bounds checks | 11.0 M | 4.3 % | 12 |
| `Vec` push/index | 10.1 M | 4.0 % | 11 |
| other inlined std glue (Option, range iterator, pointer, comparison) | 32.4 M | 12.8 % | 35 |
| below 0.5 M per file | 3.4 M | 1.3 % | 4 |

**Memory** (`cache-sim.txt`, same differencing):
- Per query there are 73.9 M data references (81 per probe) and 4.40 M L1-data misses (4.8 per probe).
- Callgrind models only L1 and a last-level cache sized from this host's 260 MB L3, so L2 behaviour is not shown.
- Wall time is consistent with a mostly compute-bound loop: collection is 683 ms over 11 queries, about 68 ns or 142 cycles per probe at 2.1 GHz for 276 instructions, about 1.9 instructions per cycle. That is an estimate from wall time and nominal frequency, not a counter reading; this host has no PMU.

## Reading

The arithmetic left after batching is about a third of the loop: the
Mersenne multiplications, plus subtraction and the one remaining division.
Batch bookkeeping, buffer zeroing, bounds checks and `Vec` traffic are
another third. Table lookup is about 15 %.

**Exact candidates (not done):**
- Fuse the backward pass with the lookup, so no 24-byte output buffer is written, zeroed and re-read, and reuse `num`/`den`/`prefix` across queries without zeroing.
- Replace `(P - y) % P` with a branch.
- Iterate without indexing, to drop bounds checks.
- Keep the Mersenne product partially reduced between multiplications.
- Use a flatter pair index (one allocation, not a `Vec` per key).

Each must reproduce the baseline's normalised output hash (`../EXACTNESS.txt`).
