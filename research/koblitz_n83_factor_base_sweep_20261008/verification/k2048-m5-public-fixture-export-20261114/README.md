# N83 a=0 K2048 public-fixture export

The `primary-k2048-m5-xcnf-fixture` command builds the retained five-summand
shared-root XCNF for any one of the 32 sealed primary-curve public points.
It holds the 339,968-point K2048 base, 2,048 signed-Frobenius columns,
orbit-domain encoding, and identity mask fixed while changing the target.
The existing `primary-k2048-m5-xcnf` command retains its fixture-zero
constructed-control and natural modes.

The exporter checks the exact input bytes before building a formula:

| Input | SHA-256 |
| --- | --- |
| K2048 compressed base object | `aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a` |
| Panel manifest | `68b43524198394a2f0d112f86df0d1c1f69ea1ff817695cf21efe2767b7d17b8` |
| Public probe corpus | `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd` |

It selects exactly one `(a=0, fixture)` row, rejects an absent or duplicate
row, checks the point on the curve and in the 81-bit subgroup, and retains
the fixture number, point, source, corpus hash, formula hash, and shape in the
export receipt. Fixture numbers are 0 through 31. An output path must be
unused; export performs no solver search.

```sh
target/release/examples/koblitz_n83_factor_base_export \
  primary-k2048-m5-xcnf-fixture \
  "$frozen_input_root" 1 "$new_output_json"
```

The new native gate lock schema `n83.k2048-m5-native-gate-lock/v2` binds the
receipt's fixture number, public point, corpus hash, source commit, and exact
XCNF before a solver call. The prior v1 lock remains readable for its
retained fixture-zero evidence. A future run must freeze and commit its v2
lock before execution, preserve every terminal status, and replay any model
against the original XCNF and group row.

This source preparation is the first step toward an ordinary-query yield
panel. The next measurement protocol must pin an admitted host, cost cap,
query order, held-out points, same-target comparison arms, and stop rule.
It must compare the K2048 base with other exact primary-curve bases before
ranking complete cold and online runtimes. The separate `a=1` curve keeps
its own candidate panel and selector.
