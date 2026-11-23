# Producer wire clarifications and checker scope

The independent design `5610dacb0ae` was frozen before checker implementation.
Producer clarification `149f2a9` and implementation `1e22241` specify the
following v1 details. They do not change the record fields or admit a new
experiment.

- Every selected pivot emits a swap, including a self-swap. Versions belong
  to slots. A self-swap increments its single slot once.
- Column c is the low-to-high bit c%64 in word floor(c/64). Every XOR event
  supplies the complete resulting row, and its named native counter includes
  every executed suffix word, including zeros.
- `invalid_input`, `unsupported_profile`, `sink_failure` and `integrity`
  require exactly `kind` and a detail string of at most 256 UTF-8 bytes. Other
  reasons contain exactly `kind`.
- Native row/column caps are captured caller/environment observations, not
  allocation sizes. Values can exceed the checker profile, including default
  20000/40000; actual reconstructed matrices still obey the independent
  4096-row/163-column caps and the supplied native caps.
- On a complete trace only, the checker recomputes the named admission sum
  in the producer's `ADMISSION.md`: input scans, conservative candidate work,
  support collection for more than one row, packing, pivot searches, swaps,
  clearing tests, suffix XORs and output words. This validates the formula
  against the supplied complete schedule, not actual instructions, allocation
  overhead or total producer execution. Censored traces carry no such total.

Initial implementation `52422341` passed nine standalone tests. The initial
test build warned about borrowing a constant as a mutable temporary; the
first strict lint run rejected a large enum variant. Follow-up `ed883f6` uses
an explicit test counter and boxes only the begin binding without changing
the wire representation. Its nine tests and strict all-target Clippy pass.
The dependency lockfile is tracked. Failed diagnostics remain in the local
evidence packet and are archived alongside successful runs.

Those tests include independently hand-calculated complete systems, all their
censored prefixes, Boolean cancellation, combination ordering, rehashed
arithmetic/version/padding/claim tampering, JSON aliases/duplicates/unknown
fields, parser/resource limits and exhaustive rank-versus-span checks for all
4096 binary 4-by-3 matrices. They do not establish compatibility with the
separately compiled native producer. Its actual five fixture streams and
boundary cases remain required before an interoperability claim.

The CLI is `boolean-matrix-trace-check TRACE.jsonl EXPECTED_BINDING.json`.
Expected binding equality checks caller-supplied identifiers, not their
truth. Output explicitly retains false execution, sink-durability, total-cost,
encoding and UNSAT/Gröbner claims, with unknown first/last fall degrees. Raw
records and derived ranks stay restricted provenance/targets, outside public
static ML features.
