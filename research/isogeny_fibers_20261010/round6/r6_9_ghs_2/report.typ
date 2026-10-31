#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= R6.9: GHS phase two

Toy binary fields only (N ≤ 18, q ≤ 32). This report makes no claim about deployed curves. Full tables, commands and caveats are in `RESULTS.md`.

== Verdict
- *Walk (task 1).* On average, reaching an LG5 curve costs 0.28–0.67 × 2·(class size)^(1/2) isogeny evaluations (measured).
  - The share of searches above that bound rises from 0 % at N = 12 to 24 % at N = 18, and the worst search is 4.07× the bound.
  - The log-log slope of evaluations against class size is 0.84, not 0.5.
  - Evaluations per curve of class size stay flat (0.048–0.068), so the falsifier "cost grows with N beyond the class size" does not fire.
  - 18–36 % of strong starts cannot reach any target with ℓ ≤ 7.
- *Relation rate (task 2).* The measured fully-split rate is within 0.88–3.22× of the exact split-divisor fraction in all 34 runs.
  - Against 1/g! it is off by more than 10× in 6 runs. At the smallest weak instance (N = 12, (l, n) = (2, 6), g = 8 over GF(4)) it is off by 585–858×, so the falsifier fires against the 1/g! model.
- *Library limit.* The library has binary Cantor arithmetic but no smooth GHS descended curve for m ≥ 2. Task 2 therefore uses random genus-g curves over GF(2^l) as a stand-in.

== Walk cost vs class size
#figure(image("fig1_walk_cost_vs_class_size.svg", width: 90%), caption: [Isogeny evaluations per search (measured) against class size; the line is 2·√s.])

== Relation rate
#figure(image("fig2_relation_rate_measured_vs_model.svg", width: 90%), caption: [Measured smooth rate against the exact split fraction (filled) and 1/g! (open); lines 1×, 4×, 10×.])
