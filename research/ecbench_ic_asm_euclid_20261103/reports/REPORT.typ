#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.5pt)
#set par(justify: true)

= ecbench index calculus: Euclid inversion in assembly, 32-bit product by one `div`

*Date:* 2026-11-03. *Repository:* `aburan28/crypto`, study `research/ecbench_ic_asm_euclid_20261103/`. *Class:* engineering. *Instrument:* Callgrind instruction counts (deterministic); no wall-time claim. Every child's counted record (recovered logarithm, GAE per phase, counters, unpriced list) is identical to `main` c13f8397ff's; only wall-clock, placement and scheduler fields differ. *Target:* half the instructions per GAE of the index calculus. *Outcome:* −11.2% on the default arm, −6.9% on the existing batched-inversion arm (0.741 of the default arm on `main`); half is not reached.

== What changed

The compiler had folded round 3's unrolled 32-bit Euclid loop back into a 13-instruction swapping loop with a run-time width test before each division; it is now written in x86-64 assembly, 8 instructions a step, the same quotients, remainders and sign (portable loop kept for other targets). Products below $2^32$ take one 64-by-32-bit `div` (the quotient is below the modulus), the same remainder. A new test checks both against the portable forms on ten edge moduli.

#image("../figures/ir_per_gae.svg", width: 100%)

== Instructions

#table(
  columns: (auto, auto, auto, auto, auto),
  [*child*], [*GAE*], [*main Ir*], [*this Ir*], [*change*],
  [32-bit `ic.pipeline` N=1024, default oracle], [1,446,809], [776,953,078], [689,798,132], [−11.2%],
  [32-bit `ic.pipeline` N=1024, `mitm-prime-batch`], [1,446,809], [618,139,505], [575,394,244], [−6.9%],
  [20-bit `rho.negation` (solve)], [1,706], [1,054,694], [967,499], [−8.3%],
  [20-bit `ic.pipeline` N=32 (solve)], [14,101], [12,399,287], [11,420,269], [−7.9%],
  [20-bit `ic.pipeline` N=45 (solve)], [10,632], [8,547,378], [7,851,935], [−8.1%],
)

Not kept: a relation guard sized from `max_trials` (better at 32 bits, worse at 20); `walk-prime-fast` (971.9M Ir). What remains on the batched arm: the table build (about 199 Ir a pair with its inversion), one Euclid inversion per walk step, the relation collector, the guard and the measurement trace; halving would need about 190M Ir more from parts of tens of millions each, and a batched walk would change trajectories (algorithmic).
