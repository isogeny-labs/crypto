# Exact ECCp-131 representation-filter sweep

## Result

Verdict: `no_qualifying_filter_in_frozen_family`. The planted four-sum root was found and replayed: `true`. The published-target bounded MITM probe found a four-sum: `false`. The 16-column modular solver control recovered its known solution: `true`.

## Held-out cells

| Base | B | Selected source | Combiner | w | offset | relation hits | null hits | log2 enrichment | ratio lower95 | pre-add |
| --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| small_x | 64 | pair_sum_y | add | 6 | 120 | 130 | 118 | 0.139163 | 0.776995 | false |
| random_base | 64 | endpoint_xor | sub_either | 7 | 72 | 118 | 126 | -0.094250 | 0.658700 | true |
| small_x | 128 | endpoint_xor | target_equal | 7 | 0 | 120 | 140 | -0.221537 | 0.609251 | true |
| random_base | 128 | endpoint_xor | xor | 6 | 0 | 129 | 118 | 0.128065 | 0.770504 | true |
| small_x | 256 | pair_sum_x | add | 6 | 56 | 120 | 107 | 0.164696 | 0.778314 | false |
| random_base | 256 | endpoint_xor | xor | 6 | 88 | 137 | 138 | -0.010454 | 0.712958 | true |

## Phase ledger

| Phase | State | Evidence |
| --- | --- | --- |
| Exact curve/target validation | completed | subgroup and curve checks in native instrument |
| Factor-base generation | completed | small-x and matched scalar-random arms |
| Hyperparameter search | completed | training-only exhaustive frozen grid |
| Held-out relation filter | completed | 8,192 relation and null samples per cell |
| F4 algebraic solve | unavailable | no exact 131-bit F4 decomposition backend is dispatched |
| F5 algebraic solve | unavailable | no exact 131-bit F5 decomposition backend is dispatched |
| F6 geometric pair index | completed | exact signed MITM root probe |
| Root finding | completed | planted control plus published target probe |
| Relation solving | control completed; natural rows separate | exact modular known-answer control; `ic prime` raw runs carry natural rank |
| Published discrete logarithm | not completed | no target logarithm inferred from a bounded null |

## Flow

The local native sweep planned, attempted, and completed six filter cells, one planted root control, one published-target probe, and one modular-solver control. Local clocks are descriptive and invalid for performance claims. Exact arity-2/3 natural relation/rank runs are retained separately. Cloud L2 execution is excluded by the repository cost gate until recorded budget headroom exists.

## Reproduction

```sh
cargo run --release --example eccp131_representation_filter -- research/eccp131_representation_filter_20261102/raw/local
```

Protocol SHA-256: `6e56a761622da5ab41aaff666c248b0a28bb062893f319440e074e20de1baa4e`. Source revision: `33355ba6aed5fbb175211a757f0cc6da460f72ab`.
