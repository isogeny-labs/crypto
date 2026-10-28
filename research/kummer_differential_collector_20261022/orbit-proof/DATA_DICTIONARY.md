# Data dictionary

| File | Unit | Meaning |
| --- | --- | --- |
| `frozen-inputs.json` | study | 21 `(bits,p,r)` models and 16 canonical nonzero jump scalars per model |
| `results/bounds.csv` | model/jump | exact cyclic orbit, minimum logical labels, current explicit-state bytes, regular cached-xADD operation lower bound |
| `results/price-grid.csv` | model/jump/price cell | canonical 36-cell product: `S/M={0,0.5,1}`, `I/M={1,10,100,1000}`, logical GAE `={0,0.1,1}`; values coincide because this bound contains only priced field multiplications |
| `results/summary.json` | study | population, ranges, and explicit scope status |
| `results/validation.json` | validation | independent parser, primality, gcd, row, formula, and population checks |

`minimum_offset_labels` is mathematical within the fixed-offset closure model.
`minimum_explicit_state_bytes_current_repr` assumes one current `[u64;4]`
value per label and is not an information-theoretic storage lower bound.
`representative_lower_bound_gae` is `24*r`, the regular cached xADD
multiplication count at the representative price; field additions are exposed
in their own column. None of these fields is a full collector cost.
