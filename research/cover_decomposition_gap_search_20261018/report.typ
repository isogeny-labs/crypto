#set page(paper: "a4", margin: 1.8cm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set par(justify: true)
#show heading: set text(weight: "bold")

#align(center)[
  #text(size: 15pt, weight: "bold")[Cover and decomposition attacks on the ECDLP: \ what has been searched, what has not]
  #v(2mm)
  Research search round, 2026-10-18 · `research/cover_decomposition_gap_search_20261018/` \
  Repository `aburan28/crypto`. Status of every statement: _proposed_ unless marked measured or derived. \
  No ECDLP speedup is claimed; no deployed-curve security claim is made or withdrawn.
]

= Question

For cover attacks (Weil descent, GHS and descendants, isogeny and gluing transfers) and decomposition attacks (Semaev summation polynomials, index calculus with algebraic factor bases) on elliptic curves over prime fields, Koblitz curves over $FF_(2^n)$ with $n$ prime, and random binary curves: what has this repository searched, what has the literature searched, and what has neither touched?

= Method

Two read-only survey agents inventoried about 90 study directories, the `RESEARCH_*.md` notes, `docs/ic/`, `backlog/` and `problems/` (appendices A, B). Three web-search agents swept the literature: covers 2015–2026, decomposition 2014–2026, and adjacent routes (appendices C, D, E; about 190 queries in all; ePrint PDFs were bot-blocked from this host, so abstracts and arXiv or author copies were used and the appendices mark where a statement rests on an abstract). The repository's earlier literature rounds (`research/literature_search_20261011`, `research_notes/Index calculus leads next steps/`) were read first. One derivation and one native census were done here (§4).

= The coverage matrix

#figure(
  image("coverage.svg", width: 100%),
  caption: [Routes against field families, status as of 2026-10-18. Cell text is a compressed status; colour is the status class. Every cell traces to a row of appendix A or B (repository) or C, D, E (literature). Source: `coverage.csv`, laid out by `plot_coverage.py`.],
)

= Findings

== The standing obstruction

For $E slash FF_q$ with $q = p$ prime or $q = 2^n$, $n$ prime, $FF_q$ has no proper subfield. Any transfer by a correspondence lands in an abelian variety of dimension $g$ over the same field, where index calculus costs $tilde(Omega)(q^(2 - 2/g)) >= q^(1/2)$, or (binary only) over $FF_2$ in dimension $>= n$, where only the large-genus Enge–Gaudry regime pays. For decomposition the same fact reads: over $FF_p$ no subset of size $B$ has membership degree below $B$; over $FF_(2^n)$ with $n$ prime the Frobenius-stable subspaces are trivial. Every generic oracle sits on the $S T^2 ≳ r slash |"Aut"|$ line, a theorem for structured algorithms since Corrigan-Gibbs–Henzinger–Wu (ePrint 2026/384).

== What the repository has already closed

*Prime fields.* Covers closed by the obstruction; same-field genus-2 catalogue prices at $-2$ bits; $(N,N)$-gluing excluded for prime-order curves (Howe; Maisner–Nart); P-192 over $FF_(p^6)$ has a constructed Type I cover with zero relations found. Decomposition: small-$x$ closed by the Coppersmith reach ($1 slash 80$ against the $1 slash 5$ needed at $m = 4$); PKM tower bases negative at $m = 4$, inconclusive at $m = 3$, $N = 21$ (running on this host); thirteen other factor-base families refuted or yield-conserving; preprocessing rho measured as the control.

*Koblitz, $n$ prime.* GHS magic number 1; isogeny class a Galois orbit ($h(cal(O)_K) = 1$); Frobenius-invariant bases give the factors $n$ and $n^2$; FGHR symmetrisation 30–416× on the solve; Frobenius-equivariant F4 31–128× on rank; solving degree grows with $ell$ on every measured cell, with $(4,5)$ and $(4,6)$ blocked by memory elsewhere; every table-lookup oracle is generic and loses to Bernstein–Lange at equal advice.

*Random binary, $n$ prime.* Magic number $m = n$ for every deployed curve; Hess's generalised Artin–Schreier covers not enumerated; shape search over eleven factor-base families with refutation degree set by $ell$ alone.

*Literature 2020–2026.* No paper proposes a new route on these families; no credible claim of progress on secp256k1, P-256, Curve25519, ECC2K-130 or the NIST binary curves. The most recent genuine cover result is Tian (J. Cryptology 2023), which needs a cubic extension; the most recent Koblitz-specific decomposition result is Galbraith–Granger–Merz–Petit (SAC 2020), explicitly "Pollard rho remains the fastest".

== New here: cyclic covers carrying $A$ at any base genus (derived, then enumerated)

For ECC2K-130 the ledger asks whether the simple 130-dimensional trace-zero variety $A$ of $"Res"_(FF_(2^131) slash FF_2)(E_0)$ is isogenous to a factor of a Jacobian of genus 130 to about 290 over $FF_2$. The repository closed GHS covers, the modular curve (genus $2^(28.92)$), and cyclic degree-131 covers of bases of genus $<= 1$. Bases of genus $>= 2$ were not treated.

*Derivation* (README §4.2). Let $C -> C_0$ be geometrically cyclic of degree 131 over $FF_2$, with $C$ carrying $A$, and $d$ the order of Frobenius on the covering group. (1) $A$ is simple and cannot lie in $"Jac"(C_0)$ inside the window, so $A$ lies in the Prym $P$. (2) The Weil numbers of $P$ are $mu_d$-stable; those of $A$ are not for $d > 1$, so $dim P >= 130 d$; with $dim P = 130(g_0 - 1) + 65 b$ this puts every $d >= 3$ case, and every ramified twisted case (class field theory over $FF_(2^d)$ forces $b >= 130 slash d$), outside the window; the $d = 2$ unramified case is a twist of a $d = 1$ case. (3) With $d = 1$, ramification needs $131 | 2^(deg P) - 1$, hence $deg P >= 130$ and genus $>= 65 dot 130$. (4) With $d = 1$ unramified, $131 | \# "Jac"(C_0)(FF_2) <= (1 + sqrt(2))^(2 g_0)$ forces $g_0 >= 3$ and $g(C) = 131(g_0 - 1) + 1$; genus 394 at $g_0 = 4$ is outside the window; at $g_0 = 3$ the Weil bound 198 forces $\# "Jac"(C_0)(FF_2) = 131$ exactly, $g(C) = 263$.

So the family inside the window reduces to genus-3 curves over $FF_2$ with Jacobian order 131. Equivalently: no curve over $FF_2$ of genus below 394 carrying $A$ has a geometric automorphism of order 131. Every structured construction in the repository has that symmetry.

*Census* (native, `examples/unramified_cyclic_cover_census.rs`, on the enumerator of `curve_construction/enumerate.rs`; output `census.json`). Every genus-3 curve over $FF_2$ (hyperelliptic models and all $2^15 - 1$ plane quartics) against every degree-6 Weil polynomial with $P(1) = 131$. A miss needs no smoothness check; completeness of the enumeration is the proof.

#figure(
  table(
    columns: 4,
    align: (left, right, right, right),
    [*family*], [*models admitted*], [*Weil polynomials with $P(1) = 131$*], [*hits*],
    [genus-3 hyperelliptic], [6,144 (smooth)], [347], [*0*],
    [genus-3 plane quartic], [32,767 (all forms)], [347], [*0*],
  ),
  caption: [Native census, 2026-10-18. The independent Python scoping run (`appendix/scoping_enum_f2.py`, counts over $FF_2 dots FF_64$ with a functional-equation check) also found 0 and 0; it is not cited.],
)

*Toy ladder.* For $n in {5, 7, 11, 13}$ with genus-2 bases the Prym would have to be $A_n$ itself, which forces $N_1(C_0) in {1, n + 1}$. The census records the pairs $(\# "Jac", N_1)$ of every genus-2 curve over $FF_2$ with $n | \# "Jac"$: $(5,2), (5,3), (10,4), (15,5)$, $(7,3), (14,5)$, $(11,4)$, $(13,5)$. None qualifies.

*Counting heuristic* for the unstructured remainder (DiPippo–Howe isogeny-class counts): expected carriers of genus $g$ about $2^(4189.5 - 62 g)$, i.e. $2^(-3870)$ at $g = 130$. It over-predicts the ladder's small targets and under-predicts structured classes, so it is a statement about the unstructured remainder only.

== What has not been searched, ranked

#table(
  columns: (auto, 1fr, 1fr, 1fr),
  align: left,
  [*\#*], [*item*], [*status*], [*falsifier and expectation*],
  [1], [Frobenius-equivariant F4 solving degree at $(m, ell) = (4,5), (4,6)$], [measured to $(4,4)$; blocked at a 3 GB cap elsewhere; nothing in the literature bounds $D$], [run on this 124 GB host under a frozen protocol; expected $D$ follows $ell + 4$; a level at 7 would move the ECC2K-130 $m = 4$ pricing by up to 96 bits],
  [2], [prime-field $D_4$-symmetrised $S_5$ lattice probe at $m = 4$], [asked for in the leads note §9; run only over $FF_(p^4)$], [Probe B/C with the $(ZZ slash 2)^3 ⋊ S_4$ invariants on a 40-bit curve; expected: lower cost per reduction, same threshold],
  [3], [Hess's generalised Artin–Schreier magic numbers for sect113r1…sect571r1], ["not enumerated" in `JACOBIAN_TRANSFERS.md`; Hess 2003 states no threat for prime degree], [one afternoon, native; expected $n$ or $n + 1$ everywhere],
  [4], [mixed-conjugate 4/5-summand test (decomposition programme W2/W3)], [planned, nothing run], [W2's $e(ell)$ census at $n = 13, 19, 23$ first; expected above budget],
  [5], [counterfactual $N = 130$ genus-255 cover, constructed], [model-positive at $-4.7$ bits; nothing built], [build the cover, norm–conorm map and $\# "Jac"$; expected the unit conversion eats the margin],
  [6], [isogeny-walk cost for GHS-extended on composite binary curves], ["beyond 3-isogeny steps not established"], [re-price with Galbraith 2024 / Galbraith–Gilchrist–Robert 2025 $tilde(O)(q^(1/4))$; no run],
  [7], [literature never cited], [Yokoyama et al. JMC 2020; Diem–Kochinke; Oyono–Thériault 2023; Huang–Raskind 2009; Delaplace–May 2019; Courtois 2016; Karabina 2015; Chi-Domínguez et al. 2021], [read and cite; none changes a target-family exponent],
  [8], [Huang–Raskind signature calculus at toy size], [never implemented by anyone], [two days; expected negative],
  [9], [ML-guided Gröbner on summation systems], [`fpn_decomposable` config unrun; HATSolver cited only], [train at $n = 17$–$23$; falsifier: a learned order beating WDSat by 2×; expected chance],
  [10], [Oakley group 3 genus-16 index calculus (composite degree)], ["not started"; $+1.1$ bits formula], [outside the asked scope; the only binary cover row where a measured non-toy row is possible],
)

== What to stop searching for

Covers of prime-field curves; GHS, Hess or isogeny-extended descents of Koblitz ($m = 1$, $h = 1$) and prime-degree random binary curves ($m = n$), once item 3 is recorded; table-lookup decomposition oracles of any arity; first-fall-degree measurements; small-$x$ Coppersmith at $S_3$, $S_4$; cyclic covers carrying $A$ at any base genus, and any carrier with an order-131 automorphism; trace-zero transfers, Kani gluing, xedni, canonical lifts, elliptic nets, Drinfeld analogues and Belyi/Hurwitz constructions for the three target families (reasons in README §5).

= Recommended next experiments on this host

1. Land this round's census as a receipt (this PR). 2. Item 1 above once the PKM $m = 3$, $N = 21$ run releases memory: a frozen protocol, three cells of $D$, the gap to rho at $n = 131$ priced by `RESEARCH_ECC2K130_DESCENT_DEGREE.md`. 3. Item 2, one day. 4. Item 3, one afternoon.

= Scope and limits

Repository statuses come from the inventories' reading of READMEs and reports, not from re-running anything. Literature statements marked as abstract-only in the appendices were not checked against full texts. The derivation in §4 uses the $mu_d$-stability lemma already used by `RESEARCH_ECC2K130_CURVE_CONSTRUCTION.md` and unramified class field theory over finite fields; it has no Lean or independent proof receipt. The census is complete for genus 3 over $FF_2$ by the enumerator's own completeness statement; a miss needs no smoothness check. Nothing here changes a baseline, a leaderboard row, or a bound.
