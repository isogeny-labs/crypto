# Main-term threshold model for the Anthropic #k-SAT clique-sum recursion

Reimplementation of the threshold model of R. Majdoddin, *Lowered crossover points of
Anthropic's subexponential 3-SAT algorithm*, arXiv:2610.41622 (30 Oct 2026), Section 3,
used in `research/tensor_contraction_ecdlp_review_20261121/REVIEW.md` §5d to ask whether the
density-one #3-SAT crossover can be pushed below his 2^420.

- `threshold_model.py`: recursion (5) of Majdoddin / (40) of the Anthropic paper,
  admissibility (S1)–(S4), crude top (`q log2 K_L`) and the Lemma C top (Prop. 3.9).
  All arithmetic in log2 K. Overheads `oz` (coefficient of ζ in the per-level factor) and
  `oK` (coefficient of 1/K in the additive term) are parameters; the paper's accounting is
  `oz = oK = 3`.
- `sweep.py`: lever combinations at density one (first draft; the final table is in the note).

## Validation against Majdoddin's published values (density one, k = 3)

| Column | his value | this model | command |
|---|---|---|---|
| (P) paper's schedule, x0 = 18, q = 9 | 33858 | 33858.0 | `--paper --x0 18 --q 9` |
| (G) least admissible s, x0 = 18, q = 9 | 28761 | 28761.3 | `--x0 18 --q 9` |
| (X) x0 = 7 | 22264 | 22263.9 | `--x0 7 --q 9` |
| (T) tree base e0 = 0.2554, x0 = 8 | 5396 | 5397.4 | `--x0 8 --e0 0.2554 --q 9` |
| (U) exact U, bar 0.1321 (x0 re-optimised, two levels) | 1538 | 1555.4 (x0 = 10) | `--x0 10 --e0 0.2554 --q 9 --exactU` |
| Lemma C top, brute base, x0 = 7 (his text: 5037, q = 2.033) | 5037 | 5056.8 (q 2.032) | `--x0 7 --top lemmaC` |
| (U+C) all proved changes (his: 417, q = 2.41, two levels) | 417 | 413.2 (x0 = 11, q 2.39) | `--x0 11 --e0 0.2554 --exactU --top lemmaC` |

Per-level schedule for (X) matches his Table 10 (log2 K: 7, 38.5, 111, 263, 575, 1205, 2474;
ζ: 2^-4.5, 2^-6, 2^-7, 2^-8.5, 2^-9.5, 2^-11) once the slack grid extends below 2^-8.5 (the
text's "j ≤ 17" is narrower than the schedules he prints).

## Proved-by-reading overheads

Re-reading the inductive step of Theorem A(b) in the Anthropic paper:

- (36)/(41): `U ≤ (ρ + 1/g) N + 4c g^4 log(2λ)`; Lemma 8 gives `K^2 L ≤ U + 2ζN + K^2`. The paper
  writes `(ρ + 3ζ)N` using `1/g ≤ ζ` (S2). Keeping `1/g` exact: per-level factor `ρ + 2ζ + 1/g`,
  and `1/g < 2^-30` on every computed schedule. Model: `oz = 2`.
- Lemma 1/8 cost: an entry of a merged table is a sum over the settings of the group's nodes,
  `(2^P)^{|G_γ|}` terms. Case (i) has `|G_γ| ≤ 2g/K`, hence the paper's `2^{2gP/K} = 2^{2N/K}`;
  case (ii), the one the recursion uses, has `|G_γ| = g/K` exactly, so `2^{N/K}`. The third
  `N/K` in (44) comes from absorbing the pieces' `2^{cλ}` via `cλ ≤ N/K`; its true size is
  `c/K_{ℓ+1}^2 < 2^-60`. Model: `oK = 1`.

Result at density one (bar 0.1321, tree base, Lemma C top): `oz = oK = 3` gives 413; `oz = 2,
oK = 1` gives **384.6** (x0 = 10, two levels, log2 K_L = 160.3).

`python3 threshold_model.py --x0 10 --e0 0.2554 --exactU --top lemmaC --oz 2 --oK 1`

## Bar

The density-one bar 0.1321 per edge bit is Peng–Sheng–Xiao's exact deterministic #3-SAT
counter O*(1.4423^m) (IJCAI 2025, arXiv:2507.14504) over U = 4m edge bits. Their paper states
that the analysis of the 2010 Zhou–Yin–Zhou O(1.4142^m) #3-SAT algorithm "does not yield a
valid runtime bound", so the bar-0.125 rows of `sweep_results.txt` are a sensitivity check only.

## Quantile balancing (lever A, note §5e.1)

`qb_model.py` replaces Hoeffding max-load padding by `L = mu + d sigma` with the excess
bits of heavy pairs enumerated outside the clique call. Per level
`e_{l+1} = rho e_l + beta(e_l) sqrt(2 rho/s + rho/s^2) + 1/K_l`, `beta(e) = min_d [e d + (sqrt(1+d^2)-d)/2]`,
no (S1), `s >= 1` free. Results (Lemma C top, tree base, x0 optimised):

| density | Hoeffding (oz=2, oK=1) | quantile balancing |
|---|---|---|
| 1 | 384.6 | 323.5 |
| 4.26 | 9669.4 | 7524.6 |

`python3 qb_model.py` and `python3 qb_model.py --density 4.26`.
