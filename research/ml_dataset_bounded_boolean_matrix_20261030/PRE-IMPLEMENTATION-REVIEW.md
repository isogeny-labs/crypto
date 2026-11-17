# Independent source review: bounded Koblitz m3 matrix-step contract

Date: 2026-10-30 UTC. Read-only review; no source edits, build, tests, generation, or cloud activity. This report is the only new artifact written by the reviewer.

Contract reviewed: `/home/ec2-user/ml-dataset-resume-20261029/koblitz-m3-dataset-adapter-contract-20261030.md`, SHA-256 `d8af782e05110d2ccd0cddcd9d21d88c1686710bd436667a049d26ea7f9d7d21`.

Native source was read from immutable Git objects at `a19b1425d08f5a946587d62854368f7207f4f237`; the contract identifies its dataset baseline as281b4ba. This review does not infer that authored controls have run or that any new profile is integrated into current main.

## Recommendation

The proposed matrix-step increment is an honest, practical next implementation, provided the native bounded entries are implemented before dataset invocation. Neither current public F4 nor F5 wrapper supplies the required allocation, deadline, or fallible observer contract. Do not approve a numeric memory envelope merely from C<=8 and R<=488.

Implement the deterministic K2 serial F4 entry first, then the bounded version of the existing F5 Koszul/Frobenius criterion and matrix step. Reuse the existing row construction, monomial order, cancellation, criterion, and suffix elimination semantics in their owning modules. Preserve existing entry behavior. Do not route the new profile through the recursive solver or substitute enumeration for its named native step.

The builder-only Weil increment remains distinct and can be integrated separately after its own native and adapter gates. The signed-domain replay is separately charged extraction and certification; it is not native F4/F5 enumeration.

## Required refinements and concrete source hazards

1. **Admit the entire representation, not only n_vars.** Require exactly n_vars=3 for the K2 native profile; at most61 input equations; each equation's own n_vars=3; at most8 canonical, distinct monomials per equation; and every mask confined to bits0..2. `F2BoolPoly::is_canonical` checks ordering/uniqueness, not mask range or its n_vars field (`pq_groebner_f2.rs:424–499`). Those public fields can otherwise invalidate the small bounds. Reject before cloning, converting, allocating schedules, or invoking an observer. Fix the admitted degree range explicitly; 1..=6 matches the current dataset matrix degree ceiling, while the mandatory fixture can remain degree4. A narrower degree4-only profile must say so.

2. **The current F4 row cap is too late for a strict bounded callback.** `visit_macaulay_rows_counted` calls the row visitor at2639–2640 and only then checks `full_count > row_cap` at2644. Its ordinary visitor copies the row into retained storage (`macaulay_rows_monos_with_mask`,2370–2379). A new bounded visitor must admit the next candidate/storage before its copy or callback; an exact/one-less control must detect a forbidden cap+1 row. Keep attempted, nonempty, selected, and acknowledged counts distinct.

3. **F5 zero/constant input disables its whole criterion.** The bounded builder emits all n coordinate equations, including zero polynomials (`koblitz_s4_bounded.rs:292–317`). `F5Criterion::new` immediately returns empty pruning sets when any input degree is0 (`matrix_f5_f2.rs:348–356`), including a zero equation. A main reduction can consequently run with no criterion work. Preserve this behavior and label the skipped-constant-input case, or explicitly freeze a nonzero-generator projection with original-index mapping and retained original equations. Do not silently remove/reorder generators or count invocation alone as nonvacuous F5-criterion validation. Constant1 must remain a separately handled, visible case.

4. **The F5 default path may build twice.** `matrix_f5_f2_with_form_timed` tries `f5_rows_packed_full_columns` then falls back when not every column occurs (`matrix_f5_f2.rs:914–949`; `koblitz_groebner.rs:2443–2546`). The failed attempt has already allocated and processed candidate rows. A cumulative model must charge both attempts, or the new K2 entry should select the existing monomial-list construction directly and never attempt this optimization. The latter is the smaller auditable increment.

5. **Small matrices do not eliminate every parallel or ambient path.** Main packing is serial at K2 dimensions because R*words<2^20 (`koblitz_groebner.rs:2241–2260`), and reduced elimination selects the scalar suffix kernel because C<256 (`:4395–4423`). That suffix kernel borrows the pivot and performs no pivot-row clone (`:3923–3975`). However, F5 readback unconditionally uses `par_iter` (`matrix_f5_f2.rs:1030–1064`). Its default builder/unpack choices and many F4 wrappers read environment variables/OnceLocks. Pin a serial readback, Reduced output, explicit uncached schedule and column construction, and direct suffix kernel in the new API; do not rely on ambient flags or a previous process's OnceLock state. With those choices, large M4RI tables, rank-certification clones, selected-column projections, support-split matrices, and Rayon worker buffers are outside this admitted route. Do not accidentally charge nonexistent pivot clones as measured work; a conservative storage allowance is a different quantity.

6. **Borrowed input remains resident while outputs are built.** The builder's returned equations, adapter canonical bytes, main matrix, criterion state, and eventual returned rows can coexist. Phase admission must include that resident state and cumulative allocations. A new per-phase full memory allowance is not justified. `try_reserve_exact` provides fallible allocation but does not prove allocator/RSS exactness; state whether bytes mean requested layouts, cumulative reservations, or another explicit model. The original worker memory limit remains separate enforcement.

## Small bounds that can guide the implementation

Under the strict K2 representation above, let E<=61, C<=8 and R<=8E<=488. These are structural bounds, not a certified byte or instruction tariff.

| Component | Bound and relevant behavior |
| --- | --- |
| Input support | At most8E<=488 masks; exactly3 selector variables. |
| Main candidates | At most8 multipliers per generator, hence at most488 attempts; cancellation can make a candidate empty. |
| Product work | At most8 term OR/parity contributions per candidate, hence at most3904 main product contributions. If sorting is retained, its comparisons/scans must be added. |
| Main reduced elimination | Rank<=8; pivot-search and row-test scans each bounded by C*R; at most C*(R-1)<=3896 one-word XORs when R>0. These counts exclude construction, criterion, conversion, admission and observer work. |
| Returned reduced rows | At most8 rows, at most8 terms each; include Vec descriptors, capacity requests and any canonicalization copy. |
| F5 lower levels | With no constant/zero input, input degrees are in{1,2,3}, so at most3 distinct levels d-d_i. At most3E*8<=1464 prefix-row attempts in total. |
| F5 Frobenius candidates | Each generator's temporary span is evaluated only at its own level: at mostE*8<=488 candidates. |
| F5 retained pruning | At most8 masks per generator, hence at most488 mask memberships. Preserve generator identity/order. |
| F5 echelon state | Each permanent or temporary echelon has rank<=8. The temporary pivots extend the permanent span; separate8-row allowances are conservative if simpler to audit. |
| F5 reduction work | Each lower/temp candidate has at most8 pivot eliminations. Candidate counts, term packing, prefix scans, pruning lookups and construction of f+1 must still be charged separately. |

The current F5 criterion stores Vecs plus hash maps/sets (`PrefixEchelon`,152–196; `TempEchelon`,303–309; `F5Criterion`,312 onward). It repeatedly transfers row storage with `mem::take`, and constructing f+1 allocates the one polynomial plus an addition output with capacity len(f)+1 (`pq_groebner_f2.rs:634–665`). Merely counting the final8 pivots misses these allocation events. A fixed-size3-variable index/pruning representation and reusable row scratch can make the new profile's allocation ledger substantially simpler, but must preserve exactly the existing criterion and be compared with it.

For raw rows, use their actual occurring column union, in the existing `mono_key` order. Always retaining all8 columns changes legacy column/report identity even if zero columns do not change the polynomial span. Preserve full F4 nonempty-row counts versus selected F5 counts, including zero products that were pruned; `F5Report::rows_pruned` is not interchangeable with the size of the criterion's multiplier sets.

## Exact API and file route

- `src/cryptanalysis/koblitz_groebner.rs`: add a bounded matrix-step entry adjacent to `matrix_f4_f2_counted`; parameterize/share the candidate visitor and serial suffix reducer with explicit gates instead of calling the ambient public wrapper. Existing relevant helpers are `visit_macaulay_rows_counted`, `macaulay_columns`, `build_macaulay_packed`, `echelon_f2_suffix_counted`, and readback in `matrix_f4_f2_counted_impl`.
- `src/cryptanalysis/matrix_f5_f2.rs`: add the corresponding bounded entry adjacent to `matrix_f5_f2`; provide a bounded constructor/path for the existing `F5Criterion`, `PrefixEchelon`, and `TempEchelon`, then use the same admitted packed-row reduction and serial readback. The mandatory profile uses `F5OutputForm::Reduced`, not a rank-certificate variant.
- A small shared native limits/report helper may be factored only under an explicitly reserved scope; no additional generic matrix engine is required. Keep the native S4 builder byte-identical unless a real defect requires a separately reviewed correction.
- Dataset integration remains the proposed new `src/ml_dataset/koblitz_m3.rs`, narrow dispatch/helpers in `solvers.rs`, and reserved-artifact semantic verification in `mod.rs`. Do not widen the old m2 `boolean_solve` and its recursive call graph.

A suitable bounded entry takes borrowed canonical polynomials, explicit K2 profile/degree, immutable limits, the original absolute deadline, and a fallible observer. Its typed result should retain stop cause, actual entry/invocation state, native_step_complete, admitted/cumulative work and storage, attempted/selected rows, acknowledged events, separate criterion and reduction word-XOR counters, and only a valid final rank/output on completion. This is an API sketch, not an implemented signature.

Reserve fixed diagnostic/report storage before callbacks, check deadline before and after callbacks, and stop immediately after refusal. Check before expensive criterion levels, candidate copies, matrix reduction boundaries, readback and terminal acknowledgement. A refused terminal event must leave step_complete=false. Caller-owned callback allocations and runtime/allocator overhead must be explicit exclusions from the native model. Do not turn observer errors into successful empty rows.

## Meaningful acceptance controls

1. For fixed3-variable polynomials, compare canonical reduced output and full row-space rank with existing F4/F5 paths and an independent tiny bit-matrix oracle. Exhaustive single-polynomial ANFs are only256 possibilities; add fixed multi-generator controls that exercise prefix order, cancellation, dependence and both kinds of F5 pruning. This is proposed software validation, not work performed by this review.
2. Preserve the existing `naive_f2_criterion_is_unsound_in_the_boolean_ring` counterexample (`matrix_f5_f2.rs:1805` onward; n_vars=3, degree5). It prevents replacing the actual Koszul/Frobenius criterion with an invalid leading-monomial shortcut. If the new API admits only degree4, retain the old regression and add an appropriate admitted-degree pruning control rather than claiming that degree5 was exercised.
3. Test zero equations and constant1 explicitly: raw-input equality, criterion-skipped identity, original generator mapping if preprocessing is introduced, correct empty/unit behavior, and no useful-work claim from an empty path.
4. Exercise actual R=488 candidate admission with a fixed maximal generator collection; separately test exact and one-less row, work, byte and event limits. The row-limit control must inspect the row/callback count so a cap+1 allocation cannot hide behind a final censored status.
5. Reject n_vars mismatches, high-bit masks, duplicate/out-of-order terms, excessive equation/support counts, unsupported degrees and overflowing limits before materialization. Check default and wide-vars builds' size model if both are supported; Mask is feature-dependent u64/u128.
6. Fixed environment-variation controls must demonstrate that the new profile does not switch caches, matrix form, direct-pack fallback, rank certificates, kernel or Rayon behavior. Do not mutate global test environment concurrently.
7. Deadline and observer refusal at admission, criterion, main reduction, readback and terminal event: no further work/event submission after refusal, no complete/rank/negative promotion from an unfinished result. Preserve acknowledged prefixes and actual counters.
8. Then run the contract's fixed Koblitz n23/31/47/53/61, both-twist controls, after coordinates/preconditions are frozen. Require actual named-engine work: nonempty admitted rows and a genuine reduction, with a separate criterion-active/pruning control for F5. Its positive3P, negative7P and exceptional-prefixP cases on domain±P,±2P are consistent when ord(P)>13; the negative target differs from every possible sum coefficient in[-6,6].

## Solver, extraction, and export claim boundary

The native F4 function is one degree-bounded Macaulay row-reduction step, not the complete iterative F4 algorithm or recursive solver. The F5 module explicitly implements a Koszul/Frobenius row criterion over that matrix, not incremental signature-compatible F5 basis construction (`matrix_f5_f2.rs:72–82`). Preserve those variants in capability/profile metadata, not merely the strings f4/f5.

Retain all original field, b=1, family, target, signed-point/domain and selector bindings. S4 roots alone do not certify signed decomposition or group-domain completeness. The original signed-domain replay has at most64 ordered triples for K2, including repeated points and identity intermediates. Positive proof can coexist with later censorship; exact cursor completion and no failure are required for a negative. Persist the first certificate before subsequent traversal, using the already corrected live/canonical identity and sticky-failure rules.

Add or force `execution_trace_complete_verified=false` and `total_operation_count_verified=false` alongside `native_basis_derivation_verified=false` in the new typed result and verifier. Boundary events, modeled reservations, F4/F5 word XORs, group additions, and PMU measurements have different scopes. Encoding-complete, matrix-step-complete, extraction-complete, positive PDP and full recursive-search completion must remain separate fields. Static feature export does not imply new S4 equation-graph support or independent consumer replay; those remain separately owned work.

## Immutable source hashes

| Git object at a19b1425d08f5a946587d62854368f7207f4f237 | SHA-256 |
| --- | --- |
| `src/cryptanalysis/koblitz_groebner.rs` | `13511afae943058c1dea854e2ebeee2f8db4ed5acc39ef0a7f27780d0244d970` |
| `src/cryptanalysis/matrix_f5_f2.rs` | `c98fca26137bf190f37f7dcdef024326ef4a53c9c8080bf17ed9dfae51f08c94` |
| `src/cryptanalysis/pq_groebner_f2.rs` | `1c2f25c90307a94450b0518434ab82b0e9da72bbba15a28e63a17e8e1119e0a1` |
| `src/cryptanalysis/koblitz_s4_bounded.rs` | `bdabebed543010db7b265462e495cd30bd8760bcd534824a992308047b9591f6` |

No complete native memory bound, asymptotic solver result, first/last fall degree conclusion, scientific measurement, or cloud corpus delivery is established by this report.
