# G-AUX checks 1 and 2 on the instrument's own source (edit E4.6), 2026-11-14

Measured. Source: f13a690df016d0a457bd40579fbb9d054f486adf plus `gaux/aux_order.patch`. The tree is the same as with
`gaux/aux_order_f13a690df0.patch` applied by `git apply`: patched `examples/pkm_tower_pilot.rs` SHA-256
`9c63c61a0db94842e8f3c9240b07c154b89e6ffbb403c8e3472fd220970e374d` in both cases.

    cargo test --locked --release --no-default-features --target x86_64-unknown-linux-musl --example pkm_tower_pilot mestre

    test tests::mestre_exact_order_equals_point_count ... ok
    test tests::mestre_and_naive_build_the_same_isogeny_tower ... ok
    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 16.64s

The same two tests also pass on main + patch, before and after the merge of origin/main on 2026-11-14 (17/17 in the full
example suite before the merge).
