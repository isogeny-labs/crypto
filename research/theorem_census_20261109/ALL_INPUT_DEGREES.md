# Every formal input degree in the canonical recursive family

Lean proves sharp individual input degree2^(m−2)for every aritym≥3and every
field, in the actual canonical recursive resultant family. Degrees are read
by explicitly constructed polynomial-ring variable permutations. The combined
binding includes actual selected-coordinate images and a Lean case split
covering every Fin(m)input position. No abstract degree-growing sequence is
assumed. [pcc:unverified]

This completes the previously open older-input degree component. It does
not prove full permutation invariance, all-arity geometric zero-sum
recursion or irreducibility. Formal variables remain independent; the older
actual smooth F5specialization to a constant remains a counterexample to
universal degree preservation after field evaluation. [pcc:unverified]

## Exact coverage

Stage s has aritys+3and zero-based positions0throughs+2. [pcc:unverified]

| Input family | Actual degree read | Sharp degree |
|:--|:--|:--|
| Arity3first/second | Actual cyclic three-variable ring equivalence, once/twice | 2 |
| First/second at stage n+1 | Explicit scalar-fixing S4seed, recursively lifted swaps, outer swap | 2^(n+2) |
| Introduced at position start+2, current stage start+n+1 | Actual bornVariable sent to outer coefficient X; actual canonical polynomial mapped by its ring equivalence | 2^(start+n+2) |
| Last at stage s | Existing canonical polynomial's ordinary outer degree | 2^(s+1) |

The Lean lemma input_position_cases proves every position is first, second,
last, or introduced with precisely these indices. Substituting m=s+3gives
2^(m−2)in every case. The main binding is
RecursiveSemaevAllDegreesAudit.all_input_degree_audit_certificate.
[pcc:unverified]

## Proof map

| Module/declaration | Actual mathematical obligation |
|:--|:--|
| RecursiveSemaevTransport.focusTransport / bornVariable / focusTransport_bornVariable | Ring equivalence constructed from actual bivariate swaps/mapEquiv; identifies selected input |
| Transport.focusTransport_scalar / resultantStep_map / focused_succ | Embedded curve scalars fixed; actual fixed-size resultant commutes with coefficient transport; actual family recurrence |
| Transport.focused_nonzero / focused_elimination_degree / transported_degree | Actual mapped polynomial remains nonzero with correct elimination degree; exact future degree |
| RecursiveSemaevSeededTransport.seededTransport / seededTransport_coordinate | Same construction from an actual initial ring equivalence; follows an actual selected initial variable |
| SeededTransport.seeded_succ / seeded_degree | Actual recurrence, degree2^n times the actual mapped initial degree, with explicit scalar-fixing premise |
| RecursiveSemaevInitialDegrees.seedZero / seedOne / canonicalS4_ring | Concrete cycle/swap seeds and actual canonical S4formula |
| InitialDegrees.reading_zero_S4 / reading_one_S4 | Actually relabeled S4equals old symbolicAt0/1; no cross-symmetry assumption |
| InitialDegrees.first_input_degree / second_input_degree | Old checked allfourS4degree4base plus actual seeded recurrence |
| RecursiveSemaevAllDegrees.raw_base_ring / raw_base_cycle | Actual S3formula and cyclic polynomial identity, in raw polynomial rings |
| AllDegrees.base_first_degree / base_second_degree / base_second_coordinate | Checked arity3degrees and second-coordinate identification |
| AllDegrees.input_position_cases / all_input_degree_audit_certificate | Formal finite-position coverage and fully quantified combined theorem |

The earlier sharp surviving-variable step is reused without modification:
Dquadratic Sylvester columns have old-degree0, two fcolumns have degree≤d;
the coefficient at2d is another actual fixed-size resultant, proved nonzero.
Transport fixes embedded curve coefficients so they are constant in the
focused variable. All declared Sylvester sizes are retained, including
when a leading polynomial's elimination degree drops. [pcc:unverified]

No generic regularity, unproved irreducibility, S4cross-permutation
invariance or geometric zero-sum recursion is an assumption of these
proofs. The scalar-fixing seed premises are proved for both concrete
initial-coordinate seeds before they are used. [pcc:unverified]

## Evidence and sources

Whole census gate passes423declarations with zero failures and zero
trusted-native exceptions. The exact five-file gate is retained losslessly
with unpack/hash equality checks. Lean4.35.0 and Mathlib56a00585 are pinned. The fresh lean4checker tool is unavailable; actual Lean kernel compilation
and the complete transitive axiom gate are used. The four final modules compile
without warnings and print only propext/Classical.choice/Quot.sound for
reported declarations. No hole, custom axiom, native decision exception or
linter waiver. All16attempts, including failed elaborations and the180second
initial full-expansion timeout, are preserved losslessly with exact sources,
raw logs, exit status and unpack/hash equality checks. [pcc:unverified]

The source plan is recursive-all-input-sources.json; actual source-before-
packet bindings and unsigned standing are in recursive-all-input-verification.json.
Previously issued files remain byte-identical. Source read scopes remain
Kosters–Yeo2015§2/Proposition2.1and relevant Semaev notes. Mathlib
Bivariate.swap, Polynomial.mapEquiv, Resultant.Basic and the existing S4
all-position degree proof were reviewed. Original Semaev2004remains403/
unread; no source-status promotion or novelty claim. No native experiment,
timing or paid compute. [pcc:unverified]

Local source commit `fd9ee4c6` precedes packet `PCC-20261110-007c40`;
its full producer-local check passes at03:57:30UTC on host762f5fc120c9.
Signed standing remains unverified. See [actual bindings](recursive-all-input-verification.json),
[inspected two-page report](reports/proof-20261110-all-input-degrees/report.pdf)
and [editable report](reports/proof-20261110-all-input-degrees/report.md).
[pcc:unverified]
