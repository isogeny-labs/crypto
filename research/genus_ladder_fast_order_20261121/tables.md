# Tables (`[pcc:unverified]`, read from analysis.json and timing.jsonl)

## FPM (primary) over matched rho, genus 3

| p | 2LP+pre | 2LP+pre+SGE(8,3) | SGE band (FPM) |
|--:|--:|--:|---|
| 8209 | 0.591 [0.459, 0.846] | 0.290 [0.224, 0.428] | below |
| 16411 | 0.741 [0.534, 1.177] | 0.383 [0.266, 0.726] | below |
| 32771 | 0.537 [0.422, 0.687] | 0.283 [0.221, 0.361] | below |
| 65537 | 0.745 [0.596, 1.218] | 0.416 [0.336, 0.678] | below |
| 131101 | 0.850 [0.590, 1.385] | 0.507 [0.354, 0.823] | below |
| 262147 | 0.460 [0.261, 0.973] | 0.293 [0.167, 0.623] | below |

## Order search, wall per order computation (single thread, `genus_ladder run --threads 1`)

| p | g | method | orders tried | wall per order (s) |
|--:|--:|---|--:|--:|
| 8209 | 3 | enumerate | 6 | 1.661 |
| 8209 | 3 | bsgs1 | 6 | 0.155 |
| 16411 | 3 | enumerate | 6 | 6.702 |
| 16411 | 3 | bsgs1 | 6 | 0.301 |
| 32771 | 3 | enumerate | 6 | 26.871 |
| 32771 | 3 | bsgs1 | 6 | 0.56 |
| 65537 | 3 | bsgs1 | 4 | 1.263 |
| 131101 | 3 | bsgs1 | 4 | 2.832 |
| 262147 | 3 | bsgs1 | 4 | 5.942 |
| 524309 | 3 | bsgs1 | 4 | 12.05 |
| 1048583 | 3 | bsgs1 | 4 | 23.189 |
| 8209 | 4 | enumerate | 3 | 5.903 |
| 8209 | 4 | bsgs1 | 3 | 25.812 |
| 16411 | 4 | enumerate | 3 | 17.446 |
| 16411 | 4 | bsgs1 | 3 | 78.214 |
