# Validation and missing checks — 6 November 2026

This is a source/proof-review packet, not an executed scientific control or a
kernel proof. All observations, transition arguments and local-check outcomes
are producer assertions under **PCC-20261106-d6132b, standing unverified**.
The general audit remains open. [pcc:unverified]

## Claim and source scope

Official CLI `tools/pcc/pcc.py claim new --kind statement` created the claim
before result citations, at source/base
`75309f57ba69d7683006108e151a21233a04bb48`. The CLI's **canonical JSON claim
digest** is `55be725d3e61e64ba4f281185b004762c4eabc3e42f29f55a1e362c774af5578`;
the **raw claim file SHA256** is
`d44439f3f90c309bae7bf1914373f47c74d495ed6860eef3ae59230344dfd04c`.
These use different byte conventions. `standing --json` exits2, unverified,
no verdicts; `verify` exits2, `not_covered`: a statement has no proof kind and
can never be verified. No signing key was read or generated. [pcc:unverified]

The ledger has182 entries, exactly the181-entry original base plus one official
claim event. `ledger verify --base 75309f57ba69d7683006108e151a21233a04bb48`
checks that chain/prefix. No prior event or claim was changed. This is NOT a
prefix claim against later main. [pcc:unverified]

Producer metadata checks re-read Git blobs for all22 source records, compare
SHA256/byte count/Git object identity, compare the3 extracted runtime regions
against f96a6d475896e06065eda55ccecf5f0132692423, and check all17 source-site
line anchors in refinement-map.json. These checks pass; they check references,
not mathematical semantics. Whole-file comparison initially exited1 because
main additionally registers guards and contains mutation tests; the scoped
runtime-region checks replace no whole-file claim. [pcc:unverified]

Primary source: read Appendix D.1/D.4.5-D.4.7 of the cached PDF, checked against
current path metadata (last change42907c9e999cc00d6abe7a50bfc1023250edd1b9,
blob0503566e95bc587a4076573a45132d97531b8ba6). D.1 allows exponential entry
work. Atlas match at a97d7c1304033a7863609096d425487cd2e2b31b used the existing
hashed-factor-base lead; exact output is KB-AND-ATLAS.txt. No full atlas-delta
or novelty review. [pcc:unverified]

Sources checked (KB): no research-kb MCP tools loaded; repository wrapper
`scripts/research-kb status` exits127, no executable. Intended queries in
KB-AND-ATLAS.txt were NOT executed: PacketTensor entry/coefficient extraction,
DFS restoration/contributions, D.4.6 signed affine row products, and zero
divisors. No successful search, KB source ID, revision or refresh is claimed.
Fallback: pinned primary source, prior reports and current atlas lead. KB
retrieval remains an explicit gap.

## Report and local policy checks

Graphviz plus Typst0.15.0 render the editable dependency.dot/README using the
included renderer. Final report.pdf: **4 A4 pages,80472 bytes**, SHA256
`fdd7effdd69c9b60316ea99a118066f9de3de65a60aa9610f16c6ca8e55cfe97`.
Every final page was rasterized with pdftoppm and visually inspected. Initial
wide diagrams were rejected as too small; the final compact diagram is a
proof-obligation dependency graph, not complete control flow. No timings or
scientific plots were generated. [pcc:unverified]

New-report prose lint passes after one initially untagged paragraph was given
its explicit unverified standing. Existing whole-log advisory prose is not
rewritten. Complete staged-tree artifacts policy and whitespace checks are
passed against the complete staged tree before publication using the unchanged
tracked policy and attributes.
These producer checks do not change PCC standing. [pcc:unverified]

`python3 tools/benchrec.py gate` passes:48 existing records, progress current.
Initial run failed with250 missing sparse references; materializing278 exact
tracked paths/sidecars made the existing inputs available. No record, progress
file, artifact, gate or scientific result was edited or uploaded. The52-byte
final gate log SHA256 is
`11e4cd82dfebc1f7cb1714f2a758240f96a38c037339d93c7e399dc5d0dfd825`.
Its temporary local path is /tmp/hashed-ecdl-dfs-bench-final.txt; storage status
NOT_UPLOADED, no artifact-store identity. [pcc:unverified]

Conductor exits127, no reservation; documented docs/CONDUCTOR.md warn/allow
fallback. Lean/lake unavailable on PATH, so no formal build, faithful imperative
refinement, transitive dependency/kernel/axiom audit or identity certificate.
Existing rank-one controls and mutation tests were read, not rerun. No broad
code panel, experiment, benchmark, challenge solve or cloud resource was run.
A metadata-check draft incorrectly compared the canonical claim digest with
raw JSON file bytes; corrected the checker to the official canonical byte
convention and recorded both identities above. Claim/ledger bytes unchanged.
[pcc:unverified]

## Coordination and publication boundary

PR3478 comment6689804776 coordinates this additive isolated-main follow-up;
its pending resolver, branch, conflicted journal and ledger remain untouched.
Its Nov5 tuple argument is unmerged/unverified, not an assumed checked lemma.
No older lifecycle was restarted. [pcc:unverified]

At the prepublication refresh main b354dfefa868fcc7e663dbc5f07bc4fbf370d585
has186 ledger entries; this branch has182, sharing181. Both append different
events after the same immutable base. No main integration or ledger rewrite
was attempted. This incompatibility must be escalated during PR integration;
reasonable-wait fallback cannot authorize rewriting/re-chaining either history.
Current AGENTS remains identical to the source pin. [pcc:unverified]

Next technical dependency: a faithful coefficient/choice-tree model and checked
imperative-state refinement, with explicit ring, shape, usize and Ok/Err
preconditions. Then join preparation, all attempted choices, outgoing powers,
operand widths and ancestor retention. The local capacity argument and source
inspection do not discharge that aggregate induction. [pcc:unverified]
