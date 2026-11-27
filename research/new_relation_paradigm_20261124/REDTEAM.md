# Red-team review: Track 2 "new relation paradigm" proposals (2026-11-25)

Reviewer: Red Team. Scope: the frozen desk artifacts
`research/new_relation_paradigm_20261124/{PROPOSALS.md, SYNTHESIS.md, ideas.md}`
at working-tree state (PROPOSALS.md blob `c17c705284`, byte-identical to its
committed version in `85d64d03f9`; SYNTHESIS.md and ideas.md committed in
`de026cd612`). All three are self-labelled "proposals only", every figure
`[pcc:unverified]`. Nothing here changes a status, reruns anything, or declares
a result supported or refuted; this challenges the arguments and names the
cheapest settling check per finding. Every assertion below is `[pcc:unverified]`.

Verdicts: **NRP-3 STANDS-WITH-CAVEAT. NRP-1 STANDS-WITH-CAVEAT (known-null,
upside overstated). "Nothing survives" STANDS-WITH-CAVEAT (one family is
heuristic-closed, not proof-closed). Genus-3 transfer closure (SYN-C):
SUPPORTED.**

---

## A. NRP-3 (Lemma A for F_{2^m}) — STANDS-WITH-CAVEAT

The theorem proposal is legitimate and worth stating, but three of its
load-bearing claims are overstated. Severity order below.

### A1 (most severe, *unsupported*): for the Koblitz target the door is largely already closed by existing repo work, not 50%-open.

NRP-3 is pitched as "opens or closes the cheap-nonalgebraic-map door" for the
binary targets NRP-1/NRP-2 attack, i.e. Koblitz curves. For a Koblitz curve
`E/F_2` the field map `x -> x^2` on coordinates **is** the Frobenius
endomorphism `tau in End(E)` (`tau^2 - t*tau + 2 = 0`), which is algebraic of
degree 2, and on `<P>` acts as the known scalar `lambda` (NRP-3's own mechanism
says `tau` acts as `lambda`). Therefore every "cheap high-degree additive map
built from field operations and Frobenius powers" that NRP-3 names as the
threat class — `sum a_i x^{2^i}` applied to coordinates and required additive on
`E` — is, on a Koblitz curve, a `Z[tau]`-combination: an honest algebraic
endomorphism acting on `<P>` as the known scalar `sum a_i lambda^i mod r`. That
is exactly the *already-recorded* killed family in `PROPOSALS.md` section 4,
row "Class-field theory ... all structure on `<P>` is multiplication by known
scalars of `Z[tau]/(Phi_m(tau))`", grounded in
`research/ecdlp_subexp_ideas_20261110/README.md` section 1.2 and the closed
lead `LEAD-STR-class-group-ecdlp-bridge`.

So the "cheap-nonalgebraic-map door" NRP-3 claims to open/close is, for the
Koblitz case, **not** a fresh ~50% theorem: the maps it worries about reduce to
known scalars. The genuinely residual part is only (i) maps into `G_a` — killed
trivially (odd-order group into an exponent-2 group is zero; NRP-3's own toy
check states this), (ii) maps into `G_m`/tori — MOV/embedding degree, covered
by Corollary B(b), and (iii) maps that use `x -> x^2` in a way that does *not*
reduce to an `E`-endomorphism (coordinate-level mixing not through the group
law), which is the binary analogue of **Conjecture A'** — and A' is labelled
`[proposed]`, open, in the prime note (`ecdlp_subexp_ideas_20261110` section 1.2,
line 159). NRP-3(a), with its "straight-line program of length poly(m)"
hypothesis, is the binary A': an open conjecture, not the binary analogue of a
*proved* lemma. [pcc:unverified]

Classification: *unsupported* (the "50% provable / highest information value"
ranking rests on the door being more open than the repo's own records leave it).

Cheapest settling check: enumerate the residual map class. Write the three-line
reduction "SLP in field ops + `x->x^2` on a Koblitz curve, additive on `E`,
whose difference map `h` vanishes identically => element of `Z[tau]` => scalar
`lambda^*` on `<P>`", and confirm the only escape is a map additive on `<P>`
but **not** on `E` (binary A'). That is a desk check against
`ecdlp_subexp_ideas_20261110` section 1.2; it costs no compute and decides
whether NRP-3 is a new door or a restatement of the `Z[tau]` closure.

### A2 (severe, *unsupported*): the proposed Weil-restriction "repair" has a stated-but-unpriced gap — Lang–Weil over F_2 has no room.

NRP-3's mechanism proposes to repair Lemma A by moving to the Weil restriction
`A = Res_{F_{2^m}/F_2} E` (dimension `m` over `F_2`, where Frobenius is a
low-degree endomorphism) and applying Lang–Weil there. The unpriced step: the
points `E(F_{2^m})` become `A(F_2)`, so the natural point count is over the
**base field F_2**. Lang–Weil / Weil bounds need a *large* field for the main
term to dominate the degree-weighted error term. The difference map `h` on
`A x A` (dimension `2m` over `F_2`) must vanish on `~r^2 ~ 2^{2m}` points; a
proper subvariety of degree `D` over `F_2` can carry on the order of
`D * 2^{2m-1}` points, so the vanishing argument needs `D < 2`, i.e. it gives
nothing for any nontrivial `D`. The prime-field Lemma A worked precisely
because `p` was large (`d < p/4` had room); the Weil-restriction move **trades
the large field `F_{2^m}` for the tiny base `F_2`** and loses that room. NRP-3
says "I do not know whether that repair goes through" — correct, and this is the
concrete reason to doubt it. The `probability_counterexample: 0.005` /
`probability_survives: 0.5` numbers do not price this gap.

Classification: *unsupported* (the repair is asserted as the expected route with
0.5 success, but the single quantitative obstruction to it is unpriced).

Cheapest settling check: write the Lang–Weil inequality for `h` on `A x A` over
`F_{2^d}` and solve for the largest `d`/degree at which the main term beats the [pcc:unverified]
error. If that forces `d` close to `m` (counting over the full extension, not
`F_2`), the "repair over the Weil restriction" claim collapses back to the
original large-field count and buys nothing new. Desk, no compute.

### A3 (moderate, *unsupported foundation*): NRP-3 extends a prime-field lemma whose own proof sketch is unreviewed.

`ecdlp_subexp_ideas_20261110` section 7 states plainly: "Lemma A is `[derived]`
and its proof sketch has not been reviewed by anyone else; the order formula for
`#L(F_p)` and Chevalley's theorem are `[literature, recalled]`." NRP-3 builds a
characteristic-2 extension on top of that unreviewed base and inherits its risk.
Probe P1 (a Lean statement of the prime-field Lang–Weil count) is the
precondition that is not yet met. NRP-3's `probability_survives: 0.5` is, by its
own mechanism text, a confessed coin-flip ("I do not know whether that repair
goes through"), not a defended estimate; it should not be read as evidence the
statement is near-provable.

Net: NRP-3 is a legitimate, honestly-caveated theorem *proposal*, and its two
explicit risks ("closes a door, not an algorithm; moves no tier") are correctly
stated. The caveat is that its headline framing (ranked #1, "highest
information value per unit cost", "~50% provable") overstates both the openness
of the door for the Koblitz target (A1) and the tractability of the repair (A2),
on top of an unreviewed prime-field foundation (A3). Settle A1 and A2 at the
desk before any Lean effort is scheduled.

---

## B. NRP-1 (Frobenius-equivariant Fourier test along <P>) — STANDS-WITH-CAVEAT (known-null; the "if it fired" upside is overstated)

### B1 (*unsupported upside*, the hidden flaw asked for): a signal would not straightforwardly give a sieve, and the null is forced by a *recorded* bound.

NRP-1 predicts a null with probability-of-signal 0.01 and says "Weil-type bounds
predict none" — that part is honest. The hidden flaw is in the
`exponent_argument`: "If `|Fhat(xi)| >= c |V|/sqrt(r) * (growth)` at a computable
`xi`, then ... a sieve over `k` with `O(1)` amortised work per candidate." Two
unpriced gaps:

1. **Frequency-knowability / subgroup-specificity.** The test enumerates *known*
   `k` over `<G>` and measures a bias in `1[x(kG) in V]` as a function of `k`.
   The attack has `R = Q + aG` with `a` **unknown**; a bias at frequency `xi`
   in the known-`k` indicator does not convert to a sieve on the unknown `a`
   unless `xi` and the induced membership probability can be evaluated without
   solving the DL. The "`O(1)` amortised per candidate" is asserted, not
   derived. This is the same gap that makes the Smart/anomalous `lambda` row the
   *only* detectable function in the measured prime-field analogue
   (`research/coord_spectrum_20261110`, P2: only the planted/anomalous linear
   functions are seen).

2. **The obstruction is recorded, contrary to the framing.** NRP-1 and the
   ranking in PROPOSALS section 6 call this "the single operation the prior note
   leaves without a recorded obstruction." The character-sum obstruction is in
   fact recorded: `research/coord_spectrum_20261110` (F3) measured the
   prime-field version and found a null that is *flatter than random*, with the
   explicit reading "a Weil-type bound `|S_g(a,b)| <= C*sqrt(p)` sits below the
   random extreme-value maximum `~sqrt(r ln r)`", citing the Kohel–Shparlinski
   bound `eta <~ 2d*r^{-1/2}`. The binary subspace indicator `1_V` decomposes as
   `2^{-(m-l)} sum_{c in V^perp} psi(Tr(c*x))`, i.e. an average of additive
   characters of the **degree-2** coordinate function `x` — the `F_2`-linearity
   of `V` does **not** raise the geometric degree of the relevant Artin–Schreier
   sum (the trace is linear; `psi o Tr` is just the additive character of
   `F_q`). So the same Weil / Kohel–Shparlinski bound that gives the F_3 null
   applies, and the null is a *known-null*, not an unmeasured open door. The
   "degree `2^{m-1}`" that NRP-3's obstruction text invokes for linearized maps
   is a red herring for *this* sum.

Classification: *unsupported* for the upside claim (exponent consequence on a
signal) and for "no recorded obstruction"; the null prediction itself is
correctly stated and consistent with `coord_spectrum`.

### B2 (*caveat*): toy-size artifact risk is real and self-flagged.

NRP-1's own risk note — for `m in {13,19,29}` the Frobenius order `ord_m(2)`
can be below `m-1`, so Frobenius-invariant subspaces exist at toy `m` and are
absent at `m=83, 131` — means a toy "signal" is most likely an artifact, not a
structure that persists to full size. This is the `m=83` fidelity concern
(AGENTS.md): a toy instance may not stand in for the full-size case. The
proposal handles it by reporting each `m` separately and adding the
random-bijection control, which is adequate *if* run, but it caps the
informational value: a null at `m <= 29` is a bounded-search failure, and a
signal at `m <= 29` is presumptively the invariant-subspace artifact.

Net: NRP-1 is fine as a cheap null-confirming probe but should be re-described
as "measure the binary instance of an obstruction already recorded for F_p
(coord_spectrum / Kohel–Shparlinski)", not as "the one operation without a
recorded obstruction", and its exponent upside should be struck or derived.

Cheapest settling check: before any run, write the one-line reduction of
`Fhat_V(xi)` to `2^{-(m-l)} sum_{c in V^perp} S(c, xi)` with
`S(c,xi) = sum_k psi(Tr(c*x(kG))) e(xi k / r)`, and cite the Kohel–Shparlinski
bound on `S(c,xi)` (degree-2 coordinate function). If that bound applies, the
null is a theorem-backed prediction and the measurement is confirmatory only.
Desk.

---

## C. "No new relation paradigm survives the repo's closures" + the nine paper-killed families — STANDS-WITH-CAVEAT

I checked each of the nine section-4 kills against its cited closure. Seven are
real and apply as stated. One is *over-stated* (C1 below). One carries a
standing caveat (C2). The overall "nothing survives" summary is defensible as a
practical statement but should not be read as "nine proof-closures".

### C1 (the family I flag as *wrongly/over-killed*): "minimal-genus F_2-cover carrying the Weil restriction (genus ~100–300 would beat rho)". [pcc:unverified]

PROPOSALS section 4 kills this with: "GHS genus floor `2^129` at `m=131`; cyclic
and Prym constructions leave only genus-n covers with an order-n automorphism,
excluded by Riemann–Hurwitz over F_2 except `n=3`". The cited lanes do **not**
support a clean proof-kill of the whole family:

- `research/cover_decomposition_gap_search_20261018` section 4 proof-closes only
  the **cyclic** (order-131-automorphism) covers, via an exhaustive census of
  genus-3 bases with `#Jac = 131` (0 hits) plus the ramified/unramified
  twisted-action cases. For the **unstructured / noncyclic** direct carriers of
  genus 130–249 it gives only a **heuristic** (section 4.4: expected number
  `2^{-3870}` at `g=130`), explicitly "crude", not a proof.
- `problems/koblitz-modular-carrier-transfer.md` is **status: open** and states
  verbatim: "no claim that every low-genus route is blocked. Noncyclic covers,
  gluing/polarization, other map degrees and projected relation algorithms
  remain separate possibilities", and "the written cyclic argument rules out
  geometrically cyclic degree-131 carriers of genus <394". Its best constructed
  carrier is genus 45,589 with **no evaluated subgroup-preserving map**.
- `research/ecdlp_leads_20261018` says "the route is now closed for ECC2K-130"
  but that closure is the genus-≤5 **base** census (cyclic covers) plus a
  genus-4 point-count lemma — it does not exhaustively cover direct noncyclic
  carriers of genus 130–249, which are not enumerable.

So the kill is *proof-closed for covers with an order-131 automorphism* and
*heuristic-closed (plus one open problem record) for the rest*. The "excluded by
Riemann–Hurwitz over F_2 except n=3" phrasing presents the Riemann–Hurwitz /
cyclic argument as if it covers the whole family; it covers the symmetric part.
Classification: *over-stated* — practically shut (heuristic `2^{-3870}`, no
construction), but not the proof-closure the one-line obstruction implies, and
the governing problem record is formally open.

Cheapest settling check: none needed to *reopen* (the evidence is already
there). To make the kill honest, relabel the row "cyclic/Prym part
proof-closed; noncyclic/unstructured part heuristic-closed, see open problem
`koblitz-modular-carrier-transfer.md`", and state the heuristic figure
`2^{-3870}` rather than citing Riemann–Hurwitz as the whole obstruction.

### C2 (*caveat, not a kill failure*): "isogeny-class lottery" kill cites an interim study.

The row kills the decomposition-friendly-representative idea with
"`3.85e-5` across 27 Elkies neighbours and 27 random curves", from
`research/isogeny_class_ic_yield_20261102`. That report's own header reads
"interim until every cell has finished", and its requirements table lists
deferred cells (P1 tier 2, P2 36/40-bit, P3a-Y). The headline invariance
figure (REPORT.md line 140: largest deviation `3.850e-5` on `D_solve`, first-fall
degree, final rank, field mults) is complete at the run scope and is
*independently* backed by KN-FIND-007 ("mean yield is a base-geometry
invariant", the lesson `L-4c9e8e66ea68` the proposals themselves apply), so the
kill survives — but it should cite the invariance result, not an "interim"
whole-study status. Classification: *supported with a standing caveat*.

### C3 (checked, real): the other seven kills apply.

- Generalized Jacobian / `F_q^*`-valued invariant: closed by `gcd(r, q-1)=1`
  (semi-abelian split on the r-part) and MOV-triviality — `ecdlp_subexp_ideas`
  section 1.1–1.3. Real. ("embedding degree 1" is loose wording for "r-part sees
  only the abelian quotient", but the conclusion holds.)
- Canonical lift / torsion in ray class field; class-field theory / elliptic
  units: both reduce to known `Z[tau]` scalars on `<P>`, closed lead
  `LEAD-TRF-padic-canonical-section` (proof) and section 1.2. Real.
- Hecke correspondences on `X_0(49)`: at `p=2` the Hecke operator is Frobenius +
  Verschiebung = existing `tau` structure. Real.
- Trace of degree-2 places / embed into `E(F_{q^2})`: buys a subfield base for a
  larger group. Real (`ecdlp_subexp_desk_20261101`).
- Rank-2 S-unit (`tau` and conjugate): orbit too large to canonicalise, constant
  factor — `ecc2k130_open7_20261114` L1. Real.
- Bilinear / MinRank reading of `m=2` decomposition: `dim(V.V) >= 2l-1`
  (Hou–Leung–Xiang), best case rho parity — `ic_m2_cycle_leads_20261030` lead 5,
  verified at REPORT.md line 93/111. Real. [pcc:unverified]

Net on "nothing survives": the three proposals NRP-1/2/3 are honestly presented
as *unmeasured doors*, not survivors, so "I did not find a new paradigm that
survives the closures" is a fair *summary*. The caveat is that it is not "nine
proof-kills": the minimal-genus-cover family (C1) is heuristic-closed with an
open problem record, and the isogeny-lottery row (C2) cites an interim study for
its number. Both should be relabelled before this inventory is cited as a
closure.

---

## D. Genus-3 transfer route closed for ECDLP (SYNTHESIS Connection C / SYN-C) — SUPPORTED

The claim "the genus-3 exponent-1 route cannot be borrowed by an elliptic
transfer for the pinned targets (prime extension degree, order preserved, no
dimension-raising map)" checks out against three independent records:

- Order preservation => hardness preservation: `problems/
  prime-field-chained-transfer-no-advantage.md` (T7 collision-equivalence, Tate;
  `phi2_* phi1^* = 0` on 300/300; 189,330 genus-2 + 28,815 genus-3 relations
  degenerate to fibres). The theorem backbone (DLP-preserving `E->E'` iff
  `#E=#E'`) is standard; the 300/300 is strong corroboration.
- Strict-subvariety screen: `research/n83_genus3_image_solver_20261031/
  DESTINATION_CALCULUS_SCREEN.md` — an order-`r~q` image in a genus-3 Jacobian of
  ambient scale `q^3` lands where Gaudry's analysis fails (ePrint 2004/073
  Remark 2); `q^{4/3} = 2^110.67` vs rho `2^40.5`, i.e. **70.17 bits above rho**.
  Correct: the Õ(q)-vs-rho exponent-1 comparison is against rho in a `q^3`
  group and cannot be reused for a dimension-1 source.
- Prime extension degree: `docs/ic/SUCCESS_CRITERIA.md` lines 284–286 ("`m=131`
  is prime, so descents that need an intermediate subfield do not apply
  directly") and line 277 (GHS genus `~2^{n-1}`); prime fields have `n=1`, no
  Weil restriction.

Caveat (minor): the genus-3 *dimension-raising* no-go (SYN-C) and the
minimal-genus *cover carrying the 130-dim factor* kill (C1) are different cost
targets — the first is about an exponent-1 *win* via a dimension-3 destination
(solidly closed), the second about a genus-130–249 carrier of `A` beating rho in
constants (heuristic-closed). SYN-C is sound; it should not be read as also
proof-closing C1's noncyclic remainder. Verdict: **SUPPORTED**, `P ~ 0` for the
pinned targets is defensible.

---

## E. Summary of classifications

| item | verdict | worst finding | type | cheapest settling check |
|:--|:--|:--|:--|:--|
| NRP-3 Lemma A char-2 | STANDS-WITH-CAVEAT | door already `Z[tau]`-closed for Koblitz (A1); Weil-restriction repair loses Lang–Weil room over F_2 (A2) | unsupported | desk: `Z[tau]` reduction (A1); Lang–Weil inequality on `A x A` (A2) |
| NRP-1 Fourier test | STANDS-WITH-CAVEAT | known-null by Kohel–Shparlinski/coord_spectrum; sieve upside not derived (B1) | unsupported upside | desk: reduce `Fhat_V` to degree-2 character sums, cite the bound |
| "nothing survives" + 9 kills | STANDS-WITH-CAVEAT | minimal-genus-cover family heuristic-closed, problem record open (C1) | over-stated | relabel row; cite `2^{-3870}` heuristic + open problem |
| genus-3 transfer closure (SYN-C) | SUPPORTED | — | — | — |

No status change is recommended or made here. The Coordinator and Validator
decide and record. The three proposals remain `proposals only`; this review
asks that NRP-3's ranking/provability, NRP-1's "no recorded obstruction"
framing and exponent upside, and section 4's minimal-genus-cover and
isogeny-lottery rows be requalified before any of them is scheduled or cited as
a closure.
