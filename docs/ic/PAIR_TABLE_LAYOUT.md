# Pair-table DRAM / layout knobs (m=3 filters)

Index-calculus m=3 spend is dominated by random DRAM on the presence
filter and bucket runs, not by another software bloom. This note lists
what the host pair-table path already enables, what is experimental, and
where GPU / NUMA fit. No second bloom is added here.

## Enabled by default (same answers and counts)

| Knob | Default | Effect |
|:--|:--|:--|
| `KIC_HUGE_PAGES` | on (`0` off) | `MADV_HUGEPAGE` on large filter / bucket / rest arrays before first write (Linux). Contents and counted probes unchanged; TLB misses drop when the kernel honours the advice. |
| `KIC_FILTER_LAYOUT` | `Top` for in-process folded builds (`low` for the file/GPU layout) | Partition-local filter words (`Top`) vs legacy low-bit words (`Low`). Same Bloom size; different false-positive sets. |
| `KIC_FILTER_DOUBLE` | on for `Top` | One extra filter bit of width on folded `Top` tables (~⅕ the absent-key admits). Off keeps the old width. |
| `KIC_FOLD_PACK` | on | One stream per partition (orbit packed into free hash bits) instead of two. Build layout only. |
| `KIC_SCAN_SOA` / `KIC_SCAN_PAIRED` / `KIC_SCAN_VFILTER` | on | Aimed-scan locality (SoA / ± pairing / vector filter). Algorithm unchanged. |

## Experimental / external

| Path | Status | Notes |
|:--|:--|:--|
| `gpu/ecc2k/pairtable.cuh` | Experimental for large `\|F\|` | Host folded builds can round-trip the Low layout word-for-word with the CUDA builder (`FilterLayout::Low`, `FILTER_PROBES=3`). Use for capacity, not as a second host bloom. |
| ecbench NUMA pinning | Enabled in measured sessions | `ecbench::isolation` pins the run CPU and binds memory to its node; `/proc/self/numa_maps` is read back. Not a library default for every `PairSumTable::build` — place the process with the isolab / ecbench harness when claiming DRAM locality. |
| macOS / non-Linux hugepages | No-op | `advise_huge_pages` compiles away; layout knobs still apply. |

## What not to do

- Do not add another host-side bloom on the same keys (RootBloom stays on Semaev root tables only).
- Do not re-fuse the blocked m=3 key/lookup split.
- Do not treat hugepage or NUMA advice as an S-column speedup without a paired cold measurement.

## Related evidence

- Filter / probe blocking: `docs/ic/runs/koblitz-probe-window-20260921.json`
- Tier width / DRAM crossover: `docs/ic/runs/koblitz-tier-crossover-20260921.json`, `docs/ic/runs/koblitz-phase-prices-20260921.json`
- Fold packing and Top layout: comments on `FilterLayout` / `fold_pack_enabled` in `src/cryptanalysis/koblitz_index_calculus.rs`
