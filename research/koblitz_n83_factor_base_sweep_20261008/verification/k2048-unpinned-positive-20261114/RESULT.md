# N83 a=0 K2048 constructed-target unpinned search

The native-XOR five-summand K2048 formula is satisfiable without explicit
witness-coordinate pins: an independent original-XCNF evaluation checked all
1,140,557 CNF clauses and 4,980 XOR rows against the archived complete
positive assignment. A fresh one-thread CryptoMiniSat 5.14.7 search on this
same formula reached the frozen one-million-conflict cap with status
`INDETERMINATE`. Thus the available solver policy did not find a model within
this cap even when a valid decomposition was present. [pcc:unverified]

The exact curve is `icv1-f2m83-tm6151469093347-debefd74`, the primary
N83 `a=0` binary curve over `F₂[z]/(z⁸³+z⁴⁵+z²+z+1)`, subgroup order
`2417851639230796216685689`. The retained K2048 base has 339,968
subgroup-usable points and 2,048 signed-Frobenius columns. The constructed
point is `(985527933806849610456454, 7623568253424067907890851)`, the
group sum of the first representatives of columns 0 through 4. Its x
coordinate is the public circuit input; the five summand and three
intermediate x-coordinate blocks are free. The validation-only witness
remained out of the solver command.

| Gate | Recorded result |
| --- | --- |
| Export source commit | `8354104964bc3f0461271fdf6164c91108394af9` |
| Exporter binary SHA-256 | `54e66ec7a867308953cbb968b77a7bb74f628d596ba6fe784abf17bdb82d093b` |
| Original unpinned XCNF | 111,478 variables; 1,140,557 CNF clauses; 4,980 XOR rows; 20,511,067 bytes; zero explicit witness x pins |
| Formula SHA-256 | `e5166108c31a375a65a9ccd379cf8a2f8c8b6fe80b0f860ade1e6c28b2e0c250` |
| Archived model on unpinned formula | `PASS_ORIGINAL_XCNF`, all 1,145,537 rows checked |
| Solver | CryptoMiniSat 5.14.7, SHA-256 `a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac` |
| Search envelope | Darwin arm64, one thread, seed 20261114, VSIDS, 32 Gaussian matrices, wide XOR columns 25,000, one-million-conflict cap, 600 s solver cap, 660 s outer watchdog |
| Host preflight | Load 3.58, 3.87, 4.64; swapouts 164,320 unchanged through terminal readback; scratch gate passed |
| Terminal | `INDETERMINATE`; exit 15; 1,000,001 reported conflicts; no model |
| Process wall / CPU | 153.815 s wall; 152.278 s user and 0.997 s system CPU |
| Peak RSS | 457,015,296 bytes from wait4 |
| Raw audit | `PASS_CUSTODY_AND_TERMINAL_CLASSIFICATION` |

The [frozen protocol](PROTOCOL.md), [lock](LOCK.json),
[known-model auditor](audit_known_model.py), [runner](run_locked.py), and
[raw auditor](audit_raw.py) define the reproducible boundary. The
[`evidence/`](evidence/) directory retains an artifact-store pointer for the
deterministic gzip original formula and a Git copy of the compressed solver
transcript, exact JSON receipts, host samples, command, and the first
stale-binary export failure. The full original XCNF also remains at the
absolute local path in the export receipt. Hydrate the formula with
`python3 tools/artifacts/artifacts.py pull` using store read access before
rerunning the auditor. The
[audit receipt](evidence/audit.json) binds the versioned copies to their
frozen hashes; it is an unsigned producer-side audit. [pcc:unverified]

This constructed-target gate explains why a million-conflict natural query
without a model cannot by itself determine decomposition frequency. The next
a=0 step is a solver or decomposition producer that yields checked ordinary
relations and novel rank on the common public corpus. Complete same-target
IC/rho timing for both N83 curves remains the factor-base selection gate.
