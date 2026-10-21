# N83 K128 split root-index stage controls

The split five-byte root index preserved the same 21,248-point factor base,
680,000 symmetric states, and 1,359,488 exact-root entries as the monolithic
index in four paired K128 controls. The two orders gave opposite timing
directions, so the index-build comparison remains inconclusive.

The dual-filter pairs ran monolithic then split in
[k128_a](runs/k128_a/result.json) and split then monolithic in
[k128_b](runs/k128_b/result.json). The shared-filter pairs used the same
orders in [k128_shared_a](runs/k128_shared_a/result.json) and
[k128_shared_b](runs/k128_shared_b/result.json). Each directory also retains
`input.json`, the frozen runner, child stdout/stderr, and process receipts.
The `status` in every paired result is `PASS_INDEX_STAGE_CONTROL`.

| Pair | Monolithic index build (s) | Split index build (s) | Earlier arm |
| --- | ---: | ---: | --- |
| dual A | 22.393 | 15.919 | monolithic |
| dual B | 14.285 | 15.502 | split |
| shared A | 18.030 | 28.390 | monolithic |
| shared B | 24.862 | 14.632 | split |

All four controls ran on the shared macOS host. The stage executable stopped
after index construction, before rank, target relation search, recovery, or
scalar replay. The target file was an invocation fixture only. The chart
[index_build.svg](index_build.svg) and its
[PNG render](index_build.png) display these exact stage intervals. The
[run PDF](RUN.pdf) and [result PDF](RESULT.pdf) were generated from the four
versioned `result.json` files by [render_report.py](render_report.py).

The split tables allocated 13,107,200 bytes versus 10,485,760 monolithic.
At K128 the union Bloom matched monolithic's 2,097,152 bytes, saving
524,288 bytes relative to separate filters on the split tables. At K4405,
the frozen layout calculation predicts a 128 MiB Bloom saving and
671,088,640 additional table bytes. Those K4405 values are capacity
calculations; [RESULT.md](RESULT.md) retains the qualification boundary and
the next full-size control.

The separate [K5086 capacity calculation](CAPACITY.md) is illustrated by
[capacity.svg](capacity.svg) and its [PNG render](capacity.png). It is shown
on the second page of [RESULT.pdf](RESULT.pdf) with measured K128 timing kept
on the first page.
