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

## Exact-source export check

The exporter built from clean commit `34cb2f0bfe30b0609a2f55a6d396ecfc60c53e55`
emitted these two local source-check rows. An independent byte read compared
each retained receipt with the raw receipt, the point with the sealed corpus,
and the XCNF length and SHA-256 with the raw formula.

| Fixture | Public point `(x,y)` | XCNF SHA-256 | Bytes | Receipt SHA-256 |
| ---: | --- | --- | ---: | --- |
| 0 | `(1834913121178197242856238,8797964190732846213981377)` | `02d07716333e073f7eb40a7646ae3652b86b2129cb4d226f5dcc611d6bda7fff` | 20,511,067 | `6e2448155e730a3d0f1d9b54ead4025d04bdf1db8ba4730f4c28dc36b54acc68` |
| 1 | `(5809312440517013698248638,3568442427236746854022671)` | `ecf8badc76aa4295b7d005f8c2f55359979e7d4f1210edf829ad6c68d1fb9bef` | 20,511,051 | `12910c8ad562f73103a43b2f44d5828bdb4a4e3fea7d0a8f1096b1f1e48895b3` |

Fixture 0 has the exact XCNF hash of the retained earlier natural arm;
fixture 1 differs. Both export receipts report 111,478 variables, 1,140,557
CNF clauses, 4,980 XOR rows, five summands, and the same 2,048 representative
domain. The raw XCNFs remain outside the repository at
`/Volumes/SSD990-2/tmp/n83-a0-public-fixture-20261114/` and are reproducible
from the hash-pinned input object. The two small export receipts are retained
beside this report. No solver search was run for either new export.

The fixture selection test passed, all 22 exporter example tests passed, and
the native gate example compiled. The full macOS release library suite ended
with 6,237 passed, 23 failed, and 135 ignored; the same 23 test names failed
in the immediately preceding evidence branch, and this branch changes no
library source or root tests. The complete output is retained at
`/Volumes/SSD990-2/tmp/n83-a0-k2048-m5-20261114/public-fixture-lib-test.log`.

This source preparation is the first step toward an ordinary-query yield
panel. The next measurement protocol must pin an admitted host, cost cap,
query order, held-out points, same-target comparison arms, and stop rule.
It must compare the K2048 base with other exact primary-curve bases before
ranking complete cold and online runtimes. The separate `a=1` curve keeps
its own candidate panel and selector.
