# Universal supporting identity certificates

Commit source and protocol before issuing records with the native
`twist-followup-identities` binary. Issue 32 deterministic points plus 64
domain-separated checker points for the cleared long-to-short Weierstrass
coordinate identity and the 2, 4 and 8 point Lagrange partitions of unity.
Every identity has a plus-one false variant that the issuer must reject.
Use the repository `identity.certificate/v1` implementation, its fixed
Mersenne modulus, source-hashed binary, one worker, 4 GiB and 60 seconds.
Write fresh output; timing is not admitted.

The schema evaluates over 2^61-1. These certificates do not certify the
specific toy or P-521 cover equations over their different characteristics,
or identify a polarized gluing graph. Those assertions retain their exact
native coefficient checks and independent PARI replay, and their unclosed
formal bridges. The report must cite each certificate's actual scope.

The E30 native panel completed 270 cells. The first PARI transport invocation
was rejected before launching GP because `verify-gp.sh` omitted the frozen
600-second option from its allowlist. Preserve this invocation correction;
adding 600 changes no experiment limit. The fresh GP run still uses the
original 600-second protocol cap and its own receipt/native admission gate.

The identity issuer has its own minimal Cargo workspace. The experiment
runner enables serde_json's preserve_order feature, whereas the unchanged
repository identity checker requires sorted object keys for its canonical
statement bytes. The first integrated-bin test caught this mismatch (20
passes, two canonicalization failures); retain its logs. Isolating the
issuer's dependencies preserves the original checker and its conformance
tests without changing the scientific runner's historical serialization.
