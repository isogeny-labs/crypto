# Fixed-size resultant roots and two domain disproofs

Lean proves an exact criterion over every algebraically closed field K.
For positive declared sizes m,n and actual degree bounds deg(f)≤m,
deg(g)≤n, Res(f,g;m,n)=0 if and only if f,g have a common finite root in K,
or both declared leading coefficients f_m,g_n are zero. The Sylvester
sizes stay fixed when a polynomial loses degree. [pcc:unverified]

Two exact Lean controls disprove broader readings. Over every field, f=g=1
at declared sizes2,2has resultant0 and no common finite root. Over Q,
f=g=X²+1has actual degrees2,2and leading coefficients1,1, and resultant0
but no rational root. The first control requires the extra leading-zero
case even over algebraically closed fields; the second requires the
splitting/closure condition for the converse. [pcc:unverified]

| Control | Actual degrees | Declared sizes | Both top coefficients zero? | Finite common root? |
|:--|:--|:--|:--|:--|
| 1,1over any field | 0,0 | 2,2 | Yes | No |
| X²+1paired with itself over Q | 2,2 | 2,2 | No | No |

Both resultants are exactly zero. These are proofs rather than finite
samples or solver output. [pcc:unverified]

## Proof map

| Declaration in DeclaredResultantRootsAudit | Exact obligation |
|:--|:--|
| full_left_root_iff | If nonzero f splits and g has its declared degree bound, use the actual root-product formula and nonzero leading factor |
| degree_lt_of_top_zero | Positive bound plus zero top coefficient implies actual degree strictly smaller |
| declared_root_iff | Both tops zero gives a vanishing fixed-size determinant; otherwise select a full-degree side, swapping resultants if necessary |
| constant_padding_control / affine_only_claim_false | Concrete constant counterexample and direct universal negation |
| rationalNoRoot_degree / rational_control | Actual rational polynomial, degree/leading coefficient, zero self-resultant and no rational root |
| declared_resultant_root_audit_certificate | Fully quantified corrected theorem plus both domain controls |

The full-left lemma needs only f.Splits. The final criterion obtains
splitting from Mathlib's standard IsAlgClosed class. The rational control
uses square nonnegativity to rule out a rational zero; it does not assume
an irrationality claim or an oracle computation. Repeated roots and small
characteristic need no separate exception. [pcc:unverified]

The leading-zero alternative is often interpreted as a common root at
infinity of degree-padded homogeneous forms. The formal statement here is
the explicit finite-root/leading-coefficient alternative. It does not
construct projective space or prove the all-arity geometric Semaev
zero-sum theorem. The latter still needs actual canonical evaluation and
point-list induction. [pcc:unverified]

## Sources and reproducibility

Reviewed Mathlib Resultant.Basic475–489(root product),334–353(degree
padding), IsAlgClosed.Basic's splitting definition and Polynomial.Roots;
also the existing actual GeneralS4common-root/gluing proof and its F5
constant-polynomial counterexample. Retained Kosters–Yeo/Semaev primary
read scopes remain unchanged, including original Semaev2004unread403.
No source promotion or novelty claim. [pcc:unverified]

Whole census gate431passes, zero failures/zero trusted-native exceptions.
Full raw receipts are preserved losslessly with unpack/hash equality checks.
Lean4.35.0/Mathlib56a00585 are pinned. Complete source02 compiles without
warnings; reported declarations use only propext/Classical.choice/
Quot.sound. No hole, custom axiom, native decision exception or linter
waiver. Attempt01failed only in coefficient-one control simplification;
its complete source/log/exit are retained. Exact gate/source and actual
packet standing are recorded separately in declared-root-sources.json and
declared-root-verification.json. No native experiment, performance
measurement or paid compute. [pcc:unverified]

Local source commit `1029720b` precedes packet `PCC-20261110-2b8991`.
Its full producer-local check passes at04:27:13UTC on762f5fc120c9; signed
standing remains unverified. See [actual bindings](declared-root-verification.json),
[inspected two-page report](reports/proof-20261110-declared-roots/report.pdf),
and [editable report](reports/proof-20261110-declared-roots/report.md).
[pcc:unverified]
