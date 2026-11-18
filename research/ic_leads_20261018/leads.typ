#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)

= Index-calculus leads after the closure-slope study (2026-10-18)

Companion to `RESEARCH_IC_LEADS_20261018.md` in this directory. *Class:* analysis and one exploratory pilot; no `S`, no rho ratio, no claim about a deployed curve. Prior-art verdicts cite the repository files named in the note.

== The filter

Five closure arguments are settled enough that a lead failing one is not a lead: (1) the generic-group bound on table oracles, with the preprocessing-rho control measured positive on 2026-10-14 and 2026-10-16; (2) no Frobenius-stable subspace at $n = 131, 163$ (2 is a primitive root); (3) shape closure at $m >= 3$ (every shape equals random, G1 killed); (4) the admissible-degree inequality (degree 3–5 at $w = 1$, 2 at $w = 2.2$ for $n = 131$); (5) the measured degree wall (closure 4 at $m dot ell <= 18$, above 4 at 21, degree 5 at 40 unknowns beyond a 15 GB host).

#figure(image("figures/lead_map.svg", width: 100%),
  caption: [Each lead against the five arguments. Green: the lead escapes or formalises the argument; yellow: untested or in flight; red: closed by this round.])

== The leads, ranked

- *L1 target-free algebraic relations* (closed 2026-10-18): solve $S_4(x_1, dots, x_4) = 0$ with every variable in $V$ and no target. Measured in two preregistered studies. In the chained presentation, the F4 degree equals the target system at $(16,4)$, with about 10 times the columns. In the eliminated $S_4$, the target-free system is one degree *higher* than the $S_4$ target system on 15 of 15 readings at $(13,3)$ and $(16$–$19, 4)$, with no Boolean top-form dependency on the progression base at $ell = 4$. Registered verdict: killed (`research/target_free_s4_20261018/`).
- *L2 dense XL at degree 5* (measured 2026-10-18): unmeasured on all 8 registered draws at $m dot ell = 21$. The real matrix is $218,918 times 619,790$ at $N = 40$ (17 GB dense), not the estimated 34,482 rows; every run stopped at the 6.3 GB pivot cap. Needs a 32 GB host or a sparse rank test (`research/dreg_xl_degree5_20261018/`).
- *L3 raised-cap sparse closure*: run at 2.5 G entries and killed by the memory cgroup at 7.2 GB after two hours; unmeasured. The existing dense engine scanning to degree 5 was killed twice as well (9.8 GB; allocation failure under 13.5 GB). Degree 5 at 40 unknowns needs more than this host for every engine tried.
- *L4 4E-class stratification*: closed by a sign argument; with free summand signs the $E\/4E$ class never obstructs a target unless the whole base lies in $2E$ without class-2 points.
- *L5 coset-indicator degree* — piloted, negative as a predictor (below).
- *L6, L7 theorem and lemma*: write the $m >= 3$ shape-closure argument and the admissible-degree inequality as statements the FFD programme can cite; neither exists in writing.
- *L8 K-233 derived row*: $"ord"_233(2) = 29$, eight stable 29-blocks, admissible degree 3–4 under the $C(N, <= D)^(2.2)$ cost and 7–8 under the $w = 1$ lower bound; a distance row for the one standard Koblitz curve where Frobenius structure helps the algebra. Not analysed anywhere.
- *L9 G4 stable non-subspace bases*: closed by counting (87 conditions, 22 cubic, $3n$ unknowns).
- *L10 ladder accounting*: Lead A ran positive twice; two documents still list it as scheduled.

== Pilot: the coset-indicator degree (negative)

#figure(image("figures/coset_indicator_pilot.svg", width: 100%),
  caption: [ANF degree of $x arrow.r "Tr"(1\/x)$ on $V$, from `coset_indicator_pilot.jsonl` (80 records; committed anchor and fresh bases at $n = 12$–$30$, progression and random bases at $n = 43$), against the measured closure degrees of the chained system. Layout only; the degrees are exact Möbius transforms.])

The indicator degree is $ell - 1$ or $ell$ on every base, progression no better than random, far above the closure degree 4 at $ell <= 6$: the closure does not refute through the parity, the indicator does not predict it, and no subspace shape at prime $n$ lowers the parity degree (Mattarei: inverse-closed subspaces are subfields). Exploratory, not preregistered.

== What would change the picture

The closure degree at $m dot ell = 21$ and beyond (64 GB host, or L2/L3); (L1, a target-free system a degree below the target system, was measured and closed on 2026-10-18); or a proof that the closure degree is $Omega(ell)$, for which L6 is the first lemma. Everything else is a constant, a row or bookkeeping, listed because the success criterion counts verified constants.
