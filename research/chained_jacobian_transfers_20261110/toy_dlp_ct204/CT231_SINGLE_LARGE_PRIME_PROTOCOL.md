# CT231 — preregistered one-large-prime native transfer experiment

Date: 2026-11-19 UTC. Owner lane: [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715). **[pcc:unverified]**

## Research question

CT230's native prefix-base experiment at p=101 found rank 34/36 after 6,540 candidates; prefix 48 and full 54 solved the public known-answer ECDLP, but prefix 48 was slower. Test whether **one-large-prime relation pairing** changes the failure/success boundary for the same composite-order genus-4 cubic-cover transfer. This is established index-calculus machinery, not a new asymptotic algorithm. The actual Jacobian has order 115593846 and is not a cyclic prime-order target.

Sources examined: CT229/230 in this branch; [Gaudry–Thomé–Thériault–Diem, Math. Comp. 76 (2007)](https://doi.org/10.1090/S0025-5718-06-01900-4), single/double-large-prime relations and cycle elimination; [Diem (2006)](https://arxiv.org/abs/math/0606607). Their asymptotic hypotheses are not transferred to this split-Jacobian experiment. Research-KB MCP and checkout-local wrapper unavailable in this connector runtime; this is a limited repository/primary-source review.

## Freeze before collecting native observations

* Same public test: C/F101: y²=u⁹+u³+46, E/F101: y²=x³+x+46, order-109 source subgroup, G=(2,37), Q=(64,19), and CT226/229 native Rust Cantor, pullback, elimination, signed-roster and offset helpers. The known answer 37 is used **only after** solving and in separately priced direct BSGS.
* Fixed signed lexicographic-prefix base sizes **[24,36,48,54]**. For each arm use at most 60 unchanged lexicographically generated offsets × 109 source shifts = 6,540 relation candidates; stop collection only at full rank.
* For each reduced genus-4 divisor, accept only squarefree fully rational *four-root* support over the complete original 54-symbol roster. Partition signed coordinates into selected prefix B and excluded set L. Classify 0 outside roots (full), 1 outside root (single large prime) or 2–4 (discard and count). Reject nonsplit and lower-degree support as before.
* Store the first relation for each out-of-base symbol, indexed by its canonical x with signed coefficient s ∈ {−1,+1}. Every subsequent relation with the same symbol generates an exact B-only row through **s_first × current − s_current × first**. This eliminates large-prime contribution without requiring its logarithm. Feed the resulting row into the same modulus-109 Gaussian elimination as CT226 and preserve consistency assertions for dependent rows.
* On reaching rank |B|, independently verify every recovered factor-base point logarithm, then derive the logarithm for each observed single-large-prime symbol from its stored source-relation and verify against its rational elliptic norm **only after** solving. Target recovery may use either a fully B-smooth or one-large-prime support with a thus-solved LP symbol. No source DLP oracle enters relation finding, joining, LA or target recovery.
* Retain all four arms, including failures, collection/rank counts, total and phase nanoseconds, per-arm same-run BSGS, target checks, genus-4 compositions and approximate bounded LP table size. Hard cap: ≤ 54 total full roster symbols, ≤ 30 excluded LP keys, ≤ 6,540 candidates and ≤ 60×109 target trials. No paid compute, external data or S3 access.
* The full 54-symbol arm is a **control** and MUST exactly retain CT229's (rank,smooth,candidates)=(54,62,1421), source scalar 37. If it does not, the experiment fails. Successful 24/36/48 arms must verify [a]G=Q. Failed arms are not assigned a complete-solve ratio.
* Compare *complete* candidate cold wall time against direct BSGS, and 36/48 vs 54 only when successful. Remain explicit about absent optimized rho/frontier IC, peak RSS, independent CI/PCC, authenticated 695/701 replay, physical C169/C13a/C13b/N2_actual/D, and uncertainty bounds.

## Correctness invariant

Write each fully split divisor relation in F109 as r_B·ℓ_B + s·ℓ_L = t, with r_B including the **subtracted offset coordinates** and t = 3k. Given a stored (r₀,s₀,t₀) for the same LP symbol, form (s₀ r_B−s r₀)·ℓ_B=s₀ t−s t₀ (mod 109). Since s,s₀=±1, no inversion is needed. On solved ℓ_B, ℓ_L=s₀(t₀−r₀·ℓ_B) mod 109. This algebra is independent of the group order of the complementary factors because all equations are pushed into the source-order-109 component through the norm map.

## Reproducible command (after code publication)

```sh
cargo test --locked --release --test ct226_native_genus4_transferred_ic ct231_ -- --nocapture
```

Results remain provisional until source-head native CI, arithmetic checks, and independent adjudication. The prior CT227/230 negative results and S3/budget STOP remain in force.
