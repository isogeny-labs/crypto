#set page(paper: "a4", margin: 19mm)
#set text(size: 10.5pt)
= Native recovery arithmetic accounting
Prime-field follow-up · 2026-11-03

Seven reversibly instrumented modules now report setup, reusable preparation, query and independent verification separately. All eight recovery receipts match the unchanged native reference. Three checks pass once; 56 earlier tests are filtered.

#image("chart.svg", width: 100%)

== Findings
For k173, polynomial work records 14,624 products and 320 inversion calls. Every polynomial inversion operand is one. Geometry records 7,740 products; phase 2,250; value division 3,904; source recurrence and normalization 4,264. Verification separately records 54 products and 11 inversions. Query allocator requests total 599,560 bytes in this build.

== What the counters mean
Unit inversions are a subset of inversion calls, not extra work to add. Scalar operations modulo p minus one are separate from source field operations and decoder arithmetic modulo r. Nested stage, thread isolation and allocator forwarding checks pass. Requested allocation bytes include realloc requests; they are not live or peak memory. Optimizer/toolchain behavior can change observed allocations. No timing weights or speed comparison are assigned.

== Next construction
Add a monic/linear remainder path and reduce quotient temporaries while retaining the product/remainder tree. Preserve the full original cohort for later validation. Integer work, retained memory, coefficient synthesis and isolated timing remain separate accounting obligations.

== Provenance
Exact transformation map, frozen hashes, compiler, commands and logs accompany receipt.json. Two packaging-only generator failures are preserved; the sole native build/test attempt passed. See DERIVATION.md and the #link("https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html")[Rust GlobalAlloc contract] for definitions and limits.
