# Quadratic ordinate offsets: complete relations and a logarithmic selector degree law

## Requested scope and actual work

Keep searching for a factor base with good relation supply, lower structured
degree and lower complete solving work. This continuation constructs and
formally checks a family without requiring each shifted graph to split
completely. It exports a larger concrete base in the repository's ICV1/FB1
formats. No new experiment or benchmark has run. The requirement-to-evidence
table is in [README](../README.md); original P-256/17-term gates remain open.

## Construction and exact degree

On E:y²=x³+Ax+B use h_u(x)=a*x²+b*x+c+t0+sum(w_i*u_i), a≠0,
u_i²-u_i=0. Include every rational point y=±h_u(x), deduplicating labels
as actual points/columns. Keep independent k-bit labels u,v for the two
summands and all four signs e,f. For target (r,s), the full system is

    m²-x-z-r = 0
    xz-r²-rm²-A-2ms = 0
    e*h_u(x)-m(x-r)+s = 0
    f*h_v(z)-m(z-r)+s = 0
    u_i²-u_i = v_i²-v_i = 0.

Lean proves that its roots imply actual on-curve, nonsingular base points
whose sum is the target, and that every such finite-target base relation
has a root. Tangents and repeated abscissae remain covered. Infinity is
handled by the exact inverse-pair theorem. No full-split or root-oracle
assumption is used. Empty fibres supply no points.

The semantic statements quantify field-valued coordinates. A CAS may also
produce algebraic-extension roots; those are not factor-base relations.
Rational-root recovery and rejection must be charged in the experiment.
The stated canonical ideal uses the displayed generators; no unrecorded
field-equation or Boolean-ring preprocessing is inserted.

Over any field of characteristic other than2, the canonical highest-form
ideal of the actual generators normalizes to

    (m²,xz,x²-alpha*mx,z²-beta*mz,u_i²,v_i²).

Its first vanishing homogeneous degree is exactly **2k+3**. The proof
counts every selector. A degree2k+2 witness survives, and all homogeneous
polynomials of degree2k+3 vanish. See [the proof map](../FORMALIZATION.md)
for the quotient map and full hypotheses. This invariant is not an observed
or proved affine F4 degree.

## Fixed fixture

The existing curve is `icv1-fp5-tm1-a77cc6af`, y²=x³+2x+2 over F17.
Take h0=4x²+9x+9 and offsets{0,2,10,12}, encoded with weights[2,10].
The four positive fibres have sizes4,1,2,2. Their eighteen signed labels
give sixteen distinct signed points at eight abscissae:
{0,3,5,6,7,9,10,16}. Exact finite membership and cardinalities are kernel
checked. This doubles the old base's four abscissae/eight signed points.

The new FB1 object is **FB1h29cc024e755c**. Metadata remains unregistered
and the inherited group-order metadata is not newly proved. The exporter
reproduces the old FB1 identifier as an independent codec reference test.
The fixed new-column pair(0,6)+(3,1)=(13,10) is a correctness unit test,
not a target-yield measurement.

The complete k=2 selector has canonical degree7. The exact dense membership
polynomial is

    T^8+12T^7+15T^6+15T^5+9T^4+10T^3+5T.

Lean checks its equality with the product over precisely these eight x
coordinates, as a polynomial identity, and its remainder degree bound.
The same-base dense S3 presentation has canonical degree9.

The fixture was hand-derived after inspecting the earlier F17 example.
It is not a frozen random trial or evidence of natural construction yield.
Two signed labels at x=9 duplicate existing points and are charged as labels,
not independent columns or rows.

## Variable tax and exact projection

Lower degree alone is not a complete work comparison. The all-bit system
has seven variables. Its full affine monomial envelope through degree7 is
choose(14,7)=3432, versus choose(11,2)=55 for two-variable dense degree9.
These are derived ambient budgets, not observed F4 matrix widths or costs.
No speedup follows from7<9.

Exact projection of one Boolean variable satisfies

    exists u in{0,1}: d+l*u=0  iff  d*(d+l)=0,

even for l=0. Lean proves this for two independent selectors and the identity

    d*(d+l)=(d+l*u)*(d+l-l*u)+l²*(u²-u).

Thus one variable per slot can be removed without losing roots. The quartic
projected residuals' canonical degree and affine solving work remain open.
The separate evaluated-tier identity artifact **IDC1hb79cfe74b833e0be** uses
32 deterministic checks at seed1101; `proof:null` preserves that tier.
The Lean theorem separately kernel checks the universal identity.

## Experiment status and controls

[PROTOCOL.json](../PROTOCOL.json) records a prospective small-prime panel:
k0–3, deterministic hash-selected targets, every attempted fibre, the all-bit
and projected selectors, same-base dense, matched low-x/hash-x and ordinary
coordinate digit controls. Controls and experiment integration are not yet
implemented. Every output requires exact finite references and group replay;
missing operation/RAM/disk counters remain null. Preserve failures and no
replacement/resampling. No discarded branch may be billed as exhaustive.

New CPU experiments default to EKS Spot. No arbitrary-command EKS tool or
ready AWS identity is configured here. The inspected GitHub EKS workflow
accepts ecbench specifications and this standalone panel is not integrated.
No launch attempt or infrastructure failure is fabricated. The previous
local exception was specific to its old completed panel; it is not reused.
Local compilation, formal proof and unit correctness checks are development.

## Prior art and search scope

- Petit–Kosters–Messeng2016, [Algebraic approaches for the ECDLP over prime fields](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
  primary introduction and factor-base/map sections: composed-map low-degree
  membership is prior work; complexity requires solving behavior.
- Repository [24-design register](../../../factor_base_designs_20261015/README.md):
  design14 already proposes learned L(3O)/L(4O) coordinates; digit encodings
  are known. The ingredients here are not claimed novel.
- Repository [PKM tower oracle note](../../../notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md),
  especially3.4: plain lifting and local degree can hide a growing system
  and degeneracy at infinity. This motivates the full top ideal and cost tax.
- Connected `isogeny-labs/crypto-autoresearcher` atlas: primary lead YAMLs
  `LEAD-ICP-digit-presentation-solving-degree`, `LEAD-ICP-pkm-tower-algebraic-oracle`
  and `LEAD-ICP-shared-coefficient-membership`, under `knowledge/leads/ecdlp/ic-prime/`.
  Read the leads directly; no local atlas CLI is installed. Digit/null controls,
  complete root recovery and custom workload custody remain obligations.
- Earlier [three-track study](../../parallel_20261031/RESULTS.md): canonical3
  versus5 did not imply lower branch-summed reduced F4 rows. Its historical
  measurements are not reused as measurements for this candidate.
- Angdinata–Xu, [ITP2023](https://doi.org/10.4230/LIPIcs.ITP.2023.6), and the
  pinned Mathlib group implementation: actual group laws are reused rather
  than axiomatized.

Repository notes, connected atlas leads, relevant primary PKM sections and
narrow quadratic/interval/Riemann–Roch factor-base searches were reviewed.
That search does not establish absence of prior work. External novelty,
natural yield, actual solving degree and lower complete cost are not
established. No benchmark/leaderboard row is promoted; the existing measured
dashboard remains unchanged because this continuation supplies formal and
fixture evidence, with missing measurement fields explicitly null.
