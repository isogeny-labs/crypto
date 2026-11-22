# CT75 scoped session receipt — 2026-11-13

## What was done (commands, files, features)

- Read current crypto root #3877 and stacked #4015 metadata, latest dated root log and CT74 checkpoint, AGENTS.md, workflow gates, and sibling cryptanalysis/autoresearcher PR summaries. Source-first reviewed CT69–74 and Snowden's primary lecture on finite-étale torsion specialization. Shared research-KB MCP, checkout wrapper, and Conductor executable unavailable in this runtime.
- Implemented source-local odd-prime torsion lifts for 11a1 at (p,ell)=(29,3),(29,5),(37,7),(59,11),(139,13). Python enumerates Hensel x digits; independently written JavaScript performs a two-evaluation finite-difference Newton x step. Each procedure receives only encoded public point coordinates and not log labels.
- Ran normal/optimized Python and normal/no-opt Node controls, all byte-identical: SHA256 `9d16bc738049c00ab76ca2f9e1cf0b0d4fc5b7b083d72dcd577c2cbffd327a33`. 22 precision rows, 126 public-target scalar-relation verifications, 34 initial encoded subgroup points, no failures. Python 6052 x-digit candidates; JS 184 derivative defect probes (not full cost or wall time).
- Fixed two **local pre-publication defects**: omitted `p` argument on the initial torsion assertion, and inconsistent serialized key order across the two independent auditors. Both corrected before published test hashes; verified every output in four modes. Recorded a genuine non-failure: naive coordinate lifting of rational order-five (5,5) remains torsion.

## Decisions made (and why)

- Keep new code under CT75 and the existing regular stacked PR #4015. Affine-only torsion testing avoids an infinity chart because all internal multiples are nonzero modulo ell; do not mislabel it complete finite-ring group arithmetic.
- Treat this as source-local finite-ring lifting, **not** prime-to-binary transfer, global algebraic point recognition, target solve, or advantage. No paid EKS/Spot or other compute launched. CI and independent PCC remain separate.

## Open threads (unfinished, pick up next time)

- Inspect exact-head GitHub scoped CI and independent PCC/review gates, preserve parent #4012 stack. Next construction: test global recognition of arbitrary encoded non-rational local ell-torsion (now explicit for ell7/11/13) and coherent reduction at a second prime, with charged field-degree, height, identification and target-solve costs.

## Errors (what went wrong, how it was handled)

- Research-KB and Conductor not installed/loaded; repository GitHub connector and primary web sources used as explicit fallback. No unresolved arithmetic mismatch; signing not verified. [pcc:unverified]
