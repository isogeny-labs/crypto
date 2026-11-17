# Retained root trace integration contract

Frozen before source edits, 2026-10-29. Integration base c1fc70d36264beedf7c302ee4b70aa73e99f93eb; Conductor T803/T805. This is software integration and correctness validation, not a new research campaign. The original corpus scope and USD200 total cap remain unchanged.

## Reconciliation and sources

The current main-descended dataset root adapter omits GCD/factorization states. These already exist in the policy/root and binary-root branches. PR2777 merged into the temporary review branch at aeb62c550931398c7822a4daa611aec87b39e88c; PR2782 merged there at f5bf2160d54a7743f0f0b8201dead31c6b7ddf2d. Neither establishes main integration. Reuse the retained engines, artifacts and independent verifiers rather than duplicate them.

Donor c14d4552e513f6e28bda0e0e045f5e64beb492db retains runtime d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83 and historical validation. Integration must preserve current startup, budget and prime-chain changes. Old tests do not certify the combined source.

Reviewed the repository's PRIME-F4-WIDE-ARITHMETIC-INVENTORY, PRIME-F4-CHAIN-CONTRACT and native polynomial helpers. FLINT's module documentation at https://fredrikj.net/fdoc/html/nmod_poly_factor/index.html describes prime-modulus, monic-factor and squarefree/equal-degree assumptions. The flintlib.org request returned403; the author's mirror was accessible. No novelty, speed or FLINT-equivalence claim.

## Integration invariants

Reuse the existing bounded prime root engine and independent verifier first, preserving typed Complete/AllFieldElements/censored/sink-failure states, odd-prime range3..2^61-1, degree/work/event/byte bounds and deterministic split choices. The binary module/adapter remains a subsequent retained-code integration, not completed by the prime increment. Higher-m roots and the full campaign remain requirements.

Keep the existing native add/sub/mul/square counter units; they differ from legacy GLV root multiplication tallies and are not retired instructions. Preserve existing schema and exact Rust u64 JSON parsing. Binary64 conversion is not allowed; decimal-string ML projection needs separate versioning. Existing per-call JSON is not JSONL.

Central replay must bind root certificates and outcome fields to the reconstructed public polynomial and exact domain, reject hash-consistent semantic tampering and missing required certificates, and retain startup/prime-chain replay. The existing verifier does not prove a complete operation trajectory; execution/cost-completeness flags stay false. Zero-polynomial roots are not an empty set.

The current export path calls verify_cell(true) and lists all artifacts in restricted source_artifacts; preserve its public feature bytes. export.rs is T804-owned and must not be edited. Adding the root role to solution_certificates is a separately pending label-reference integration, not implicit permission to change that file. State this limit explicitly.

No wholesale replacement of current dataset files. Record exact donor functions/hashes and adapted hunks. Native module source can be reused byte-for-byte; module registration, root dispatch, capability reporting, central role/domain/outcome checks and relevant tests must accompany it.

## Acceptance and delivery

- Native prime root tests and independent root certificate checks: zero/constant/repeated/non-split roots, large prime coefficients, caps and sticky sink failure.
- Dataset central replay/tamper tests, missing/duplicate certificate checks, actual bounded CLI fixtures for both twists. Root-set completeness, whole-search completion, PDP witnesses and scalar recovery remain distinct.
- Required full release library, strict applicable lint and CLI integration gates on combined runtime; preserve current startup/prime-chain regressions. Retain failures and source/binary bindings.
- Verify export retains restricted artifacts and unchanged public features without editing T804's source. Do not claim solution_certificates wiring until it is integrated.
- Archive source/logs/fixtures to S3 with fresh GET/hash; publish reviewable Git delivery. Fixtures do not count as scientific cloud cells.

The initial uncommitted proposal for a new GLV trace API was withdrawn on discovering retained code. No runtime edits or experiments occurred under it. Parent retains review/delivery control. No IAM change, grant, generator restart, new EKS Job, budget or deadline extension.
