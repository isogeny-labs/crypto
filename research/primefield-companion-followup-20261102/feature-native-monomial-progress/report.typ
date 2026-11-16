#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Exact observed progress and failure flow
2026-11-07 · Development checks; full comparison not launched

The matched producer now emits progress markers before and after each public-target call, before each arm and after teardown. Markers occur outside accounting scopes. The arithmetic implementations and raw success format are unchanged; output overhead is not claimed wall-time neutral. [pcc:unverified]

#image("flow.svg", width: 100%)

== Evidence and invariant
The selected native test passes with414 inherited tests filtered. Its two-target trace has18 markers, six observed starts and six verified returns across three arms. Four synthetic methods pass and cover all128 prefixes of the full127-marker schedule, partial final markers, reordered/duplicate/wrong-arm events and receipt mismatches. The release binary compiles; describe mode executes no kernel. [pcc:unverified]

The parser accepts only an exact prefix of the fixed schedule. It records started queries, forward-verified returns, incomplete calls, targets not yet observed as started and completed arm teardown. A verified return does not imply a completed comparison or complete fixed-cost accounting. Every failure record has complete_measurement_claim=false. Binary/protocol and captured stdout/stderr hashes must match; the CLI accepts only the pinned progress-enabled binary and refuses successful processes.

== Export boundary
The preserved canonical schema requires at least one attempt for every row, and its linter expects all planned cells. It cannot honestly encode unstarted target cells. This component writes a separate monomial.failure-flow/v1 custody record instead of inventing attempts. Canonical integration remains open. A missing process receipt after host/recorder loss requires external process or Job reconciliation; partial output alone is not terminal evidence.

== Next work and scope
The new release digest must replace the earlier instrument binding in an amended draft before freeze. Preflight, canonical handling of unstarted cells, exact launch publication and authoritative remaining budget/live capacity remain outstanding. No full20-target comparison or cloud action. Original4,563-case cohort unchanged.

Sources: matched producer, bounded capture wrapper, preserved export schema/linter. Logs, binary/source hashes, parsed native trace and synthetic tests are retained. Claim PCC-20261107-b96ce1 remains unsigned (local verdict not_covered). No native failure, mathematical novelty or performance conclusion.
