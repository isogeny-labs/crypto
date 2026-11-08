# Reciprocal signed-mode filter

Additive T-1896 construction: derive the signed-mode product from one coefficient stream and apply its reciprocal norm in the existing quadratic quotient. Public observation method consumes signs, not a hidden scalar. Prime scalar field r, exact order m>=3 dividing (r-1)/2, and nonzero inverse-chart input are required; epsilon is tested as a branch hypothesis.

`generate.sh` copies the prior Rust sources unchanged except for a uniquely renamed predecessor checker entry point, then creates driver.rs with signed.rs and checks-signed.rs. The inherited implementation is preserved. The new code is a filter, not a replacement recovery adapter. See DERIVATION.md for zero cases, orientation, cost obligations and sources.

Qualification is deterministic software correctness, with no timing comparison, production panel or cloud submission. Dense coefficients and scalar enumeration are confined to checks-signed.rs; the filter uses the existing bounded coefficient stream.

Replay from repository root:

```sh
sh research/primefield-companion-followup-20261102/feature-native-signed-reciprocal/generate.sh
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-native-signed-reciprocal/driver.rs -o /tmp/signed-check
timeout 60 /tmp/signed-check
timeout 60 /tmp/signed-check --inherited
```

Reference controls use seven (r,m) pairs: (7,3),(13,3),(17,4),(41,5),(97,3),(281,5),(281,35), caps1,2,7 and radii1,2,r-1. They include h odd/even, all normalized scalar values, both epsilon hypotheses, zero observations, repeated quotient factors and invalid inputs. Full Cargo is not rerun: the existing missing-fixture limitation is unchanged. Frozen original cohort is untouched.
