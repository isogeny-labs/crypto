# Branch helper repair: source-only candidate

The failed v2 helper reported four elaboration errors. This candidate preserves theorem statements and changes only the proof: it exposes the local evaluation map e, unfolds the local multiplicity m, supplies an explicit ENat natural-cast finiteness proof, and reduces local bindings in the support-sum hypothesis before rewriting.

The five proposed calls are corrected helper, target, controls, separate named axiom readback and expected failure. Thirteen previously successful compiled imports are pinned by artifact hashes independently of source hashes. All retained successful stdout was checked: 40 printed names across12 modules use only the expected three standard axioms; Nonbranch printed warnings only. The final target audit covers transitively used declarations. No new blanket prerequisite audit is required.

The proposed limit is 2400 seconds: 900 precheck, 600 compiler, 900 postcheck; one CPU, 8 GiB, no swap, 128 tasks, no retries. Previous 397.613-second compiler phase is scheduling evidence, not a runtime guarantee. Exact runner adaptation, latest ledger, publication, independent review and serial release remain pending. Failed prior output and charges remain in branch-v2-results-20261109.

No new compiler was invoked. The repair is uncompiled and its proof acceptance remains unresolved.
