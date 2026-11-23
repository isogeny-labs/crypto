# Orientation-guarded anchor completion

The public guard `bit_j(x)=bit_j(y)` repairs the older anchor family's omitted
crossings. The constructed correction preserves all source values, has an
explicit computed quotient, and gains the diagonal schedule at every retained
crossing, coefficient by coefficient. Central claims
PCC-20261114-347a54/7a81c4/e9d567 are unsigned [pcc:unverified].

## Construction and exact domain

This is the original rational/2-adic Cauchy tensor model. It is separate from
the elliptic norm coefficient chart. Put `L=2^a`,
`S={-L,...,L-1}`, `C(T)=product_(t in S)(T-t)`,
`kappa=C'(0)`, `delta=y-x`, and `0<=x,y<L`. Choose retained bits `u<=a`
and arbitrary nonnegative integer multiplicities `m_j`. Define

```
M_xy = { j<u : bit_j(x)=bit_j(y) }
B_delta = (1/kappa) product_(t in S, t!=delta)(T-t)
F_xy = product_(j in M_xy) ((2^j-T)/(2^j-delta))^m_j
Q_xy = (F_xy-1) divMonic (T-delta)
H_xy = Q_xy/kappa
W_new[x,y] = B_delta F_xy
```

If `delta=2^j`, adding `2^j` flips bit j even when carries occur. Thus every
selected denominator is nonzero. Every factor is one at delta, so the actual
computed quotient has zero remainder. The punctured nodal product gives
`C=product_(t!=delta)(T-t)*(T-delta)`. Consequently
`W_new-B_delta=C*H_xy`. No successful quotient or correction is provided as a
hypothesis. At every source node `C=0`, hence values are preserved. The
normalizer is nonzero because it is a product of nonzero source differences.
At zero the completed matrix remains the identity. This does not construct
the separate nonscalar-origin channel [pcc:unverified].

Let `M=sum_(j<u)m_j`, including unused anchors. Then
`deg F_xy<=M`, `deg W_new<=2L-1+M`, and `deg H_xy<M`, including the zero
polynomial with degree minus infinity. Thus `M<=2L` fits the earlier
`deg H<=2L-1` window. Source membership is explicit in the degree theorem;
it is derived for bounded actual indices, never assumed for a missing node.
PCC-20261114-278ad6 and the degree rows in the proof map [pcc:unverified].

## Actual coefficientwise valuation bridge

For nonzero displacement, let `s=v_2(delta)`. Integer divisibility and binary
remainders show that bit s differs, and every lower bit agrees. Thus s is
omitted by the mask and, for every selected j,
`v_2(2^j-delta)=min(j,s)`. This is derived from actual bits and actual rational
values; it is not an assumed favorable denominator. At a diagonal entry the
valuation is exactly j. A unified proof index uses s=u there, since every
selected j is less than u. PCC-20261114-d1c39d [pcc:unverified].

At the literal local chart `T=t+2^(a+1)Z`, with nonzero integer t of valuation
i<=a, the numerator has coefficientwise content at least `min(i,j)` for
j!=i, and at least i+1 for j=i. The extra bit follows because
`t=2^i*k` with odd k, so `2^i-t` is divisible by `2^(i+1)`; the linear
coefficient has valuation a+1. Zero coefficients are handled as having every
lower bound, rather than using the valuation API's default zero value.
Multiplication and finite coefficient sums propagate these exact bounds
[pcc:unverified].

Write

```
beta(i,s,j) = (i+1 if j=i else min(i,j)) - min(s,j)
G_i = m_i - sum_(j>i, j<u) (j-i)m_j
```

At a crossing, bit i agrees, so i is selected. Its contribution is at least
m_i; lower anchors contribute nonnegatively; each higher anchor costs at
worst `(j-i)m_j`. Omitted higher anchors cost zero. Thus every coefficient
of F_xy in that chart has valuation at least G_i. This includes the older
missing pair `(x,y)=(0,1)` at level i=1, analytically for every u>1.
PCC-20261114-7a81c4 [pcc:unverified].

At a noncrossing the first differing bit satisfies s<=i, making every selected
factor's content nonnegative. Above the retained range j<u<=i, all gains are
nonnegative. At zero every numerator has content j, at least the selected
denominator's valuation. Every nonzero literal source index has valuation
at most a, so this covers the endpoint -L as well. Therefore feasible
`G_i>=0` makes F_xy integral in every literal source chart.
PCC-20261114-e9d567 [pcc:unverified].

For the whole matrix, the current formal transfer is explicitly conditional:
if the actual base chart has content k, the completed chart has content k;
at a retained crossing it has content k+g whenever G_i>=g. The correction's
content is derived, but the base-content premise has not yet been discharged
in this new project. PCC-20261114-8e3afc [pcc:unverified].

## Evidence, controls and scope

Source freeze: `a970572ef271aef7c9a6b852413957fb7cff5b9c`.
Pinned Lean4.35/CryptoLean/Mathlib revisions are in the exact manifest.
Sixty-six authored declarations, failed sources and raw logs are retained.
Pinned gate:66/0 extra trusted axioms. Native literal checks:25 tests pass,
format passes. Strict Clippy:4 inherited shared SHA256 library style errors,
plus1 test-only style error; shared sources are unchanged. Fresh replay of OrientationProduct, with default imports, exits0.
The [final receipt](../../evidence/validation-001.json) pins actual logs and
checker bytes [pcc:unverified].
[Proof map](../../PROOF_MAP.md), [protocol](../../PROTOCOL.md),
[bindings](../../evidence/claim-pins.json),
[source pins](../../evidence/source-freeze.json).
All PCC records are unsigned; local checking does not manufacture signed
standing [pcc:unverified].

Literal certificate `IDC1h5e866a7c08042f35` records only the universal ring rearrangement
`c*p*(1+z*q)-c*p=(p*z)*(c*q)`. It does not certify masks, monic division,
2-adic statements, rank or an algorithm. The quantified claims use Lean.
Native artifact checks include a dropped-subtraction rejection and an
understated-degree rejection. No timing, CPU isolation, solve, scaling panel,
paid experiment or previously blocked native fixture is reported.

Prospective contract status: bit/valuation bridge and actual polynomial
construction/budgets implemented; correction-content family implemented;
absolute base-chart content and an integral weighted rank factorization
remain open. In particular, masks above i can vary inside the older source
tiles. Any tile/Walsh/moment refinement must be charged. The old necessary
Fibonacci mass is not evaded. Its retained-u scope cannot be replaced with
an all-a obstruction. There is no claimed efficient finder, extraction,
recursive complexity or complete expected bit cost [pcc:unverified].

## Sources and next proof obligation

KB unavailable; exact fallback scope is in the protocol and journal.
Primary crypto sources at `1385553ca87f95043aefd5c592c5e4103e329ddf`:

- `research/prime_pole_anchor_20261030/repeated/README.md`: prior anchor family,
  exact missing-mask counterexample and retained-u Fibonacci cost.
- `research/prime_pole_anchor_20261030/repeated/lean/PROOF_MAP.md`: existing
  abstract budget proof, not an all-entry valuation construction.
- `research/prime_jet_completion_20261030/README.md` and followup report:
  actual W+C H window and inherited rank/mode obligations.
- `backlog/XP-20261101-rank-directed-jet-completion/README.md`: draft bounded
  search, not executed or newly authorized by this proof work.
- `research/prime_common_kernel_20261030/reviews/exact-filter-packing.md`:
  exact-filter limitations; this entrywise construction supplies no packing
  escape or whole-density solver.
- Pinned Mathlib/Core primary APIs: binary remainders, monic division and
  rational valuations, used directly in the checked sources.

No novelty claim is made for interpolation, elementary valuation algebra,
Fibonacci schedules or prior repeated anchors. Bounded fallback searches do
not establish priority. Next: establish literal Cauchy base content and construct
actual integral U,V mode factors with all mask refinements and degree/precision
costs charged. See [named gap](../../../../problems/orientation-anchor-integral-rank-gap.md).

The current study diagram is explanatory and explicitly separates local
proof artifacts from open algorithmic obligations. Existing IC scoreboard,
progress timeline and norm-study charts receive no new performance values:
this component contains no runtime or solve measurement.


[Mode-accounting successor note](../../MODE_ACCOUNTING.md) keeps the additive
shift and increased tile count explicit. Three isolated successor drafts
(Lucas/factorial, a binary-subset enumeration control, and a strict
first-difference content margin) are retained separately from the66 audited
project statements. Their local elaboration is data claim
PCC-20261114-6f9128; it does not promote new rank or base-content results
[pcc:unverified].

Gate data PCC-20261114-50876e; final replay/validation data
PCC-20261114-6b0d58. All69 new records (66 theorem bindings and3 data
records) remain unsigned/default not_covered [pcc:unverified].
