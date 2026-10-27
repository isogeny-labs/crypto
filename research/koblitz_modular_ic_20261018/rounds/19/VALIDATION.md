# Round19 validation and formal limits

The full final Lean project build passes (9043 jobs). The transitive audit
of every local project module, including KoblitzRound19OrbitCover, is in
runs/lean-full-audit.log:732 theorems, zero failing and zero trusted.
ledger.json records this checked count separately from the79 new names.
The native recorder requires zero failing and zero trusted results and
checks that all79 new named results occur in that audit. Only propext,
Classical.choice and Quot.sound are allowed; no sorry or custom axiom is
accepted. No independent-host or lean4checker validation is claimed.

The earlier full proof-gate invocation includes the cache, build and audit
receipts for its then-current modules. G24 and G25 were added afterwards.
The final equivalent build plus AuditAxioms invocation enumerates all
compiled project modules explicitly. Its first audit process was lost
across a runtime migration; its empty log is retained. The restarted final
audit, rather than the earlier720-result audit, is used by the recorder.
The first restart failed while opening an existing mathlib .ir.sig file
with a filesystem "Disk quota exceeded" error. That diagnostic is retained
as lean-full-audit-quota-attempt-02.log. A separate copy/read of the named
176-byte file then passed, justifying a fresh audit retry. This resource
failure is not a theorem failure and is not counted as a passing audit.
After the next Spot migration, the completed732-result log was recovered;
no audit was repeated. The approved handoff and local Conductor were
reconciled first. No active EKS job or study-specific EKS job was found,
and no generator or source-side followup was relaunched. This recovery
resumes the unfinished evidence freeze, replay and PR publication only.

Root release tests pass4487 with119 ignored; native tests pass123 with1
ignored. The first root run failed an unrelated F4 cost-count threshold
(19 versus20); both the isolated control and full rerun pass. The original
failure remains recorded. Native controls compare complete RREF solution
spaces with independent brute force, test the actual elliptic identity and
conjugate, check norm determinants independently on small fields, and
exercise the three universal identity ASTs. These share some arithmetic
primitives and are not an external implementation audit.

Site assembly passes49 tests, workflow YAML4 and budget policy18. The
affected workflow has zero violations at its25-minute cap. Backlog
validation, Rust formatting, generated dashboard source/hash checks and
the pinned standalone CyclicWindow Lean control pass. Receipt names and
commands are mapped in runs/COMMANDS.md.

The preflight process produced all15 JSON outputs and seven generated
Lean modules. Their bytes match the final expected artifacts. Its stdout
receipt was lost across the migration; preflight-recovery.log records the
subsequent comparison rather than inventing an original exit receipt.
The post-freeze fresh extraction and full REPLAY.sh run provide a separate
complete receipt, including three source-bound IDC checks with128 extra
parsed checks each. Both deterministic archives require every member
hash, fresh native manifests and byte-identical repacking. Actual archive
counts, sizes and completed receipts are in ARCHIVE_VALIDATION.txt.
The first post-migration replay stopped before arithmetic because cargo
was absent from the shell PATH. Its log and copied inputs are retained
in validation-runs/replay-attempt-01. The installed1.99.1 toolchain is
unchanged; the fresh replay adds its existing bin directory to PATH.

Failed Rust and Lean sources, elaboration failures, interrupted duplicate
proofs, wrong-polarization sections, bounded null searches and the initial
ordinary-only companion accounting are retained. The final auxiliary
bundle uses the actual degree57 companion. Canonical matching is not
inferred from equality of polarization matrices.

PROOF_MAP.md separates kernel-checked identities and finite certificates
from native search exhaustion, actual norm substitutions, field/group
semantics and written divisor/descent geometry. In particular the actual
native norm matrix is not connected to its evaluation vector in Lean.
The G25 point evaluations and48 kernel noninvariance residues are native;
the exact Gram matrix and its binding to C are kernel checked.

Open requirements remain: proper normalization and explicit pole divisors,
the integral factor inclusion and divisor evaluator, canonical symmetric
Pic2 matching and full weighted theta/kernel actions,243 quotient sections
and proper equations, a complete simultaneous Q32 source arc or separator,
and whole quotient saturation. These are construction/formalization gaps,
not conclusions of nonexistence. No IC efficiency assessment is supplied.
Parent-head CI evidence is identified as such; the updated PR head needs
its own successful applicable checks before merging.
