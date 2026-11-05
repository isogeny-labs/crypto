# Analysis boundary correction review

Accepted for isolated compile/native controls, with original whole-path gates still closed. Reviewed immutable sat-native-analysis-retention-correction-20261105 manifest 2afb4b9321eee7a7d9f7ad4082992cbc3b6a5910f707c1a353570154037ee459. Its exact base equals the independently accepted XOR test correction's proposed source. External apply-check passed. No builds or native tests performed.

analyze_for_search now preserves an existing analysis-owned partial clause when analyze returns its empty refusal sentinel; it stores a nonempty returned output only if no pending clause already exists. The real solve path uses that same helper and returns Unknown on refusal. Legacy successful analysis returns the same tuple. This closes the source-level pending-clause overwrite identified in the initial review.

New test builds a real two-level watched CNF conflict, searches at most64 admission checkpoints for actual nonempty partial output, compares direct analysis with the exact helper called by solve, and requires sticky Work/Unknown without losing the pending clause. It does not inject a fake pending clause. Private fixture trail setup is scaffolding, not a claim of a fully admitted public solve; whole-path observed/XOR/theory/conflict entry gates remain unchanged.

Native test success remains required. Original deadline pinning, unmetered API restrictions, finite isolated validation and separate reviewed production opening remain as specified in REVIEW.md. With both corrections, the proposed targeted suite contains13 new native tests.
