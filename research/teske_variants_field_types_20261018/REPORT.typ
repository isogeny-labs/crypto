#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm), numbering: "1 / 1")
#set text(size: 8.6pt)
#set par(justify: true, leading: 0.5em)
#set heading(numbering: none)
#show heading.where(level: 1): set text(size: 12pt)
#show heading.where(level: 2): set text(size: 9.8pt)
#show heading.where(level: 3): set text(size: 9pt)
#show heading: set block(above: 0.8em, below: 0.4em)
#show table: set text(size: 7.2pt)
#show raw: set text(size: 7.6pt)
#show link: set text(fill: rgb("#2a78d6"))

= Teske-type hidden Weil descents by field type: where the three
ingredients exist
<teske-type-hidden-weil-descents-by-field-type-where-the-three-ingredients-exist>
2026-10-18. A research search round (AGENTS.md "Research searches must
leave visual reports"): literature reading plus exact native enumeration
over toy fields. Protocol: #link("PROTOCOL.md"), frozen at commit
`62b4502c7` before the first run. Tool:
#link("../../examples/teske_variants_field_types.rs")[`examples/teske_variants_field_types.rs`]
(std-only Rust, 4 threads; `rustc --edition 2021 -O`, tests with
`--test`). Recorded runs: `20261018T0630Z` (Parts A and B;
#link("results/run-20261018T0630Z/"), run report
#link("reports/run-20261018T0630Z/run-report.pdf")) and `20261018T0845Z`
(Part C for every `q`; #link("results/run-20261018T0845Z/"), run report
#link("reports/run-20261018T0845Z/run-report.pdf")), each with `env.txt`
beside its files. The first run was killed by a restart of the session’s
worker while its census was at `q = 13`; the census root search was then
replaced by polynomial splitting (byte-identical output for `q ≤ 11`,
checked) and Part C was rerun. One earlier launch, `20261018T0624Z`, was
stopped after a smoke run exposed a normalisation bug in Part C; its
directory keeps a `NOTE.md`. This report as PDF: #link("REPORT.pdf").
Conductor is not installed on this host (the installer answers HTTP 403
through the session proxy), so there is no task card. The round solves
no discrete logarithm, times nothing, and makes no statement about any
deployed curve. Every number below is a count or a formula on exact
invariants; the exponents are formula-level labels in the sense of
`docs/curves/JACOBIAN_TRANSFERS.md`.

#strong[Tags.] \[T\] theorem, stated by a cited source or proved here;
\[E\] exact computation by the tool, reproducible and checked; \[H\]
heuristic; \[O\] out of scope or not established.

== Answer
<answer>
Teske’s trapdoor system (J. Cryptology 19 (2006); ePrint 2003/058) hides
a GHS-weak curve over `F_{2^161}` behind an isogeny. It needs three
things: a weakness that is #strong[not an isogeny-class invariant], a
#strong[transfer] that follows the isogeny, and a weak set
#strong[sparse inside the isogeny class], so that an auditor cannot find
the weak member by walking the class faster than Pollard rho. This round
asked over which field types and extension degrees all three exist, and
what other ways there are to attach a Weil-descent target to a curve.

- #strong[\[T\] Prime fields `F_p`: none of the ingredients.] There is
  no subfield to descend to (GHS 2002 §5), every same-field cover of
  genus `g ≥ 2` moves the problem into a group of size `p^g` where index
  calculus costs at least `p^{1−o(1)} > p^{1/2}`, and base change to
  `F_{p^k}` followed by a descent leaves the subgroup of size `p` inside
  a group of size `≥ p^k`. The only non-class-invariant structure left,
  the endomorphism ring, gives no exponential gain. No Teske-type
  variant via descent exists over `F_p` (§3).
- #strong[\[E, T\] Binary fields `F_{2^{nl}}`.] The exact combinatorics
  of `x^n − 1` over `F_2` (Part A) place the sparse regime at `n = 7`
  for the type-I genus-7 descent only (`δ = 4`, margin `+0.5`,
  i.e. `+11.5` bits at `N = 161`: an auditor’s walk costs as much as
  rho), while the type-II genus-8 descent (`δ = 3`, `−11.5` bits: the
  walk costs `2^69`) and Hess’s gGHS variant (`δ = 2`) are dense, which
  is why Menezes–Teske call `GF(q^7)` partially weak. `n = 3` and
  `n = 5` are dense for every curve (weak fields). The sparse regime
  reappears at `n = 15, 21, 31` and other odd `n` with small `ord_n(2)`,
  with genus `15`, `21–31`, `31`, but there the public sub-descents
  (`n = 5` inside `n = 15`) already beat rho, and `n = 31` is the
  `F_{2^155}` case of Hess 2003 (`2^{−38}` weak orbit classes per
  isogeny class). Prime `n` has `m ≥ ord_n(2) + 1` and nothing descends
  (Menezes–Qu). Table in §4.
- #strong[\[E, H\] Odd characteristic `F_{q^n}`.] The weak set of a
  Kummer-descent branch type with `r` points has `≈ q^{r−3}` curves: the
  exact counts at `q ≤ 13` reproduce the exponent `r − 3` with constants
  between `0.04` and `2.1` (Part B), so `δ = n + 3 − r` and the margin
  of the smallest-genus type is `a = n + 3 − r − n/2`: `n = 2: −1`
  (genus 2, Scholten, no gain anyway), `n = 3: −1/2` (genus 3: dense,
  the isogeny-walk regime Joux–Vitse conjecture), `n = 4: 0` for the
  `r = 5` type and `−1` for the `r = 6` types (all genus 5),
  #strong[`n = 5: +1/2` (genus 5)], `n = 6: 0` (genus 9), `n = 7: −1/2`
  (genus 7), prime `n ≥ 11`: genus `≥ 1793` (Diem). #strong[The
  odd-characteristic analogue of Teske’s field is `F_{q^5}`], with the
  genus-5 family `y^2 = ∏_{i=0}^{3}(x − α^{q^i})`, `α` of degree 5 (Diem
  2003; Hess 2004 Table 3). The isogeny-class statistics (every `j` over
  `F_{q^n}` traced) show the predicted drift: the share of curves whose
  class contains a weak member saturates for `n = 3` at the classes with
  `#E ≡ 0 (mod 4)`, stays flat for `n = 4`, and falls with `q` for
  `n = 5` (§5).
- #strong[\[T, E\] A second association, without covering maps.] By
  Tate, a genus-3 curve `C/F_q` has `Jac(C)` isogenous to
  `Res_{F_{q^3}/F_q} E` exactly when `L_C(T) = 1 − tT^3 + q^3T^6`. For
  hyperelliptic `C` this forces `t` even and a Frobenius cycle type
  `(1,1,3,3)`, `(2,3,3)`, `(1,1,6)` or `(2,6)` on the eight Weierstrass
  points (Theorem 3, from the Frobenius action on `Jac(C)[2]`), so
  #strong[no hyperelliptic genus-3 Jacobian is isogenous to the Weil
  restriction of a prime-order `E/F_{q^3}`, by any isogeny], which
  sharpens the remark of Tian 2020 §1 that covering maps from
  hyperelliptic curves cannot reach prime order. The census of every
  genus-3 hyperelliptic curve over `F_q`, `q ≤ 13`, finds `9,472`
  Res-shaped isomorphism classes of which only `331` carry the GHS
  compositum signature (a Frobenius-cyclic Klein four-group on the
  hyperelliptic line, Theorem 2): the isogeny relation is far wider than
  the covering construction. Whether any non-signature class carries a
  cover of small degree is open \[O\] (§6).
- #strong[No undiscovered affected curve type was found] inside the
  enumerated families; the one new structural fact is Theorem 3, and the
  one new place where a hidden descent can live in odd characteristic is
  `n = 5`. What remains open, and the next discriminating steps, are
  listed in §7.

== 1. Framework: exponents, margin, and what an auditor can do
<framework-exponents-margin-and-what-an-auditor-can-do>
For `E/F_{q^n}` with a descent target of genus `g` over `F_q` (exponents
in units of `log q`, constants dropped):

#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [quantity], [exponent], [tag and source],
  [Pollard rho on the prime-order subgroup],
  [`n/2`],
  [\[T\]],
  [index calculus on a genus-`g` Jacobian over `F_q`, double large
  primes],
  [`2 − 2/g` (`1` for a genus-3 plane quartic)],
  [\[H\] Gaudry–Thomé–Thériault–Diem; Diem–Thomé 2008; constants grow
  with `g`],
  [weak curves among the `≈ q^n` isomorphism classes],
  [`q^{n−δ}`],
  [\[E\] Parts A and B],
  [members of a random isogeny class],
  [`q^{n/2}`],
  [\[T\] Hasse–Weil; GHS 2002 §5],
  [weak members per class, uniform-distribution heuristic],
  [`q^{n/2−δ}`],
  [\[H\] GHS 2002 §5; Menezes–Teske 2006; Yee 2016 §5.2 (tested on
  `F_{2^18}`, `F_{2^24}`, `F_{2^30}`)],
  [auditor’s isogeny walk to a weak member, when one exists],
  [`min(δ, n/2)`],
  [\[H\]],
  [margin `a = δ − n/2`],
  [`> 0`: sparse, hidden; `< 0`: dense, weak field],
  [definition],
)
]

A hidden descent in Teske’s sense needs `a > 0` (the auditor’s walk
costs at least rho) together with `2 − 2/g < n/2` (the owner gains).
With `a < 0` and `2 − 2/g < n/2` the walk plus the descent beat rho for
every class that contains a weak member: Menezes–Teske–Weng’s weak
field, extended by Galbraith–Hess–Smart’s isogeny step. The auditor’s
answer in the sparse regime is not the walk but the direct test of the
published curve for every weak type (Part A’s magic numbers; Part B’s
branch types), which costs polynomial time and which the repository’s
`audit_curve` and `ghs_screen` already do for binary fields.

The exponent `2 − 2/g` is quoted, not measured; MMT 2002 Table 1 and
Appendix A give the Enge–Gaudry running times with constants for every
composite `N ∈ [100, 600]`, and this round does not redo that.

== 2. Sources read (exact locations)
<sources-read-exact-locations>
#align(center)[#table(
  columns: 3,
  align: (col, row) => (auto,auto,auto,).at(col),
  inset: 6pt,
  [source], [what it states (quoted or tightly paraphrased)], [class],
  [Teske, #emph[An elliptic curve trapdoor system], ePrint 2003/058
  (abstract; the PDF returns HTTP 403 here), J. Cryptology 19 (2006)
  115–133],
  ["a pair of elliptic curves over `F_{2^161}`": GHS "reduces the ECDLP
  in `E_s(GF(2^161))` to a hyperelliptic curve DLP in the Jacobian of a
  curve of genus 7 or 8, which is computationally feasible, but by far
  not trivial"; "`E_pb` is isogenous to `E_s`"; "the best attack on the
  ECDLP in `E_pb` is the parallelized Pollard rho method".],
  [T (abstract only)],
  [Galbraith–Hess–Smart, #emph[Extending the GHS Weil descent attack],
  Eurocrypt 2002 (author PDF, Hess’s page)],
  [§1: "One might 'hide' such a curve by taking an isogeny to a curve
  `E_1` for which the GHS attack is not effective. Knowledge of the
  'trapdoor'…" (the trapdoor idea predates Teske’s paper). §5:
  `p = min(1, s q^{t+1−n/2}/(nr))` weak classes per isogeny class;
  `n = 7`: "every curve over `F_{q^7}` can be attacked by reducing to a
  hyperelliptic curve of genus 7 or 8 over `F_q`" for `q ≥ 2^11`; table
  of `p` for `q = 2, …, 2^5`: `0.404, 0.286, 0.269, 0.286, 0.323`;
  `n = 31`: basic proportion `6q^{−25}`, extended `6q^{−9.5}/(31r)`,
  "one in `2^52`" for the IPsec curve; "for curves over large prime
  fields the techniques of Weil restriction do not apply".],
  [T; the per-class formula is H],
  [Hess, #emph[The GHS attack revisited], Eurocrypt 2003 (author PDF)],
  [§4: `m = d` from `b = (αβ)^2`, genus `2^d − 1`, about `s q^{2d−1}/2`
  curves; per-class proportion `min{1, s q^{2d−1}/(2 q^{n/2} n r)}`;
  `F_{2^155}`: `2^{−38}` against `2^{−52}`; the search costs `≈ 2^39`
  operations. §5.2 (towers) and §5.3 (descent from extensions: "for
  prime `n` this approach will however not give an improvement").],
  [T; proportions H],
  [Hess, #emph[Weil descent attacks] (survey, author PDF), Thm 30 (from
  Diem 2003) and Table 3],
  [Odd characteristic, prime `n`: `g ≥ 2^{φ_2(n)−2}(n−4)+1`; "For
  `n = 5` and `n = 7` there are families … for which `g` assumes the
  lower bounds 5 and 7"; `n = 11, 13, 17, 19, 23`:
  `g ≥ 1793, 9217, 833, 983041, 9729`; "The attack is not feasible for
  prime `n ≥ 11`". Table 3 lists the `n = 5` family
  `Y^2 = ∏_{i∈{0,1,2,3}}(X − σ^i(a))` and the `n = 7` family
  `∏_{i∈{0,1,2,4}}`.],
  [T],
  [Maurer–Menezes–Teske, LMS JCM 5 (2002) (Cambridge PDF)],
  [Table 1:
  `N = 161, n = 7, l = 23, m = 4, g = 7, I = 94, ρ = 80, T = 34`;
  `N = 165, n = 15, m = 5, g = 15, I = 57`;
  `N = 170, n = 5, m = 5, g = 15, I = 171`. Remark 20: "only `≈ 2^94`
  out of `≈ 2^162` elliptic curves over `F_{2^161}` have magic number
  `m = 4` relative to `n = 7`"; finding an isogenous `m = 5` curve over
  `F_{2^165}` "may be difficult, as one has essentially to search
  through the entire set of `2^57` curves". §7: the random curve E161-2
  has `(m, g) = (7, 64)` for `(n, l) = (7, 23)`.],
  [T],
  [Menezes–Teske, #emph[Cryptographic implications of Hess’ generalized
  GHS attack], ePrint 2004/235 (abstract)],
  ["`GF(q^7)` are potentially partially weak … for half of all elliptic
  curves over `GF(q^7)`, namely those curves `E` for which `#E` is
  divisible by 4"; `GF(q^3)` partially weak, `GF(q^6)` potentially weak,
  `GF(q^8)` potentially partially weak; `N` not divisible by 3, 5, 6, 7,
  8: not weak.],
  [T (abstract)],
  [Menezes–Teske–Weng, #emph[Weak fields for ECC], CT-RSA 2004 (ePrint
  2003/128, abstract)],
  [fields such as `GF(2^210)` are weak: "all instances … can be solved
  in significantly less time" than rho.],
  [T (abstract)],
  [Karabina–Menezes–Pomerance–Shparlinski, CACR 2010-03],
  [eq. (4): gGHS genus `g = 2^t − 2^{t−r} − 2^{t−s} + 1`; Thm 9: for all
  but `o(q^n)` of the `b`, `g ≥ 2^{n/2+o(n)}`; "the gGHS attack can be
  useful for at most `q^{16 log_2 n + O(1)}` elliptic curves"; with
  isogenies "at most `q^{n/2+O(log n)}` … negligible".],
  [T],
  [Yee, #emph[On the effectiveness of isogeny walks for extending cover
  attacks], MMath thesis, Waterloo 2016 (PDF)],
  [§5.2: weak orbit classes per isogeny class `s q^{t+1−n/2}/(nr)`;
  `n = 3`: `2(q^2−1)` weak curves, `√q/(nr)` per class, "around half of
  the ordinary isogeny classes will not contain any weak curves"; §6.2:
  `F_{2^150}` "partially weak" (walk `2^50` steps, `2^9` faster than rho
  per step); §7: for odd characteristic "an effective method for
  discerning whether a curve is weak will be needed in order to utilize
  the isogeny walk in a meaningful way".],
  [T, H as labelled],
  [Diem, Compositio 147 (2011) (Cambridge PDF)],
  [Prop. 4.10: `g(C) ≤ (2^n − 1)(2n − 1)` under Condition 2.7; "Let `q`
  be odd. In this case `k(E)|k(x)` is (tamely) ramified at 4 places."],
  [T],
  [Chao (with Momose), ECC 2007 slides (pdf)],
  [sl. 20–21: `(2,…,2)` coverings with `g = d g_0` exist in odd
  characteristic only for `(d, n) = (2,2)`, `(5,4)`, `d = 2^n − 1`, and
  decomposable `(3,3)`; densities `Θ(q^2)` for `d = 3`, `Θ(q^2)` for
  `(5,4)`, `Θ(q^{dℓ−3})?` for `d = 2^n − 1`; sl. 24 (hyperelliptic
  genus-3 family, degree 8), sl. 25 (non-hyperelliptic Types 1–2, degree
  4), sl. 33: Type I `#{λ} ≈ q^3/2`.],
  [T; densities marked "?" are H],
  [Iijima–Momose–Chao, ePrint 2013/487 and 2015/805 (abstracts)],
  [complete classification of `(2,…,2)`-coverings under the isogeny
  condition `g(C) = d·g(C_0)` for elliptic and genus-2, 3 hyperelliptic
  `C_0`; "17 classes are newly found"; for elliptic `C_0` over `k_d`
  with `d = 2, 3, 5, 7`.],
  [T (abstracts)],
  [Joux–Vitse, Eurocrypt 2012 (author PDF)],
  [§3: two isogeny-walk strategies; §4.1: the family (3), `Θ(q^2)`
  curves, walk `≈ q` steps, "we conjecture that all curves with
  cardinality divisible by 4 are vulnerable"; (4): non-hyperelliptic
  covers for "about half of the curves having their full 2-torsion
  defined over `F_{q^3}`"; §5: `F_{p^4}`, "approximately 3/4 of the
  elliptic curves … are directly vulnerable" to a genus-2 cover over
  `F_{p^2}`, most admit a genus-9 cover over `F_p`; App. C: `n = 6`,
  genus 9 for "at most `1/q^3`" of the curves.],
  [T; conjecture H],
  [Tian, #emph[Cover attacks for elliptic curves with prime order],
  arXiv 2012.07173 (2020; J. Cryptology 36 (2023))],
  [abstract: an `F_q`-rational `(ℓ,ℓ,ℓ)`-isogeny from `Res(E)` to the
  Jacobian of a genus-3 curve, then index calculus, `Õ(q)` "in some
  elliptic curve groups of prime order over `F_{q^3}`"; §1: "a necessary
  condition is that the elliptic curve has three rational points of
  order 2 \[3,13\]"; "\[19\] shows that `C` needs to be
  non-hyperelliptic if the elliptic curve has prime order"; the isogeny
  needs an `F_q`-rational maximal isotropic `Λ ⊂ W[ℓ]` not containing
  subgroups of `E, E^σ, E^{σ^2}`; example over `F_{523^3}` with prime
  order.],
  [T],
  [Gorla–Massierer, arXiv 1405.1059],
  [trace-zero index calculus `Õ(q^{2−2/(n−1)})` for `E/F_q` seen over
  `F_{q^n}`, "lower than that of generic attacks … for `n ≥ 5`";
  feasible for `n = 3` up to 30-bit `q`; for `n = 5` the Gröbner step is
  the bottleneck. A class-invariant attack (it depends only on the
  group), so not a trapdoor.],
  [T],
  [Arita, Asiacrypt 2000 (abstract, Springer page)],
  [some curves over `F_{3^n}` of composite degree are weaker than rho
  via a `C_ab` curve on the Weil restriction.],
  [T (abstract)],
  [Arita–Matsuo–Nagao–Shimura, IEICE E89-A (2006) (abstract)],
  [quartic extensions of odd characteristic: reduce to a Scholten form,
  then genus 2 over `F_{q^2}`, then a `C_ab` model over `F_q`; "many"
  such curves.],
  [T (abstract)],
)
]

Not obtained (HTTP 403 or paywall): the ePrint PDFs of Teske 2003/058,
Menezes–Teske 2004/235, MTW 2003/128, Momose–Chao 2009/236,
Iijima–Momose–Chao 2009/613, 2013/487, 2015/805, the IACR archive PDFs
of GHS 2002 and Hess 2003 (the author copies were used instead), Diem’s
JRMS 2003 paper, Thériault’s JRMS 2003 paper, Scholten’s preprint (404).
Nothing below rests on their bodies except as reported by the papers
that cite them.

== 3. Prime fields: no descent variant \[T\]
<prime-fields-no-descent-variant-t>
Let `E/F_p` with a prime-order subgroup of size `r ≈ p`. Rho costs
`p^{1/2}`.

- #strong[No Weil restriction.] `F_p` has no proper subfield, so there
  is no extension `K/k` to descend through: the GHS, gGHS, Kummer and
  Artin–Schreier constructions all start from `[K : k] = n ≥ 2` (GHS
  2002 §5; Hess survey §5).
- #strong[Same-field covers do not pay.] A cover `C → E` over `F_p` with
  `C` of genus `g ≥ 2` (the repository’s `COVERS.md` constructs genus-2
  ones for every `E/F_p`) moves the problem into `Jac(C)(F_p)` of size
  `≈ p^g`. Index calculus there costs `Õ(p^{2−2/g}) ≥ Õ(p)` (double
  large primes; Diem’s plane-curve `Õ(p^{2−2/(d−2)})` is `≥ Õ(p)` as
  well), above `p^{1/2}`. Genus 1 covers are isogenies and change
  nothing.
- #strong[Base change then descend does not pay.] Over `F_{p^k}` the
  subgroup is still of size `p` inside `E(F_{p^k})` of size `p^k`; any
  descent to a Jacobian over `F_p` of genus `≥ k` lands in a group of
  size `≥ p^k`, with the same bound. `JACOBIAN_TRANSFERS.md` prices this
  route at the most favourable genus and finds no `formula_advantage`
  row for a prime-field curve.
- #strong[What varies inside a prime-field isogeny class.] The trace,
  the group order, the embedding degree and anomaly are class invariants
  and cannot be hidden. The endomorphism ring and the rational torsion
  structure vary with the position in the volcano, but the only ECDLP
  effect known is the automorphism factor `√#Aut`, visible on the curve
  itself (`j ∈ {0, 1728}`).
  `research/nist_trapdoor_slot_program_20261011/` (hypothesis H1d)
  already records that the isogeny-hiding #emph[asymmetry] exists over
  `F_p` while no weak class exists to hide.

So the Teske pattern has no prime-field variant through descent; the
repository’s P-256 assessment stands. \[O\] A weakness over `F_p` that
is neither a class invariant nor a cover would be new mathematics; none
is proposed here.

#figure([#image("figures/taxonomy.svg")],
  caption: [
    Taxonomy of the three ingredients by field type
    (figures/taxonomy.svg).
  ]
)

#figure([#image("figures/margins.svg")],
  caption: [
    Margins by n (figures/margins.svg, from binary\_summary.csv and
    oddchar\_types.csv).
  ]
)

== 4. Binary fields: the exact map of magic numbers (Part A) \[E, T\]
<binary-fields-the-exact-map-of-magic-numbers-part-a-e-t>
For odd `n` the irreducible factors of `x^n − 1` over `F_2` are the
cyclotomic cosets of 2 modulo `n`; a product `F` of them with
`lcm(orders) = n` is the annihilator of a `b` that lies in no proper
subfield, the number of such `b` is exactly `q^{deg F}` (`q = 2^l`), and
the magic number and genus are `m = deg F + [x+1 ∤ F]`,
`g = 2^{m−1} − [x+1 ∤ F]` (GHS Thm 1; MMT Thm 5). Hess’s gGHS pairs
`(F_α, F_β)` give genus `2^t − 2^{t−r} − 2^{t−s} + 1` from `≈ q^{r+s−1}`
values of `b`. The tool enumerates all of this for `3 ≤ n ≤ 99`
(`binary_types.csv`, `binary_gghs.csv`, `binary_summary.csv`) and
reproduces, as unit tests, the GHS 2002 per-class table for `n = 7`,
Hess’s `2^{−52}` and `2^{−38}` at `n = 31`, and MMT’s counts `2^94`,
`2^57`, `2^171` for `N = 161, 165, 170` (protocol checks 1–4).

#strong[Margins of the smallest-genus descent] (`δ = n − deg F`; margin
`a = δ − n/2`; `binary_summary.csv`):

#align(center)[#table(
  columns: 8,
  align: (col, row) => (right,right,left,right,left,right,left,right,).at(col),
  inset: 6pt,
  [`n`], [`ord_n(2)`], [type I: `(deg, g, δ)`], [`a_I`], [type II:
  `(deg, g, δ)`], [`a_II`], [gGHS `(r+s, g, δ)`], [`a_gGHS`],
  [3],
  [2],
  [(2, 3, 1)],
  [−0.5],
  [(3, 4, 0)],
  [−1.5],
  [(2+2, 3, 0)],
  [−1.5],
  [5],
  [4],
  [(4, 15, 1)],
  [−1.5],
  [(5, 16, 0)],
  [−2.5],
  [(1+4, 15, 1)],
  [−1.5],
  [#strong[7]],
  [3],
  [#strong[(3, 7, 4)]],
  [#strong[+0.5]],
  [(4, 8, 3)],
  [−0.5],
  [(3+3, 7, 2)],
  [−1.5],
  [9],
  [6],
  [(6, 63, 3)],
  [−1.5],
  [(7, 64, 2)],
  [−2.5],
  [(1+6, 63, 3)],
  [−1.5],
  [11],
  [10],
  [(10, 1023, 1)],
  [−4.5],
  [(11, 1024, 0)],
  [−5.5],
  [—],
  [−4.5],
  [13],
  [12],
  [(12, 4095, 1)],
  [−5.5],
  [(13, 4096, 0)],
  [−6.5],
  [—],
  [−5.5],
  [#strong[15]],
  [4],
  [#strong[(4, 15, 11)]],
  [#strong[+3.5]],
  [(5, 16, 10)],
  [+2.5],
  [(4+4, 15, 8)],
  [+0.5],
  [17],
  [8],
  [(8, 255, 9)],
  [+0.5],
  [(9, 256, 8)],
  [−0.5],
  [(8+8, 255, 2)],
  [−6.5],
  [19],
  [18],
  [(18, 2^18−1, 1)],
  [−8.5],
  [(19, 2^18, 0)],
  [−9.5],
  [—],
  [−8.5],
  [#strong[21]],
  [6],
  [#strong[(5, 31, 16)]],
  [#strong[+5.5]],
  [(6, 32, 15)],
  [+4.5],
  [#strong[(2+3, 21, 17)]],
  [#strong[+6.5]],
  [23],
  [11],
  [(11, 2047, 12)],
  [+0.5],
  [(12, 2048, 11)],
  [−0.5],
  [(11+11, 2047, 2)],
  [−9.5],
  [25],
  [20],
  [(20, 2^20−1, 5)],
  [−7.5],
  [(21, 2^20, 4)],
  [−8.5],
  [—],
  [−7.5],
  [27],
  [18],
  [(18, 2^18−1, 9)],
  [−4.5],
  [(19, 2^18, 8)],
  [−5.5],
  [—],
  [−4.5],
  [#strong[31]],
  [5],
  [#strong[(5, 31, 26)]],
  [#strong[+10.5]],
  [(6, 32, 25)],
  [+9.5],
  [(5+5, 31, 22)],
  [+6.5],
  [33],
  [10],
  [(10, 1023, 23)],
  [+6.5],
  [(11, 1024, 22)],
  [+5.5],
  [(10+10, 1023, 14)],
  [−2.5],
  [#strong[35]],
  [12],
  [(7, 127, 28)],
  [+10.5],
  [(8, 128, 27)],
  [+9.5],
  [#strong[(3+4, 105, 29)]],
  [#strong[+11.5]],
  [45],
  [12],
  [(10, 1023, 35)],
  [+12.5],
  [(11, 1024, 34)],
  [+11.5],
  [(4+6, 945, 36)],
  [+13.5],
  [63],
  [6],
  [(6, 63, 57)],
  [+25.5],
  [(7, 64, 56)],
  [+24.5],
  [(6+6, 63, 52)],
  [+20.5],
  [93],
  [10],
  [(7, 127, 86)],
  [+39.5],
  [(8, 128, 85)],
  [+38.5],
  [(2+5, 93, 87)],
  [+40.5],
)
]

Reading. (i) For prime `n`, `deg F ≥ ord_n(2)` and the sparse regime
(`a > 0`) coincides with a genus that is useless (`n = 17, 23`: genus
255, 2047), except `n = 7` (genus 7) and `n = 31` (genus 31): Teske’s
field and Hess’s `F_{2^155}`. (ii) For composite `n` a product of
#emph[non-primitive] factors can beat the primitive one: `n = 21`
reaches `m = 6` (genus 31) with `(x^2+x+1)(x^3+x+1)` instead of `m = 7`
from an order-21 sextic, and Hess’s pairing lowers it to genus 21;
`n = 35, 45, 93, 99` behave alike (check 4). (iii) `n = 3, 5` have
`δ ≤ 1` for every variant: the descent is public and the field is weak
(MTW 2004; Menezes–Teske 2006 "`GF(q^3)` partially weak"). (iv) At
`n = 7` the three variants split: type I (genus 7, `δ = 4`) is sparse,
type II (genus 8, `δ = 3`) and gGHS (genus 7, `δ = 2`) are dense. The
GHS 2002 table (`p = 0.404, …, 0.323` for `q = 2, …, 2^5`) and the
"`q ≥ 2^11`: every class" statement are the type-II numbers;
Menezes–Teske’s "half of all curves" (`#E ≡ 0 mod 4`) is the gGHS
number.

#strong[Bits for Teske’s and the literature’s fields]
(`binary_bits.csv`; rho `N/2`; owner `(2 − 2/g)·l`; walk
`min(δ, n/2)·l`; margin `(δ − n/2)·l`; "per class" \= expected weak
Frobenius-orbit classes per isogeny class, capped at 1):

#align(center)[#table(
  columns: 10,
  align: (col, row) => (left,left,right,right,right,right,right,right,right,right,).at(col),
  inset: 6pt,
  [`N = n·l`], [variant], [`g`], [`δ`], [rho], [owner], [walk],
  [margin], [`log2 #weak`], [per class],
  [161 \= 7·23],
  [GHS-I],
  [7],
  [4],
  [80.5],
  [39.4],
  [80.5],
  [+11.5],
  [71],
  [4.3e−6],
  [161 \= 7·23],
  [GHS-II],
  [8],
  [3],
  [80.5],
  [40.2],
  [69.0],
  [−11.5],
  [94],
  [1],
  [161 \= 7·23],
  [gGHS 3+3],
  [7],
  [2],
  [80.5],
  [39.4],
  [46.0],
  [−34.5],
  [117],
  [1],
  [161 \= 7·23],
  [generic],
  [64],
  [0],
  [80.5],
  [45.3],
  [0],
  [−80.5],
  [162],
  [1],
  [161 \= 23·7],
  [GHS-I],
  [2047],
  [12],
  [80.5],
  [14.0],
  [80.5],
  [+3.5],
  [79],
  [1.1e−3],
  [155 \= 5·31],
  [GHS-I],
  [15],
  [1],
  [77.5],
  [57.9],
  [31.0],
  [−46.5],
  [125],
  [1],
  [155 \= 5·31],
  [generic],
  [16],
  [0],
  [77.5],
  [58.1],
  [0],
  [−77.5],
  [156],
  [1],
  [155 \= 31·5],
  [GHS-II],
  [32],
  [25],
  [77.5],
  [9.7],
  [77.5],
  [+47.5],
  [33.6],
  [1.9e−16],
  [155 \= 31·5],
  [gGHS 5+5],
  [31],
  [22],
  [77.5],
  [9.7],
  [77.5],
  [+32.5],
  [48.6],
  [6.4e−12],
  [165 \= 5·33],
  [GHS-I],
  [15],
  [1],
  [82.5],
  [61.6],
  [33.0],
  [−49.5],
  [133],
  [1],
  [165 \= 15·11],
  [GHS-I],
  [15],
  [11],
  [82.5],
  [20.5],
  [82.5],
  [+38.5],
  [46],
  [3.1e−14],
  [165 \= 15·11],
  [GHS-II],
  [16],
  [10],
  [82.5],
  [20.6],
  [82.5],
  [+27.5],
  [57],
  [6.4e−11],
  [168 \= 7·24],
  [GHS-I],
  [7],
  [4],
  [84.0],
  [41.1],
  [84.0],
  [+12.0],
  [74],
  [2.9e−6],
  [168 \= 21·8],
  [gGHS 2+3],
  [21],
  [17],
  [84.0],
  [15.2],
  [84.0],
  [+52.0],
  [35],
  [5.3e−18],
  [210 \= 5·42],
  [GHS-I],
  [15],
  [1],
  [105],
  [78.4],
  [42.0],
  [−63.0],
  [169],
  [1],
  [210 \= 7·30],
  [GHS-I],
  [7],
  [4],
  [105],
  [51.4],
  [105],
  [+15.0],
  [92],
  [2.9e−7],
  [210 \= 7·30],
  [GHS-II],
  [8],
  [3],
  [105],
  [52.5],
  [90.0],
  [−15.0],
  [122],
  [1],
  [210 \= 15·14],
  [GHS-I],
  [15],
  [11],
  [105],
  [26.1],
  [105],
  [+49.0],
  [58],
  [1.7e−17],
  [210 \= 21·10],
  [gGHS 2+3],
  [21],
  [17],
  [105],
  [19.0],
  [105],
  [+65.0],
  [43],
  [5.2e−22],
  [300 \= 15·20],
  [GHS-I],
  [15],
  [11],
  [150],
  [37.3],
  [150],
  [+70.0],
  [82],
  [5.6e−24],
  [600 \= 15·40],
  [GHS-I],
  [15],
  [11],
  [300],
  [74.7],
  [300],
  [+140],
  [162],
  [2.4e−45],
)
]

What the table says, and what it does not. \[E\] At `N = 161` the two
genera of Teske’s abstract are two different auditor problems: a planted
genus-7 (type I) curve costs the auditor a walk as long as rho (the
class holds `4·10^{−6}` such orbit classes on average, so one must be
planted), while a genus-8 (type II) curve is reached by a `2^69`-step
walk, the figure MMT 2002 Remark 20 and Teske’s paper discuss; the gGHS
genus-7 route covers half of all classes outright (Menezes–Teske 2006).
\[H\] The owner’s `39–40` bits are the formula `(2 − 2/g)·l`; MMT’s
Enge–Gaudry estimate for the same genus is `T = 34` bits (Table 1), the
same order. \[E\] At `N = 155` the public `n = 5` descent (genus 15/16
over `F_{2^31}`, `58` bits) already sits below rho for every curve, and
the sparse `n = 31` descents (genus 31–32 over `F_{2^5}`) are the ones
GHS 2002 searched 31 days for and did not find for the IPsec curve;
Hess’s `2^{−38}` per class is the gGHS row. \[E\] For
`N = 165, 210, 300, 600` the `n = 15` type-I descent (genus 15 over
`F_{2^{N/15}}`) is sparse by `27–140` bits while the public `n = 5`
descent is already below rho: an auditor who only prices the public
route misses the sparse one by `40–150` bits of owner advantage. \[O\]
Nothing here is a running time; genus `≥ 100` rows are listed for
completeness and are infeasible by constants (MMT’s cutoff `gl ≥ 4097`
kills none of the rows above, but their Table 1 shows how fast `T` grows
with `g`). The auditor’s remedy in every sparse row is the direct
magic-number test of the published curve for every factorisation
`N = n·l` (`audit_curve`, `ghs_screen`), which costs polynomial time and
which the sparsity of the weak set does not help to evade.

== 5. Odd characteristic: theorems used (Theorems 1–3)
<odd-characteristic-theorems-used-theorems-13>
=== Theorem 1 (weak-set size of a branch type) \[H, checked E\]
<theorem-1-weak-set-size-of-a-branch-type-h-checked-e>
Fix `n ≥ 2` and a branch type `τ`: a `σ`-stable set `S ⊂ P^1(F̄_q)` that
is a union of `σ`-orbits of sizes `d_1, …, d_k`, `Σ d_j = r`, together
with the positions of a 4-subset `B` whose translates `σ^i B` cover `S`.
The branch sets of type `τ` are obtained by choosing one point of
`P^1(F_{q^{d_j}})` of exact degree `d_j` per orbit, so there are
`≈ ∏ q^{d_j} = q^r` of them (exactly, up to the `∏ d_j` rotations and
the point `∞`). The group `PGL_2(F_q)` (order `q^3 − q`) acts on
`P^1(F̄_q)` commuting with `σ`, so it maps branch sets of type `τ` to
branch sets of type `τ` and the cross-ratio class (the `j`-invariant of
the double cover branched at `B`) is constant on its orbits. A generic
stabiliser is trivial, so the number of `j ∈ F_{q^n}` carrying a
degree-2 function of type `τ` is `≈ c_τ q^{r−3}` with `c_τ` bounded, and
a random curve over `F_{q^n}` is weak of type `τ` with probability
`≈ c_τ q^{r−3−n}`, i.e. `δ = n + 3 − r`. The literature’s densities are
the cases `n = 2, r = 5` (all curves: Scholten), `n = 3, r = 5`
(`Θ(q^2)`: Momose–Chao), `n = 5, r = 5` (`Θ(q^2)`: Chao 2007 sl. 21),
`n = 6, r = 6` (`≤ 1/q^3`: Joux–Vitse App. C) and `n = 7, r = 7`
(`Θ(q^4)`: Chao’s `Θ(q^{dℓ−3})`). Part B counts the weak `j` exactly and
finds the exponent `r − 3` on every type (§5).

=== Theorem 2 (the GHS compositum signature) \[T, proved here\]
<theorem-2-the-ghs-compositum-signature-t-proved-here>
Let `n = 3`, `q` odd, and let `C/F_q` be the GHS compositum of a branch
type with `(m, r) = (3, 5)`: `S = {s_0, s_1, s_2} ∪ {t_1, t_2}` with
`{s_i}` a 3-orbit, `{t_1, t_2}` `σ`-stable, `B_i = S ∖ {s_i}`. Write
`K = F_{q^3}`, `L = K(x, y_0, y_1, y_2)` with `y_i^2 = f_i(x)`, `f_i`
the quartic with root set `B_i`, and `τ_i` the sign change of `y_i`.
Then:

#block[
#set enum(numbering: "(a)", start: 1)
+ `ι = τ_0τ_1τ_2` is the hyperelliptic involution of `C_K`, and `C/⟨ι⟩`
  is a conic `D`. #emph[Proof.] The fixed field of `⟨ι⟩` is
  `K(x, y_0y_1, y_0y_2)`, a `(Z/2)^2`-cover of `P^1_x` branched over
  `B_0 △ B_1 = {s_0, s_1}` and `B_0 △ B_2 = {s_0, s_2}`, three points in
  all: genus `1 + 2^0 (3 − 4) = 0` by the formula of Theorem 1 of the
  previous study. A genus-3 curve with a genus-0 quotient by an
  involution is hyperelliptic with that involution. ∎

+ The reduced automorphism group of `C_K` contains the Klein four-group
  `V = (Z/2)^3/⟨ι⟩`, acting on the hyperelliptic line `D ≅ P^1_u`, with
  quotient `P^1_x` and branch points `s_0, s_1, s_2` (inertia `τ_jτ_k`
  at `s_i`); the eight Weierstrass points of `C` are the two fibres of
  `D → P^1_x` over `t_1, t_2`, where the inertia is `⟨ι⟩`. In a
  `V`-adapted coordinate `u` on `D` (`V = {u, −u, 1/u, −1/u}`, quotient
  `w = u^2 + u^{−2}`) the Weierstrass octic is
  `(u^4 − w_1 u^2 + 1)(u^4 − w_2 u^2 + 1)`, so the Weierstrass set is a
  union of two free `V`-orbits. The `τ_i` are `K`-rational and are
  permuted cyclically by the `q`-Frobenius `σ` (`σ: y_i ↦ y_{i+1}`), so
  `V` is `σ`-stable and `σ` acts on its three involutions as a 3-cycle.

+ Consequently the Frobenius cycle type on the Weierstrass points is
  `(1,1,3,3)` when `σ` fixes each `V`-orbit (an affine map of a
  `V`-torsor of order 3 in `S_4` is a 3-cycle with one fixed point) and
  `(2,6)` when `σ` swaps the two orbits. The previous study recorded
  exactly these two cycle types on all 81 Joux–Vitse instances. ∎
]

The census tests (b) directly: it finds every `F_{q^3}`-rational
involution of `P^1` preserving the Weierstrass set (from the images of
three roots), looks for a `σ`-stable Klein four-group among them whose
involutions `σ` permutes cyclically and which has no fixed point in the
Weierstrass set. On all 12 Joux–Vitse controls (`p = 5, 7, 11, 13`,
three arms) the signature is found and the cycle type is as stated (test
`controls_jv_family`).

=== Theorem 3 (Weil-restriction-shaped hyperelliptic genus-3 Jacobians)
\[T, proved here\]
<theorem-3-weil-restriction-shaped-hyperelliptic-genus-3-jacobians-t-proved-here>
Let `C/F_q` be a hyperelliptic curve of genus 3, `q` odd, and
`E/F_{q^3}` of trace `t`. If `Jac(C)` is isogenous over `F_q` to
`Res_{F_{q^3}/F_q} E` (by an isogeny of any degree), then `t` is even,
and the `q`-Frobenius permutes the eight Weierstrass points of `C` with
cycle type `(1,1,3,3)`, `(2,3,3)`, `(1,1,6)` or `(2,6)`. In particular
`#E(F_{q^3}) = q^3 + 1 − t` is even: #strong[no hyperelliptic genus-3
Jacobian over `F_q` is isogenous to the Weil restriction of an elliptic
curve of prime order over `F_{q^3}`.]

#emph[Proof.] Isogenous abelian varieties have the same characteristic
polynomial of Frobenius, `P(T) = T^6 − tT^3 + q^3`, and its reduction
modulo 2 is the characteristic polynomial of Frobenius on the 2-torsion
`A[2] = T_2(A)/2T_2(A)`. For a hyperelliptic curve with Weierstrass set
`W` (`|W| = 8`), `Jac(C)[2]` is the Galois module of even-cardinality
subsets of `W` modulo `W` itself, a subquotient of the permutation
module `F_2[W]`; the characteristic polynomial of Frobenius on `F_2[W]`
is `∏_i (T^{c_i} + 1)` over the cycle lengths `c_i`, and the two trivial
factors removed by passing to even subsets and modulo `W` give
`χ_{Jac[2]}(T) = ∏_i (T^{c_i} + 1) / (T + 1)^2`. If `t` is odd,
`P(T) ≡ T^6 + T^3 + 1 = Φ_9(T) (mod 2)`, irreducible of degree 6 (2 has
order 6 modulo 9), and `Φ_9 | T^c + 1` requires `9 | c`, impossible with
`c ≤ 8`. So `t` is even and
`P(T) ≡ T^6 + 1 = (T + 1)^2 (T^2 + T + 1)^2`. Over `F_2`,
`T^c + 1 = ∏_{d|c} Φ_d` with `Φ_1 = Φ_2 = T + 1`, `Φ_3 = T^2 + T + 1`,
`Φ_4 = (T+1)^2`, `Φ_6 = Φ_3`, `Φ_8 = (T+1)^4`, and `Φ_5`, `Φ_7` of
degrees 4 and 6 (products of irreducibles different from `Φ_3`). Hence
`∏_i (T^{c_i} + 1) = (T+1)^4 (T^2+T+1)^2` forbids cycles of length 5 and
7, needs `Φ_3` exactly twice (two 3-cycles, or one 6-cycle), and needs
`(T+1)` with multiplicity 4: with two 3-cycles the remaining two points
form `(1,1)` or `(2)`; with a 6-cycle likewise. That leaves exactly the
four cycle types stated. ∎

The census finds #strong[only] these four cycle types among all
Res-shaped classes at `q = 3, 5, 7, 11, 13` (`census_patterns.csv`), and
no class with `#E` prime; the GHS compositums of Theorem 2 occupy the
`(1,1,3,3)` and `(2,6)` columns, never `(2,3,3)` or `(1,1,6)`. Theorem 3
gives an auditor a two-line necessary condition (even trace; cycle type)
for any claim that a hyperelliptic genus-3 Jacobian over `F_q` is a
Weil-restriction target, and it says that Tian’s route
(`(ℓ,ℓ,ℓ)`-isogenies for prime order) must end on non-hyperelliptic
curves, for every isogeny and not only for covering maps.

== 5b. Odd characteristic: exact weak-curve and class counts (Part B)
\[E\]
<b.-odd-characteristic-exact-weak-curve-and-class-counts-part-b-e>
Setting and notation as in §1 and Theorem 1
(`q = p ∈ {3, 5, 7, 11, 13}`, `n = 2..7`, orbit sizes and bounds as
frozen in the protocol; `oddchar_types.csv`, `oddchar_classes.csv`,
`oddchar_checks.txt` of run `20261018T0630Z`). Every weak `j` lies in
`F_{q^n}`, every trace is in the Hasse range, the all-`j` tables have
exactly `q^n` entries, and the `n = 3` `(3,5)` curves have
`#E ≡ 0 (mod 4)` on both twists (checks 5–6, no violation at any
`(q, n)`).

#strong[Types found] (check 7). Per `n` the types with genus `≤ g_max`
and period `n`, as `(orbit sizes: m, r, g)`: `n = 2`: `2+1+1+1`,
`2+2+1`, `3+2` (2, 5, 2); `2+2+1+1`, `2+2+2` (2, 6, 3). `n = 3`:
`3+1+1`, `3+2` (3, 5, 3); `3+3`, `6` (2, 6, 3); `3+1+1+1`, `3+2+1`,
`3+3` (3, 6, 5). `n = 4`: `4+1` (4, 5, 5); `4+1+1`, `4+2` (3, 6, 5).
`n = 5`: `5` (4, 5, 5) only. `n = 6`: `3+2+1`, `6` (4, 6, 9). `n = 7`:
`7` (3, 7, 7). These are exactly the `(m, r, g)` triples the previous
study’s `types.csv` lists at the minimum for each `n` (`m2r5`;
`m2r6, m3r5`; `m3r6, m4r5`; `m4r5`; `m4r6`; `m3r7`), and the `n = 3`
list is Momose–Chao’s: `3+1+1`/`3+2` are the hyperelliptic family (3),
`3+3`/`6` their Types 1 and 2 (degree-4 non-hyperelliptic).

#strong[Weak-`j` counts against `q^{r−3}`]
(`figures/oddchar_counts.svg`). The ratio `weak_j / q^{r−3}` at
`q = 7, 11, 13`:

#align(center)[#table(
  columns: 7,
  align: (col, row) => (right,left,right,right,right,right,right,).at(col),
  inset: 6pt,
  [`n`], [type `(m, r, g)`], [`q = 7`], [`q = 11`], [`q = 13`],
  [`δ = n + 3 − r`], [`a = δ − n/2`],
  [2],
  [`2+1+1+1` (2,5,2)],
  [0.143],
  [0.157],
  [0.154],
  [0],
  [−1],
  [2],
  [`3+2` (2,5,2)],
  [0.286],
  [0.306],
  [0.308],
  [0],
  [−1],
  [3],
  [`3+1+1` (3,5,3)],
  [0.510],
  [0.496],
  [0.485],
  [1],
  [−0.5],
  [3],
  [`3+2` (3,5,3)],
  [0.429],
  [0.455],
  [0.462],
  [1],
  [−0.5],
  [3],
  [`3+3` (2,6,3)],
  [0.169],
  [0.166],
  [0.165],
  [0],
  [−1.5],
  [3],
  [`6` (2,6,3)],
  [0.169],
  [0.166],
  [0.167],
  [0],
  [−1.5],
  [3],
  [`3+2+1` (3,6,5)],
  [0.490],
  [0.496],
  [0.497],
  [0],
  [−1.5],
  [4],
  [`4+1` (4,5,5)],
  [1.000],
  [1.041],
  [1.018],
  [2],
  [0],
  [4],
  [`4+1+1` (3,6,5)],
  [0.499],
  [0.515],
  [0.505],
  [1],
  [−1],
  [4],
  [`4+2` (3,6,5)],
  [0.429],
  [0.455],
  [0.462],
  [1],
  [−1],
  [#strong[5]],
  [#strong[`5` (4,5,5)]],
  [#strong[1.020]],
  [#strong[1.000]],
  [#strong[1.006]],
  [#strong[3]],
  [#strong[+0.5]],
  [6],
  [`3+2+1` (4,6,9)],
  [0.980],
  [0.992],
  [0.994],
  [3],
  [0],
  [6],
  [`6` (4,6,9)],
  [0.974],
  [0.994],
  [0.993],
  [3],
  [0],
  [7],
  [`7` (3,7,7)],
  [1.992 (`q=7`), 2.005 (`q=5`), 1.815 (`q=3`)],
  [],
  [],
  [3],
  [−0.5],
)
]

Every ratio is within a factor of 1.2 of itself across the three largest
`q` (the decision rule asked for a factor 3): #strong[H\_B holds for
every type enumerated], with constants `c_τ` between `0.14` and `2.0`.
The exact counts at `q = 13`: `n = 5` has `170` weak `j` among `371293`
(`1/q^3`); `n = 3` has `82 + 78` hyperelliptic-family and `363 + 366`
Type 1/2 weak `j` among `2197` (about `0.4 q^2`, Chao’s `≈ q^3/2`
Legendre forms counted per `j` and per type); `n = 6` has `2184 + 2181`
among `4826809` (`≈ q^3`, Joux–Vitse’s "at most `1/q^3`" met with
constant `1`); `n = 7` at `q = 7` has `4782` among `823543` (`2 q^4`).

#strong[Isogeny-class statistics] (`oddchar_classes.csv`, all-`j` trace
tables for `q^{2n} ≤ 2·10^11`; `figures/oddchar_classes.svg`). Share of
all curves over `F_{q^n}` that lie in a class containing a weak curve of
the type, and the mean number of weak curves per covered class:

#align(center)[#table(
  columns: 6,
  align: (col, row) => (right,left,right,right,right,left,).at(col),
  inset: 6pt,
  [`n`], [type], [share `q = 7`], [`q = 11`], [`q = 13`], [mean weak per
  covered class (7, 11, 13)],
  [2],
  [`2+1+1+1`],
  [0.41],
  [0.42],
  [0.42],
  [2.3, 3.2, 3.7],
  [3],
  [`3+1+1`],
  [0.25],
  [0.25],
  [0.22],
  [5.6, 7.5, 8.2],
  [3],
  [`3+2+1` (g\=5)],
  [0.66],
  [0.66],
  [0.67],
  [10.5, 20.0, 25.4],
  [3],
  [`3+3` (g\=3, Type 1)],
  [0.42],
  [0.42],
  [0.42],
  [6.8, 12.6, 17.3],
  [4],
  [`4+1`],
  [0.15],
  [0.18],
  [0.13],
  [7.0, 6.3, 8.6],
  [4],
  [`4+1+1`],
  [0.39],
  [0.38],
  [0.23],
  [9.5, 14.6, 28.5],
  [#strong[5]],
  [#strong[`5`]],
  [#strong[0.077]],
  [#strong[0.061]],
  [#strong[0.054]],
  [#strong[7.1, 6.7, 6.1]],
  [6],
  [`6` (`q = 7` only)],
  [0.14],
  [—],
  [—],
  [7.8],
  [7],
  [`7` (`q = 3, 5`)],
  [0.34, 0.34],
  [—],
  [—],
  [12.3, 15.7],
)
]

Reading. (i) At `n = 3` the covered share of the hyperelliptic family
saturates near `0.25` while the number of weak curves per covered class
grows (`5.6 → 8.2`): the weak curves pile up in the classes with
`#E ≡ 0 (mod 4)` (check 5), exactly the restriction Joux–Vitse state
("not all curves are vulnerable to this isogeny walk") and Yee
quantifies ("around half of the ordinary isogeny classes will not
contain any weak curves"); the Type-1 covers (`3+3`) reach `0.42`, and
the genus-5 type `0.67`. (ii) At `n = 5` the covered share
#strong[falls] with `q` (`0.077 → 0.054`, the ratio `0.70` against the
predicted `(7/13)^{1/2} = 0.73` for `q^{n/2−δ} = q^{−1/2}` weak members
per class) while the weak count per covered class stays flat: this is
the sparse regime, where most classes hold no weak member and a planted
one is not reached by a walk. (iii) At `n = 4` the `r = 5` type
(`a = 0`) keeps a flat share and the `r = 6` types a larger, drifting
one, as the margins say. (iv) The walk length inside a covered class
(curves over weak curves) grows with `q` for every `n ≥ 4` (`n = 5`:
`26, 81, 118`), roughly as `0.06 q^3` against rho’s `q^{5/2}`
(`130, 401, 609`): at toy size the ratio is flat (`0.20, 0.20, 0.19`)
because the constant is small; the exponent gap `q^{1/2}` only shows
beyond `q ≈ 250`. Nothing at `q ≤ 13` is a statement about cryptographic
sizes; the direction of every drift matches §1.

#strong[Where the odd-characteristic Teske field is.] `n = 5` is the
only extension degree with a positive margin: the family
`y^2 = ∏_{i=0}^{3}(x − α^{q^i})`, `α` of degree 5 (Hess Table 3, Diem
2003), has `≈ q^2` curves among `q^5`, descends to genus 5 over `F_q`
(owner exponent `2 − 2/5 = 1.6` against rho’s `2.5`), and a walk reaches
it in `≈ q^3 > q^{5/2}` steps. The test an auditor needs is the one
Yee’s thesis asks for in its last paragraph: given `E/F_{q^5}`, decide
whether some degree-2 function has a branch set of this type. The branch
set is a 4-subset of a 5-orbit, so the test is: over the
`PGL_2(F_q)`-orbit representatives of the `q^5`-many degree-5 `α` (there
are `≈ q^2`), check whether `j(E)` equals the cross-ratio invariant of
`{α, α^q, α^{q^2}, α^{q^3}}`; Part B’s enumeration is this test run for
every `E` at once. At `n = 6` the margin is `0` with genus 9
(Joux–Vitse’s App. C family), and at `n = 4` the `r = 5` type sits at
margin `0` with genus 5.

#figure([#image("figures/oddchar_counts.svg")],
  caption: [
    Weak j-invariants per branch type divided by q^(r−3)
    (figures/oddchar\_counts.svg, from oddchar\_types.csv).
  ]
)

#figure([#image("figures/oddchar_classes.svg")],
  caption: [
    Isogeny-class coverage and weak members per covered class
    (figures/oddchar\_classes.svg, from oddchar\_classes.csv).
  ]
)

== 6. The census: Weil-restriction-shaped genus-3 hyperelliptic
Jacobians (Part C) \[E, T\]
<the-census-weil-restriction-shaped-genus-3-hyperelliptic-jacobians-part-c-e-t>
Run `20261018T0845Z` (`census.csv`, `census_summary.csv`,
`census_patterns.csv`, `census_checks.txt`; the `q ≤ 11` files are
byte-identical to those of run `20261018T0630Z`, which was killed at
`q = 13`). For every odd `q ≤ 13` the tool enumerates every squarefree
`y^2 = c f(x)` (`c ∈ {1, ν}`, `f` monic of degree 8 with no `x^7` term
or of degree 7 with no `x^6` term), keeps the Res-shaped ones
(`#C(F_q) = q + 1`, `#C(F_{q^2}) = q^2 + 1`), computes `t` from
`#C(F_{q^3})`, joins models into `F_q`-isomorphism classes by the moves
that generate `PGL_2(F_q)` modulo translations (every neighbour found,
no duplicate model: asserted), and records per class the trace, the
Weierstrass cycle type, the `F_{q^3}`-rational reduced automorphism
group and the GHS signature of Theorem 2. Checks 8–9 pass at every `q`
(no Hasse violation, `t` constant on each class, every signature class
has `#E ≡ 0 (mod 4)` and cycle type `(1,1,3,3)` or `(2,6)`); the twelve
Joux–Vitse controls pass (check 10).

#align(center)[#table(
  columns: 14,
  align: (col, row) => (right,right,right,right,right,right,right,left,left,right,right,right,left,left,).at(col),
  inset: 6pt,
  [`q`], [models], [squarefree], [Res-shaped models], [classes],
  [signature], [no signature], [`(1,1,3,3)` yes / no], [`(2,6)` yes /
  no], [`(1,1,6)`], [`(2,3,3)`], [`#E` prime], [`t` subfield-shaped
  (with sig.)], [traces (with sig.)],
  [3],
  [5,832],
  [3,888],
  [152],
  [21],
  [6],
  [15],
  [4 / 2],
  [2 / 2],
  [11],
  [0],
  [0],
  [7 (0)],
  [9 (2)],
  [5],
  [187,500],
  [150,000],
  [3,088],
  [158],
  [20],
  [138],
  [12 / 30],
  [8 / 20],
  [76],
  [12],
  [0],
  [70 (8)],
  [21 (10)],
  [7],
  [3,294,172],
  [2,823,576],
  [48,352],
  [538],
  [39],
  [499],
  [19 / 116],
  [20 / 134],
  [211],
  [38],
  [0],
  [92 (1)],
  [37 (9)],
  [11],
  [42,517,464],
  [38,652,240],
  [293,440],
  [3173],
  [106],
  [3067],
  [54 / 832],
  [52 / 803],
  [1168],
  [264],
  [0],
  [547 (6)],
  [73 (16)],
  [13],
  [135,150,652],
  [124,754,448],
  [682,472],
  [5582],
  [160],
  [5422],
  [92 / 1264],
  [68 / 1432],
  [2198],
  [528],
  [0],
  [1042 (28)],
  [93 (36)],
)
]

Reading. (i) #strong[Theorem 3 is met exactly]: among all 9,472
Res-shaped classes only the four predicted cycle types occur, no other
pattern (`8`, `1+7`, `3+5`, `4+4`, `2+2+4`, …) appears although
squarefree octics of every pattern were enumerated, and no class has
`#E` prime (`classes_numE_prime = 0` at every `q`). (ii) #strong[H\_C is
refuted]: 331 of 9,472 classes carry the compositum signature; at
`q = 13`, `160` of `5582`. The signature classes grow like `q^2`
(`6, 20, 39, 106, 160` at `q = 3, 5, 7, 11, 13`), in step with the
number of weak `j` of the `n = 3` hyperelliptic family in Part B
(`6, 22, 46, 115, 160`: at `q = 13` the two counts coincide), while the
non-signature classes grow like `q^{3.5}` (`15, 138, 499, 3067, 5422`;
`5422/3067 = 1.77` against `(13/11)^{3.5} = 1.79`), the growth expected
if a positive fraction of all `≈ q^5` genus-3 hyperelliptic curves have
`a_1 = a_2 = 0` (`a_1 ∈ O(√q)`, `a_2 ∈ O(q)`). (iii) The non-signature
classes split by cycle type: the `(1,1,6)` and `(2,3,3)` ones
(`2198 + 528` at `q = 13`) can never be compositums by Theorem 2(c); the
`(1,1,3,3)` and `(2,6)` ones (`1264 + 1432`) pass the necessary
cycle-type condition but have no Frobenius-cyclic Klein four-group:
their `F_{q^3}`-rational reduced automorphism group is trivial for
`1202` of the `1264` `(1,1,3,3)` classes and of order 2 for the other
`62`, and no class has a `σ`-stable Klein four-group without also having
the signature. (iv) `t` has the subfield shape `t_0^3 − 3qt_0` (so
`Res E ~ E_0 × T` splits) for `1042` classes at `q = 13`, `28` of them
with the signature; the remaining `4380` non-signature classes are
isogenous to a #emph[simple] Weil restriction by an isogeny that is not
the compositum’s. (v) Every signature class has `#E ≡ 0 (mod 4)`, as
Joux–Vitse state for the family (3), and the traces reached by signature
classes (`36` of `93` at `q = 13`) are a minority of the traces reached
by Res-shaped Jacobians at all.

What this says for the search. The association `Jac(C) ~ Res E` is a
property of the isogeny class of `E` (its trace), not of a model, so it
is not hideable; what varies with the model is the #emph[degree] of the
cover `C_K → E` induced by the isogeny, which the census does not
compute. The compositum classes have a known degree-8 cover; for the
`5422` others at `q = 13` the degree is open \[O\]. The strong form of
the question, whether a non-compositum class carries a cover of degree
comparable to 8, is the one experiment that would widen the
odd-characteristic weak set beyond Momose–Chao’s families, and §7 names
it as the next step.

#figure([#image("figures/census.svg")],
  caption: [
    Res-shaped genus-3 hyperelliptic classes by signature
    (figures/census.svg, from census\_summary.csv).
  ]
)

== 7. Other ways to attach a Weil-descent target to a curve, and where
to look next
<other-ways-to-attach-a-weil-descent-target-to-a-curve-and-where-to-look-next>
Ways found in the literature and in this round, with their
class-invariance:

#align(center)[#table(
  columns: 6,
  align: (col, row) => (left,left,left,left,left,left,).at(col),
  inset: 6pt,
  [association], [field], [what it needs], [class invariant?],
  [hideable?], [source],
  [GHS Artin–Schreier compositum],
  [`F_{2^{nl}}`],
  [small `deg Ord_b`],
  [no (`b`)],
  [yes when `a > 0` (§4)],
  [GHS 2002; MMT 2002],
  [gGHS (Hess)],
  [`F_{2^{nl}}`],
  [`b = (αβ)^2` with small annihilators],
  [no],
  [mostly dense (§4)],
  [Hess 2003/2004],
  [Kummer compositum],
  [`F_{q^n}`, `q` odd],
  [branch type with small `(m, r)`],
  [no (branch set)],
  [only `n = 5` (§5)],
  [Diem 2003; Momose–Chao],
  [isogeny-class extension],
  [both],
  [a weak member in the class],
  [by definition],
  [the walk is the auditor’s tool, cost `q^{min(δ,n/2)}`],
  [GHS 2002 §4; Yee 2016],
  [`Jac(C) ~ Res E` by `L`-polynomial (Tate)],
  [`F_{q^n}`],
  [`L_C = L_{Res E}`],
  [yes (isogeny class of `Res E`)],
  [no: it is a property of the trace `t`],
  [Part C; Theorem 3],
  [`(ℓ,ℓ,ℓ)`-isogeny `Res E → Jac(C)`],
  [`F_{q^3}`],
  [a rational maximal isotropic `Λ ⊂ Res E[ℓ]`],
  [class-level existence; curve-level construction],
  [not studied \[O\]],
  [Tian 2020],
  [Scholten form, genus 2 over `F_{q^2}`],
  [`F_{q^4}`, `F_{q^2}`],
  [full rational 2-torsion, or odd order and `j ∉ F_q`],
  [partly (2-torsion is a volcano-level property)],
  [no: 3/4 of curves],
  [Scholten; Joux–Vitse §5],
  [trace-zero / decomposition index calculus],
  [`F_{q^n}`],
  [nothing special],
  [yes (group only)],
  [no: everyone],
  [Gorla–Massierer; Gaudry–Diem],
  [`C_{ab}` curves on the Weil restriction, char 3],
  [`F_{3^n}`],
  [composite `n`, special `E`],
  [no],
  [not assessed \[O\]],
  [Arita 2000],
)
]

Where a "different type of curve" could still hide, and the
discriminating step for each:

+ #strong[`F_{q^5}` in odd characteristic.] The genus-5 family
  `y^2 = ∏_{i=0}^{3}(x − α^{q^i})` is sparse (`a = +1/2`) and its branch
  type is the only one with genus `≤ 9` and period 5 (Part B found
  exactly one type for `n = 5` at every `q`). The auditor’s test is
  explicit: for a published `E/F_{q^5}`, decide whether some degree-2
  function has branch set `{α, …, α^{q^3}}` for a degree-5 `α`; Yee’s
  open question (an efficient weak-form test in odd characteristic) is
  exactly this, and the Legendre-form criterion of Chao sl. 29–32 (a
  quadratic condition in `λ`) is the model to follow. Next step: write
  the `n = 5` analogue of `ghs_screen` (the `B`-test over
  `PGL_2(F_q)`-orbits).
+ #strong[Non-GHS Res-shaped Jacobians.] The census shows that, already
  at genus 3, most hyperelliptic Jacobians isogenous to a Weil
  restriction are not compositums. Each is a target with a cover of
  #emph[some] degree; the degree decides whether the transfer is
  computable. Next step: for the non-signature classes at `q = 5, 7`,
  compute the kernel of an explicit isogeny `Res E → Jac(C)` (via Tian’s
  normal-Weil-set method for `ℓ = 3`, or by matching `(2,2,2)`-kernels)
  and record its degree.
+ #strong[The `(1,1,6)` and `(2,3,3)` columns.] By Theorem 3 these
  Jacobians are isogenous to `Res E` but can never be GHS compositums. A
  constant-fraction count of them against the `(1,1,3,3)`/`(2,6)`
  columns at every `q` (§6) says the non-compositum relation is not an
  artefact of small `q`.
+ #strong[Binary `n = 21, 35`.] Hess’s pairing gives genus 21 at
  `n = 21` and 105 at `n = 35` from products of non-primitive factors,
  sparser than the basic descent (`δ = 17` against 16). Whether the gGHS
  map is explicit there, and the Enge–Gaudry cost at genus 21 over
  `F_{2^8}` (`N = 168`) or `F_{2^10}` (`N = 210`), is a direct extension
  of MMT’s table with Hess’s genus formula. These `N` are not in use;
  the row is an auditor’s checklist entry, not a threat.
+ #strong[Characteristic 3.] Odd-characteristic Kummer descents apply
  verbatim (Theorem 1 does not see `q`), so `F_{3^{5l}}` has the same
  `a = +1/2` regime; Arita’s `C_{ab}` route is a second, Artin–Schreier
  flavoured, association that this round did not enumerate \[O\].

Negative results of the round, kept: no branch type of period `n` with
genus below the previous study’s floor was found (check 7); no
Res-shaped hyperelliptic Jacobian with prime `#E` exists (Theorem 3,
confirmed by the census at five `q`); no prime-field variant exists via
descent (§3).

== 8. Checks and controls
<checks-and-controls>
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [check (PROTOCOL.md)], [where], [outcome],
  [1. `n = 7`: type I `(3, 7, δ=4)`, type II `(4, 8, δ=3)`, gGHS
  `(3+3, 7, δ=2)`; GHS 2002 per-class table
  `0.404, 0.286, 0.269, 0.286, 0.323`],
  [test `check1_n7`],
  [pass],
  [2. `n = 31`, `q = 2^5`: basic `δ = 25`, per class `≈ 2^{−52}`; gGHS
  `(5,5)`: genus 31, `δ = 22`, `≈ 2^{−38}`],
  [test `check2_n31`],
  [pass],
  [3. MMT counts `2^94`, `2^57`, `2^171` for `N = 161, 165, 170`],
  [test `check3_mmt_counts`],
  [pass],
  [4. `n = 15`: minimal `m = 5`; `n = 21`: `m = 6` from a degree-5
  product],
  [test `check4_composite_n`],
  [pass],
  [5. every `j(B) ∈ F_{q^n}`; every trace in the Hasse range; `n = 3`,
  `(3,5)`: `#E ≡ 0 (mod 4)` on both twists],
  [`oddchar_checks.txt`],
  [pass (0 violations at every `(q, n)`)],
  [6. all-`j` tables list exactly `q^n` values],
  [`oddchar_checks.txt`],
  [pass (25 tables, counts equal `q^n`)],
  [7. branch types found per `n` against `types.csv` of the previous
  study],
  [this report, §5],
  [pass (§5b)],
  [8. `#C(F_{q^3}) = q^3 + 1 − 3t` with `t` integral in the Hasse range;
  `t` constant on each class],
  [`census_checks.txt`],
  [pass (0 violations, `t` constant on every class, 5 `q`)],
  [9. signature classes have `#E ≡ 0 (mod 4)` and cycle type `(1,1,3,3)`
  or `(2,6)`],
  [`census_checks.txt`],
  [pass (331 signature classes)],
  [10. the 12 Joux–Vitse controls (`p = 5, 7, 11, 13`; linear, split,
  conjugate) are Res-shaped with the recorded `t`, cycle type and
  signature],
  [test `controls_jv_family`],
  [pass],
  [field tables: table addition equals digit addition; inverses;
  subfield membership],
  [test `field_tables_consistent`],
  [pass],
  [census bookkeeping: no duplicate model, every neighbour of a
  Res-shaped model is in the enumeration (asserted)],
  [`census_checks.txt`, assertions],
  [pass (0 missing at every `q`)],
)
]

Negative and positive controls. The twelve Joux–Vitse `H` curves are the
positive control for the signature test (all pass; a false test would
have had to pass them by accident with the Klein-four group found from
the roots). The negative control is Theorem 3: a class with a cycle type
outside the four allowed ones, or with `#E` prime, would contradict a
proof; none appeared among 9,472 classes. The exponent test of H\_B is
controlled by the literature’s densities at `n = 2, 3, 5, 6, 7` (Theorem
1), which the counts reproduce.

== 9. What is not established
<what-is-not-established>
- \[O\] No cost here is measured. The exponents `n/2`, `2 − 2/g` and
  `min(δ, n/2)` drop constants that decide feasibility in practice (MMT
  2002 Table 1 has the Enge–Gaudry constants for binary fields; for
  genus `≥ 100` nothing is feasible).
- \[H\] The uniform distribution of weak curves over isogeny classes is
  the heuristic of GHS 2002 §5; it is tested only at toy size (Yee 2016
  and Part B here), and Sato–Tate makes classes unequal in size. Part
  B’s class statistics are exact at `q ≤ 13` and show the predicted
  direction of drift, not an asymptotic law.
- \[O\] The census does not compute the degree of the isogeny
  `Res E → Jac(C)` for the non-signature classes; "isogenous" says a
  cover of some degree exists, not that it is computable.
- \[O\] Part B enumerates branch types with orbits inside one ambient
  field per `n` (sizes `{1,5}` at `n = 5`, `{1,7}` at `n = 7`, no
  3-orbits at `n = 4`); types with other orbit sizes have `r ≥ 7` at
  those `n` and genus above the frozen `g_max`, so the smallest-genus
  types are not affected, but the count of larger-genus types is
  incomplete there.
- \[O\] Characteristic 3 and Arita’s `C_{ab}` route, Thériault’s
  superelliptic (`y^d = f`) descents, and the non-hyperelliptic genus-3
  census (plane quartics) are not enumerated.
- \[O\] Nothing here concerns any standardised curve: the NIST and SEC
  binary curves use prime `N`, the prime-field curves have no descent
  (§3), and `F_{2^155}`, `F_{2^161}` and `F_{q^5}` fields are discussed
  only as the literature’s examples.
- \[O\] No Lean statement accompanies Theorems 2 and 3 (AGENTS.md
  "Mathematical results should include Lean proofs"); they are
  elementary (Riemann–Hurwitz, the `F_2` factorisation of `T^c + 1`),
  and a formalisation is listed as owed.

== 10. Canonical graphs checked
<canonical-graphs-checked>
No registered curve gains or loses a route, so
`docs/curves/jacobian-transfers.json`,
`docs/curves/JACOBIAN_TRANSFERS.md`,
`docs/index-calculus-scoreboard.html`, `docs/ic/progress-timeline.json`
and the `docs/performance-gains/` graphs are unchanged. The
`GHS_GENUS_FLOOR` table in `src/cryptanalysis/jacobian_transfer/mod.rs`
(`[1, 2, 3, 5, 5, 9, 7, …]`) is consistent with the smallest genera
found here (`n = 2..7`: `2, 3, 5, 5, 9, 7`). The previous study’s
`figures/genus_vs_n.svg` is unchanged. New graphs of this round:
`figures/margins.svg`, `figures/oddchar_counts.svg`,
`figures/oddchar_classes.svg`, `figures/census.svg`, and the diagram
`figures/taxonomy.svg`.

== References
<references>
- E. Teske, #emph[An elliptic curve trapdoor system], J. Cryptology 19
  (2006) 115–133; ePrint 2003/058.
- S. D. Galbraith, F. Hess, N. P. Smart, #emph[Extending the GHS Weil
  descent attack], Eurocrypt 2002, LNCS 2332, 29–44.
- F. Hess, #emph[The GHS attack revisited], Eurocrypt 2003, LNCS 2656,
  374–387; #emph[Generalising the GHS attack on the elliptic curve
  discrete logarithm problem], LMS J. Comput. Math. 7 (2004) 167–192;
  #emph[Weil descent attacks] (survey, author PDF).
- M. Maurer, A. Menezes, E. Teske, #emph[Analysis of the GHS Weil
  descent attack on the ECDLP over characteristic two finite fields of
  composite degree], LMS J. Comput. Math. 5 (2002) 127–174.
- A. Menezes, E. Teske, #emph[Cryptographic implications of Hess’
  generalized GHS attack], AAECC 16 (2006) 439–460; ePrint 2004/235.
- A. Menezes, E. Teske, A. Weng, #emph[Weak fields for ECC], CT-RSA
  2004, LNCS 2964, 366–386; ePrint 2003/128.
- A. Menezes, M. Qu, #emph[Analysis of the Weil descent attack of
  Gaudry, Hess and Smart], CT-RSA 2001.
- K. Karabina, A. Menezes, C. Pomerance, I. Shparlinski, #emph[On the
  asymptotic effectiveness of Weil descent attacks], CACR 2010-03; J.
  Math. Cryptol. 4 (2010).
- R. Yee, #emph[On the effectiveness of isogeny walks for extending
  cover attacks on elliptic curves], MMath thesis, Waterloo, 2016.
- C. Diem, #emph[The GHS attack in odd characteristic], J. Ramanujan
  Math. Soc. 18 (2003) 1–32; #emph[On the discrete logarithm problem in
  elliptic curves], Compositio Math. 147 (2011) 75–104.
- C. Diem, E. Thomé, #emph[Index calculus in class groups of
  non-hyperelliptic curves of genus three], J. Cryptology 21 (2008)
  593–611.
- F. Momose, J. Chao, #emph[Scholten forms and elliptic/hyperelliptic
  curves with weak Weil restrictions], ePrint 2005/277; #emph[Elliptic
  curves with weak coverings over cubic extensions of finite fields with
  odd characteristic], ePrint 2009/236; J. Chao, ECC 2007 slides.
- T. Iijima, F. Momose, J. Chao, #emph[Classification of
  elliptic/hyperelliptic curves with weak coverings against the GHS
  attack under an isogeny condition], ePrint 2013/487;
  #emph[Classification of elliptic/hyperelliptic curves with weak
  coverings against the GHS attack without isogeny condition], ePrint
  2009/613; #emph[Elliptic curves with weak coverings …, d \= 2, 3, 5,
  7], ePrint 2015/805.
- A. Joux, V. Vitse, #emph[Cover and decomposition index calculus on
  elliptic curves made practical], Eurocrypt 2012, LNCS 7237, 9–26.
- S. Tian, #emph[Cover attacks for elliptic curves with prime order],
  arXiv:2012.07173 (2020); #emph[Cover attacks for elliptic curves over
  cubic extension fields], J. Cryptology 36 (2023).
- E. Gorla, M. Massierer, #emph[Index calculus in the trace zero
  variety], Adv. Math. Commun. 9 (2015) 515–539; arXiv:1405.1059.
- S. Arita, #emph[Weil descent of elliptic curves over finite fields of
  characteristic three], Asiacrypt 2000, LNCS 1976, 248–258.
- S. Arita, K. Matsuo, K. Nagao, M. Shimura, #emph[A Weil descent attack
  against elliptic curve cryptosystems over quartic extension fields],
  IEICE Trans. E89-A (2006) 1246–1254.
- N. Thériault, #emph[Weil descent attack for Kummer extensions], J.
  Ramanujan Math. Soc. 18 (2003) 281–312.
- J. Scholten, #emph[Weil restriction of an elliptic curve over a
  quadratic extension], preprint (2003).
- P. Gaudry, E. Thomé, N. Thériault, C. Diem, #emph[A double large prime
  variation for small genus hyperelliptic index calculus], Math. Comp.
  76 (2007) 475–492.
- B. Smith, #emph[Isogenies and the discrete logarithm problem in
  Jacobians of genus 3 hyperelliptic curves], J. Cryptology 22 (2009)
  505–529.
- Repository: `src/cryptanalysis/ec_trapdoor.rs`,
  `ghs_class_coverage.rs`, `jacobian_transfer/`,
  `research/odd_char_descent_genus3_20261014/`,
  `research/nist_trapdoor_slot_program_20261011/`,
  `research/notes/index-calculus/RESEARCH_SEED_GRIND_TRAPDOOR_20261002.md`;
  aburan28/ml-cryptanalysis `experiments/ghs-descent/report.md`.
