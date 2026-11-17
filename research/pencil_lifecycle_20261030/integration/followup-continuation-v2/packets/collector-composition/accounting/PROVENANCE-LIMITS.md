# Root review correction: provenance checks remain partial

Root's independent inspection identified three limits in the current checker:

1. The expected index contributes only `(archive_row, case_index)` identifiers. Its workload, seed and other identity fields are not compared with the archive.
2. Observed identity fields are compared when present/non-null. Missing or null required identity fields are currently skipped, not rejected.
3. The historical execution receipt's archive/input provenance is recorded but not authenticated against the archive hash consumed by this checker.

The passing result therefore establishes unique disposition coverage, consistency of supplied observed identity fields, and once-per-physical-row setup accounting. It does not establish complete index-to-archive-to-receipt provenance authentication. Existing JSON and receipts remain unchanged as evidence of the exact checked version. The four negative controls cover their explicit mutations; they do not cover omitted identity fields or swapped input provenance.

Next correction: define required fields from the actual expected-index, output and receipt schemas; fail closed for omitted required identities, compare expected-index identities to archive and output, and verify receipt-manifest hashes through the original pre-execution manifest. No new experiments are needed for this file-level work.
