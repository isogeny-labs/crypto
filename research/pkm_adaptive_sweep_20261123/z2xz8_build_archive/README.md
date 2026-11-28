# Pinned build and diagnostic receipts

`preflight_BUILD.json` is the original six-map preflight build receipt (SHA-256
`e09abcf0d0ff07d82f1392bca04e3abb159fea853539d8a328d40b4a5f8c63b7`).
`oracle_rebuild_BUILD.json` is the v4 cold-runner build receipt (SHA-256
`21b70ef0dc1799f545efd257b5d858ff8bda82cc65d6a9f4b87f378280cf82cf`).
The latter pins source commit `62908d5299c04c0b9bda686eba7f19e271b09786`
and binary SHA-256 `de461514bcb5f6538781ee25785f11ac33940711a93c0b6cb6a5dfbe0a587002`.
The executable is not stored here; rebuild and verify its hash before running
the frozen cold protocol. `DIAGNOSTIC.json` and `DIAGNOSTIC_RAW.json` preserve
the one-trial setup-ledger replay and its raw CLI report.

`../z2xz8_preflight_archive/` preserves the preflight receipt and all input,
stdout and stderr bytes. Receipt paths record their original location; the
archive copies have the same basenames and SHA-256 hashes.
