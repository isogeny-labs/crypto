#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)

= Q5: the grevlex comparisons replayed with msolve (F4)

Run 2026-11-02, 18:20–18:31Z. Protocol `PROTOCOL_Q5_MSOLVE.md`, frozen at `233b8fa` before
any msolve data. Card CARD-isogeny-vs-mu-factor-base-20261102-3. The director's assessment:
executed as specified, and the relay was verbatim once the known output-order artifact of
`task_card.py` is taken into account.

*Instrument.* The engine is msolve 0.6.5 (Ubuntu `0.6.5-1build2`, binary SHA-256
`c2722288…`), run as `-v 2 -g 2 -t 1` with DRL order and a 120 s timeout. The counted unit is
`cells`: the sum, over F4 rounds, of matrix rows × columns. It is deterministic for a fixed
version and input. The systems and seeds are those of Q2 (targets 0–3) and Q4 (targets 4–7).
The Q4 candidates come from the frozen Buchberger selection, which replays byte for byte: its
`q4_candidates.csv` is identical to the Q4 run's. The run covered 6486 systems, with *0* gate
failures and *0* timeouts.

#table(columns: (auto, 1fr), stroke: 0.4pt,
  [*Q5a*], [*Presentation explains the cost under F4 as well, in 16/16 comparisons.* The
  ratio A2/B (Möbius-presented $mu_k$ over the Vélu chain) is 1.000 in every cell, with CIs
  inside 0.98–1.02. The F4 matrix sizes are identical.],
  [*Isogeny gap*], [The gap is smaller under F4. B/A1 is 1.32–1.43 under F4, against
  1.58–2.19 with Buchberger, so the pre-set 1.5× persistence threshold is met in 0/16.
  B2/B (the 2-torsion terminal) is 4.3–7.4. Dickson/A1 is 1.00.],
  [*Q5b*], [*No improvement, 0/16.* The grevlex-selected coordinates cost 0.95–1.59× native.
  The lex-optimal coordinates (C_sel_lex) are *worse* under F4, at 1.16–1.70×, and so are the
  S1-optimal ones (C_best, 1.59–1.69×). The inverse-variable form costs 3.2–3.5×.],
)

#figure(image("figures/q5_buchberger_vs_f4.svg", width: 100%), caption: [Grevlex ratios
per cell. Circles are Buchberger termops (Q2/Q4); squares are msolve F4 cells (Q5).
Values come from `q2_cells.csv`, `q4_cells.csv` and `q5_cells.csv`.])

*Reading.*
- *The obstruction story holds under F4.* The cost difference between $mu_k$ and isogeny
  chains is entirely a matter of presentation (A2 = B exactly), and coordinate changes don't
  help F4.
- *The size of the gap depends on the engine.* Under F4 the Vélu chain costs about 1.3–1.4×
  $mu_k$, not 2×.
- *The lex gains from Q4 don't carry over.* Those coordinates make F4 slower.

*Caveats.*
- *Launcher bug.* The launcher meant to write the msolve version string to `host.txt`, but
  `${Version}` was shell-expanded and the field came out empty. The binary SHA-256 is
  recorded and matches `0.6.5-1build2` on the run host.
- *Missing rows.* Each Vélu candidate has 378 rows instead of 384. At the (7681, 3) target
  where |F| = 0 there are no "dec" targets to draw, as in Q4.
- *Scope.* Toy primes and m = 2. This is not an ECDLP cost claim.
