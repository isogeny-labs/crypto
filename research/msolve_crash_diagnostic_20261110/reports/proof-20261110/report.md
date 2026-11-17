---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Source-guided msolve diagnostic

Eight fixed local calls retain every outcome for the Boolean ideal over
F65537, crossing ordinary/saturation, exact linear algebra1/2 and
original/address+undefined builds. Original l1ordinary passes its exact
known-answer check; original l2ordinary crashes139 and both saturation
calls time out. All four instrumented calls stop earlier at a signed-shift
diagnostic. No paid compute. [pcc:unverified]

![Frozen cells, actual outcomes and scoped interpretation. [pcc:unverified]](diagnostic.pdf){width=96%}

\newpage

# Exact outcomes, formal boundary and trust

| Build / linear algebra | Ordinary | Saturation |
|:--|:--|:--|
| Original /2 | exit139, empty output | timeout124 at5s |
| Original /1 | exit0, expected basis passes | timeout124 at5s |
| Address+undefined /2 | exit1, signed-shift diagnostic | exit1, signed-shift diagnostic |
| Address+undefined /1 | exit1, signed-shift diagnostic | exit1, signed-shift diagnostic |

Every instrumented stderr identifies neogb/hash.c409, generate_short_divmask:
signed1 shifted by31. Source reading shows int32_t res, literal1 and
uint32_t return type sdm_t. First-error instrumentation prevents reaching
the original crash/timeout sites. This establishes neither their cause nor
a complete source fix. All source files remain byte-equal to the official
pinned0.6.5tarball. [pcc:unverified]

The original l1ordinary output passes normalized known-basis equality,
input reductions, Buchberger S-pairs, staircase3 and three zeros among
the four Boolean candidates. This is not exhaustive enumeration of
F65537². A successful fixed output does not prove general msolve
conformance to F4Run. The frozen Lean ideal-containment and saturation
counterexample retain their original exact polynomial domains.
[pcc:unverified]

Flow is8planned,8attempted,6completed,2not_completed(timeouts),
0excluded. Earlier v1planning/dispatch failures and v2ordinary139/
saturation60stimeout remain unchanged. New protocol7bf6fb55 freezes
all8cells, exact inputs, analyzer, expected bases and5s per-call/40s
batch allowance before data. Source commitf79068d3 precedes affected
outputs. No outcome-dependent retry or changed frozen parser.
[pcc:unverified]

The export passes all schema, protocol and file-hash checks. Failed
metadata drafts/checks are retained separately. Packet
PCC-20261110-743f20 remains not_covered: unsigned artifact attestation
is not independent execution/recomputation. There is no timing,
statistical, generic-system regularity, other-machine or C-refinement
claim. [pcc:unverified]

Primary source scope: msolve CLI exact LA1/2 and saturation options,
field-width dispatch, F4 linear-algebra paths and reported hash routine;
GCC official address/undefined and first-error instrumentation options.
KB unavailable, wrapper127; bounded primary/source fallback. Prior
formal proof maps remain in the theorem-census and original F4 studies.
Next diagnostic: separately preregister address-only instrumentation to
observe beyond the earlier undefined-shift check; do not relabel these
first-error runs as a memory-error diagnosis. [pcc:unverified]

Exact protocol/source/input/build pins, raw files and all failed outcomes:
protocol.json, provenance.json, frozen-files.sha256 and export.json.
This report and editable diagram summarize the actual eight-cell record.
[pcc:unverified]

Raw argv/stdout/stderr/output/status/timestamps are archived without byte
changes in evidence/runs.tar.gz, with hash inventory and roundtrip check.
Run `python3 materialize-raw.py` before checking export files in a fresh
checkout; existing unequal files are rejected. [pcc:unverified]
