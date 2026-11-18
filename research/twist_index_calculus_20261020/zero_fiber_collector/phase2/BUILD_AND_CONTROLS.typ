#set page(paper: "a4", margin: 16mm, numbering: "1")
#set text(size: 8.7pt)
#set par(justify: true)
= Phase 2: native build and recovery controls
#text("Checkpoint 2026-10-24T23:55:52Z. Native gate v1 is terminal; the full 85-configuration panel has not started. This report preserves the first gate's scope, including its subsequent verifier correction.")

#image("figures/pipeline.svg", width: 100%)

#table(columns: (1.1fr, 0.7fr, 2fr), inset: 4pt,
  [Round], [Result], [Raw directory under ../runs/],
  [Compile check v1], [Exit 101], [phase2-check-v1: json! expression syntax],
  [Compile check v2], [Exit 101], [phase2-check-v2: integer conversion type],
  [Compile check v3], [Exit 0], [phase2-check-v3],
  [Bootstrap build], [Exit 0], [phase2-bootstrap-build-v1],
  [Selection and fixtures], [Both exit 0], [phase2-preregistration-v1],
  [Native gate v1], [89 pass; 0 fail], [phase2-native-gate-v1: three phases exit 0, source seals stable])

== Actual positive controls
All four zero/plain phi/doubling pipelines execute on p37 seed0 and recover scalar 271 with independent target and factor-point verification. Four first-run physical-set/RREF replays match immutable complete prior references. Two additional zero-phi executions have exact deterministic A/A equality; both raw clock and memory observations remain.

The zero-phi audit checks 81 zero systems and obtains factor rank 2/2. One target query with 72 finite systems completes rank 3/3. These are positive-control counts, not an advantage over the matched plain pipeline or a result for all 21 cases.

== Preserved failure and validator correction
The first source-copy verification overlaps a syntax repair and fails. Its exact reconstructed snapshot matches the pre-compile manifest; both copies and checks remain. Compile v2's snapshot also matches. No historical collector, full census or 189-case panel is rerun.

Post-gate review finds that array-to-set deserialization can hide duplicate physical tuples. The auditor now rejects that corruption and includes a duplicate-target-tuple mutation control. A fresh v2 native gate is queued for that verifier revision. Its terminal result is required before the full panel; the first gate's evidence is unchanged.

== Boundaries and remaining work
The frozen family baseline gives 85 distinct configurations and 255 first/A/A executions, with 85 independent first-run audits and 170 parity checks. Twenty best-parent roles reuse an identical registered plain control; p97 seed1 adds union-l2. Full comparison is pending. Shared-host elapsed values are unadmitted, and scoped field/group/F4 row counts are not complete GAE prices. Strongest IC/rho references, formal bridges and scaling remain open.

#text(size: 7.5pt, "Gate source f93e0cff07; runner SHA-256 daeb82aec36b876927d98bd48051fc4bf9b28d64c3e5a18b6c5f8e02c04dad37. Per-file compiled-source seals are authoritative. See BUILD_AND_CONTROLS.md and ../EXECUTION.md for provenance and all retained attempts.")

#pagebreak()
= Follow-up gate and complete p37 control
#text("2026-10-25. The tightened verifier passes a fresh 89-test gate with no failures. All three phases exit zero and source seals match. The standalone Rust workload/reducer builds successfully and seals all 85 configurations before the full comparison starts in commit af66996a19. Earlier positive controls are retained separately from registered panel executions.")

== Work through verified recovery
The native metadata tool reads the retained p37 seed0 positive-control records without repeating a solve. Every row recovers 271 and verifies all target and factor points.

#table(columns: (1.4fr, 1fr, 0.8fr, 0.8fr, 0.8fr, 0.8fr, 0.8fr), inset: 4pt,
  [Configuration], [Zero rank], [Zero F4], [Target F4], [Total F4], [Queries], [Final rank],
  [Plain phi], [0/2], [0], [72], [72], [1], [3/3],
  [Plain doubling], [0/2], [0], [72], [72], [1], [3/3],
  [Zero + phi], [2/2], [81], [72], [153], [1], [3/3],
  [Zero + doubling], [1/2], [216], [72], [288], [1], [3/3])

Phi needs fewer zero systems than doubling here. Neither precomputation reduces its matched online query count or target F4 systems. Each adds systems relative to its plain control. These exact finite system counts are not a new admitted elapsed-time or complete GAE price. One selected positive control does not establish a result for the full panel.

== Launcher controls and ownership
The final driver's nested outer-busy control exits 64 without creating output. An actual execution with a pre-existing first-phase command exits 65, leaves its marker byte-identical and starts no candidate. Both executed scripts, hashes, commands and exits remain under phase2-driver-controls-v2.

The managed twist-zero-pipeline-panel-v1.service owns the full comparison. It executes 85 distinct configurations with first/A/A, 85 independent first-run audits and 170 parity checks. Each phase obtains the common reservation; candidate limits remain one worker, 8 GiB, 1,200 seconds outer and 1,180 seconds cooperative. Per-system input/result journals and every partial or failed case remain. The panel's terminal result and versioned archive are separate from this build/control report.

== Exact bindings
#text(size: 7.5pt, "Tightened production runner: abd9a046676f8e1da1212899438bd155d47b8f88fc3cfc814209dbf95f0efc1f")

#text(size: 7.5pt, "Native metadata runner: 6e1a21eb92cf9ce092e5a9449df6c2a211817aa0c500a1291630e94971077d9c")

#text(size: 7.5pt, "Workload JSON: 47604fa481c5c26ef17e9e9a41494461556c1b9356e20e9652a499926a499b77")

== Preserved layout checks
The first PDF inspection finds narrow diagram labels extending past boxes. Its PDF, diagram and sources remain under phase2-controls-layout-v1. The corrected v2 page is inspected successfully. This follow-up retains that history, adds the native control table, and renders its PNG at 150 dpi from the same vector source. No native result changes.

Complete calibrated IC/rho price, globally strongest IC comparison, formal bridges and broader scaling remain unresolved. No VALIDATION.json or leaderboard promotion is inferred.
