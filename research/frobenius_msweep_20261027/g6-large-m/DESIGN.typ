#set page(margin: 2cm)
#set text(size: 10pt)
#set heading(numbering: "1.")
#align(center)[
  #text(size: 15pt, weight: "bold")[Cell G6: Frobenius and 2-torsion structure at m = 9..20] \
  #text(size: 10pt)[Design note, revision 2, prepared and not run (2026-10-27). Every number below is arithmetic or a unit-test result; no experiment has produced data.]
]

= Question and instrument

Does any Frobenius or 2-torsion structure change the relation yield of an ordered-slot factor base at toy $n$, against the same instrument without the
structure, at matched slot-size profile? (Revision 2 dropped the cost half, since NI and UP have identical operation counts by construction, and the attribution
to $m$: at fixed toy $n$ the slot size moves with $m$, so cells with $n = 13, 17$ at $m = 9, 10$ vary it at fixed $m$.)

The instrument is an exact census. For ordered slots $S_1, dots, S_m$ (slot $S_i$ = points whose $x$ lies in a coordinate subspace $V_i$, including the 2-torsion
point $T$ for $x = 0$) it computes $H[R] = |{(P_1, dots, P_m) in S_1 times dots times S_m : sum P_i = R}|$ for every group element $R$ at once, by two
meet-in-the-middle halves and all pairs. It runs no solver, so it cannot show an improvement over the index-calculus baseline and reports no distance to rho.

= Arms and controls

#table(columns: (auto, 1fr), stroke: 0.4pt, inset: 4pt,
  [NI], [random normal basis, slot $i$ = conjugate indices $equiv i mod m$ (the `n83_rotated_m10` rule): the Frobenius-structured design],
  [UP], [generic basis, each slot redrawn until it has exactly the point count of the paired NI slot: matched profile, no Frobenius structure],
  [UP2 / DUP], [independent second UP draw (A/A); UP with two slots made identical (positive control, expected dispersion about 2)],
  [NC / NR / ONB], [contiguous and random partitions in a normal basis (unpaired, descriptive); the type-II optimal normal basis at $n = 23$ (one instance)],
)

Decision rule (frozen, implemented in `analyze.rs`): paired log-ratios of yield efficiency, dispersion and the orbit Fano factor; EQUIVALENT by TOST at
$plus.minus 10 %$, DIFFERENT only beyond a Bonferroni critical value and the margin, otherwise UNRESOLVED with the minimum detectable effect.
A cell whose positive control, A/A, conservation, planted-tuple or digit-model check fails is VOID. NI/UP pairs whose NI instance is a Frobenius chain are a
derived stratum (next section), not a test.

= What the design work found before any data

+ On-curve status is Frobenius invariant ($x$ is on the curve iff $x^2$ is), so full interleaved slots are Frobenius rotations of each other. Their sizes are tied together
  and the factor-base size takes only a few values, each a whole number of bits apart (figure below, hand arithmetic of PROTOCOL section 4.3).
  At $n = 23$ the production partition rule cannot reach the window $log_2 mu in [-10, 5]$ for $m >= 16$; those cells are expected UNMATCHABLE, and the census measures that exactly.
+ The translation $P arrow.r.bar P + T$ acts on $x$ as $x arrow.r.bar sqrt(b)/x$ (proved, tested). For prime $n$ only a one-dimensional subspace is stable, so the
  $2^(m-1)$ stabiliser needs non-subspace slots. For $k$ stable slots, $H_iota [R] = 2^(k-1) (H_"plain" [R] + H_"plain" [R+T])$: the stable base is the plain base queried at
  $R$ and $R+T$, at $2^k$ times the enumeration. Relabelling unless an $iota$-aware enumerator exists.
+ *(Review, revision 2.)* In the class-(0,1) cells the NI slots are ${T, plus.minus sigma^i P}$, a chain of consecutive Frobenius images, and
  $tau^2 minus.plus tau + 2 = 0$ forces collisions among the $3^L$ digit sums: 9711 distinct of 177147 ($n = 23$, $m = 12$), 2335 of 19683 ($m = 14$), 543 of 2187
  ($n = 19$, $m = 12$), matched exactly to the group law and the real builder. NI yield efficiency is then lower by $times 4$ to $times 18$ and dispersion higher by
  $times 6$ to $times 31$ as an algebraic fact. The cells report this as CHAIN_DERIVED, cross-checked at run time; it is a toy-slot-size effect and says nothing about
  production slot size or $n = 131$. The earlier prior "EQUIVALENT within 10 percent" is withdrawn.
+ Orbit-union (symmetric) bases are oversized at toy $n$ by $2^8$ ($m = 9$) to $2^26$ ($m = 20$) for a single orbit, and sane only near $n = 131$ (three orbits at $m = 20$). They cannot be enumerated.
+ The chained SAT encoding has $(m-1)(3 n^2 + 4 n) + n (2 m - 1)$ variables and $2^(m-2)$ pattern instances ($65536$ at $m = 18$, not $16384$); the source stops at $m = 6$.

#figure(image("design_feasibility.png", width: 85%),
  caption: [Sizes the Frobenius-interleaved design can take at $n = 23$, $a = 0$ (hand arithmetic: $A$ and $B$ are the on-curve bits of $beta$ and $beta + beta^(2^m)$).])

= Cannot reach

Solver costs (SAT, Groebner, degree of regularity); $n = 131$; orbit-union bases at $m >= 9$; Frobenius-stable slots; the variant that excludes $x = 0$; the prime-subgroup stratum;
$m in {13, 15, 17, 19}$; the $n = 83$ production gate.

= Status

Crate `g6-large-m` builds and passes 32 unit tests on tiny fixtures; 26 cells are planned. The S3 resume-checkpoint gate blocks runs in the preparation container; `RUN_CARD.md` lists the exact commands.
