# Coverage inventory before diagnostic 24

This source inventory records implemented counters and explicit remaining
work. A priced observed subtotal is not a complete operation total.
[pcc:unverified]

| Path / computation | Current evidence | Remaining work |
|---|---|---|
| `gaudry_cubic::{mm,am,sm}`, `residual_walk::{inv_mod,pow_mod}` | Existing modulus-classified hooks | Audit every caller's inline operations |
| `large_prime_filter` modular helpers | New exact tiny-kernel count test passes | Container and normalization work is separate |
| `WorkRow::axpy`, including source certificate | New sorted merge, 768 map-oracle cases and inherited rowspace/replay tests pass | No claim about whole-filter bookkeeping |
| Graph map/set/sort/clone and graph adapter | Modular work now hooked; legacy wall includes spent work | Emit counts for key comparisons, probes, scans, copying, normalization, scheduling and certificate replay |
| `jv_sieve` residual stream | Jacobian work uses field hooks | Four initial `gen_range` draws and two inline scalar additions per residual need measured RNG/reduction/scalar coverage |
| Sieve / factor-base / F4 | Existing field, sieve-word, probe and F4 counters | Audit all native paths, including rejected work and solver self-checks |
| Wiedemann / Berlekamp–Massey | Existing shared modular hooks and RNG-word charges | Check inline arithmetic, coefficient scans and input/output mapping |
| Answer replay | Native answer check plus separately measured outer replay | Operand rechecks included where inseparable; public domain check excluded |
| Native rho | Existing explicit operation counts, phase tally, measured generic harness prices, outer replay | Generic harness is not the native inversion formula; legacy tuned credits need explicit complete bitop pricing; setup/native coverage audit remains |
| RNG implementation | Selected public hooks count word draws | Generator internals and rejection draws require an explicit coverage decision/count, not silent zero |
| Instruction audit | Hook phase markers exist | Neither Valgrind nor perf is installed in this runtime; arrange a local native audit tool before claiming complete ranked costs |

Sources checked: `docs/units/BITOPS.md`, `src/prime_hyperelliptic/ops.rs`,
`jv_bitops.rs`, the counted cover/rho harness, and the source paths above.
The hyperelliptic convention explicitly prices nontrivial wide random
reductions as one field/modr multiplication and canonical identity reductions
as zero; this is a reuse pointer for the next normalization/RNG patch.
Existing counts and records must keep replaying unchanged. [pcc:unverified]

The conditional stage24 protocol tests adapter execution/ownership and scalar
recovery only. Its analyzer checks primitive schemas, exact repricing,
phase sums, declared gaps, zero foreign/other hooks and false comparative
eligibility. It cannot discharge this inventory by passing an endpoint.
[pcc:unverified]
