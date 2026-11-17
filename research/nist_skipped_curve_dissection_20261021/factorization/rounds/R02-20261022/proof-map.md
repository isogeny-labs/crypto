# R02 product proof map

Every theorem proves an exact product over `Nat`, including the explicit composite
residual in the partial row. PARI certificates separately prove primality.

| Target | Status | Declaration in `SkippedNistFactors` | Identity certificate |
|---|---|---|---|
| `p256-4p-minus1` | complete | `p256_4p_minus1_product` | [IDC1ha9e7397787ef6ad6](identity-certificates/p256-4p-minus1.json) |
| `published_224-twist2` | complete | `published_224_twist2_product` | [IDC1hba153a09201c0fcb](identity-certificates/published_224-twist2.json) |
| `published_256-6n-plus1` | complete | `published_256_6n_plus1_product` | [IDC1hc89465534930d407](identity-certificates/published_256-6n-plus1.json) |
| `published_256-7n-minus1` | complete | `published_256_7n_minus1_product` | [IDC1h33062f51f7b63266](identity-certificates/published_256-7n-minus1.json) |
| `published_256-twist2` | complete | `published_256_twist2_product` | [IDC1h46b7f67ddaf4a975](identity-certificates/published_256-twist2.json) |
| `skipped_224_1485-conductor4` | complete | `skipped_224_1485_conductor4_product` | [IDC1ha1cc0bf6d8f34d00](identity-certificates/skipped_224_1485-conductor4.json) |
| `skipped_224_5736-twist2` | complete | `skipped_224_5736_twist2_product` | [IDC1hd9715e802f498529](identity-certificates/skipped_224_5736-twist2.json) |
| `skipped_256_876-4n-minus1` | complete | `skipped_256_876_4n_minus1_product` | [IDC1h665fb7ca69035f6a](identity-certificates/skipped_256_876-4n-minus1.json) |
| `skipped_256_876-twist2` | partial | `skipped_256_876_twist2_product` | [IDC1hd8e26f4ad8a9e4a1](identity-certificates/skipped_256_876-twist2.json) |

Lean 4.35.0: nine theorems, zero failing, zero trusted native proofs; all axiom
lists empty. Gate and separate identity checks are retained in the raw archive.
