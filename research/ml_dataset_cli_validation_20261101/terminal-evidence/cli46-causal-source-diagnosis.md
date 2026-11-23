# Causal-panel source diagnosis from retained final008 evidence

Read-only inspection of `/var/tmp/ml-dataset-cli46-e0a865-final008-20261101/policy/causal` and source e0a8650be059. Structural per-cell facts and manifest hashes are saved in cli46-causal-source-diagnosis.json. No inputs, caps, recorded data, source files or outputs were changed; no generator, solver or replay was run.

## Observed channel result

The export contains four static features and four restricted targets/index rows, zero public prefix features, zero public algebraic features, eight restricted prefix records, and zero restricted algebraic records. The eight prefix records are four root ideal-equality audits plus four censored summaries. All four audits stop at native_result sequence2 with representation-not-found, 20136 coefficient operations, 630 generated rows and peak1740 retained entries. None reaches a branch feature or algebraic target row. The bounded equality search reports unknown, not exhaustion of its ten-million-operation or8192-row budget.

All four root systems contain five polynomials of degrees [4,4,2,2,2]. All four returned native bases are exactly input rows [4,3,2], with degrees [2,2,2], pairs_above_bound=2, steps=0 and an empty trace step list. Total retained event counts are31,357,471,648 respectively. Cell0 is a native-deadline budget refusal; cells1–3 have independently verified positive decompositions. Those outcomes do not imply equality of the returned basis ideal to the original input ideal. The failure affects all four at the same first node and therefore is not explained by cell0's deadline.

## Concrete mechanism

`f4_fp.rs::gm_update` around2010 retires an older output generator as soon as the new leading monomial divides its leading monomial. Its comment explicitly requires processing their pair before the generating-set reduction is justified. `add` around3095 places degree-above-bound pairs in the deferred list; these cases have two such pairs under frozen max_degree3. The report closure around2995 returns only alive polynomials. Thus the two quartic S3 constraints disappear from the reported set before their deferred reductions are processed. `VisitSearch::walk` around5005 recursively specializes that returned set. The original-root candidate check around4834 prevents accepting spurious PDP roots, but does not restore the missing ideal equality.

`prime_chain_prefix.rs` around559 asks for two-way equivalence between the complete incoming system and this returned set. `prefix_algebra::equivalent` around552 first attempts to express every incoming generator in the returned set. The certificate is absent; consequently algebraic.emit_node (around575) is skipped, and the prefix interpreter breaks before branch handling (around596). The exporter is wired and retains the reason correctly. This is an actual generating-system/representation mismatch, not missing static factor-base identity, unsupported modulus, lost raw events, or a generic exporter dispatch omission.

## Can existing native provenance fix it?

Not by supplying a missing equality witness for this unchanged returned set. The root performs zero elimination steps. Existing F4Trace contains input leading monomials, useful-row basis indices/monomial multipliers and new leading monomials; it is a remake trace, not full field-coefficient combination provenance. The quadratic returned rows each have immediate one-way identity witnesses as original rows4,3,2. That does not establish the reverse containment required for honest branch-state equivalence. Raising the bounded search cap cannot justify treating an unproved relaxed system as equal.

A correct minimal producer construction is an explicit ideal-preserving generating set for the traced visitor: retain normalized original/retired generators whenever a truncated or interrupted computation has unprocessed pairs, alongside verified derived generators, and use that same set in the reported event and recursive specialization. This can be scoped to the traced visitor instead of silently changing all completed F4 basis consumers. Keep the existing degree/time/work budgets; retaining input equations does not authorize processing higher-degree pairs. Record any extra serialization/storage costs under existing caps and retain refusals. At these zero-step roots, a permutation/scaling certificate is direct and small because all generators remain original inputs. General derived rows require actual coefficient-combination provenance (or the existing bounded independent search); a Boolean contributor set is not a replacement.

A separate improvement can admit one-way verified algebraic consequences independently of two-way equality, since the three quadratic identities are valid. That alone cannot satisfy the frozen requirement for both causal and algebraic channels and must not be presented as a fix for branch equality. Preserve the current failed run and its unknown summaries.

## Meaningful next implementation controls

- Pin all four retained original systems, returned bases and manifests as regression inputs. Confirm the old root set remains uncertified and no branch features are emitted merely because a PDP witness exists.
- On the corrected producer, independently verify both containment directions for the preserved generating set, with source-row order/scaling transformations checked exactly; mutate generator indices, coefficients and omitted quartics to force rejection.
- Confirm the reported generating set is exactly the set specialized for the next recursion node, and reconstruct branch choice from the same pre-action state. Do not certify a union while the engine still branches on an unrelated relaxed representation.
- Exercise n7 both twists and both recorded policies under the unchanged degree3/seed42/work16384 envelopes. Treat any new resource refusal as evidence, not a reason to resample or increase caps. A fresh protocol/run would be separate from this immutable failed attempt.
- Retain input and partial proof data when provenance, serialization or verifier budgets refuse, with no provisional public feature emission.

The concrete next change is producer-side ideal preservation plus independently verified direct identities, followed by causal replay over that truthful representation. The original nonempty requirement remains unchanged.
