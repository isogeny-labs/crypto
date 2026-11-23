# Bounded leased affine consequences, v1

Frozen before implementation, 17 November 2026. This continues the user's
Curve25519 index-calculus goal with fixed native development controls. It does
not replace the original growing-family, yield, selection, recovery, or complete
cold-cost obligations in ROUTE_CRITERIA_V3.md.

## Sources and competing predictions

KB MCP absent; the repository wrapper status returned127. Read the repository
branch-affine-v1/{PROTOCOL.md,NEXT.md}, native source/checker and fixed controls,
and Soos/Nohl/Castelluccia, *Extending SAT Solvers to Cryptographic Problems*,
section3.2 pp248–249, https://www.msoos.org/wordpress/wp-content/uploads/2011/03/Extending_SAT_2009.pdf.
That paper's quiescent linear reasoning, explanations and restored state are
prior art over GF2. No novelty is claimed for Gaussian elimination or leasing
conditional explanations. Its depth choices do not justify tuning our controls.

H1: shared original constraints and a bounded connected slice of already-linear
residuals avoid repeated whole-System/dependency reconstruction while retaining
checked useful propagation. H0: the restricted slice is redundant or overhead
still exhausts the old budgets. Preserve every outcome, including regressions.

## Construction and finite schedule

New isolated Rust crate; retain the old worklist source and every earlier source
snapshot/evidence byte. Disabled mode must match old v1 answers, full decisions,
counts and schedule on the same33 systems. Query inputs remain public System,
semantic branch indices, mode and finite limits; no planted values or replay
tables in the query.

The original System and original dependency/cache arrays remain shared and
unchanged. At propagation quiescence, before a nonunique branch or Unsupported,
specialize original equations under current assignments. Keep only affine
residuals. Choose a connected slice deterministically from the lowest equation
index: at most32 rows,64 unknown variables and256 coefficient entries, with
remaining rows ignored as an explicitly incomplete slice. Do not infer absence
from omitted rows. Row order, pivot order and root order are ascending.

Sparse forward Gaussian operations carry original-equation weights. Only unit
rows or nonzero constants can affect the search. A separate direct BigUint
checker recomputes weighted original equations under the exact replayed premise,
checks coefficients and bindings, and validates each propagated unit. Component
leases are dropped before Undo, with an exact LIFO trail. No copied original
System, appended original-prefix copy, dependency rebuild, or persistent
incremental-basis claim. Recompute only the bounded slice at each new quiescence.

Keep old4096vars/8192equations/32768terms/degree2/1024nodes/depth128,
10million diagnostic events and64MiB cumulative modeled reservations unchanged.
Additional finite1024pass/65536trace guards remain. Component and checker field
arithmetic, scans, row/weight storage and trace reservations are recorded and
charged before admission; retain explicit guard refusals. Model bytes are not
RSS or complete allocator accounting. Whole cold parsing, serialization,
original/group replay, allocator metadata and costs outside the new component
remain separate open accounting obligations. No speed, complete-cost or scaling
claim follows from these development counters.

## Controls and acceptance

Copy the same28 sealed public requests byte for byte:13 fixed p101 image targets
times original/indexed, plus actual-q1 original/indexed TWO-column fixtures.
Keep every old root order/role/solver limit; do not change caps from outcomes.
The actual fixture is not the original huge q213156431 base. No solver call is
made on the huge original systems. Preserve five same Fp29 truth domains,
including conditional propagation, rollback contradiction, protected-input and
nonlinear Unsupported. Independent original/root and native indexed-row/group
replay remain required. Enumerate their bounded original truth sets in Rust.

Mutation controls cover premise, coefficient, origin index, unit application,
omitted Drop, wrong Undo, duplicate assignment and bad root; malformed public
requests and forced resource caps must refuse. Caps/Unsupported are incomplete
outcomes, never absence certificates. First gate: exact disabled identity,
checked conditional arithmetic/lifetimes, all33 outcomes retained and no lost
baseline completed root. Pruning requires a separately labelled per-input node
reduction on a completed curve solve; synthetic reductions stay separate.

## Custody and publication

Use native Rust controls, separate immutable copied executable/qualifier and
distinct build/pre-call/post-call manifests. Retain first source/build/control
failures and raw outputs. Follow actual artifact policy with genuine receipts;
no policy/exemption or frozen-history edits. Extend the existing regular lane
PR4475 if still open. Restore the SAME paused CI checker after publication;
that checker runs no research. Reports include editable diagram, recorded-value
graph and inspected PDF. All producer standing remains unsigned/unverified.
