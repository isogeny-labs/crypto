# msolve0.6.5 source versus F4Run

Source revision: `algebraic-solving/msolve@5e72b9d99eaea55fe87eb8ac945ec9e914a69327`
(tag v0.6.5). Local copies used for reading are outside this repository;
URLs and SHA256 pins are in sources.json. [pcc:unverified]

| Site | Observation | F4Run obligation and remaining gap |
| --- | --- | --- |
| `src/neogb/symbol.c:198–233` | `select_spairs_by_minimal_degree` sorts pairs and uses `ps[0].deg`; it checks `max_gb_degree` against this pair degree. | A pair-degree log is not itself a certificate of every row's actual total degree. Degree-compatible order and row degree preservation must be bound to the exact run. |
| `symbol.c:312–363` | Pair rows use exponent difference lcm−leading exponent and `multiplied_poly_to_matrix_row`. | Matches the monomial-times-prior-polynomial shape of `StepRows`, conditional on exponent/arithmetic/hash representation correctness and actual degree≤D. |
| `symbol.c:491–546` | `find_multiplied_reducer` tests leading-monomial divisibility, forms exponent differences, and inserts another monomial multiple. | Same shape. Need a checked correspondence between hash-table rows and MvPolynomial, plus source polynomial provenance. |
| `symbol.c:554–630` | Symbolic preprocessing closes under available reducers and constructs the matrix. | Monomial construction is visible; row-span membership and coefficient field correctness still need certificates/refinement. |
| `symbol.c:756–772` | Trace replay bypasses normal selection/preprocessing via `generate_matrix_from_trace`. | A theorem about the ordinary path cannot silently cover unchecked traces. Trace digest, multiplier and basis-index replay need validation. |
| `f4.c:430` and`:614` | Linear-algebra call sites; final reduction also occurs. | F4Run allows field-linear combinations of prior and step rows. The C arithmetic and output-span property have not been formally verified here. |
| `f4sat.c:429–458` | The comment “simplify idea from F4” occurs in the saturation routine; it replaces a saturation multiple by a previously handled divisor multiple. | Ordinary GB and saturation are separate paths. Proving saturation membership in a saturation ideal does not automatically preserve membership in the unsaturated ideal used by the existing F4Run theorem. |

Conclusion at this scope: source reading supports several syntactic row
construction obligations; it does not establish full implementation
conformance. No claim that msolve is formally verified is made.
[pcc:unverified]

The existing model is in
`research/ecdlp_weakness_lean_20261104/lean/F4Containment.lean`:
`F4Run D F G` requires `G0⊆span F` and each next basis in the span of earlier
bases and their monomial multiples of total degree at most D. Its own header
already discloses the missing implementation bridge. [pcc:unverified]
