# Outer primality observation patch

Source baseline: 6b026f9eb1dcdfaa3951240fea68f59334b2e889.

Implementation reference: `src/cryptanalysis/three_leads.rs` functions `is_prime` and `modpow`. This patch observes that existing deterministic u64 algorithm; it makes no new primality algorithm or timing claim. Reviewed accounting references: `/home/ec2-user/pencil-accounting-review-20261029/historical-price-contract.json` and `coverage-ledger.csv`. No new experiment or workload was run.

The candidate outer source validation invokes p only after all preceding representation predicates pass, and r only after source validation passes and r>2. Both short-circuit orders remain unchanged. Rejection returns typed `ValidationError` carrying both optional observations. Null denotes a call not invoked; a missing ledger in a historical record denotes unknown, not zero. Successful setup adds `outer_primality` with schema `pencil-outer-primality/v1`. The existing setup elapsed timer begins after these checks and is not redefined by this patch. Common method-call cold timing covers the checks.

`modular_muls` and `modular_squares` are disjoint. Each includes its modular product reduction; `product_reductions` is diagnostic, not an extra charge. `standalone_modular_reductions` covers trial divisibility and witness reductions, preserving the original repeated a%p evaluation. No event is a scalar inversion. Exponentiation here must never be removed by inverse-expansion normalization. Loop and divisibility counters are explicit diagnostics, not complete native control or CPU costs. Parent analyzers must continue to fail closed on incomplete coverage.

Domains remain separate: base_modulus uses p; scalar_modulus uses r. Under the inherited grid, base products can map to M/S and scalar products to scalar native operations; no favorable scalar square discount is introduced. This ledger does not include symbolic constructor validation or subsequent Scholten/Field/Ring constructor work.

Reference actual invocation boundaries: engine rho_native calls Ring::new(r), whose primality assertion precedes its scalar counts initialization. Transferred rho additionally calls Scholten::new(p), including actual nested construction, before target transfer. Plain/negation do not receive fabricated pencil outer source validation calls. Those existing reference validations remain separate uninstrumented coverage obligations. Candidate Scholten::new(p) is also separate actual work, not counted twice here.

Validation command (local software tests, no cohort):

```
CARGO_TARGET_DIR=/tmp/pencil-primality-target cargo test --manifest-path research/pencil_large_prime_20261029/Cargo.toml --features producer --lib
```

Result: 33 passed, 0 failed, 1 ignored. Raw stdout/stderr retained as cargo-test.log. Tests include exact independent small-divisor/prime/composite/large-prime event counts, exponent-zero behavior, differential parity against the actual old engine on 10,000 small values and selected large u64 edges, and rejected-input invocation-prefix retention plus JSON error serialization. The ignored existing full-target test was not run.

Root integration must change ecbench/pencil.rs `.map_err(str::to_string)?` to `.map_err(|error| error.to_string())?`. The rejected API record is not a completed or capped scientific trial and cannot acquire a completed price. Combined root compilation/replay remains the integrating owner's responsibility.
