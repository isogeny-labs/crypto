# H116 multi-prime boundary source gate

No prospective p=211 or p=241 boundary expansion existed when this receipt was
written. Both sources compile with optimization and warnings denied under
`rustc 1.100.0 (0ef1bc386 2026-11-09)`. The only executed source path was the
retained p=31 calibration self-test. [pcc:unverified]

```text
b4f0928d3076645fe745e996d3bff1d814370808ffbc1f6e2369feaf22d3a259  FROBENIUS_BOUNDARY_PANEL_PROTOCOL.json
75eafc8480abe90e5f59ce354c50a61aabf2139b4d83d21cccd45ae7410cfc00  FROBENIUS_BOUNDARY_PANEL_PROTOCOL.md
0cfc8232cfb9a1ddb9d24822163601074253604f52a09176a0b386f9fcdcde4a  h116_frobenius_boundary_panel.rs
5ccfacafa0f80227a40d189e04aa75b6c717422bf3a66ab6af117a4b37df8f82  h116_frobenius_boundary_panel_verify.rs
85dc25d303991d25fd38d677a260f16cfd2b387f6e3d442e6550dbd76a174783  h116-frobenius-boundary-panel
01aea86ae8c9eecc8d79bb126a62c7fa4b020d239dc3b00730c0d0fabcaa2168  h116-frobenius-boundary-panel-verify
8fb0dad351eb1b7948e42f553b6e9ff7f33ae91527e4bd09d8566ed767fe4eaf  FORMAL_SCALAR_OOS_ORACLE.json
```

The producer self-test reconstructs the retained p=31 normalized scalar
prefix through coefficient 17 and checks its cleared target-curve identity.
It printed `h116 Frobenius boundary panel self-test ok`. The verifier was
compiled but not run because no prospective result exists yet. [pcc:unverified]

The environment's pinned Rust toolchain does not contain the optional
`rustfmt` component; this is recorded as a formatting-tool limitation, not a
scientific result or a relaxed compile gate. [pcc:unverified]

The repository-wide `cargo test --release --lib` pre-push attempt was stopped
after repeated identical crates.io proxy connection failures while updating the
index. It reached no compilation or test failure. The two directly affected
standalone binaries remain qualified by their strict builds above.
[pcc:unverified]

