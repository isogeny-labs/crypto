# Native bounded Boolean implementation handoff

2026-10-30 UTC. Design-first commit3571c10b877e; basea19b1425d08f5a946587d62854368f7207f4f237. Source changes are additive in the two delegated files. No legacy body, builder, dataset, exporter, recursive solver or cloud workflow was modified.

## What was done

- Added koblitz_groebner::bounded_matrix::{matrix_f4_f2_bounded,required_reservation,Limits,Report,Event,Stage,Stop,Engine,CriterionStatus}. Strictly admits exactly3 variables,<=61 canonical equations, masks0..7, at most8 terms each, degree1..6. The entry reuses echelon_f2_suffix_counted directly after fixed-domain parity construction and actual-column packing.
- Added matrix_f5_f2::matrix_f5_f2_bounded and private bounded_k2_criterion/bounded_k2_reduce. They implement the existing W_i(d) Koszul/Frobenius criterion with permanent and temporary fixed pivot arrays, preserve generator/level/multiplier order and the zero/constant bypass, then call the common bounded main matrix route. No naive self-leading-ideal substitution.
- Every dynamic allocation is fallible and precovered by a cumulative requested-layout reservation; no input clone, row-growth allocation, pivot clone, criterion hash map or Rayon path. Fixed arrays contain the native small-domain criterion and pending row supports. A main candidate is capped before its callback or product/copy. Rank/output is only published after acknowledged Finish and its post-callback deadline check.
- Authored11 tests: six F4 and five F5. These include256 ANFs*6 degrees against legacy F4 and an independent numeric-column row-space oracle;256 ANFs*6 degrees of exact F5 criterion pruning/counter equality; fixed multi-generator/unsound-shortcut/Koszul/Frobenius controls; empty/zero/one statuses;61-equation active criterion and488-attempt maximum; exact/one-less work, byte, row and event limits; invalid representations; pre/post-observer deadlines; sticky first/intermediate/terminal callback failure. Exact canonical returned rows are compared with the legacy paths in addition to independent spans.
- Scoped Rust1.98 rustfmt completed. git diff --check completed with exit0 before final documentation. No Cargo invocation, compilation, tests, scientific measurements, data generation or cloud action was performed by this implementer.

## Resource model mapping

DESIGN.md is authoritative. entry_reservation maps its first row to Gate::prepare; main_reservation covers Gate::main, product, columns, pack, allocation attempts, suffix RREF and serial output; criterion_reservation covers bounded_k2_criterion/reduce. Gate::reserve never refunds, Gate::vec cumulatively counts actual dynamic layout requests, and Gate::emit separately counts attempted and acknowledged callbacks. required_reservation includes the F5 permit even when the criterion bypasses constants. Maximum structural main attempt count488 and criterion count1952 yield at most2448 boundary event attempts under completion, below4096.

At E61 the frozen modeled work is2,809,152 for F5 (including F4's main work), not an instruction count. On the ordinary64-bit Mask/usize layout the cumulative requested-layout allowance is24,576 bytes; wide-vars uses size_of rather than that numeric literal. Neither value proves physical memory or compiler stack usage. The original worker envelope and caller input/capture coexistence remain adapter obligations.

Only typed report counters describe this new API's prefix. The main reused kernel has its existing tally side effects; this API does not promise a complete process/global f4_word_ops or bitop tally. F5 criterion XOR counts represent one packed-row XOR per elimination, matching the existing one-word criterion counter semantics even though the fixed K2 storage is a byte. They are not retired64-bit instructions. Caller-owned observers, allocator/runtime internals, wall clock implementation and compiler stack layout are excluded exactly as frozen.

## Decisions and limits

- Native_step_complete means one admitted matrix step only. No complete Gröbner basis, recursive search, enumeration, infeasibility, scalar recovery, full trace or total-operation verification is asserted by this API.
- Stop/criterion status and partial counters remain available on failure; rank is absent and rows empty. Boundary callbacks are not primitive state certificates; acknowledged pre-candidate events do not prove that candidate completed. A callback can acknowledge and then cross the deadline; its acknowledgment is retained while completion is refused.
- The tiny scalar RREF is atomic: deadline checks immediately surround it, not each pivot. Observer body allocation, blocking and panics remain caller responsibilities; ordinary returned refusal stops all later work.
- No environment-mutating test was authored: the new route has no environment/cached/parallel selection calls by source inspection, and old wrappers are only test references. No claim of exhaustive environment testing.
- All11 native tests remain UNRUN. No source/algorithm/memory-proof acceptance is claimed from formatting or this self-review. Parent owns independent review, pinned build, strict lint, focused/full execution, evidence persistence and delivery.

## Open threads

Parent source review and serialized validation; correction of any real diagnostics while retaining initial evidence. Dataset Koblitz m3 integration, signed-domain extraction, first positive certificate persistence, independent consumer replay, exporter support, recursive/larger-selector profiles and cloud corpus generation remain separate work.

## Errors retained

Initial Conductor check used obsolete migratedproject-prefixed identifier and returned404; corrected exact UUID check found only parent-owned T868/T876, whose active task cards were read before edits. Sparse docs/CONDUCTOR.md and latest session log were read from tracked Git objects; parent owns shared logging. No compiler/test errors have been observed because no compilation/test was attempted.

## Independent-review correction, 2026-10-30

Independent source review of b8579d2 by binary_integration_audit found that prepare marked backend_invoked after Begin but before any native phase permit. A one-equation entry-only work budget4224 therefore reported invocation despite refusing the main/criterion phase. The original commit and review remain retained; this correction is based on root's f8a12010b138 common lint fix and leaves that fix untouched.

The flag now becomes true only after the named F4 main or F5 criterion phase reservation succeeds. Its explicit meaning is **native phase admitted**, not nonempty work or completion. Main row-cap refusal can follow admission; empty F4 can execute an admitted rank-zero step. EmptyInput/SkippedConstantInput F5 phases remain explicit with zero criterion counters, including a subsequent main-permit refusal. Useful adapter acceptance must require actual nonzero native work and the appropriate active-criterion control, not the flag alone.

Three additional tests are authored (14 total): F4 entry-only/one-less/exact main admission, F5 entry-only/one-less/exact criterion admission followed by main refusal and explicit bypass semantics, and both engines reaching Finish whose callback returns Ok only after the original absolute deadline. The last control asserts actual Finish was reached, all callbacks were acknowledged, native work/storage occurred, and the final result is Deadline with no rank/rows. All14 remain unrun by this implementer. Scoped formatting/static whitespace checks are not compile or test evidence; root retains serialized validation ownership. One initial apply_patch attempt rejected a formatted-context mismatch without changing files; the narrower patch then applied.
