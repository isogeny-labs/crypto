# Actual final control launch

The one recorded final Rust identity-controls call was launched by a Python
subprocess wrapper that opened the separate stdout/stderr paths. This was a
procedural departure from the repository's shell-only thin run-orchestration
rule. The implementation and all identity arithmetic ran in the retained
Rust executable; Python did no field/polynomial evaluation. The actual launch
is disclosed rather than relabelled as the shell reproduction below. No
unchanged native rerun was made to improve the procedural record.

The final deliverable contains the Rust implementation and a shell-only
REPRODUCE.sh. Source/binary pre-call and post-call checks, exact stdout/stderr
and exit status are retained. These remain unsigned producer development
controls, not admitted benchmarking or signed execution evidence.
