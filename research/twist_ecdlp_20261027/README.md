# Can the quadratic twist help the ECDLP on E? (2026-10-27)

**Status.** Measured negative result at small sizes, from toy fields up to
40 bits. No arm that uses the twist beat its matched baseline in counted
operations. The report with its graph is `report.pdf`, built from
`report.typ` and `summary.svg`.

## Question

Let E/F_q have quadratic twist E'. Over F_{q^2}, E(F_q) and E'(F_q) are the
+1 and −1 Frobenius eigenspaces of E(F_{q^2}). The study tests whether that
structure gives a non-generic route to the ECDLP on E.

The proposed **separation lemma** is the following. Let R ∈ E(F_q) and write
R = A + B, where A is a sum of E(F_q) points and B is a sum of E'(F_q) points.
Then 2B = O. So B = O whenever E has no rational 2-torsion, and the twist
carries no information about log R.

## Method

There is one directory per idea. Each holds:

- `PREREGISTRATION.md`, frozen before any data was computed;
- the PARI/GP scripts;
- `RESULTS.md` and `results.json`;
- `VERIFICATION.json`.

`VERIFICATION.json` records an independent replication (Haiku), two
adversarial verifiers (Sonnet, one for mathematical correctness and one for
controls and statistics), and, where the first replication did not match, a
reconciliation diagnosis. The run, replication and verification agents were
Sonnet and Haiku.

The primary cost metric is operation counts. Wall times were taken on a
shared 4-core host and are contention-affected.

## Results

| Idea | Status | Key number |
|---|---|---|
| E0 separation lemma | supported | 0 violations in 8,254 + 34,498 classes; 2B = O always, and B = O iff E(F_q)[2] = O |
| E1 + G3 mixed-solution census (Semaev, x-only) | structure supported; rate inconclusive at 2^20 and above | 0 mismatches between the mixed solutions and the predicted family over 91,200 + 508,800 targets; measured/reference ratio 0.9965 (12 bit), 0.9935 (16 bit), 0.85 [0.29, 1.47] (20 bit); 24 bit unresolved |
| E2 twin-target amortization | supported, and the gain hypothesis is refuted | a joint E + E' table costs 1.000–1.062× separate runs |
| E3 F_{q^2} eigen-decomposition | supported (a loss) | 53.3–2,812× more work per E-relation (33 powered cells) |
| E4 Koblitz K0 ↔ K1 pair, τ-orbit factor base | supported (no gain) | identical 773,404 K0 relations, at 2.0246× the counted solves (21.4 M vs 43.2 M) |
| E5 Kummer-line multiplicative walk | inconclusive | 4.6× (K-mix) and 6.2× (K-pure) the baseline cost at I/M = 3, but falsifiers F1/F2 fired and a major statistics issue is unresolved |
| E6 genus-2 gluing of E × E' | supported (construction only) | #Jac(C) = #E·#E' on 580/580 records; "no gain" comes from a cost model and is not measured |
| G1 negation-map rho baseline | supported | solves 100%; the K-mix walk costs 1.287–5.907× its cost, no row below 1 |
| G2 decoupled A/B projection arm | inconclusive | "≥ 1.0069× baseline" is an identity of the metric (major statistics issue) |

**Replication.**

- E0, E3 and G3 reproduced on independent replication.
- For E1, E2, E4 and E6, the original mismatches were diagnosed as
  incomplete replication runs.
- For E5 and G1 the cause was unseeded randomness, which is now fixed in the
  scripts.
- For G2 the differences were timing only.

These seven were **not** re-replicated after diagnosis.

## Conclusion (scoped to the sizes tested)

The twist gives exact structure: the separation lemma, the predicted family
of mixed solutions, and the gluing of E × E' to a genus-2 Jacobian. It gives
no measured speedup for the ECDLP on E. For R ∈ E(F_q), the twist component of
any decomposition lies in E[2], so it carries no information about the
logarithm.

## Open

- The mixed-solution rate at 2^20–2^24 and at m ≥ 4.
- Factor bases over extension fields.
- A measured (not modelled) genus-2 Jacobian arm.
- Pricing every arm at 0, 0.1 and 1 GAE per operation (AGENTS.md §5a).
- A cheaper x-only step.
- Re-replication of E1, E2, E4, E5, E6, G1 and G2 after reconciliation.

## Files

| file | what it is |
|---|---|
| `report.pdf`, `report.typ`, `summary.svg`, `summary.png`, `plot_summary.py` | report, graph, plotting-only source |
| `E*/`, `G*/` | per-idea preregistration, scripts, results and verification |
| `EXCLUDED_RAW_SHA256.txt` | SHA-256 of 4,303 raw output files kept out of the repository for size: per-target census outputs and files over 1 MiB |

Neither the leaderboard nor `benchmarks/` changes: this study claims no
improvement.
