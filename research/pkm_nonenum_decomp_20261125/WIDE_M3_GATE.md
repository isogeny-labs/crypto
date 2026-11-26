# Three-summand PKM arithmetic gate for named prime fields

Date: 2026-11-25. This is exact prime arithmetic plus a labelled random-model
heuristic, not a relation benchmark. The prime constants and valuations are
those in `research/pkm_degree32_wide_transfer_20261124/results.json`.

For a factor base of abscissa size `B=2^t`, use the planning proxy
`lambda_3=B^3/(6p)` for expected unordered three-summand decompositions per
random target. The strict height `t_1` is the least integer with
`2^(3t_1)>=6p`. Actual lifted point counts, duplicate multisets, subgroup
membership and rank will change the constant; measure them before pricing a
whole pipeline. A shortfall of one height level changes `lambda_3` by a
factor of eight, so it is a cost penalty rather than a logical impossibility.

| Field | `v2(p-1)` | `v2(p+1)` | strict `t_1` | best available `p±1` height | proxy targets per relation at that height | construction decision |
|:--|--:|--:|--:|:--|:--|:--|
| s59 minus | 16 | 1 | 21 | Kummer 16 | about `2^12.6` | density costly; solver unpriced |
| s59 plus | 1 | 18 | 21 | norm-one/Dickson 18 | about `2^6.6` | density potentially testable; solver unpriced |
| s59 registry | 7 | 1 | 21 | Kummer 7 | about `2^39.6` | density costly; solver unpriced |
| NIST P-192 | 1 | 64 | 65 | norm-one/Dickson 64 | about `2^2.6` | density near constant; solver unpriced |
| NIST P-224 | 96 | 1 | 76 | Kummer 76 | order one | degree available; solver unpriced |
| NIST P-256 | 1 | 96 | 87 | norm-one/Dickson 87 | order one | degree available; solver unpriced |
| NIST P-384 | 1 | 32 | 129 | norm-one/Dickson 32 | about `2^290.6` | density gate fails |
| NIST P-521 | 1 | 521 | 175 | norm-one/Dickson 175 | order one | degree available; solver unpriced |

The auxiliary-isogeny construction has no direct `p±1` valuation condition.
Its required height is still approximately `t_1` for this proxy and its
auxiliary search and compact solver must be priced separately. None of the
named fields has a measured three-summand compact witness solver. In the
current implementation diagnostic, a generic Groebner root call is censored
after 60 seconds already at `t=3` over `F_97`. Therefore mathematical degree
availability alone does not admit a wide run. The next cost decision requires
a source-pinned solver whose certified prefix work and peak memory are
measured on the frozen independent-curve ladder.

## Existing implementation ceiling (2026-11-25 audit)

`src/cryptanalysis/ic_framework/pkm_semaev.rs` already has a separate
degree-2 Dickson tower and F4 witness path for `p+1` analogue fields.
Its `dickson_abscissae` currently accepts `1 <= t < 30`; the native F4
backend uses a `u64` modulus and at most 64 tower variables. Thus the
s59-plus available height 18 is within these representation limits, subject
to a verified norm-one construction and measured complete cost. P-192's
height 64, and the favorable P-256 and P-521 heights, exceed the current
backend's limits as well as its field width. These are implementation gates,
not arithmetic exclusions. The `p-1` Kummer staged-resultant diagnostic
returned exact witnesses at `t=3` but was censored in its second resultant
at `t=4`; see `RESULTANT_DIAGNOSTIC_20261125.md`. Its expanded output is
unsuitable for wide cost projection. P-384 remains screened out by the
relation-density proxy independently of solver choice.
