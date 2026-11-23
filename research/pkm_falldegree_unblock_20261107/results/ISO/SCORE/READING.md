# The 2-isogeny arm at `P1`: readings (Amendments 16–19)

Derived by `iso/score_iso.py` (the bundle pinned in Amendment 16/19) from the manifest-checked rows of jobs `iso-a` (fetch
`20261115T0618Z`) and `iso-14` (fetch `20261115T0515Z`). The input hashes are in `inputs.sha256`; the scorer output is
`score_iso.json`. Every number is `[pcc:unverified]`.

## Gates (all pass)

- **G1:** pass on both pods.
- **G-AUX on each pod:** "pass 16 draws, 0 mismatched".
- **G2:** 20 checked, 0 violations.
- **G3:** 40 capped runs, 0 disagreements (`iso-a`).
- **Independent naive recount** of every selected auxiliary curve: 14 curves (`t = 1..14`), 0 mismatched.
- **Certificates:** all 28 draws certified inconsistent (0 `V × V` solutions); 0 decomposable.
- **Scorer:** no conflicts, no STOP, exit 0.

**Process note.** Both jobs write `CERT-m2-iso-p1.jsonl`. On the first scoring pass, copying both into one directory
overwrote `iso-a`'s certificates, so `t = 13` read "undecided". The final pass keeps them under distinct names
(`CERT-m2-iso-p1.jsonl` for `iso-a`, `iso14.cert.jsonl` for `iso-14`); `inputs.sha256` lists both.

## Medians of `d_F` (lower median of 2 draws), isogeny `P1`, and Kummer `P1` beside it (admitted)

| `t` | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 |
|:--|--|--|--|--|--|--|--|--|--|--|--|--|--|--|
| 2-isogeny `P1` | 3 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 6 | 6 | 6 | ≥6 | ≥6 |
| draws with `d_F ≥ 6` | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 | 2 | 2 | 2 | 2 |
| Kummer `P1` (stage A, `t = 12` bracket) | 3 | 4 | 4 | 4 | 4 | 5 | 5 | 5 | 5 | 6 | 6 | 6 | ≥6 | ≥6 |

Per-draw sources are in `score_iso.json`:
- `t = 10`: closure exact 6, and F4 refutes at `D = 6`.
- `t = 11, 12`: closure `≥ 6` (cmax 5), cap 5 does not refute, and F4 refutes at 6, so exact 6.
- `t = 13, 14`: closure `≥ 6`, and cap 5 does not refute; F4 stopped at its 1,800 s budget, so there is no upper bound.

## Readings (rules frozen in Amendments 16 and 19)

- **P-ISO-1: holds.** All eight medians at `t = 1..8` are exact and equal the `P0` values (3, 5, 5, 5, 5, 5, 5, 5).
- **P-ISO-2: grows.** Per `t`: 9 "flat at 5"; 10, 11, 12 "grows" (exact 6); 13 "grows" (`≥ 6`, draws certified). The range reading
  is "grows". The prediction "flat at 5 at `t = 9..13`" is **refuted**.
- **P-ISO-3 (`t = 14`): grows** (`≥ 6`).
- **§6 as written:** H-LFD reads "`d_F` grows" (3 to 6).

## Comparison with Kummer (descriptive only; two draws per cell, no test)

- The isogeny ladder leaves 5 at `t = 10`, the same `t` as the Kummer ladder.
- From `t = 6` on, the isogeny medians equal Kummer's.
- At `t = 2..5` the isogeny median is one degree higher (5 against 4).
- On this measure, the solving degree of the summation system, the 2-isogeny tower (Vélu `L_j`) gives no advantage over the
  squaring tower (`μ_k`) at `P1`.
- This is not an index-calculus cost comparison and not an improvement claim. Relation cost, the auxiliary-curve search and
  matched rho are not measured.

## Not done

- The exact `d_F` at `t = 13` and `t = 14` would need the Amendment 7 B bracket at cap 6. No reading needs it: P-ISO-2 and
  P-ISO-3 are decided.
- No isogeny null arm (E4.9). So "grows" is not split into a Semaev-specific effect and a tower-ring effect.
