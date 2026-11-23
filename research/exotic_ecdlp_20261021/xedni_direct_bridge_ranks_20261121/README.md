# Three direct bridge curves have proved rational rank one

Proof-enabled Sage rank calls and Simon two-descent independently agree on
exact rational Mordell–Weil ranks for all five direct coefficient-two bridge
curves. Source indices **2, 201, and 511 have rank one**; indices **85 and
353 have rank two**. Each method completed within the frozen 25-second
per-cell cap. Consequently, three of these five explicit curves meet the
`d<2` premise, while the two rank-two curves fail it.

| Source index | Simon lower–upper | Proof-enabled PARI rank | `d<2` |
| ---: | --- | ---: | --- |
| 2 | 1–1 | 1 | yes |
| 85 | 2–2 | 2 | no |
| 201 | 1–1 | 1 | yes |
| 353 | 2–2 | 2 | no |
| 511 | 1–1 | 1 | yes |

The [protocol](protocol.json) fixes the five curve records by source hash,
method order, parameters, and cap. The checked Sage launcher runs one worker
per curve and method, retaining all ten status, elapsed-time, stdout, and
stderr records in [`cells.jsonl`](cells.jsonl). Its
[`summary.json`](summary.json) records ten completed cells. The
[`audit.py`](audit.py) checks source identity, cell order, caps, agreement of
both methods, proof flags, and the known rational point in every Simon
witness set; [`audit-result.json`](audit-result.json) is `PASS`.
The [`sage-runtime-info.json`](sage-runtime-info.json) receipt identifies
the accepted build.

The five exact rational group relations were already proved in Lean in the
[direct-lift packet](../xedni_direct_word2_lifts_20261121/README.md). A Lean
formalization of rational Mordell–Weil rank, descent, and the five rank
certificates is beyond the available project library; the rank statements
here are Sage proof-enabled computations corroborated by Simon bounds, not
Lean declarations. The proof map for each relation remains in that packet.

The rank-two outcomes identify a concrete next construction search: vary
the two free rational Weierstrass coefficients while preserving reduction
and the selected `P+2Q` word, then check whether rank-one curves exist for
source indices 85 and 353. Until replacements are found, the six-row
rank-20 witness does not consist entirely of curves verified to satisfy
`d<2`. The previously measured index-425 curve has its own unresolved
rank interval and is kept separate.

Replay from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info
python3 research/exotic_ecdlp_20261021/xedni_direct_bridge_ranks_20261121/run.py
python3 research/exotic_ecdlp_20261021/xedni_direct_bridge_ranks_20261121/audit.py
```
