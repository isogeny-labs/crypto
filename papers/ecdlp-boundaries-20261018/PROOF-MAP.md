# Theorem, certificate and formalization map

This is an authoring inventory, not an independent scientific review. `crypto:` and `crypto-autoresearcher:` identify repositories. Exact commit and Git object identities are in `source-map.json`.

| Paper subject | Exact object / hypotheses | Existing source | Supported scope and open obligation |
|---|---|---|---|
| Nagao comparison | Finite polynomial generator sets over F2; equality versus literal `>=` condition; true-side field equations outside versus adjoined | AR held `inputs/NAGAO-2015-984/paper_fulltext.md`, Defs.5–6/Lemma4; `tools/lemma4_inside_form.py`; `tools/test_lemma4_inside_form.py`; `KN-FIND-936151` | Two finite witnesses. Equality: true3/fake2 under both placements. Current tests: literal `>=` outside3/fake2, inside2/fake2. Finding prose's blanket `>=` void is stale. Fresh primary PDF transcription and independent checker replay owed. No Proposition5/ECDLP complexity refutation is inferred. |
| Semaev/closure | Chained Boolean representation; declared subspace/target law; saturated mutant closure W4, not plain M4 and not automatically `d_F4` | crypto `research/dreg_closure_scale_20261016/{PREREGISTRATION,RESULTS}.md`; AR `KN-FIND-9cba50`, `KN-FIND-5a8d3e` | Eight m3 draws at n19/21 ell7 report saturated W4 without1. The dimensions are on `ceil(n/m)`; the degree-meter bridge remains missing. m4 ell<=4 positive closure results do not refute all degree surfaces above that range. |
| Compact genericity | Generic advice generation, known scalar maps, equality/table membership only, indexing without extra coordinate predicate | crypto `RESEARCH_IC_LEADS_AND_CONNECTIONS_20261010.md` §§0–2,7; `GenericInvariance.lean` | Conditional simulation schema. Current pipeline/code-model bridge, random-label versus handle-only distinction, exact preprocessing theorem/units/automorphism accounting still owed. The “25 bits” historical estimate is not measured anew or asserted as a theorem here. |
| Mean yield | Finite pattern domain M, function M→G; uniform complete target law for the mean | AR `experiments/EXP-FB3-001/conservation.md`, `KN-FIND-007`; crypto `FactorBaseDesigns/Yield.lean`; campaign `IdeasExecution/OrderedYield.lean` and `runs/factor-base-lean-{build.log,axioms.txt}`, `runs/ordered-yield-{build.log,axioms.txt}` | Exact sum of fibre counts =|M|. Fresh core build/axiom audit and the ordered-domain application bridge passed. Tuple/multiset/typed convention, concrete sum-map/complete-target instantiation and nonempty target cardinal for rational mean division remain separate obligations. No bound on search cost, rank or nonuniform-target mean is inferred. |
| Monodromy | Ordered independent x parameters; separable degree2 inversion quotient; generic distinct signed sums modulo diagonal; good finite specializations | AR `papers/semaev-conservation-specialization/paper.tex`; `KN-FIND-a8990a`, `KN-FIND-6edecd` | Elementary abelian regular group C2^(m−2); rational factor-base locus completely split. Full independent specialist proof/novelty review remains owed. Older `KN-FIND-a1f3c2` broad closure language is not adopted. No result about an unspecified symmetric parameter base or relation-search cost. |
| Isogeny oracles | GGM: independent subgroup encodings and known action scalars. AGM: algebraically represented oracle inputs plus class-joint challenges and explicit transport for single-instance reduction | crypto `RESEARCH_ISOGENY_FIBERS_ECDLP.md` Thm15.1,§17.7; `research/isogeny_fibers_20261010/round4/16_7_agm_isogeny_oracles/RESULTS.md` | Linear-form generic simulation; AGM relative equivalence to transported challenges. Fix initial handles/domain accounting before using exact success constants. Composite binary isogeny classes can change the available attack exponent; prime-field assertion is conjectural in source. |

## Exact existing Lean core

`crypto:research/factor_base_designs_20261015/lean/FactorBaseDesigns/Yield.lean`:

- `sum_count (D : List τ) (φ : τ → G) {Gs : List G}` assumes decidable equality on G, `Gs.Nodup` and `∀ x∈D, φ x∈Gs`; concludes `(Gs.map (count D φ)).sum = D.length`.
- `patterns_neutral` assumes equal domain-list lengths and image coverage in the same nonduplicate target list; concludes equal total counts.
- `restrict_le` filters the domain list and proves pointwise count does not increase.
- Committed `proof_logs/axioms.txt` lists `[propext, Quot.sound]` for these three declarations; committed `lake_build.txt` reports success. The present campaign also freshly rebuilt all nine jobs and reran the axiom audit; see `research/ideas_execution_20261018/runs/factor-base-lean-{build.log,axioms.txt}`.
- Toolchain: `leanprover/lean4:v4.34.1`, project has no Mathlib dependency.
- New ordered application bridge: `research/ideas_execution_20261018/lean/IdeasExecution/OrderedYield.lean` proves Cartesian-power length, ordered total yield and equal-cardinality ordered-pattern neutrality. Six-job build passes; all three audited declarations use only `[propext, Quot.sound]`. Multiset enumeration, concrete sum-map/complete target instantiation and nonempty target cardinal for rational mean division remain open.

`crypto:research/conductor_gap_cm_pairs_20261010/lean/GenericInvariance.lean`:

- `Alg.next` takes cost, previous equality bits and random tape, not a group element/encoding.
- `run_map` proves bounded runs commute with an injective operation-preserving map; `dlog_invariant` gives the two-input specialization.
- Its correction dated20261016 explicitly excludes the representation-hashing ecbench rho/kangaroo implementations from its theorem coverage.
- It contains no compact-pipeline transition system or preprocessing success lower bound. Those are separate missing formalization targets.
- This campaign freshly rebuilt all four jobs and reran the axiom audit; successful checking preserves, rather than expands, the stated interface.

No new Lean source with unproved axioms or `sorry` is supplied as a substitute.

## Source conflicts preserved

1. `KN-FIND-936151` says a literal `>=` reading universally voids the counterexample. Current pinned test class narrows that to the true-side-inside placement. The held lemma text names the outside placement. The manuscript includes both axes and leaves final textual adjudication pending.
2. The original user phrase “Assumption1 fails off the diagonal” is broader/different than the read sources. The closure study's n19/21,m3,ell7 dimensions are diagonal, but its W4 metric is distinct. Neither fact permits a direct `d_F4` refutation claim.
3. The step-back generic note states constant domination by preprocessing rho; the draft adopts only the conditional generic-interface simulation and requires a measured constant comparison separately.
4. The transfer note's raw product-of-fibres display omits an explicit codomain-image restriction; the existing programme manuscript already uses the correct full-pullback statement. This draft states that finite-group version.
5. The older monodromy finding declares a broad open problem closed; the later independent-corroboration finding explicitly declines closure of its relation-rate and imprimitivity parts.
