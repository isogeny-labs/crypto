# Independent development review: Dickson/S3 quotient gate

Date: 2026-11-22 (America/Los_Angeles). Reviewer: algebra derivation agent, distinct from the Rust gate implementation agent. This is an independent source review and local development replay, not PCC verification or a Lean proof.

## Outcome

No correctness defect found in the reviewed gate. The final source SHA-256 is `b76cd5a1efeff77bc065554d89acc9b53be051e6b17811cca589c2dc93a018b1`, for `research/prime31_ic_20261115/native/examples/prime31_dickson_quotient.rs` (23,556 bytes). The gate is suitable as bounded correctness evidence and a kernel-operation diagnostic. It does not establish a relation-search advantage against the existing group scan, complete ECDLP recovery, or any asymptotic speedup.

I independently reran:

```sh
cargo test --offline --locked --release --manifest-path /workspace/scratch/2f841b0b9a19/gate-test-adapter/Cargo.toml --example prime31_dickson_quotient
```

Result: **5 passed, 0 failed**, including 25 curve/target relation-set cases at depths 2 through 6, synthetic degree drops and the zero polynomial, nonreduced outer ring, the zero-depth resultant sign, inverse pairs for the identity, and an altered-resultant negative control. The retained log is `independent-gate-test.log`.

## Algebra and extraction reviewed

The coefficient construction implements exactly q(Y)=(X-r)^2Y^2-2((X+r)(Xr+A)+2B)Y+(Xr-A)^2-4B(X+r). The four-product homogeneous recurrence correctly computes Res_Y(F_k,q) modulo the outer factor-base polynomial. The monic first argument fixes the resultant orientation through a drop in q's degree.

The full-fibre product comparator is coefficient-exact and uses degree-at-most-two factors. Expanded full-fibre Horner is correctly compared after denominator clearing on the regular domain; the filtered-base Horner comparator uses support equality rather than erroneously demanding coefficient equality. The source explicitly notes that both Horner counts include unit rescaling which a normalized gcd-only oracle can omit.

The `x=r` factor is excluded only from normalized Horner, while the homogeneous recurrence retains it. The candidate extractor takes gcds before enumerating roots, and the full-fibre versus lifted-fibre support check verifies the lift lemma at each tested target. Signs and repeated indices are preserved in the existing two-entry Relation representation; equal indices are not accidentally deduplicated away. The oriented line formula uses one inverse of 2s and is independently replayed with the original curve arithmetic.

The zero ordinate and identity exclusions are explicit. On the verified prime-order benchmark curve all nonzero targets have nonzero ordinate; this is not generalized to arbitrary curves with rational 2-torsion. The synthetic degree-drop tests validate the polynomial recurrence, not the full-fibre lift lemma for excluded 2-torsion targets. Exhaustive testing on additional small-field curves and an order-three tangent instance would be a separate extension, not a completed claim.

## Important corrections incorporated during review

1. Kernel counts are now explicitly labeled as excluding factor-base setup, extraction, verification and complete logarithm recovery. They must not be reported as total solver work.
2. Per-layer support and degree counts were added; the former largest-polynomial allocation statistic alone could not demonstrate dense coefficient support or peak live memory.
3. A filtered-H Horner comparator was added because the existing repository quotient oracle uses the smaller lifted membership polynomial H, not the full fibre F. Full-F coefficient checking remains an additional correctness control.
4. The asymptotic comparison was tightened: an O(M^2 log N) uniform upper bound is not intrinsic when M=Theta(N). Before saturation, degrees grow geometrically; there are only O(1) saturated levels when N/M is bounded. For trimmed schoolbook arithmetic the sharper kernel bound is O(M^2*(1+log_2(N/M))), 1<=M<=N. Thus both recurrence and sparse product have quadratic kernels for M=Theta(N); the observed constants determine this bounded comparison. Under standard fast multiplication and prepared reduction the corresponding kernel sum can be O(Mult(M)*(1+log_2(N/M))). Setup, gcd and extraction remain charged separately.

## Original source and dependency preservation

I independently fetched the current remote native library, Cargo manifest and Cargo lockfile, then compared their Git blob identities to the local files. All matched exactly:

| File | Git blob |
|---|---|
| native/lib.rs | `42e1e7c1912be43190959c5be8f84cf59c867786` |
| native/Cargo.toml | `e555cc3fe273de1092ed798361c988d80930c875` |
| native/Cargo.lock | `aa7d4f196ac87bc0936f54a1b415f717558446e8` |

The development test adapter points directly at these existing sources and the new example. Its separate cached dependency lock differs from the original in eight package names, documented in `gate-test-adapter/receipts/dependency-differences.json`. Therefore this is not an exact-original-lock build receipt. The original lock was preserved, and the original offline build's missing dependencies remain a reproducibility limitation to resolve in CI or an environment with those packages.

## Native identity certificates and exact expansion

Four actual `identity.certificate/v1` records were issued through the unmodified repository certificate module, then replayed with the unmodified repository SHA-256 implementation. All have `proof:null`, 32 native evaluation points over p=2^61-1, and 64 additional self-replay points. Native-hash replay seed: `105832046729`; receipt: `dickson-identity-certificates/native-hash-replay.json`.

| Identity | Issued id | Exact normalized terms | Largest absolute coefficient |
|---|---|---:|---:|
| Centered S3 with f(r) substituted | `IDC1hf62e2402e4871a82` | 13 | 4 |
| Discriminant 16f(x)f(r) | `IDC1h869923a46a34c70c` | 9 | 16 |
| Homogeneous trace step | `IDC1hd3786f53034b6b9d` | 3 | 4 |
| Homogeneous norm step | `IDC1h3366a93945da9204` | 4 | 4 |

The issuer additionally expands both sides as sparse integer polynomials with checked i128 arithmetic and verifies coefficient equality over Z. This strengthens the algebraic evidence beyond modular sample evaluations, but it is still a native software check and not a formal proof-kernel result. The evaluated native certificate itself only has its stated p=2^61-1 scope; it is not mislabeled as a prime31 benchmark or universal formal proof.

The new reusable issuer is `examples/identity_certificate_dickson_s3.rs`, SHA-256 `ac8991b14edb003cc2d5f1728112bbe08c92378409325888d2620de8547cfb34`. The temporary adapter's imported upstream certificate source has Git blob `114f1f4a3f2a1be027a9b046ef166259df49dd45` and SHA-256 `73d79c9ad5ee10904956bdd2baf4a971ec35306220da3f5101bbba8598aa8646`; upstream SHA-256 source has Git blob `9566e3de448e21ff98811cfc4c6992ff3448962e` and SHA-256 `2265436018b18cf4bc3eeeb922ba59469c6d664c864e84a0ad2cbc126fa65dbb`.

The parent reviewer can run the issuer in `--check` mode with a separately chosen seed to record an additional independent replay; this review does not claim that parent replay has already occurred.

## Formalization runtime

`lean`, `lake` and `elan` were absent from PATH. The standard locations `/home/agent/.elan`, `/usr/local/bin/lean`, `/usr/local/bin/lake`, `/home/agent/.cargo/bin/lean` and `/home/agent/.cargo/bin/lake` were also absent. No Lean source was represented as compiled, and no formal proof reference was fabricated.

## Remaining research gate

The unchanged objective is a demonstrated improvement over the strongest relation-search control followed by a complete, independently reproduced ECDLP cost advantage. This iteration supplies an exact compositional identity and falsifiable diagnostic. Its next algebraic target is a sublinear common-root isolation method that uses the succinct recurrence without expanding Theta(M) coefficients or enumerating the whole fibre. A fixed 31-bit prime and this bounded diagnostic cannot establish that asymptotic theorem.
