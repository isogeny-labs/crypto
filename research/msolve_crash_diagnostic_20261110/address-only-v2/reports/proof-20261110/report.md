---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Address-only msolve fault-site diagnostic

Four separately frozen local calls inspect the unchanged0.6.5source
beyond the earlier combined-sanitizer first error. ExactLA2ordinary
aborts134 with an ASAN READ SEGV at la_ff_32.c543;
LA1ordinary passes its exact known-answer basis, and both saturation
calls hit5swatchdogs. [pcc:unverified]

![Separately frozen calls and scoped fault-site observation. [pcc:unverified]](diagnostic.pdf){width=96%}

\newpage

# Actual outcomes and source boundary

| Exact linear algebra | Ordinary | Saturation |
|:--|:--|:--|
| 1 sparse/dense | exit0; exact basis check passes | timeout124 at5s |
| 2 sparse | exit134; ASAN READ SEGV | timeout124 at5s |

The ASAN backtrace identifies the 17-bit sparse reducer at
src/neogb/la_ff_32.c:543. The path includes the exact sparse echelon
routine, the sparse linear-algebra dispatch and core_f4. This is the
observed fault site of the instrumented execution, not a complete
root-cause or well-defined C-semantics proof. [pcc:unverified]

The earlier signed1shift31 is not instrumented by this build, and the
source remains unrepaired. Omitting that check does not make the C
execution defined. All original source files remain byte-equal to the
pinned official0.6.5tarball. The ordinaryLA1known-answer check validates
normalized basis equality, input reductions, S-pairs, staircase3 and
three Boolean-candidate zeros. No fullF65537²census or generic solver
refinement is established. [pcc:unverified]

Protocol version 2 (hash dc99c976) freezes all four cells, with a
five-second limit per call and twenty-second batch allowance. Source
commit a549657b precedes affected data. Flow: 4 planned, 4 attempted,
2 completed, 2 timeouts and 0 excluded. The previous
8combined-sanitizer cells and original2failures remain unchanged.
No native call or parser was rerun under the earlier protocol.
[pcc:unverified]

The export passes schema, protocol and all-file hash validation. Packet
PCC-20261110-abac7a is not_covered: unsigned artifact attestation lacks
independent signed recomputation/replay. No paid compute, timing,
statistical, other-machine or Crefinement claim. The existing Lean
ideal-containment/saturation-domain proofs retain their exact scope.
[pcc:unverified]

All28raw files are archived losslessly with per-file hashes and byte-equal
roundtrip checks. Run `python3 ../materialize-raw.py .` in this study
before checking the export from a fresh checkout; unequal existing files
are rejected. Exact sources and outputs: protocol.json, provenance.json,
frozen-files.sha256, export.json and evidence/runs-inventory.json.
[pcc:unverified]

Source review remains pinned to upstream CLI, coefficient-width/matrix
dispatch and exact reducer paths, plus official GCC sanitizer options.
KB absent/wrapper127; bounded primary/source fallback. Next source audit:
check the basis-index rewrite against the coefficient-table lookup and
current upstream correction before any separately preregistered patch
experiment. [pcc:unverified]
