# Pre-execution F13 dual implementation bindings

The reference adapter is exactly the old cm8-transfer.cpp with `main` renamed
and an explicit successful `return 0` where C++ supplies it implicitly for main.
No arithmetic/predicate/order changes. The first compile correctly rejected
the missing explicit return; original adapter/log and a mistakenly masked shell
exit marker are retained as failed preparation, not a successful build or run.

Request vectors count the inverse kernel separately from a common boundary.
Reference enumeration charges one scalar/Phi/comparison per actual trial;
cached reference charges7scalar/7Phi construction calls once plus actual lookup
comparisons. Candidate charges one D and one scalar4 call. EACH route also
executes the common input/output boundary: two scalar7 checks, one Phi image
verification, two curve-equation checks and three group comparisons. These
units have different costs; field operations/internal scalar work, allocation,
cache storage, hashing and audit remain unpriced. No total ranking or speedup.

The new Rust auditor reuses the OLD independent Rust prime-field/group prefix,
not any C++ arithmetic; its scalar multiplication is linear repeated addition.
Both complete S3 streams are also audited by the unchanged old Rust engine.
Candidate arm/index bytes must equal frozen reference; new dual/source metadata
is bound separately by dual.json/dual.sha256. All nine corruption payloads are
rehash-checked and must differ; source/scalar controls fail semantic checks
before final byte parity. All files/source versions are frozen before data.

The unchanged existing universal construction identity/certificate17 applies
to the same translated degree-two construction. New finite coordinate/group
equalities and a general additive-map implication are checked in Lean; no new
universal curve-polynomial/geometric-duality theorem is silently asserted.

Post-first-run metadata correction: the successful receipt originally named its
seven rejected valid points "nontorsion". Every finite elliptic group point is
torsion; these are points not killed by7. Change only the JSON key to
rejected_valid_non_r7_points, retain the old successful run/source, freeze this
correction and replay the FULL contract. No arithmetic/predicate/count changes.
