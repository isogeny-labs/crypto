# Local Rust scale results — 15 November 2026

The native port merged in [PR 4340](https://github.com/isogeny-labs/crypto/pull/4340).
The local scale study is complete under the user's explicit local/Git-only
authorization. All observations and the hand proof below remain
**[pcc:unverified]** pending independent signed validation.

The quadratic Vieta core plus binary smooth-coset constraints avoids constructing
summation polynomials or interpolating their coefficients. The measured extractor
uses composed quadratic remainders, gcds and finite-field root extraction. It
returns complete ordered two-term relations, including a diagonal branch;
three-term streaming is compared with scalar pair-table MITM and nested lookup.

| Conditioned queries | m | Median Vieta primitive multiplications | Median batched lookup multiplications |
|---|---:|---:|---:|
| 62-bit field | 8192 | 628,385,351 | 49,364 |
| P-224 | 2048 | 41,002,860 | 13,419 |
| P-224 | 8192 | All 10 retained Vieta calls reached stage deadlines | All 10 retained controls completed |

Successful numeric rows use five retained conditioned targets. The median paired
warm Vieta/lookup ratios are 12,729.6 and 3,055.6, respectively. Inverses/square
roots include their primitive multiplications; additions, hashing, sorting and
allocation are not priced by that metric. These are generic u128/BigUint
references, not measurements against the library's optimized Montgomery and
cached square-root backends or the fastest external polynomial implementation.

Charging descriptor and factor-base construction changes the comparison. P-224
conditioned cold multiplication ratios are 0.4566 at m32, 0.5514 at m128, 1.4705
at m512 and 5.1788 at m2048. The small cold reductions depend on this generic
reference's costly uncached Tonelli–Shanks setup; no fastest-method conclusion
follows. Streaming three-term Vieta also loses to the matched MITM control:
at repaired P-224/m128, its median warm ratio is 637.9 and cold ratio 8.1072.

## Complete scope and failures

Four fields (17,31,62,224 bits), m=32,128,512,2048,8192 for two terms and
m=16,64,128 for three terms. Each configuration has two warmups and ten retained
targets, alternating arm order and A/A controls. Random-rational and conditioned
sum populations remain separate; random targets are not assumed unsatisfiable.
Small curves are component fixtures without an asserted prime subgroup order.
P-224 uses the standard curve. Full field/curve/coset and base hashes are in raw
records; c=1. No general rational/isogeny tower is measured.

| Epoch | Planned arm calls | Recorded | Successful | Failed | Missing |
|---|---:|---:|---:|---:|---:|
| Original two-term primary | 960 | 960 | 948 | 12 | 0 |
| Original three-term panel, including ancillary two-term calls | 1152 | 1140 | 1140 | 0 | 12 |
| Separate full P-224/m128 fixture repair | 96 | 96 | 96 | 0 | 0 |

The twelve failures are P-224/m8192 Vieta 120-second stage deadlines (two warmups
plus ten retained). The original final three-term configuration stopped after
round 6 because its untimed fixture context inherited previous arm time. A
separately frozen source epoch corrected that lifetime and repeated the complete
configuration. It is not pooled with its incomplete predecessor. All raw rows
and original binaries remain unchanged.

Separate-process wide scalar replay checked all **2,184 successful calls**
across epochs, without a set mismatch. Original replay exits nonzero because it
also retains twelve failed rows; repaired replay succeeds. This producer replay
is not an independent signed replay certificate.

## Artifacts and reproduction

- [Four-page report and graphs](reports/local-scale/report.pdf);
  rendering source [report.typ](reports/local-scale/report.typ).
- Native summaries: [original](scale-20261115T2053Z/summary.json),
  [repair](scale-20261115T2053Z/repair-summary.json). All statistical summaries
  and exports are computed in Rust; Python only renders recorded values.
- Frozen [primary](protocol-local.json), [three-term](protocol-local-three.json)
  and [repair](protocol-three-repair.json) protocols, distinct pre-call manifests,
  Git publication receipts, compressed immutable binaries, raw outputs, isolation
  logs and hash manifests under [scale-20261115T2053Z](scale-20261115T2053Z).
- [Errata](ERRATA.md) records the expected missing-replicate findings, stale
  repair instrument commit and rejected post-call metadata amendment. The
  primary export passes canonical validation; neither three-term export is
  represented as a clean protocol check.

Replay uses the pinned executable (decompress to a temporary executable first)
or the native crate:

```sh
cargo test --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml
cargo run --release --locked --manifest-path research/vieta_coset_rust_20261115/native/Cargo.toml --bin vieta-coset-replay -- research/vieta_coset_rust_20261115/scale-20261115T2053Z/main/*.jsonl
```

Replay accepts mixed metadata/driver/isolation lines and checks query rows only.
Use `repair/*.jsonl` separately for the repair epoch. See the frozen driver and
protocols for the exact measurement commands; rerunning is a new measurement,
not an extension of these frozen bytes.

The unmodified isolated runner used CPU4 on a 5-vCPU KVM VM (AMD EPYC 9V74),
recorded 2.23 seconds other-process CPU over 1,918.50 seconds, and marked the
original interval uncontended. PSI telemetry was absent and host threads could
not all be moved. **All wall clocks are below-L2 diagnostics, wall_valid=false**;
no timing speedup is asserted. No builds/tests/heavy analysis ran during calls.
Peak RSS was 26,712 KiB. No cloud spending occurred. Git-only custody was
authorized; S3 and independent-admission gates remain unsatisfied. No canonical
benchmark-index or IC-leaderboard row is promoted.

## Degree bound and next implementation target

[LAST_FALL_BOUND.md](LAST_FALL_BOUND.md) supplies a hand argument that the exact
two-term binary power-chain lift has last fall degree at most
**2 log2(m)+2**, for p>3, nonzero c and m=2^d dividing p−1. It handles r=0 with
quadratic local algebras, without asserting radicality. At m8192 the bound is
28. Compressed degree-d projectors are proof witnesses, not executable
interpolation. The argument awaits independent or formal validation; it does
not bound solver runtime or the joint three-term system.

The next useful implementation comparison is fast polynomial multiplication
and an optimized counted Montgomery/square-root backend on these same fixtures.
The current dense schoolbook gcd/remainder implementation is the measured
scaling bottleneck. No further performance run is included here.

Sources checked: Petit–Kosters–Messeng §3.3; repository `prime_fast`,
`prime_tower`, `pkm_smooth_tower`, `fp_backend`; Huang–Kosters–Yang–Yeo,
[arXiv:1505.02532](https://arxiv.org/html/1505.02532), definitions 2.1 and 2.5.
The KB MCP was unavailable and `scripts/research-kb status` returned 127;
primary/internal sources were read directly. No novelty claim.
