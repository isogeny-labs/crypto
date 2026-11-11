#set page(margin: 2cm)
#set text(size: 10pt)
= PKM Kummer tower on primes matched to the tower depth
2026-11-06 · `examples/pkm_tower_pilot.rs`, m = 2, t = 5–9, p = 2,081 … 524,801

*Question.* Does the F4 solving degree of the prime-field PKM tower system stay flat when the field grows with the tower depth? All earlier runs used one fixed prime.

*Answer.* No. The degree is 4, 5, 5, 5, 5 for t = 5–9, the same as at the fixed prime. The random-coefficient null gives the same degree at every depth. The slope is +0.2 per step, so the prime-field lead stays closed. These are unverified assertions.

#figure(image("fig_matched.svg", width: 95%), caption: [Solving degree against tower depth: matched-prime tower (dots), null (crosses), fixed-prime staircase (grey).])

*Not claimed.* No cost, no rho comparison. m = 2 only: the deciding m ≥ 4 cell needs a large host.
