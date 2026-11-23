# Preserve incomplete native checker outcomes

Read-only inspection of published central integration found no concrete Rust type, visibility, module-path or disjoint-field borrow error. This is source inspection, not a full-module typecheck.

A concrete classification defect was found: dynamic_boolean_search_bqi.rs preserves the BQI audit but maps every failed checker result to bridge::Failure::Checker. Native central previously classified that as Rejected even when the retained result was bqi::Failure::Unknown(resource exhaustion). The narrow proposed central patch recognizes only that exact retained Unknown case as Incomplete. Malformed, equality mismatch and source mismatch remain rejected. It changes no budget, audit, source identity, positive scope, deadline or first-stop behavior.

Actual BQI accounting invokes the owner before every admitted resource increment. Bridge Meter passes the increment to semantic Nested Meter, which passes it to the original ReplayBudget. Each ancestor enforces its existing cumulative work/bytes and deadline; no per-row reset can renew original allowance. Allocation resource increments consume both modeled work and bytes. BQI local cap refusal can occur before the original owner callback and is therefore detected through the retained audit.

Validation: 14 focused tests pass (4 new controls plus 10 actual BQI donor controls). The new resource control forces each of all nine real checker resource caps to zero; all remain retained Unknown results classified as incomplete. Wrong identity remains EqualityMismatch and is not classified as exhaustion. Actual valid identity passes. Original owner refusal remains sticky with bounded charges. Actual bridge/checker modules are included directly; Row/ProofTerm/Charge/Owner are explicit structural fixture seams. No full central typecheck or full Cargo build was performed.

The first focused compile hit the local 1 GiB virtual-memory cap in the multithreaded linker. compile.initial.log preserves this failure. Retrying with a single linker thread under the same cap passed; tests.log records all 14 results. No published repository source was changed.
