# CT240 — all five reduced norm bases complete finite F103 source DLP, no priced speedup (2026-11-19)

**Hosted Python arithmetic screen completed; not cold native timing, not an ECDLP advantage, not signed PCC. [pcc:unverified].**

## Provenance and replay
- [Precommitted CT240 protocol](CT240_F103_REDUCED_NORM_LP_PROTOCOL.md) landed first at `1f479750238f9ba663c212f6a0855f9448774bbd`.
- [Producer script](ct240_f103_reduced_norm_lp.py) committed next at `ea6d6dbe333ebab5db3adb3109f4a99dc628b243` and bounded workflow then committed at exact run SHA **`8551564f73e8e8a43cd9229884a545c360ff7070`**.
- [Exact source hosted CI #44592120906](https://github.com/isogeny-labs/crypto/actions/runs/44592120906), job **135800592732**, completed **success**. One original output row for each of five registered bases; no grep echo doubles in this run.
- [Five original structured rows](ct240_f103_reduced_norm_lp_finite_panel.jsonl), directly frozen from the original decoded hosted log, not reconstructed estimates.
- Replay: `cd research/chained_jacobian_transfers_20261110/toy_dlp_ct204 && python3 ct240_f103_reduced_norm_lp.py`. It requires original CT223, CT237 and CT238 Python modules alongside it, uses assertions and prints structured rows.

## Fixed public toy inputs and exact results
E/F103: y²=x³+x+24, C/F103: v²=u⁹+u³+24, r=109, G=(1,51), Q=(42,71). For all 60 fixed offset families, the script enumerated exactly 6,540 candidate Cantor relations, accepted 589 raw split witnesses and checked **every** raw elliptic norm relation independently, including its compressed column identity. Four mutation controls (column corruption, RHS corruption, bad scalar, duplicate orbit symbol) were rejected.

| Base B | Excluded LP norm columns | Base rank | LP pivots / determined LP logs | Full / 1LP / 2LP accepted | Relation candidates to full rank | Target trials | Source scalar |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 12 | 10 | 12/12 | 10/10 | 5 / 11 / 15 | 589 | 27 | 73 |
| 16 | 6 | 16/16 | 6/6 | 8 / 11 / 7 | 381 | 27 | 73 |
| 18 | 4 | 18/18 | 4/4 | 14 / 10 / 2 | 381 | 27 | 73 |
| 20 | 2 | 20/20 | 2/2 | 19 / 7 / 1 | 381 | 27 | 73 |
| 22 | 0 | 22/22 | 0/0 | 27 / 0 / 0 | 381 | 27 | 73 |

**Scientifically relevant constraint:** the B12 solver reduces its dense factor-base rank to 12 but acquires 10 independently pivoted and recovered excluded classes. It therefore has **22 solved source norm variables in total**, not 12. All other successful arms also acquire a total of 22 base+LP pivots. This is a *finite observation*, not a general rank lower bound; no free work saving follows from counting only base pivots. B12 needs 589/381 = **1.546×** as many relation candidates as the full norm control. The B16/B18/B20 arms tie B22's *candidate* count but may differ in actual matrix/LP arithmetic cost.

All five arms independently verify each determined norm E factor logarithm by elliptic multiplication and the recovered source scalar in both E and the Jacobian pullback. The known-answer scalar 73 is used only for public test checks, not to construct relations or solve equations. The producer reuses CT237/CT238 algebra modules; external and independent native execution remain separate gates.

## Decision
Proceed only with [precommitted CT241 native paired-cost protocol](CT241_F103_NATIVE_REDUCED_NORM_LP_PROTOCOL.md) and the existing CT238 Rust arithmetic, requiring full cold setup, online LP/base elimination, target and verification, primitive/memory diagnostics and same-host BSGS. The new [CT241 exact-source native run](https://github.com/isogeny-labs/crypto/actions/runs/44592813955) was triggered, **but no result is inferred until compilation, all 15 rows and terminal status are inspected**. Strongest optimized rho, frontier IC, original authenticated 695/701 norm-point reuse physical C169/C13a/C13b/N2_actual/D costs, independently repeated native timing and signed PCC remain open.

No paid EKS/Spot/GPU, protected merge, artifact-auth bypass, or unsupported speedup claim. [pcc:unverified].
