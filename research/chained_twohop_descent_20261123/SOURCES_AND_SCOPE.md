# Two-hop descent: sources and incremental scope

Study tracked by ACTIVE-only `c44ad9de4414b57e8bbce51dfd2447477bb33a73`
and draft [crypto#4992](https://github.com/isogeny-labs/crypto/pull/4992),
stacked on completed-publication #4990 to retain its PCC prefix. No new
scientific source has been compiled or executed at this checkpoint.
[pcc:unverified]

The source audit identifies a standard obstruction, not a novel theorem:

- [Magma Richelot documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1615),
  paragraphs on double Richelot isogenies: a nontrivial descended double step
  with nonrational intermediate kernels has mixed kernel type (4,2,2).
  Its doubled subgroup has one nonzero point, necessarily rational.
- Castryck, Decru and Smith, *Hash functions from superspecial genus-2 curves
  using Richelot isogenies*, Journal of Mathematical Cryptology 14 (2020),
  section6, Definition2, [DOI](https://doi.org/10.1515/jmc-2019-0021):
  the three geometric composition types are (4,4), (4,2,2) and (2,2,2,2),
  with8/6/1 extensions per first kernel. The finite kernel mechanism is not
  restricted to the superspecial setting of that paper.
- Existing crypto `CT13_DESCENT_OBSTRUCTION.md` already excludes mixed
  (4,2,2) kernels on its odd-order F17 fixture by the characteristic subgroup
  2K argument. It deliberately leaves other 2-power types outside that claim.
- Existing crypto `CT21_RICHELOT_FRONTIER.md` and
  `T20_DIRECT_NORM_LEAD_NOTES.md`, and cryptanalysis
  [CT50_BIDUAL_RETURN.md at f9c7b49e](https://github.com/isogeny-labs/cryptanalysis/blob/f9c7b49e770212c24e29ca5c87e202e2aa6e8493/experiments/chained-transfer-reference/ct50/CT50_BIDUAL_RETURN.md)
  establish the earlier six-cycle single-rational-step forced return.
  Their old Python/JS/C++ programs were read as source records, not executed.
- Existing `CUBIC_FROBENIUS_TRACE_DESCENT_05.md` treats additive trace,
  covariance and eigenlines. Averaging or projecting maps is a different
  construction from the degree16 composition considered here.
[pcc:unverified]

Two fresh preprints were located: Hung Dang, arXiv:2611.27775v1 and
arXiv:2611.23694v1, on splitting statistics and superspecial Frobenius types.
Only their abstracts were inspected; no proof, computational formula or
ordinary-curve consequence from them is used here. The shared KB tools are
not loaded. Its wrapper is tracked, but the default executable points into
the locked autoresearcher layout; no vault source or executable is opened.
Conductor remains unavailable127. The canonical experiment-protocol skill
also points into the unread locked repository, so its checker is not run and
no canonical-schema validation is claimed. The explicit study-local protocol
governs this exhaustive development qualification. [pcc:unverified]

## Proposed complete two-step corollary

Let char(k) differ from2, let A and C be abelian surfaces over k, and let
f = psi o phi be a k-defined composition of two geometric (2,2) isogenies.
Assume A[2](k) is trivial and L=ker(phi) is not Galois stable. Then
ker(f)=A[2], so f=u o [2] for a k-isomorphism u:A->C.

Written argument: K=ker(f) has order16 and exponent at most4. In type
(4,4), its four-element two-torsion subgroup equals2K and also L, hence L
is characteristic in K and Galois stable, a contradiction. In mixed type,
2K has order2 and its unique nonzero element is fixed by Galois, contradicting
the rational-two-torsion hypothesis. In the remaining type K=A[2] by order;
quotient factorization through the k-defined multiplication map gives u.
This is a standard kernel-classification corollary, proposed for explicit
structural qualification here. The geometric quotient/factorization theorem
is a written mathematical dependency, not a newly implemented Lean Picard
instance. [pcc:unverified]

The incremental lab scope is to combine the previously separated mixed-kernel
and single-rational-step restrictions into a precise two-step filter, and
check its finite kernel premises natively with an independent Lean component.
It does not assert a new literature result, nontrivial map, or solver gain.
The standard polynomial-only IDC1 checker over 2^61-1 does not certify finite
group automorphism/cardinality propositions; no unrelated polynomial identity
will be substituted as their certificate. Lean bindings and native finite
records will be reported at their actual scope, with IDC/general geometry
coverage left explicit. [pcc:unverified]

## Boundaries that must survive validation

Nonrational **individual quadratic factors** do not imply a nonstable kernel:
a Frobenius-permuted triple can define a rational Richelot kernel. The earlier
(3)(3) fixtures have such kernels and are not ruled out. A rational target
equation alone does not imply that the chosen composite morphism descends.
The corollary does not cover three or more hops, odd-degree maps, trace/norm
combinations, or first kernels already rational. On an odd-order subgroup a
scalar return preserves the DLP and has the usual inverse-of2 recovery; it
does not change the underlying destination isomorphism class. [pcc:unverified]
