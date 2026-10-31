# Scoped Boundaries for Elliptic-Curve Index Calculus

**Research draft, anonymized, not independently reviewed, not submitted, not published, and not cleared for either.** Item 19 remains partial. This draft introduces no scientific trials and claims no new conservation principle, novelty or official research-state decision. Fresh Lean checks and an ordered-domain application bridge are recorded below.

[boundaries.tex](boundaries.tex) and its sections are a substantive six-result companion to the existing programme manuscript in `../ecdlp-index-calculus-program/` and the earlier `crypto-autoresearcher:papers/semaev-conservation-specialization/` draft. The original audit's statement that no ECDLP manuscript existed is not current.

The paper distinguishes what each source establishes:

- Exact polynomial counterexamples to the true/fake first-fall comparison under explicit conventions. Current tests correct the finding prose's overly broad claim that the literal `>=` reading always voids the refutation; the true-side placement matters.
- Completed degree-four mutant-closure failures in a particular binary representation. These are not an established refutation of Semaev's `d_F4` assumption. The two cited m=3 cells have `ell=ceil(n/m)`; a metric bridge is still missing.
- A conditional generic simulation schema for table-based decomposition. The concrete compact-pipeline interface and preprocessing lower-bound bridge remain obligations.
- Exact conservation of mean decomposition multiplicity, with coverage, rank and search cost kept distinct.
- The ordered Semaev cover's signed-sum monodromy and good-specialization law. A different symmetric base or an exceptional specialization is a different object.
- Generic isogeny-map lower bounds and the more restricted coordinate-aware reduction to class-joint transported challenges. A blanket “constants only” claim is not supported in composite binary extensions.

[PROOF-MAP.md](PROOF-MAP.md) gives exact hypotheses, proof status, source conflicts, reproduction pointers and Lean boundaries. [source-map.json](source-map.json) binds source revisions and Git objects; no historical source was modified.

## Lean inventory

Yield conservation already has a core formalization in `crypto:research/factor_base_designs_20261015/lean/FactorBaseDesigns/Yield.lean`: `sum_count`, `patterns_neutral` and `restrict_le`. This campaign freshly rebuilt the pinned nine-job project and reran its axiom audit. It added `research/ideas_execution_20261018/lean/IdeasExecution/OrderedYield.lean`, proving ordered-domain length B^m and total multiplicity under exact image-coverage hypotheses; the six-job build and audit pass with `[propext, Quot.sound]`. Fresh receipts live under the campaign's `runs/`. Multiset enumeration and a rational-mean division convention remain separate obligations.

`crypto:research/conductor_gap_cm_pairs_20261010/lean/GenericInvariance.lean` proves injective-transport invariance for its handle-only interface. Its four-job project and axiom audit also freshly pass. Its source explicitly excludes the ecbench representation-hashing walk implementation from that theorem's coverage. It does not formalize the current compact pipeline or the probabilistic preprocessing lower bound. The first requested formalization target is still open.

## Build

```sh
make
make check
```

The Makefile includes `../paper.mk`. No external figure generator is needed: the claim-chain diagram is typeset from text. The root session owns the PDF build, log inspection and any corrections after this source handoff. A successful build does not discharge scientific review or the remaining formalization work.

## Before submission

- [ ] Freshly transcribe Definitions 5–6 and Lemma 4 from the held Nagao PDF, including literal condition and true-side field-equation placement; resolve source/test wording drift.
- [ ] Independently replay the small polynomial counterexamples and their degree bounds.
- [ ] Bind the exact Semaev source version, system family and `d_F4` definition; supply a proved bridge or an actual counterexample in that exact quantity before alleging refutation.
- [ ] Independently verify closure saturation, unsatisfiability and non-membership for the cited negative instances.
- [ ] Audit each compact-pipeline coordinate predicate and setup operation; implement the concrete/model correspondence and correct resource counters.
- [ ] Verify the exact preprocessing theorem, advice units, success probability and automorphism assumptions from the primary publication.
- [x] Instantiate the existing Lean yield core for ordered tuples and freshly check the pinned project and axiom audit.
- [ ] Complete any multiset enumeration, concrete sum-map/target enumeration and rational-mean bridges required by the final statement.
- [ ] Complete the compact-pipeline Lean interface and lower-bound bridges; a toy model alone does not suffice.
- [ ] Obtain independent function-field/arithmetic-geometry review of the signed-sum quotient and characteristic-two scheme argument; reconcile the older broader closure language with the later qualified finding.
- [ ] Check the isogeny oracle's initial-handle/domain accounting and coordinate-aware algebraic-query/transport assumptions.
- [ ] Verify every bibliography entry, novel contribution boundary and source revision. Existing drafts are prior internal work, not evidence of external novelty.
- [ ] Resolve authorship and obtain required review before any actual external publication or submission action.
- [x] Build the PDF and pass `make check`; retain the source and PDF together. Independent scientific review remains open.

Conductor was attempted before edits and was unavailable (`command not found`, exit 127). The session exposed no knowledge-search tool, so the draft uses primary repository artifacts and indexed findings. These are tool limits, not mathematical conclusions.
