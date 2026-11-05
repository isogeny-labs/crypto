# Source-order mutable signed-graph find preservation

2026-11-01. Two main theorems, eight controls and ten named axiom readbacks pass. This construction models actual parent/potential updates, rather than another affine identity alone. No scientific execution.

## Model and checked invariants

State contains finite-node parent and potential functions. Represents means every node value equals its signed affine parent expression at fixed target k. Forest means each parent either is the node itself or has strictly smaller rank, for an immutable Nat rank witness. This is an input acyclicity witness, not an assumption that find already preserves its output.

Run is an inductive successful-execution relation matching native find order: read parent; if root return identity; otherwise recurse on the parent, producing mutated state t; read t.pot[x]; compose with returned potential; update t.parent[x] to root and t.pot[x] to composed potential. The post-recursion potential read is deliberate and matches native source. It is not silently replaced by a pre-recursion read.

run_preserves proves simultaneously: final Forest, final Represents for EVERY node, returned root self-parent, original queried value equals returned potential evaluated at root, root rank no greater than queried rank, and all nodes of greater rank retain their parent/potential fields. The recursive frame applies to child x because its parent rank is strictly lower. Thus recursive compression cannot change x before the source's post-recursion read. The local update then preserves the queried node equation using affine composition; every other node keeps its local equation. The root is distinct from the compressed child by strict rank, so its self-parent property is retained. This closes the mutable frame obligation rather than assuming it.

run_exists separately derives a finite successful Run for every node in a rank-decreasing forest by strong induction on rank. It supplies model totality without assuming an execution trace. The result is an operational relation over finite functional heaps, not a compiled Rust evaluator or a measured recursion bound.

## Controls

A three-node chain2->1->0 with rank0,1,2 and two negative signed edges executes the actual two-level Run constructors. At k=2, root value7 gives node1 value1 and node2 value12. After compression both parent links are0, the composed potential evaluates to12, every node equation remains valid and root0 remains fixed. A negative control resetting all potentials to identity while retaining the original parent graph violates representation. Eight controls and both theorem readbacks pass. First two proof attempts failed only in induction constructor binder/induction-hypothesis handling; exact failed files/logs/receipts remain.

## Exact source and remaining obligations

Source three_leads.rs find1580-1596 reads parent, recursively finds parent, then reads potential[x] and writes both fields, as modeled. Array indices are finite typed nodes; native Vec bounds, actual allocation and i8/word interpretations are not modeled here. Arithmetic is an arbitrary commutative ring using the previous affine composition theorem. Native Ring bridge, sign closure, counters and compiled execution remain separate.

The proof assumes an initial rank-decreasing parent forest and node-value representation. Showing node creation and union updates establish/preserve those premises is next; this packet does not prove union mutation or fixed-anchor metadata preservation. The finite Run existence theorem does not claim native recursion-stack capacity or a runtime bound. Independent source-fidelity review of this new packet is pending.

## Provenance

Source-first prospective private plan, fresh Lean checks and matching before/after local source/helper bindings. Copied imported GraphAlgebra helpers and cached Mathlib retain the stated trust boundary. No sibling review or full independent review is claimed. Primary references: native SignedGraph find and prior compression identity; standard decreasing-rank forest induction. No scientific run, cohort reduction, cost result or repository mutation.
