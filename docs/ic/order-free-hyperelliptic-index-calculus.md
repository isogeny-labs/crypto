# Prime-field hyperelliptic index calculus without the ambient order

The Rust library entry point is [solve_order_free](../../src/cryptanalysis/order_free_hyperelliptic_ic/mod.rs). It accepts a checked imaginary curve model \(y^2=f(x)\) over an odd prime field, with \(f\) monic, squarefree, and of degree \(2g+1\). It takes two reduced Mumford classes \(P,Q\) in a specified cyclic subgroup and the prime \(r=\operatorname{ord}(P)\). It returns a scalar after checking \([s]P=Q\). The ambient Jacobian order is unnecessary.

The collector enumerates split and ramified places through a degree cutoff. It retains a single label for each conjugate pair and uses a signed coefficient for the other place. Each trial masks a random combination of \(P,Q\) with a random integer combination of the base. Smooth trials produce a matrix of exact integer relations. The solver uses fraction-free elimination to obtain a primitive dependence between the rows. Integer cancellation is important when \(P\) lies in \(rJ\), since reduction of a factor-base coefficient modulo \(r\) need not preserve its group contribution.

The default cutoff follows the split-place and smoothness bounds in the research note, capped at the genus. The caller can supply a smaller cutoff and configure the polynomial and relation budgets through OrderFreeIcConfig. The enumerated residue-field sizes must fit in a machine word. Exhaustion is returned as a structured error. An overly small cutoff can reduce coverage. The implementation uses dense masks and matrices, and the defaults target small reproducible inputs. The asymptotic bound belongs to the proof with growing genus and suitable budgets. Generic fixed-genus inversion remains open. [Prime-field research and proof](https://github.com/isogeny-labs/cryptanalysis/pull/1760) [pcc:unverified]

## Reproduce the known-answer checks

~~~sh
cargo test --locked --release --lib order_free_hyperelliptic_ic
~~~

- Over \(\mathbb F_3\), the genus-three curve \(y^2=x^7+2x^3+1\) has a test generator \(P=[7]U\) of order 7. The native test checks divisibility and the source relation, then solves \(Q=[2]P\) using both the default and degree-one cutoffs. The test includes a valid relation for which reducing a coefficient modulo 7 before group cancellation would give the wrong scalar. [pcc:unverified]
- Over \(\mathbb F_{29}\), the genus-two fixture independently used for the pairing witness has \(Q=[3]P\) in an order-7 subgroup. The native test uses smooth-divisor relation collection directly. [Original fixture](https://github.com/isogeny-labs/cryptanalysis/pull/1760)
- The remaining tests cover budget exhaustion, subgroup validation, signed place decompositions, and primitive integer cancellation. They check arithmetic and solver behavior on bounded inputs.

## Local validation

An isolated Rust 1.100.0 release build compiled the solver and its unchanged Jacobian, field, and utility sources directly, with the repository's primality helpers and locked numerical dependencies. It passed all 11 solver checks and 47 dependency unit tests. The existing ignored arithmetic benchmark stayed ignored. The isolated build exercises the native implementation and source scalar checks. PCC standing is unverified. [pcc:unverified]

The full library test build is blocked at base commit 9ec88b7637aac4ce816739155f12f518b393da37. The compiler reports duplicate definitions in ic_boundary and ic_framework, missing linalg_mixed integration, and mismatched F4 Hilbert diagnostic APIs. These diagnostics are in paths matching the base revision. The full test suite could not reach execution.
