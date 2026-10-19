# F4 early-stop study, recovered 2026-10-19

Recovered L4 implementation and historical raw records. Fresh full release-library validation: **4,400 passed, zero failed, 118 ignored**. See [the validation report](resume-validation-20261019/README.md), [PDF](resume-validation-20261019/REPORT.pdf), and original [run commands](run.sh).

The original `aggregate.sh` is preserved as a historical placeholder and now exits nonzero so it cannot report a false successful aggregation. Use `run.sh` for a fresh run; integration review and performance-comparison gates remain open, so this PR stays draft. The saved full-library validation predates the opt-in diagnostic adjustment and must be rerun on the current head.
