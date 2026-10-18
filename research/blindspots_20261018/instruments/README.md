# Blind-spot campaign instruments

These are the Rust instruments for the blind-spot campaign. They used to live in
`examples/`. They were moved here byte for byte after the last card ran, so they
are frozen measurement source. The maintained-code rustfmt and clippy gates
lint only `src/`, `tests/`, `examples/` and `benches/`, so they do not apply
here (`scripts/clippy-maintained.sh`, `.github/scripts/ci-scope.mjs`). Nothing
was reformatted or lint-fixed: rewriting the source would break the sha256 pin
in every task card.

Every card and protocol names an instrument by its old `examples/` path. Read
that path as the file here with the same name; the sha256 is unchanged. A
receipt also records the commit it ran at, and checking out that commit
restores the old path exactly. Some cards for items 03, 04 and 06 ran on an
earlier version of an instrument, from before its repair. Those bytes are at
the commit their receipt records.

All of these build with standalone `rustc` (see each card's `build` argv), and
none links the `crypto` library.

| old path (named in cards) | path now | sha256 |
|:--|:--|:--|
| `examples/blindspots_01_fp2_gaudry.rs` | `research/blindspots_20261018/instruments/blindspots_01_fp2_gaudry.rs` | `0629a73c7ff443e70c6ce16c0872b98a5aebc5a71d25f7bff60156c8693ff559` |
| `examples/blindspots_02_deployed_binary_pricing.rs` | `research/blindspots_20261018/instruments/blindspots_02_deployed_binary_pricing.rs` | `a7bf5c5f0ace27a685dc1ebd3ce8f8667b8a672b864089f7399523aac7daa50c` |
| `examples/blindspots_03_polynomial_method.rs` | `research/blindspots_20261018/instruments/blindspots_03_polynomial_method.rs` | `c4a2a1ac94687e6a8170d651e5ecca81c444d4656f293207437f8394a224647c` |
| `examples/blindspots_04_presentation_search.rs` | `research/blindspots_20261018/instruments/blindspots_04_presentation_search.rs` | `9648dce3fc7a830df89298135b62dbea726b307630a37c2d719b3f998ba0db78` |
| `examples/blindspots_05_elimination_orders.rs` | `research/blindspots_20261018/instruments/blindspots_05_elimination_orders.rs` | `f21a761b97177a130c610d8041cad256c8dfec44e2419fe20233030e69e82adf` |
| `examples/blindspots_06_signature_gb.rs` | `research/blindspots_20261018/instruments/blindspots_06_signature_gb.rs` | `48a39c343ff7abee7fb38bb9a0790d10e353c7b71a0eb5b9209ad60425f45dbc` |
| `examples/blindspots_07_blocked_closure.rs` | `research/blindspots_20261018/instruments/blindspots_07_blocked_closure.rs` | `2d019d295a6e8e74ee814b7ca69ebfa17ddcdd3b7db17b00f0ee427b64c0cf2b` |
| `examples/blindspots_08_learned_branching.rs` | `research/blindspots_20261018/instruments/blindspots_08_learned_branching.rs` | `02478185f7001474c7510e5dd87f3eb5aca0d5ff5a536760f226e2fd892a280a` |
| `examples/blindspots_09_learned_f4.rs` | `research/blindspots_20261018/instruments/blindspots_09_learned_f4.rs` | `7e46e27eaf9f5901f74772e9fd2a5f95457ab3182cda7ffcad320fcd820e5743` |
| `examples/blindspots_10_kernel_search.rs` | `research/blindspots_20261018/instruments/blindspots_10_kernel_search.rs` | `f69bbce49c64a6ff05670fa07ea615ce913f3eb35a5f94597220ca7f363fdb80` |
| `examples/blindspots_11_learned_prefilter.rs` | `research/blindspots_20261018/instruments/blindspots_11_learned_prefilter.rs` | `e6f23662c89f4ed99dc9407817c61a8aa58412c276cccb023af46371c7bc0d66` |
| `examples/blindspots_12_degree_surrogate.rs` | `research/blindspots_20261018/instruments/blindspots_12_degree_surrogate.rs` | `c7eef63fb4aabb23856c60963a4bc28e78a7cb90182a46b68187499e8bfdc102` |
| `examples/blindspots_13_rho_negation.rs` | `research/blindspots_20261018/instruments/blindspots_13_rho_negation.rs` | `cb4a84e5300e64e088d98cd816f09d6c7d0830a7684cada519c18540aa0d20ad` |
| `examples/blindspots_14_prime_m4.rs` | `research/blindspots_20261018/instruments/blindspots_14_prime_m4.rs` | `f6696fc0cafff9120189072a7785452422d68c7944a2812d4667e27c8872b376` |
| `examples/blindspots_15_genus3_la.rs` | `research/blindspots_20261018/instruments/blindspots_15_genus3_la.rs` | `73cfa4891f8eb4651b728e426aa4fd47f2180120b06c60081dab1b230ad5462b` |
| `examples/blindspots_16_gate.rs` | `research/blindspots_20261018/instruments/blindspots_16_gate.rs` | `fcdc33b38b13a781e88fab15b4e434f14ea4afc0bd7147eddb1c57f5afc1a7f9` |
| `examples/blindspots_16_size_gated_oracle.rs` | `research/blindspots_20261018/instruments/blindspots_16_size_gated_oracle.rs` | `d6a15aa23eb805edde518d12ab19e5b44653e44e60de3b46e69b542a4acd4b99` |
| `examples/blindspots_17_cyclo_cost.rs` | `research/blindspots_20261018/instruments/blindspots_17_cyclo_cost.rs` | `15030cd378f688dcf56df74cdbe385b3385ae08831908632e4da88c3f4c7e976` |
| `examples/blindspots_19_checks.rs` | `research/blindspots_20261018/instruments/blindspots_19_checks.rs` | `751277a8c075483cb0d1edcff9ef44e76f21164e989e289c8c6a54950c158ec5` |
| `examples/blindspots_20_quantum_classical.rs` | `research/blindspots_20261018/instruments/blindspots_20_quantum_classical.rs` | `a2e9f4a1f2529af28a1e191cc2383e6233f157f999dcf2ac240921520511e30a` |
