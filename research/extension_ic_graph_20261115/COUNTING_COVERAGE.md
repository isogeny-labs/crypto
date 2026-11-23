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

After diagnostic24, the projection adapter adds optional observed subtotals
for mapping/filter/replay/output and input/candidate matrix nonzeros. The
solver has separate original/projected call subtotals and attempt identities.
These measure the existing hooked work; native coverage remains incomplete.
The analyzer checks the sequence, shapes, sums and remainder within LA.
Source tests/build and any later actual endpoint are separate obligations.
[pcc:unverified]

Local instruction-tool setup subsequently builds Valgrind3.28.0 from the
[official release](https://valgrind.org/downloads/current.html), source
SHA2569f7a18d128f05d3a80fea53e6953f34aad7200a425ceaee46099a8a798a7dd9c.
The release page's published SHA1 matches; no GPG signature check is asserted.
Read the [Callgrind manual](https://valgrind.org/docs/manual/cl-manual.html)
on Ir events and dump/collection control. Installed under
`/workspace/ic-local-tools/valgrind-install`; /bin/true smoke test succeeds.
Receipts are validation/valgrind-local-*. No cryptographic instruction audit
has run yet, and a future audit remains diagnostic, not proof of complete
instrumentation or physical gate costs. [pcc:unverified]

After26/27, shared Wiedemann/BM data scans/comparisons, two scalar additions
per cover residual and selected stream-compatible modular samplers have
exercised counters. Diagnostic27records actual rejected draws, separately
priced integer multiply/add events and explicit unpriced subtraction/clz/
shift counts, with per-phase conservation and unchanged RNGstream tests.
The old inventory above describes its earlier checkpoint; generator
internals, other sampling paths, native/container coverage and complete
strongest-rho pricing remain open. No total or ranking is promoted.
[pcc:unverified]
