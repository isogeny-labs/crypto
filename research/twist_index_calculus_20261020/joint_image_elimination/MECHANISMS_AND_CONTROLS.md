# Mechanisms to distinguish in twist-assisted index calculus

Search round: 2026-10-24. This inventory preserves the broader research
question while the registered runs finish. Proposed constructions below
are not executed results or novelty claims.

![Separate construction, rank and cost obligations](figures/joint-mechanism.svg)

## Mechanisms already tested or currently running

| Mechanism | What could improve | Required distinguishing control | Evidence boundary |
|---|---|---|---|
| Transport a relation by a cheap subgroup action | Number of represented copies | Canonical augmented rows and independent RREF | The parent study shows that jointly transported copies normalize to the same folded row |
| Add a known-log image to the factor base | Candidate domain and rank trajectory without extra image-log unknowns | Equal-size, disjoint generic image, here doubling | Completed 189+21 controls; more phi tuples do not produce more distinct rows or a consistent cold gain |
| Exploit phi's coordinate equation jointly | Polynomial construction and root-solving work | Same joint construction for doubling; strongest pure symmetry | 297 constructions validated; global native failures and source-block/implicit comparisons retained separately |
| Partition sources by U^12 | Bounded-degree elimination while keeping all phases | Same source partition for all arms; union of complete physical sets | All three regression unions match their independent signed-set/RREF oracle; local ranks pure 1/3, phi 2/3, doubling 1/3 |
| Use all ordinary-factor permutations | Smaller pure baseline representation | Full four-factor elementary coordinates, not a distinguished fourth source | Pure finite and geometric regression solves are complete and independently audited; both rank 1/3 |
| Concentrate at R=O | Reusable factor-log relations | Count zero folded rows and independent factor-log rank; then charge target descent | 21 zero fibers and three full p37 population censuses registered; no target recovery from zero-only rows |

Uniform total signed-tuple mass is conserved when the candidate domain has
the same size. A higher probability of any useful relation, lower variance,
better independent-rank yield or lower solving cost could still matter.
The finite-count Lean module now passes its transitive-axiom audit. The
domain's physical injectivity and disjointness are separate executable
hypotheses; the theorem does not assume that the old conditioned traces
were uniform.

## What symmetry can and cannot add to a folded matrix

Suppose an endomorphism preserves the chosen cyclic prime-order subgroup
and acts there by a known scalar lambda. Applying it jointly to a relation
and to its residual multiplies the already folded augmented row by lambda.
When lambda is nonzero, the projective row is unchanged. Such transported
copies can reduce search or parameter work, but supply no extra independent
row. This statement concerns joint transport of a relation; it does not
equate the sets of newly found mixed relations.

The possible gains therefore need separate measurements: fewer independent
factor-log columns, fewer different systems to solve, cheaper systems, or
new physical relations with better rank yield. The existing fold already
credits the known subgroup action. Awarding the same orbit multiplicity
again as independent yield would double count it. The parent study's
canonical augmented-row check is the executable control; the actual
endomorphism/group/rank implication is not a new fully formalized Lean
theorem here.

## Frobenius-related disjoint bases: an existing method to test fairly

Galbraith, Granger, Merz and Petit construct Frobenius-related factor bases
for subfield curves, and separately study Frobenius-invariant bases. Their
symmetry-breaking construction expresses image logs through one base's
logs. The comparison retains the actual polynomial-system solving cost;
its factorial saving is conditional on the relevant solving-cost
comparison. Invariant bases also reduce the number of unknowns. These are
established mechanisms, not a new observation from the current study.
[Primary paper](https://eprint.iacr.org/2020/1315),
[accepted manuscript, sections 3.1–3.2](https://pure.royalholloway.ac.uk/files/39395430/Accepted_Manuscript.pdf).

**Proposed extension to this exact twist.** The implemented transported
Frobenius acts on abscissae as

    tau_x(x) = t^(-(p-1)) * x^p.

The present L0 is invariant. A non-eigenvector line Fp*beta can instead
have several distinct image lines. A candidate beta with several nonzero
Kummer components needs explicit field-rank, disjointness, lift, subgroup,
sign and eigenvalue checks on every factor. A four-line collector would
need a fully specified S5 system and exact physical-set/rank controls.

There is a crucial baseline issue: an orbit of distinct lines can require
more independent log columns than the already invariant L0 base, even if
it adds no columns relative to its own non-invariant source base. Compare
both its source-line control and the best invariant-line baseline, with
their actual base sizes and matrix dimensions. An advantage against the
first alone would not establish a gain against the second. This extension
is not implemented or run in the present receipts.

## Zero relations and several low-degree image maps

The predeclared [doubled-triangle control](SUPPLEMENT_ZERO_DOUBLING_IDENTITY.md)
predicts genuine physical zero relations whose folded rows vanish.
The separately implemented post-run checker is registered. Its native
version-2 gate passes 75 tests; the separate block-oracle correction in the
external checker passes a fresh 76-test version-3 gate. The zero checker is
designed to reconstruct them from group points and read the retained oracle
sets without repeating the oracle. The resulting check would distinguish
zero-fiber physical counts from useful precomputation rank.

The subsequent exact [pure-zero component search](reports/run-20261024T0303Z-pure-zero/REPORT.md)
finds a certificate expressing 2048 times the quartic discriminant in the
ideal of the four H4 component coefficients. Integer replay and +1
mutation rejection pass. Both new Lean component/field theorems pass;
independent IDC1h09d838b7dca9895f also passes, with 32 extra points and
both constant and multiplier mutations rejected.
The readable argument excludes a signed zero sum of four distinct
Fp-abscissa points when B has degree four and characteristic is not 2 or 3.
It does not cover mixed image coordinates, nonzero residuals or larger
relation arities. In particular, precomputation using random known
multiples of an anchor has a general residual, so this zero-only
obstruction does not rule out ordinary index-calculus precomputation.

The stronger [square-component check](reports/run-20261024T0445Z-square/REPORT.md)
finds 64*S³ in the same ideal, S=64*s4-(4*s2-s1²)². Its combined
10-theorem Lean gate passes and excludes three distinct roots, allowing
the fourth abscissa to repeat. Independent IDC1h65c11f01d0b1e67a passes
exact replay, 32 extra points and both mutation controls.
The proposed finite-Fp residual filter retains infinity and must pass
the frozen full-tape physical-presentation, unique-row, RREF and recovery
audit before use. Both mixed image controls can use it; no twist-specific
or complete-cost gain has been measured from it.

Five or more distinct ordinary factors remain an explicit open
precomputation avenue. A useful test must count independent factor-log
rank after folding, rather than assume that additional zero sums carry
information. The current certificate concerns S4 and does not exclude
that avenue. No higher-arity polynomial collector is implemented here.
The [frozen five-factor diagnostic](SUPPLEMENT_FIVE_FACTOR_ZERO.md)
specifies all 21 cases, pure/phi/doubling domains, exact partition
multiplicities and anchored factor-log rank. Its provenance amendment
reconstructs factor logs from the saved verified matrix, because the
historical `recovered` field stores only the target scalar. No five-factor
run is claimed.

Another untested construction uses several distinct known-log images in
one relation, rather than the present three-plus-one pattern. Low-degree
CM maps could make that representation cheaper than scalar-image
controls. Every proposed image needs an explicit coordinate map,
nonzero subgroup scalar, injectivity/disjointness checks and complete
costs. Enlarging the candidate domain is also available with generic
scalar images; its contribution must be separated from a twist-specific
algebraic saving. No such several-image collector has been measured here.

## Stronger algorithmic baselines remain separate work

The pinned msolve 0.11.0 help also documents `-S`, a specialized saturation
algorithm over the supported small prime fields. This is an untested
representation control: components saturated by their discriminant can
be supplied without the explicit inverse-discriminant variable. It would
need a fresh frozen input, checked projection equivalence and the same
physical-set oracle; it is not one of the currently running 21 literal
GB inputs. The exact installed help is retained in
`results/msolve-0.11.0-help.txt`. Likewise a
complete GB and a complete Fp physical decomposition are different
computations; the current external checker preserves that distinction.

Trace-zero index calculus already has a substantial literature. Gorla and
Massierer analyze relation generation, sparse linear algebra and a double
large-prime variation under stated geometric and solving heuristics; their
prototype also illustrates the gap between asymptotic formulas and actual
polynomial-system costs. Their trace-zero subgroup is a precisely defined
group, not a property automatically supplied by placing points on a twist.
[Primary paper, sections 3–4 and 6](https://arxiv.org/html/1405.1059).

The present collector has neither a new large-prime graph algorithm nor a
measured sparse black-box linear algebra implementation. Those baselines,
strong matched rho, scaling and complete prices remain required before a
general competitiveness claim. The current rung can establish bounded
construction, completeness or pipeline improvements within its declared
scope; it does not establish a new asymptotic attack.

## Graph coverage

This round adds the construction profiles and explicit pipeline diagram
in the study. The old admitted cold-cost plots retain their frozen values.
No newly admitted performance point, curve transfer or proved subgroup
edge moves the canonical IC leaderboard, performance-gain graphs or curve
registry. Those graphs are left unchanged for that reason. Failed and
incomplete runs retain their own descriptive reports.
