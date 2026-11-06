#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
= Lead R5.4: Conjecture 10 over prime fields, and the oracle-dependence of the saturation law
_2026-10-12. Theory round with native sanity checks (`examples/r5_4_theory_checks.rs`). Full proofs in `RESULTS.md`; toy instances only; no sub-rho claim; no claim about any deployed curve._

== Verdict
*(i) Conjecture 10.* Over $FF_p$ every known attack family (generic/rho with automorphisms, MOV/Frey--Rück, anomalous, Weil descent/GHS, summation-polynomial index calculus on curve-independent x-sets) has a cost equal to a function of $(p, t)$ times a member factor bounded by a constant: $sqrt(3)$ for rho (unit groups of size $<= 6$), 1 for pairings and anomalous attacks, not applicable for descent, $<= 6 dot h$ for index calculus (heuristic). *Theorem 4:* if two members' endomorphism rings differ only at primes $<= Delta$, DLP transfers between them at per-instance cost $"poly"(Delta, log p)$ (vertical part unconditional, horizontal under GRH). So Conjecture 10 can fail only across a prime conductor gap $ell >= p^epsilon$. *Measured:* such classes are a fraction $1.7 dot 10^(-2)$ ($epsilon = 0.1$), $2.7 dot 10^(-4)$ ($0.25$), $3.8 dot 10^(-6)$ ($0.4$) of all $1 048 576$ ordinary traces at $p approx 2^36$. Open (Conjecture 10′): is there a prime-field index calculus whose relation probability depends on the conductor? Over $FF_(2^N)$, $N$ composite, and $FF_(p^k)$, $k>1$, the failing hypothesis is the existence of proper subfields (GHS genus depends on the member's coefficients).

*(ii) Saturation law.* For an oracle sweeping $(m-k)$-tuples from a domain $Omega$ with a $k$-summand subsolver, a kernel-saturated base pays $mu^(m-2) dot (|Omega|/|F|)^(m-k)$ (Prop. 15.2′). Sweeping $F$: $mu^(m-2)$. Sweeping a transversal of $F\/M$: $mu^(k-2)$, i.e. *1 for the pair-table mitm oracle*. The domain ratio $mu^(m-k)$ is exact (Lemma 7). A1-R's $2.95 times$ $= 2.17$ (redundant sweep, law 2) $times 1.36$ (index-2 image base, law 1.51); the law with $Omega = F$ predicts 3.02. *Characteristic 2 (Theorem 9):* the quotient by $T = (0, sqrt(b))$ is the Frobenius conjugate, $x(2P) = u(P)^2$ with $u = x + sqrt(b)\/x$, so $[2](F_u) = F_"tz"$: the quotient-domain u oracle *is* the trace-zero oracle, and any algebraic system in a $T$-invariant coordinate is the plain x-frame system with target $x(2R)$. 0 exceptions on 30 bases over 10 binary curves.

== Figures
#figure(image("conductor_gap.svg", width: 92%), caption: [Fraction of ordinary isogeny classes over $FF_p$ (all traces, one prime per size) whose conductor index has a prime factor $ell >= p^epsilon$: the only region where Conjecture 10 is not reduced to Theorem 4. Measured (markers, exact) and the union-bound heuristic (dashed). Data: `conductor.json`.])
#figure(image("cost_factors.svg", width: 95%), caption: [Saturation cost and hit ratios: Prop. 15.2′ (red) against recorded values from round 3 (§15.2 census), round 4 (16.4), §7.A1-R and this round's exact char-2 census (`char2.json`). Ranges are the recorded spreads across curves.])

== Exact checks (measured, `char2.json`)
#table(columns: 3, [check], [instances], [exceptions],
  [$x(2P) = u(P)^2$; halving criterion $P in 2E <=> "Tr"(x) = "Tr"(a)$], [every point, 10 curves, $n in {11,13,17}$], [0],
  [$[2](F_u) = F_"tz"(V^2)$; $D_m^G$ equal ($m = 2, 3$); columns equal], [30 bases], [0],
  [transversal sweep finds every decomposable target; $"hits"_F = 2 dot "hits"_Omega$], [30 bases, all of $G$], [0])

== Predictions for R5.1 (A1 cell, $m = 3$)
P1 exact: $[2](F_u)$ equals the tz base, 19 = 19 columns. P2 exact: a transversal sweep halves hits and lookups, same decomposable targets. P3: the quotient-domain u arm costs about the tz arm, $approx 4.8 dot 10^3$ GAE per independent relation (vs 10 387), S $approx 85$ (vs 140). P4: $m = 2$ unchanged. P5: a u-symmetrised F4 oracle does the x-frame oracle's work per call.

== What this does not say
Not a proof of Conjecture 10 (no non-generic lower bounds exist); Theorem 4's horizontal part is conditional on GRH / heuristic path finding; cost factors other than the exact domain ratio are heuristic; Corollary 9.2 covers Weierstrass binary curves and the 2-torsion point only, not binary Edwards or 4-torsion formulations. Literature marked "recalled" in `RESULTS.md` was not re-opened.
