# Half-GCD and certified rational towers: results

**Mixed component result:** half-GCD reduces multiplication counts on the two large word-field cosets, but increases them at P224/m2048 and all three rational configurations. The historical default remains schoolbook and both faster-polynomial backends remain explicit. Every observation is producer-owned and unverified: PCC-20261116-c8911d [pcc:unverified].

## Fresh construction plus query

| Conditioned finite configuration | n | Newton/Euclid | Half-GCD | Lookup | Paired N/H median [observed range] |
|---|---:|---:|---:|---:|---|
| Coset word62 /8192 | 3 | 300,898,756 | 268,564,836 | 4,986,949 | 1.120395 [1.120395, 1.120395] |
| Coset word62 /16384 | 3 | 1,168,899,513 | 809,643,807 | 9,999,434 | 1.443721 [1.443721, 1.443721] |
| Coset P224 /2048 | 3 | 22,039,580 | 30,590,000 | 7,917,433 | 0.720483 [0.720481, 0.720486] |
| Rational word62 /2048 | 3 | 78,793,124 | 87,343,544 | 52,438,265 | 0.902106 [0.902106, 0.902106] |
| Rational P224 /128 | 3 | 60,967,975 | 61,121,763 | 61,070,770 | 0.997484 [0.997484, 0.997484] |
| Rational P224 /512 | 3 | 249,999,177 | 251,522,257 | 249,257,307 | 0.993945 [0.993945, 0.993945] |

All values are precomputed by the frozen Rust analyzer. N/H>1 means fewer half-GCD multiplications. These are component arithmetic counts; memory/hash/sort are unpriced. A/A half repeats and unused-backend lookup controls have ratio1 on all12 retained population/configuration groups [pcc:unverified].

## Query-only counts

| Configuration | Newton/Euclid query | Half-GCD query | Lookup query | Paired N/H |
|---|---:|---:|---:|---:|
| Coset word62 /8192 | 300,898,741 | 268,564,821 | 49,364 | 1.120395 |
| Coset word62 /16384 | 1,168,899,497 | 809,643,791 | 98,828 | 1.443721 |
| Coset P224 /2048 | 22,039,567 | 30,589,987 | 13,419 | 0.720483 |
| Rational word62 /2048 | 27,615,873 | 36,166,293 | 12,602 | 0.763580 |
| Rational P224 /128 | 327,859 | 481,647 | 1,143 | 0.680704 |
| Rational P224 /512 | 2,697,449 | 4,220,529 | 3,663 | 0.639126 |

At rational/P224/m512 the generic descriptor/fiber certification alone costs 247,301,728 multiplications, versus4,220,529 in the half-GCD query. A structural certificate for known conjugate-square towers is the next construction target; no auxiliary-isogeny-search cost or new performance result is asserted. Lookup has fewer charged multiplications than half-GCD in all six conditioned cells. The Newton baseline at rational/P224/m128 is slightly below lookup in total counts; the table preserves that outcome [pcc:unverified].

## Other retained population

| Seeded random-abscissa finite configuration | n | Newton/Euclid total | Half-GCD total | Lookup total | Paired N/H |
|---|---:|---:|---:|---:|---:|
| Coset word62 /8192 | 3 | 300,897,075 | 268,563,155 | 4,986,949 | 1.120396 |
| Coset word62 /16384 | 3 | 1,168,897,792 | 809,642,086 | 9,999,434 | 1.443722 |
| Coset P224 /2048 | 3 | 22,030,638 | 30,581,058 | 7,917,433 | 0.720401 |
| Rational word62 /2048 | 3 | 78,791,146 | 87,341,566 | 52,438,265 | 0.902104 |
| Rational P224 /128 | 3 | 60,958,788 | 61,112,576 | 61,070,770 | 0.997484 |
| Rational P224 /512 | 3 | 249,989,986 | 251,513,066 | 249,257,307 | 0.993944 |

The generator uses seeded modular reduction and canonical square-root signs; its historical population label is not an assertion of uniform curve-point sampling. Populations/representations/fields/sizes are not pooled. No identity target occurred in this main panel; identity and finite two-torsion branches remain separately tested. No confidence interval, exponent fit, elapsed-time or fastest-backend inference.

## Flow, verification and boundaries

- Primary:240 planned/attempted/successful calls,0 failures,0 missing;60 warmup calls excluded from comparative summaries,180 retained calls on36 retained targets. Wide scalar replay matches all240 calls/48 total inputs [pcc:unverified].
- Calibration:60 successful calls on12 inputs, separately replayed, excluded from primary. Environment reconnect lost its supervisors, while the original bounded driver/native processes continued without restart. Aggregate exit/start/wall and isolation receipt are unknown; preserved null-exit supervisor command. All clocks invalid for admitted timing, contended=null in its export. Pending statement9dc34f [pcc:unverified].
- Main: persistent supervisor reports19:51:06–20:11:42UTC, exit0. Isolated runner wall1233.621504883s, other-process CPU1.17s, RSS12,800KiB, detected contention=false. Missing PSI/unmovable host threads keep all clocks belowL2 and wall_valid=false [pcc:unverified].
- Every arm freshly executes descriptor/certification. Lookup builds its needed point table/hash set; finite Vieta candidates do not. External fixture construction/draw/complete scalar verification are excluded and recorded. Inverse/sqrt primitive multiplications are included; high-level counters are not double-priced.
- Immutable source/binary/analysis manifest matched after calls. Canonical preflight and main exports pass with file verification and zero findings. Source9d2106eb2a, qualifier42d0cfd08912..., main protocolac9f3009..., calibration72e70723... [pcc:unverified].
- Full shared-source qualification:12 unit groups/5 CLI tests, plus2 separate post-calibration/post-run nonempty positive integration cases, fmt/clippy pass. Root full release library preparation hit180s while compiling; no root-suite pass [pcc:unverified].

## Degree audit and representation coverage

The general runtime accepts coprime rational N/D with degN=q>=2, degD<=q-1 and total degree<=16384. It rejects noncanonical input, poles, partial/repeated fibers, field mismatch and budgets. Every sibling fiber must split distinctly. Homogeneous finite-target membership uses no interpolation or division in the quadratic algebra, and the diagonal/identity cases are separate. A genuine normalized Vélu2-map builder and nonempty prime-order toy-curve relation are included. Performance fixtures are Möbius-conjugated Kummer fibers; auxiliary-isogeny search/construction costs are not measured.

The hand bound d_F<=2*sum(q_i-1)+2 remains unverified. Pending statement640dba pins a fresh audit and a different finite V_T closure calculation: all six declared cases have literal coordinate/projector checks, first five have local action/quotient-dimension checks, including nilpotent/nonsplit/empty local-algebra controls. Mixed-degree case has no closure calculation, as declared. Independent general proof review remains external. See DEGREE_AUDIT.md and AUDIT_ADDENDUM.md; original pinned proof files are unchanged [pcc:unverified].

## Artifacts and reproduction

See report.pdf, SVG/PNG previews, PROTOCOL.md, frozen JSON protocols, run/build.json, original hash receipt, raw per-input checkpoints, supervision-loss receipt, main-supervisor.json, isolation JSONL, summary.json, replay.json, and the study-root export-next-{preflight,main}.json. Use the exact source/lockfile/compiler/build receipts and source-pinned native qualifier. Its immutable binaries are local ignored cache only, with no remote binary-archive availability assertion.

All execution is explicitly authorized local/Git-only. No cloud experiment or local S3 operation. The bench record is closed censored; prospective S3 addresses are not uploads/availability claims. No protected benchmark-index or leaderboard promotion. PR4410 remains subject to current-head gates/reviews; earlier shared artifact credential failures are not bypassed.

Sources checked: FLINTv3.3.1 nmod_poly/hgcd.c and gr_poly/hgcd.c; internal isogeny_eval/poly.rs, ic_framework/prime_tower.rs, HKYY definitions2.1/2.5, PKM section3.3 and the pinned tower argument. KB MCP absent and wrapper status/two searches127; direct primary/internal fallback. No novelty claim.

## Rust API example

```rust
use crypto_lib::cryptanalysis::vieta_coset::*;
let a = Arithmetic::new(WordField(17), Limits::default())?
    .with_polynomials(PolynomialBackend::HalfGcd);
let map = a.velu_two_map(&Curve { a: 1, b: 0 }, 0)?;
let tower = a.certify_tower(&[map], 12, 42)?;
let pairs = a.solve_tower(&Curve { a: 2, b: 2 }, &tower, &Some((7, 11)), 42)?;
// [((5,1),(7,6)), ((7,6),(5,1))]
```

The example charges one cumulative context. The experiment uses fresh construction/query contexts with separately recorded work, so the query deadline starts after construction. `PrimeField` callers must supply an actual prime modulus; primality is a caller precondition. Normalized auxiliary maps create a field subset, not a discrete-log transfer.
