#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 9.5pt)
#set table(inset: 4pt, stroke: 0.4pt + gray)
#show table: set text(size: 7.5pt)
= Curve25519 index calculus, wave 5: thirteen new leads, one law found
in the data (2026-11-12)
<curve25519-index-calculus-wave-5-thirteen-new-leads-one-law-found-in-the-data-2026-11-12>
#strong[Class.] Lead proposals and desk checks. Nothing was run on a
curve except reading committed cell files; no ecbench session, no row,
no leaderboard or atlas change. No pcc claim was registered, so every
statement is an unverified assertion \[pcc:unverified\].

#strong[Owner\'s request (2026-11-12).] \"Devise a bunch of new leads
for Curve25519 index calculus, find some new structure or something to
exploit.\"

#strong[How.] Three lanes worked in parallel from the same briefing.
Their full records are in `lanes/`:

- `real_parameters.md`: structure of the real curve\'s p, A, λ and ℓ
  (idea-generator, 5 records).
- `pipeline.md`: whole-pipeline cost of the current baseline
  (idea-generator, 4 records).
- `synthesis.md`: connections to other fields (idea-synthesist, 4
  records and 8 rejections).

The main session checked the lanes\' arithmetic and code citations, and
tested one law against the committed data (`checks/`). Lanes had no
shell, so every figure a lane derived by hand stays unchecked unless
this README says it was checked.

== 1. Answer
<1-answer>
- #strong[A law in the existing data.] The Gröbner solving degree of a
  decomposition system on a subgroup base V \= μ\_d tracks the
  #emph[largest prime factor] e\_max of d, not |V| \= d. It equals d + 1
  at every prime d (20 of 20 recorded cells). It is e\_max + 2 to 3 at
  composite d with m \= 2, and e\_max + 3 to 5 with m \= 3. The
  synthesis lane preregistered eight k \= 22, m \= 3 values that it
  states it had not read; all eight match the recorded cells
  (`checks/sd_law_check.out.txt`). The lane\'s statement that it had not
  read them cannot be verified \[pcc:unverified\].
- #strong[What the law says about the real curve.] p − 1 \= 4 · 3 ·
  65147 · C, with C a 236-bit probable prime (checked here). Every μ\_d
  base of Curve25519 with d ∤ 12 therefore has e\_max ≥ 65,147, and so
  solving degree at least 65,148 on that route. The useful |V| window
  for the PKM attack, about 2^64 to 2^85, holds no divisor of p − 1 at
  all. This is a security statement for the subgroup-base route, not an
  attack \[pcc:unverified\].
- #strong[Measured-improvement candidates for the baseline.] Four
  pipeline levers on the walk-collision IC arms could each give a
  measured gain at k \= 20 and 24. The largest prediction is about 0.83×
  cost at k \= 20 for recycling set-up points. None changes an exponent.
  After all four, the C \= 2 track would sit about 1.7 to 1.8× above
  rho-neg at k ≥ 28, which is the distance remaining \[pcc:unverified\].
  The owner ruled on 2026-11-04 that these arms count as index calculus
  (`research/curve25519_autolab_20261104/README.md`).
- #strong[Algebraic-track candidates.] Two levers target the cost of one
  decomposition system: a flat membership presentation and a coset rule
  from the descent character. They improve a component, not the
  baseline, because at m ≤ 3 plain enumeration on the same base is still
  cheaper than F4 (pipeline lane, §0).

=#figure(image("sd_law.png", width: 85%), caption: [Recorded F4 solving degree against the largest prime factor of d on all 50 complete exponent-study cells (k = 20, 22, 24; m = 2, 3). Unverified.])

= 2. Ranked leads
<2-ranked-leads>
The rank orders leads by expected measured improvement per unit cost,
then by how decisive they are \[pcc:unverified\]. \"Improvement\" leads
could produce a verified gain over the lab\'s IC baseline. \"Security\"
leads explain why the real curve resists. \"Prior\" is the lane\'s
stated probability that the prediction holds.

#align(center)[#table(
  columns: 6,
  align: (col, row) => (right,left,left,left,left,right,).at(col),
  inset: 6pt,
  [\#], [lead], [class], [prediction], [cheapest test], [prior],
  [1],
  [W5-PL-3 final \[d\]G \= Q check from the stored \[2^i\]G ladder],
  [improvement, pipeline],
  [−2 % at k \= 20, −0.7 % at k \= 24; walks unchanged, so paired runs
  differ only by the check],
  [hours of Rust; minutes of paired runs],
  [0.85],
  [2],
  [W5-PL-1 store every set-up point in the collision table],
  [improvement, pipeline],
  [0.83× at k \= 20, 0.95× at k \= 24, none at k ≥ 28; never worse at w
  \= 0],
  [about 1 day of Rust; minutes of runs; needs a rho reference-parity
  control],
  [0.75],
  [3],
  [W5-SYN-3 solving degree \= largest prime factor law],
  [security + instrument],
  [prime d: SD \= d + 1; composite: e\_max + \[2, 3\] (m \= 2), + \[3,
  5\] (m \= 3)],
  [done in-sample here (50 cells, 8 of 8 preregistered); next: k \= 24,
  m \= 3 and a Lean or CAS argument for the prime case],
  [0.8],
  [4],
  [W5-RP-1 flat y^d \= 1 per summand instead of the power-map chain],
  [improvement, algebraic component],
  [flat cost lies on the prime-d curve; chain / flat 70 to 260× at d \=
  27, 28 (k \= 20, m \= 3); d \= 36 completes under 3.5 GB],
  [under 1 h; chain cells already on disk],
  [0.65],
  [5],
  [W5-SYN-1 descent-character coset rule for μ\_d bases],
  [improvement, algebraic component],
  [gidx 0 needs 2× the solves per relation at m \= 3; control m \= 2 at
  1×],
  [about 10 min census, under 1 CPU-h],
  [0.6],
  [6],
  [W5-PL-2 walk that adds a random recent stored point],
  [improvement, pipeline],
  [−3 to −4 % at k ≥ 28 (the only lever there), about −22 % at k \= 20,
  overlapping PL-1],
  [stage-0 simulation in `src/bin/nc2_stage0.rs`, no group operations],
  [0.5],
  [7],
  [W5-RP-4 split sparse/dense F4 linear algebra],
  [instrument (memory)],
  [peak memory ≤ 1/4 of dense F4; k \= 20, m \= 3, d \= 36 completes
  within 12 GB, so the frozen exponent estimator can apply],
  [about 1 day of Rust, up to 8 h of runs],
  [0.4],
  [8],
  [W5-PL-4 jump count at the law\'s optimum],
  [improvement, configuration],
  [8 / 16 / 30 / 57 jumps at k \= 20 / 24 / 28 / 32; 0.983× at k \= 32],
  [configuration only],
  [0.4],
  [9],
  [W5-SYN-2 split of the yield heuristic at the Weil threshold],
  [security + corpus instrument],
  [mean yield is a theorem: 2× a random base on the right coset, 0 on
  the other],
  [shares the census tool of \#5],
  [0.6],
  [10],
  [W5-RP-2 binomial versus dense versus random base of equal size],
  [security],
  [dense / binomial cost 1 to 8; random / dense 0.5 to 2],
  [under 2 h],
  [0.55],
  [11],
  [W5-RP-3 torus and cyclotomic-torus bases for Curve25519],
  [security],
  [no 2^32-smooth divisor ≥ 2^64 of Φ\_k(p) for k ∈ {1, 2, 3, 4, 6};
  torus base costs as a random base],
  [desk census plus minutes],
  [0.6],
  [12],
  [W5-SYN-4 sumset base x(A ⊕ B) on an auxiliary curve],
  [improvement, low prior],
  [wins only for 60 ≲ d ≲ 1,500 at m \= 3; needs no smooth p ± 1],
  [about 1 day, ≤ 6 CPU-h],
  [0.25],
  [13],
  [W5-RP-5 CRT tensor presentation μ\_{d1 d2} \= μ\_{d1} × μ\_{d2}],
  [improvement, low prior],
  [beats the chain on ≥ 2 of 3 cells; never beats flat],
  [under 1 h],
  [0.15],
)
]

== 3. How the leads connect
<3-how-the-leads-connect>
- #strong[\#3 and \#4 test the same tension from two sides.] The chain
  presentation lowers the per-variable degree to e\_max but adds one
  variable per prime factor of d. In the exponent study, cost rose about
  8.5× per prime factor (validator joint fit). The flat presentation
  keeps one variable but has degree d, so by \#3 it should have solving
  degree d + 1 even for composite d. \#4 measures which side wins. A
  flat win would also remove the only reason the μ\_d route needs smooth
  p − 1. A chain win would confirm the SD law as the cost driver.
- #strong[\#5 overlaps an earlier lab result.] The lab\'s T₀-symmetry
  study (`research/curve25519_t0_symmetry_20261102/`) found that x ↦
  1/x, the action of the 2-torsion point, doubles coverage for pair
  tables. \#5 is the same symmetry seen from the algebraic side: with g
  \= 1 the base μ\_d is closed under x ↦ 1/x, so each relation is found
  2^(m−1) times. \#5 should cite that study, and its census must show
  the duplicates are not already removed by the engine.
- #strong[\#1, \#2, \#6 and \#8 all act on the walk-collision arms.] \#2
  and \#6 overlap at k ≤ 24 and their gains do not add. \#8 is moot if
  \#6 lands. The pipeline lane\'s desk finding behind \#6 and \#8
  explains the measured 3 % walk excess as the repeat rate of a stored
  walk with 16 fixed jumps: (1 − 1/16)^(−1/2) \= 1.0328 (checked here).
  That arithmetic is checked; the fit to the walk data is the lane\'s
  and is not checked.
- #strong[\#7 is the instrument the exponent question is waiting for.]
  The m \= 3 verdict stayed \"undetermined\" because d \= 36 ran out of
  memory at 12 GB (`research/curve25519_exponent_study_ext_20261110/`).
  \#7 is the only lead here that could let the frozen estimator apply.

== 4. Structure looked at and rejected
<4-structure-looked-at-and-rejected>
The real-parameter lane rejected these as levers, with reasons in its
file: the small-height λ \= 121665 / 121666 of the 2-isogenous Legendre
curve, the small A, the near-power-of-two ℓ \= 2^252 + c, the carry
structure of 2^255 − 19, NTT evaluation over a subgroup, μ\_4, E′\'s
full 2-torsion, cofactor 8, and the small 2-adic valuations of p ± 1.
The synthesis lane rejected Guruswami–Sudan list decoding, sum-product
bounds, BKK mixed volumes, Kani–Robert torsion attacks, Blahut linear
complexity and lift-fraction coset selection. The pipeline lane closed
these directions for the current baseline: linear algebra (about 10 row
operations per run), large primes and cycle-finding (each large prime is
another unknown), multi-target reuse (excluded by the owner),
factor-base size (autolab grids are flat), and Montgomery batching (no
change in counted GAE) \[pcc:unverified\].

== 5. Checks done in this session
<5-checks-done-in-this-session>
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [check], [result], [file],
  [p − 1 and p + 1 factorisation by trial division plus a Fermat test],
  [p − 1 \= 4 · 3 · 65147 · C, C 236-bit probable prime; p + 1 \= 2 · 5²
  · 79 · M, M composite (244 bits). The brief\'s \"C ≈ 221 bits\" was
  wrong, as the lane said],
  [session log],
  [2-isogenous Legendre curve on the k \= 20 miniature],
  [E′ \= E/⟨(0,0)⟩ has \#E′ \= \#E and full 2-torsion; λ \= 231/232 mod
  p; the Legendre model is the quadratic twist of E′ (point count 2p + 2
  − \#E′)],
  [session log],
  [SD law, all complete sym cells at k \= 20, 22, 24],
  [prime d: SD \= d + 1 in 20 of 20; preregistered k \= 22, m \= 3: 8 of
  8],
  [`checks/sd_law_check.py`, `.out.txt`],
  [Pipeline lane code citations],
  [`plugins.rs` walk parameters, `ic_boundary.rs` jump draw with γ
  uniform in \[1, r), and the `g.mul` recovery check all exist as
  cited],
  [session log],
  [Owner ruling on walk arms],
  [C \= 2 seed arms and walk arms with known-log G jumps count as IC
  (2026-11-04)],
  [`research/curve25519_autolab_20261104/README.md` line 99],
)
]

== 6. Not done and caveats
<6-not-done-and-caveats>
- research-kb, ecdb and the lead atlas bodies were not reachable from
  this runtime. Novelty is checked only against the 176 atlas lead ids
  and the repository files the lanes read. The nearest atlas ids are
  named in each lane record. A knowledge-base pass is owed before any
  lead becomes a task card.
- Every lane number derived by hand is unchecked unless §5 lists it.
  Most literature was read from abstracts. The pipeline lane owes a
  literature check on \"self-sum walk\" and \"recycled set-up points\"
  (Teske; Bernstein–Lange, ePrint 2012/294).
- No experiment was run, so there is no benchmark record, Conductor task
  or resume checkpoint.
- The lanes ran on a different model tier from the agents\' defaults,
  because the default tier\'s weekly limit was reached. That changes no
  rule.

== 7. Next steps
<7-next-steps>
The owner decides which to run. In the order of §2, the cheapest pair
with a likely measured gain is \#1 then \#2. Both need a frozen
protocol, a paired single-target ecbench session, a validator and a
matched rho reference under the §5a band. The cheapest decisive desk
item is \#3 at k \= 24, m \= 3, using cells already on disk once they
exist.

== Files
<files>
#align(center)[#table(
  columns: 2,
  align: (col, row) => (left,left,).at(col),
  inset: 6pt,
  [file], [content],
  [`lanes/real_parameters.md`],
  [5 records on the real curve\'s parameters (W5-RP-1 to 5)],
  [`lanes/pipeline.md`],
  [4 records on the walk-collision pipeline (W5-PL-1 to 4)],
  [`lanes/synthesis.md`],
  [4 cross-field records (W5-SYN-1 to 4) and the rejections],
  [`checks/sd_law_check.py`, `checks/sd_law_check.out.txt`],
  [the SD-law check on committed cells],
  [`checks/sd_law_figure.py`, `sd_law.svg`, `sd_law.png`],
  [the figure],
  [`REPORT.typ`, `REPORT.pdf`],
  [this README as a report],
)
]
