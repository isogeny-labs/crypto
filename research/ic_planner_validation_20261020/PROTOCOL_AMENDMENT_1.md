# Amendment 1: repeat the validation on registered curves

Frozen 2026-10-20, after sessions `c24`, `c28` and `c30` of
[`PROTOCOL.md`](PROTOCOL.md) and before any session of this amendment.
`PROTOCOL.md` itself is unchanged.

## Why

AGENTS.md §11 requires every curve a document names to be in
`docs/curves/registry.json`, and `scripts/check_curve_names.py` enforces it in
CI. The three `prime_search` curves of the original protocol are not
registered. Registering them means regenerating the registry, which `main`
carries stale: 2,572 curves against the 8,034 a fresh build finds. That
cascades into `docs/curves/covers.json` and into
`scripts/build_cover_transfer_assessments.mjs`, whose ledger asserts exactly
2,572 curves. That is a registry decision outside this change. So the original
sessions stay in the repository and are reported without naming their curves,
and the identical design is repeated here on three curves that are already
registered.

## Curves

Registered prime-order curves (cofactor 1). None is in the planner's evidence:
the Table B ladder, the 20-bit holdout, or the 32-bit studies.

| spec | curve | source | log2 r | planned base |
|:--|:--|:--|--:|--:|
| `specs/r24.json` | `icv1-fp24-tm5099-a6f137c6` | `research/ic_frontier_campaign_20261015/prime_curves.json` | 24.00 | 128 |
| `specs/r26.json` | `icv1-fp26-tm1775-7e8fb6df` | `docs/curves/sources/generated.json` | 25.35 | 175 |
| `specs/r28.json` | `icv1-fp28-t28201-65099091` | `research/ic_frontier_campaign_20261015/prime_curves.json` | 28.00 | 323 |

Each is built with `prime_explicit` from its registered parameters and
generator. The planner (`ic-planner/1`, unchanged) gives the plan, and its
rho outlooks are 12.23, 14.45 and 20.08.

## Everything else

Arms, targets (8, seed 20261021), rounds (3 + 1 warm-up), order, isolation,
measures, predictions P1–P5 and the decision rule are exactly those of
`PROTOCOL.md`. The amendment's sessions are the primary result. The original
three sessions are a supplementary replication: reported by construction
(`prime_search` bits and seed) and never by slug, with the same analysis.
