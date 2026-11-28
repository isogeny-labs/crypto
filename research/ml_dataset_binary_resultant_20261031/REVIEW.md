# Independent pure binary-m3 resultant review

Reviewed external patch SHA256 `b9b498a4509b63457692d2a46ca6131d8af44110f7b173cbbc6a4cdada5f3be8`, draft module, notes and source bindings. All four bound source-file hashes match the current filesystem: operation_tally.rs, f2m.rs, binary_semaev.rs and pq_groebner_f2.rs. No worktree changes, compilation, Rust tests or cloud activity performed.

## Algebra and representation

No algebraic blocker found. Native S4 uses the characteristic-two quadratic resultant of the two S3 coefficient triples. The reference builds the corresponding4x4 Sylvester matrix and evaluates all24 permutation products, with no sign distinction in characteristic two. Its multiplication/reduction uses independent u128 bit operations and never calls native multiplication, squaring or resultant code. At n<=61 the largest product bit is120, so all intermediate shifts fit u128; reduction leaves degree below n before narrowing to u64.

The two S3 coefficient triples retain the actual supplied b. Eight native evaluations select codes by the three Boolean assignment bits in the same order that Möbius inversion and F2BoolPoly::eval use. The Boolean coefficient transform is complete, and descending cmp_mono sorting produces canonical, duplicate-free masks0..7. The verifier checks equation count, n_vars, mask range and canonical order before comparing all eight evaluations. Equality on all eight assignments establishes equality in the three-variable Boolean quotient.

Stored n, full modulus bits, actual b, target_x and ordered codes are compared against inspected inputs. Modulus exponents must be sorted, distinct, below n and include constant1; b is nonzero and code0<code1. No implicit b=1 substitution appears. The module intentionally does not prove irreducibility, point membership, signed-domain validity or PDP equivalence; these remain original-token/domain responsibilities. Degenerate leading coefficients still produce the specialized Sylvester determinant; no stronger group-solving conclusion is asserted.

## Admission and actual field path

Metadata4096 admission precedes bounded inspection. Construction admission precedes native arithmetic and vector reservations; deadline checks surround each resultant evaluation and equation construction. Native S4 has9multiplications,5squares and8additions per evaluation, hence72/40/64 across8. This agrees with112 charged field operations plus the separately listed add/ANF work. Current n<=61 values occupy one inline word, mul/square use the bounded3-word view of20-word scratch, and reduction has width>=1 with at most n-1 folds over the bounded low terms. No trace-table/inversion or symbolic-polynomial path is called.

The reference has6 coefficient products plus24*4 determinant products per assignment:816 reference products. Each uses n multiply-bit iterations and n-1 reduction-bit iterations. The separately charged256 permutation candidates per assignment and bounded canonical/evaluation work are present. These quotes are declared modeled operations/requested capacities, not machine instructions, RSS or allocator-overhead measurements.

OriginalAdmission constructs no budget/deadline and accepts the caller's cumulative ledger. The pure interface cannot prove that a future implementation actually debits the original budget or preserves first stop; the notes correctly require that integration. Its callbacks must also remain free of tally activation. A check_deadline/reserve implementation that starts a tally after inspect would invalidate the nonallocating field-path premise; this must be reviewed when the real adapter is supplied.

## Narrow correction requested

`current_thread_active_or_borrowed()` uses `try_borrow()`. A pre-existing shared borrow of an empty tally stack therefore returns false, contrary to its name and stated borrowed-state guard. If global ACTIVE is nonzero due to another thread, tick subsequently attempts borrow_mut and would panic while that shared borrow remains. Existing borrow lifetimes are private and ordinary current calls do not demonstrate this situation, but the new accessor should implement its stated contract. Use `try_borrow_mut()` solely to inspect emptiness, without mutating state, and add controls for existing shared and mutable borrows. This is a small accessor correction, not a resultant-algebra defect.

## Controls and activation scope

Draft controls are substantive: actual arbitrary b values and all three-bit assignments compared against the independent determinant; altered retained value; a changed genuine ANF coefficient while canonical shape remains valid; invalid mask; different original b; failed construction admission; charged-but-expired admission; active tally refusal with no recorded field operations; malformed modulus and repeated selector rejection.

Additional useful pure controls before claiming the entire represented n2..61 domain: n2 and n61/full-width inputs, dense low-term reduction, changed original modulus with retained built object, descending selector order, and verify-side admission/deadline refusal. These need not imply runtime curve eligibility or mathematical irreducibility testing; choose fixed valid irreducibles for positive field cases and label quotient-only cases accordingly.

The patch has no module registration or dispatcher change, so its tests will not run merely by applying the file. Notes explicitly require authorized test-only registration and tally-accessor scope, followed later by verified-original/shared-budget/custody/central replay integration. That boundary is honest: no native matrix, full solver, ordinary-binary runtime population, first/last fall degree or group-negative capability is claimed. Algebra/source review is favorable; address the accessor contract and run the registered pure component controls before treating it as validated.

### Corrected tally guard revision

Reviewed updated patch SHA256 `ce327a2d0ba7fb6e32d013964760b32efcc711e98f786f89c1c856417995f4dc`. The pure resultant module is unchanged. The accessor now uses `try_borrow_mut()` as an exclusive availability probe without mutating contents. Added controls retain a shared Ref and a RefMut on an empty stack, require refusal and unchanged contents, then require admission after release; a populated active tally control checks its existing entry survives inspection. This resolves the specific accessor-contract finding on source review. No remaining concrete pure-component integration blocker found; module registration, real shared-budget adapter, broader edge controls and compilation/test evidence remain separate obligations. I did not run those tests.

## Boundary-controls update review

Reviewed patch SHA256 `f1e8d6b4412a62ef67fe5e49d4b72419f64c0600daaff521cd2a96e76dbb2cfe`. No new blocking issue found. Production remains unchanged after the exclusive tally-availability probe correction. Independently recomputed both saved n=2 and dense n=61 golden cases from the separate Python polynomial-long-division, 24-permutation determinant, subset-sum ANF reference: all resultant and ANF arrays match the saved JSON. Dense n=61 is explicitly quotient-algebra evidence, not an irreducibility assertion.

New substitution controls change modulus, target, and ordered codes. Admission controls exercise both verifier reservations and all four deadline gates, retaining the expected 4096 or 120064 modeled work and 0 or 8192 bytes; success uses one cumulative build-and-verify admission object. These are meaningful failure-accounting controls, not an implemented runtime budget adapter. Rust tests/builds were not run in this review.
