# LMFDB q16 catalog slice for the degree-three screen

Source: LMFDB `av_fq_isog` export through the connected LMFDB SQL/export
interface on 2026-10-28. The exact query was:

```sql
SELECT label, poly, curves FROM av_fq_isog
WHERE g=3 AND q=16 AND p_rank=3 AND has_jacobian=1
  AND poly[2]=0 AND poly[3]=0 AND poly[5]=0
  AND poly[6]=0 AND poly[7]=4096 ORDER BY label
```

The query returned 126 class rows. The `curves` arrays contain 10,664
cataloged equations. The exported JSONL is
`catalog_all_q16_weil_classes.jsonl`, with SHA-256
`799c3f1da10b0c37d60fc0e2ed17af00f0900be8edb39baad0de3c5abada7b95`.
The export is a snapshot of cataloged equations, not an assertion that
LMFDB lists every isomorphism class or every elliptic cover. Each catalog
row retains the exact LMFDB label and Weil-polynomial coefficients.

The selection is restricted to ordinary genus-three Jacobian classes
whose degree-six Weil polynomial has only the constant, cubic, and leading
terms. That form is necessary for a Jacobian isogenous to the Weil
restriction of an elliptic curve over `F4096` to `F16`. The screen then
tests 24 branch-supported degree-three rational-function patterns per
cataloged model. Neither the SQL filter nor the pattern list is a global
test of all possible GHS transfers.
