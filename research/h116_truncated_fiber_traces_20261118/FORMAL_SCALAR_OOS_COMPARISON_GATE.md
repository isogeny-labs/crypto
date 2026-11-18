# H116 formal-scalar OOS comparison source gate

Status: bound after native oracle generation and before candidate comparison.

```text
a84761b550a4118c7c2ec793cce71d534118151a20c10ee4e144ab8b44546fc4  h116_formal_scalar_oos.py
8823c6af2e9f1af45ab098d0469448520a42a0d28c68022f4f09ca288b637b33  h116_formal_scalar_oos_verify.py
8fb0dad351eb1b7948e42f553b6e9ff7f33ae91527e4bd09d8566ed767fe4eaf  FORMAL_SCALAR_OOS_ORACLE.json
```

`h116_formal_scalar_oos.py` checks the frozen candidate and cohort hashes,
independently repeats deterministic model selection, and computes every
available candidate prefix before opening the oracle path. It then compares
the already fixed candidates with the retained full maps.

`h116_formal_scalar_oos_verify.py` does not import either candidate source. It
independently repeats model selection, full-map quotient division, recurrence,
the transformed first-order residual, the cleared map identity, target scaling
and numerator mutations using only retained JSON. Both sources pass Python
bytecode compilation before the comparison run. [pcc:unverified]
