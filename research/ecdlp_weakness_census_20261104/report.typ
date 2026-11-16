#set page(paper: "a4", margin: 1.8cm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#show table: set text(size: 8pt)

= Exact censuses against five ECDLP hardness assumptions

Date 2026-11-04. Study `research/ecdlp_weakness_census_20261104/`; Lean companions in
`research/ecdlp_weakness_lean_20261104/`. Every number is an unverified assertion under
the repository's proof-carrying-claims rule (no signed validator report) [pcc:unverified].
Each census is an exact computation in Rust (no dependencies, deterministic seed,
in-code self-tests); `plots.py` only lays out recorded values.

== What was asked

Attack ten theorems and assumptions that hold up ECDLP hardness, and run the
$r_2$-versus-$k$ rank census that the earlier triage left as the step toward
refuting Petit--Quisquater Assumption 2. The owner's aim is to find out whether a
subexponential algorithm is possible. Nothing here shows that one exists; the results
remove or sharpen individual supports.

== The ten attacks

#table(
  columns: (auto, auto, auto, auto),
  align: left,
  [\#], [target], [instrument], [outcome],
  [1], [`HEUR-GB-PQ-ASSUMPTION-2` (m = 2)], [`pq2_rank` census + Lean criterion], [literal reading fails on the 111 instances with $n <= 44$; iterated degree > 3 for n = 15 and 17--44, exactly 4 on 33 fresh instances at n = 17--27; msolve F4 degree 4 at n = 17--34],
  [2], [degree-2 criterion generalised], [Lean `SubspaceCriterion`], [$d_"lin" <= "sd"$ for any Macaulay degree; kernel-checked],
  [3], [`HEUR-DECOMPOSITION-PROBABILITY` (prime field)], [`pf_yield` census], [zero class = Poisson once unordered; variance 1.45× Poisson for negation-closed bases],
  [4], [mean of the yield heuristic], [Lean `MeanCount`], [mean $|F|^m / |G|$ is a theorem],
  [5], [`GAP-PF-TRACE-PULLBACK-FACTOR-BASE`], [`trace_pullback` census], [no dim-1 subspace of $FF_(p^2)$ has a trace image below $0.32 dot \#E(FF_p)$],
  [6], [`HEUR-BK-GG-DECOMP-PROPORTION` (1/n!)], [`ext_decomp` census], [0.70--0.99 of $1/n!$ at n = 2, 3; premise fails for subfield curves],
  [7], [overbroad Shoup reading, "BSGS optimal"], [Lean `TransparentEncoding`], [refuted: identity encoding gives the log with zero group operations],
  [8], [`HEUR-BK-GG-FACTOR-BASE-SIZE` ($|F| approx |V|$)], [`pq2_rank` census], [supported: $|F'|/|V|$ in 0.84--1.31 over 99 instances, 0.997--1.004 for $n >= 30$],
  [9], [bridge $k = N - "affdim"(Z)$, case $|Z| = 1$], [Lean `BooleanNullstellensatz`], [unique zero puts every $X_i - z_i$ in the ideal; kernel-checked],
  [10], [`HEUR-PF-PKM-RANK-DELTA` ($Delta = 10$)], [`rank_deficit` census], [$Delta = 10$ gives full rank in all samples on 2 of 10 curves; deficits 1--3 persist],
)

== pq2_rank

#figure(image("figures/pq2_dlin.svg", width: 100%), caption: [Degree lower bounds per instance. d_iter: least degree whose iterated closure reaches all k linear consequences. d_lin: the same for the one-shot Macaulay matrix.])

#figure(image("figures/pq2_low_vs_k.svg", width: 100%), caption: [Dimension of the degree-≤1 part of each row space against k.])

119 instances, n = 8 to 47, three per n; the sweep stopped during n = 47. The iterated degree-3
closure reaches k for every n ≤ 14 and at n = 16, and reaches only 1 at n = 15 and on all
84 instances from n = 17 to 44. Fresh degree-4 sweeps at n = 17 to 27 reach k on all
33 instances, and msolve's F4 reaches maximal step degree exactly 4 on all 54 main-sweep
instances from n = 17 to 34. So the first fall degree 2 is not the solver degree on any instance with $n <= 44$, and the value 3 of
the Kousidis--Wiemers reading is not reached from n = 17 to 44. An independent naive
saturation in Python agrees with the Rust closure on five instances, and a same-seed rerun
reproduces the log. Lean (`F4Containment`) proves that an F4 run with maximal step
degree $D$ stays inside the degree-$D$ closure, so the census values bound F4 from below;
$d_"lin" <= "sd"$ is kernel-checked. The degree 4 matches
Kousidis--Wiemers' Magma value for $n <= 36$; they report 5 from $n = 37$, so a window ending
at $n = 34$ does not decide whether the degree stays bounded. m = 2 only. [pcc:unverified]

On fresh instances beyond the census window, msolve gives maximal step degree 4 at $n = 36$.
At $n = 37$, run on isolab-eks, it starts a degree-5 F4 step (a 1,016,044 × 610,665 matrix,
53 GB) after four degree-4 steps, reproducing with a second implementation the step to 5
that Kousidis--Wiemers report for Magma. Kubernetes stopped the run at its deadline about
three hours into that step, so the final basis check is unavailable; the corpus records the
result as an obstruction to Semaev's $d_"F4" <= 4$ (`MEAS-PQ2-MSOLVE-N37-STEP5`). A rerun to
completion was not started because the account's monthly budget is exhausted. [pcc:unverified]

== pf_yield

#figure(image("figures/pf_yield.svg", width: 100%), caption: [Unordered pair counts U(R): dispersion and zero class, three factor-base shapes, p from 2003 to 80021.])

Eighteen prime-order curves, $mu = B^2/r$ near 0.5, 1, 2, every target enumerated.
P(U ≥ 1) matches $1 - exp(-"mean" U)$ within about 1% on average for all three base shapes;
var/mean of U is about 1.45 for both negation-closed bases and 0.97--0.99 for the random
base not closed under negation. [pcc:unverified]

== rank_deficit

#figure(image("figures/rank_deficit.svg", width: 100%), caption: [Maximal rank deficit over five samples against the surplus Δ, negation pairs merged.])

Ten curves, p from 1009 to 16001, B ≈ √r. At Δ = 10 full rank in all samples on 2 of 10
curves; maximal deficit 1--3 at Δ = 10 and up to 2 at Δ = 40. [pcc:unverified]

== trace_pullback

#figure(image("figures/trace_pullback.svg", width: 100%), caption: [Trace image size of every one-dimensional subspace factor base, ten curves.])

min over V of $|"Tr"(F'_V)| \/ \#E(FF_p)$ is 0.32--0.39, at $V = FF_p dot i$ on every curve;
median 0.60--0.67. [pcc:unverified]

== ext_decomp

#figure(image("figures/ext_decomp.svg", width: 100%), caption: [Decomposable fraction over $1/n!$; curves with $FF_(q^n)$ coefficients against the subfield-curve control.])

0.350--0.413 at n = 2 and 0.101--0.165 at n = 3 for $FF_(q^n)$-coefficient curves
(0.70--0.99 of $1/n!$); 0.0003--0.002 at n = 3 for curves defined over $FF_q$. [pcc:unverified]

== Reproduce

`cargo run --release` in each crate directory; `python3 plots.py` for the figures;
Lean: `lake env lean AxiomAudit.lean` in the Lean project.
