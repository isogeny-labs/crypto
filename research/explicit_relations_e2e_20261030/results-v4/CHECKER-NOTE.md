# Coverage and status semantics

Canonical schema, flow and file hashes pass. The protocol-aware checker
assumes a rectangular30-cell design. The frozen v4contract explicitly plans
up to24cells:6each at48/52bits,4each at56/58/60bits, stopping after the first
incomplete rung. All20prescribed cells through58bits were attempted; all16
recoveries through56bits passed, and4work-limit failures stopped the ladder.
The4potential60-bit cells were not attempted under the frozen stopping rule.
The frozen native analyzer passed that contract; canonical findings remain
verbatim in protocol-check.log and are not reported as a checker pass.

Native limited maps to schema failed, retaining native_status in counters.
The canonical checker counts a failed process as a completed process outcome,
so flow.completed is20. This does not mean20recoveries: coverage.json reports
16verified recoveries and4incomplete recoveries; raw CSV retains all failures.
