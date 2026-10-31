# Outer dynamic claim boundary

Review source0fd2d838db414f45c227bfb6aa7f4f796022aba2 and existing m2/m3 outer outcome verification. For the dynamic profile only, solver completion follows the frozen continue-after-first-witness/exhaustion contract; do not inherit m3 early-positive completion semantics.

The future parent reconciler must use separate independent replay fields: first_witness_verified plus witness_capture_committed for a durable positive claim; independent_group_domain_complete for mathematical domain exhaustion; solver_completed_verified for a consistent successful producer terminal; negative_verified for the checked terminal negative claim. Never infer solver completion from a witness or domain exhaustion alone. A later outer persistence failure must suppress completion while retaining already validated mathematical facts and the original publication acknowledgement.

Reject unproved scalar recovery, native derivation, full operation trajectory, retired-instruction and exact fall-degree claims using explicit known claim keys. Native invocation/statistics are producer-reported boundary facts, not independently proved algorithm execution or basis derivation. Before exposing them, reconcile against validated native metadata instead of fabricating values from the mathematical result.

Original profile selection must OR frozen solver identity and reserved dynamic path/role/strategy. Existing generation plan validation intentionally rejects the new identifier until runtime integration and all gates are ready. Historical f4 keeps its existing semantics. This file records integration obligations; no outer reconciler or production activation is implemented by it.

## Implemented reconciliation increment

verify_dynamic_claims now checks mathematical/durability/completion labels against their separate replay fields, requires boolean core labels, refuses unsupported proof claims, checks optional native flags against explicitly reported metadata, prevents completion after outer persistence failure and validates status/censoring. Native metadata now exposes reported invocation and statistics completeness; complete statistics cannot coexist with a reported non-invocation. These reported flags do not prove native execution or basis derivation. Extraction-completeness promotion remains unsupported until an exact corresponding replay result is exposed.

The actual ordinary/exceptional producer-file tests exercise accepted labels, rejected proof upgrades, nonboolean positive labels, flipped native metadata, outer persistence failure and conservative timeout labels. A metadata control checks the invocation/completeness inconsistency. New controls are unrun at commit time. Preceding source0fd2d838db414f45c227bfb6aa7f4f796022aba2 passed584focused tests and strict release Clippy. Parent registration and runtime/profile activation remain separate work.
