# PKM fall-degree study on RunPod, 2026-11-15: report

Every number is `[pcc:unverified]`. None of this is an index-calculus cost, a rho comparison, or an improvement claim.

## What ran (reserved 16-vCPU RunPod pods; Amendments 15–20; git storage, Amendment 17)

| job | result | details |
|:--|:--|:--|
| `iso-a`: 2-isogeny arm, `P1`, `t = 1..13` | done | `results/ISO/iso-a/` |
| `iso-14`: 2-isogeny, `P1`, `t = 14` | done | `results/ISO/iso-14/` |
| `pfd-x-t13-k0-c6-rp`: Kummer `t = 13`, draw 0, cap 6 (attempt 4) | done | `results/KUMMER/pfd-x-t13-k0-c6-rp/` |
| `pfd-b-rp-1`, `pfd-b-rp-2`: stage B (attempt 3, as two jobs) | done | `results/KUMMER/pfd-b-rp-{1,2}/` |
| `pfd-b-g3b`: stage B G3 rerun (Amendment 20) | done | `results/KUMMER/pfd-b-g3b/` |
| local `m = 4` smoke test (decision 17; not a registered row) | SIGKILL at about 14 GB, no row | `results/SMOKE/` |

The runner's guards passed on every pod. Every fetch matched the pod's own SHA-256 manifest. Every pod has been deleted.
Spend (derived from pod-hours at US$1.04/h): about US$17.

## Readings

1. **2-isogeny arm at `P1`** (`results/ISO/SCORE/READING.md`).
   - Gates: G1, on-pod G-AUX, G2 0/20, G3 0/40; 14/14 curves recounted; 28/28 draws certified.
   - **P-ISO-1 holds. P-ISO-2 grows** (`d_F` = 6 exact at `t = 10, 11, 12`; ≥ 6 at 13). **P-ISO-3 grows** (≥ 6 at 14).
   - The isogeny ladder leaves 5 at `t = 10`, the same `t` as Kummer, and equals Kummer from `t = 6` on. At `t = 2..5` it is
     one degree higher.
   - **On this measure, the low-degree 2-isogeny maps give no solving-degree advantage over the squaring maps.**
2. **Kummer `t = 13`** (`results/KUMMER/pfd-x-t13-k0-c6-rp/READING_T13.md`).
   - Draw 0, cap 6 refutes, so `d_F = 6`. Median 6 = `c*(13)`: **"holds at t = 13"** (it was "undecided").
   - Like `t = 12`, this cell does not separate C-step from "flat at 6". The decisive cell, `t = 14` (predicting 7), is not run.
3. **Stage B** (`results/KUMMER/STAGE_B_READING.md`).
   - Gates: G2 0/41; G3 0/123 (rerun under Amendment 20 after a tool crash).
   - `m = 3` at `P1` grows 5 → 7, and **the null reads identically** (shape-driven).
   - `m ≥ 4` ladders are censored ("mixed" or "insufficient").
   - At `P0` the isogeny tower is never below Kummer.

## Instrument findings recorded along the way

- F4's `muladds` counter depends on `RAYON_NUM_THREADS` (`gaux/GAUX.md`).
- The frozen `gate3` has a temporary-file name collision between tower and null rows under `--jobs > 1` (Amendment 20).
- Two jobs write `CERT-m2-iso-p1.jsonl`. Scoring them together needs distinct names (`results/ISO/SCORE/READING.md`).

## Open (owner)

- Kummer `t = 14` cap 6, the decisive C-step cell.
- The 8 registered `m = 4` jobs, under the US$60 ceiling (decision 16).
- An isogeny null arm, to split "grows" into Semaev-specific and tower-ring effects.
