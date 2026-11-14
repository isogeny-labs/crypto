# Independent Lean review plan — square-kernel normalizer

Task: Conductor T-1829. Plan frozen before reviewer dispatch. This is a technical proof review; it changes no experimental or PCC admission status.

## Bound producer claim and inputs

Review the 24 named theorems in `CauchyNormalizer.lean` and `KernelNodes.lean` at crypto commit `a2be660914e8e485eefd45f2958eb6d18344dadf` (the theorem source last changed at `77a4f24e9c9907b9eec19ca1f612433ecc954dd8`; the fresh-replay packet was added at `2b720caa2`). The requested claims are the exact statements in `lean-normalizer/PROOF_MAP.md`: unit 2-adic valuation for the actual shifted-grid puncture ratio, preservation of source-node values and the origin identity by the normalized square kernel, and doubled valuation wherever the original kernel is nonzero. The producer explicitly leaves coefficientwise chart content, uniform mode counts, degree/recovery cost, recursion, ECDLP correctness, and code-to-Lean equivalence unproved.

SHA-256 inputs: `CauchyNormalizer.lean` `b30198a97a6564b81b68cb373999e9d49922e1e3a087455a09ae787dc150019a`; `KernelNodes.lean` `f14e3da98cb3270fb7e4b82de8d5ebe9b73584a092ccf936444047cf6e6557f9`; `PROOF_MAP.md` `09bc97d2c045fc6c84acbd3aea1638d4f5dafc56626e0406f1a95acdcf2ca70a`; `lakefile.toml` `66b7802092ddd66d46bba3f71ad572ee4cd07d3994f0d69e3fc2a3b0ff1cfb8d`; `lean-toolchain` `ca5f1f601d34c49079e38d7e7041eb0f2a9c227fbc1df0f404820069a052330c`. Pins: Lean 4.35.0, CryptoLean `148f5dc389b341401e06d76abe7783c4ccf42f2e`, Mathlib `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2` per the producer packet. These pins and source inventory will be checked against the actual manifest before any machine verdict.

## Prior and review joints

Prior, recorded before independent verdicts: the factorial/binomial identities and pointwise `w²/rho` valuation look mathematically plausible; confidence is lower for the identification of the Lean kernel and indices with the intended native construction. A producer gate receipt alone cannot establish fresh execution or semantic fidelity.

| Joint | Owner | Worked attack / required check |
| --- | --- | --- |
| Binary all-ones binomial row, signed puncture product, 2-adic unit ratio | Blind algebra reviewer | Derive independently from definitions; test the nearby false row `choose(2,1)=2`, and an out-of-range index such as `a=1,k=4`. |
| Shifted-grid kernel, node and origin identities, finite-index assumptions | Source-fidelity reviewer | Compare elaborated theorem types to `PROOF_MAP.md`, original source claim, and native kernel formula; seek an index/shift or totalized-division mismatch, including `a=0`, center, noncenter, and out-of-range cases. |
| Machine build, named declarations, transitive axioms, verifier/CI coverage | Coordinator | Rebuild the exact two modules under pinned dependencies in an isolated input tree; audit all named and generated declarations and reject holes, custom axioms, `unsafe`, native evaluation, stale inputs, or omitted source. A receipt consistency check alone is insufficient. |
| Claim-to-experiment bridge | Coordinator with source-fidelity reviewer | Identify each unproved link to coefficientwise content, counts, degree, recovery, native execution, and costs. No proof of an algebraic lemma can close these by inference. |

## Blindness and controls

The blind algebra reviewer receives only this plan's mathematical definitions and target statements: for `N=2^(a+1)`, center `c=2^a`, `P(k)=∏_{0≤j<N,j≠k}(k-j)`, `rho(k)=P(k)/P(c)`, `B_k(t)=∏_{0≤j<N,j≠k}(t-(j-c))/P(c)`, and `S_k(t)=B_k(t)^2/rho(k)`. It must derive `v₂(rho(k))=0`, `S_k(m-c)=B_k(m-c)` for valid `k,m`, and the origin Kronecker identity, or give a counterexample. It may read no producer Lean source, proof map, reports, receipts, sibling report, or Git history until its report is frozen. Its permitted inputs are this section and standard mathematics only; it must attest to sources actually read.

The source-fidelity reviewer may read the immutable producer source and source claim, but not the blind reviewer's report or private reasoning. It must not treat a successful Lean build as semantic proof. Each reviewer writes a separate report and `review_attestation` with joint ownership and actual reads. The Coordinator reconciles reports only after both are frozen; disagreements remain visible.

Positive controls: small grids `a=0,1,2`, center and noncenter nodes, exact puncture products and valuation. Negative controls: `choose(2,1)`, an index outside `k<N`, artificially zero normalization, and a nonzero point away from source nodes to distinguish pointwise node preservation from polynomial equality. Record whether each is a mathematical counterexample, an expected theorem precondition failure, or an infrastructure failure.

## Evidence and outcome rules

Fresh logs, input hashes, exact theorem types, source coverage and axiom output go in this review directory. Record machine status, semantic fidelity, and review outcome separately for every cited claim. `validated_scoped` requires fresh machine verification, independent faithful correspondence, satisfiable hypotheses, controls, and complete attestations. Until then the outcome is `inconclusive` or `needs_revision`. The final packet will include an inspected diagram/PDF, unresolved bridges, and a PR; no PCC signature or official ledger transition is implied.
