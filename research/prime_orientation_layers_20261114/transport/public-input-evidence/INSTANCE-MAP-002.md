# Actual binary-call word port inventory screen

1. Structure: actual kernelLogged emitted multiply/add counts, declared q-bit encodings of two inputs and one output for each binary field call.
2. Existing laws: kernelLogged_multiply/add; Fintype.card_sum/prod/fin/bool and card_le_of_injective; ordinary Nat multiplication monotonicity. No new generic graph or circuit law.
3. Actual inventory: BinaryCalls=Fin(multiply events)+Fin(add events); WordPorts=BinaryCalls*Fin(3q), with all known calls counted.
4. Chain: actual counts -> exact cardinal6q*((2m-1)*|S|+m^2). If q>=m, at least6m^3 declared ports. An actual injective incidence assignment to E*Bool (one of two endpoints per edge) would force at least3q*T edges and at least3m^3 under q>=m.
5. Missing endpoint arithmetic only. Incidence embedding remains an explicit representation condition, not constructed wiring. The cubic screen applies to this literal q-bit-per-word realization; no arbitrary-circuit/algorithm lower bound or sub-exponential expected cost.
6. Public moment/evaluation constructors are actual earlier002source; arithmetic and port-count costs are kept distinct from powers/denominators/bit preparation and field gates. No implicit transport from ZMod2^q word arithmetic to Fp. [pcc:unverified]
