# Small-prime backend and algebraic endpoint control

This extends the fixed-F101 chain snapshot at aabae1979457c3ba9e5112487a907b50cab15a99. All claims remain unsigned/unverified. No benchmark or paid experiment has run. [pcc:unverified]

The opt-in native `small-prime` feature uses the unchanged repository Fp2 context for checked odd primes≤4096. Exhaustive enumeration is capped at101. Field/curve/chart tags reject mixed domains; invalid moduli, singular/degenerate models, noncanonical points and unsupported quotients fail closed. The feature imports `src/prime_hyperelliptic/fp2.rs` outside its Cargo package; checkout and source guards must include it. Runtime inputs are frozen in `small-prime-inputs.sha256`. Field arithmetic reuse is disclosed; it is not independently validated by reusing that layer. [pcc:unverified]

| p | Nonresidue | Each Jacobian order | Rational2-torsion source/target | Source elliptic order | Preserved r; generator |
|---|---|---|---|---|---|
|13|2|180|4/4|10|5; (10,6)|
|29|2|960|4/16|40|5; (24,15)|
|31|3|1008|16/4|28|7; (19,2)|
|101|2|9216|4/16|96|3; (48,20)|

The first three fields were fixed before execution;31 was separately declared as a nonresidue-variation control. All source/target classes, both dual norms, equal fibers and odd-subgroup scalar relations pass. Both F101 hops agree exhaustively with the previous model backend. The final suite passes25 tests and strict Clippy; the root default example import check passes. Raw receipts: `small-prime-final-01.txt`, `small-prime-clippy-final-01.txt`, `root-feature-import-check-01.txt`; claim PCC-20261111-cd7c2f. Equal2-torsion counts at13 do not establish an isomorphism. [pcc:unverified]

The unchanged cryptanalysis BigUint reference agrees on all eight complete class sets, including independent Fp/Fp² orders, self-inverse counts, paired additions and dual norms. Its chart adapter uses BigUint directly. It independently counts the two elliptic quotients as10/18,40/24,28/36,96/96. Its quadratic-field counter shares the existing algorithm lineage, so this does not independently validate that layer or provide signed standing. Development receipt: cryptanalysis `experiments/chained-transfer-reference/evidence/small-prime-development-01.txt`, PCC-20261111-db1613. Immutable feature pin/replay remains separate. [pcc:unverified]

[GLOBAL_SUCCESSOR_COLLAPSE.md](GLOBAL_SUCCESSOR_COLLAPSE.md) supplies a written argument over every field of characteristic different from2,3,5:

    n R f* = tau phi [2], tau(X,V)=(X/100+1/125,−V/1000),

with phi the standard2-isogeny on v²=U³+9U²+24U. The actual correspondence-base image projects to(1/125,0); its2-torsion translation is essential. Seven new cleared ring lemmas compile in `lean/SuccessorCollapse.lean`. The total is25 written lemmas, gates22+8+7 declarations, standard axioms only. The geometric completion/Picard step is a written argument using the primary sources, not a full Lean construction. Claims ec195a and6f6202 remain unsigned. Native controls cover all rational curve points, both infinities and all elliptic whole fibers at the four fields. [pcc:unverified]

This identifies the first elliptic endpoint with a direct expression. It does not identify the final Jacobian-valued two-hop map with that endpoint or establish a cost ratio. No cross-prime transfer is implied by multi-field implementation checks. Implementation support remains≤4096; the algebraic argument's scope does not extend the backend. [pcc:unverified]

Reproduce from crypto with `cargo test --release --locked --offline --features small-prime --manifest-path research/chained_jacobian_transfers_20261110/native/Cargo.toml -- --nocapture`, strict Clippy on the same feature, and the existing proof gate on all three Lean files. Check the runtime manifest with `sha256sum --check research/chained_jacobian_transfers_20261110/small-prime-inputs.sha256`.

Cryptanalysis#1469/#1471 are merged. #1473 freezes the prior two-hop source at aabae197, before this feature. Its new source pin is a later checkpoint. The crypto workflow includes the external field source and all three proof files, but still awaits default-branch registration. S3/budget/vault/KB and current crypto ledger/archived-receipt blockers remain. Owner rules require extending the existing PR and forbid choosing a ledger side; no alternate lineage or bypass was introduced.

Next: immutable feature replay, then examine the quadratic-cover restriction for elliptic full groups of odd order and validate an alternative odd-degree cover if warranted. Computational advantage and prime-field descent remain open.
