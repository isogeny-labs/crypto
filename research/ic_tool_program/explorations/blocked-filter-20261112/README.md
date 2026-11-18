# A split-block presence filter for the folded table (lever 3): no gain

**What this is.** A timed exploration on `main` at `0a1969d089`, plus this
branch's probe port. It ran on 2026-11-12 on this session's container
(Intel Xeon @ 2.1 GHz, 4 vCPU, AVX-512), not on a reference-class host. It
is a record, not a round, and none of its runs is pooled with any round's.

**Outcome:** dropped. Once the probe and the build are vectorized, the
split-block filter is within noise of today's filter at n = 53 and 2^47.2,
at both 8 and 16 bits a key, and slower before that. Every arm gave the
default's certificates and counts on every row.

## Candidate

[`candidate.patch`](candidate.patch), on top of the branch's probe port. It
is reverted from the branch: the default build is unchanged, and the patch
is kept here.
- **Switch:** `KIC_FILTER=blocked` (16 bits a key) or `blocked8` (8 bits a
  key, today's size). Only `fold_partitioned` builds the blocked filter;
  the two-pass fallback keeps today's.
- **Layout:** one bit in each of the eight words of a 64-byte line, picked
  by the split-block Bloom salts on the hash's top half. The line is
  aligned through eight spare words.
- **Huge pages:** `KIC_FILTER_HUGE=1` asks for 2 MiB pages on a fresh
  filter (`madvise`). Transparent huge pages run in `madvise` mode on this
  host.
- **Probe and set:** AVX-512, chosen at run time: one 512-bit load, the
  salted products, one test (probe) or one OR and store (set). The first
  version was scalar.
- **Unchanged:** the byte estimate (`folded_byte_size`), so the choice of
  table tier does not move. Answers are unchanged too, because every
  admitted key is still checked against the entries.

## Runs

`M1-T01` at `icv1-f2m53-tm56619371-dac20a85` (2^44.3) and `icv1-f2m61-t158598901-ab42b6c5` (2^47.2) from the suite. Arms are
interleaved within each round, three rounds, under `isolated_bench --cpus
2` with `RAYON_NUM_THREADS=1`. A run the busy gate refused was asked for
again. Raw outputs are in [`runs.tar.xz`](runs.tar.xz), with hashes in
[`SHA256SUMS`](SHA256SUMS). Each figure is the mean over clean processes
of each process's median over its repetitions, in ms.

**1. Scalar probe and set (`explore3`)**

| row | arm | processes | collect | build | set-up |
|:--|:--|--:|--:|--:|--:|
| icv1-f2m53-tm56619371-dac20a85 | default | 3 | 200.3 | 106.5 | 357.8 |
| icv1-f2m53-tm56619371-dac20a85 | huge pages | 3 | 200.3 | 112.8 | 375.5 |
| icv1-f2m53-tm56619371-dac20a85 | blocked 8 | 2 | 268.6 | 118.6 | 436.1 |
| icv1-f2m53-tm56619371-dac20a85 | blocked 16 | 2 | 252.2 | 144.9 | 481.3 |
| icv1-f2m53-tm56619371-dac20a85 | blocked 16, huge | 3 | 247.0 | 136.9 | 441.6 |
| icv1-f2m61-t158598901-ab42b6c5 | default | 3 | 1120.4 | 202.3 | 1399.7 |
| icv1-f2m61-t158598901-ab42b6c5 | huge pages | 3 | 1092.6 | 213.0 | 1397.2 |
| icv1-f2m61-t158598901-ab42b6c5 | blocked 8 | 3 | 1408.3 | 243.6 | 1725.0 |
| icv1-f2m61-t158598901-ab42b6c5 | blocked 16 | 3 | 1403.0 | 247.4 | 1722.3 |
| icv1-f2m61-t158598901-ab42b6c5 | blocked 16, huge | 3 | 1385.6 | 236.5 | 1704.0 |

**2. AVX-512 probe, scalar set (`explore3b`)**

| row | arm | processes | collect | build | set-up |
|:--|:--|--:|--:|--:|--:|
| icv1-f2m53-tm56619371-dac20a85 | default | 3 | 212.9 | 110.3 | 379.7 |
| icv1-f2m53-tm56619371-dac20a85 | blocked 8 | 3 | 202.5 | 124.3 | 393.5 |
| icv1-f2m53-tm56619371-dac20a85 | blocked 16 | 1 | 199.1 | 157.6 | 407.1 |
| icv1-f2m61-t158598901-ab42b6c5 | default | 3 | 1101.9 | 199.2 | 1392.6 |
| icv1-f2m61-t158598901-ab42b6c5 | blocked 8 | 3 | 1075.0 | 220.2 | 1374.2 |
| icv1-f2m61-t158598901-ab42b6c5 | blocked 16 | 3 | 1114.8 | 242.6 | 1443.1 |

**3. AVX-512 probe and set (`explore3c`, the final candidate)**

| row | arm | processes | collect | build | set-up |
|:--|:--|--:|--:|--:|--:|
| icv1-f2m53-tm56619371-dac20a85 | default | 3 | 196.9 | 108.9 | 363.7 |
| icv1-f2m53-tm56619371-dac20a85 | blocked 8 | 3 | 203.1 | 116.7 | 374.4 |
| icv1-f2m53-tm56619371-dac20a85 | blocked 16 | 2 | 185.8 | 117.4 | 360.0 |
| icv1-f2m61-t158598901-ab42b6c5 | default | 3 | 1073.7 | 208.2 | 1367.5 |
| icv1-f2m61-t158598901-ab42b6c5 | blocked 8 | 3 | 1083.9 | 204.7 | 1371.3 |
| icv1-f2m61-t158598901-ab42b6c5 | blocked 16 | 3 | 1083.2 | 223.7 | 1386.6 |

## Reading

- **No layout gains.** In the final candidate, set-up against the default
  is +2.9% and −1.0% at icv1-f2m53-tm56619371-dac20a85 (blocked 8, blocked 16), and +0.3% and +1.4%
  at icv1-f2m61-t158598901-ab42b6c5. The A/A spread of these runs is several percent: the default
  itself moved between 357.8 and 379.7 ms at icv1-f2m53-tm56619371-dac20a85 across the three
  explorations.
- **Why, by inference:** a probe costs about the same whatever the
  layout, because the filter line's miss dominates. Lowering the
  false-positive rate (modelled at about 0.028 → 0.0005 at 16 bits a key)
  removes too little of the admitted stage to show, and a filter twice as
  large costs the build.
- **Huge pages alone** (A16's idea, retried on today's code) gain nothing
  at icv1-f2m53-tm56619371-dac20a85 and 2.5% in collect at icv1-f2m61-t158598901-ab42b6c5, inside the same spread.
- **Consistent with history:** A14–A19 found the filter stage near
  exhausted.

## Lead

`LEAD-ICB-koblitz-setup-phase-cost`: a split-block filter adds no
measurable gain on this host. A reference-class or EPYC rerun was not
possible, because the Iso Lab bridge's CodeBuild quota reads 0
([`../scan-stages-main-epyc-20261111/`](../scan-stages-main-epyc-20261111/README.md)).
