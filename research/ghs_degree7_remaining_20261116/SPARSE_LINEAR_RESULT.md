# Sparse and repeated-linear completion: trace +22

The parameter-adapted sources at commit `6b863a236f75` completed both
source-orbit representatives on the fixed carrier.

| arm | mask | systems or assignments | exact identities | CPU s |
|---|---:|---:|---:|---:|
| sparse: `1,Qi,Qi³,QiQj` | 68 | `243,405,405,270` | 0 | 39.039 |
| sparse: `1,Qi,Qi³,QiQj` | 84 | `243,405,405,270` | 0 | 39.537 |
| linear: `H²,H⁴,H⁶,QiH²,QiH⁴` | 68 | `58,806` per first shape; `98,010` per second | 0 | 1.901 |
| linear: `H²,H⁴,H⁶,QiH²,QiH⁴` | 84 | same | 0 | 1.837 |

The [sparse 68](sparse_68.json), [sparse 84](sparse_84.json),
[linear 68](linear_square_68.json), and
[linear 84](linear_square_84.json) receipts retain all per-shape
candidate counters and source hashes. Sparse source SHA256 is
`9fdbb0d8ee1563ad058758f62c67f49631b89f5406977c3bf4261c78cfd92555`;
linear source SHA256 is
`35cd152c3da138ffcdfa4b5f9e0add686e58d6ec6d52332888f2057d887bacc4`.
The sparse `QiQj` arm checked 65,340 nonzero leading-coefficient choices per
mask. Every candidate that passed its degree/coprimality filter received the
exact polynomial square check.

This finishes the two source masks in these particular denominator classes.
The complete degree-seven conclusion needs the independent denominator
partition argument and the existing quadratic-square receipts; it is stated
separately. [pcc:unverified]
