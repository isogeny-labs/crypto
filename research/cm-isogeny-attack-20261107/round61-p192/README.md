# Round61: marked Mobius orbits

Read report.md, orbit-proof.md, scope.json and both amendments. The native candidate uses complete transverse norms; the known-label scalar reference is separate. The full-width P-192 check qualifies manufactured matrix equations and public affine operations.

From this directory, hydrate any repository .artifacts.json entries before manifest checks. Keep the sibling inputs listed in dependencies.sha256 available. Build and replay with one process and a finite command timeout:

```sh
CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=/tmp/mobius-target timeout 120 cargo build --release --locked --manifest-path native/Cargo.toml
/tmp/mobius-target/release/mobius-orbit --check orbits.json > /tmp/orbits.json
cmp orbits.json /tmp/orbits.json
/tmp/mobius-target/release/mobius-orbit --scalar-reference --check scalar-reference.json > /tmp/reference.json
cmp scalar-reference.json /tmp/reference.json
/tmp/mobius-target/release/mobius-orbit --residue-control --check residue-control.json > /tmp/residue.json
/tmp/mobius-target/release/mobius-orbit --residue-control --scalar-reference --check residue-reference.json > /tmp/residue-ref.json
/tmp/mobius-target/release/mobius-orbit --large-residue-control --check large-residue.json > /tmp/large.json
/tmp/mobius-target/release/mobius-orbit --large-residue-control --scalar-reference --check large-residue-reference.json > /tmp/large-ref.json
/tmp/mobius-target/release/p192 "$PWD" --check P192-matrices.json > /tmp/P192.json
/tmp/mobius-target/release/p192 "$PWD" --residue-check --check P192-residue.json > /tmp/P192-residue.json
sha256sum --check dependencies.sha256
sha256sum --check manifest.sha256
```

Receipts count exhaustive qualification and walks together, not an online interval. The norm constructs a fresh target graph and enumerates r factors. The original, r17 and r197 panels retain their separate frozen/amended provenance. failures.json retains five fruitless attempts. The legacy broad check timed out during compilation. Scientific standing is unverified until an independent signed PCC verdict. No P-192 full norm evaluation or unknown-scalar recovery was performed.
