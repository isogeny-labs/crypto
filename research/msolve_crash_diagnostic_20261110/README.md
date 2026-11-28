# Bounded msolve crash diagnostic

Eight frozen local calls cross ordinary/saturation, exact sparse2 versus
sparse/dense1, and the original versus address+undefined sanitizer build.
The unchanged source is msolve0.6.5, commit5e72b9d9. All source files
match the pinned official tarball byte for byte. No paid compute.
[pcc:unverified]

| Build / linear algebra | Ordinary | Saturation by x |
|:--|:--|:--|
| Original /2 | exit139, empty output | timeout124 at5s, empty output |
| Original /1 | exit0; exact expected basis passes | timeout124 at5s, empty output |
| Address+undefined /2 | exit1, signed-shift diagnostic | exit1, signed-shift diagnostic |
| Address+undefined /1 | exit1, signed-shift diagnostic | exit1, signed-shift diagnostic |

Every instrumented call stops at neogb/hash.c409: signed1 shifted by31.
The source initializes int32_t res and uses an int literal1; its returned
sdm_t is uint32_t. This is an earlier diagnostic, not evidence that the
shift causes the original l2crash or either timeout. No upstream source
was modified. Different failure points under instrumentation are retained.
[pcc:unverified]

The original l1ordinary result is checked by the frozen exact C++ analyzer:
expected normalized basis, all input reductions and S-pairs zero,
staircase3, Boolean-candidate zero count3. This is not a fullF65537²
census or a C-implementation refinement theorem. The existing Lean
ordinary-F4Run ideal containment and saturation-domain disproof remain
separate, unchanged. [pcc:unverified]

Flow: planned8, attempted8, completed6, not_completed2(timeout),
excluded0. All crashes/diagnostics/timeouts remain in the export.
The old v1registration issue and v2ordinary139/saturation60stimeout
are preserved in their original studies. This is a new predata diagnostic,
not an outcome-dependent retry under those protocols. [pcc:unverified]

Source f79068d3 commits the protocol, inputs, analyzer and runner before
all8calls. Protocol validates; all-file export check passes. Initial
metadata/schema mistakes and their exact failed checks are retained as
numbered draft records. Frozen solver/analyzer/expected rows unchanged.
The packet PCC-20261110-743f20 is not_covered: artifact attestation is
unsigned and lacks an independent recomputation/replay. No independent
execution or general correctness conclusion. [pcc:unverified]

Read scope: original F4Run/source audit in ../theorem_census_20261109/F4_MODEL.md;
pinned upstream CLI, field-width dispatch, exact linear algebra and
sanitizer-reported hash path; official GCC Instrumentation Options
address/undefined/first-error sections. KB absent/wrapper127; bounded
primary/source fallback. No novelty, speed, statistics, memory-safety
proof, general generic-ideal measurement or other-machine validation.
[pcc:unverified]

See protocol.json, frozen-files.sha256, provenance.json, export.json,
evidence/analysis.jsonl and raw per-call files. The
[report](reports/proof-20261110/report.pdf) includes an editable DOT diagram.
[pcc:unverified]

Raw argv/stdout/stderr/output/status/timestamps are archived without byte
changes in evidence/runs.tar.gz, with hash inventory and roundtrip check.
Run `python3 materialize-raw.py` before checking export files in a fresh
checkout; existing unequal files are rejected. [pcc:unverified]
