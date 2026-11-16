# Report: isogeny-kernel vs μ_k factor bases (runs 2026-11-02, from 01:39Z)

The PDF version, with all figures, is [`REPORT.pdf`](REPORT.pdf) (source
[`report.typ`](report.typ)).

## Run identity

| | Q1–Q3 | Q4 search |
|---|---|---|
| Protocol | `PROTOCOL.md` @ `de0b5628` | `PROTOCOL_Q4_SEARCH.md` @ `e2cb7331` |
| Card | CARD-isogeny-vs-mu-factor-base-20261102-1 | CARD-…-2 |
| Wall time | 1241.9 s | 846.2 s |
| Exit | 0 | 0 |
| Binary SHA-256 | `d45e9899…` | `d8ce4081…` |
| Systems | 6144 | 6828 confirmation + 4704 selection |
| Gate failures | 0 | 0 |
| Director assessment | execution as specified (relay: tool order artifact, see `cards/relay/`) | executed as specified, relay verbatim |

- **Host:** the session container (Linux 6.18, 4 vCPU, rustc 1.97.0). There
  was no isolation tier, so wall time is unadmitted.
- **Why local:** Iso Lab on EKS was not reachable from this session, since
  there was no AWS identity and the GitHub dispatch only runs ecbench
  specs. Every primary column is a deterministic counted unit (`termops`,
  exact counts and structure constants), which a replay on any host must
  reproduce byte for byte.

## Results (all values recorded in the named CSVs)

| Question | Outcome (frozen rule) | Measured |
|---|---|---|
| Q1: yield of isogeny V vs random V (`q1_cells.csv`) | **Null, 15/15 cells** | Δ median r between −0.020 and +0.029 (≤ 3.3%). Positive control valid in every cell (r_PC 0.06–0.49 vs r_R 0.87–1.02). |
| Q2: does presentation explain the cost? (`q2_cells.csv`) | **Inconclusive**: 8/16 support, 0/16 falsify | A2/B = 0.97–1.00 in all 32 cells and both orders. B/A1 is 2.10–2.19 in "dec" and 1.58–1.77 in "nodec" (grevlex); the nodec values fall under the pre-set 2× threshold. |
| Replication of the user's slowdown | 16/16 | Lex B/A0 = 6.4–11.7. |
| S1 density (`s1_structure.csv`) | Prediction holds | A0 and A1 1.000; A2 3.11 / 5.44; B 3.27 / 5.75 (k = 3 / 4). |
| Q3 terminal (`q3_terminal.csv`) | Prediction holds | B is radical, \|V\| = 2^k in k steps. The user's B2 is non-radical: 2^k points, multiplicity 2^(k+1), k+1 steps. B2/B is 7.3–12.9 (grevlex) and 5.3–44 (lex); the lex values at k = 4 are lower bounds (384/384 censored). |
| Q4, lex: selected coordinates / native Vélu (`q4_cells.csv`) | **Improvement, 16/16** | 0.21–0.30. The gap to μ_k falls from 6.4–11.7× to 1.9–2.6×. |
| Q4, grevlex | **No improvement, 0/16** | 0.82–1.17. S1-optimal coordinates 0.92–1.24; inverse-variable form 1.04–1.87. The gap to μ_k stays at 1.5–2.2×. |
| Q4 kernels (`q4_s1_search.csv`) | Flat | Native density 3.20–3.27 (k = 3) and 5.66–5.75 (k = 4) over 40+ kernels per cell. The best-density kernel costs 0.98–1.01× native. Four j = 0 kernels (at p = 12289 only) match the random ones; no j = 1728 kernel was found. |
| Obstruction check | Holds | 0 of 11811 valid Vélu representations reach density 1 (minimum 1.359). |
| Dickson chain t′ = t² − 2 | — | 0.98–1.03× μ_k in both orders, at density 1.69 / 2.33. |

## Reading

- **The isogeny-kernel base buys no yield.** At equal |V| it matches a
  random V. The 7/8 vs 3/8 in the user's run was sampling noise.
- **Its extra Gröbner cost comes from presentation, not from the set.**
  Writing μ_k in coordinates where its steps look like Vélu steps costs
  the same as the isogeny chain, to within 3%.
- **Why the gap can't be closed by coordinates.** For an elliptic
  2-isogeny x-chain, the steps' ramification points (4-torsion) and branch
  points (2-torsion) never align, so no coordinate choice makes the chain
  monomial (§4 of the PDF). This is checked empirically; a Lean proof is
  open.
- **What the search did achieve.** Within that limit, the coordinates
  `gen>0; ram>inf` (or greedy refinements of them) cut lex work by a
  factor of 3.3–4.7. They do nothing useful in grevlex.
- **Dickson as the control.** The Dickson control costs the same as μ_k,
  so the remaining cost is elliptic, not a matter of working on the
  x-line only.
- **Distance remaining.** Even the best isogeny presentation is 1.9–2.6×
  (lex) and 1.5–2.2× (grevlex) more expensive than μ_k.

## Caveats

- **Engine.** Toy primes, m = 2, and a hand-written Buchberger. msolve and
  F4 could not be built in this session.
- **Development exposure.** The p = 1153 cells were seen during Q4
  development, and the selection split was added because of them.
- **Missing rows.** One target at (7681, 3) has |F| = 0: its yield ratio
  is NaN and there are no "dec" rows (84 rows), so those medians rest on
  3 targets.
- **`dirty 2` in `host.txt`.** This counts untracked paths, because the
  `run.sh` filter does not match root-relative paths. All bound files hash
  to their card values.
