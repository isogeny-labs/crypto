# Observed AES block statistics

These commands measure caller-supplied 16-byte plaintext and ciphertext blocks. They accept a local JSON file or a checked `store:` artifact reference. Every corpus is bounded to 64 MiB and 100,000 observations. Plaintexts and ciphertexts must each be distinct, as they are for a block permutation.

## Differential activity

`crypto cryptanalysis aes-differential-statistics --input FILE` accepts `input_difference_hex` (32 hex digits) and a `pairs` array. Each pair contains `p0_hex`, `p1_hex`, `c0_hex`, and `c1_hex`, each exactly 32 hex digits. The command verifies that every plaintext pair has the stated nonzero XOR difference. It reports the number of nonzero output-difference bytes for each pair in histogram bins 0 through 16. The count is an observed activity measurement; the command does not infer a round count from the corpus.

```sh
crypto cryptanalysis aes-differential-statistics \
  --input docs/ic/fixtures/aes_differential_statistics_public.json
```

The public fixture contains two pairs encrypted with AES-128 under the public key `000102030405060708090a0b0c0d0e0f`. Both output differences have 16 active bytes. The four ciphertext blocks were checked against OpenSSL AES-128-ECB without padding.

## Linear parity

`crypto cryptanalysis aes-linear-statistics --input FILE` accepts `input_mask_hex` and `output_mask_hex` (32 hex digits each) and `pairs` with `plaintext_hex` and `ciphertext_hex`. Both masks must be nonzero. For each pair, the command XORs the bit parities of the masked plaintext and masked ciphertext. It reports zero and one counts, their signed difference, empirical correlation `(zero-one)/N`, and empirical bias `(zero-one)/(2N)`.

```sh
crypto cryptanalysis aes-linear-statistics \
  --input docs/ic/fixtures/aes_linear_statistics_public.json
```

The public four-block AES-128 fixture uses masks `01` and `02` on byte zero. It has one zero parity and three one parities, giving signed count -2, correlation -0.5, and bias -0.25. These are measurements of four selected blocks; statistical conclusions require a separately defined sampling protocol and more observations.

Both commands reject duplicate plaintexts, duplicate ciphertexts, malformed blocks, and out-of-range corpus sizes. They do not take a secret key or assert key recovery.

Both fixture files also passed live Setec S3 upload, HEAD, remote verification, fresh-cache `store:` replay through the corresponding command, SHA-256 comparison, and exact-key cleanup. The isolated prefix and checks are in [the acceptance receipt](../store/AES_OBSERVED_STATISTICS_S3_ACCEPTANCE_20261022.json).
