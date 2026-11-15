# Endomorphism-degree census (2026-11-12)

Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen by hash before any data
([`frozen.sha256`](frozen.sha256), re-verified after the run), with one dated
reporting amendment in [`AMENDMENTS.md`](AMENDMENTS.md). Code:
`src/bin/endo_degree_census.rs`, `src/cryptanalysis/chained_transfer/endo_census.rs`.
Bench record: `benchmarks/records/20261112T1558Z-endo-degree-census.json` (open).
Every number below is an unverified assertion produced by that run
(`[pcc:unverified]`); none is signed by pcc. Nothing here is a claim about
index-calculus cost.

## Verdict against the frozen rule

- Rule 1 (validity) holds: all 8800 control curves are detected with the right
  `d_K` and the right crater status, the 8 unit tests pass, and the largest
  `|z|` over the tested null sizes is 1.447 [pcc:unverified].
- Rule 2 fires **as written**: 185 registry rows of (a) have `p >= 2^30`,
  `|d_K| <= 44`, `d_K` not in {-3, -4}, and are not known non-crater
  (141 distinct `(p, t)`) [pcc:unverified]. The program prints
  `WORTH_RUNNING_ON_NATURAL_CURVES`.
- Read what those rows are. 184 of the 185 are decidable and all 184 are on the
  crater [pcc:unverified]; they are the lab's own CM-constructed curves (for
  example `D = -19` with a Frobenius order of large conductor), not curves anyone
  deployed. Restricted to (b), the named and published curves, the only
  candidate is `bls/Bandersnatch` (`d_K = -8`, class minimum degree 2, 255 bits;
  its crater status is "unknown" because the registry row records no `a`, `b`)
  [pcc:unverified]. The honest summary is outcome 3 in spirit: beyond `j = 0`
  and `j = 1728`, curves with a degree `<= 11` endomorphism exist **only by
  construction**, and the registry already holds such curves for
  `D = -7, -8, -11, -19, -43` at `p` up to 61, 255, 60, 170, 61 bits
  (counts below) [pcc:unverified]. The small-`D_E` experiment does not need any
  new curve construction at the sizes a solver can reach.

## Null population: the prediction was right

Uniformly random `(a, b)` over random primes of exactly `b` bits, seed
20261112, order by the crate's Shanks-Mestre counter; zero counter failures at
every size [pcc:unverified]. "Expected" is the exact formula (sum over the
realised primes), frozen in `predict.py` before the run.

| bits | curves (ordinary) | hits `|d_K|<=44` | fraction | expected hits | frozen prediction | z |
|---|---|---|---|---|---|---|
| 12 | 30000 (29701) | 2172 | 7.313e-2 | 2206.7 | 7.15e-2 | -0.78 |
| 16 | 100000 (99786) | 1864 | 1.868e-2 | 1803.3 | 1.88e-2 | 1.45 |
| 20 | 100000 (99941) | 422 | 4.222e-3 | 442.3 | 4.29e-3 | -0.97 |
| 24 | 100000 (99985) | 128 | 1.280e-3 | 115.1 | 1.157e-3 | 1.20 |
| 28 | 50000 (49997) | 16 | 3.200e-4 | 14.2 | 3.14e-4 | 0.47 |
| 32 | 20000 (20000) | 1 | 5.0e-5 | 1.54 | 6.98e-5 | -0.43 |
| 40 | 2000 (2000) | 0 | 0 (upper 95%: 1.9e-3) | 0.007 | 4.03e-6 | -0.09 |

[pcc:unverified] (every row). Sizes 12 to 28 are "tested" (expected hits at
least 5) and agree with the exact formula; 32 and 40 bits are consistent but
have no power, as the protocol said. The exact formula was also checked against
an exhaustive enumeration of every `(a, b)` for `p = 101, 103` (unit test) and
`p = 101, 211` (`predict.py` development) [pcc:unverified]. Scaling:
`P(|d_K|<=44) ~ C/sqrt(p)`, `C` near 4 for `p` around 1.5 times `2^(b-1)`
(ratios 3.7 to 4.5 across sizes) [pcc:unverified]. For a 256-bit curve that is
about `4 * 2^-128`. See `census.svg`.

Per-discriminant counts of the 2172 hits at 12 bits and 1864 at 16 bits are in
`census.json` (`hit44_by_d`); `d_K = -3` and `-4` carry the most, as the class
numbers and `1/sqrt|d|` weights predict [pcc:unverified].

### Degree buckets, null (minimum endomorphism degree over the isogeny class)

| bits | 1 | 2 | 3 | 4-5 | 6-8 | 9-11 | 12-1100 | > 11, unresolved |
|---|---|---|---|---|---|---|---|---|
| 12 | 659 | 369 | 176 | 338 | 273 | 357 | 15905 | 11624 |
| 16 | 549 | 340 | 136 | 297 | 243 | 299 | 15320 | 82602 |
| 20 | 129 | 65 | 24 | 70 | 47 | 87 | 3774 | 95745 |
| 24 | 45 | 24 | 7 | 22 | 17 | 13 | 1006 | 98851 |
| 28 | 6 | 3 | 1 | 3 | 2 | 1 | 134 | 49847 |
| 32 | 0 | 0 | 0 | 0 | 0 | 1 | 16 | 19983 |
| 40 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2000 |

[pcc:unverified]. Counts are over ordinary curves; the buckets "12-1100" and
"> 11" come from a search to `|d_K| <= 4400` (so "> 11" at these sizes means
`|d_K| > 4400`, minimum degree above 1100). Degree 1 is an automorphism
(`d_K = -3, -4`).

## The class bound is not the curve: the crater (post hoc)

The census reports the **class** minimum degree. For a given `E`, `End(E)` has
conductor `c | f`, and its minimum degree is about `c^2 |d_K| / 4` unless
`c = 1`. The crater is decided exactly only for `h(d_K) = 1`. Among null hits,
the crater is rare [pcc:unverified]:

| bits | hits found on the crater (decidable only for `h(d_K)=1`) | `f / sqrt(p)` over hits (median) |
|---|---|---|
| 12 | 26 of 2172 hits | 0.445 |
| 16 | 7 of 1864 | 0.450 |
| 20 | 0 of 422 | 0.444 |
| 24 | 0 of 128 | 0.480 |
| 28 | 0 of 16 | 0.516 |

[pcc:unverified]. `posthoc.py` (written after the data, labelled post hoc)
computes the exact probability that `End(E) = O_K`: with the same Hurwitz
derivation it is the `c = 1` term only. For `h = 1` discriminants it gives
1.196e-3, 7.74e-5, 4.62e-6, 2.7e-7 at 12, 16, 20, 24 bits, i.e. 35.5, 7.7, 0.46
and 0.03 expected crater hits against 26, 7, 0, 0 observed [pcc:unverified].
It falls by a factor of about 16 per 4 bits: **`P(E on the crater) ~ 1/p`**, not
`1/sqrt(p)`. The exact probability that `|D_E| <= 44` (orders of conductor
`c | f` with `|d_K| c^2 <= 44`) is about three times the crater probability
(3.71e-3 at 12 bits, 2.38e-4 at 16 bits) [pcc:unverified].

So for a random curve, the class has a degree `<= 11` endomorphism with
probability near `4/sqrt(p)`, but `E` itself has one with probability near
`1/p`. In a class-level hit, the isogeny that reaches the crater has degree
`c`, and `f` is about `0.45 sqrt(p)` at the median, so the walk is as long as
`sqrt(p)`-scale, the size of a rho run. This is the owner's point in numbers: a
small-degree isogeny supplies no endomorphism unless the cycle closes, and here
the cycle closes only after a `sqrt(p)`-scale descent.

## Registry (a) and named curves (b)

Skipped, with reason: 102 binary, 68 Koblitz, 1371 extension-field and 4
subfield rows (1545 of 22278 rows, none prime-field); no prime row failed the
Hasse check, so 20733 prime rows were analysed, 3 supersingular
[pcc:unverified]. The registry is **not random**: 938 distinct `(p, t)` carry
the 20733 rows [pcc:unverified], and it is built from volcano, conductor-gap,
BN/BLS and testbed curves chosen for earlier experiments.

| population | rows (ordinary) | `|d_K|<=44` | fraction |
|---|---|---|---|
| a: all prime rows | 20733 (20730) | 2184 | 10.5% |
| a: distinct `(p, t)` | 938 (935) | 273 | 29.2% |
| a: `p <= 16` bits | 198 (195) | 20 | 10.3% |
| a: 17-32 bits | 19073 | 1012 | 5.3% |
| a: 33-64 bits | 1161 | 1102 | 94.9% |
| a: 65-128 bits | 15 | 11 | 73.3% |
| a: 129-256 bits | 215 | 18 | 8.4% |
| a: > 256 bits | 71 | 21 | 29.6% |
| a: `p >= 2^30` | 1952 | 1152 | 59.0% |
| b: named standard | 54 | 6 | 11.1% |
| b: pairing-friendly (BN, BLS) | 29 | 29 | 100% |
| b: other named (SafeCurves, Pallas, ...) | 40 | 4 | 10.0% |

[pcc:unverified] (every row). The 33-64 bit tier is 95% hits because it holds
the lab's CM-constructed families; that is a description of the registry, not
an estimate for any random curve. For comparison, the exact null expectation
at 24 bits is 1.15e-3 [pcc:unverified].

Hits by discriminant for `p >= 2^30` (rows / distinct `(p,t)`):
-3: 173 / 52; -4: 79 / 3; -7: 232 / 7; -8: 139 / 3; -11: 74 / 4; -19: 403 / 118;
-43: 52 / 15 [pcc:unverified]. Bit ranges of the non-(-3,-4) families:
-7: 46 to 61 bits, -8: 53 to 255, -11: 47 to 60, -19: 37 to 170, -43: 42 to 61
[pcc:unverified]. Note few distinct `(p, t)` for -4, -7, -8 and -11 (3, 7, 3,
4): many rows are other models of the same few classes.

Named curves (orders taken from `docs/curves/registry.json`, whose standards
rows cite `docs/curves/standards/registry.json`, `src/ecc/curve.rs` and
`src/ecc/curve_zoo.rs`; the P-256 and secp256k1 `p` and `n` literals were also
found verbatim in `src/ecc/curve.rs` and agree with the registry row) are
[pcc:unverified]:

- `d_K = -3`, on the crater (`j = 0`, automorphism of order 6): secp160k1,
  secp192k1, secp224k1, secp256k1, wap-wsg-idm-ecid-wtls8, -wtls9; every BN
  curve (bn158 to bn638, Fp224BN to Fp512BN, bn254, Fp254BN) and every BLS12,
  BLS24 and BLS48 G1 curve in the registry; Pallas, Vesta, Tweedledum and
  Tweedledee. That is 6 + 28 + 4 = 38 rows.
- `d_K = -8`, class minimum degree 2: `bls/Bandersnatch` only.
- No named standard curve has `d_K = -4`, and none has `-7, -11, -15, ..., -43`.
  The rest of the 54 named standard curves (P-192 to P-521, brainpool,
  Curve25519 and Curve448 and their forms, GOST, SM2, FRP256v1, prime239vN,
  secp112 to 128 and 160r) have `|d_K| > 44`, so their class has no
  endomorphism of degree `<= 11`; the class minimum degree is not computed
  (needs the factorisation of `Delta`) [pcc:unverified].

## Controls

Positive: `D = -3, -4, -7, -8, -11, -19, -43`, 200 curves each at 16, 24, 32
and 40 bits with a random twist: 5600 of 5600 detected with `d_K = D`, all on
the crater. Negative (non-maximal): `D = -12, -16, -27, -28`, same sizes: 3200
of 3200 detected with `d_K = -3, -4, -3, -7` and all reported **not** on the
crater; counter failures 0 [pcc:unverified]. (Per-`D`, per-size rows are in
`census.json` and `run.log`.)

## What this implies for `small_de_endo_fb_20261112`

1. **How often does a low-degree endomorphism exist?** On a random curve, in
   the isogeny class, with probability near `4/sqrt(p)` (`d_K` within 44); on
   the curve itself, near `1/p` [pcc:unverified]. For cryptographic `p` neither
   is observable. Every named curve with a degree `<= 11` endomorphism in the
   repository other than `d_K = -3` is `bls/Bandersnatch`, designed for it.
2. **Beyond `j = 0` and `1728`, the experiment is a designed-curve experiment.**
   It must run on CM-constructed curves (one-time construction cost, never in
   a per-instance total), and the registry already has them for `D = -7, -8,
   -11, -19, -43` at 37 to 61 bits and `D = -8` at 255 bits [pcc:unverified].
   A claim from it applies to a designer choosing such a curve, not to deployed
   standard curves.
3. **Choose `p` with `f` small.** A class-level hit on a random curve needs a
   `sqrt(p)`-scale descent; use curves with `End(E) = O_K` (decided exactly by
   `j(E) = j_D mod p` for `h = 1`, as in this program), so there is no
   isogeny cost.
4. The census says nothing about whether a factor base closed under the
   endomorphism is of useful size; the experiment's own decision rule is
   untouched.

## Limits

- The crater is decided only for `h(d_K) = 1` (seven discriminants); for the
  other eight the repository has no Hilbert class polynomial, so those rows are
  "class bound only".
- Ten registry rows have no recorded `a` or `b` (crater cannot be computed);
  Bandersnatch is one of them.
- The null's counter could give up (twist retry, then count as failed): 0
  failures. The counter is the crate's `frobenius_trace_bsgs`; sampling is
  deterministic (`null_transcript_sha256` in `census.json`), and two runs gave
  byte-identical `rows.jsonl` and `census.json` apart from the elapsed time
  [pcc:unverified].
- 32 and 40 bits have almost no expected hits; they do not test the prediction.
- Ran locally on 4 cores in 104 seconds, not on the EKS Spot route; this is a
  small CPU job, and the task asked for the local `./target/release` binary.
  The bench record is left open.

## Files

`PROTOCOL.md`, `frozen.sha256`, `AMENDMENTS.md`, `predict.py`/`predict.json`
(pre-data), `census.json`, `rows.jsonl` (hit and named rows), `run.log`,
`census.svg` (`plot.py`), `posthoc.py`/`posthoc.json` (post hoc).
Reproduce: `cargo build --release --bin endo_degree_census &&
./target/release/endo_degree_census`.
