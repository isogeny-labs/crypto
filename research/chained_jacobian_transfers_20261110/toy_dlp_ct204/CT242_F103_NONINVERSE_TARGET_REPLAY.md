# CT242 — public Q37/Q51 target controls exclude the inverse-three coincidence (2026-11-19)

**Result: exact finite F103 public-target recovery on four registered source model cases; independent public-point norm identity verified before scalar reconstruction. No new native timing or PCC. [pcc:unverified].**

## Motivation
In E(F103) of order 109, the degree-three source norm needs the inverse factor `3^{-1} mod 109 = 73`. The CT238–241 original known-answer target was also `[73]G`. This coincidence could permit a broken constant-73 decoder to appear correct. The CT242 control targets use Q37=(21,6) and Q51=(73,17), neither with logarithm 73.

## Source and checks
- Precommitted exact target protocol: [CT242_NONINVERSE_F103_TARGET_PROTOCOL.md](CT242_NONINVERSE_F103_TARGET_PROTOCOL.md) at `7176fb1b244b4c404e42ae5f7744ea21876818e0`, before producer.
- Producer [ct242_f103_noninverse_targets.py](ct242_f103_noninverse_targets.py) at `3c5bf31c9af2209abf42c56c8de83b883a37b955` reuses CT240's public deterministic full Cantor relation stream, solves B12 and B22 norm bases with exact excluded LP elimination, verifies every known base/LP E norm-point logarithm.
- [Hosted exact-source CI #44594695563](https://github.com/isogeny-labs/crypto/actions/runs/44594695563) job `135809715755`, head `2d590933c525fee279e32245db47b715be73cd24`, **success**, exactly four native log records from the original Python arithmetic (not native Rust).
- The **point-only** norm identity checked for each accepted target relation is `sum_i c_i P_i = [3]Q + [3a]G`. It uses public Q, never log(Q). The candidate scalar is reconstructed as `k = inverse3 * sum_i c_i log_G(P_i) - a (mod 109)`. Both `[k]G=Q` and `[k]phi*(G)=phi*(Q)` are asserted *before* the known-answer comparisons k=37 or51. A hardcoded inverse-three answer 73 would fail.
- Original four exact rows frozen in [ct242_f103_noninverse_panel.jsonl](ct242_f103_noninverse_panel.jsonl). Actual output:

| Reduced/full norm base | Public Q input | Recovered log | Target candidates | Norm target identity checks | Factor relation candidates |
|---:|---|---:|---:|---:|---:|
| 12 | Q37=(21,6) | 37 | 19 | 1 | 589 |
| 12 | Q51=(73,17) | 51 | 5 | 1 | 589 |
| 22 | Q37=(21,6) | 37 | 19 | 1 | 381 |
| 22 | Q51=(73,17) | 51 | 5 | 1 | 381 |

The 37/51 constants appear only as final post-recovery known-answer assertions. The normal Python CI invocation is `cd research/chained_jacobian_transfers_20261110/toy_dlp_ct204 && python3 ct242_f103_noninverse_targets.py`.

## Scientific status and next decision
These controls rule out **the specific fixed inverse3=73 decoder false-positive** for this finite code path. They do not prove that every possible decoder bug is absent. They reuse CT240/CT237 Python arithmetic, so there is no independent native Rust execution, no native cold timing, and no PCC. The precommitted CT241 Rust 15-paired cold single-target experiment is separately [running](https://github.com/isogeny-labs/crypto/actions/runs/44592813955) at the last inspection; do not infer its outcome from this success. The authenticated original 695/701 norm-reuse fixture and physical C169/C13a/C13b/N2_actual/D remain independently gated. No paid compute, merge, or speedup claim.
