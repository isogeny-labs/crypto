# Explicit public-target IC commands

`ic` and `icx` accept caller-supplied public curve, generator, and target points for
two bounded field families. Both commands reject malformed curves and points
before relation collection. A successful result includes a scalar checked by
an independent multiplication of the supplied generator.

## Prime field

```sh
ic solve-prime --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 --json
```

The example target is `[190]G`. The command requires a prime `p` in
`5..=1000000`, a nonsingular short-Weierstrass curve, a prime subgroup order
`n` in `3..=4000000`, canonical coordinates, and `[n]G = [n]Q = O`.
When `n` also divides the cofactor, it checks exact membership in `<G>`;
an order-`n` point in another subgroup is rejected.
It counts `#E(F_p)` and records the exact cofactor before collection.
Relation collection is limited by `--fb-size`, `--extra-relations`,
`--max-trials-per-relation`, and `--max-relation-attempts`. The JSON report
separates factor-base construction, relations, linear algebra, and final
verification, and includes a same-target Pollard-rho reference unless
`--rho-steps 0` is set. The prime relation sampler currently uses the
process random source, so a second invocation is an independent run rather
than a bit-for-bit replay.
The verified public solve output used for the S3 integration check is frozen
as [`prime271_solve_report_20261018.json`](fixtures/prime271_solve_report_20261018.json).
Its SHA-256 is recorded in the linked
[`IC_REPORT_S3_ACCEPTANCE_20261018.json`](../store/IC_REPORT_S3_ACCEPTANCE_20261018.json)
receipt; the S3 acceptance object was deleted after the fresh pull check.

### Certified isogeny route

`solve-prime` can transport the supplied `G` and `Q` along one root-origin
`IW1` route before collecting IC relations:

```sh
ic solve-prime --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 \
  --route-file docs/ic/fixtures/prime271_isogeny_routes.json \
  --route-id IW1E11h1h1a619f4ab250 --json
```

`--route-file` also accepts an indexed `store:NAME` or `sha256:DIGEST`
reference. The solver verifies and fetches a missing route file through the
configured content store before checking the graph and transporting points.
For example, after publishing the fixture as
`isogeny-routes/public-prime271/walk-11/routes.json`, replace the local path
above with
`store:isogeny-routes/public-prime271/walk-11/routes.json`.

The fixture was enumerated by `isogeny_walk walk` with `--primes 11`,
`--max-curves 3`, and `--max-depth 1`; the two edge certificates also passed
`isogeny_walk verify`. The IC command independently counts the starting
curve order, re-verifies every graph node, edge, and route ID, constructs each
point map from its certified kernel polynomial, checks the codomain and
isomorphism, and tests that each map commutes with point doubling. It rejects
an edge that kills the selected subgroup. Its JSON receipt records the route
hash, both transported points, and validation counts. Any scalar found on
the destination is checked on both the destination and the original public
curve. The Pollard-rho reference stays on the original `G,Q` pair.

## Cubic extension field

```sh
ic solve-cubic --input docs/ic/fixtures/cubic_f7_public_target.json \
  --seed 1 --max-residuals 100000 --json
```

The input describes `F_p[t]/(t³-c)` and `y²=x³+ax+b`, with field elements
as three canonical coefficients in increasing powers of `t`. The included
fixture has `p=7`, `c=2`, full curve order `347`, and target `[5]G`; its
coordinates and curve order were also computed independently with direct
extension-field arithmetic and point enumeration. The command requires prime
`p` in `7..=101`, a noncube `c<p`, prime `n` in `3..=2000000`, a nonsingular
curve, and valid subgroup points. It counts `#E(F_{p³})`, requires `n` to
divide that order, and supports a cofactor `h = #E/n` up to 64 when `h` is
invertible modulo `n`. It samples cofactor components, verifies relations in
the full group, then projects each relation by `[h]` before linear algebra.
The projected factor base must contain a nonidentity subgroup point.
`--input` also accepts `store:NAME` or `sha256:DIGEST`; the artifact store
checks the indexed content hash when it fetches the JSON into its cache.
It reports the Gaudry-style relation and linear-algebra counters and runs a
same-target rho reference unless `--skip-rho` is set.
The checked `ic solve-cubic` output for this input is frozen as
[`cubic_f7_solve_report_20261018.json`](fixtures/cubic_f7_solve_report_20261018.json).

An earlier public input on a curve of order `344 = 8 × 43` is retained as
[`cubic_f7_cofactor8_input.json`](fixtures/cubic_f7_cofactor8_input.json).
Before the full-order check was added, its bounded run used 100,000 residuals
and found zero decompositions; the exact report is
[`cubic_f7_cofactor8_bounded_20261018.json`](fixtures/cubic_f7_cofactor8_bounded_20261018.json).
That observation does not establish that no decomposition exists. The updated
CLI rejects the input before relation collection because all three factor-base
points project to the identity in its order-43 subgroup. The cofactor-12
fixture [`cubic_f7_cofactor12_public_target.json`](fixtures/cubic_f7_cofactor12_public_target.json)
has four nonidentity projected base points and recovers its externally
specified scalar with full-group relation checks and subgroup replay. Its
charged run and same-target rho reference are frozen in
[`cubic_f7_cofactor12_solve_report_20261018.json`](fixtures/cubic_f7_cofactor12_solve_report_20261018.json).
A separate live S3 test fetched this input into a fresh cache through the
`solve-cubic` command and verified scalar 5; its upload, HEAD, remote check,
and cleanup receipt is
[`CUBIC_TARGET_S3_ACCEPTANCE_20261018.json`](../store/CUBIC_TARGET_S3_ACCEPTANCE_20261018.json).

### Checked cubic-field isogeny route

The cofactor-12 input also has a rational order-3 kernel over its specified
cubic field. Both `ic` and `icx` can verify the corresponding graph and
transport the public generator and target before collecting relations:

```sh
ic solve-cubic \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --route-file docs/ic/fixtures/cubic_f7_cofactor12_isogeny_route.json \
  --route-id root-to-velu3 --seed 1 --max-residuals 1000 --skip-rho --json
```

The route file can also be an indexed `store:NAME` or `sha256:DIGEST`
reference. Its schema fixes `field_characteristic` and `cubic_nonresidue`,
lists bounded short-Weierstrass `curve_nodes` with canonical three-coefficient
`a` and `b`, and records each edge's source, target, odd prime degree in
`3..=31`, and affine `kernel_generator`. Each route is a root-origin sequence
of edge IDs. The command verifies every node, edge, and route, checks each
kernel's exact prime order, independently reconstructs the Vélu codomain,
checks the transported point map against doubling and subgroup order, and
counts the destination curve order. A recovered scalar is replayed on both
the destination and the caller's original `G,Q` pair. The rho reference stays
on that original pair. The JSON `route` receipt records the graph hash,
verification counts, destination model, and transported points.
A live run resolved this route through `store:` from an empty cache, recovered
and replayed scalar 5, verified the object in S3, and removed only its isolated
acceptance object. Its nine checks are recorded in
[`CUBIC_ROUTE_S3_ACCEPTANCE_20261020.json`](../store/CUBIC_ROUTE_S3_ACCEPTANCE_20261020.json).

### Cubic-field j graph numeration

Both CLIs can enumerate distinct rational j-invariant neighbours from a
specified cubic-field curve using independently evaluated modular
polynomials:

```sh
ic cubic-graph --p 7 --c 2 --a 3 6 4 --b 4 4 6 \
  --max-ell 5 --max-nodes 32 --seed 1 --json
icx cubic-graph --p 7 --c 2 --a 3 6 4 --b 4 4 6 \
  --max-ell 5 --max-nodes 32 --seed 1 --json
```

The field must be exactly `F_p[t]/(t³-c)` with canonical coefficients,
prime `p` in `7..=101` (necessarily `p ≡ 1 mod 3`), and noncube `c`. Supported odd prime degrees are
3, 5, and 7 when `p > degree + 1`. `--max-nodes` is bounded to 1–64;
`truncated` records when that limit omits a discovered neighbour. Nodes
are distinct j-invariants in the exact cubic field. Edges record distinct
target j-invariants per degree and pass a second modular-polynomial
evaluation. This numeration does not include edge multiplicities or kernel
generators. To transport an IC target, supply a separately certified
`--route-file` as described above.

The public p=7 command above completed without truncation: 6 j nodes and
18 distinct degree-labelled directed edges. Both `ic` and `icx` returned
the same JSON data. A separate exhaustive test evaluated every candidate
target j over all 343 field elements for both degrees and found exactly the
reported neighbours. The frozen CLI output is
[`cubic_f7_j_graph_public_20261021.json`](fixtures/cubic_f7_j_graph_public_20261021.json),
SHA-256 `4e07e7c2fc117ecf8deec4916ac9d0178325f5118464ae5881c980c0ab2069de`.
A p=97, c=2 run with degree 3 and a two-node cap completed in 1.144 seconds
on the local host and correctly marked its output truncated; that check
validates command execution at the upper prime range, not complete graph
coverage there.

### From a cubic j neighbour to a checked IC route

For a neighbour reached by a rational odd-prime kernel, either CLI can
construct the exact one-hop Vélu route file consumed by `solve-cubic`:

```sh
ic cubic-route-generate \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --degree 3 --target-j 3 5 0 --seed 1 --max-samples 256 --json \
  > /tmp/cubic-route.json
ic solve-cubic \
  --input docs/ic/fixtures/cubic_f7_cofactor12_public_target.json \
  --route-file /tmp/cubic-route.json --route-id root-to-velu3 \
  --seed 1 --max-residuals 1000 --skip-rho --json
```

The generator counts the source curve order, checks the supplied public
points and subgroup, samples a rational kernel of the requested odd-prime
degree in `3..=31`, and checks the Vélu codomain, point maps and subgroup
before returning a route. `--target-j` selects an exact j-invariant from
`cubic-graph`; omit it to accept the first checked rational-kernel edge.
The field and subgroup bounds match `solve-cubic`; `--max-samples` is
1–10,000. A bounded search that does not find a matching rational kernel
exits unsuccessfully. The constructor handles edges with a rational kernel
generator; other j-neighbours remain outside this route format.

For the public p=7 source, the command selected destination j=`[3,5,0]`,
returned the checked codomain `a=[3,5,6]`, `b=[4,4,6]`, and its output was
accepted by `ic solve-cubic`. The solver verified the route and recovered
scalar 5 on the source and destination. The
[generated route](fixtures/cubic_f7_generated_route_20261021.json)
SHA-256 is `fe9e14bfe7e53afa681b1fdad0b41be5e96779930309eb06014f4b269e9ebe0f`.
The same route generator is exposed through `icx`; both release CLIs
produced semantically identical route JSON and exited successfully.
The generated route also passed a live Setec S3 push, HEAD, remote verify,
fresh-cache `solve-cubic` fetch and scalar replay, exact-object cleanup, and
final HEAD 404. The isolated prefix and checks are in
[`GENERATED_CUBIC_ROUTE_S3_ACCEPTANCE_20261021.json`](../store/GENERATED_CUBIC_ROUTE_S3_ACCEPTANCE_20261021.json).

The prime and cubic commands return `bounded_unknown` with a nonzero exit
status if their configured IC budget ends without a verified scalar. These
finite input ranges describe the implemented solvers; they do not certify
performance or coverage on larger curves. The timings are local diagnostics,
not a claim against the repository's full IC-versus-rho measurement gates.

## AES analysis adapters

```sh
crypto cryptanalysis aes-attack --technique differential
crypto cryptanalysis aes-attack --technique linear
crypto cryptanalysis aes-attack --technique square
crypto cryptanalysis aes-differential-observed --input chosen-pairs.json
crypto cryptanalysis aes-linear-observed --input known-pairs.json
crypto cryptanalysis aes-linear-parity --input known-pairs.json \
  --column 0 --row 0 --input-mask 1 --output-mask 1
crypto cryptanalysis aes-distinguisher --technique impossible --samples 4096
crypto cryptanalysis aes-distinguisher --technique yoyo --samples 4096
crypto cryptanalysis aes-distinguisher --technique mixture --samples 4096
crypto cryptanalysis aes-distinguisher --technique zero-correlation
crypto cryptanalysis aes-cpa --input traces.json
crypto cryptanalysis aes-cache-timing --input isolated-lookup-timings.json
crypto cryptanalysis aes-dfa --input fault-pairs.json
crypto cryptanalysis aes-dfa-schedule --last-round-key-hex 13111d7fe3944a17f307a78b4d2b30c5
```

These commands expose the existing AES-128 two-round differential,
one-round linear, and four-round integral recoveries. `--key-hex` supplies
the analysis instance; `--samples` and `--seed` configure the differential
and linear corpora. Each recovered result is checked against that instance.
The rest of the AES modules have separate input models and remain outside
this adapter; `crypto cryptanalysis sbox`, `auto`, `boomerang`, `rectangle`,
`aes-related-key`, and `aes-visual-demo` provide other entry points.
`aes-distinguisher` exposes four further bounded analyses: a four-round
impossible differential, a two-versus-three-round yoyo comparison, a
one-versus-two-round mixture identity comparison with subspace diagnostics,
and exact zero-correlation
mask enumeration on the AES S-box. The first three use a supplied AES-128
analysis key; the S-box check concerns the public substitution table. Each
command reports the configured sample count and its observed structural
check. These are distinct input models and do not imply an attack on
full-round AES.

`aes-differential-observed` accepts caller-supplied data for the reduced
two-round cipher (the last round omits MixColumns):

```json
{"alpha":115,"pairs":[{"p0_hex":"00112233445566778899aabbccddeeff","p1_hex":"73112233445566778899aabbccddeeff","c0_hex":"<32 hex digits>","c1_hex":"<32 hex digits>"}]}
```

Supply at least two and at most 100,000 pairs. Each pair must change only
plaintext byte zero by `alpha`, and its ciphertext difference must have
exactly active positions 0, 7, 10, and 13. The result contains four
last-round key-byte candidates at those positions and reports
`partial_candidate`; this does not identify a whole AES-128 key.

`aes-linear-observed` accepts `{"pairs":[{"plaintext_hex":"<32 hex
digits>","ciphertext_hex":"<32 hex digits>"}],"verification_pairs":[...]}`.
It requires 16–100,000 training pairs and permits up to 100,000 independent
verification pairs. The one-round model omits final MixColumns. The current
byte recovery routine enumerates S-box input-byte candidates and checks a
constant output XOR across observations; its output is not a LAT-derived
multi-round linear attack. It reports `candidate` without holdout data and
`verified` only when the recovered input key reproduces every distinct
verification pair with the reduced-round cipher. A mismatch reports
`verification_failed` and exits unsuccessfully. Both observed-data commands
accept local files or checked store references.

`aes-linear-parity` exposes the actual S-box LAT majority-vote analysis on
caller-supplied one-round known pairs. Select a ciphertext byte by `--column`
and `--row` (0–3 each) and nonzero 8-bit input/output masks. The command
requires 64–100,000 distinct plaintext samples and rejects a zero-bias LAT
entry. Its output gives the signed LAT entry, theoretical bias, observed
zero/one equation counts, vote margin, and the candidate parity bit
`a·K0[(column+row) mod 4,row] XOR b·K1[column,row]`; an exact vote tie is
`ambiguous`. This is one bit of effective key material for the stated
one-round model. The command does not use `verification_pairs`, so it rejects
inputs that contain them. Its JSON can be loaded through a checked store
reference as well as a local file.

The release CLI acceptance used the public AES-128 key
`000102030405060708090a0b0c0d0e0f` and 4,096 distinct plaintexts. For
sample index `i` from 0 to 4095, the plaintext is the first 16 bytes of
`SHA-256("aes-linear-parity-public-v1" || i.to_bytes(4, "big"))`. Ciphertexts
were independently generated by pre-whitening with the key, applying the
AES S-box and ShiftRows once, then XORing the AES round-1 key; this is the
declared no-MixColumns one-round model. The compact JSON input uses
`{"pairs":[{"plaintext_hex":"...","ciphertext_hex":"..."},...]}` with
no verification pairs. Its SHA-256 is
`cd4162f785affed42a08ecd47b28248c89f01d7212abb36d46f0f80a65e52197`.
With `--column 0 --row 0 --input-mask 1 --output-mask 1`, the LAT entry was
+12; 2,213 equations evaluated to zero and 1,883 to one. The CLI reported
candidate parity bit 0, matching the public key schedule's bit 0 of input
key byte `00` XOR bit 0 of round-1 key byte `d6`. Binary SHA-256:
`0e704c9a2e3637cd53f4cfde6a3e1927a5204dbbaa3073ece3864b2ee129d15b`;
report SHA-256: `2b91b90f1509eb6fca644d25962603747a5ec0ee91621d3b92b07c967698c031`.
The same input with masks 1 and 2 was rejected because that LAT entry is zero.

`aes-cpa` accepts JSON with `plaintexts` as 16-byte hex strings and
`traces_per_byte` as sixteen equal-length arrays of finite measured values.
Each channel corresponds to first-round S-box leakage for the matching
plaintext byte. It requires 16–100,000 aligned samples and reports the
candidate AES-128 key, peak and runner-up absolute correlations, and the
first-round Hamming-weight leakage model. Pass
`--known-ciphertext-hex HEX` for the first plaintext to verify the full
ten-round AES encryption; without that check the status is `candidate`.
This command analyzes supplied traces. Trace acquisition and alignment
depend on the measured device.

The release CLI acceptance report is
[`aes_cpa_public_report_20261018.json`](fixtures/aes_cpa_public_report_20261018.json).
It uses 512 noiseless Hamming-weight samples per byte with key
`000102030405060708090a0b0c0d0e0f`. The first plaintext is
`00112233445566778899aabbccddeeff`; for sample `i` from 1 to 511,
plaintext byte `j` is `(37i + 19j) mod 256`. Each trace is the bit count
of the AES S-box output at that plaintext byte XOR the matching key byte.
The input JSON SHA-256 is
`81b50a60c0361b33d7df6fe73cf5827942ce8dd10ff6284e931c97172a9ed2d8`.
The known-answer ciphertext is `69c4e0d86a7b0430d8cdb78070b4c55a`.

`aes-cache-timing` accepts JSON with `plaintexts` as 16-byte hex strings
and `timings_per_byte` as sixteen arrays aligned to those plaintexts. Each
array must measure one separately instrumented first-round lookup, with a
faster observation when that lookup reaches cache line zero. The command
requires 64–100,000 observations, finite timings, and at least four samples
in every plaintext high-nibble bucket for every byte by default. It returns
one candidate **high nibble** per key byte, the least populated bucket count,
and the gap between the two lowest bucket means. Equal minimum means leave
a byte unresolved. The status is `candidate` or `ambiguous`; neither means
a full AES-128 key was recovered. Ordinary whole-encryption timings do not
satisfy this isolated-lookup input model. The runnable public corpus and
checked release result are
[`aes_cache_timing_public_input.json`](fixtures/aes_cache_timing_public_input.json)
and
[`aes_cache_timing_public_report_20261020.json`](fixtures/aes_cache_timing_public_report_20261020.json).
The corpus has 64 observations, with four samples in every bucket. It uses
an exact zero-noise isolated-lookup model for a public key and is an input
contract check, not evidence about a device's timing leakage.

`aes-dfa` accepts 1–64 correct/faulted ciphertext pairs under a single-byte
fault at the start of AES-128 round 9, before SubBytes. The input is a JSON
object with a `pairs` array. Every entry has `plaintext_hex`,
`correct_ciphertext_hex`, and `faulted_ciphertext_hex`, each exactly 16 bytes
in hex. The command checks that each pair has the expected four-byte
diffusion pattern, intersects round-10 key candidates by affected column,
reverses the key schedule when every column has one survivor, and verifies
the master key by encrypting every supplied plaintext. It also replays each
faulted ciphertext by injecting a nonzero byte fault at the declared round-9
boundary under that key. An unresolved set of
candidates is reported as `bounded_ambiguous` with a nonzero exit status;
contradictory or malformed pairs also fail. The input may be a file path or
a `store:`/`sha256:` artifact reference. The reproducible public input and
result are
[`aes_dfa_round9_public_input.json`](fixtures/aes_dfa_round9_public_input.json)
and
[`aes_dfa_round9_public_report_20261018.json`](fixtures/aes_dfa_round9_public_report_20261018.json).
The input SHA-256 is
`1b1eb355e4c71ed6b5b194987ce03f4a887c2648e7f42dc688c2739edbfee8a1`.
Its 16 correct ciphertexts also match independent OpenSSL AES-128-ECB
encryption under the recovered key. The corpus varies all four injected
source rows in each of the four MixColumns columns; every column has one
surviving round-key tuple. The release report records
`faulted_pairs_replayed: 16` after the recovered master key reproduces every
faulted ciphertext through an injected round-9 byte fault.

`aes-dfa-schedule` reverses a supplied AES-128 round-10 key to the master
key and checks that forward expansion reproduces it. The example round key
maps to master key `000102030405060708090a0b0c0d0e0f`. The output is in
[`aes_dfa_schedule_public_report_20261018.json`](fixtures/aes_dfa_schedule_public_report_20261018.json).

## SHA-family cryptanalysis adapters

```sh
crypto hash sha256 --hex 000102ff
crypto hash sha3-224 --hex 000102ff
crypto hash sha3-256 --hex 000102ff
crypto hash sha3-384 --hex 000102ff
crypto hash sha3-512 --hex 000102ff
crypto cryptanalysis hash-auto --hash sha256
crypto cryptanalysis hash-auto --hash sha224
crypto cryptanalysis hash-auto --hash sha384
crypto cryptanalysis hash-auto --hash sha512
crypto cryptanalysis hash-auto --hash sha512-224
crypto cryptanalysis hash-auto --hash sha512-256
crypto cryptanalysis hash-auto --hash blake3
crypto cryptanalysis hash-auto --hash sha3-224
crypto cryptanalysis hash-auto --hash sha3-256
crypto cryptanalysis hash-auto --hash sha3-384
crypto cryptanalysis hash-auto --hash sha3-512
crypto hash shake128 --output-bytes 32 --hex 000102ff
crypto hash shake256 --output-bytes 32 --hex 000102ff
crypto cryptanalysis hash-auto --hash shake128 --output-bytes 32
crypto cryptanalysis hash-auto --hash shake256 --output-bytes 32
```

`hash-auto` runs a bounded 32-bit-truncated birthday search and a 4,096-trial
single-input-bit differential-bias measurement for each SHA-3 fixed-output
variant. These measurements concern truncated outputs and empirical bias;
they do not establish collisions or differential weaknesses for the full
digests. SHA-3 uses a sponge, so the command records that the
Merkle-Damgård length-extension method does not apply. SHA-256 has a
separate length-extension adapter with explicit prefix length and digest
input. SHAKE128 and SHAKE256 require an explicit `--output-bytes` value in
`1..=64` for `hash-auto`. The bounded analysis treats that many output bytes
as the digest, truncating the birthday search to at most 32 bits, and reports
per-byte differential measurements. The standalone `crypto hash` command
supports longer SHAKE outputs independently.

All fixed-output SHA-2 algorithms exposed by `crypto hash`, plus BLAKE3,
have bounded `hash-auto` birthday and differential reports. SHA-512 and
SHA-256 expose their complete chaining state and also support the explicit
`crypto cryptanalysis length-extension` command for a bare secret-prefix
digest with a supplied secret length. SHA-224, SHA-384, SHA-512/224,
SHA-512/256, and BLAKE3 have no length-extension adapter in this CLI; their
reports make no claim that these constructions resist every extension
technique. The birthday section concerns only a 32-bit truncated output.
