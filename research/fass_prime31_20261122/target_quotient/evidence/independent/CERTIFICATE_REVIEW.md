# Independent fixed d=6, k=2 identity-certificate replay

Date: 2026-11-23 UTC. Separate `/root/map_cost_audit` reviewer on the same host and session team. This is unsigned independent-role validation of an evaluated identity certificate, not signed independent execution, a separate-host reproduction, or Lean verification. [pcc:unverified]

## Outcome

The immutable native checker accepted certificate `IDC1h7e169f28f6a50bb6` with a fresh reviewer seed, exit 0, under the frozen 60-second timeout and 2 GiB virtual-address-space limit. It rechecked 32 issuer-stream points and checked 32 additional points from the reviewer's new stream. The certificate's `proof` field is `null`. Stderr was empty, and all nine source, dependency, protocol, binary, certificate, and seed hashes were unchanged across execution.

The exact fresh seed is **`3750397280611057423`**, generated from eight OS random bytes after the certificate existed. It differs from the issuer seed `5851947445165183829` and producer replay seed `5851947445098402898`. All three are retained as exact decimal strings in this review's structured receipt, avoiding binary64 rounding of integers above 2^53.

## Identity and source audit

The complete 8,444-byte certificate driver was inspected. It fixes `d=6`, `k=2` and the field modulus `2305843009213693951 = 2^61-1`. This field is different from the Prime31 qualifier field.

The left expression uses the denominator-free quadratic norm recurrence with

    alpha = (1-T)^2, beta = -2T(1+T), gamma = T^2,
    U_0=2, U_1=-beta,
    U_j=-beta U_(j-1)-alpha gamma U_(j-2),
    h(T)=alpha^6+gamma^6-U_6.

The right expression is a Euclidean-division witness formed from the closed binomial coefficients, independently of that recurrence:

    h(T) = 1 - 12T + 66T^2 - 220T^3 + T^4 Q(T).

The driver explicitly constructs `Q`. Both syntactic degree bounds are 12. `check_expected` requires exact equality of the certificate statement with this fixed constructed expression, the expected field modulus, and `proof:null`, before invoking the unchanged published checker. Thus a valid but unrelated identity cannot be substituted for this certificate in the replay.

The published checker validates the statement variables and degree bound, recomputes the statement identifier and the issuer-point evaluation digest, and evaluates the additional stream. Its source and SHA-256 implementation match both the immutable producer source snapshot and base commit `f0ca664eca0081a30adfcc79dcf62a32de40d2af`.

The statement concerns one univariate specialization and an explicit remainder modulo `T^4`. It does not prove a generic determinant nonvanishing statement, a rational-root quotient rank theorem, the selector-target lemma, or a new relation solver.

## Reproducibility records

Executed from `target_quotient/native/certificate`:

```sh
timeout 60s sh -c 'ulimit -v 2097152; exec target/immutable/certificate-attempt1 check ../../evidence/producer/certificate-attempt1/cusp-d6-k2-remainder.identity.json 3750397280611057423'
```

Both pre-call and post-call receipts are timestamped `2026-11-23T04:25:04Z`. No timing result is inferred from these custody timestamps. No source rebuild, certificate reissue, certificate mutation, producer-evidence mutation, or corrected retry occurred.

| Subject | SHA-256 |
|---|---|
| Certificate driver | `e8f9f046fb2d70190e5b304d7b9588c3de3c041cb4351af141059efd1eef7adb` |
| Certificate manifest | `1dd61b1d673c94967aad4251f0b60bfc875ad1c2c9d1368a5fc354d6a1384675` |
| Certificate lockfile | `b61277dc8134c4e77140f590ae681439ac1df68c10e52dcf43412c36ab8200ff` |
| Published identity checker | `d81ee915ad769a87d7aac5d9664631239607f4a0deb27850cdb52cea84e34270` |
| Published SHA-256 source | `2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf` |
| Immutable checker executable | `0e5b36d69348c056170d7d21f2fba6b86dbe0127f0ffc79ba467b4c663fc7d01` |
| Input certificate | `5b8adc9c6e41d9f07a620d6dc5bca9d96655fdabe22123eee3e6caf3589963dd` |
| Reviewer stdout | `27cb082037e27a2f3072214f2321f4e985234fcfa7a7c87754fbea1c013ba8fe` |

Raw pre/post manifests, exact seed, source-comparison receipts, 493-byte stdout, empty stderr, exit status, and copies of the producer's build/call manifests and issuance output are under `certificate-replay1/`.

## Controls and evidentiary limits

The producer's retained issuance output reports two negative controls: adding 1 to the right expression is rejected as false, and changing the recorded evaluation digest is rejected. Source inspection confirmed these are actual assertions in issue mode. This reviewer did not rerun issue mode or those controls; independent check-mode output correctly records them as `null`.

The checker derives each stream through SHA-256 into a 64-bit state, expands it with SplitMix64, and reduces outputs modulo the field prime. The 32 extra checks are deterministic after the chosen seed. They are not 32 independently sampled uniform field elements, so this review does not infer an IID Schwartz-Zippel false-accept probability from the point count. The retained outcome is the exact acceptance of this fixed expression under the recorded streams, together with the independent algebraic source review.

Generic mathematics, fixed-fixture qualifier checks, and this evaluated identity certificate remain separate evidence categories. No Lean or Lake command was invoked, and no signed PCC standing is asserted by this review.
