# Worker acknowledgement handoff and activation dependencies

Read-only integration audit, 2026-10-31, after the parent inventory and delivered-outcome increments. No runtime profile is activated by this contract.

## Observed current paths

`mod.rs`'s worker receives `SolverData`, checks each payload against reserved output names and duplicate paths, and calls `storage::write_bytes` for every returned payload. `types.rs::SolverData` contains an outcome and payloads only. `solvers.rs`'s bound wrapper also flushes returned payloads into capture before finishing the emitter. In contrast, the staged Koblitz parent persistence path already stores both capture and root copies and retains the actual acknowledgements. Sending these bytes through either generic return loop repeats admitted storage work.

Conductor read-only check: T887 holds mod.rs and solvers.rs and is this thread's existing shared integration task; its lease was refreshed live. T936 holds types.rs. Do not modify the shared SolverData contract or T936's scope for this integration. T954 continues to own storage implementation.

## Intended worker return path

Add an internal worker-specific return value in solvers.rs when the actual Koblitz producer is ready to return it. It carries the existing SolverData plus a separate set of root acknowledgements moved from the adapter's retained owners. Existing adapters continue to use their existing payload path. Koblitz must not pass its already acknowledged payloads through either generic capture publication or root writing again.

The worker must preserve reserved-name, prefix and duplicate checks across both lists. It must bind the acknowledgement channel to the same attempt and root used by original admission; an arbitrary solver-outcome JSON field cannot introduce acknowledgements. Only successful storage calls create those records. A successful capture publication without a root acknowledgement remains partial, and a parent record itself is registered exactly once. Moving acknowledgements must preserve existing receipts without cloning retained payload buffers, serialization, or writes.

Any collection allocation or validation work added by this handoff needs an explicit admission in the original allowance before it executes; a private Rust wrapper alone is not resource accounting. Worker registry transfer must preserve acknowledged roots even after a later parent stop. The final outer completion check uses both canonical mathematical evidence and independently verified parent delivery. Retained witness facts remain separately reportable.

## Setup audit before producer activation

`admit_original` re-reads original config, execution and capture files, checks the live emitter, and derives a single original deadline and resource allowance. File admission currently precedes construction of the adapter ledger. Its allocations and bounds therefore require review rather than assuming a complete setup audit.

`original_math` performs curve/factor-base checks, then constructs the canonical two-code/four-signed-point input. Its bounded input cardinalities (four signed points, at most 255 construction candidates, at most two lifts per candidate) do not price `curves::verify_public`, `curves::verify_factor_base`, `Group::read`, or domain construction. Their irreducibility, order, subgroup and construction replay work and retained allocations need explicit bounded admission or a validated-original token from the existing worker. No caller-prepared curve may be treated as original-input provenance merely because it passes local arithmetic checks.

## Completion evidence required

Use actual original execution/capture files for positive, negative and capped CLI fixtures. Include wrong-attempt/root, duplicate/reserved acknowledgement paths, partial capture/root failure, parent stop, and early setup failure. Show that already stored bytes are not republished, all allocation/work admissions remain original, and central replay checks the moved acknowledgements. Preserve both twists and requested field-boundary fixtures. Full current-source library/CLI validation and delivery gates precede runtime activation. The overall corpus axes and cloud budget remain unchanged.
