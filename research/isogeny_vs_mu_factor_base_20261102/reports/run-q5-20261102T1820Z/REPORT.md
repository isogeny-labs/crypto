# Q5: the grevlex comparisons replayed with msolve (F4)

PDF: [`REPORT.pdf`](REPORT.pdf); source [`report.typ`](report.typ); figure
[`figures/q5_buchberger_vs_f4.svg`](figures/q5_buchberger_vs_f4.svg) (drawn by `../../plot_q5.py`).

- **Run.** Protocol `PROTOCOL_Q5_MSOLVE.md`, frozen at `233b8fa`; card CARD-…-3; 690 s.
  The director's assessment: executed as specified, and the relay was verbatim (`cards/`).
- **Engine.** msolve 0.6.5 (Ubuntu `0.6.5-1build2`, binary SHA-256 `c2722288…`).
- **Counted unit.** `cells`, the sum of F4 matrix sizes (rows × columns) over rounds.
- **Scale and checks.** 6486 systems, 0 gate failures, 0 timeouts. The frozen Q4
  selection replayed byte for byte.

| | Result (grevlex, 16 comparisons each) |
|---|---|
| Q5a | **Presentation explains the cost under F4, 16/16.** A2/B = 1.000 in every cell: the F4 matrix sizes are identical. |
| Isogeny gap | B/A1 = 1.32–1.43 under F4, against 1.58–2.19 with Buchberger; the 1.5× persistence threshold is met in 0/16. B2/B = 4.3–7.4. Dickson/A1 = 1.00. |
| Q5b | **No improvement, 0/16.** The grevlex-selected coordinates cost 0.95–1.59×. The lex-optimal ones cost 1.16–1.70× (worse), the S1-optimal ones 1.59–1.69×, and the inverse-variable form 3.2–3.5×. |

## Caveats

- **Launcher bug.** The msolve version field in `host.txt` is empty, because `${Version}` was
  shell-expanded. The binary hash is recorded.
- **Missing rows.** Each Vélu candidate has 378 rows instead of 384, because of the
  |F| = 0 target at (7681, 3).
