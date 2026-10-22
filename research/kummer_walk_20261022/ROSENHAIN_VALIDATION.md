# Rosenhain addendum validation

- The raw input is the already checked 21-row quintic factorization file
  from the first Kummer model gate. Its SHA-256 is pinned in the manifest.
- The producer tried all 20 ordered finite-root pairs for each model,
  independently computed the square condition and checked an exact
  transformed polynomial identity for the first admissible pair.
- The released producer was run twice. The second JSONL matched the first
  byte for byte (`cmp` exit 0); the initial output was held in
  `/var/tmp/rosenhain-first.jsonl` during replay.
- A separate native validator reread all 21 rows and checked the record
  predicates, 21 distinct primes, the selected square roots, five distinct
  normalized roots, and coefficient-vector equality. It reported 21 passes,
  zero failures, and `square_pairs_values=[10]`.
- This validates Rosenhain normalization only. Theta constants and fast
  Kummer arithmetic were not executed or priced.
- The six-coefficient equalities are exact model-specific equalities over 21
  different `Fp` fields, with both vectors in the raw rows. The repository's
  `identity.certificate/v1` records a single symbolic statement over the
  fixed field `2^61-1`; it cannot encode this variable-field record without
  changing the claim. The native coefficient and replay checks are the
  evidence used here.
