# N83 a=0 K2048 five-summand XCNF gate

The retained 339,968-point K2048 signed-Frobenius base supports a checked
five-summand constructed relation in the shared-root chained-S3 encoding.
CryptoMiniSat 5.14.7 returned SAT in 839 conflicts; independent replay checked
all 1,141,221 original CNF clauses, 4,980 XOR rows, the curve group sum,
subgroup membership and the 2,048-column relation row. The first same-public-
point natural query reached the frozen one-million-conflict cap and returned
`INDETERMINATE` after 137.253 seconds. This is one censored query; it supplies
no relation-yield estimate or end-to-end factor-base ranking.

| Cell | Positive control | Public natural query |
| --- | ---: | ---: |
| Curve | N83 Koblitz `a=0`, subgroup order 2,417,851,639,230,796,216,685,689 | same |
| Actual base points / signed-Frobenius columns | 339,968 / 2,048 | same |
| Summands / identity mask | 5 / 0 | 5 / 0 |
| Formula variables / CNF clauses / XOR rows | 111,478 / 1,141,221 / 4,980 | 111,478 / 1,140,557 / 4,980 |
| Solver exit / terminal status | 10 / `SATISFIABLE` | 15 / `INDETERMINATE` |
| Conflicts | 839 | 1,000,001 |
| Solver process wall / CPU seconds | 1.024 / 0.279 | 137.253 / 136.857 |
| Peak RSS | sampled peak is incomplete for the brief control | 1,413,316,608 bytes from `wait4` |
| Original XCNF and group replay | PASS | no model |
| Checked relations at this cap | 1 constructed control | 0 |
| Final rank / target descent / same-point rho | unmeasured | unmeasured |
| Selected end-to-end total | unknown | unknown |

The natural target is the frozen public point
`(1834913121178197242856238, 8797964190732846213981377)` on the exact
`a=0` curve. The constructed control uses distinct stored orbit representatives
in columns 0–4, pins their five x coordinates, three intermediate x blocks,
and the resulting target x. Both formulas share the same source-bound
five-summand S3 circuit and 2,048-representative root domain. The natural
formula removes all 747 control pins. The nominal unordered five-multiset
count per subgroup element is 15.6528; it is a capacity calculation, not an
observed rate. K2048 m5 and full-D18 m4 change both base and arity.

The [protocol](PROTOCOL.md) was committed before export and amended with exact
solver limits and formula hashes before each solver call. The exporter source
was committed at `54753787383afe3a2a0fe7e7773dc47941e50025`; its binary
SHA-256 is `8f611eaf35b1e071a4ce05567685f425fb1dbfb446f57415921f4b2f6593f432`.
The positive export used that clean commit; the natural export used clean
commit `9c993d2f2b2b4835dca4b480045ea337d66f472f`, with the same binary
and unchanged compiled exporter code. The input object was independently
downloaded from the retained S3 URI and checked at 10,313,647 bytes,
SHA-256 `aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`.

The solver used one thread, seed 20261114, `--maxconfl 1000000`,
`--maxtime 600`, a 660-second process-group watchdog, `--maxnummatrices 32`,
`--autodisablegauss 0`, `--gaussusefulcutoff 0`, and VSIDS branching. The
natural transcript records 28 messages declining wide Gaussian matrices,
including 20,916-column matrices. Thus this run tests those exact solver flags
with CryptoMiniSat's remaining column admission limit; it does not measure a
wide-matrix variant. The prior [matrix-admission smoke](../external-xor-heuristic-panel-20261025/RESULT.md)
showed `--maxmatrixcols 25000` can admit 20,916-column matrices on a different
N83 formula, with no established runtime improvement. A new locked arm on
this exact formula is the concrete next solver diagnostic.

The [raw manifest](evidence/RAW_MANIFEST.json) retains compressed original
XCNFs and solver transcripts, export receipts, commands, host samples,
positive independent replay and both terminal receipts. The
[byte-level audit](audit_raw.py) recomputes each retained file hash, expands
both XCNFs/transcripts, checks source and command bindings, host gates,
positive replay and terminal conflict counts; its [result](evidence/audit.json)
is `PASS_RAW_CUSTODY_AND_TERMINAL_CLASSIFICATION`. The full uncompressed files
remain under `/Volumes/SSD990-2/tmp/n83-a0-k2048-m5-20261114/raw`.
The host's three 15-second preflight samples stayed below load 24 in each
arm, and swapouts stayed 164,320. This is a local L0 diagnostic, so its wall
times are not a Tier 0 or Tier 1 speedup comparison. [pcc:unverified]

The next selection gate on `a=0` needs a common-query yield and novel-rank
panel, full-width logarithm solve, unseen-target descent, charged cold and
online phase costs, and an independently verified rho solve on the same
public point and calibrated resources. The `a=1` curve is a separate
53-bit-subgroup comparison; its already approved single two-instance Spot
allocation remains price-gated under the frozen bid.
