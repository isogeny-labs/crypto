# CT: Selected-call cost frontier for fixed Jacobian chains

**Standing: [pcc:unverified].** Counter-only prospective audit; no ECDLP speedup, timing, EKS job, native implementation or signed PCC claim.

## Frozen inputs

The script reads, but never edits, the CT82 original fixture
`research/chained_jacobian_transfers_20261110/evidence/public-n4-complete-chain-checkpoint-01/01-literal-setup-and-selected-vectors.json` on [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877). The source is the fixed complete-chain checkpoint `161c5bab610c2b72e8fb45287e5391ea062fe37c`; its original 695-record native producer execution is an **unsigned** source claim. The prospective transformation is taken from [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523), `experiments/chained-transfer-public-target/NORM_POINT_REUSE_PROTOCOL_01.md`, published before optimized code: `N(Phi(Z))=[2]Z`.

Reviewed prior work: cryptanalysis #1500's measured-field-counter negative direct/isogeny toy comparisons; elliptic dual identities and isogeny evaluation methods in Bostan–Morain–Salvy–Schost (2008), https://arxiv.org/abs/cs/0609020. No novelty attributed to duals or elliptic isogenies. Shared research-KB MCP and checkout-local fallback unavailable in this connector-only workspace; the source review is therefore incomplete.

## Conditional symbolic crossover

The actual selected fixture contains **48 rows** (24 per direction and 12 per cofactor sector). Each route has exactly 20 original counter categories. With `C13` and `C169` the *unmeasured* cost of one elliptic evaluation of the named degree, and `D` the *unmeasured* cost of one norm-point doubling, the **prospective** protocol requires:

| Route | Original map counts | Proposed map counts | New norm-point doubling |
| --- | --- | --- | --- |
| Direct | 2 × C169 | 1 × C169 | 1 × D |
| Literal sequential | 4 × C13 | 2 × C13 | 2 × D |
| Retained-lift | 4 × C13 | 2 × C13 | 1 × D |

On **every** selected row, direct and retained-lift have identical values for all *other recorded* primitives. Therefore, under a common per-call primitive pricing model, the **prospective** difference is

\[
C_{\text{direct}}-C_{\text{retained}}=C169-2C13.
\]

The crossover is exactly `C169 = 2*C13`, conditional on equal setup/hidden costs. The literal route is componentwise no cheaper than retained-lift on all recorded counters (and adds corrections/Phi calls, half checks, and one doubling). This prunes the **literal route in this selected-call counter model**, not from a future whole-program benchmark.

Reuse can lower each selected-call bill only if `C169 > D` (direct), `2*C13 > D` (retained), or `C13 > D` (literal). Across 48 rows the expected evaluation-call savings are 48, 96 and 96 respectively. These are counts, **not** CPU/GPU time, a generic complexity result, or a source DLP improvement. For the 48 selected rows, the literal route's surplus half-checks vary between four and eight.

## Reproduce and falsify

Run from the repository root:

```sh
python3 -S research/chained_jacobian_transfers_20261110/cost_frontier_20261114/selected_call_frontier.py --self-test
python3 -O -S research/chained_jacobian_transfers_20261110/cost_frontier_20261114/selected_call_frontier.py --self-test
python3 -S research/chained_jacobian_transfers_20261110/cost_frontier_20261114/selected_call_frontier.py
python3 -O -S research/chained_jacobian_transfers_20261110/cost_frontier_20261114/selected_call_frontier.py
```

The first two use a synthetic 48-vector fixture and must reject five deliberate mutations: omitted record, altered original map count, reversed literal/retained dominance, unexpected primitive, and invalid cofactor sector. The next two read the **original** committed fixture and fail closed on missing or extra fields, direction/sector imbalance, duplicate selected inputs, changed original evaluation counts and counter differences.

A separate JavaScript inspection of the original GitHub fixture, not this Python implementation, checked all 48 original rows and gave the same symbolic crossover, 24/24 directions, 12×4 sectors, map-call savings, and nonnegative literal-retained deltas. This is an *unsigned independent arithmetic path*, not signed PCC or a native 695-record replay.

## Mandatory next gate

After genuine-22 custody release, implement native reuse in a sibling literal source as the already-frozen protocol requires; preserve all 695 records, 672 exceptional descriptors, four half branches, full-order controls and actual omission rejections. **Then** price the complete elliptic evaluation, norm-point doubling, Jacobian Phi/halves/corrections, field-primitive setup/reuse and clock time on matched hosts. Compare both surviving routes against the original source's best complete rho and index-calculus references. No assumption about transferring an unrelated-prime twist is admissible.

No EKS/Spot paid launch or protected merge performed. [pcc:unverified]
