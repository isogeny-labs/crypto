# Prime-field polynomial-calculus closure with source certificates

The degree-4 backend completes the preserved natural norm system and recovers
the cold control's discrete logarithm through the existing index-calculus
pipeline. Every accepted basis has a local derivation DAG, checked by a
separate native executable before quotient extraction. Independent OpenSSL
replay accepts the cold queries, signed relations, kernel and final scalar.
PCC-20261115-be28ec remains unverified. [pcc:unverified]

The implementation uses standard polynomial calculus / MutantXL closure over
the actual prime field. Newly independent low-degree pivots are multiplied by
each variable and reduced by scalar row operations. A PC-DAG-CERT-1 proof
records original equations, variable products and linear combinations. The
checker reconstructs every referenced polynomial, checks intermediate degree,
then requires Buchberger completion and zero remainders of original equations.
Producer and checker share canonical arithmetic/order primitives; they do not
share elimination. This is native symbolic verification, not a formal proof of
either program or a signed external PCC verdict. [pcc:unverified]

| Fixed control | Outcome |
|---|---|
| Six small ideals | Exact rational sets and quotient dimensions match |
| Four adverse certificates | Reject at their intended check |
| Natural norm, degree4 | Complete source-ideal GB; one point, quotient1 |
| Same norm, degree5 | Outer-cap exit137; unknown |
| Same norm, degree6 | Reduction-step cap, exit2; unknown |
| First cold target Q=(0,6) | Scalar7;9 queries,4 relations,1 kernel |
| Independent OpenSSL | Cold replay passes; old18-target replay also passes |

These are fixed software correctness controls. There is no timing comparison,
random-target yield estimate, fitted exponent, speedup or claim-tier dataset.
Higher caps need not be faster under a fixed schedule and resource budget.
Unknowns do not establish certificate-degree lower bounds. [pcc:unverified]

The natural degree4 trace has2861 attempted rows,1000 pivots,184708 reduction
steps,3861 generated nodes and1692 retained proof nodes (165262 bytes). These
are recorded native counters, not an independently certified matrix-rank result.
The proof bounds intermediate degree; expanded input multipliers may have
higher degree than4. The earlier flat-Macaulay panels are preserved separately.
[pcc:unverified]

Build and repeat the software panel in a new directory:

```sh
research/norm_pc_closure_20261115/build.sh /path/to/msolve-native /tmp/pc-build
research/norm_pc_closure_20261115/qualify.sh /tmp/pc-build /tmp/pc-panel-new
research/norm_pc_closure_20261115/build-cold.sh /path/to/msolve-native /tmp/pc-build
research/norm_pc_closure_20261115/qualify-cold.sh /tmp/pc-build /tmp/pc-cold-new
```

Use the parent's pinned FLINT3.1.3 installation (bootstrap documented in
shared_norm_algebraic_20261115). All research computation is C++17, compiled
with strict warnings. The wrappers only build/run/capture native executables.
Fresh paths prevent evidence overwrite. Bounds are in PROTOCOL.md; no cap was
raised or failed solver retried. Producer/verifier source bf6d29e42d is unchanged
for cold recovery; its driver/guard/replayer source is b31a2ca3a9.

Each raw round has a deterministic `runs.tar.xz` under evidence/. Check its
SHA256 file in that directory, then extract there with `tar -xJf runs.tar.xz`.
The whole qualification archive is40784 bytes and committed directly. Cold002
is283208 bytes: use `python3 tools/artifacts/artifacts.py pull
research/norm_pc_closure_20261115/evidence/cold-002` to hydrate its manifest.
Local AWS CLI is absent; the documented same-repository artifact-autofix
[run43690919442](https://github.com/isogeny-labs/crypto/actions/runs/43690919442)
uploaded and downloaded the bytes successfully. Bot commit dcc5c0c677 retains
the manifest, ignore rules and SHA256
629341e03334c65633c3fb68fe55d06dad7487b1ea5786948504f11e5c76ee4f.
Local hydrated verification also passes. No archive is split.
[pcc:unverified]

Qualification hashes: the runner manifest was overwritten post-run by the
unchanged pre-call build digest list in another order. That build manifest,
an explicitly labelled post-run reconstruction and a custody note are retained.
Every native input/output/status/certificate stays unchanged. Cold002 retains
separate pre-call/build manifests, the initial failed compilation/setup logs,
and the successful parent replay regression. Initial cold001 ran no solver.

Sources checked (KB unavailable): repository wrapper was unavailable in the
preceding continuation; no research-kb tools are loaded. Read existing
`src/cryptanalysis/{inherited_f4,degree_reduction}.rs`, the prior MutantXL
preregistration, the fall-degree/literature notes, and the parent's selective
oracle and initial-elimination contracts. Boolean monomial arithmetic was not
reused for prime fields. The cited MutantXL/HKY primary papers were not newly
retrieved in this component; no novelty or exhaustive review is claimed.

The conditional per-attempt envelope is in COST.md. The explicit source still
admits only p<2^31,20 variables and degree<=6, with small quotient extraction
bounds. A uniform sufficient degree, useful fixed-base yield, arbitrary-prime
implementation and all cold costs on a growing admissible family remain open.
The original complete subexponential-algorithm criterion remains active.
[pcc:unverified]
