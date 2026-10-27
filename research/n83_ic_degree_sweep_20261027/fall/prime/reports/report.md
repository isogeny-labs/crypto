# N83 standard first-fall grid

All requested m=2..9 cells have a standard leading-syzygy first-fall value under the stated S3-chain presentation. Binary/Koblitz values come from direct quadratic elimination and explicit nonzero linear witnesses (R3). Prime values use the written Lefschetz/filtered-Koszul lower bound plus native-checked factored witnesses (R4); they are not full n83 matrix-rank measurements. Full Lean formalization and independent research review remain open.

| m | ell=floor(83/m) | Candidate x-domain | Koblitz P/N | Binary P/N | Prime P/N (proof-assisted) |
|---:|---:|---|---|---|---|
| 2 | 41 | 2^41 | 2 / 2 | 2 / 2 | 24 / 24 | 
| 3 | 27 | 2^27 | 2 / 2 | 2 / 2 | 17 / 17 | 
| 4 | 20 | 2^20 | 2 / 2 | 2 / 2 | 14 / 14 | 
| 5 | 16 | 2^16 | 2 / 2 | 2 / 2 | 12 / 12 | 
| 6 | 13 | 2^13 | 2 / 2 | 2 / 2 | 10 / 10 | 
| 7 | 11 | 2^11 | 2 / 2 | 2 / 2 | 9 / 9 | 
| 8 | 10 | 2^10 | 2 / 2 | 2 / 2 | 9 / 9 | 
| 9 | 9 | 2^9 | 2 / 2 | 2 / 2 | 8 / 8 | 

Prime result: d_ff=floor(ell/2)+4 under the standard definition as the first nontrivial leading-form syzygy modulo Koszul. The explicit multiplier is a product of differences of paired normalized selector variables. Its product with the endpoint leading quartic vanishes, and its affine product is nonzero and has lower degree. The native certificate checks this for each original target. The lower-bound argument uses Phuong–Tran Theorem2.7 and a finite filtered Koszul complex whose leading operators act on disjoint tensor factors; see ARGUMENT.md.

This is distinct from the earliest NEW affine degree fall. The prime affine product's novelty in V(F,d-1) is not established. That quantity, all last-fall degrees and regularity remain unresolved. R3 binary value2 certifies both notions. The original96 regularity attempts remain resource-limited; these new results do not turn them into completed regularity measurements.

Validation: five explicit small-matrix controls over65537, three native R4 tests, all16 original-presentation replays and16 new certificate replays, pinned source/binary/image and per-input hashes. R3 independently records32 binary witnesses and48 replays. Same-code replay is not independent scientific review. Lean4.34.1 checks one structural upper-bound dependency without axioms; PROOF_MAP.md lists missing quotient/Koszul/Lefschetz bridges. No complete formal theorem is claimed.

Frozen R4 protocol6e12ffdda6e1610783f0e3d0caea595387737eb798900659f3466d50edcf79b7. Source0c9543a2b5a95266792dadf15a921ac24a779bf0. Exact job request, immutable image, terminal receipt and raw S3 location are preserved beside the unified export. Wall times are diagnostic. No statistical or ECDLP performance claim.

Primary references: [Caminata–Gorla, Definition1.3](https://arxiv.org/html/2112.05579); [Phuong–Tran, Theorem2.7](https://arxiv.org/html/2211.13548); [Kosters–Yeo, binary trace falls](https://arxiv.org/html/1503.08001).
