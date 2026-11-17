# Prime affine-chain ML projection proposal

2026-10-29. Source inventory: parent-frozen `7a1bd350b26`.
**Static graph implemented and locally validated under T804.**
Source `1565b1fecd7` and its [retained evidence](../../research/ml_dataset_20261028/validation-prime-graph-20261029/README.md) bind the software checks.
`export.rs` and `prime_chain_graph.rs` implement the static portion below as
`strict-structural-allowlist-v4`. The prefix replayer, action examples, trained
model and new solver coverage remain proposals. This extends [DESIGN.md](DESIGN.md)
and the [prime-chain contract](PRIME-F4-CHAIN-CONTRACT.md), not their scientific
claims. Earlier v3 exports remain immutable and do not gain equation graphs.

## Existing inputs and eligibility

`solvers.rs::{prime_chain_input,verify_prime_f4_chain_input}` retain and reconstruct
`prime-f4-input.json`, schema `ml-dataset.prime-f4-chain-input/v1`, strategy
`prime-affine-s3-chain-f4-visitor/v1`. This is the m=3..15 affine S3 chain, with
2m-2 variables, m-1 chain equations and m leaf-membership equations. The legacy
m=2 file has a different schema and must not enter this projection by filename.
Leaf domains are the exact signed factor-base abscissae; auxiliary domains are
all Fp. Infinity branches are not covered; negative observations concern this
encoding only. Reconstruction permits additional metadata, so it is validation,
not permission to copy an entire source object into model input.

`export.rs::feature_projection` currently rebuilds verified curve, field,
generator, target, factor-base points and symmetry/orbit fields through explicit
scalar-type allowlists. The v4 prime-chain extension adds verified static equation graphs, but no native
events. Its public
features, restricted labels and restricted source index are separate files.
`mod.rs::verify_cell(..., true)` binds recognized chains to frozen family/n/m/twist,
checks artifacts and reconstructs their equations. This does not independently
replay every native trace transition. The current exporter additionally requires
`witness.json` for eligible standard public features; that requirement does not
allow the projection to read its contents. Keep this admission behavior unless a
separate reviewed change establishes an alternative public-only admission gate.

Admit static graphs only from sealed, hash-verified cells passing the current
run/plan/configuration binding, public instance and factor-base checks, and exact
chain reconstruction. Missing or invalid mathematical inputs fail closed; an
unsupported native modulus alone does not invalidate a preserved static system.
The p<2^32 native limit remains: equations at p=2^61-1 are structural inputs,
not evidence of native solving or available action/cost labels.

## Static graph contract: `prime-chain-graph/v1`

Build a fresh allowlisted object from the verified inputs. Retain the existing
projected public mathematics, with an explicit `cyclic-subgroup-factor-base` PDP
domain, then add an encoding identifier, monomial order and the following graph.
The encoding identifier describes mathematics, not solver version or outcome.

| Node or relation | Exact content and interpretation |
|---|---|
| Field context | Prime modulus, curve a/b, target coordinates; no sampling metadata. |
| Variable nodes | Contiguous indices 0..2m-3; leaf indices 0..m-1 and auxiliary indices m..2m-3; exact role and domain reference. Names are derived from indices. |
| Equation nodes | Chain rows first in construction order, then membership rows in leaf order; kind and ordered chain incidence/leaf owner. Do not infer membership ownership from a heuristic. |
| Monomial nodes | One per distinct nonzero exponent vector in the whole system; full vector of length 2m-2, ordered lexicographically for graph IDs independently of polynomial term order. Include the all-zero vector when used. |
| Equation–monomial edges | One edge per nonzero term, carrying the exact field coefficient. No duplicated monomial in an equation; no zero coefficient or implicit monic rescaling. |
| Monomial–variable edges | Only positive exponents, carrying the exact exponent; zero entries are recoverable from dimension and absent edges. |
| Domain nodes | One sorted unique finite x-domain shared by leaves; one symbolic `all_Fp` domain for auxiliaries. Never materialize p field elements. |
| Point/domain incidence | Exact signed factor-base points and x-to-point incidence; distinguish both y signs and self-negating points. Group lifting is not implied by an x-only root. |
| Symmetry relations | Verified point-index negation/Frobenius actions and orbit membership from the existing projection, with explicit null for missing images. |

Recover the exact canonical polynomial list and domains from the graph and compare
with reconstructed input before publication. Preserve chain ordering: arbitrary
leaf permutations can change which exceptional affine branches the encoding covers.
An augmentation must explicitly transform equations, variable/domain maps and
labels together; do not assert unrestricted PDP equivalence. Over Fp, p-power
Frobenius on rational coordinates is identity; do not manufacture a nontrivial
prime-field orbit. Binary/Koblitz/PKM representations require their own future
schema and verifier and must not be relabeled as this graph.

All field elements, point coordinates, modulus, subgroup/group orders and domain
values use canonical unsigned decimal **strings**, including values below 2^53.
Grammar is `0|[1-9][0-9]*`; field elements must be less than p. Reject signs,
whitespace, leading zeros, exponent notation, fractions and non-string projected
field elements. Raw chain `domains` contain JSON u64 numbers, unlike its coefficient
strings: parse these using Rust's exact integer representation, range-check, then
serialize strings before any JavaScript/floating-point consumer sees them. For
p=2305843009213693951, p-1 must round-trip as `"2305843009213693950"`.
Small bounded indices/counts/exponents may be JSON integers with explicit u32
bounds; consumers must never route field values through IEEE-754. Any tensor
encoding uses lossless bit/limb tokens with an inverse check; scaled real-valued
approximations cannot replace canonical values or define graph identity.

Hash a specified canonical UTF-8 JSON encoding of the allowlisted mathematical
object, including schema and PDP/encoding domain, with SHA-256. Specify sorted
object keys, stable array orders and no insignificant whitespace in implementation.
The hash is a join key, not a learned token. Source paths, seeds, IDs, solver,
termination, run time and provenance never enter mathematical identity. Extra
unknown source keys are ignored, never recursively copied. Resource caps must
exclude oversized graphs with restricted reason records, not silently drop terms.

The implementation uses mathematical encoding `odd-prime-affine-s3-chain/v1`.
The inverse decoder reconstructs polynomial rows and domains from graph nodes
and edges, checks signed-point incidence and prime-field action maps, and compares
with the separately reconstructed chain input before publication. It permits
m=3..15, primes below 2^62, at most 4096 terms/monomials, 128 signed points and
8 MiB serialized graph bytes. Current chain construction further limits the
finite leaf domain to 32 distinct abscissae. Limits exclude the complete graph
with a restricted reason; they never truncate its equations. The enclosing
export row retains its existing 64 MiB limit and materializes source-cell JSON;
these bounds do not claim a total process RSS limit.

The production route requires a sealed prime/F4/m3..15 cell and the exact
`complete-prime-field-polynomial-system` JSON artifact role after central replay.
A legitimate early attempt without chain input retains restricted provenance
with no matched feature. Invalid equation/tag/role data fails export. The m=2
legacy artifact keeps its existing generic feature projection. Native elimination
need not have run for a verified static graph to be eligible; unsupported wide
moduli retain their execution status only in restricted targets.

## Proposed decision-prefix view: `prime-chain-prefix/v1`

The first eligible task is **next recursive branch attempt after a completed
native call**, not F4 pivot/pair scheduling. A second task may choose a native
strategy at a nonterminal `System` entry. Current events cannot provide faithful
intra-F4 decision examples: `NativeResult` delivers an already completed useful-row
trace, omits zero rows/private pivots and lacks pre-decision candidate snapshots.
Do not flatten it into invented earlier decision states.

Every example has a precisely replayed cut immediately before the action label.
Project only public initial graph, active original-variable mapping, fixed partial
bindings, current specialized equations, and explicitly verified state already
available at that cut. At `System`, exclude that node's later `NativeResult`. At a
branch cut, the returned polynomial set from the preceding finished native call
is available, but only its polynomial contents enter through a fresh graph
projection; omit report totals, statuses, trace lengths and useful-row selection
metadata. A finished call can be degree/staircase/deadline-truncated; its returned
set is not a complete Gröbner-basis certificate. A task requiring completeness
must establish it separately, and a timeout without a subsequent branch supplies
no branch example. Active variables have no value:
never copy stale entries from an assignment buffer. Exclude terminal systems with
no active variables and all `Candidate` payloads from public prefix inputs.

For branch learning, the action is `(original_variable, field_value)`. The legal
set is a verified finite-domain/root iterator or symbolic all-Fp iterator, with
already attempted values excluded where replay establishes that state. Do not
materialize a full field or infer the candidate set from future branch records.
The raw `Branch` event records an action already selected; its variable, value
and source are restricted labels for the cut before that event, not its features.
Previous branch choices on the active ancestor path may enter as fixed partial
bindings. Verified pre-cut iterator progress, including already attempted values
at the current parent, may determine action-domain exclusions; this is the sole
sibling-history projection. Future recorded branch events may not be used to
construct the current domain; independently reconstructed admissible values
remain permitted. Sibling subtree candidates, complete assignments and success
flags remain excluded.
No supplied witness, secret scalar, generator multiplier or membership logarithm
is needed to build the view. Exclude source/global node and sequence IDs from
learned features; use local contiguous variable/graph indices.

Current event order alone is insufficient to certify this state. A separate
prefix replayer must validate parent/child maps, specialization, basis derivation,
branch feasibility and iterator progress against the public system. Root lists
and selection state are not emitted in full today: deterministically reconstruct
and check them, or add a versioned pre-decision event in a later implementation.
If neither succeeds, exclude that prefix; do not guess. Recomputed validation work
is separately accounted and cannot be claimed as originally observed instruction
or root-work counts. Counterfactual action costs require new controlled execution;
a single observed trajectory cannot label all legal alternatives.

Freeze byte/event cut positions in restricted provenance. Changing any suffix
strictly after the cut must leave public prefix bytes and identity unchanged.
No total event count, completion marker, final degree, elapsed future time,
winning branch rank, censoring outcome or content-derived future digest is a
feature. Keep labels and joins outside model-readable input. Prefix identity
hashes only projected state, task kind and exact admissible action-domain
representation; distinct source trajectories may share one feature row with
separate restricted observed labels. Collect eligible prefixes from successes,
failures and capped runs under a frozen sampling rule, not only winning paths.

## Restricted supervision and scientific interpretation

Potential labels are the observed next admissible action, verified group-lift
success, encoding-relative completion, and measured incremental work after the
cut with exact units/scope. A retained prefix ending at a trace cap, timeout or
sink error is censored; missing suffix is not a negative label. Distinguish a
verified positive PDP witness from ECDLP recovery and from an exact algebraic
certificate. Equation reconstruction checks the input; positive PDP labels require
independent replay of the saved point/sign witness against the full group equation.
Neither check certifies native trace transitions or exact degree labels; those
need separate validation. Existing field-operation counters are observations, not retired CPU
instructions; total-run counts do not become per-action costs by attribution.
Store policy/version/caps, selection probabilities and validation receipts in
restricted provenance, even if useful later for stratified evaluation.

Models can propose branch policies, recognize repeated polynomial/domain
structure, and suggest algebraic invariants. Predictions require solver/group
replay. A degree drop or maximum visited degree does not label exact first/last
fall degree; preserve definitions, bounded observations and `unknown` separately.
No universal conjecture or SAT-solver improvement follows from this projection.
Future SAT examples need their own clause/trail pre-decision contract and proof
checks. Train only after frozen baselines, leakage controls and evaluation protocol.

## Split identity and deduplication

Keep whole-field-size components as the conservative starting point used by
`export.rs` (`field-size-nN`), so every generator/key, both twists, every m, solver,
encoding augmentation and orbit-related task at that size stays together. Feature
hashes alone are not split identities. Do not split branch prefixes or alternate
signed lifts independently from their parent component. Cross-generator public
coordinates may differ while all such instances remain correlated.

Before a new dataset release, build a restricted equivalence/link index for
verified isomorphic models, twist pairs, known isogenous models and Frobenius or
subfield/extension correspondences. Union all linked components, including links
across sizes; compute partition assignment from a frozen component manifest and
split seed. This is additional work: current export explicitly does not merge
cross-size algebraic links. If a cross-partition link cannot be resolved, withhold
the affected evaluation components rather than claim independence. Preserve fixed
held-out field sizes, field representations and encoding families as applicable;
reconcile overlap with unions before freezing the release. Never repartition an
already evaluated release after seeing labels. Keep component IDs, split seeds
and source occurrence mappings in the restricted index, not mathematical features.

## Acceptance tests required before implementation may publish

- Tiny prime systems at m=3/4/15 and both twists: graph round-trip matches every
  reconstructed coefficient, exponent, role/domain and signed-point incidence.
  Include zero constants, repeated x values across signs and empty polynomial rows
  where supported. Reject wrong dimensions, duplicate edges and invalid action maps.
- At p=2^61-1, coefficients/domains near p and 2^53 retain exact values through
  exporter and consumer. Reject rounded numbers and noncanonical decimal forms;
  exercise finite domains and symbolic all-Fp without enumerating the field.
- Alter coefficient, target, membership row, auxiliary domain, m, twist or public
  curve, even while recomputing artifact hashes: reconstruction/config binding
  rejects. Missing input/base, changed chain tags and m=2 filename collision cannot
  bypass admission. Unknown metadata must not become features.
- Inject private scalar/seed/witness/outcome fields at every source nesting level:
  projected bytes remain identical or admission rejects. Projection must succeed
  with witness payload access denied after its independent admission verification.
- Prefix replayer rejects forged parent edges, stale active bindings, invalid
  specialization/basis/branch, missing prerequisites and reordered/gapped events.
  Modify future outcomes and suffix lengths without changing the cut: features
  remain byte-identical. No same-step result appears in a pre-native example.
- Trace/sink/time caps retain censorship and partial counts; they cannot create a
  false no-PDP or exact fall-degree label. Unsupported wide-native cells can supply
  static graphs but no fabricated native decisions, completion or instruction data.
- All twists/generators/keys/orbits/augmentations/prefixes and known cross-size links
  share partitions; adversarial duplicate mathematical tasks cannot cross splits.
  IDs, provenance and labels remain inaccessible to the model input loader.

Use the established Semaev construction cited in the frozen contract:
[Semaev, 2015, sections 2–4](https://arxiv.org/html/1504.01175) and the decomposition/
lifting separation in [Gaudry's 2004 ECC slides](https://cacr.uwaterloo.ca/conferences/2004/ecc2004/gaudry.pdf).
For fall-degree claim boundaries, retain the references and precise hypotheses in
[DESIGN.md](DESIGN.md), including [Huang–Kosters–Yang–Yeo](https://arxiv.org/abs/1505.02532)
and [Generalized First Fall Degree Assumptions](https://eprint.iacr.org/2015/358).
These inherited references motivate the contract; this proposal asserts no novelty.
