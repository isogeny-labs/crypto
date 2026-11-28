# Signed-product image and public recovery

T-1899 integrates the signed reciprocal equation with canonical reciprocal fibers across changing radii. The optional `recover_signed_image` adapter uses public source signs and final point verification. Radius scaling requires paired bases sV and s^-1 V^-1; taking the old-chart norm at sV is generally wrong. See DERIVATION.md for the proof, phase update and cost accounting.

`generate.py` copies predecessor Rust sources, replaces the canonical image's equation, and generates a separate adapter by exact substitutions. Existing methods are unchanged. `transport.rs` and `image-checks.rs` are new. Generation and metadata use Python only as source/report orchestration; all research arithmetic and controls run in Rust.

Replay from the repository root:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-signed-image/generate.py
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-native-signed-image/driver.rs -o /tmp/t1899-check
timeout 60 /tmp/t1899-check
timeout 60 /tmp/t1899-check --inherited
timeout 30 lean research/primefield-companion-followup-20261102/feature-native-signed-image/Transport.lean
```

Deterministic controls: six parameter pairs (17,4),(41,5),(97,3),(193,3),(281,5),(281,35), scales1,2,3,r-1, four quotient polynomials including repeated factors, and radii2,1,3,2. Dense reference coefficients are produced by repeated multiplication of X-1. Independent image support checks evaluate original F0 and F1*F_(m-1), without reciprocal substitution, over fixture image roots. Public recovery uses E/F257: y²=x³+x+7, P=(1,3), order281, m5, block7, caps8/16/64, radii1/2/3/4, extra offsets0..16, call cap128.

No timed/production/cloud experiment, no changes to the frozen original cohort and no automatic method selection. Historical full Cargo missing-fixture limitation remains; standalone affected checks are the validation boundary.
