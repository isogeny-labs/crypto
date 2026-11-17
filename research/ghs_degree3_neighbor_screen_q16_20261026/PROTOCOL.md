# Degree-three maps to neighboring order-3974 sources

26 October 2026. Conductor T-5410. This follow-up uses the exact
Artin--Schreier criterion and two complete degree-three divisor patterns
established in the [preceding screen](../ghs_automorphism_screen_q16_20261026/REPORT.md).

## Frozen inputs and question

- Twelve normalized ordinary order-3974 elliptic models in
  [`../ghs_binary_cubic_isogeny_q16_20261026/results.json`](../ghs_binary_cubic_isogeny_q16_20261026/results.json),
  each `E: v^2+u*v=u^3+a2*u^2+a6` over Conway `K=GF(4096)` and with
  largest prime subgroup order 1987.
- Four archived genus-three hyperelliptic models in LMFDB class
  `3.16.a_a_aet`, frozen in the preceding screen's
  [`catalog_models_raw.json`](../ghs_automorphism_screen_q16_20261026/catalog_models_raw.json).
- The exact degree-three enumerator and positive control in the preceding
  screen's [`screen_degree3_pattern.py`](../ghs_automorphism_screen_q16_20261026/screen_degree3_pattern.py).

For each of the 48 ordered source/model pairs, test both zero/pole
patterns for every separable degree-three cover to that exact elliptic
source over `K`. Retain every `R,t` witness. A map to a neighboring
elliptic curve would still require a constructed, subgroup-preserving
source isogeny before a transfer attack claim.

Replay constructed smooth genus-three covers to the exact source for
both zero/pole patterns. For the five-support control, choose a hidden
double zero and solve its local Artin--Schreier cancellation equation;
require the enumerator to recover the resulting map. Controls over
`K` establish search sensitivity, not descent to `k`.

## Prior work and claim boundary

[Tian--Li--Wang--Yu](https://doi.org/10.1007/s10623-018-0457-6)
already construct cofactor-two degree-three covers in odd characteristic.
[Tian](https://arxiv.org/pdf/2012.07173) gives an evaluated cubic
Weil-restriction isogeny method, and
[Frey--Kani](https://doi.org/10.1007/978-3-642-25261-7_1)
discuss characteristic-two genus-three Jacobian correspondences.
The present finite screen tests exact binary curve equations. Shared
point counts or a Tate isogeny are not evaluated subgroup maps.
