#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Canonical resultant protocol integration
The real prospective protocol now validates against a pinned upstream canonical tool and schemas. It remains draft. The capture and success-export adapters have passed source-bound synthetic checks; no native panel or cloud job was executed.
#image("flow.svg", width: 100%)
== Checked behavior
Six synthetic test methods cover draft refusal, absent-preflight rejection,18 admission mutations, a complete240-row synthetic export validated with upstream file checks, bad/incomplete input rejection, exclusive outputs, check-only behavior and mocked failed/timeout/launch-error capture. validation-02/receipt.json pins the source and logs. Producer software evidence only.
== Admission and failure semantics
The wrapper is neither a cloud submitter nor an authorization service. External admission assertions must be authentic and independently verified. CPU/RAM caps remain the Job/operator's responsibility. One named attempt directory prevents reuse under the same admission. A missing receipt after host loss invalidates the attempt. Partial attempts retain failure receipts and rows; no unexecuted cell is invented to satisfy canonical export cardinality. Failure-flow integration remains pending.
== Sources and corrections
Canonical authority: crypto-autoresearcher commit41bfaed260093c69da299cbad936205530909b6f; unmodified bytes and hashes retained. Prior panel/analyzer and admission records reviewed. KB unavailable. Corrected schema order enum, analysis file metadata and CPU metadata collection; initial failures are described in README. No research result or timing comparison follows from these software checks.
