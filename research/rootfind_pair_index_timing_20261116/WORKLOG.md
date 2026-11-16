# Work log

## 2026-11-16, protocol freeze

### What was done

- Read the previous K13/K19/K23 first-witness and storage-parity records,
  the binary pair-table source and the program paper. KB revision
  `9141e386254340a9bdba3036f2bb706a` predates the current prototypes.
- Checked the live correct-account EKS budgets: monthly USD 2,094.914
  against 1,300 and daily USD 216.868 against 200. The frozen L2 panel is
  not admitted at those readbacks.
- Conductor T-6708 still reserves both production collector source files.
  T-6814 claims this separate study, test and resume paths. [pcc:unverified]

### Decisions made

- Measure a direct-build table component on actual Koblitz pair/probe
  streams, with hash A/A and every correctness check. This is a local L0
  selection screen for later production integration, not the F4/F5/F6
  complete-call panel. [pcc:unverified]

### Open threads

- Implement and commit the standalone harness, execute once with the
  checkpoint gate, retain raw rows, and publish the visual run report.

### Errors

- The frozen draft described a default `u32` hash reference. A source read
  before implementation showed that production uses `u64` keys, a custom
  `MixHasher`, and capacity `a*a+1`. Pre-source Amendment 1 corrects the
  control without changing inputs or timing boundaries. [pcc:unverified]
