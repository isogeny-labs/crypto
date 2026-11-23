# Fixed-cofactor tensor-contraction IC against rho (2026-11-07)

Counts-only measurement of `ic.tensor_contraction_cofactor_v1` (m = 2)
against the matched rho reference `rho.negation` on the three registered
16-bit fixed-cofactor curves `toy_curve_with_cofactor(16, 20261021, c)`.

| c | slug | r |
|--:|---|--:|
| 2 | `icv1-fp16-tm166-90917c6c` | 32843 |
| 4 | `icv1-fp16-tm124-317fecaa` | 16411 |
| 8 | `icv1-fp16-tm328-cbf43d6b` | 8231 |

- Protocol: `protocol.json` (experiment.protocol/v1, frozen before any data).
- Specs: `docs/ecbench/specs/tensor-ic-cofactor-c{2,4,8}-16-s{1,2}.json`.
  Each curve has two shards of 4 targets, and each shard has its own
  `target_seed`. A single tensor target is estimated at 350–490 s on the
  registered curves, so 8 targets do not fit one 3600 s EKS request. Each spec
  has one measured round and no warm-up, because S is deterministic per
  (workload, seed).
- Arms: `rho-neg` (reference), `tensor-cof-m2` (candidate) and `rho-neg-aa`
  (A/A control).
- Decision: per curve, with both shards pooled. The paired ratio of summed S,
  tensor over rho, needs a 95% interval that excludes 1 across the §5a band.
  Holm correction applies over the three curves.
- Scope: toy sizes, exponential counters, no [Ope26] solver. No wall-time
  claim is made, and nothing here bears on deployed curves.

Status: frozen, not run. EKS submission is blocked on a fresh checkpoint
upload token.
