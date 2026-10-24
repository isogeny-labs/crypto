# SafeCurves / SECG exotic-trapdoor slots

Browsing copy of the executed census and toy grind in [`research/safecurves_trapdoor_slots_20261017/`](../../research/safecurves_trapdoor_slots_20261017/). It is a count-of-candidates / rho-work-factor study, not a solver timing benchmark.

The census reports weak non-quadratic sextic classes at rho work factors 2^28 (secp192k1), 2^55.5 (secp224k1), and 2^65.5 (secp256k1). At 48 and 96 bits, the toy search found a strong published/twist pair with a weak sibling after 2–3 scanned primes; the native 96-bit run independently reproduced the same hit after 3. At 128 bits the certified pair appeared after 40 scans. The 256-bit bounded probe stopped after 12 scans with seven orbits uncertifiable under the configured factorization budget and no certified triple among the five certifiable candidates; the registered 256-bit prediction remains open.

No deployed-curve weakness beyond the named census rows is claimed, and no key recovery is shown. The toy instances do not establish a 256-bit grind result.

- [Criteria/slots diagram](slots.svg)
- [Census rho-cost graph](rho_costs.svg)
- [128-bit native grind graph](grind128_classes.svg)
- [Full protocol and report](report.pdf)
- [Round-C native report and raw JSON](../../research/safecurves_trapdoor_slots_20261017/round_c/REPORT.md)
