# Amendment 2: source formatting replay

2026-11-15 07:24 UTC. `cargo fmt --check` found whitespace/style
differences in the native verifier after run `20261115T0718Z`.
`cargo fmt` changed only formatting. The original source is preserved
in S3 checkpoint `20261115T071742Z-5d414672f962` with SHA-256
`2c5cd29995324ba4c0acdf0f229f178e3812a5d9dc10aadced95295fbec8b3f6`.
The formatted source SHA-256 is
`993eb5e85793ee3e222dfcb18cc207937340430dd4f1e958ee439b7b2d149a93`.
Repeat the same fixed native calibration after a new checkpoint gate,
preserve both run records, and report any differing values.
