# H116 Frobenius-boundary v1 retained failure

`FROBENIUS_BOUNDARY_RESULT.tsv` is the unchanged output of the producer frozen
at commit `ec89b5d977`. It is not an observation of the frozen mathematical
boundary because its scalar-map positive control failed. [pcc:unverified]

The retained row reports a first difference at coefficient four and delta nine,
but also reports `scalar_identity=false`. The independent division-polynomial
verifier rejected the scalar prefix with exit code 101; its stderr is retained
in `FROBENIUS_BOUNDARY_V1_FAILURE.txt`. The verifier's independently computed
prefix agrees with the retained `V-1` prefix through coefficient 14 and would
first differ at 15, but that value is not substituted into the v1 result.
[pcc:unverified]

Code inspection found one exact formula error. For a rational input `X=N/D`,
the duplication numerator is

```text
N^4 - 2A N^2D^2 - 8B ND^3 + A^2D^4.
```

The frozen producer subtracted the last term. Version two may change only this
sign and output path; it keeps the fixture, retained map, precision, controls,
first-difference prediction 15 and delta prediction 10 unchanged. Version-one
files and hashes remain byte-equal. [pcc:unverified]
