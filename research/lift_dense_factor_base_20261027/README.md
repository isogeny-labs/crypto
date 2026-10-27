# Lift-dense factor bases: twist avoidance through the trace

2026-10-27. Results report: [`REPORT.pdf`](REPORT.pdf) (source
[`REPORT.typ`](REPORT.typ)). Protocol, frozen before the run at `80ac6c421`
with Amendment 1 at `397f055bf`: [`PROTOCOL.md`](PROTOCOL.md). Run:
[`reports/run-20261027T0356Z/`](reports/run-20261027T0356Z/).

**Question.** On a binary curve y² + xy = x³ + ax² + b, a nonzero x lifts
to E(F_q) iff Tr(x + a + b/x²) = 0; otherwise it lifts to the quadratic
twist. Twist points cannot carry relations
([`twist_obstruction_20261026`](../twist_obstruction_20261026/README.md)),
so the twist-lifting abscissae of a factor-base subspace V are dead weight.
Relation yield is exactly proportional to |V ∩ L|^m. Can V be chosen to
avoid the twist?

**Answer.**
- **Composite degree N = d·n, n ≥ 3: yes, completely, by the trace.**
  - Let u satisfy Tr_{N/d}(u) = Tr_{N/d}(√b/u) = 0. Then on the F_{2^d}-line
    u·F_{2^d} the lift condition collapses to the constant Tr(a).
  - When Tr(a) = 0, every abscissa of the line lifts. That gives 2^n relations
    per Gaudry–Diem decomposition trial, against the standard base
    x ∈ F_{2^d}, at the same descent shape.
  - When Tr(a) = 1, the whole line lies on the twist.
- **Prime degree (ECC2K-130): no.** There is no subfield, so the
  construction does not exist. Exact counts show that lift sets *repel*
  linear structure: they contain fewer all-lift subspaces than random sets
  of equal density.

| check | result |
|:--|:--|
| twisted line lifts (Tr(a) = 0 / Tr(a) = 1), N ∈ {15, 16, 18, 20, 21}; u exists on 0/32 (n = 2), 11/24 (n = 3), 16/16 (n = 4) | 100 % on every curve with a u / 0 % on every curve |
| u-count rule (n = 2: none unless √b ∈ F_{2^d}; n = 3: 0 or 2(2^d − 1); n = 4: multiple of 2^d − 1) | holds on 72 of 72 curves (H-B) |
| m = n yield, twisted / standard (Tr(a) = 0) | measured 4.65–35.3 against predicted 4.94–21.1 per curve; pooled measured/predicted per cell 0.906–1.022. H-A (Poisson rule) failed on 25/87 rows; H-A′ (empirical variance, Amendment 1) fails on 1/87 rows (z = −4.35), all pooled cells pass |
| c2pnb208w1, c2pnb368w1 (Tr(a) = 0) | 180 / 330 valid u ∈ F_{2^16} (π_16-stable); 40/40 sampled line elements lift (standard base: 23/40, 25/40) |
| c2pnb176v1, 272w1, 304w1 (Tr(a) = 1) | the same lines lift 0/40: inapplicable by the trace obstruction |
| upstream C1 cost model (2LP), applicable rows | time / memory-(a) / mesh-a: +0.50 to +0.78 bits; memory-(b) metrics: −0.51 to +0.57 bits (**model output, not a measurement**); falling back to a half sub-base recovers the standard rate, so the option is weakly dominant |
| prime-degree repulsion, exact all-lift 4-subspace counts / E₄ (curves vs density-paired nulls) | N = 11: 0.921 ± 0.018 vs 1.027 ± 0.053, one-sided permutation p = 2·10⁻⁵ (**confirmed**, H-C); N = 9: 0.68 vs 1.02; N = 13: 0.981 vs 0.994, p = 0.019 |
| largest all-lift subspace, prime N ≤ 13 (exact) | curve median ≤ null median (4/4, 4/5, 5/5 at N = 9, 11, 13); planted subspaces recovered 100 %. Above N = 13 the budgeted search found 0 of the planted subspaces, so those values are 'found under budget' only |

**What it means for the targets.**
- **ECC2K-130** (N = 131 prime): no gain. Two facts are theorems:
  - the Weil bound caps all-lift dimension at N/2 + 1;
  - there is no subfield.

  One fact is measured toy evidence: lift sets contain fewer all-lift
  subspaces than random sets.
- **c2pnb208w1 and c2pnb368w1:**
  - The relation probability of the existing Gaudry–Diem n = 4 lane rises
    16×, and the base doubles.
  - Under that lane's own double-large-prime cost model, this is +0.5 to
    +0.8 bits on the time, memory-(a) and mesh-a metrics, and −0.5 to +0.6
    bits where memory model (b) charges the larger base.
  - It is a cost-model change, with c_trial assumed unchanged. It becomes a
    measured improvement only when the C1 toy ladder runs with both bases.
- **Basic Gaudry without large primes:** the model is LA-dominated, so a
  doubled base costs −2 bits there. Use 2LP.
- There is no leaderboard, benchmark or rho row, because no ECDLP cost was
  measured.

## The trace homomorphism, in one place

The study records where the trace acts and what each use is worth:

| use of the trace | what it gives | status |
|:--|:--|:--|
| Kummer map E(F_q)/2E → F_2, P ↦ Tr(x_P) + Tr(a), as a sum constraint | Kosters–Yeo trace equation, one linear equation; falls at degree 2 | closed, `fall_degree_bounds_20261008` |
| any homomorphic sum filter | ≤ log₂(cofactor) = 2 bits on ECC2K-130 | closed, LEAD-ICB-cofactor-torsion-quotients |
| Tr_{q^k/q} on points (trace-zero, extension embedding) | kills the rational line or enlarges the problem | closed (Balasubramanian–Koblitz; extension-embedding lead) |
| Tr_{N/d} on the factor-base line, as a membership condition | lift condition constant on u·F_{2^d}: 2^n relation yield | **new here**, composite N, Tr(a) = 0 |
| the same at prime N | no subfield; lift sets repel subspaces | **new here**, negative |

## Layout and reproduction

```sh
cargo run --release -- full reports/run-<UTC>/                     # ~27 min, 4 threads
cargo run --release -- lines reports/run-<UTC>/lines_amend1/       # Amendment 1 rerun
python3 -I c2pnb/c2pnb_lines.py <binary-curve-params.txt> reports/run-<UTC>/c2pnb_lines.json
python3 c2pnb/c2pnb_delta.py reports/run-<UTC>/c2pnb_delta.json
python3 summarize.py reports/run-<UTC>/ && python3 plot.py reports/run-<UTC>/
```

- `src/main.rs`: std-only harness.
  - exact unpruned subspace counting (N ≤ 13);
  - budgeted kmax search;
  - planted, null and structured arms;
  - twisted-line construction with exact u-counts;
  - point-arithmetic yields.
- `c2pnb/`:
  - real-parameter checks;
  - cost-model delta;
  - the upstream model, copied unchanged (sha256 in `FREEZE.sha256`);
  - the input parameter file and its sha256, kept in the run directory.
- `summarize.py`: scoring against PROTOCOL §4 and Amendment 1.
  `plot.py`: layout only.

## Provenance notes

- `summarize.py` was changed after the run, in two places, both recorded
  in git history:
  - the Amendment 1 scoring (H-A′);
  - a post-hoc correction of the m = 2 instrument check, which is not a
    decision rule. Sampled targets exclude O and T, so the prediction drops
    the k pairs {P, −P}: 2k²/(#E − 2).

  The frozen version's sha256 is in `FREEZE.sha256`. The as-registered
  H-A verdict ("FAIL", 25/87 rows) is kept in `summary.json`.
- The resume checkpoint could not be uploaded: this session has no AWS
  profile or upload token. The pushed freeze commit served as the durable
  checkpoint.
- The lead atlas lives in aburan28/crypto-autoresearcher and cannot be
  pushed from this session. The new lead and its negative prime-degree half
  should be added there: LEAD-ICB-factor-base-subspace-choice and
  LEAD-ICB-composite-degree-c2pnb.
