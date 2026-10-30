# Three ECDLP follow-ons: checked components

Status: **native software conformance and conditional algebraic proofs checked;
full priced pilots incomplete**. This is an additive continuation of the
[three proposals](../reports/new-leads-20261028/REPORT.md), not a claim of faster
index calculus or a complete research programme.

Read the [results report](../reports/three-lead-components-20261029/REPORT.md),
[diagram](../reports/three-lead-components-20261029/component-evidence.svg),
[PDF](../reports/three-lead-components-20261029/report.pdf), prospective
[protocol and versioned amendments](PROTOCOL.md), [proof map](proof-map.json)
and [complete final conformance records](validation/).

- Transfer: exact source multipliers certify the symmetric-to-chain bridge.
  Low-degree selection includes cancellations between rows. The prime-order
  control retains two degree-at-most-four consequences and one verified
  subgroup relation; the full query price remains unknown.
- Sharing: exact intersection and a separate pre-elimination path preserve all
  leaves. The prime-order control shares seven input rows before leaf
  elimination; its three refutations and one positive leaf are unchanged.
  Compilation, certificate and solve costs have not been compared.
- Quotient: the existing binary E0 control admits the concrete map
  `Phi=[23](-1-Frob)^3`, complementary to `Psi=[23]Frob^3`.
  All 4,232 supplied-kernel points are annihilated. Fourteen normalized Vélu
  comparisons agree up to the recorded sign. A complete algebraic relation
  oracle, lifting/rank audit and full price remain outstanding.

## Reproduce the component checks

Rust 1.99.1, the committed Cargo.lock, no crypto-library dependency:

    cargo test --release --lib --locked --manifest-path research/ic_algebraic_crossover_20261023/followons_20261029/Cargo.toml -- --test-threads=1
    cargo clippy --release --locked --all-targets --manifest-path research/ic_algebraic_crossover_20261023/followons_20261029/Cargo.toml -- -D warnings

To emit complete fresh records, set `IC_THREE_LEADS_EVIDENCE_DIR` to a new
directory before the test command. Existing records are never overwritten.
The committed records came from the final six-test source, not a later
performance run. Cargo's test-runner duration is not a benchmark result.

Lean 4.34.1:

    lean -j1 research/ic_algebraic_crossover_20261023/followons_20261029/ThreeLeads.lean

Seven lemmas pass. Five use only the standard `propext`, `Classical.choice`
and `Quot.sound` foundations; the finite-kernel and subgroup-injectivity
bridges use no axioms. None uses `sorryAx`, a custom axiom or native_decide.
An external fresh `lean4checker` is unavailable; no such recheck is claimed.

The two v1 identity certificates have IDs `IDC1h02c42d4a878d05d0` and
`IDC1hf913cc1220ef209b`. Each passed its 32 derived issuer points and 64
separately seeded checker points. Adding one to either identity is rejected.
Their proof hashes bind the actual checked Lean source. This same-session
check is not external independent replay.

## Boundaries

The F32/F64/F128 controls enumerate their complete rational point sets and
check every map-homomorphism pair. The F(2^22) ambient order is the recorded
input 4,193,912, not a new cardinality proof. Both map implementations share
field/point arithmetic; external independent arithmetic/replay remains open.
All component costs, speedups and ECDLP crossover fields stay null.

The algebraic/generic track rule is explicit: the constraint machinery is
intended for the algebraic track. Enumeration, group sums and truth tables
here are correctness references, not an algebraic IC improvement.

Full pilots and the current EKS/image/adapter gap are tracked in
[the problem record](../../../problems/ic-three-lead-full-pilots.md).
No cloud job was launched, no historical process was inspected or inferred
live, and no frozen workload/cap/original deadline was changed. The original
29 October 20:02:21 UTC deadline and USD100 extra-cloud total remain intact.
The main/auto-merge authorization rejection is separate and unresolved.
