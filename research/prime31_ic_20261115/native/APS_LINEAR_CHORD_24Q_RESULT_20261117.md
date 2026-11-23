# EXP-R31-APS-LINCHORD — complete cold 24Q result (2026-11-17 UTC)

**Result: negative / inconclusive optimization gate. [pcc:unverified]** The existing Rust APS one-relation index-calculus-derived solver recovered **24/24 unknown public scalar targets in each of five arms** on the actual ordinary cofactor-one curve at `p=2^31-1`. The separately implemented Rust affine auditor replayed **120/120** scalar-point equations and verified the signed APS congruences, original-versus-candidate transcripts, aggregate cold totals and the frozen gate. The new linear-chord oracle fails the preregistered 5% whole-solver gain and 18/24 paired-win conditions. This is **not** a new cheaper, non-generic ECDLP algorithm. Neither unfolded rho is a strongest-WZ reference.

## Immutable source and full provenance

- Prospective protocol: [`APS_LINEAR_CHORD_PROTOCOL_20261117.md`](APS_LINEAR_CHORD_PROTOCOL_20261117.md), published in Git commit `f401170141d1ed46fce52704888b414bf1d77a26` **before** source change and measurements.
- Original scientific source: `1df714645aa683cf975bc76fcacb9842cb4780fd`. Candidate source after modification: `d7ae92905312b45b723a28629bfaf5d3e70afee1`; independent affine auditor fixed at `42352dc4421563d122d9d681987f47801c309389`. Same unchanged Rust package and all native prerequisites in the immutable pinned checkout.
- **Passing, source-specific** [GitHub Actions run #44010498590](https://github.com/isogeny-labs/crypto/actions/runs/44010498590), job `133796089709`. Test scopes: 40/40 library, 23/23 solver example (including independent 256-pair chord-root/S3 identity), 1/1 Rust auditor unit test. Output-based gate checks run within `prime31_linear_chord_audit.rs`.
- [Raw ZIP artifact #14295409872](https://github.com/isogeny-labs/crypto/actions/runs/44010498590/artifacts/14295409872) (30-day GitHub retention; download and archive for permanent custody). Contains **198,521-byte** raw 24Q × five-arm JSON with all attempts and output logs, source SHA manifest, and independent-affine audit JSON. This summary and audit headline are separately committed to Git; raw source-independent full artifact is currently in GitHub Actions only.
- Previous first CI [#44010310623](https://github.com/isogeny-labs/crypto/actions/runs/44010310623) correctly **failed before measuring** because the separate affine auditor mishandled O+P; identity arithmetic repaired and re-tested, without changing the frozen protocol or timed candidate.

## Measured complete cold cost

All columns sum complete in-process cold calls on exactly the same 24 public Q, source setup, algebraic search, rejected rounds, arithmetic and verification paid (GitHub-hosted Ubuntu 24.04, Rust 1.100.0, 1 Rayon thread).

| Arm | Correct/24 | Cold total (ns) | Cold total (ms) |
|:--|--:|--:|--:|
| Incumbent APS window4/public-y/packed index | 24/24 | 81,680,616 | 81.680616 |
| **Linear-chord batch-inversion APS** | **24/24** | **81,006,788** | **81.006788** |
| Linear-chord APS A/A repeat | 24/24 | 81,015,446 | 81.015446 |
| Unfolded rho32 | 24/24 | 85,208,157 | 85.208157 |
| Unfolded rho64 | 24/24 | 83,519,404 | 83.519404 |

- Linear-chord whole-cold / incumbent = **0.9917504539877613**: only **0.825%** aggregate reduction, not 5%.
- Linear-chord paired wins vs incumbent: **13/24**, not 18/24; A/A aggregate drift **0.01069%**.
- **Frozen ≥5% gain + ≥18/24 wins + A/A ≤5% verdict: FAIL.**
- Candidate / rho64 = `81,006,788 / 83,519,404 = 0.969915…` (descriptive). Candidate / rho32 = `81,006,788 / 85,208,157 = 0.95069…`. Neither proves advantage against the optimized generic frontier or captures uncertainty.
- Phase decomposition is revealing: incumbent S3 oracle **42,964,022 ns** versus candidate **43,787,310 ns** (**1.916% slower**). Incumbent source setup **29,777,554 ns** versus candidate **28,376,568 ns** (**4.705% faster**). The net modest wall reduction is not isolated to the intended algebraic oracle; **do not credit the one-multiplication-per-pair reduction with an end-to-end solver gain.** The profile was interleaved, not hardware-isolated, with 24 Q and only a single shared-host trial.
- Correctness: original/new signed search attempts and solution transcripts match for all 24 Q; Rust auditor compared 48 paired APS attempt sequences and verified every complete solver log `[k]G=Q` using a **separate** affine addition/doubling implementation. This does not constitute independent signed PCC.

## Mathematical identity and scientific interpretation

For `P=(x,u)`, `T=(z,v)`, `x≠z`, the two S3 roots are `((u±v)/(x−z))²−x−z`. The old Rust pair oracle inverted `(x−z)²` and multiplied squared numerators; the candidate batched inverses of `x−z` and squared the slopes. Mathematically exact, but classic group arithmetic; pair search still explicitly visits O(B²) first/second candidates. No asymptotic ECDLP advance or new non-generic index calculus is obtained.

## Prior-lead and negative-work disposition

- **Stop** this exact unsquared-difference chord optimization for a single cold target unless isolated profiling isolates a real low-level field gain; do not rerun identical seeds until a favorable number appears. The preceding buffer-reuse attempt in PR #4484 likewise missed its frozen gate with 1.00167× measured total, so stop microallocation alone.
- Current APS incumbent from #4391/#4454 is a complete Rust one-relation solver **based on published APS 2018**, pair-MITM equivalent. Previous 74.434ms vs 79.833ms rho32 on separate 24Q was not significant; comparing absolute times across these unmatched jobs is invalid.
- Most promising **non-generic lead**: target-conditioned RR9/S9 `F=U+yV` extraction with original-ideal certificates, reusing #4466 Hermite signed-norm tests and #4228/#4349 quotient/rootfinding. First *blind* constructed-positive, natural and truly negative fibres at B4/B8, including Dickson31 negatives; then unknown-target B31 complete solving without enumerating 48,903,492 eight-point supports. Count all elimination/rootfinding, relation collection, linear algebra and failed trials.
- Previously assessed/retained negatives: autoresearcher #3301 41-lead heat map; #3305 28 Semaev alternative tests/errata; crypto #4108/#4363 optimized IC vs rho controls; censored anchored m6/m8 F4; auxiliary #1536 transport blocked; Vieta-conic no full prime31 gain. Do not retry these unchanged.
- **Strongest complete comparator still missing**: tuned Wiener–Zuccherato negation-map rho source in `src/cryptanalysis/ic_boundary.rs`, previously blocked by unrelated pinned `src/cryptanalysis/sat.rs` defects (#4468). Isolate a source-identical Rust comparator (not invented rho) and fastest mature external IC implementations, same Q, field, CPU isolation and full cold costs before any speed claim.

**Validation:** producer GitHub Actions tests and algebraic replay, [pcc:unverified]; no independently signed PCC, external fastest-tool victory, or leaderboard promotion. Other PR CI workflows may fail on unrelated base files, including a duplicate `run` key in `.github/workflows/curve-covers.yml`; this scoped experiment's CI is green.
