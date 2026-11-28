#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Source quotient trace: a precise measurement boundary
T-1843 · 2026-11-07 · Instrument construction and synthetic validation

Implemented an opt-in trace for the weighted quotient kernel inside the public-point source over Fp. Code inspection separates this from Modes' polynomial kernel over scalar modulus r. Keeping those layers separate is necessary before attributing source costs or selecting further optimizations.

#image("flow.svg",width:100%)

*Recorded event.* Source modulus, quotient length, divisor degree, full relevant support size and weighted support cost, actual branch, quotient-only field products and inversion entries, and success. Branch markers are inside the implementation. Counter snapshots exclude remainder multiplication. Audit support scans and trace allocation are overhead; no elapsed-time interpretation is made.

*Integrity.* Same-thread begin/finish, no nested capture, checked counters, at most 100,000 records per target, explicit invalid status on drops or overflow. Normal build excludes trace module and exports. Public collector inputs remain points; scalar indices are only target-generation/reference labels.

#table(columns:(2fr,1fr,2fr),inset:4pt,
 [Software check],[Coverage],[Outcome],
 [Native synthetic cases],[120],[Reference equality; metadata; routes],
 [Exact sparse counts / A/A],[Accepted synthetic paths],[Pass],
 [Record cap / overflow],[Forced failures],[Detected; capture invalid],
 [Thread isolation / lifecycle],[Same-thread controls],[Pass],
 [Parser fabricated panel],[617 target headers],[Accepted as synthetic test only],
 [Parser malformed panels],[6 mutations],[All rejected],
 [Collector / normal source],[Build only],[Pass; collector not executed])

*Prospective panel.* The valid draft protocol covers 617 targets from five existing prime-field fixtures, fresh Source per target, two independent identical processes, and exact transcript equality. Proposed shared ceiling: one attempt, 120 seconds, one CPU, 1024 MiB, USD 1; zero funds reserved. It is not frozen or admitted. Published source/binary binding, canonical execution/export adapter, admitted preflight and current budget/quota evidence are required before data collection. The environment opt-in is not authorization.

*Preserved failures.* Two receipt-wrapper filename errors followed successful native builds; the synthetic binary name was corrected and the normal metadata artifact reconciled without recompilation. Original wrappers/failures remain. System jsonschema lacked the required validator; the existing virtualenv validates the corrected draft. No workload data or comparative result was produced.

*Evidence and next step.* README.md defines the boundary and prospective fixtures. Native receipts and generator pins retain provenance. PCC-20261107-7f4842 is unsigned producer evidence. Original 4,563-case cohort unchanged. Next complete the admission/export adapter, then collect the frozen panel when its existing execution requirements are satisfied. Sources: exact predecessor and recovery modules, pinned protocol authority, FLINT series division; KB unavailable.
