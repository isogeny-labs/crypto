# Terminal arithmetic qualification

PCC-20261108-67eafc is unsigned producer evidence pending independent signed verification. [pcc:unverified]

validation01 passed first attempt:17556 power value/count cases,96 reciprocal batches with464 inverse identities, three malformed-prefix controls, inherited progression234 setups/151020 evaluations and meter controls. All102 copied-source records reverse exactly. Inherited recovery stdout is byte-identical to the saved predecessor; 116 compiler dependency hashes were rechecked. [pcc:unverified]

```
terminal_power_cases=17556 prefix_batches=96 inverse_values=464 malformed_prefix_controls=3 exact_helper_counts=true
progression_shared_inverse_setups=234 sample_and_weight_checks=2556 evaluation_values=151020 degree_r_minus_one_setups=45 setup_fermat_calls_each=1 invalid_domains=5
interval_meter_self_test=pass direct_lengths=5 naive_poly_products=6 eea_calls=1 fermat_calls=1 fermat_products=12 blocked_aa=pass allocator_trait=pass nesting_error_unwind_overflow=pass
```

Derived counts: positive exponent n uses bit_length(n)-1+popcount(n) products; zero exponent uses zero. A reciprocal tail of lengthN uses2*max(N-1,0) products. Each integrated nonempty Fermat batch therefore saves three scalar multiplication calls relative to its previous power plus tail. This is source-derived component accounting, not runtime or total-recovery measurement.

No compiler/native/render failure occurred. Compiler warnings remain in the receipt logs. Shared PCC reservation was initially held byT1891; source work proceeded under separate scopes, then the cleared ledger scopes were reserved before registration. Historical full Cargo missing-fixture limitation remains. No cloud/timed workload or production width comparison ran.
