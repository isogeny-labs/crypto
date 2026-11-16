# Triple-mode qualification

PCC-20261108-130f40 is unsigned producer evidence; independent signed review pending. [pcc:unverified]

validation01 passed first attempt:40 reader configurations,975 coefficient equalities,1920 quotient equalities,3640 candidate-state equalities and843 public target/cap cases. There were735 subsequent direct-image updates in the public controls. Outcomes, routes, all recorded source stats and direct initialization/update/materialization counts match. All105 source records reverse exactly; inherited stdout is byte-identical; 119 compiler dependency hashes rechecked. [pcc:unverified]

```
triple_readers=40 coefficient_equalities=975 quotient_equalities=1920 candidate_state_equalities=3640 buffer_bound=3cap_denominators unsupported_alignment_rejected=true
triple_public_cases=843 exercised_updates=735 outcomes_routes_source_calls_equal=true
terminal_power_cases=17556 prefix_batches=96 inverse_values=464 malformed_prefix_controls=3 exact_helper_counts=true
progression_shared_inverse_setups=234 sample_and_weight_checks=2556 evaluation_values=151020 degree_r_minus_one_setups=45 setup_fermat_calls_each=1 invalid_domains=5
interval_meter_self_test=pass direct_lengths=5 naive_poly_products=6 eea_calls=1 fermat_calls=1 fermat_products=12 blocked_aa=pass allocator_trait=pass nesting_error_unwind_overflow=pass
```

Buffer bounds refer to logical/requested entries, not allocator overhead or process peak memory. Actual temporary storage includes requests, interval values, denominator/prefix vectors, triple output and recursive quotient values. The reader uses a single anchor traversal; the evaluator may do more quotient work before C0 pruning. No timing or total-work comparison was run, and no automatic policy switch is made.

The unsupported h/m alignment is rejected by the reader; the optional public adapter retains the original eligibility fallback. Original staged and prior recovery paths are retained. No compile/native/render failure occurred. The first report render used italic text for underscores in coefficient indices; the final render uses mathematical subscripts and was inspected. Historical full Cargo missing-fixture limitation remains. No cloud or production panel ran.
