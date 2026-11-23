# Native tracer development protocol

Question: can an exact native trace distinguish a redundant consequence that
lowers a bounded closure threshold from one that only has low polynomial degree?
Frozen inputs: `fixtures/system-{2147483647,9007199254740881}.json`, copied
unchanged from the historical prototype. They are the two-variable Semaev f3
systems described in README.md. No new input selection or sampling.

Development acceptance: exact polynomial/provenance identities; independent
Buchberger completion checks on the native reference; regression agreement
with the stored historical threshold values on both fixed inputs; incomplete
caps never become finite answers. Observe matrix dimensions, ranks, degree
falls, reference-basis containment, certificate degrees and top-form Hilbert
functions. Retain per-candidate and leave-one-out profiles.

Resource bounds: 5,000 ambient columns, degree cap 8, eight candidates,
2,000 generated sources and 200,000 submitted rows per closure computation;
reference Buchberger capped at 128 basis elements and 10,000 pairs. Budget
exhaustion is an incomplete computation. No timed scientific campaign,
external-solver speed comparison, benchmark promotion, or cloud spend.
Local builds and development tests are the scope of this PR.

Discovery is the baseline closure and is charged separately. Downstream
selection cannot erase its maximum degree. Oracle construction and membership
are separately identified, not supplied as derived rows. No runtime speedup
is inferred from degree or row-count differences [pcc:unverified]. Candidate selection uses a
lexicographic objective: closure threshold, witnessed top-form d_reg, submitted
rows, submitted-row nonzeros. Missing values sort last.

Historical Python outputs are unchanged snapshots, not native execution
receipts. Native test output is producer evidence, not signed independent PCC
validation. All numerical observations remain `[pcc:unverified]` until the
repository's independent verification process supplies a verdict.
