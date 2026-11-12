# Two more IC set-up levers, both null: build-side pairing and filter lookahead

**What this is.** A timed exploration on `main` at `21f21a45a3`. It ran on
2026-11-12 on this session's container: Intel Xeon @ 2.1 GHz, 4 vCPU,
AVX-512, 2 MiB L2 per core, 260 MiB L3. This is a record, not a round, and
none of its runs is pooled with any round's. Both changes are reverted
from code.

**Outcome:** both dropped. Neither moves set-up beyond this host's
run-to-run spread, and the arithmetic says neither could.

## 1. One inverse per ± twin pair in the fold build

- **Patch:** [`candidate.patch`](candidate.patch).
- **The idea:** the paired kernel from lever 2
  ([`../paired-inverse-20261112/`](../paired-inverse-20261112/README.md))
  applied to `build_folded_within`'s rows.
- **Why it applies:** the folded rows sort the base by `(orbit, index)`.
  A point and its negation sit at indices `2j` and `2j + 1` and share a
  signed orbit. So every orbit boundary is even, and each row past its own
  orbit is made of whole twin pairs. The patch checks this and falls back to
  the unpaired kernel otherwise.
- **Control:** `KIC_BUILD_PAIRED=0`.
- **Checks:**
  - `the_partitioned_fold_build_is_one_table_word_for_word_on_any_thread_count`
    passes with pairing on and off; it checks that the bulk build equals the
    scalar build word for word.
  - `koblitz_index_calculus::` and `koblitz_fast::` give 214 passed and
    0 failed.
  - Stored pairs and status are identical in every run below.
- **Runs:** `M1-T01` at `k0n53` and `k0n61`, four rounds, arm order
  alternating, under `isolated_bench --cpus 2` with `RAYON_NUM_THREADS=1`
  ([`run-build.sh`](run-build.sh)). Each figure is the per-process median
  over repetitions, in ms.

| row | arm | build (r1–r4) | collect (r1–r4) |
|:--|:--|:--|:--|
| k0n53 | unpaired | 103.3, 108.5, 108.3, 120.6 | 183.5, 193.5, 187.5, 193.8 |
| k0n53 | paired | 108.2, 108.7, 109.0, 111.5 | 187.3, 193.2, 191.6, 214.4 |
| k0n61 | unpaired | 209.2, 205.8, 210.5, 182.2 | 950.0, 1064.4, 1076.4, 1075.2 |
| k0n61 | paired | 198.8, 192.0, 209.3, 195.1 | 1046.1, 1075.4, 1119.6, 1052.7 |

- **Why it cannot show (inference):** the build stores 3.71 M pairs at
  `k0n53` and 6.27 M at `k0n61`. Lever 2's micro-benchmark saved about
  0.55 ns per summand, which gives about 2 ms of a 108 ms build and about
  3.5 ms of a 200 ms build. That is about 0.6% and 0.3% of set-up, below
  the spread above. The build's remaining time per stored pair, about
  27 ns, sits in hashing, partition streams, the counting sort and the
  filter fill, not in the additions.

## 2. Filter-probe prefetch distance in the scan

- **The change:** a temporary binary that read the scan's `LOOKAHEAD`
  (fixed at 32 in the code) from `KIC_LOOKAHEAD`
  ([`run-lookahead.sh`](run-lookahead.sh)). The binary's SHA-256 is
  `6a70504b…ee90b7c`.
- **Runs:** three rounds, with the order of the four values reversed every
  other round. Each figure is collect in ms, per-process median.

| row | 16 | 32 | 64 | 128 |
|:--|:--|:--|:--|:--|
| k0n53 | 200, 199, 215 | 210, 202, 194 | 196, 194, 200 | 170, 193, 192 |
| k0n61 | 1079, 887, 1095 | 1109, 1102, 1116 | 1097, 1057, 1041 | 1082, 1067, 1135 |

- **Reading:** no value separates from the others. The filter (8 bits per
  key, 3.7–6.3 MB) is larger than one core's L2, so probes are served from
  L3, and a deeper prefetch window does not add overlap that shows.
  Together with lever 3, this leaves the filter stage as found.

## Lead

`LEAD-ICB-koblitz-setup-phase-cost`: neither the build's additions nor
the filter's prefetch depth is where the set-up's time goes on this host.
The build's unexplained cost is about 27 ns per stored pair, which the
pair streams and the counting sort would have to explain. A per-step probe
of `fold_partitioned` is the next measurement before any further lever.

Raw outputs: [`runs.tar.xz`](runs.tar.xz), with hashes in
[`SHA256SUMS`](SHA256SUMS).
