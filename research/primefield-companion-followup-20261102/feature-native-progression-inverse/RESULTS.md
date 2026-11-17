# Shared progression inversion qualification

PCC-20261108-f7fef5 is unsigned producer evidence; independent signed verdict pending. [pcc:unverified]

validation01 passed on the first attempt. It checks234 setups,2556 sample/weight identities,151020 evaluation values,45 degree-r-minus-one setups and five invalid-domain cases. Each setup records exactly one exponent-r-minus-two event. The existing instrument tests pass. Inherited stdout matches the saved predecessor byte for byte. There are101 reversible source records and 114 hashed compiler dependencies, rechecked after validation. [pcc:unverified]

```
progression_shared_inverse_setups=234 sample_and_weight_checks=2556 evaluation_values=151020 degree_r_minus_one_setups=45 setup_fermat_calls_each=1 invalid_domains=5
interval_meter_self_test=pass direct_lengths=5 naive_poly_products=6 eea_calls=1 fermat_calls=1 fermat_products=13 blocked_aa=pass allocator_trait=pass nesting_error_unwind_overflow=pass
```

The derived saving per setup is B(r-2)-3 scalar products, where B is bit length plus popcount for the exact retained binary exponentiation helper. Both inverse verification multiplications remain, as do polynomial samples, weights and cache shape. No timing or empirical total-work conclusion is made.

No compiler/native/render failure occurred in this attempt. Compiler warnings are preserved. Historical full Cargo missing-fixture limitation remains; this standalone check does not resolve it. The production width panel remains draft/unrun and must be rebound before using the new constructor.
