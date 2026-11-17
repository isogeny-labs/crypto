# Run the crypto and cryptanalysis CLIs locally

Install Rust/Cargo, a C/C++ compiler, CMake, Python 3 and the GitHub CLI.
Authenticate `gh` for the repositories. The S3 check near the end also needs
the AWS CLI and an AWS profile with access to the approved bucket.

Until crypto PR #2117 merges, use its PR head for the new crypto commands and
the merged `main` branch of cryptanalysis:

```sh
mkdir -p /path/to/cli-work
cd /path/to/cli-work
gh repo clone isogeny-labs/crypto
gh repo clone isogeny-labs/cryptanalysis
(cd crypto && gh pr checkout 2117)
(cd cryptanalysis && git switch main && git pull --ff-only)
```

After #2117 merges, use `git switch main && git pull --ff-only` in `crypto`
as well. Record `git rev-parse HEAD` in both checkouts beside any acceptance
results so the binaries can be tied to their exact source revisions.

Use a checkout of this repository for `CRYPTO_REPO` and a checkout of
`isogeny-labs/cryptanalysis` for `CRAX_REPO`. The latter builds the Rust `crax`
binary and the native C `ca` binary. Rebuild after changing either checkout;
an executable already in a target directory does not establish which source
revision produced it.

The commands below use separate Cargo target directories on an external volume
because each repository has a large build. Change the paths to suit your Mac.

```sh
export CRYPTO_REPO=/path/to/crypto
export CRAX_REPO=/path/to/cryptanalysis
export BUILD_TMP=/path/to/build-tmp
export CRYPTO_TARGET_DIR=/path/to/crypto-target
export CRAX_TARGET_DIR=/path/to/crax-target
mkdir -p "$BUILD_TMP" "$CRYPTO_TARGET_DIR" "$CRAX_TARGET_DIR"

(cd "$CRYPTO_REPO" && TMPDIR="$BUILD_TMP" CARGO_TARGET_DIR="$CRYPTO_TARGET_DIR" \
  cargo build --release --locked \
  --bin crypto --bin ic --bin icx --bin ecdlp-nist --bin isogeny_walk)
(cd "$CRAX_REPO" && tools/bootstrap_crypto_analysis_core.sh && \
  TMPDIR="$BUILD_TMP" CARGO_TARGET_DIR="$CRAX_TARGET_DIR" \
  cargo build --release --locked --manifest-path suite/Cargo.toml --bin crax)

export CRAX_CRYPTO_BIN="$CRYPTO_TARGET_DIR/release/crypto"
export CRAX_IC_BIN="$CRYPTO_TARGET_DIR/release/ic"
export CRAX_ICX_BIN="$CRYPTO_TARGET_DIR/release/icx"
export CRAX_ECDLP_NIST_BIN="$CRYPTO_TARGET_DIR/release/ecdlp-nist"
export CRAX_ISOGENY_WALK_BIN="$CRYPTO_TARGET_DIR/release/isogeny_walk"
export CRAX="$CRAX_TARGET_DIR/release/crax"

"$CRAX" --help
"$CRAX_IC_BIN" --help
```

To build the native C CLI and its delegation paths:

```sh
cmake -S "$CRAX_REPO" -B "$BUILD_TMP/cmake-ca" \
  -DCMAKE_BUILD_TYPE=Release -DCA_CUPQC=OFF
cmake --build "$BUILD_TMP/cmake-ca" --target ca -j 4
export CA="$BUILD_TMP/cmake-ca/ca"
export CA_CRYPTO_BIN="$CRAX_CRYPTO_BIN"
export CA_CRAX_BIN="$CRAX"
export CA_ICX_BIN="$CRAX_ICX_BIN"
export CA_ISOGENY_WALK_BIN="$CRAX_ISOGENY_WALK_BIN"
"$CA" prime 1019
```

## Quick checks

These use public known-answer inputs. Each solver reports whether its answer
was verified against the supplied generator and target. `ca ic` addresses a
prime field's multiplicative group; `ca icx` delegates to elliptic-curve IC.

```sh
"$CRAX" --json hash --algorithm sha256 --text abc
"$CRAX" --json hash --algorithm sha3-256 --text abc
"$CRAX" --json hash --algorithm shake256 --text abc --output-bytes 32
"$CRAX" --json aes block-encrypt \
  --key-hex 000102030405060708090a0b0c0d0e0f \
  --hex 00112233445566778899aabbccddeeff
"$CRAX" --json aes-attack --technique differential --samples 4096
"$CRAX" --json aes-attack --technique linear --samples 4096

"$CRAX_IC_BIN" solve-prime --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 --json
"$CRAX_IC_BIN" solve-cubic \
  --input "$CRYPTO_REPO/docs/ic/fixtures/cubic_f11_public_target.json" --json
"$CRAX_ICX_BIN" cubic-graph \
  --input "$CRYPTO_REPO/docs/ic/fixtures/cubic_f11_public_target.json" \
  --include-degree-two --max-ell 5 --max-nodes 64 --json
"$CRAX_ECDLP_NIST_BIN" solve P-256 --secret 12345 --interval-bits 16 --json
"$CA" icx solve-prime --p 271 --a 1 --b 19 --n 281 \
  --gx 3 --gy 7 --qx 231 --qy 54 --json
```

Expected results include SHA-256 of `abc` =
`ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad`,
SHA3-256 of `abc` =
`3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532`,
AES block ciphertext `69c4e0d86a7b0430d8cdb78070b4c55a`, verified
prime-field scalar 190, verified cubic-field scalar 5, 15 cubic graph nodes,
and verified bounded P-256 scalar 12345.

The [external-target guide](ic/EXTERNAL_TARGET_CLI.md) covers supplying your
own prime-field or cubic-field curve, target, and certified isogeny route.
The [feature grid](CLI_FEATURE_COVERAGE.md) states the supported algorithms,
work bounds, and validation limits. A graph edge by itself does not certify
transport of an ECDLP target; use the routed solve and check its certificate.

To generate and use a verified two-hop prime-field route, then replay a fresh
isogeny walk:

```sh
"$CRAX_IC_BIN" prime-route-generate --p 107 --a 3 --b 5 --n 31 \
  --gx 11 --gy 37 --qx 16 --qy 61 --degree 2 \
  --max-curves 8 --max-depth 2 --json > "$BUILD_TMP/prime-route.json"
ROUTE_ID=$(python3 -c 'import json,sys; routes=json.load(open(sys.argv[1]))["routes"]; print(next(r["id"] for r in routes if r["status"] == "verified" and len(r["edge_ids"]) == 2))' "$BUILD_TMP/prime-route.json")
"$CRAX_IC_BIN" solve-prime --p 107 --a 3 --b 5 --n 31 \
  --gx 11 --gy 37 --qx 16 --qy 61 \
  --route-file "$BUILD_TMP/prime-route.json" --route-id "$ROUTE_ID" \
  --rho-steps 0 --json
"$CRAX_IC_BIN" solve-cubic \
  --input "$CRYPTO_REPO/docs/ic/fixtures/cubic_f11_routed_solve_target.json" \
  --route-file "$CRYPTO_REPO/docs/ic/fixtures/cubic_f11_routed_solve_route.json" \
  --route-id root-to-velu2 --skip-rho --json

export WALK_DIR="$BUILD_TMP/isogeny-walk-f271"
"$CRAX_ISOGENY_WALK_BIN" walk --curve custom --p 271 --a 1 --b 19 \
  --order 281 --cofactor 1 --gx 3 --gy 7 --name prime271-a1-b19 \
  --primes 11 --max-curves 16 --max-depth 1 --threads 1 \
  --audit-points 1 --out "$WALK_DIR"
"$CRAX_ISOGENY_WALK_BIN" verify --curve custom --p 271 --a 1 --b 19 \
  --order 281 --cofactor 1 --gx 3 --gy 7 --name prime271-a1-b19 \
  --audit-points 2 --dir "$WALK_DIR"
```

The two routed solves return verified scalar 5. The walk produces three curves
and two edges, and `verify` reports `result: pass`.

## S3 artifact acceptance

The approved bucket is `setec-astronomy-artifacts`. Use a unique disposable
prefix for acceptance and `ic/v1/runs/` for retained artifacts. Your selected
AWS profile needs write, HEAD, read, and delete access to the acceptance prefix.

```sh
export AWS_PROFILE=your-profile
export AWS_DEFAULT_REGION=us-east-2
export RUN_ID="local-$(date -u +%Y%m%dT%H%M%SZ)-$$"
export RUN_DIR="$BUILD_TMP/crypto-store-$RUN_ID"
mkdir -p "$RUN_DIR"
export CRYPTO_STORE_REMOTE="s3://setec-astronomy-artifacts/ic/v1/acceptance/$RUN_ID"
export CRYPTO_STORE_INDEX="$RUN_DIR/index.json"
export CRYPTO_STORE_CACHE="$RUN_DIR/cache"
printf 'local artifact check %s\n' "$RUN_ID" > "$RUN_DIR/input.bin"

"$CRAX_CRYPTO_BIN" store push "$RUN_DIR/input.bin" \
  --name "acceptance/$RUN_ID/input.bin" --json
"$CRAX_CRYPTO_BIN" store verify "acceptance/$RUN_ID/input.bin" \
  --remote --download --json
CRYPTO_STORE_CACHE="$RUN_DIR/fresh-cache" \
  "$CRAX_CRYPTO_BIN" store pull "store:acceptance/$RUN_ID/input.bin" \
  --out "$RUN_DIR/retrieved.bin" --json
cmp "$RUN_DIR/input.bin" "$RUN_DIR/retrieved.bin"
aws s3 rm "$CRYPTO_STORE_REMOTE/" --recursive
```

The `store` [protocol guide](store/README.md) describes content verification
and publication. The [exact-head acceptance receipt](store/PR2117_EXACT_HEAD_S3_ACCEPTANCE_20261025.md)
also covers prime and cubic target artifacts and isogeny-walk
publish/exists/fetch/replay. The commands above exercise a fresh upload and
download using the currently built binary; perform them after building from
the merged revision for a release check.
