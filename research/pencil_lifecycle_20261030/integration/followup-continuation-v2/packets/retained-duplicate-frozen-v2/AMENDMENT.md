# Additive prospective correction v2

Preserves sealed v1. No native compilation or execution. Input archive,420-case index,labels,source adapter,120second cap and target policy remain byte-identical. Only archival wrapper aggregation, explicit setup row metadata and protocol/report text change.

1. Nested adapter invariant_error,candidate_rejected,noncanonical_label now enter failed_dispositions with physical row/case identity, top-level and nested status. any_failed_dispositions and failed_disposition_count reflect them. There are no expected-negative controls in this archival arm. audit_pass requires completeness and no failures/accounting errors; wrapper exits nonzero otherwise.
2. Repeated setup_checks_field/constructor_field are explicitly nonadditive metadata. Analysis builds one per_target_ledger keyed physical archive_row, checks equality of every repeated copy and aggregates that ledger once. Inconsistent/malformed copies make totals invalid and audit_pass false; retained rows are untouched. Initialization failures without counters remain failures rather than invented zero-cost entries.

Source-only review required before native execution. Python syntax parsed; no helper, Cargo or archival run executed.
