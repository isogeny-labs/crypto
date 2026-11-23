# Arena candidate readiness

Implementation and local native validation complete; no new timed N83 arm has
run at this freeze. The PROTOCOL.md requirement table remains the acceptance
contract. Actual experimental validation and higher-cap continuation are pending.

The full release library passes4482tests,0fail,119ignored. SAT-focused suite
passes32tests, including unchanged pinned digests across restart/reduction,
incremental model enumeration, lazy clauses and XOR word boundaries. The new
test physically shrinks a large detached buffer, relocates a live root reason,
clears a stale unassigned reason, preserves every clause ordinal, compares all
64CNF assignments before/after incremental prefix promotion, and round-trips
CNF and native-XOR exports. Exporter19, independent model-checker2, CLI13,
unknown-scalar producer3, native guard40unit+7card tests pass. Native guard
negative controls reject incomplete/invalid reclamation counters, missing or
changed1M prefix counters and duplicate post-prefix snapshots.

The initial new test compile used a nonexistent`from_dimacs`name. Its original
failure log is retained as validation/arena-tests-01.txt; the full failed SAT
source snapshot is retained for the raw archive. The corrected test uses
`parse_dimacs`and`parse_dimacs_xor`; arena-tests-02.txt and the full release
suite pass. No experimental worker ran during this development failure.

Worker SHA256`80f296aee1fdb166623522033f1020fb9ca17e8f6259a06f484d31bb0bed35bd`.
Guard-v1 SHA256`563d2168849f7096c217c6bcbba18ed915e2c3c44bc50f422de226631b0dbad0`.
Source and binary inventories, full checks and compiler versions are in
validation/. The original pinned image was restored by repository digest
and its imageID verified. The no-argument native loader smoke reaches the
expected usage refusal(exit1); it performs no query or SAT search.

## CI repair, separate publication

Published CI snapshot`7a9371c19264d6641b124e85816e24efa856000e`is tree-identical
to local commit`5b06c4d366`. Its parent is the prior PR head; no force update or
main changes were discarded. Fresh exact-head workflow outcomes remain pending.

| Original failing workflow | Diagnosed cause | Fix |
| --- | --- | --- |
| Release, backlog, IC research framework | Two CLI matches omit new RelationQueryStarted variant | Explicit progress and experiment handling |
| Koblitz unknown-scalar panel | JSON serializer omits same variant | Serialize replay coefficients and coordinates; regression test |
| ic-leaderboard | Indexed-XOR SUMMARY.json absent from path filters | Add exact input to pull_request and push paths |
| curve-names | Diff mode disregards documented frozen evidence directory exemption | Apply existing directory policy to diff mode; live-source rejection test |
| Conductor-gap native controls | Hash-verified Lean archive extraction cannot execute missing zstd | Install zstd before extraction; hash pin preserved |

Local naming lint against the PR base reports0problems, and4naming regression
tests pass. Ten complete failed job logs and original workflow/job metadata
are retained under task scratch/ci-original for the raw archive, including
the three platform-specific Release failures. Their checkout/source hashes
remain original. CI success is not inferred from these local checks.

The original raw ownership archive remains accessible in
setec-astronomy-artifacts(HEAD36,130,808bytes). New raw evidence is to be sealed
and uploaded separately; previous objects and reports remain immutable.
