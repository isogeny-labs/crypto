#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.5pt)
#set par(justify: true)
#show link: underline

= Sub-exponential ECDLP over prime fields: a desk evaluation against the lead atlas

*Date:* 2026-11-01. *Study:* `research/ecdlp_subexp_desk_20261101/` (README with the full tables, data, figure source).
*Atlas:* `aburan28/crypto-autoresearcher` `main` cfc1dae2, `knowledge/leads/ecdlp/` (154 leads, reviewed 2026-10-30).
*Classification:* desk evaluation. No improvement is claimed, nothing was measured, no disposition changes.

== 1. Why every sub-exponential route must read the encoding, and where each one stops

Shoup's bound (`LEAD-RHO-generic-sqrt-lower-bound`) puts every algorithm that uses only the group law and
equality at $Omega(sqrt(N))$ on a prime-order subgroup, and at $S dot T^2 gt.tilde N$ with advice $S$. A
sub-exponential algorithm therefore reads coordinates, the curve equation or the field. The atlas sorts the
program's work by the routes through which an encoding can be read; for prime fields each route ends at a
named obstruction (lead ids in the README, section 1):

- *algebraic factor bases* (Semaev systems): a size-$B$ algebraic set over $FF_p$ has description degree
  $>= B\/3$, and the mean decomposition yield is $binom(B+m-1, m)\/N$ for every base of size $B$ (theorem; measured
  ratio $1.000000$ to a random base);
- *small-root decomposition* (Coppersmith): every lattice on a convex monomial set reaches at most $0.3293$ per
  $S_3$ block against a demand of $0.8889$ (proof);
- *representations and subset sums*: generic, so $T gt.tilde sqrt(N)$ (theorem); *enumerative, pair-table and
  MITM oracles*: no sub-rho cell at any arity (cost model);
- *$p$-adic lifts*: prime-to-$p$ torsion lifts uniquely, so every additive order-$ell$ image is zero; jet oracles are
  GGM-simulable (proof); the anomalous channel needs $r = p$;
- *multiplicative transports*: a homomorphic image factors through the embedding degree; no self-pairing on a
  prime-order subgroup;
- *covers and Jacobians over the same field*: index calculus costs $p^(2-2\/g) >= p$ against rho's $p^(1\/2)$;
- *the isogeny class and endomorphisms*: generic cost is an isogeny-class invariant; $sqrt(|"Aut"|)$ at most;
- *global lifts*: the $QQ$-lift (xedni) is closed by the literature; the function-field section timing is open.

#figure(image("figures/atlas_tally.svg", width: 100%),
  caption: [The atlas by family and disposition, as read from the 154 lead files (`data/atlas_tally.json`). 74 leads are closed with a named obstruction, 54 have a decided next step, 17 are blocked, 9 are unexamined.])

== 2. Sixteen candidate mechanisms, each against the atlas

The sixteen candidates were drawn from the standard cryptanalytic toolbox and from the routes above, and each
was put through `python3 tools/lead_atlas.py --match`. Every one lands on an existing lead (figure 2; raw hits in
`data/candidate_matches.json`; the reading of each hit is in the README, section 2). Ten land on closed or
exhausted leads, two on blocked ones, four on open ones (`LEAD-RHO-cheaper-step-primitives`,
`LEAD-EXO-invertible-hint-subset`, `LEAD-EXO-four-list-ktree-join`, `LEAD-STR-class-group-ecdlp-bridge`), and
none is new. The atlas's rule for this case is to advance the matched lead's next step, not to file a duplicate.

#figure(image("figures/candidate_matches.svg", width: 100%),
  caption: [Each candidate's best `--match` hit (word-overlap score) and that lead's disposition. The "best lead" is the author's reading of the hits; the score is not a verdict.])

One pointer the atlas lacks: the polynomial-interpolation degree lower bounds for the elliptic-curve discrete
logarithm (Lange and Winterhof, "Interpolation of the elliptic curve Diffie-Hellman mapping", 2003; "On degrees
of polynomial interpolations related to elliptic curve cryptography", WCC 2005) bear on the open next step of
`LEAD-EXO-invertible-hint-subset`, which asks for the maximal agreement of degree-$d$ polynomials with the log
graph. The theorem statements were not retrieved in this session; the pointer is recorded as unverified.

== 3. What a "drastic change" can honestly mean

+ On prime fields the exponent is $1\/2$ on every route the program has measured; the routes that could lower it
  are the open algebraic ones, and each has a cheap, decisive next step (section 4). None has a measured excess
  over its matched null.
+ Below-$sqrt(N)$ exponents are known on extension fields $E(FF_(p^n))$, $n = 3, 5$ (Gaudry, Diem, Joux–Vitse);
  the program's open leads there are the only places where such an exponent is a measurement away. Moving
  effort there changes the target, and is not a prime-field result.
+ The owner's criterion (AGENTS.md, 2026-10-15) counts every measured improvement over the current index-calculus
  baseline. The prime-field baseline last moved on 2026-10-29 ($N = 45$, $0.818 [0.773, 0.866]$ at $w = 0$, 20
  bits); the relation phase is 68–93% of $S$ on Table B. The distance to rho on that table is 5.5–13.5× at
  10–24 bits.

== 4. Open next steps, ranked by cost and by what each outcome decides

#table(
  columns: (auto, auto, auto, auto),
  [*lead*], [*next step (atlas)*], [*cost*], [*decides*],
  [`ICP-digit-presentation-solving-degree`], [certified degree-growth ladder, $m = 2, 3$, $s = 2..8$, independent primes, matched nulls], [toy, CPU-hours], [semi-regular growth closes the lane at scale; a bounded $d_"lf"$ below the null is the first elliptic early fall over $FF_p$],
  [`STR-class-group-ecdlp-bridge`], [the simulation lemma: curve-aware GGM with free class-group action still needs $Omega(sqrt(N))$], [desk / Lean], [a checked simulator closes the lane with a theorem; a failure names the one bridge a reduction must use],
  [`EXO-invertible-hint-subset`], [the interpolation-degree bounds first, then $A(d)$ on the toy ladder], [desk, then toy], [bounds forcing degree $>= c|S|$ close the low-degree variant as known],
  [`TRF-miller-cocycle-nonhom-transfer` / `EXO-miller-pairing-cocycle`], [verify K1/K2 on toy curves; equidistribution of $ell(k)$ with a planted control], [toy, hours], [equidistribution at the $1\/p$ rate closes both],
  [`EXO-four-list-ktree-join`], [the bucket-variance class $W$ of the O2 kernel against $1\/sqrt(N)$], [toy, CPU-hours], [slower decay reopens a bucket-concentrated filter],
  [`EXO-global-lift-height`], [$T_"section"(d, chi, p)$ on $E_1$ surfaces, $>= 3$ primes, Gröbner control], [CPU-days], [$T > p^(1\/2) "poly"(log p)$ closes the function-field variant],
  [`RHO-cheaper-step-primitives`], [tag-tracing walk on a 64-bit toy curve, ns per useful step], [engineering, hours], [$< 0.95$ opens an engineering lead; $>= 1.0$ closes it],
)

== 5. Scope

This note proposes no new lead, measures nothing and changes no disposition. The candidates are one author's
choice; `--match` is a word-overlap score and the "best lead" column is a reading of its hits. Python here only
lays out recorded values (`plot.py`); the data files are the atlas's own YAML, re-serialised.
