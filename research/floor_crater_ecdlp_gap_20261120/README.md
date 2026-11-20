# Floor vs crater ECDLP over a large prime conductor gap

Companion to `PROTOCOL.md`. Separates Lean structural bounds from
measured solver ratios and counted bridge cost. `[pcc:unverified]`

Draft PR: https://github.com/isogeny-labs/crypto/pull/4841

Claims (standing unverified until CI signs on `main`):
- theorem PCC-20261120-a79787 `[pcc:unverified]`
- measurement PCC-20261120-3ec0e9 `[pcc:unverified]`

## Verdict `[pcc:unverified]`

1. **Lean.** Always-correct solvers on crater (`f=1`) and floor (`f=ℓ`)
   differ by at most twice the degree-`ℓ` isogeny evaluation cost; a
   conductor valuation gap forces a step with `ℓ ≤ d`
   (`FloorCraterEcdlp`, proof-gate PASS, 7 theorems; PCC-20261120-a79787).
2. **Solvers.** Matched ecbench sessions (2026-10 + this study's rho
   retest) show **no reproducible ECDLP cost difference** for implemented
   methods: floor/crater `S` intervals include 1 (PCC-20261120-3ec0e9).
3. **Bridge.** Known-kernel Vélu costs `Θ(ℓ)` field multiplications; on
   the certified `-19` pairs this is already ~3.7× `√N`. Transfer via the
   isogeny is therefore more expensive than solving either side directly
   at these sizes (same measurement claim).

This is **not** a claim that floor ECDLP is asymptotically harder than
crater ECDLP for Pollard rho. The difference that is forced is the
**bridge ceiling**, not a solver-side conductor effect.

## Lean proof map

Project: `lean/` (path-requires `conductor_prime_gap_bounds` → CryptoLean /
Mathlib / TransferBound / DeepGap). Receipt:
`receipts/research_floor_crater_ecdlp_gap_20261120_lean/` (`PASS`, 7 thm,
0 trusted). Axioms: `[propext, Classical.choice, Quot.sound]`.

| Result | Declaration | Kind |
| --- | --- | --- |
| Crater/floor mutual degree-`ℓ` step | `floor_crater_valid_step` | Law |
| Valuation gap `v_ℓ(1) ≠ v_ℓ(ℓ)` | `floor_crater_valuation_gap` | Law |
| Gap forces large step | `floor_crater_forces_large_step` | Law |
| Always-correct DLP ceiling | `floor_crater_dlp_within_isogeny_cost` | Law |
| Transfer overhead `+ 2·C` | `floor_crater_transfer_overhead` | Law |
| Bridge overhead ≥ `2ℓ` under Vélu hyp. | `floor_crater_bridge_overhead_ge` | Law |
| Hasse / linear-Vélu vs rho scale | `floor_crater_velu_vs_rho` | Law |

Unformalized bridges (documented, not axiomatized): `End(E) ≃ O_f`;
Robert Remark 4.5.

## Experiments

### A. Known-kernel bridge cost

`cargo +1.99.1 run --example floor_crater_bridge_cost --release`
→ `results/bridge_cost.md` / `bridge_cost.json`.

| ℓ | `√N` | Vélu `M` | `C_bridge / √N` |
|---:|---:|---:|---:|
| 263 | 585.5 | 2134 | 3.645 |
| 8209 | 17891.1 | 65710 | 3.673 |
| 131101 | 285728.0 | 1048858 | 3.671 |

Kernel search / `Φ_ℓ` not included (lower bound once a kernel is known).

### B. Solver crater vs floor (prior + retest)

Prior: `research/conductor_gap_cm_pairs_20261010/results/crater_vs_floor.md`
(five sessions; no reproducible conductor effect under the PROTOCOL
decision rule, pending rho retest).

**Rho retest (this study):** `ℓ = 8209`, measurement seed `20261120`,
16 targets/curve, L0, 768/768 records with audit OK
(`sessions/ell8209_q29_rho_retest_20261120/`) `[pcc:unverified]`.

| arm | crater wl | floor wl | floor/crater | 95% bootstrap | sees gap? |
|---|---:|---:|---:|---|---|
| rho-neg | 16 | 48 | 1.055 | [0.943, 1.198] | no |
| rho-neg-aa | 16 | 48 | 1.055 | [0.943, 1.198] | no |

The unexplained 2026-10 ρ interval 1.22 [1.005, 1.46] **did not reproduce**
under a fresh measurement seed.

## lean-instance-gate (summary)

- **Structure above:** `TransferBound.Isogeny` / `Solver`,
  `ConductorOrthogonality.ValidConductorStep` / `ConductorWalk`,
  `HasseTransferCeiling.linear_velu_gap_is_rho_scale`.
- **Theorem used as stated:** `within_isogeny_cost`, `transfer_cost`,
  `conductor_gap_forces_large_step`.
- **Instance:** crater `f=1`, floor `f=ℓ`, mutual degree-`ℓ` step.
- **Missing / open:** `End(E) ≃ O_f`; poly(log ℓ) navigation.

## Literature & KB

KB revision `d47bbc4827ae4884b5df841f6110b2f2` (ready). Sources:
`source:61978a6dbd7332419587ebcddfa6e562` (ECC2K-130 conductor gap);
`source:0a8d21e5814d34be9ffe9c1ff14fabdb` (CryptoLean Conductor);
prior study `research/conductor_gap_cm_pairs_20261010/`.

## Verification

```bash
export PATH="$HOME/.elan/bin:/opt/homebrew/bin:$PATH"
tools/lean/proof-gate.sh --out research/floor_crater_ecdlp_gap_20261120/receipts \
  research/floor_crater_ecdlp_gap_20261120/lean
ecbench verify --dir research/floor_crater_ecdlp_gap_20261120/sessions/ell8209_q29_rho_retest_20261120
```
