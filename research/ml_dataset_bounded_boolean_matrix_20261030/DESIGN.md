# Bounded serial Boolean K2 matrix steps

Frozen before source changes, 2026-10-30. Base a19b1425d08f5a946587d62854368f7207f4f237. Parent delegates T868 koblitz_groebner.rs and T876 matrix_f5_f2.rs plus this research directory. Conductor check identified exactly those parent-owned tasks; their active cards were read. No dataset, builder, exporter, legacy wrapper, cloud or recursive-search changes. Parent owns builds and delivery.

## Sources and algorithm identity

The copied ADAPTER-CONTRACT.md and PRE-IMPLEMENTATION-REVIEW.md retain the original proposed adapter scope and concrete native hazards. Their source hashes are respectively d8af782e05110d2ccd0cddcd9d21d88c1686710bd436667a049d26ea7f9d7d21 and 14894730e61a20a18e605d3d20b697229974c473ad9c365493881a9d016083a9.

Read the existing matrix_f5_f2.rs module documentation, F5Criterion::new, PrefixEchelon, TempEchelon, Macaulay construction and echelon_f2_suffix_counted. The source's Boolean criterion is W_i(d)=V_{i-1}(d-d_i)+span{s*(f_i+1):deg(s)<=d-2*d_i}. Reproduce that actual criterion in fixed three-variable storage, preserving generator order and the existing constant/zero bypass. This is reuse of the native algorithm, not a new mathematical result. The small representation is compared directly with F5Criterion in authored controls.

Primary background checked: Eder/Faugere, https://arxiv.org/pdf/1404.1774, section 3 on Macaulay/signature rows and section 6.1 on Koszul syzygies. Those general signature algorithms do not by themselves establish the Boolean quotient adaptation. The existing module gives that adaptation and its counterexample to including the current generator's leading ideal. This increment is a fixed-degree matrix step, not full signature-compatible F5 or a recursive solver.

Rust Vec official reference https://doc.rust-lang.org/std/vec/struct.Vec.html#method.try_reserve_exact was checked: requested capacity is fallible; allocator over-allocation is possible. Consequently the bytes below are a conservative cumulative requested-layout model, NOT RSS, allocator metadata, total process memory, instruction counts, or a proof of exact native cost.

## API and strict representation

Add public bounded API/types inside koblitz_groebner::bounded_matrix and public matrix_f5_f2::matrix_f5_f2_bounded. F4 entry is bounded_matrix::matrix_f4_f2_bounded. Borrow inputs; n_vars must equal3; E<=61; degree in1..=6; each polynomial must itself declare3 variables, contain<=8 distinct canonical terms, and use only masks0..7. Check cheap shape caps before inspecting terms, before copying or invoking observers. Invalid representations are never normalized silently.

Explicit limits: conservative work<=16,777,216 units; requested-layout bytes<=64MiB; main candidate row attempts<=488; observer event attempts<=4096. Zero limits are valid censorship controls. Fixed metadata envelope8192 bytes is admitted before representation traversal/callback; no native diagnostic String. Caller-owned input and callback state are excluded and must be included separately by any adapter. Rejected inputs retain no copied equations. Empty and zero equations remain visible; any constant-degree input disables the F5 criterion as in legacy code, reported distinctly from active criterion work. No preprocessing or input reordering.

Return report with typed engine/stop/criterion status, reserved work/bytes, actual candidate/nonempty/selected rows, actual criterion rows/zero reductions/pruning counts, separate criterion/main word XORs, attempted/acknowledged events, and returned rows/rank only after terminal acknowledgment and deadline check. On every failure the returned rows are empty, rank absent, native_step_complete=false. Retained counters describe the completed prefix only. No full trajectory, ideal derivation, recursive completion, UNSAT, solution count or total cost flag is exposed as true.

## Exact serial route

Use fixed multiplier order [0,1,2,4,3,5,6,7], filtered by degree. It is the existing uncached three-variable monomial schedule. Boolean product is XOR parity of masks (term.mask | multiplier), represented by a u8 support bitset; this is exactly product_monos sort/parity semantics. Column order uses existing mono_key, sorted by a fixed insertion sort over<=8 masks. Main columns are the actual selected-row support union, not always all8. No ambient environment, caches, HashMap, Rayon, direct-pack fallback, M4RI, support split, rank-certificate clone, or projection.

Criterion uses per-level fixed [u8;8] pivot rows indexed by ordered columns, plus fresh temporary pivots per generator. Process only distinct levels d-d_i (at most3); before inserting generator i's prefix products, read existing pivot leading masks and the reduced Frobenius products f_i+1. Filter products to the current level after cancellation, matching pack_product's missing-column fallback. Count lower prefix attempts and zero reductions separately from Frobenius candidates. Criterion bitsets are fixed [u8;61]; no allocations or generator clones.

Main selected supports are retained in fixed [u8;488]. Admit each eligible main candidate against the row cap before observer or product/copy, including zero/pruned candidates. Then fallibly reserve exactly the selected number of Vec<u64> descriptors and one u64 per selected row. Reuse echelon_f2_suffix_counted(matrix, C, ..., true) directly: existing allocation-free scalar borrowed-pivot RREF, atomic small kernel, no copies. It is preceded/followed by deadline and fallible boundary events. Serial output uses fallibly reserved outer Vec (<=8) plus one terms Vec (<=8) per nonzero reduced row; create canonical raw F2BoolPoly without from_monos extra allocation. Legacy functions remain byte-identical apart from inserting the new module/entry/tests.

## Conservative resource model, frozen before code

All calculations use checked u64 arithmetic and size_of for feature-sensitive layouts. Work units are deliberately conservative loop-operation allowances, not retired instructions. Reserve before the covered phase; reservations are cumulative and never refunded after refusal. All initial validation, native bookkeeping, observer-call boundary overhead, and destruction are covered by the following allowances; observer body, allocator internals, platform clock implementation, runtime/TLS initialization, compiler stack layout and process infrastructure are explicit exclusions.

Let E<=61, R=8E<=488, C<=8, Q=32E (3*8E prefix plus8E Frobenius candidates). Row cap is an actual next-main-candidate cap; model reservation still uses the structural worst-case R. Fixed metadata8192 includes report/gate/result descriptors and simultaneously live fixed arrays (488 support bytes,61 degrees,61 pruning bytes,<=4 sets of8 masks/pivots/indexes, and bounded scalar scratch); this is a reviewed allowance rather than a claim about compiler frame layout.

| Phase / bounded source operation | Reserved work | Requested-layout bytes / allocation rule |
| --- | --- | --- |
| Entry cheap caps, metadata, term validation, degree/support scan | 4096+128E |8192; before input traversal/first observer |
| Main candidate dispatch, up-to8 term OR/XORs, support merge, row admission/counters/events, final column insertion sort, row packing and readback |4096+1024R |R*(size_of(Vec<u64>)+size_of(u64))+8*size_of(F2BoolPoly)+64*size_of(F2BoolMono); one request per actual allocated row/output, no growth/reallocation |
| Main suffix RREF pivot search, swap, row bit tests, suffix XOR, bookkeeping and tally |64*9*(R+1), included in main phase permit |0 additional; borrowed pivots, one word per row, no parallel buffers |
| F5 level/generator dispatch, initialization, masks/leading scans and criterion event gates |4096+128E, only F5 |0 additional beyond metadata |
| F5 prefix/Frobenius candidate packing, cancellation, up-to8 pivot probes/XORs, insertion/leading masks/counters |1024*(Q+1), only F5 |0 additional beyond metadata |

The large multipliers make each line's scalar loop bound explicit: main term packing<=8R, column packing<=8R, insertion sort<=64 comparisons/swaps, readback<=64 tests; kernel pivot search/row tests each<=8R and XOR<=8*(R-1) for nonempty R. F5 candidate packing<=8Q plus at most8 level tests and8 pivot tests/XORs per candidate; at most3E generator passes scan<=8 pivot masks. No polynomial-degree dependent unbounded loop is admitted. Maximum loop counts use fixed mask domain rather than arbitrary coefficient degrees.

Work and byte admission occurs before callbacks or allocations for that phase. All dynamic allocations use try_reserve_exact with prechecked Layout::array; capacity is never exceeded by push. Allocator may round upward, so original worker memory enforcement is still necessary. Cumulative requests account for coexisting matrix and output vectors and every row allocation; no hidden f+1 or input clone. Rows are not cloned when pivoting. Partial allocation failure drops owned prefixes and returns Allocation; no success claim.

## Deadline and observer contract

Accept one absolute Instant deadline from the caller; never refresh it. Gate before/after each observer and at each criterion/main candidate, criterion level, main packing/reduction/readback and terminal. Main kernel is an atomic admitted <=488-by8 unit; deadline is checked immediately before and after, not guaranteed preemptive within the kernel. Callback owns its own allocation/time behavior. Each event contains a fixed copied counter/reservation snapshot; events themselves are caller-retained, no native event Vec. Attempts count invocation; acknowledgments count Ok, even if the subsequent deadline check fails. First error stops all later processing. Event cap is checked before invocation. Terminal refusal or boundary timeout leaves rank/output unavailable. Acknowledged boundary events are not every primitive operation.

## Authored acceptance controls and validation handoff

Write but do not run: all256 single-polynomial ANFs at degrees1..6 versus legacy F4 plus independent tiny row-space oracle; fixed multi-generator row-space/RREF controls; exact F5Criterion pruning/counters including the known n_vars3 degree5 unsound-shortcut case; nonzero Koszul and Frobenius controls; empty/zero/one bypass; strict malformed inputs; maximum61-equation/488-attempt control and row cap one-less proving no cap+1 event; exact/one-less work/byte/event limits; first/terminal observer failure; expired and callback-expired deadlines; complete results canonical and failure results have no rank/rows. Legacy APIs are unchanged. Tests are software validation, not scientific runs or cost comparisons. Parent compiles, runs strict lint/focused/full gates and preserves failures. Any failed authored premise is reported and corrected without hiding the initial evidence.

## Remaining scope

No dataset dispatch, original signed-domain extraction, first-certificate persistence, independent consumer, new export schema or cloud generation is delivered by this native increment. No F6, larger selector profile, recursive solving, generalbinary m3, fall-degree conclusion, wall-time gain, exact operation count or new budget is claimed.
