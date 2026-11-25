# Separate `p-1` and `p+1` PKM construction and cost gates

Date: 2026-11-25. This refines `WIDE_M3_GATE.md`; it is a derived screen,
not a wide-field solver benchmark. All rows use the same `m=3` planning
proxy `lambda_3=B^3/(6p)` for `B=2^t`. The least power-of-two height with
proxy at least one is `t_req` in the earlier table. The actual number of
lifted factor-base columns, failed target queries, rank, linear algebra,
descent and rho cost must be measured before an end-to-end conclusion.

The paper's `p-1` map is a multiplicative subgroup or coset whose
membership is a composition of low-degree power maps. The `p+1` rows
below use a separate norm-one torus trace and Dickson recurrence; they
are an experimental extension, not a construction attributed to the
PKM paper. Auxiliary-isogeny fibres are a third map family and require
their own auxiliary search and complete cost.
For `p+1=2^v q` with odd `q>1`, an unramified Dickson fibre can reach
height `v` only after selecting a noncritical, nonzero top trace from the
odd-order norm-one subgroup. The current default `c_top=0` reaches at
most `v-2` as a norm-one fibre. See the proof and per-field construction
gate in `research/pkm_pplus1_fibre_gate_20261125.md`.

| Field | Side | Available height | `m=3` constant-density height | Construction and present cost decision |
|:--|:--|--:|--:|:--|
| s59 minus | `p-1` Kummer | 16 | 21 | Constructible in the current `u64`/64-variable representation (48 tower variables); proxy about `2^12.6` targets per relation at maximum height. Retain as a measured-cost gate; no wide run is admitted from density alone. |
| s59 minus | `p+1` Dickson | 1 | 21 | A height-1 norm-one fibre needs a noncritical top trace; even then the available height is too small for this three-summand density plan. |
| s59 plus | `p-1` Kummer | 1 | 21 | Available height is too small for this three-summand density plan. |
| s59 plus | `p+1` Dickson | 18 | 21 | Proxy about `2^6.6` targets per relation. Since `257 | (p+1)/2^18`, an order-257 norm-one element gives the required nonzero `c_top`; `c_top=0` reaches only height 16. Field and tower widths fit the current native representation (54 tower variables), but numeric top selection, split/lift verification, witnesses, solver time, rank and cold cost remain open. This is the first wide candidate only if the small-field F4 gate passes. |
| s59 registry | `p-1` Kummer | 7 | 21 | Proxy about `2^39.6` targets per relation; density gate rejects this parameter choice. |
| s59 registry | `p+1` Dickson | 1 | 21 | A height-1 norm-one fibre needs a noncritical top trace; available height is too small for this three-summand density plan. |
| NIST P-192 | `p-1` Kummer | 1 | 65 | Available height is too small for this three-summand density plan. |
| NIST P-192 | `p+1` Dickson | 64 | 65 | Proxy about `2^2.6` targets per relation; an order-three norm-one element gives `c_top=p-1` and a degree-`2^64` fibre through 64 quadratic layers (`c_top=0` reaches 62). The current `u64`, `t<30`, and 64-tower-variable backend cannot represent this row; full-rank storage at the proxy base size is also far outside the bounded panel. No implementation admitted. |
| NIST P-224 | `p-1` Kummer | 96 | 76 | Choose `t=76` for the constant-density proxy, rather than all 96 available levels. Current arithmetic is `u64` only; a useful base at `B=2^76` is far outside the bounded full-rank pipeline. No implementation admitted. |
| NIST P-224 | `p+1` Dickson | 1 | 76 | A height-1 norm-one fibre needs a noncritical top trace; available height is too small for this three-summand density plan. |
| NIST P-256 | `p-1` Kummer | 1 | 87 | Available height is too small for this three-summand density plan. |
| NIST P-256 | `p+1` Dickson | 96 | 87 | Choose `t=87` for the constant-density proxy; this height can use `c_top=0`. Current field/tower widths and a useful base at `B=2^87` exceed the bounded pipeline. No implementation admitted. |
| NIST P-384 | `p-1` Kummer | 1 | 129 | Available height is too small for this three-summand density plan. |
| NIST P-384 | `p+1` Dickson | 32 | 129 | Height 32 needs a nonzero top trace, while `c_top=0` reaches 30. Even at maximum height, the proxy is about `2^290.6` targets per relation; density gate rejects this parameter choice before solver cost. |
| NIST P-521 | `p-1` Kummer | 1 | 175 | Available height is too small for this three-summand density plan. |
| NIST P-521 | `p+1` Dickson | 521 | 175 | Choose `t=175` for the constant-density proxy; this height can use `c_top=0` even though `p+1` is a pure power of two. Current field/tower widths and a useful base at `B=2^175` exceed the bounded pipeline. No implementation admitted. |

The large-base decisions are conditional on the same random-model density
proxy: a useful constant-density three-summand base has on the order of
`B` columns, so even a one-bit rank-state vector scales as `B/8` bytes.
A nominal one-column-per-abscissa vector is `2^61` bytes at `B=2^64`
for P-192 and grows further for P-224,
P-256 and P-521. Actual lifted-column counts and storage formats can
change constants, while a smaller base pays a larger failed-query count.
This table screens the stated constant-density design and does not prove
an impossibility result for every PKM solver or every arity.

Evidence: exact prime valuations and arithmetic proxies are in
`WIDE_M3_GATE.md` and
`research/pkm_degree32_wide_transfer_20261124/results.json`.
The native Dickson height limit is in
`src/cryptanalysis/ic_framework/pkm_semaev.rs` at `dickson_abscissae`;
the tower F4 backend uses a `u64` modulus and at most 64 tower variables.
The staged `p-1` resultant implementation already reached its 60-second
cap at `t=4`; see `RESULTANT_DIAGNOSTIC_20261125.md`. None of those
component diagnostics supplies a wide complete-cost measurement.
