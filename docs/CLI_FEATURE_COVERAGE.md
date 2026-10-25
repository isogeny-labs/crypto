# CLI feature coverage (v1)

This is the current command contract for the `crypto`, `ic`, `icx`,
`ecdlp-nist`, and `isogeny_walk` release binaries. The sibling
`cryptanalysis` repository exposes these through `crax`; its native
`ic zp` command solves discrete logs in `(Z/pZ)*`, a different group from
the elliptic-curve IC commands below. The [2026-10-17 audit](ic/CLI_CAPABILITY_AUDIT_20261017.md)
is a historical baseline. Use command `--help` for individual options and
the linked guides for schemas. All work bounds are input admission rules,
not estimates of the cost of a general cryptographic instance.

| Algorithm and field | Command and supplied input | Result check or certificate | Resource envelope |
| --- | --- | --- | --- |
| Elliptic-curve IC over `F_p` | `ic` or `icx solve-prime` with `p,a,b,n,G,Q`, or `solve-prime-file --input` with a local or checked store reference | Reports factor-base, relation, linear-algebra and replay stages; verifies `[k]G=Q`; same-target rho reference unless disabled | Prime `p` in 5..1,000,000 and prime `n` in 3..4,000,000; relation and rho budgets are explicit. [Target guide](ic/EXTERNAL_TARGET_CLI.md). |
| Elliptic-curve IC over `F_(p^3)` | `ic` or `icx solve-cubic --input` with a strict field modulus, curve, subgroup, `G,Q` target file or checked store reference | Counts and checks the curve and subgroup; reports IC stages and verifies the scalar; optional same-target rho | Prime characteristic 7..101, prime subgroup order 3..2,000,000, bounded residual search. [Target guide](ic/EXTERNAL_TARGET_CLI.md). |
| Prime-field isogeny transport into IC | `prime-route-generate` followed by `solve-prime --route-file --route-id` | Route generator checks kernels, codomains and maps; solver rechecks ordered edges, transported `G,Q`, and scalar on both models | Prime kernel degree 2..31 when a rational route is found within the curve/depth budgets. [Route guide](ic/EXTERNAL_TARGET_CLI.md#certified-isogeny-route). |
| Cubic-field isogeny transport into IC | `cubic-route-generate` followed by `solve-cubic --route-file --route-id` | Certified Vélu route and mapped subgroup points; independent route check before IC and source/destination scalar replay | Degree 2..31 subject to field and kernel search bounds; `--max-depth` 1..4 and `--max-samples` 1..10,000. [Route guide](ic/EXTERNAL_TARGET_CLI.md#from-a-cubic-j-neighbour-to-a-checked-ic-route). |
| Cubic isogeny j-graph numeration | `ic` or `icx cubic-graph --input --max-ell --max-nodes` | Degree-labelled j-neighbours and explicit truncation status; graph nodes alone do not certify point transport | Degrees 2, 3, 5 and 7; at most 64 nodes. [Graph guide](ic/EXTERNAL_TARGET_CLI.md). |
| Other ECDLP | `ecdlp-nist solve CURVE` with supplied `--point` or `--point-file` and interval, or planted control | Scalar multiplication checks a reported solution; method, iteration and feasibility limits appear in the report | Whole-group and interval methods have explicit work gates; [P-256 interval receipt](ic/fixtures/ecdlp_nist_interval_cli_20261022.json). |
| Artifact upload, existence and retrieval | `crypto store push`, `verify --remote [--download]`, and `pull`; IC input, route and report options accept checked store references | Content-addressed SHA-256 index; HEAD checks existence and size; `--download` and pull hash remote bytes; missing/corrupt objects fail closed | `CRYPTO_STORE_REMOTE=s3://setec-astronomy-artifacts/ic/v1/runs` for retained runs, isolated `ic/v1/acceptance/<run-id>` for disposable checks. [Store guide](store/README.md). |
| SHA-2, SHA-3, SHAKE and other digests | `crypto hash ALGORITHM [MESSAGE]`, `--hex`, or `--file` with local/checked store reference | Digest output; SHA-256, SHA-3 and SHAKE file paths have streaming implementations and independent known-answer checks | SHAKE/cSHAKE require `--output-bytes`; per-algorithm limits are checked before work. [Hash and streaming guide](ic/EXTERNAL_TARGET_CLI.md#sha-family-cryptanalysis-adapters). |
| Hash cryptanalysis | `crypto cryptanalysis hash-auto`, `hash-differential-observed`, `length-extension`, or `hash-joux-reduced` | Bounded birthday/differential statistics; SHA-256 bare-prefix length-extension can replay a constructed suffix; Joux command emits and replays every link in a reduced compression-state collision chain | Joux retains 8..24 state bits after **each** block, with 1..8 links and at most 200,000 trials per link. It does not produce a full-width hash collision. SHA-3 is analyzed by bounded statistics, not a collision or preimage solver. [Hash guide](ic/EXTERNAL_TARGET_CLI.md#sha-family-cryptanalysis-adapters). |

## AES analysis input and result grid

`crypto aes` provides seven primitive commands for block encryption/decryption,
CTR, CBC and GCM. The 25 `aes-*` commands below are selected with
`crypto cryptanalysis COMMAND`; `crax aes COMMAND` forwards them to the
sibling binary. Analysis outcomes vary by technique. In particular, a
statistical observation, a cost estimate, or a candidate set is not a
verified AES master key. The [AES adapter guide](ic/EXTERNAL_TARGET_CLI.md#aes-analysis-adapters)
gives corpus schemas, work bounds and examples.

| Command | Input | Result scope |
| --- | --- | --- |
| `aes-sbox-table` | DDT/LAT/BCT/DLCT kind and optional row | Exact S-box table or row |
| `aes-algebraic-equations` | None | Checks quadratic equations on all 16 input/output pairs of a four-bit AES-like S-box |
| `aes-slide-comparison` | Sample count and seed | Pair observations for a repeated-round-key variant and AES-128, limited to the sampled set |
| `aes-grover-cost` | Key size 128, 192 or 256 | Logical resource estimate, not an executed search |
| `aes-biclique-small-scale` | Four-nibble base key, plaintext, optional captured ciphertext and 1..15 rounds | Matching keys within a four-key biclique grid on the four-cell small-scale model; no full AES attack claim |
| `aes-trail-check` | Active-byte pattern sequence | Branch-constraint check for the supplied trail |
| `aes-attack` | Key, seed and samples; differential, linear or Square technique | Reduced-round recovery: 2-round differential, 1-round linear or 4-round Square |
| `aes-distinguisher` | Key, samples and selected technique | Bounded impossible, yoyo, mixture, boomerang or S-box zero-correlation measurement |
| `aes-related-differential` | Nonzero S-box input/output differences | One-round shifted-family analysis |
| `aes-cpa` | Aligned power traces; optional known ciphertext | First-round key candidate, with full-key verification only when a ciphertext is supplied |
| `aes-dpa` | Aligned power traces; optional known ciphertext | Difference-of-means key candidate, with full-key verification only when a ciphertext is supplied |
| `aes-cache-timing` | Isolated lookup timing observations | Key-byte high-nibble estimates |
| `aes-differential-observed` | Captured two-round chosen pairs | Differential key-byte candidates and checked corpus identity |
| `aes-differential-statistics` | Captured plaintext/ciphertext pairs and difference | Observed output-difference activity |
| `aes-linear-statistics` | Captured pairs and input/output parity masks | Observed linear correlation |
| `aes-higher-order-observed` | Complete plaintext cube and captured pairs | Observed XOR derivative |
| `aes-mitm-observed` | Complete 256-pair two-round delta set | Two round-key-byte candidates |
| `aes-integral-observed` | Complete 256-pair lambda set | Observed Square integral balance |
| `aes-square-observed` | Complete four-round lambda sets; optional held-out pairs | Recovered AES-128 key checked on training and, when supplied, held-out pairs |
| `aes-linear-observed` | One-round known pairs; optional held-out pairs | Key candidates with separate held-out replay when supplied |
| `aes-linear-parity` | Known pairs, column/row and S-box masks | Effective-key parity inference |
| `aes-dfa-schedule` | Established AES-128 last-round key | Inverse key schedule and re-expansion check |
| `aes-dfa` | Correct/faulted ciphertext pairs | Verified AES-128 master key when faults determine a unique candidate |
| `aes-related-key` | Key size, difference and rounds | Key-schedule diffusion and bounded local-collision report |
| `aes-visual-demo` | Demo selector | Diagrams of selected attack mechanisms |

The observed differential, linear and Square corpora also have versioned
formats and checked `store:` inputs. See the [cross-CLI corpus receipt](store/AES_CROSS_CLI_CORPUS_ACCEPTANCE_20261023.json)
and [Square corpus receipt](store/AES_SQUARE_CORPUS_ACCEPTANCE_20261023.json).
These commands do not implement a general full-round AES key-recovery attack.

## Release status

This grid describes the reviewed PR binaries. The crypto commands above
remain on PR #2117 until its exact-head checks and merge gates pass. The
sibling `crax` release is being repaired through cryptanalysis PRs #1111 and
#1113. A local test or S3 acceptance receipt does not replace those hosted
checks or the final merged-revision build.
