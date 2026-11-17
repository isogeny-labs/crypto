# Rosenhain addendum data dictionary

| Artifact | Unit | Fields |
| --- | --- | --- |
| `results/rosenhain-models.jsonl` | one of 21 exact quintics | `p`, `pairs_tried`, `square_pairs`, `fp_rosenhain_map_found`, `all_correct`, `selected` with root pair, scale and root images, and two coefficient vectors |
| `results/rosenhain-summary.json` | native validation aggregate | model count, pass count, failures, square-pair value set, explicit null cost ratio |
| `results/rosenhain-manifest.json` | addendum run | frozen input hash, commands, output paths, exact scope and missing-cost status |
| `ROSENHAIN_SHA256SUMS` | artifact | SHA-256 digest of the addendum files |

All field elements are integers in `[0,p)`. `selected.sqrt_scale` satisfies
its square modulo `p`. `lambda_mu_nu` lists the other three transformed
rational roots. `coefficient_identity` is an exact six-coefficient equality,
not a probabilistic evaluation check. This gate has no speed measurement or
paired rho interval, so cost and ratio values are null.
