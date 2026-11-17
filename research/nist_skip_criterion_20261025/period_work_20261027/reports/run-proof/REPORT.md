# Proof checks: all attempts retained

27 October 2026. Lean4.34.1 checks eight final theorems. The unchanged repository
axiom auditor reports eight checked, zero failing and zero trusted, with empty
axiom lists. Four literal vector inequalities and the generic monotone-budget
implication are proved. Monotonicity and transitive comparison are explicit
assumptions; this does not prove anything about unrecorded historical runtime,
the original caller, curve security or an attack. Native literal-data binding
matches the independently checked table.

![Proof attempt history](visual.png)

The diagram preserves sequence and status instead of comparing cost: no
runtime or RSS was measured. Attempts001 and002 fail thread startup under a
4 GiB virtual-address cap. Attempt 003 fails decidability elaboration; unfolding
the unchanged predicate resolves it without changing vectors or statements.
Attempt 004 builds six proofs but its1GiB auditor exceeds its allocation.
Attempt 005 builds the final eight, while its auditor cannot locate external
lean on PATH. Audit 006 passes with the verified binary on command-local PATH.
Failed outputs and source snapshots are preserved, not accepted as proofs.

Final kernel checking uses one worker and1GiB internal allocation limit;
auditing uses one worker and4GiB, with no virtual-address cap or measured RSS.
Shared local host, no isolation tier, timing reference, random seed or legacy
execution. The official release bytes and published SHA match. lean4checker,
elan and lake are unavailable; exact compiler and auditor commands are retained.
This is not a full repository proof-gate or hosted CI pass. These
formalization attempts are additions after validation, not the frozen primary
source revision. Their source hashes and commands are in the evidence bundle.

[Proof map](../../proof/PROOF_MAP.json), [attempts](../../proof/attempts.json),
[commands](../../proof/commands.json), [kernel output](../../proof/kernel-build.txt),
[auditor output](../../proof/axiom-audit.txt),
[reproduction](../../proof/REPRODUCE.md). Raw archive paths
runs/proof-attempts/build001 through build005 and audit006 retain every failure.
