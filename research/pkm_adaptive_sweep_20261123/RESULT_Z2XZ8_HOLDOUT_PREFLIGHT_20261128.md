# Held-out X1(2,8) exact-size map preflight

The six new curves and 96 targets were selected and committed as
`f7e32666ab7` under the precommitted
`PROTOCOL_Z2XZ8_EQUAL_SIZE_COLD_20261128.md`. The selection uses field sizes
11 and 14 bits and no map or solver feedback. An independent Euler-character
and group-law verifier replayed all curve orders, generators, target points,
hashes and workload files before the first map call. The manifest SHA-256 is
`99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8`.

The release `ic` binary was built from clean full source commit
`a0642f42e205aba4951dcf9732721d62211d7c33` with Rust 1.99.1; binary
SHA-256 `8e5ba8c40647a076310a2f4205774caaa6925e7eadda6ca961fe8e4590a7a027`.
The build receipt is `/Volumes/SSD990-2/pkm-equalsize-build-20261128/BUILD.json`,
SHA-256 `e09abcf0d0ff07d82f1392bca04e3abb159fea853539d8a328d40b4a5f8c63b7`.

The separate diagnostic preflight invoked one target and one MITM relation
trial per curve with `rho-runs=0`. The one-query process exited incomplete
on every curve. These are **map-construction observations**, not complete
target results or qualifying timing ratios. Native selection found an
interval-equal lifted base on every held-out curve without replacing a
curve or changing the `m<=64`, 64-distinct-coset caps.

| Curve | `p` | Interval and map columns | Selected `m` | Distinct cosets tested | Search GAE | Tonelli modular multiplications |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `b11_c0` | 2081 | 17 | 10 | 6 | 731 | 37,715 |
| `b11_c1` | 2113 | 16 | 2 | 22 | 1,454 | 11,672 |
| `b11_c2` | 2273 | 17 | 24 | 1 | 1,454 | 139,904 |
| `b14_c0` | 16417 | 14 | 7 | 9 | 778 | 62,572 |
| `b14_c1` | 16481 | 20 | 6 | 28 | 1,139 | 16,015 |
| `b14_c2` | 16673 | 17 | 5 | 5 | 350 | 16,889 |

The append-only store is
`/Volumes/SSD990-2/pkm-z2xz8-holdout-preflight/receipts.jsonl`, SHA-256
`4e849aebeb3da8047b7d69fa4c953483dd7d3bb6a7d45659f2971abd0a195cea`.
It retains every launch, exit, workload hash, raw stdout/stderr hash, wall,
child CPU, peak RSS, pre/post load and native counters. The replay checker
validated all six report identities, 12 raw file hashes, one-target inputs,
source/binary/build identities and interval lift counts. Runner SHA-256:
`a13ce4fa1bdf9fcc99e1fded1d62b10db84e8b58a14c41eb8ae5d6eb00938a40`;
verifier SHA-256:
`94768e7876ac5e246913561b081ce6ea2231d2a5a2f337c3dc4dafb3153cba1e`.

The search GAE is group work only. Tonelli modular multiplications, target
Jacobi calls, coefficient arithmetic, Vélu evaluations, and other native
work are not all converted into the CLI's reported GAE. Host load was high,
so process wall is diagnostic. The one-query cap cannot collect rank or
verify a target. The next gate is a source-pinned full cold panel with
all sixteen frozen targets, equal-size interval and existing PKM controls,
certified joins, full rank and matched rho. It must retain failed and
censored arms and price all positive work before any cost claim.

Replay:

```sh
python3 research/pkm_adaptive_sweep_20261123/verify_z2xz8_holdout.py
python3 research/pkm_adaptive_sweep_20261123/verify_z2xz8_holdout_preflight.py
```
