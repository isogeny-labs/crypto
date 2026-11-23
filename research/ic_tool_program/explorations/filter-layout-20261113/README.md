# Partition-local presence filter (top-bits layout) for CPU folded builds

- **Status:** measured and kept, 2026-11-13, on this session's container.
- **Decision:** the owner chose "CPU-only with a layout tag" after the
  build-step diagnostic
  ([`../build-steps-20261113/`](../build-steps-20261113/README.md)).

## The change (`src/cryptanalysis/koblitz_index_calculus.rs`)

- **`FilterLayout`:** `Low` (today's) or `Top`. `Top` takes a key's filter
  word from the hash's top bits, the bits that also pick the build's
  partition, so each partition fills only its own run of the filter. Its
  in-word bits come from hash bits 16–33.
- **Recorded per table:** every folded table records its layout in
  `FoldedParts` and `FoldedStorage`. Probes follow it through `FilterGeom`,
  the same branch-free arithmetic for both layouts.
- **Default:** CPU folded builds default to `Top`; `KIC_FILTER_LAYOUT=low`
  restores today's layout.
- **Unchanged:** non-folded tables, GPU table files and the GPU-parity
  tools (`examples/dump_fold_vectors.rs`, `examples/load_fold_table.rs`)
  stay `Low`.
- **Tests:**
  - `both_filter_layouts_give_the_same_answers`: same buckets and same
    answers, different filter, on 3 curves; the layout survives a
    round trip through the parts.
  - `the_low_filter_layout_is_filter_slot`: the Low geometry equals the
    old arithmetic exactly.
  - Every Koblitz IC and fast-curve test passes (216).

## A/B ([`ab/PROTOCOL.md`](ab/PROTOCOL.md), frozen before any row; [`ab/report/report.pdf`](ab/report/report.pdf))

42 rounds, same binary, a same-session A/A. Ratios are old over new.

| row | build | set-up | collect |
|:--|:--|:--|:--|
| k0n53 | **1.104** [1.069, 1.142], established | 1.033 [1.007, 1.061], not established | 0.996, no regression |
| k0n61 | **1.142** [1.098, 1.164], established | 1.000 [0.983, 1.034], not established | 0.991, no regression |

- **Mechanism:** only the counting sort plus filter fill moves, from 10.69
  to 7.86 and from 12.68 to 8.38 ns per stored pair.
- **Identity:** zero mismatches.
- **Scope:** a component result for the build phase on this host class.
  It makes no claim against rho and adds no board row.
