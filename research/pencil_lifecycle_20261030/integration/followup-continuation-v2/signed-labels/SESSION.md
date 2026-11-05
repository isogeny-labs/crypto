## 2026-11-02 saved label equations

### What was done
T-1386 added offline native affine Cantor reconstruction for saved candidate/restart labels, source/transfer header and torsion checks, and returned scalar verification. Read existing source and source map before edits. No collector run. Final stable-source check verified731 candidates and3 source scalars; both coherent corruptions rejected.

### Decisions made
State arithmetic sharing explicitly; no independent-arithmetic or universal-proof claim. Preserve full1260 scope, existing cost controls and runtime/prospective-review gates.

### Open threads
Independent review, positive restart/duplicate/identity coverage, exact scheduler reconstruction and remaining matched cohort.

### Errors
First attempt assertions passed but source hash check failed on two new files because formatting was still running. Preserved as unsuitable evidence; repeated after formatting completed. Fixed wrapper to propagate binding failure and bind its own files, then checked again. Summary glob initially included check-first.sh as a directory; restricted enumeration to directories without new computation. Historical root suite failures remain.

### Publication preparation
Rendered and inspected the one-page PDF and editable SVG. Degree distribution:728 degree-two and3 degree-one candidate records. Three check receipts retained, including initial source-binding failure. Scoped rustfmt passes. Dependency lock explicitly included for reproducible locked checks.

All six archived logs were freshly downloaded and matched hashes/byte lengths. No collector or scientific worker rerun; final source bindings remain unchanged.
