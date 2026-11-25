# M31 weekly ECC research plan
Date: November 23, 2026 (America/Los_Angeles).
Status: revised plan from the manual run; proposed execution steps, with existing owners retained. All source observations and written interpretations below are PCC-unverified unless an independently signed standing is explicitly supplied.

## Objective and fixed instance
Develop a native Rust algebraic index-calculus solver for an ordinary M31 elliptic curve and compare complete cold single-target cost against the strongest applicable reproducible IC baseline and optimized negation-map Pollard rho. New experiments use p = 2147483647; they do not fall back to smaller source fields.

The retained B677 reference uses y^2 = x^3 + 847514846*x + 2087444724, subgroup order r = 2147446831, G = (625037715,2100627251), and Q = (976979610,1228979761). These are inherited source parameters, not a fresh point-count/primality certificate from this run. Another M31 curve in the producer study uses different coefficients and order: field equality does not make the two instances comparable. Source: [capture protocol](https://github.com/isogeny-labs/crypto/blob/45eb2ec79bc11168dc434e8a235ea358cf674f9f/research/lp_graph_algorithms_20261123/M31_CAPTURE_REPLAY_20261124.md).

## Changes since the earlier November 23 plan
- The retained M31 generic-decomposition capture is now a concrete integration control: ten queries, 645 stored rows, null recovered scalar, exact SHA-256 e2407b695d4df7a7c9d71eae00895a643aebe3afab50d1c338e58778884dabab. Its 613 eligible rows and 32 exclusions are protocol/producer fields awaiting the current native replay. The separate reported eleven-query recovery must not be substituted for this input. [PR 5076](https://github.com/isogeny-labs/crypto/pull/5076), [PR 5097](https://github.com/isogeny-labs/crypto/pull/5097).
- The RR5 payload report now records 19 checker tests passing at its named development bundle. Scientific B677 extraction is still unrun under its recorded execution dependencies. Source review identifies a four-signed-leaf input limit; projected graph rows must retain an original-row witness path. [Payload report](https://github.com/isogeny-labs/crypto/blob/0a3932b2801738c14fd166a7239e519a587e8346/research/prime31_four_summand_20261122/scaling-b677/payload-integration/README.md).
- The decomposition lineage has advanced to compact projective/Veronese charts, but its M31 rank diagnostics do not supply coupled M31 projected-set correctness or measured global F4 cost. [PR 5087](https://github.com/isogeny-labs/crypto/pull/5087).
- Existing transfer controls include M31 genus-two split-Jacobian work; later F103 controls remain F103. The missing ordinary-elliptic source map and complete solving-cost obligations remain explicit. [PR 4997](https://github.com/isogeny-labs/crypto/pull/4997), [PR 5066](https://github.com/isogeny-labs/crypto/pull/5066).

## Ranked blockers and decisions
1. Native execution admission and build/source custody: retain the existing RR5 payload/image/checkpoint/budget owners; resolve each recorded dependency in that lane before its scientific run. A read-only report or public commit is not execution admission.
2. Authentic nonenumerative relation extraction: the saved exhaustive-pair capture is a useful generic control but does not discharge this algebraic requirement. Keep original signed rows, useful independent rank, target recoverability and complete extraction costs distinct.
3. Matched complete-cost evidence: no new same-target IC/rho comparison was executed in this manual pass. Keep setup, rejected attempts, filtering, algebra, recovery, verification and memory in one exclusive ledger. M31 data alone cannot establish asymptotics as field size grows.

## Exactly three prioritized next steps

### 1. Qualify the source-bound M31 integration and comparison inputs
Continue the existing PR 5076 and PR 5099 owners. Bind the exact saved ten-query input to its full 677-point table and original query coefficients; preserve row order, signs and target orientation. Feed original four-term relations to rr5_receipt_check; preserve longer projected-row provenance separately. Keep the ten-query censored control separate from the eleven-query recovery arm. Freeze the strongest applicable native IC and negation-rho comparators on the same instance and public targets.

Dependencies: current native build repairs, exact source/binary identity, agreed row transport and the task's existing execution authorization.
Deliverable: a native original-row replay/target-certificate receipt, matched configuration manifest, and the owned identical-stream graph comparison at prefixes 256/512/613.
Pass/fail: exact group equalities and target-column agreement for every authenticated input row; independent useful rank and witness reconstruction agree with a simple reference; no unexamined tail is called checked. Report refusal, incomplete input, missing binary identity or failed admission explicitly.
Decision enabled: determine whether the existing controls and candidate pipeline are ready for a scientific comparison, without using this generic control as algebraic-extraction evidence.

### 2. Execute the genuine coupled M31 extraction gate
Continue the existing RR5/Semaev owners rather than creating a competing run. Freeze the actual summation-polynomial and factor-base system, complete projected-output semantics, public targets, holdouts and resource caps. M31 original coordinates can require infinity or extension-valued auxiliary witnesses, represented with M31 coefficients and an explicit model; a rational-only restriction must be a declared different contract. Check zero-root multiplicity in any Mersenne Frobenius shortcut. Small-field results are historical controls and do not replace this M31 gate.

Dependencies: step 1's input/correctness contracts, complete native charts and root recovery, source-pinned genuine F4 comparator, required checkpoint/admission and resource envelope.
Deliverable: complete M31 projected-set correctness controls and a bounded nonenumerative extraction cohort with exact signed relations, duplicates, independent useful rank, failures, operation counts, memory and every extraction phase.
Pass/fail: the declared output sets agree before performance interpretation; the extractor actually completes its registered public workload and produces useful relations. Timeout, omitted charts, dependent-only output or incomplete root recovery fails that gate.
Decision enabled: keep, revise or reject this extractor/arity/base configuration on its actual output and cost. A local first-fall or planted-rank diagnostic cannot decide it.

### 3. Run and independently check the complete cold M31 comparison
After the preceding gates pass, use the strongest applicable reproducible IC baseline and optimized negation-map rho on identical ordinary-M31 curves, targets and resources. Charge all setup and failed attempts to one public target; report genuine warm reuse separately. Preserve the project's operation-price band, calibrated costs, failure policy and independent replay requirements.

Dependencies: an admitted working extractor, source-pinned comparators, complete phase counters and valid isolated execution.
Deliverable: frozen per-target raw records, verified recovered scalars, independent relation/rank replay, recomputed complete-cost comparisons, peak memory and repeated-run uncertainty. [pcc:unverified]
Pass/fail: every required target is verified and complete candidate cost is lower than both references under the declared accounting and uncertainty criteria. Missing completions, costs, matched resources or independent replay leave the claim unresolved.
Decision enabled: promote a reproducible complete M31 advantage for broader validation, or retain a negative/censored result and revise the responsible phase.

Impact score: 3/10 — The plan now incorporates an exact retained M31 input, a concrete checker interface constraint and current ownership, with three explicit execution gates. It is a decision and preparation result; no new complete-solver measurement was produced.

