# Capacity-bounded powered image recovery

T-1905 integrates section powering into QuadRing and an optional canonical signed-recovery adapter. `recover_powered_image` adds a final power_slots argument. Cap0 retains streaming; ample capacity permits powered branch evaluation and scalar terminal phase. Capacity is 2*m*deg(g) logical output coefficients, not process peak memory or a speed policy.

`generate.py` copies the signed-image predecessor and produces the separate PoweredImage/adapter. `powered.rs` supplies cyclic section arithmetic and terminal scalar phase; `powered-checks.rs` checks exact section/state/public equivalence. All research arithmetic is Rust; Python only generates source and metadata/reports.

Replay from repository root with a fresh binary:

```sh
set -e
python3 research/primefield-companion-followup-20261102/feature-native-powered-image/generate.py
timeout 90 rustc --edition=2021 -O -C overflow-checks=yes --emit=link,dep-info research/primefield-companion-followup-20261102/feature-native-powered-image/driver.rs -o /tmp/powered-image-fresh
timeout 60 /tmp/powered-image-fresh
timeout 60 /tmp/powered-image-fresh --inherited
timeout 30 lean research/primefield-companion-followup-20261102/feature-native-powered-image/Product.lean
```

Section checks include nonunits, repeated quotient factors and m not dividing h. State checks use (r,m)=(17,4),(41,5),(97,3),(193,3),(281,5), all scalars, plus (281,35) at seven fixed scalars0,1,2,3,7,17,280; radii2,1,3,2; power_slots0/16/1024. Public checks use E/F257 y²=x³+x+7, P=(1,3), order281, m5, block7; all targets at lift caps8/16/64 and power_slots0/16/1024, radii1/2/3/4, extra offsets0..16, call cap128.

Validation01 checks branch integration; validation02 adds powered terminal phase; validation03 adds explicit powered/fallback phase coverage assertions and reruns inherited regression. Earlier snapshots preserve changed inputs. No compile/native failure, timed panel, cloud job or original-cohort change. Historical full Cargo missing-fixture limitation is unchanged.
