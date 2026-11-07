# Deterministic projected-resultant panel preparation

This successor connects the preserved accounting backends to a fixed native input generator, a separate Cartesian-product oracle, scalar A/A and planted-product controls, and a strict complete-panel analyzer. No comparison panel has been executed. The original4563-case cohort is unchanged.

## Inputs and oracle

`panel.rs` fixes seed20261107, xorshift64* transitions and rejection-based bounded draws. This deterministic PRNG is only an input-construction device, not a cryptographic random generator. Sample distinct field elements without replacement; split into roots/points. For each disjoint case, clone it and replace its first point with its first root. The resulting pairs have exactly zero/one common root respectively.

There are48 cases: p509 and1000003; n16/32/64/128; m=n/2,n,2n; disjoint/common-root. Both moduli are checked by exact trial division in native code. Native `--inputs` emits `inputs.tsv`, including expected outputs from a separately written Cartesian product of differences followed by square-and-multiply projection. The oracle imports no polynomial backend or accounting projection. The input file contains public synthetic roots/points, no unknown scalar labels. Input/oracle construction is outside the measurement interval and does not execute the candidate algorithms on the panel.

The input bytes and generator are reproducible; they are not yet a frozen canonical experiment protocol. `validation-01/receipt.json` binds their hashes. None of the p13 software fixtures appears in the panel.

## Execution and controls

The scalar, A/A and positive arms use one scalar code path. Positive adds one explicitly counted multiplication by1 after projection, with black_box preventing constant folding of its multiplier. This is a counted-source control, not a claim about emitted instruction count. Native software checks assert the same output, exactly one added Product event and equality of every other counter, for both a nonzero and common-root p13 fixture. A/A snapshots agree exactly. All three algorithms match the independent oracle on those fixtures. [pcc:unverified]

The prospective `--panel INPUTS PROTOCOL_SHA INPUT_SHA NEW_OUTPUT` command checks exact equality with regenerated inputs, rotates arm order by case index, runs the oracle outside accounting, checks every output, and writes/flushed rows to a new output file. Its supplied hashes are identifiers, not authentication: an external admitted launcher must verify the actual hashes and canonical freeze. A failed run retains partial rows; it cannot produce an accepted complete comparison. **Do not invoke --panel before protocol and budget/capacity admission.** This round invoked only --self-test, --inputs and --metrics.

`analyze.py` requires a complete240-row panel with exact row order, arm, input metadata, oracle values and metric inventory. It rejects missing/extra/duplicate rows, boolean/negative/overflowed integer metrics, nonzero live bytes/failures/error flags, wrong projection counts, A/A disagreement and any positive-control deviation other than +1 Product. The CLI checks external successful-exit, raw/input/binary/protocol/metric pins and writes analysis exclusively to a new path. It does not authenticate an unsigned producer or independently certify a protocol freeze. Each comparison is per-case with separate dimensions; common-root cases remain distinguished; no weighted total, time, speedup or statistical generalization.

## Validation and custody

`validate.py NEW_DIRECTORY` builds the native runner, executes software tests, regenerates input/metric bytes and requires exact equality, then runs synthetic Python analyzer tests. It never invokes --panel. Build90s, each subsequent command30s, sequential, one replay invocation. Receipt binds all directly compiled native modules and Python/data inputs, compiler/binary/log hashes and command results. Source generation is inherited unchanged from the previous accounting directory.

Five Python test methods include22 row/control mutations, duplicate/nonfinite JSON, wrong panel dimensions, and six external-receipt pin failures. The CLI test fabricates240 synthetic rows from the input descriptors; it does not evaluate cryptography or archive comparative results. It verifies success only for the complete consistent synthetic file and refuses overwriting output. Native software tests cover deterministic input membership, distinctness/intersections, exact trial division and direct-oracle handwritten cases. No native or wrapper test failure; two inherited atomic fetch_update deprecation warnings retained. [pcc:unverified]

Sources checked: preceding accounting implementation and comparison draft; scalar resultant derivation/reference direct products; Rust allocator contract already recorded in the preceding instrument. KB unavailable: no MCP tools; scripts/research-kb status reports missing executable. No novelty claim or independent review.

## Remaining before measurement

Implement canonical experiment.protocol/v1 and experiment.export/v1 integration, source/image publication binding and an admitted launcher that produces external receipts on both success and failure. Complete preflight under the authorized backend; then freeze before panel data. Proposed ceiling remains one attempt120s,1CPU/1024MiB,concurrency1,USD1 total; no funds reserved, remaining funds/capacity unverified, no submission. Preserve failures and the shared cloud limits. Existing native --panel is a computation entry point, not an admission service.

After a valid comparison, evaluate backend/cutoff choices from the separate cost dimensions; then advance implicit orbit-product evaluation and compact nonlinear scalar decoding.

Evidence: PCC-20261107-7a624d, unsigned producer attestations; validation-01 passed.
