# Certicom ECCp-131 native input validation — 2026-10-29

The user-requested prime counterpart is officially **ECCp-131**, according to
[Certicom's curve list](https://www.certicom.com/en/curves-list). This change
adds the public p, a, b, n, h=1, P and Q as decimal-string runner input in
`crates/hashed-ecdl-tensor/examples/eccp131.json`, mirrored byte-for-byte at
`cryptanalysis:bindings/rust/hashed-ecdl-tensor/examples/eccp131.json`.
No private scalar or fixture seed is supplied. Existing native Rust source
and the separately landed native validation receipt remain unchanged.

Input SHA-256:
`47da1dae89307bd5ded251a1109155e352ef11c25dadcadeebe7eb3c109c4c81`.
It is checked against the official hex parameter transcription in the paired
scalar reference. The companion's 39 Python development tests additionally
check all 361 pairs in a small group and 16 published P/Q multiples against
independent OpenSSL. Its actual target graph's construction is a separate
implementation, not a native challenge-graph result.

## Actual native checks

Host: Linux x86-64 cloud development container, Rust 1.99.1
(58cf8f9c3 2026-10-13), minimal stable toolchain installed during this follow-up.
Historical earlier statements that Cargo was unavailable record the initial
checkout and are superseded for these commands. No timing comparison is made.

```sh
cargo test --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml
cargo run --locked --release --manifest-path crates/hashed-ecdl-tensor/Cargo.toml -- solve crates/hashed-ecdl-tensor/examples/eccp131.json /tmp/eccp131-native-receipt.json
```

The regression suite exits 0: **27 passed, zero failed**, with runner and
documentation targets also passing. Its [raw output](hashed-ecdl-eccp131-tests.txt)
is retained. These are the unchanged native backend's regression checks, not
new challenge recovery tests.

The challenge command exits 1 with **`resource limit: AKS multiplicative order`**
under the bundled 2,000,000-step, 200,000-vector, 4,096-bit limits. It loads the
challenge input but exhausts exact prime certification before graph/relation
search and writes no recovery receipt. The paired
[raw preflight](https://github.com/isogeny-labs/cryptanalysis/blob/a3e8d29f05625a38a6c5f6c1361527b782dcb43c/experiments/hashed-ecdl-tensor-reference/prime-development/native-preflight.txt)
and command/source/environment manifest preserve that failure. Increasing
limits has not been verified to solve this or either published challenge.

The Python companion separately compiles the published prime target into
1,564,506 edges, 654,523 variables and degree 1,032. It reaches its default
u_0=0 exact-leaf budget limit without a challenge witness. The two budget
failures concern different stages and do not prove nonexistence, a practical
speedup, or a theorem obstruction.

Both challenge scalars remain unresolved. The existing native general
correctness/complexity audit and the companion's independent parity/general
ell checks remain open. The pipeline diagram/PDF and detailed retrospective
correctness receipt live in the paired companion's `prime-development/`.
Existing native papers, diagrams, source manifests and leaderboard rows are
preserved. No new theorem, formal proof, scientific cloud experiment or
admitted IC/rho measurement is claimed. Development checks use the explicit
CHECKPOINT_SKIP reason; no new experiment or compute reservation was launched.
