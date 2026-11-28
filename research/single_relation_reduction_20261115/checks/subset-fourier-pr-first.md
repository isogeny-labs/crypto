Assemble finite-subset Parseval and actual ordered additive-triple counts from existing character laws. For S in a finite additive commutative group, explicit bounds |sum_(x in S)chi(x)|<=M for every nontrivial character, with M>=0, yield |T(S)-|S|^3/|G||<=M|S|. This is a conditional finite-group theorem; it assumes no elliptic analytic estimate or sampling law. [pcc:unverified]

Validation at source5b7a9d2: build004 and the canonical all-source/axiom gate audit149theorems,0failures,0trusted native exceptions. All four builds, including001/003failures, complete gate logs, source hashes and both probes are retained. Generic and actual Mathlib affine-point-group assembly probes pass with explicit finiteness/flatness. Axiom prints use only the standard foundations. Hosted CI is separate and may encounter the existing billing lock. [pcc:unverified]

Inheritance gate, scoped at consumer crypto38ae306f73 before edits:

1. Structure above: existing AddCommGroup/Fintype, Finset and AddChar G complex values.
2. Laws used as stated: AddChar.sum_apply_eq_ite/card_eq; character conjugation; finite-sum/norm laws; positive-factor cancellation. Orthogonality is inherited, not re-proved.
3. Instance: actual subset character sums and actual x+y=z triples, with no new group model.
4. Assembly: pre-edit floor-law probe at crypto4cab4d5c88, then endpoint application to generic subsets and the existing affine point group. The curve analytic input remains explicit and unformalized.
5. Missing generic components: subset energy/Parseval, triple Fourier representation and the norm/count deviation; all live in CryptoLean.Group.SubsetFourier.
6. Statement/inheritance map:

| Statements | Kind / quantified structure | Inheritance |
|---|---|---|
| charSum / returnCount | Generic definitions over actual subsets | Existing finite sums and group equality |
| energy / parseval | Generic assembly laws, shared | Existing character orthogonality and complex norm/conjugation |
| triple_identity | Generic assembly law, shared | Existing orthogonality applied to actual additive triples |
| triple_deviation_scaled / triple_deviation | Generic endpoints, shared | Explicit M>=0 and nontrivial flatness; norm bound, Parseval and positive-cardinality cancellation |

Proof chain: expand finite coefficients, replace the character sum by existing orthogonality, restructure into actual counts; isolate the trivial coefficient, bound remaining norms, replace one factor by M, apply Parseval and cancel the positive group cardinality. Only the new generic finite-sum definitions are opened in their owning assembly laws. No character internals, curve equation or external analytic theorem is unfolded/axiomatized; no proof limits or native evaluation.

Consumer: crypto PR4311. Its coordinate-interval application reads Kohel--Shparlinski2000 sections3--4 and Kim--Tibouchi's2019conference manuscript section2/Lemma3.1, restricted to x and characteristic>3. It concerns independent uniform subset inputs; correlated/adaptive lists and a fast selector remain outside this theorem. KB unavailable in that runtime. No numerical case or novelty claim.
