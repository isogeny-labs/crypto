# Simplicity-factorization replay receipt

15 November 2026 UTC. Frozen expectation and correction are in `../../PROTOCOL.md`, Amendments B and C. Toolchain: SageMath 10.10.rc0. Command from repository root:

`sage research/ghs_n7_base_change_20261115/src/simplicity_replay.sage > research/ghs_n7_base_change_20261115/reports/simplicity-20261115/result.txt 2>&1`

The corrected command exited zero. It checked both trace signs at s=1 through 10. In both arms, `A0 × F_(2^s)` has irreducible degree-14 polynomial for s=1–6 and 8–10, and degree-two factor with multiplicity seven at s=7. At s=7 the *different* restriction `Res_(F_(2^49)/F_(2^7)) E_7` factors as degrees 2 and 12. The original check incorrectly asserted those two polynomials were equal and exited one; the unmodified failure output is preserved in `failed-initial.txt`. The general theorem does not depend on the finite range.

SHA-256 digests at this receipt:

- `src/simplicity_replay.sage`: `c2beac34724315e51c89405074163aef4d3e1fa675e80ed4209cd209a5914b60`
- `reports/simplicity-20261115/failed-initial.txt`: `b93df7a16557ae80598942f389c09d952dffba2c3f0c7d1035510de5fb11dcb5`
- `reports/simplicity-20261115/result.txt`: `30560671c4f2f2df1ee298e0e163c171c749e6d51581ca3ef7f482ef2cd49b32`

This is a local finite algebra check. The proof of the all-s statement is `../../GENUS_FLOOR.md` and remains paper-level pending independent review. [pcc:unverified]
