# Section-search identity issuer

This isolated Rust tool reuses the maintained identity certificate source
unchanged and the Cargo.lock-pinned sha2 backend through a thin adapter.
It produces two evaluated residual identities for the
section-search-v1 derivation, checks a second fixed stream and refuses
plus-one false identities. It implements neither the proposed geometric
product query nor a coefficient/root search.

Run only through the frozen study's reproduction/qualification instructions.
The certificate field is2^61-1. Fixed checks are producer unsigned evidence,
not a formal identity over Curve25519 or an independently signed receipt.
