# N83 a=0 K2048 five-summand XCNF gate

The retained 339,968-point K2048 signed-Frobenius base supports a checked
five-summand constructed relation in the shared-root chained-S3 encoding.
The native Rust gate ran CryptoMiniSat 5.14.7 against the frozen original
XCNFs. Its constructed control returned SAT in 839 conflicts and 0.423803125
seconds of process wall; separate Rust replay checked all 1,141,221 original
CNF clauses, 4,980 XOR rows, all 339,968 archive rows, curve group sum,
subgroup membership and the 2,048-column relation row. The first same-public-
point natural query returned `INDETERMINATE` at 1,000,001 conflicts after
137.753047167 process-wall seconds. This one capped query supplies no
relation-yield estimate or end-to-end factor-base ranking. [pcc:unverified]
The native measurement claim `PCC-20261114-27045d` has `not_covered`
standing: the retained audit is unsigned and the measurement claim lacks
keyring-signed recompute and replay proofs.

| Cell | Positive control | Public natural query |
| --- | ---: | ---: |
| Curve | N83 Koblitz `a=0`, subgroup order 2,417,851,639,230,796,216,685,689 | same |
| Actual base points / signed-Frobenius columns | 339,968 / 2,048 | same |
| Summands / identity mask | 5 / 0 | 5 / 0 |
| Formula variables / CNF clauses / XOR rows | 111,478 / 1,141,221 / 4,980 | 111,478 / 1,140,557 / 4,980 |
| Solver exit / terminal status | 10 / `SATISFIABLE` | 15 / `INDETERMINATE` |
| Conflicts | 839 | 1,000,001 |
| Native solver process wall seconds | 0.423803125 | 137.753047167 |
| Native solver user / system CPU seconds | 0.263083 / 0.033797 | 136.349600 / 1.044079 |
| Native wait4 peak RSS bytes | 83,705,856 | 1,422,950,400 |
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

The [native evidence](native-evidence/) retains source-bound commands, three
15-second host samples, wait4 receipts, compressed complete solver transcripts,
the independent [positive replay](native-evidence/positive/replay.json), and
the [native audit](native-evidence/audit.json). The Rust
[auditor](../../../../examples/n83_k2048_m5_native_audit.rs) recomputes
retained file hashes, expands the original XCNFs and solver transcripts,
checks source and command bindings, host gates, replay counts and terminal
classifications. It reports `PASS_NATIVE_RAW_CUSTODY_AND_TERMINAL_CLASSIFICATION`.
Both arms had preflight load below 24 and stable swapouts at 164,320. The exact
native runner source commit was `d75033d50260210c8f094d00acfbccc1ad35d395`,
runner binary SHA-256 `b0ab62790d2ef1a7e1ddfd2a654f666f138cecb355bfa4e9aece4c16ec166db8`,
and replay binary SHA-256 `d0f90335c5674c8145aeac8686f0fbaddeb6fab4b51e007eeecbff08a20aacd5`.
The original full XCNFs and archive remain under
`/Volumes/SSD990-2/tmp/n83-a0-k2048-m5-20261114/raw`; the repository retains
compressed copies of both XCNFs in [historical evidence](evidence/).

The earlier Python-orchestrated local run remains retained in the historical
[raw manifest](evidence/RAW_MANIFEST.json) with its own
[audit](evidence/audit.json). It returned the same SAT/`INDETERMINATE` terminal
classes and conflict counts, with process walls 1.024 and 137.253 seconds.
Those historical timings are separate from the native measurements above.
This is a local L0 solver diagnostic; no Tier 0 or Tier 1 speedup comparison
is inferred from the wall times. [pcc:unverified]

The next selection gate on `a=0` needs a common-query yield and novel-rank
panel, full-width logarithm solve, unseen-target descent, charged cold and
online phase costs, and an independently verified rho solve on the same
public point and calibrated resources. The `a=1` curve is a separate
53-bit-subgroup comparison; its already approved single two-instance Spot
allocation remains price-gated under the frozen bid.
