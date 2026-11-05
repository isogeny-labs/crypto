# N83 first-fall certificate sweep

32 exact first-fall certificates; 16 bound-only results; all 48 certificate computations and fresh-process replays completed. Last fall remains unmeasured. These are observations and checked algebraic certificates, not an independent research review.

Balanced ell=floor(83/m), candidate x-domain 2^ell. P/N denotes planted/natural. A bound is not an attained degree.

| m | ell | Koblitz P/N | Binary P/N | Prime P/N |
|---:|---:|---|---|---|
| 2 | 41 | 2 / 2 | 2 / 2 | >=5 (bound) / >=5 (bound) |
| 3 | 27 | 2 / 2 | 2 / 2 | >=5 (bound) / >=5 (bound) |
| 4 | 20 | 2 / 2 | 2 / 2 | >=5 (bound) / >=5 (bound) |
| 5 | 16 | 2 / 2 | 2 / 2 | >=5 (bound) / >=5 (bound) |
| 6 | 13 | 2 / 2 | 2 / 2 | >=5 (bound) / >=5 (bound) |
| 7 | 11 | 2 / 2 | 2 / 2 | >=5 (bound) / >=5 (bound) |
| 8 | 10 | 2 / 2 | 2 / 2 | >=5 (bound) / >=5 (bound) |
| 9 | 9 | 2 / 2 | 2 / 2 | >=5 (bound) / >=5 (bound) |

Binary method: no nonzero input generator below degree two; a constant combination of quadratic rows produces a nonzero linear polynomial. Both pivot orders agree and the saved combination is reconstructed from the original equations in a separate process. Prime method: the selector ideal has no degree falls and a nonsingular degree-four coefficient minor excludes additional falls through degree four. Last fall and regularity do not follow from these certificates.

All exact values depend on the specified S3-chain presentation. The binary rank difference counts lower-degree vectors in the original quadratic span, not the full closed V(F,2). Candidate domains have not been exhaustively lifted to curve points.

Controls: degree-two-to-linear positive; duplicate, homogeneous and zero-row negatives; seven native unit tests. Source, binary, input-list and per-system hashes are pinned in protocol.json. Same-code fresh-process replay is not an independent scientific review. Wall times are invalid for performance claims.

Evidence: experiment.export.json, evidence/results.json, evidence/replay.jsonl, terminal.json and DESIGN.md. Earlier regularity failures are preserved in the parent study. Exact prime first/last values remain part of the active goal; lower bounds do not complete it.

References: [Kosters and Yeo](https://arxiv.org/html/1503.08001); [Caminata and Gorla](https://arxiv.org/html/2112.05579); [Kousidis and Wiemers](https://arxiv.org/html/1906.05594).
