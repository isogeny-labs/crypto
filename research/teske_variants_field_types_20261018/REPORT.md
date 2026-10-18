# Teske-type hidden Weil descents by field type: where the three ingredients exist

2026-10-18. A research search round (AGENTS.md "Research searches must leave
visual reports"): literature reading plus exact native enumeration over toy
fields. Protocol: [PROTOCOL.md](PROTOCOL.md), frozen at commit `62b4502c7`
before the first run. Tool:
[`examples/teske_variants_field_types.rs`](../../examples/teske_variants_field_types.rs)
(std-only Rust, 4 threads; `rustc --edition 2021 -O`, tests with `--test`).
Recorded run: `RUN_STAMP` (results in [results/RUN_STAMP/](results/RUN_STAMP/),
`env.txt` beside them; run report
[reports/run-RUN_STAMP/run-report.pdf](reports/run-RUN_STAMP/run-report.pdf)).
One earlier launch, `20261018T0624Z`, was stopped after a smoke run exposed a
normalisation bug in Part C; its directory keeps a `NOTE.md`. This report as
PDF: [REPORT.pdf](REPORT.pdf). Conductor is not installed on this host (the
installer answers HTTP 403 through the session proxy), so there is no task
card. The round solves no discrete logarithm, times nothing, and makes no
statement about any deployed curve. Every number below is a count or a
formula on exact invariants; the exponents are formula-level labels in the
sense of `docs/curves/JACOBIAN_TRANSFERS.md`.

**Tags.** [T] theorem, stated by a cited source or proved here; [E] exact
computation by the tool, reproducible and checked; [H] heuristic; [O] out
of scope or not established.

## Answer

Teske's trapdoor system (J. Cryptology 19 (2006); ePrint 2003/058) hides a
GHS-weak curve over `F_{2^161}` behind an isogeny. It needs three things: a
weakness that is **not an isogeny-class invariant**, a **transfer** that
follows the isogeny, and a weak set **sparse inside the isogeny class**, so
that an auditor cannot find the weak member by walking the class faster
than Pollard rho. This round asked over which field types and extension
degrees all three exist, and what other ways there are to attach a
Weil-descent target to a curve.

- **[T] Prime fields `F_p`: none of the ingredients.** There is no subfield
  to descend to (GHS 2002 §5), every same-field cover of genus `g ≥ 2` moves
  the problem into a group of size `p^g` where index calculus costs at least
  `p^{1−o(1)} > p^{1/2}`, and base change to `F_{p^k}` followed by a descent
  leaves the subgroup of size `p` inside a group of size `≥ p^k`. The only
  non-class-invariant structure left, the endomorphism ring, gives no
  exponential gain. No Teske-type variant via descent exists over `F_p`
  (§3).
- **[E, T] Binary fields `F_{2^{nl}}`.** The exact combinatorics of
  `x^n − 1` over `F_2` (Part A) place the sparse regime at `n = 7` for the
  type-I genus-7 descent only (`δ = 4`, margin `+0.5`, i.e. `+11.5` bits at
  `N = 161`: an auditor's walk costs as much as rho), while the type-II
  genus-8 descent (`δ = 3`, `−11.5` bits: the walk costs `2^69`) and
  Hess's gGHS variant (`δ = 2`) are dense, which is why Menezes–Teske call
  `GF(q^7)` partially weak. `n = 3` and `n = 5` are dense for every curve
  (weak fields). The sparse regime reappears at `n = 15, 21, 31` and other
  odd `n` with small `ord_n(2)`, with genus `15`, `21–31`, `31`, but there the
  public sub-descents (`n = 5` inside `n = 15`) already beat rho, and
  `n = 31` is the `F_{2^155}` case of Hess 2003 (`2^{−38}` weak orbit classes
  per isogeny class). Prime `n` has `m ≥ ord_n(2) + 1` and nothing descends
  (Menezes–Qu). Table in §4.
- **[E, H] Odd characteristic `F_{q^n}`.** The weak set of a Kummer-descent
  branch type with `r` points has `≈ q^{r−3}` curves: the exact counts at
  `q ≤ 13` reproduce the exponent `r − 3` with constants between
  `0.04` and `2.1` (Part B), so `δ = n + 3 − r` and the margin of the
  smallest-genus type is `a = n + 3 − r − n/2`: `n = 2: −1` (genus 2,
  Scholten, no gain anyway), `n = 3: −1/2` (genus 3: dense, the isogeny-walk
  regime Joux–Vitse conjecture), `n = 4: 0` for the `r = 5` type and `−1`
  for the `r = 6` types (all genus 5), **`n = 5: +1/2` (genus 5)**,
  `n = 6: 0` (genus 9), `n = 7: −1/2` (genus 7), prime
  `n ≥ 11`: genus `≥ 1793` (Diem). **The odd-characteristic analogue of
  Teske's field is `F_{q^5}`**, with the genus-5 family
  `y^2 = ∏_{i=0}^{3}(x − α^{q^i})`, `α` of degree 5 (Diem 2003; Hess 2004
  Table 3). The isogeny-class statistics (every `j` over `F_{q^n}` traced)
  show the predicted drift: the share of curves whose class contains a weak
  member saturates for `n = 3` at the classes with `#E ≡ 0 (mod 4)`, stays
  flat for `n = 4`, and falls with `q` for `n = 5` (§5).
- **[T, E] A second association, without covering maps.** By Tate, a
  genus-3 curve `C/F_q` has `Jac(C)` isogenous to `Res_{F_{q^3}/F_q} E` exactly
  when `L_C(T) = 1 − tT^3 + q^3T^6`. For hyperelliptic `C` this forces `t`
  even and a Frobenius cycle type `(1,1,3,3)`, `(2,3,3)`, `(1,1,6)` or
  `(2,6)` on the eight Weierstrass points (Theorem 3, from the Frobenius
  action on `Jac(C)[2]`), so **no hyperelliptic genus-3 Jacobian is
  isogenous to the Weil restriction of a prime-order `E/F_{q^3}`, by any
  isogeny**, which sharpens the remark of Tian 2020 §1 that covering maps
  from hyperelliptic curves cannot reach prime order. The census of every
  genus-3 hyperelliptic curve over `F_q`, `q ≤ 13`, finds `CENSUS_TOTAL`
  Res-shaped isomorphism classes of which only `CENSUS_SIG` carry the GHS
  compositum signature (a Frobenius-cyclic Klein four-group on the
  hyperelliptic line, Theorem 2): the isogeny relation is far wider than
  the covering construction. Whether any non-signature class carries a
  cover of small degree is open [O] (§6).
- **No undiscovered affected curve type was found** inside the enumerated
  families; the one new structural fact is Theorem 3, and the one new
  place where a hidden descent can live in odd characteristic is `n = 5`.
  What remains open, and the next discriminating steps, are listed in §7.

## 1. Framework: exponents, margin, and what an auditor can do

For `E/F_{q^n}` with a descent target of genus `g` over `F_q` (exponents
in units of `log q`, constants dropped):

| quantity | exponent | tag and source |
|:--|:--|:--|
| Pollard rho on the prime-order subgroup | `n/2` | [T] |
| index calculus on a genus-`g` Jacobian over `F_q`, double large primes | `2 − 2/g` (`1` for a genus-3 plane quartic) | [H] Gaudry–Thomé–Thériault–Diem; Diem–Thomé 2008; constants grow with `g` |
| weak curves among the `≈ q^n` isomorphism classes | `q^{n−δ}` | [E] Parts A and B |
| members of a random isogeny class | `q^{n/2}` | [T] Hasse–Weil; GHS 2002 §5 |
| weak members per class, uniform-distribution heuristic | `q^{n/2−δ}` | [H] GHS 2002 §5; Menezes–Teske 2006; Yee 2016 §5.2 (tested on `F_{2^18}`, `F_{2^24}`, `F_{2^30}`) |
| auditor's isogeny walk to a weak member, when one exists | `min(δ, n/2)` | [H] |
| margin `a = δ − n/2` | `> 0`: sparse, hidden; `< 0`: dense, weak field | definition |

A hidden descent in Teske's sense needs `a > 0` (the auditor's walk costs
at least rho) together with `2 − 2/g < n/2` (the owner gains). With `a < 0`
and `2 − 2/g < n/2` the walk plus the descent beat rho for every class that
contains a weak member: Menezes–Teske–Weng's weak field, extended by
Galbraith–Hess–Smart's isogeny step. The auditor's answer in the sparse
regime is not the walk but the direct test of the published curve for every
weak type (Part A's magic numbers; Part B's branch types), which costs
polynomial time and which the repository's `audit_curve` and `ghs_screen`
already do for binary fields.

The exponent `2 − 2/g` is quoted, not measured; MMT 2002 Table 1 and
Appendix A give the Enge–Gaudry running times with constants for every
composite `N ∈ [100, 600]`, and this round does not redo that.

## 2. Sources read (exact locations)

| source | what it states (quoted or tightly paraphrased) | class |
| --- | --- | --- |
| Teske, *An elliptic curve trapdoor system*, ePrint 2003/058 (abstract; the PDF returns HTTP 403 here), J. Cryptology 19 (2006) 115–133 | "a pair of elliptic curves over `F_{2^161}`": GHS "reduces the ECDLP in `E_s(GF(2^161))` to a hyperelliptic curve DLP in the Jacobian of a curve of genus 7 or 8, which is computationally feasible, but by far not trivial"; "`E_pb` is isogenous to `E_s`"; "the best attack on the ECDLP in `E_pb` is the parallelized Pollard rho method". | T (abstract only) |
| Galbraith–Hess–Smart, *Extending the GHS Weil descent attack*, Eurocrypt 2002 (author PDF, Hess's page) | §1: "One might 'hide' such a curve by taking an isogeny to a curve `E_1` for which the GHS attack is not effective. Knowledge of the 'trapdoor'…" (the trapdoor idea predates Teske's paper). §5: `p = min(1, s q^{t+1−n/2}/(nr))` weak classes per isogeny class; `n = 7`: "every curve over `F_{q^7}` can be attacked by reducing to a hyperelliptic curve of genus 7 or 8 over `F_q`" for `q ≥ 2^11`; table of `p` for `q = 2, …, 2^5`: `0.404, 0.286, 0.269, 0.286, 0.323`; `n = 31`: basic proportion `6q^{−25}`, extended `6q^{−9.5}/(31r)`, "one in `2^52`" for the IPsec curve; "for curves over large prime fields the techniques of Weil restriction do not apply". | T; the per-class formula is H |
| Hess, *The GHS attack revisited*, Eurocrypt 2003 (author PDF) | §4: `m = d` from `b = (αβ)^2`, genus `2^d − 1`, about `s q^{2d−1}/2` curves; per-class proportion `min{1, s q^{2d−1}/(2 q^{n/2} n r)}`; `F_{2^155}`: `2^{−38}` against `2^{−52}`; the search costs `≈ 2^39` operations. §5.2 (towers) and §5.3 (descent from extensions: "for prime `n` this approach will however not give an improvement"). | T; proportions H |
| Hess, *Weil descent attacks* (survey, author PDF), Thm 30 (from Diem 2003) and Table 3 | Odd characteristic, prime `n`: `g ≥ 2^{φ_2(n)−2}(n−4)+1`; "For `n = 5` and `n = 7` there are families … for which `g` assumes the lower bounds 5 and 7"; `n = 11, 13, 17, 19, 23`: `g ≥ 1793, 9217, 833, 983041, 9729`; "The attack is not feasible for prime `n ≥ 11`". Table 3 lists the `n = 5` family `Y^2 = ∏_{i∈{0,1,2,3}}(X − σ^i(a))` and the `n = 7` family `∏_{i∈{0,1,2,4}}`. | T |
| Maurer–Menezes–Teske, LMS JCM 5 (2002) (Cambridge PDF) | Table 1: `N = 161, n = 7, l = 23, m = 4, g = 7, I = 94, ρ = 80, T = 34`; `N = 165, n = 15, m = 5, g = 15, I = 57`; `N = 170, n = 5, m = 5, g = 15, I = 171`. Remark 20: "only `≈ 2^94` out of `≈ 2^162` elliptic curves over `F_{2^161}` have magic number `m = 4` relative to `n = 7`"; finding an isogenous `m = 5` curve over `F_{2^165}` "may be difficult, as one has essentially to search through the entire set of `2^57` curves". §7: the random curve E161-2 has `(m, g) = (7, 64)` for `(n, l) = (7, 23)`. | T |
| Menezes–Teske, *Cryptographic implications of Hess' generalized GHS attack*, ePrint 2004/235 (abstract) | "`GF(q^7)` are potentially partially weak … for half of all elliptic curves over `GF(q^7)`, namely those curves `E` for which `#E` is divisible by 4"; `GF(q^3)` partially weak, `GF(q^6)` potentially weak, `GF(q^8)` potentially partially weak; `N` not divisible by 3, 5, 6, 7, 8: not weak. | T (abstract) |
| Menezes–Teske–Weng, *Weak fields for ECC*, CT-RSA 2004 (ePrint 2003/128, abstract) | fields such as `GF(2^210)` are weak: "all instances … can be solved in significantly less time" than rho. | T (abstract) |
| Karabina–Menezes–Pomerance–Shparlinski, CACR 2010-03 | eq. (4): gGHS genus `g = 2^t − 2^{t−r} − 2^{t−s} + 1`; Thm 9: for all but `o(q^n)` of the `b`, `g ≥ 2^{n/2+o(n)}`; "the gGHS attack can be useful for at most `q^{16 log_2 n + O(1)}` elliptic curves"; with isogenies "at most `q^{n/2+O(log n)}` … negligible". | T |
| Yee, *On the effectiveness of isogeny walks for extending cover attacks*, MMath thesis, Waterloo 2016 (PDF) | §5.2: weak orbit classes per isogeny class `s q^{t+1−n/2}/(nr)`; `n = 3`: `2(q^2−1)` weak curves, `√q/(nr)` per class, "around half of the ordinary isogeny classes will not contain any weak curves"; §6.2: `F_{2^150}` "partially weak" (walk `2^50` steps, `2^9` faster than rho per step); §7: for odd characteristic "an effective method for discerning whether a curve is weak will be needed in order to utilize the isogeny walk in a meaningful way". | T, H as labelled |
| Diem, Compositio 147 (2011) (Cambridge PDF) | Prop. 4.10: `g(C) ≤ (2^n − 1)(2n − 1)` under Condition 2.7; "Let `q` be odd. In this case `k(E)|k(x)` is (tamely) ramified at 4 places." | T |
| Chao (with Momose), ECC 2007 slides (pdf) | sl. 20–21: `(2,…,2)` coverings with `g = d g_0` exist in odd characteristic only for `(d, n) = (2,2)`, `(5,4)`, `d = 2^n − 1`, and decomposable `(3,3)`; densities `Θ(q^2)` for `d = 3`, `Θ(q^2)` for `(5,4)`, `Θ(q^{dℓ−3})?` for `d = 2^n − 1`; sl. 24 (hyperelliptic genus-3 family, degree 8), sl. 25 (non-hyperelliptic Types 1–2, degree 4), sl. 33: Type I `#{λ} ≈ q^3/2`. | T; densities marked "?" are H |
| Iijima–Momose–Chao, ePrint 2013/487 and 2015/805 (abstracts) | complete classification of `(2,…,2)`-coverings under the isogeny condition `g(C) = d·g(C_0)` for elliptic and genus-2, 3 hyperelliptic `C_0`; "17 classes are newly found"; for elliptic `C_0` over `k_d` with `d = 2, 3, 5, 7`. | T (abstracts) |
| Joux–Vitse, Eurocrypt 2012 (author PDF) | §3: two isogeny-walk strategies; §4.1: the family (3), `Θ(q^2)` curves, walk `≈ q` steps, "we conjecture that all curves with cardinality divisible by 4 are vulnerable"; (4): non-hyperelliptic covers for "about half of the curves having their full 2-torsion defined over `F_{q^3}`"; §5: `F_{p^4}`, "approximately 3/4 of the elliptic curves … are directly vulnerable" to a genus-2 cover over `F_{p^2}`, most admit a genus-9 cover over `F_p`; App. C: `n = 6`, genus 9 for "at most `1/q^3`" of the curves. | T; conjecture H |
| Tian, *Cover attacks for elliptic curves with prime order*, arXiv 2012.07173 (2020; J. Cryptology 36 (2023)) | abstract: an `F_q`-rational `(ℓ,ℓ,ℓ)`-isogeny from `Res(E)` to the Jacobian of a genus-3 curve, then index calculus, `Õ(q)` "in some elliptic curve groups of prime order over `F_{q^3}`"; §1: "a necessary condition is that the elliptic curve has three rational points of order 2 [3,13]"; "[19] shows that `C` needs to be non-hyperelliptic if the elliptic curve has prime order"; the isogeny needs an `F_q`-rational maximal isotropic `Λ ⊂ W[ℓ]` not containing subgroups of `E, E^σ, E^{σ^2}`; example over `F_{523^3}` with prime order. | T |
| Gorla–Massierer, arXiv 1405.1059 | trace-zero index calculus `Õ(q^{2−2/(n−1)})` for `E/F_q` seen over `F_{q^n}`, "lower than that of generic attacks … for `n ≥ 5`"; feasible for `n = 3` up to 30-bit `q`; for `n = 5` the Gröbner step is the bottleneck. A class-invariant attack (it depends only on the group), so not a trapdoor. | T |
| Arita, Asiacrypt 2000 (abstract, Springer page) | some curves over `F_{3^n}` of composite degree are weaker than rho via a `C_ab` curve on the Weil restriction. | T (abstract) |
| Arita–Matsuo–Nagao–Shimura, IEICE E89-A (2006) (abstract) | quartic extensions of odd characteristic: reduce to a Scholten form, then genus 2 over `F_{q^2}`, then a `C_ab` model over `F_q`; "many" such curves. | T (abstract) |

Not obtained (HTTP 403 or paywall): the ePrint PDFs of Teske 2003/058,
Menezes–Teske 2004/235, MTW 2003/128, Momose–Chao 2009/236,
Iijima–Momose–Chao 2009/613, 2013/487, 2015/805, the IACR archive PDFs of
GHS 2002 and Hess 2003 (the author copies were used instead), Diem's
JRMS 2003 paper, Thériault's JRMS 2003 paper, Scholten's preprint (404).
Nothing below rests on their bodies except as reported by the papers that
cite them.

## 3. Prime fields: no descent variant [T]

Let `E/F_p` with a prime-order subgroup of size `r ≈ p`. Rho costs
`p^{1/2}`.

- **No Weil restriction.** `F_p` has no proper subfield, so there is no
  extension `K/k` to descend through: the GHS, gGHS, Kummer and
  Artin–Schreier constructions all start from `[K : k] = n ≥ 2`
  (GHS 2002 §5; Hess survey §5).
- **Same-field covers do not pay.** A cover `C → E` over `F_p` with `C` of
  genus `g ≥ 2` (the repository's `COVERS.md` constructs genus-2 ones for
  every `E/F_p`) moves the problem into `Jac(C)(F_p)` of size `≈ p^g`. Index
  calculus there costs `Õ(p^{2−2/g}) ≥ Õ(p)` (double large primes; Diem's
  plane-curve `Õ(p^{2−2/(d−2)})` is `≥ Õ(p)` as well), above `p^{1/2}`.
  Genus 1 covers are isogenies and change nothing.
- **Base change then descend does not pay.** Over `F_{p^k}` the subgroup is
  still of size `p` inside `E(F_{p^k})` of size `p^k`; any descent to a
  Jacobian over `F_p` of genus `≥ k` lands in a group of size `≥ p^k`, with
  the same bound. `JACOBIAN_TRANSFERS.md` prices this route at the most
  favourable genus and finds no `formula_advantage` row for a prime-field
  curve.
- **What varies inside a prime-field isogeny class.** The trace, the
  group order, the embedding degree and anomaly are class invariants and
  cannot be hidden. The endomorphism ring and the rational torsion structure
  vary with the position in the volcano, but the only ECDLP effect known
  is the automorphism factor `√#Aut`, visible on the curve itself
  (`j ∈ {0, 1728}`). `research/nist_trapdoor_slot_program_20261011/`
  (hypothesis H1d) already records that the isogeny-hiding *asymmetry*
  exists over `F_p` while no weak class exists to hide.

So the Teske pattern has no prime-field variant through descent; the
repository's P-256 assessment stands. [O] A weakness over `F_p` that is
neither a class invariant nor a cover would be new mathematics; none is
proposed here.
