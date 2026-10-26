#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#set par(justify: true)
#show raw: set text(size: 8pt)

= Hybrid guess-and-determine at (m, ℓ) = (3, 7): results

Study `research/dreg_hybrid_guess_20261024/`, pre-registered with four dated amendments. Class: stage diagnostic with a derived row; no `S`, no speed claim, no row beats rho. Host: shared 4-core 15 GB VM, not isolated.

== Registered verdict

- *V-small.* On 10 of 10 unsatisfiable draws (8 old, 2 fresh) all 2^k branches refute at degree 4 for some k\* ≤ 2 (five draws k\* = 1, five k\* = 2).
- *Cheaper than the projected degree-5 closure* on every draw, at every exponent e and §5a weight g: 116 to 260 times at the e = 3 floor, 1,060 to 3,400 times at e = 4.1–4.25.
- *m = 4 row (derived): not improved at the exponent level.* Registered rule needs the S_lin row below +10.32 bits; with κ = 2 it is +58.27. Only the constant-k reading gives +8.43 bits (degree 4 floor +7.62, degree 5 +10.32). Constant versus growing k is unresolved; ℓ = 8 did not run.
- *Controls:* planted solutions survive (no solution-bearing branch refuted; the empty branches did). Placebo and A/A reproduce the records (columns, rank, outcome exact; word ops within 0.007 %). Strict reading: the registered product-count equality failed at 0.3–0.7 %, so the study is formally uninformative; amended reading (2 %) passes.

#figure(image("reports/hybrid.svg", width: 100%),
  caption: [Left: hybrid word operations at k\* (bars, measured) against the degree-5 closure projected from each draw's own unspecialised degree-4 cost (ticks, extrapolated). Right: implied m = 4 row, bits over rho at w = 1 (derived). Source: `plot.py` from `summary.json` and `runs/model-output.txt`.])

== Per draw

#table(columns: 8, align: (left, right, right, left, left, right, right, right),
  [cell], [draw], [k\*], [k = 1], [k = 2], [ops at k\*], [with failed], [RSS GB],
  [(19,7)], [0], [2], [fails], [4 of 4], [9.71e11], [1.50e12], [2.9],
  [(19,7)], [3], [1], [2 of 2], [4 of 4 probe], [5.34e11], [5.34e11], [3.1],
  [(19,7)], [5], [2], [fails], [4 of 4], [9.62e11], [1.54e12], [2.8],
  [(19,7)], [7], [1], [2 of 2], [not run], [5.56e11], [5.56e11], [6.0],
  [(21,7)], [0], [1], [2 of 2], [not run], [9.36e11], [9.36e11], [5.0],
  [(21,7)], [2], [1], [2 of 2], [not run], [8.84e11], [8.84e11], [4.9],
  [(21,7)], [3], [2], [fails], [4 of 4], [1.54e12], [2.56e12], [6.2],
  [(21,7)], [6], [1], [2 of 2], [not run], [1.02e12], [1.02e12], [5.1],
  [fresh (19,7)], [0], [2], [fails], [4 of 4], [8.45e11], [1.10e12], [3.0],
  [fresh (19,7)], [1], [2], [fails], [4 of 4], [9.14e11], [1.44e12], [2.7],
)

Probe, (19,7) draw 3, k = 1, 2, 3: total 5.34e11, 8.11e11, 1.022e12 word ops (x1.52, x1.26 per guess; about 0.47 bit a guess; one draw).

== Implied m = 4 row (derived, w = 1, bits over rho)

#table(columns: 3, align: (left, right, right),
  [scenario], [κ = 1], [κ = 2],
  [degree 4, k = 0 (floor)], [+7.62], [+7.62],
  [degree 5, k = 0], [+10.32], [+10.32],
  [S_const, k = κ], [+7.95], [+8.43],
  [S_lin, k = κ(ℓ − 6)], [+30.72], [+58.27],
  [S_linm, k = κ m (ℓ − 6)], [+84.82], [+84.82],
)

§5a band: rho charged 0, 0.1, 1 unpriced operation per step moves S_const (κ = 2) to +8.43, +8.29, +7.43; the ranking against degree 5 is the same at every weight. Units are unconverted touches; log2 K of 7.4 to 8.4 bits would close the gap, K unmeasured.

== Not run

(4, 7) infeasible (N ≈ 84, over 1 TB); E8 (3, 22, 8), fresh (21,7) draws and most probes did not run (lock contention).

== Disclosures

- Runs outside the heavy-compute lock: the 21 branches in `runs/UNLOCKED.txt` (2 threads, at most 3.1 GB), 08:45 to 10:52 UTC; locked branches in that window overlapped them. No wall time here is comparable across runs; word-op counts are unaffected.
- No shared files changed. New: `examples/dreg_hybrid.rs`.
