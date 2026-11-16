# Applying the Hexagon review to ECDLP index calculus

This round delivers a native relation oracle and a source-constrained lead map.
It records development checks on public fixtures, not an experiment, unknown
DLP recovery, speedup or new attack-cost bound. No scientific state is promoted.

## Concrete application

The factory previously enumerated every admissible m-tuple to test a batch of
public targets. Its new opt-in `decompose` path enumerates m−1 prefix points,
derives the last as R minus their sum, and looks it up in the actual archived
base. It preserves unordered multisets, repetitions and inverse-pair exclusion.
Both signs remain separate point identities. Returned sums are replayed.
The existing exhaustive evaluator and its frozen v1 receipts remain available.

This applies an existing generic technique, already present in
`crates/hashed-ecdl-tensor/src/finite.rs` and IC subtract oracles, to the new
multiplicative-coset, norm-one-trace and Dickson archives. It is not a new
algebraic PDP algorithm. A BTree index costs O(B log B) storage construction;
terminal work scales with the number of supplied targets and up to B^(m−1)
prefixes. Index validation, group/bit costs and setup are outside the reported
prefix/lookup counters. This is not an end-to-end runtime estimate.

For an IC caller, generate R = [u]G + [v]Q, pass only finite R and the public
base to the oracle, and turn the returned point-index multiset into a row
sum(c_j log_G(P_j)) = u + v*x modulo the subgroup prime. Preserve repeated
coefficients and signed identities. Keep u,v outside the oracle; the existing
collector/linear algebra must verify rows, rank, target coefficient and recovered
scalar. This new endpoint provides decompositions, not that full solve pipeline.

```sh
cargo build --release --locked --manifest-path tools/factor-base/Cargo.toml
FB=tools/factor-base/target/release/factor-base
$FB decompose STORE MANIFEST_SHA256 research/ecdlp_ic_leads_20261030/request-0.json /tmp/fresh-decomposition.json
$FB replay-decomposition STORE MANIFEST_SHA256 /tmp/fresh-decomposition.json
```

Use the matching manifest from the first configuration in
`tools/factor-base/examples/prime-structured.json`; requests 1 and 2 accompany
the other two configurations. These use public toy generators as targets,
so they are correctness fixtures with known logs, not independent attack trials.
Receipts bind targets, arity, cap, manifest, policy, witnesses and counters.
Cap exhaustion returns `complete: false`; absent witnesses then establish no
nonexistence. No relation-yield estimate or useful-rank value is fabricated.

## Leads and application boundaries

| Lead | Application | Current status |
| --- | --- | --- |
| Final-slot derivation | Archived structured-base relation generation | Implemented here; generic track |
| Actual graph width/locality | Choose a contractor from the actual selector graph and validated cutset | Existing bounded backend; new comparisons unperformed |
| Useful rank and partials | Odd-order elimination with raw provenance and target-bearing dependency checks | Existing collector; new campaign unperformed |
| Weighted objectives | Exact local-cost profiles as controls before heuristic base selection | Exhaustive reference only |
| Algebraic PKM towers | Include membership constraints inside the PDP system | Open; existing torus leading-form obstruction retained |
| Overlap and coverage | Deduplicate points and check actual subgroup/decomposition coverage | Existing checks required for every candidate |

`leads.json` carries exact code touchpoints, source assumptions and limits.
Prior review: `research/hexagon_cscc_20261030/`, all 22 versioned primary PDFs;
primary PKM sections 3.1–3.4; `research/prime_ic_program_20261029/README.md`;
and `research/pkm_dickson_torus_20261015/README.md`. The latter already records
that its tested torus systems share the split Dickson leading forms and do
not remove that degree obstruction. No new Lean/source proof was checked.
The learning/locality, weighted, Boolean-certificate and affine-union papers
are applied with their hypotheses; no counting exponent is transplanted onto
an unmeasured dense ECDLP encoding.

## Validation

The factory release suite passes 23 tests with one existing fixture writer
ignored; strict all-target Clippy and formatting pass. The new control compares
exact witnesses with the old exhaustive traversal for all three prime recipes
and a binary base at arities 2–4, including duplicate queries. It checks cap
censorship and receipt tampering. Both traversals use the same curve arithmetic;
this is not an independent arithmetic audit. CLI runs on all three prime
examples produce replayable decompositions. All six frozen archives and the
old evaluation receipt replay. Raw outputs and toy receipts are in `checks/`.
The source identity now includes the new oracle implementation. Historical
manifest/source identities remain immutable; new builds acquire new identities.

The original broad root-library run from the previous task is separate; it
is not used to qualify this change. Existing frontier and performance graphs
receive no new measurements or bounds and remain unchanged. This round's
diagram and PDF explain the new integration. Full ECDLP cost, unknown-target
recovery, a faster algebraic solver and independent complexity proofs remain
unperformed.

Final CLI outputs are in `checks/final/`. Earlier successful controls and the
pre-optimization `initial-decomposition.rs` are retained: their source digest
was reconstructed exactly from that file and the otherwise unchanged source
inputs. The only production difference is hoisting prefix negation outside the
query loop. No timing comparison is made.
