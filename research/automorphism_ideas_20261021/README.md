# Automorphism paper: unresolved research ideas

**2026-10-21 | Planning report | No experiments run**

## Repository fit and prior work

Reviewed against crypto base commit
`d182367040a110e2fd2422fa8f86e986f040abf9`.
The user requested PRs preserving the paper-derived ideas for both repositories.

| Existing work | Consequence for this plan |
| --- | --- |
| [Torsion/auxiliary-input note, section 7](../notes/ecdlp-general/RESEARCH_TORSION_AUXILIARY_INPUTS.md) already prices symmetry and distinguishes auxiliary inputs | Do not advertise symmetry itself as new or remove the existing comparator's symmetry. |
| [PR 123](https://github.com/aburan28/crypto/pull/123) and [PR 483](https://github.com/aburan28/crypto/pull/483) cover coordinate/orbit and automorphism-rho work | Search outcomes must identify an additional map, action or controlled ablation. PR references are prior-work pointers, not new measurements here. |
| [PR 870](https://github.com/aburan28/crypto/pull/870) and [PR 1829](https://github.com/aburan28/crypto/pull/1829) address accounting and rho-walk behavior | Preserve current matched rho accounting and audit fruitless cycles. |

This repository owns the mathematical preconditions, native certificates and
blocked experiment design. The companion cryptanalysis PR maps the same five
ideas into its existing candidate catalog and C implementation boundaries.
The first concrete next experiment is DGM-01; DGM-02 through DGM-05 are
separate gated workstreams, not evidence that a new attack exists.

The [blocked backlog card](../../backlog/XP-20261021-automorphism-ideas/README.md)
and [draft protocol](../../backlog/XP-20261021-automorphism-ideas/PROTOCOL.md)
preserve all five workstreams. Their harness, fixture choices, native checker
and frozen execution protocol are not implemented by this PR.

## Scope and evidence

This is a literature-derived research plan dated 2026-10-21, requested as ideas
for the crypto and cryptanalysis repositories. It does not introduce a solver,
run an experiment, or establish a new speedup. Study-local labels DGM-01 through
DGM-05 are planning labels, not candidate, curve, certificate, or run IDs.
Exact experimental curve/subgroup fixtures and immutable IDs remain to be
selected and frozen. Run IDs: none.

The supplied source was Duursma, Gaudry and Morain, *Speeding up the Discrete
Log Computation on Curves with Automorphisms*, ASIACRYPT 1999 [S1]. We compared
its mechanisms with repository notes and the primary literature listed below.
The distinctions worth carrying forward are explicit orbit accounting,
auxiliary-curve symmetry searches, and algebraic rejection of bad walks.
Orbit folding itself is already prior art and already appears in these repos.

![Proposed accounting: preserve the admissible set and distinguish matrix compression from the action on the target subgroup. No measured values.](orbit-compression.png)

## DGM-01: preserve the factor base, compress its logarithm unknowns

**Status: proposed ablation of an existing mechanism.** Let $H=\langle G\rangle$
have prime order $r$, and let an automorphism $\phi$ preserve $H$ with a known
scalar action $\phi(G)=[\lambda]G$. The literature's coefficient-transport rule is

$$\log_G(\phi^i(B))=\lambda^i\log_G(B)\pmod r.$$

This is a cited rule under the stated hypotheses, not a new certified identity.
Before using a new map, independently verify its action and emit the repository's
required evidence. A map merely acting on coordinates is insufficient.

Keep the complete, subgroup-filtered admissible set $\mathcal B$ for decomposition.
Map each point to an orbit representative and transport its coefficient into
the representative's column. Record both $|\mathcal B|$ and the actual number
of columns; neither an orbit representative count nor a dimension is the factor
base cardinality. Short orbits and stabilizers must be counted explicitly.
If the original base is not stable, taking its orbit closure changes the base
and must be tested as a separate treatment.

Under unchanged admissible points, decompositions remain available. An ideal
factor-$m$ reduction of columns can give a factor-$m^2$ reduction of a quadratic
linear-algebra stage; sparsity, fill, useful rank and implementation costs can
change this estimate. It is not an end-to-end speedup claim. Gaudry [S2] already
develops this factor-base idea; novelty must be measured relative to the existing
folded implementation, not a deliberately weakened unfurled baseline.

**Acceptance:** freeze the same target stream, point set and relation witnesses;
independently replay unfolded and folded equations modulo $r$; require identical
accepted decompositions and successful known-answer recovery. Compare current
folding with any proposed extension, reporting matrix rank, nonzeros, memory,
transport cost and complete online wall time. Retain failed and dependent rows.

## DGM-02: search the auxiliary curve for hidden symmetries

**Status: proposed search, with a historical positive example.** DGM's
Sakai-Sakurai example uses a coordinate translation $x\mapsto x+1$ with a
polynomial adjustment of $y$ [S1, section 4.3, Table 1]. This motivates testing affine and
fractional-linear coordinate transformations on a concrete auxiliary curve or
Jacobian, beyond the obvious negation and Frobenius candidates.

A candidate record needs the defining curve, coefficient field, exact map,
inverse and domain, proof that the defining equation is preserved, extension
across exceptional points, and the induced action on the relevant Jacobian.
Then verify that the target prime-order component is stable and determine its
scalar action without assuming the unknown target logarithm. Map discovery,
evaluation and subgroup transport all have costs.

Separate two quantities: the orbits of admissible prime divisors used by index
calculus, and the image of the automorphism group on the target subgroup used by
rho. Several formal symmetries can induce the same subgroup action. A large
coordinate group does not automatically enlarge rho's useful quotient.

**Acceptance:** produce a native replayable map/action certificate, enumerate
orbit lengths on an exact small fixture, and compare the induced subgroup
action with existing negation/Frobenius. Record bounded searches with no
additional action as bounded negatives, not nonexistence proofs. No unverified
isogeny or transfer edge is added to the curve registry.

## DGM-03: audit multiplicative walks before timing them

**Status: proposed correctness and mixing audit.** DGM motivates inexpensive
updates such as $R\mapsto R+\phi^j(R)$. Its order-four setting also illustrates
the identity $(1+\phi)(1-\phi)=2$ when $\phi^2=-1$ [S1]. Before benchmarking,
enumerate short update products in the endomorphism ring and look for products
equal to the identity, an orbit symmetry, or another small exceptional action.
A cheap step can create fruitless cycles or strong dependencies.

A usable quotient walk needs a consistent successor for every representative,
an invariant partition and distinguished-point predicate, and correct transport
of both coefficients in $R=aG+bQ$. Record cycles, deterministic escape actions,
restart histories, and useful collisions between independent starts. Check
that collision denominators are invertible before recovering a logarithm.
Short-product screening is necessary evidence, not a proof of random mixing.

Modern negation-map analyses [S3, S4] must be incorporated before adopting a
1999 walk recipe. Compare the present working walk, a deliberately enabled
multiplicative variant, and an audited variant on matched synthetic targets.
Report failure rates and online time to independently verified recovery;
additions per step alone are insufficient.

## DGM-04: keep P-192/P-224 exclusions narrow and reproducible

**Status: prior assessment awaiting a repository-native reproduction.** The
preceding paper review screened the standard NIST P-192 and P-224 parameters
[S7, S8]: their $j$ values are neither 0 nor 1728; prime-field Frobenius acts
trivially on their rational points. For an ordinary curve, the trace test
$t=p+1-r$, $\Delta=t^2-4p$ distinguishes whether the endomorphism algebra is
$\mathbb Q(i)$ or $\mathbb Q(\sqrt{-3})$. The prior assessment found neither
case for these two curves. This PR provides no native certificate and does not
promote that assessment to an independently verified repository result.

Reproduce from the exact field, model, order and generator using native integer
arithmetic, check ordinarity and the square tests, and emit the required proof
or identity evidence. The intended exclusion concerns same-$\mathbb F_p$
isogenies to the special $j=1728$ or $j=0$ ordinary classes. It does not exclude
all endomorphisms, extension-field routes, higher-dimensional transfers, or
generic negation. An exceptional curve reached in a drawing is not a useful
DLP route until its map and subgroup transport are verified.

## DGM-05: separate lifting from auxiliary-input attacks

**Status: literature-gated proposal.** DGM discusses characteristic-zero lifting
and Xedni. The primary Xedni proposal [S5] and subsequent analysis [S6] were
consulted at abstract level only in this review. The analysis describes severe
lifting/dependence obstacles and asymptotic failure of the original method.
Those abstracts do not justify closing every possible lifting variant.

Read the full papers before designing a run. State precisely which obstruction
a variant changes, how dependent lifts are produced, the height/rank costs, and
how a relation yields the target logarithm without an oracle. Treat any
rank-one/Wieferich mechanism as a separately sourced hypothesis, not an
implemented continuation of Xedni. Keep this distinct from Cheon's attack:
auxiliary powers of the unknown scalar change the input problem. Any such
assumption must appear explicitly in both the candidate and the comparator.

**Acceptance:** a complete algorithm and input contract, a worked public
known-answer example, and a native independent checker before timing or funding.
A literature gap is an open check, not evidence of a working attack.

## Measurement contract for future work

The primary comparison is the matched single-target ONLINE wall ratio
$T_{\rho,\mathrm{online}}/T_{\mathrm{candidate},\mathrm{online}}$,
on the same target, subgroup, hardware and success criterion. Report reusable
setup separately and supply full cold cost as a supplementary view. Price
canonicalization, map evaluation, relation search, rank recovery, target
descent and final verification in their appropriate intervals.

Freeze public synthetic/known-answer fixtures, code revision, seeds, resource
caps, raw logs and verifier output. Do not average away failures or present
batch amortization as a one-target win. Tiny fixtures establish correctness
only; Koblitz small-field diagnostics do not settle ECC2K-130 behavior.
No wall times, samples, confidence intervals or newly verified algebraic
certificates exist for the proposals in this report.

## Primary sources and reading status

- **S1 (full supplied paper):** Duursma, Gaudry, Morain (1999),
  [Speeding up the Discrete Log Computation on Curves with Automorphisms](https://doi.org/10.1007/978-3-540-48000-6_10),
  especially sections 2.3, 3, 4.2-4.3 and 6.
- **S2 (full paper):** Gaudry (2000),
  [An Algorithm for Solving the Discrete Log Problem on Hyperelliptic Curves](https://iacr.org/archive/eurocrypt2000/1807/18070019-new.pdf),
  section 4.2. Read through a
  [mirror of the primary paper](https://dulieu.antoanthongtin.gov.vn/tapchiantoanthongtin/82c664d3-6052-4a70-bb2c-352ea46222f2/18070019-14082017110714.pdf).
- **S3 (full paper):** Bos, Kleinjung, Lenstra,
  [On the Use of the Negation Map in the Pollard Rho Method](https://www.joppebos.com/files/negation.pdf).
- **S4 (abstract/metadata):** Bernstein, Lange, Schwabe (2011),
  [On the correct use of the negation map in the Pollard rho method](https://eprint.iacr.org/2011/003).
  Full implementation details remain a pre-experiment reading gate.
- **S5 (abstract only):** Silverman,
  [The Xedni Calculus and the Elliptic Curve Discrete Logarithm Problem](https://uwaterloo.ca/combinatorics-and-optimization/sites/default/files/uploads/documents/1999_05.pdf).
- **S6 (abstract only):**
  [Analysis of the Xedni Calculus Attack](https://uwaterloo.ca/combinatorics-and-optimization/sites/default/files/uploads/documents/1999_06.pdf).
- **S7 (parameter sections):** NIST,
  [FIPS 186-4](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf),
  historical P-192 parameters. This withdrawn edition is cited for the
  historical curve, not current deployment guidance.
- **S8 (parameter sections):** NIST,
  [SP 800-186](https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=935198),
  P-224 parameters.

## Graph and validation record

Checked the existing `docs/index-calculus-scoreboard.html` and
`docs/ic/progress-timeline.json` alongside the prior-work note. This planning
change supplies no new measured row, certified identity, curve or verified
route, so their sources and generated copies stay unchanged. No performance
result is added to `benchmarks/`.

The editable diagram source is [orbit-compression.svg](orbit-compression.svg).
The PDF is built from this README by [Makefile](Makefile). See
[VALIDATION.md](VALIDATION.md) for document checks and unavailable native gates.
