# Alternating full K1107 four-byte root-index control

Two alternating, sequential K1107 pairs used the same macOS ARM64 release
binary (`4de2b556aa4963f125ac0fb86b58467d71cb7bcdaf2d74b2e8d99880ea0b6449`),
same previously public point, and the same 183,762-point factor base. The
only algorithm flag changed between arms was `KIC_INDEXED4_FP2`. Every arm
completed within its 900-second cap, reached rank 1,107, reconstructed the
same four-point relation, and independently replayed scalar
`3640978018804724`. Both [plain→FP2](runs/plain_fp2) and
[FP2→plain](runs/fp2_plain) retained raw process receipts and report
`PASS_PAIRED_FULL_REPLAY`. The exact producer source SHA-256 was
`a99c5a554af48949fe253c28a440ce03095ecdc14eadc8e6087dab8df509e049`.

| Order | Layout | Index build, s | Rank stage, s | Whole process, s | Target online, ms | Peak producer RSS, GiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| plain→FP2 | plain | 57.847 | 95.062 | 154.066 | 363.975 | 1.338 |
| plain→FP2 | FP2 | 57.763 | 92.454 | 151.435 | 370.575 | 1.376 |
| FP2→plain | FP2 | 93.048 | 98.691 | 193.284 | 384.499 | 1.358 |
| FP2→plain | plain | 138.966 | 109.189 | 250.171 | 1,132.299 | 1.323 |

The plain/FP2 index-build ratios were 1.001 and 1.493 in the two orders;
the whole-process ratios were 1.017 and 1.294. The local host was shared and
its load varied substantially: `uptime` reported load averages 22.03,
24.04, 58.08 before the first pair; 276.40, 257.83, 177.12 during the
reverse pair; and 146.91, 207.99, 194.52 afterward. These paired costs are
exploratory diagnostics. Their spread does not justify selecting the FP2
layout for K5500, where its side array would add 0.75 GiB. A fresh Tier 1
host and larger-base run are the next performance and peak-RSS gates.

`run.py` fixes the public point, binary, producer source, flags and arm caps.
`verify.py` independently checks the retained source through Git, binary and
input digests, exact base dump, rank, group relation, scalar and exclusive
target phases. `archive.py` links the byte-identical base dump and binary
already retained by the parent ID31 and FP2 controls, avoiding duplicate
large files. Run the checked archive replay from the repository root:

```sh
python3 -m research.n83_fourbyte_pair_20261027.verify \
  research/n83_fourbyte_pair_20261027/runs/plain_fp2
python3 -m research.n83_fourbyte_pair_20261027.verify \
  research/n83_fourbyte_pair_20261027/runs/fp2_plain
```
