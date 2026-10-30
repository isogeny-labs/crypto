# Root-finding decomposition engine in ecbench: results

For the Bochum v0.2 hashed-factor-base algorithm, see the
[native Rust tensor implementation](docs/hashed-ecdl.md): compact encodings,
exact contraction, relation collection and certificate replay. Correctness
controls do not establish the conditional asymptotic bound or a speedup.

The [Hexagon complexity integration](research/hexagon_cscc_20261030/README.md)
adds exact divisor-prefix factorization, bounded weighted cost profiles, three
prime-field factor-base recipes, and a searchable, version-pinned review of all
22 works in the `cs.CC` collection on 2026-10-30. Its implementation and proof
statuses are recorded separately.

The separately scoped [scalar tensor companion](docs/hashed-ecdl.md#scalar-tensor-companion)
in cryptanalysis adds binary/prime-field coordinate checks and Certicom ECCp-131
inputs. No Python execution path is introduced here and neither challenge is solved.

**Decision: engineering.** The new oracles verify, replay identically, and
match the query stream of the existing table/subtract arms; they do not
change the asymptotic group-operation count of the decomposition, so
nothing here is an *advance*. Against Gröbner (`inherited-f4`) the root
finder's `S` is a complete count beside a lower bound and is not a
speedup. Against `mitm` / `subtract` the ops ratio is 1 (same stream);
wall time on this L0 macOS host is descriptive only and favours the root
finder. No security claim; no ECC2K-130 inference.

| | |
|---|---|
| protocol | [`PROTOCOL.md`](PROTOCOL.md) (committed before any session ran) |
| engine ids | `pdp3-koblitz:m=2\|3,engine=rootfind`; `rootfind-subtract:m=2\|3` (see `docs/ecbench/README.md` §7) |
| specs | `docs/ecbench/specs/smoke-koblitz-rootfind.json`, `smoke-prime-rootfind.json`, `claim-koblitz-rootfind-n13.json`, `claim-koblitz-rootfind-n19.json`, `panel-prime-rootfind.json` |
| binary | `ecbench` release SHA-256 `1e972df4d8e769629ad4fd4a023214b05ee513a2cfea6f3ef51f2ca9e61fe531` |
| host | Apple M4 Pro, 14 logical CPUs, macOS 25.6.0 aarch64; env class `ECBENV2h59034d4cd61b`; `--cpus none --allow-busy` → every run **L0** |
| classification | **engineering** (same decomposition work, faster constant / cleaner charge); accounting vs Gröbner lower bounds |

## Complete sessions

| session | id | records | audit | replay cert (SHA-256 of audit JSON) |
|---|---|---:|---|---|
| [`sessions/smoke-koblitz`](sessions/smoke-koblitz) | `ECBS1hd86e5cfc2b77` | 42/42 verified | OK | `1cb597b686e68d783c8c370806686740ef316e862ebd191b6482470f2605c2b5` |
| [`sessions/smoke-prime`](sessions/smoke-prime) | `ECBS1hb8f2623c82e2` | 36/36 verified | OK | `bfb3f0207a2ae4f57ee8e176bd3cd05e44b8eb0d108340f70d4323e647188a1f` |
| [`sessions/claim-n13`](sessions/claim-n13) | `ECBS1hfd5bf1885227` | 224/224 verified | OK at `2245b75`, 168/168 replays identical; **pre-#2043**: replayable only by its recording binary (lazy inherited-F4 charge) | `8e6a76596041d9a182e2584b9844ecf4846d827e32fbb1c9e3bd3d68e36a19ae` |
| [`sessions/claim-n13-v2`](sessions/claim-n13-v2) | `ECBS1h396db1437f4a` | 224/224 verified | OK on `main` `2b9f3f72f`, 168/168 replays identical, run twice; accounting version 2 | `59e7e2bf46ffaa1716e069e1a400d0f77e5bff43b87b0729b4e7f902d76e3bac` |
| [`sessions/panel-prime`](sessions/panel-prime) | `ECBS1h045961daac5c` | 768/768 verified | OK, 576/576 replays identical | `19d4aba584a99a48c46fd7b70262d31972a6d11770771826e2c5ac10a49d74ce` |
| [`sessions/claim-n19`](sessions/claim-n19) | `ECBS1h52e3269d5dab` | 224/224 verified | OK, 168/168 replays identical | `f3a3105be311cee69d7c2328a283a3c7632739136d832fd86d9de6e30998136d` |

Incomplete starts kept under [`abandoned/`](abandoned/) (SIGKILL / shell detach
at mid-session; status left as recorded).

## Boundary and unit

As in [`PROTOCOL.md`](PROTOCOL.md): floor `√(π/2A)` in `S`; reference
`rho.signed_frobenius_strong` (Koblitz) / `rho.negation` (prime); cold
`S` with every phase charged. Gröbner arms mark `lower bound` (word XORs
counted, not charged). Root-finding Koblitz arms are complete counts.

## Tables (mean `S`, L0)

Full `ecbench table` output: [`tables/`](tables/). Headline cells:

### Koblitz `E_0`, `n=13`, `d=5` (`claim-n13`, 8 public targets × 3 rounds)

| arm | oracle | mean S | S / strong-rho | lower bound |
|---|---|---:|---:|---|
| `rho-strong` | — | 14.118 | 1.000 | yes |
| `rho-frob` | — | 1.871 | 0.133 | yes |
| `ic-f4-m3` | `pdp3-koblitz:m=3,engine=inherited-f4,…` | 15.928 | 1.128 | yes |
| `ic-rootfind-m3` | `pdp3-koblitz:m=3,engine=rootfind` | 38.822 | 2.750 | **no** |
| `ic-f4-m2` | `pdp3-koblitz:m=2,engine=inherited-f4,…` | 16.642 | 1.179 | yes |
| `ic-rootfind-m2` | `pdp3-koblitz:m=2,engine=rootfind` | 28.383 | 2.010 | **no** |

`ic-rootfind-m3 / ic-f4-m3` ops = 2.44 [2.35, 2.53] is **diagnostic only**
(complete count over a lower bound). Descriptive solve-wall sums over the
24 measured runs: F4-m3 5.70 s vs rootfind-m3 0.13 s (~43×); F4-m2 0.45 s
vs rootfind-m2 0.12 s (~3.6×). Not admitted (L0).

**Re-recorded under accounting version 2 (`claim-n13-v2`, 2026-10-28).** The owner made
the eager inherited-F4 charge (post-#2043, version 2) the method identity, so the table above
is pre-#2043. The same spec on `main` gives identical rho and root-finder S. The inherited-F4
arms are now complete counts rather than lower bounds: `ic-f4-m3` 534.859 [476.5, 600.4]
(37.9× strong rho), `ic-f4-m2` 35.758 [30.5, 41.9] (2.53×). That is an accounting change, not a
regression. With both engines complete, `ic-rootfind / ic-f4` ops is 0.0726 (m3) and 0.7938 (m2),
but the §5a band (0, 0.1 and 1 GAE per unpriced operation) is unresolved for both pairs, so
neither engine ranks above the other. The IC replay plan and the backend leaderboard
(`docs/backend-leaderboard/`) cite `claim-n13-v2`. Report, graph and PDF:
[`reports/run-20261028T0403Z/`](reports/run-20261028T0403Z/REPORT.md).

Smoke Koblitz: `ic-rootfind-m2 / ic-mitm-m2` ops = **1.000** [1.000, 1.000]
(identical `S` and query stream).

### Koblitz `E_0`, `n=19`, `d=7` (`claim-n19`, 8 public targets × 3 rounds)

| arm | mean S | S / strong-rho | lower bound |
|---|---:|---:|---|
| `rho-strong` | 3.163 | 1.000 | yes |
| `ic-f4-m3` | 4.904 | 1.550 | yes |
| `ic-rootfind-m3` | 53.996 | 17.072 | yes |
| `ic-f4-m2` | 8.097 | 2.560 | yes |
| `ic-rootfind-m2` | 34.688 | 10.967 | yes |

Two sizes are not a scaling claim. Rootfind remains above rho and above the
Gröbner lower-bound `S` (complete charged table work vs uncharged solver XORs).

### Prime panel (18/20/22/24-bit `prime_search`, seeds of
`research/ecbench_all_candidates_20261003/spec-prime.json`)

| arm | 18-bit mean S (ref×) | 20-bit | 22-bit | 24-bit |
|---|---:|---:|---:|---:|
| `rho-neg` | 1.294 (1.00) | 1.344 | 1.480 | 1.080 |
| `ic-subtract` | 310 (239×) | 449 | 927 | 2775 |
| `ic-rootfind-m2` | 310 (239×) | ≈subtract | ≈subtract | ≈subtract |
| `ic-mitm` | 9.96 (7.7×) | 10.5 | 16.2 | 43.7 |
| `ic-rootfind-m3` | (see table) | | | |

`ic-rootfind-m2 / ic-subtract` ops = **1.000** [1.000, 1.000] over 96 pairs
(same candidates, same relations; GAE differs by one re-addition per
relation; `inversions` counted uncharged). Descriptive solve-wall:
subtract / rootfind-m2 ≈ 2.5× over the panel.

No IC arm is below rho on these curves.

## Hypotheses (from PROTOCOL)

- **H1:** every root-finding run verified; every audit replay `identical`. **Met.**
- **H2:** same stream as control oracles where the control completes. **Met**
  for `rootfind-m2` vs `mitm-m2` (Koblitz smoke) and vs `subtract` (prime panel).
- **H3:** root-finding solve wall below Gröbner at `m=3` on completed
  workloads. **Met descriptively** on claim-n13 (L0, not admitted).

## Inadmissible / caveats

- All wall figures are L0 on a shared macOS host (load often ≫ 1); do not
  cite them as admitted speedups.
- Two Koblitz sizes ≠ a scaling claim (AGENTS.md §5).
- Do not divide rootfind `S` by Gröbner `S`.
- Interrupted first attempts of claim-n13 are under `abandoned/`, not in the scoreboard.

## Reproduce

```bash
# Build
cargo build --release

# Run the protocol demos
cargo run --release -- demo

# Hash a string
cargo run --release -- hash sha256 "hello world"

# Auto-run every applicable cryptanalysis attack against a named cipher
cargo run --release -- cryptanalysis auto --cipher toyspn-2r
cargo run --release -- cryptanalysis auto --cipher aes-2r

# Discover what attacks/ciphers are registered
cargo run --release -- cryptanalysis list-ciphers

# Run the full empirical research bench
cargo run --release -- cryptanalysis bench

# Tests (all 843 of them)
cargo test --release
```

---

## Index-calculus research executable

The standalone ic command supports named/custom curve inspection, generated
known-answer experiments, and bounded factor-base comparisons with live
stage reporting.

    cargo build --release --bin ic
    ./target/release/ic ecc2k-130
    ./target/release/ic run --degree 11 --curve-a 1 --known-log 53 --solver enumerate
    ./target/release/ic generate --degree 9 --seed 42 --out fixture.json
    ./target/release/ic inspect --file fixture.json
    ./target/release/ic compare --degree 7 --curve-a 1 --samples 3 --holdout 2 --json

Imported points are inspected only. Synthetic runs retain the library's
small-curve limits and verify their known answers. Comparison reports include
unsuccessful candidates, separate holdout checks, time and memory measurements,
and explicitly bounded conclusions.

See [the ic guide](docs/ic/README.md) for the parameter schema, capabilities,
limits, resource accounting, and report semantics. The original crypto
command remains the default for cargo run.

The [EC index-calculus baseline](research/index_calculus_baseline_20260914/README.md)
adds counting bounds, an operation-accounting contract, and a certified WDSat control
with frozen measurements and reproducible commands.

## Elliptic-curve surface tools

[`tools/ecdlp-surfaces`](tools/ecdlp-surfaces/README.md) is a standalone Rust
crate with three command-line tools: `surface_search` enumerates containing
quadrics, `transfer_check` checks the transported group law and prime subgroup,
and `frobenius_filter` applies necessary conditions to supplied abelian-surface
polynomials. Point arithmetic uses small prime fields, up to 10,000. Equivalent
embeddings preserve the ECDLP; no destination-solver speedup is established.
The [report](docs/ecdlp-surfaces/REPORT.md) includes the diagram, PDF, fixtures,
and native identity certificates. Run `tools/ecdlp-surfaces/check.sh` to validate
the crate and replay its examples.

For genus-2 and higher Jacobian transfers, use the
[`ecdlp-surfaces` skill](.agents/skills/ecdlp-surfaces/SKILL.md) and its
[higher-genus reference](.agents/skills/ecdlp-surfaces/references/higher-genus.md)
for the existing cover/arithmetic tools, native pullback APIs, and construction
and subgroup-validation requirements.

## Isogeny walks

`isogeny_walk` walks a prime-field curve's isogeny class (P-256, P-224,
P-192 or a custom curve).

- **Certified.** Every isogeny is certified by an explicit kernel
  polynomial, and every curve is recorded with its ICV1/EC1 identity and
  traits.
- **Stored.** Runs can be stored in S3, write-once and hash-checked.

    cargo build --release --bin isogeny_walk
    ./target/release/isogeny_walk walk --curve p256 --max-ell 61 --max-curves 2000 --out runs/p256
    ./target/release/isogeny_walk verify --curve p256 --dir runs/p256
    ./target/release/isogeny_walk walk --curve p256 --max-ell 61 --max-curves 20000 \
        --store s3://crypto-autoresearcher/isogeny-walk --prune-local --out runs/p256-20k

See [the isogeny-walk guide](docs/isogeny-walk/README.md) for:
- every command;
- S3 setup and IAM;
- offline use;
- sizing;
- what the tool does and does not establish.

## Algorithm coverage

### Symmetric primitives

| Module                    | Algorithm                            | Standard / Origin           |
|---------------------------|--------------------------------------|-----------------------------|
| `symmetric::aes`          | AES-128 / AES-256 + CTR + GCM        | FIPS 197 / SP 800-38A,D     |
| `symmetric::aes_cbc`      | AES-CBC                              | NIST SP 800-38A             |
| `symmetric::chacha20`     | ChaCha20 + Poly1305 + AEAD           | RFC 8439                    |
| `symmetric::serpent`      | Serpent block cipher                 | Anderson–Biham–Knudsen 1998 |
| `symmetric::threefish`    | Threefish (Skein tweakable cipher)   | Skein NIST submission        |
| `symmetric::sm4`          | SM4 Chinese national block cipher    | GB/T 32907                  |
| `symmetric::gost_magma`   | GOST R 34.12-2015 Magma (64-bit)     | GOST R 34.12-2015           |
| `symmetric::kuznyechik`   | GOST R 34.12-2015 Kuznyechik (128-bit) | GOST R 34.12-2015           |
| `symmetric::cmac`         | CMAC-AES MAC                          | NIST SP 800-38B             |
| `symmetric::des`          | DES (Data Encryption Standard, broken — teaching) | FIPS PUB 46-3 |
| `symmetric::des3`         | Triple-DES (EDE3, deprecated by NIST 2017)   | NIST SP 800-67  |
| `symmetric::rc4`          | RC4 stream cipher (broken — teaching) | Rivest 1987 / RFC 6229 vectors |
| `symmetric::rc5`          | RC5-32/12/16 block cipher             | Rivest, FSE 1994            |

### Block-cipher modes (generic over `BlockCipher<N>` trait)

The `symmetric::modes` submodule hosts modes that work against any
[`BlockCipher<N>`] implementor — AES, SM4, Serpent, Kuznyechik, Magma.

| Mode                            | Type                           | Standard               |
|---------------------------------|--------------------------------|------------------------|
| `modes::ecb` (PKCS#7 padded)    | Confidentiality (broken — teaching only) | NIST SP 800-38A §6.1 |
| `modes::cfb`                    | Self-synchronising stream      | NIST SP 800-38A §6.3   |
| `modes::ofb`                    | Pre-computable keystream       | NIST SP 800-38A §6.4   |
| `modes::ccm`                    | **AEAD** (Counter + CBC-MAC)   | NIST SP 800-38C / RFC 3610 |
| `modes::siv`                    | **AEAD, misuse-resistant** (deterministic) | RFC 5297            |
| `modes::eax`                    | **AEAD** (three-CMAC construction)         | Bellare-Rogaway-Wagner FSE 2004 |
| `modes::ocb3`                   | **AEAD** (single-pass, offset-codebook)    | RFC 7253 |
| `modes::gcm_siv` (+ POLYVAL)    | **AEAD, nonce-misuse-resistant** (GCM variant) | RFC 8452 |
| `modes::kw`                     | Deterministic key wrap         | NIST SP 800-38F / RFC 3394 |
| `modes::xts`                    | Disk encryption (ciphertext stealing) | IEEE P1619 / NIST SP 800-38E |

### Hashes / XOFs

| Module           | Algorithm                  | Standard                |
|------------------|----------------------------|-------------------------|
| `hash::md4`      | MD4 (broken — teaching/target)  | RFC 1320            |
| `hash::md5`      | MD5 (broken — teaching/target)  | RFC 1321            |
| `hash::sha256`   | SHA-224 / SHA-256          | FIPS 180-4              |
| `hash::sha512`   | SHA-384 / SHA-512          | FIPS 180-4              |
| `hash::sha3`     | SHA3-256/512, SHAKE128/256 | FIPS 202 (Keccak)       |
| `hash::blake3`   | BLAKE3                     | BLAKE3 spec (via crate) |
| `hash::ripemd160`| RIPEMD-160                 | ISO/IEC 10118-3         |
| `hash::siphash`  | SipHash-2-4                | Aumasson–Bernstein 2012 |
| `hash::sm3`      | SM3 Chinese national hash  | GB/T 32905              |
| `hash::streebog` | GOST R 34.11-2012 (256/512)| GOST R 34.11-2012       |

### Hash-function cryptanalysis

Generic attacks over the `HashFunction` / `MerkleDamgardHash` trait:

| Attack                                  | Module                     | Notes                                |
|----------------------------------------|----------------------------|--------------------------------------|
| **Length-extension attack**            | `cryptanalysis::hash_attacks::length_extension_attack` | Works against any MD-Damgård hash used as `H(secret \|\| msg)` MAC.  Tested on MD4, MD5, SHA-1. |
| **Birthday-collision search**          | `cryptanalysis::hash_attacks::birthday_collision_search` | Generic `√(2^b)` collision finder over a truncated hash. |
| **Joux multicollision** (CRYPTO 2004)  | `cryptanalysis::hash_attacks::joux_multicollision` | Chain `t` collisions → `2^t` equivalent messages at cost `t · 2^(b/2)`. |
| **Differential bias**                  | `cryptanalysis::hash_attacks::differential_bias` | Per-output-byte bias measurement on a chosen input differential. |
| **MD5 Wang differential**              | `cryptanalysis::md5_differential` | Reduced-round near-collision finder. |
| **SHA-1 differential**                 | `cryptanalysis::sha1_differential` | Round-function table + near-collision search. |

CLI:

```bash
# Isogeny-volcano tools (docs/volcano/README.md)
crypto volcano profile --koblitz-n 83 --ell 6473        # conductor, depth, level tables
crypto volcano koblitz-step --n 131 --ell 263          # ECC2K-130: the 262 level-1 neighbours over F_(2^131)

# Auto-run every applicable hash attack
crypto cryptanalysis hash-auto --hash md5
crypto cryptanalysis hash-auto --hash md4
crypto cryptanalysis hash-auto --hash sha1

# Length-extension demo (interactive)
crypto cryptanalysis length-extension --hash sha1 \
    --secret-len 16 --message-hex "&role=guest" \
    --suffix-hex "&role=admin" --digest-hex <hex>

# AES related-key attack: key-schedule diffusion + avalanche + local collision
crypto cryptanalysis aes-related-key --key-bits 256 --rounds 4
crypto cryptanalysis aes-related-key --key-bits 128 --rounds 4 --avalanche-trials 1024

# AES visual demos — ASCII art for every major attack
crypto cryptanalysis aes-visual-demo --demo all              # all four below
crypto cryptanalysis aes-visual-demo --demo truncated-diff   # 4-round trail grid
crypto cryptanalysis aes-visual-demo --demo integral         # 3-round Square balance
crypto cryptanalysis aes-visual-demo --demo dfa              # Piret-Quisquater fault diff
crypto cryptanalysis aes-visual-demo --demo key-schedule     # AES-128 vs AES-256 diffusion

# Visual demos for every NON-AES attack module
crypto cryptanalysis visual-all --target all                 # full bundle (Markdown)
crypto cryptanalysis visual-all --target sbox-ddt            # DDT heat-maps (AES + Serpent + PRESENT)
crypto cryptanalysis visual-all --target sbox-lat            # LAT heat-maps
crypto cryptanalysis visual-all --target walsh               # Walsh-Hadamard signed spectrum bars
crypto cryptanalysis visual-all --target pollard-rho         # ρ-iteration trajectory plot
crypto cryptanalysis visual-all --target hnp                 # HNP recovery probability curve
crypto cryptanalysis visual-all --target bleichenbacher      # nonce-bias histogram
crypto cryptanalysis visual-all --target length-extension    # MD-Damgård chain diagram
crypto cryptanalysis visual-all --target joux                # multicollision tree
crypto cryptanalysis visual-all --target j0-twists           # twist factor-size bars
crypto cryptanalysis visual-all --target birthday            # cumulative collision-probability curve
```

### Library-wide visual demos

Beyond the cryptanalysis-side visualizations, every major module
ships ASCII-art demos:

```bash
# Symmetric ciphers
crypto visual symmetric --target ecb-penguin   # leak structure through ECB
crypto visual symmetric --target modes         # CBC/CFB/OFB/CTR/GCM/CCM/SIV/XTS/KW dataflow
crypto visual symmetric --target aes-trace     # 11 AES-128 state grids round-by-round
crypto visual symmetric --target chacha        # ChaCha20 4×4 word state evolution
crypto visual symmetric --target avalanche     # AES avalanche heat-map at 1/2/3/5/10 rounds

# Hash functions
crypto visual hash --target md                 # Merkle-Damgård chain (MD5/SHA-2/RIPEMD)
crypto visual hash --target sponge             # SHA-3 / SHAKE sponge construction
crypto visual hash --target blake3             # BLAKE3 binary-tree mode
crypto visual hash --target sha256-trace       # SHA-256 working-variable trace
crypto visual hash --target avalanche          # MD5 vs SHA-256 single-bit avalanche

# Elliptic curves
crypto visual ecc --target curve               # scatter plot of points on toy F_p curve
crypto visual ecc --target ops                 # point-add and point-double geometric diagrams
crypto visual ecc --target ladder              # double-and-add scalar-mul trace
crypto visual ecc --target ecdsa               # ECDSA sign/verify dataflow

# Everything at once
crypto visual all

# Color control
crypto --color always visual symmetric --target ecb-penguin    # force colors
crypto --color never  visual ecc       --target curve          # plain text
CRYPTO_COLOR=always crypto visual hash --target sponge         # via env var
NO_COLOR=1 crypto visual all                                   # standard NO_COLOR
```

**ANSI colors**: every visual renderer emits ANSI colors when stdout
is a TTY.  Heat-maps grade `dim → blue → green → yellow → red` (low
to high); state grids highlight non-zero bytes in **yellow**; active
patterns paint `▓` in **bright red**; signed bars are **green** for
positive, **red** for negative; recovery progress bars are **green
filled, dim unfilled**; verdict markers (`✓ ✗ ⚠`) get their canonical
colors.  Set `NO_COLOR=1` or `--color never` to suppress.

### Visual output samples

The `aes-visual-demo` subcommand renders ASCII diagrams.  For example,
`--demo truncated-diff` shows the classical wide-trail AES diffusion:

```
        input      R1 SR      R2 SR      R3 SR      R4 SR
  row 0    ▓ · · ·    ▓ · · ·    ▓ · · ·    ▓ ▓ ▓ ▓    ▓ ▓ ▓ ▓
  row 1    · · · ·    · · · ·    · · · ▓    ▓ ▓ ▓ ▓    · · · ·
  row 2    · · · ·    · · · ·    · · ▓ ·    ▓ ▓ ▓ ▓    · · · ·
  row 3    · · · ·    · · · ·    · ▓ · ·    ▓ ▓ ▓ ▓    · · · ·

**Active S-boxes per round**

round  0 │█                              1
round  1 │███████                        4
round  2 │██████████████████████████████ 16
round  3 │███████                        4

**Total active S-boxes** N = 25, Pr ≤ 2⁻¹⁵⁰
```

The same machinery powers every other attack's test output — run any
`#[ignore]`-flagged `demo_*` test with `--nocapture` to see it.

### Elliptic-curve cryptography

| Module                | Algorithm / Curve                                           | Standard               |
|-----------------------|-------------------------------------------------------------|------------------------|
| `ecc::field` / `point`| Generic short-Weierstrass arithmetic                        | SEC 1                  |
| `ecc::curve`          | secp256k1, P-256                                            | SEC 2, FIPS 186-4      |
| `ecc::curve_zoo`      | Brainpool {160,192,224,256,320,384,512} r1, FRP256v1, NIST P-{192,224,384,521}, SECG k-curves, GOST tc26 paramsets | RFC 5639 / FIPS 186-4 / SEC 2 / GOST |
| `ecc::p256_field`     | Optimised P-256 field arithmetic (Solinas reduction)        | FIPS 186-4             |
| `ecc::secp256k1_*`    | Optimised secp256k1 field + Jacobian coordinates            | SEC 2                  |
| `ecc::ct`             | Constant-time scalar multiplication primitives              | —                      |
| `ecc::curve25519`     | Curve25519 prime field                                      | RFC 7748               |
| `ecc::x25519`         | X25519 ECDH                                                 | RFC 7748               |
| `ecc::ed25519`        | Ed25519 EdDSA signatures                                    | RFC 8032               |
| `ecc::ecdsa`          | ECDSA with RFC 6979 deterministic nonces                    | FIPS 186-4 + RFC 6979  |
| `ecc::barrett_ecdsa`  | ECDSA with Barrett-reduced fields                           | —                      |
| `ecc::ecdh`           | ECDH key exchange                                           | RFC 8422               |
| `ecc::schnorr`        | Schnorr signatures (BIP-340 flavour)                        | BIP-340                |
| `ecc::sm2`            | SM2 signature + encryption + ZA                             | GB/T 32918             |
| `ecc::gost_3410_2012` | GOST R 34.10-2012 signature (256/512)                       | GOST R 34.10-2012      |
| `ecc::ec_kcdsa`       | Korean EC-KCDSA (Q = x⁻¹·G convention)                       | TTAS.KO-12.0011        |
| `binary_ecc`          | Binary-field ECC over F_{2^m}                                | NIST B-curves          |

### Pairings

| Module        | Algorithm                                | Notes                       |
|---------------|------------------------------------------|-----------------------------|
| `bls12_381`   | BLS12-381 pairing-friendly curve         | Optimal Ate pairing, Fq/Fq2/Fq6/Fq12 towers |

### Asymmetric / public-key

| Module                      | Algorithm                          | Standard         |
|-----------------------------|------------------------------------|------------------|
| `asymmetric::rsa`           | RSA keygen (Miller-Rabin), enc, sig, CRT-CT decrypt | PKCS#1 / RFC 8017 |
| `asymmetric::paillier`      | Paillier additively-homomorphic encryption | Paillier 1999 |
| `asymmetric::elgamal`       | ElGamal encryption (multiplicative group) | ElGamal 1985 |
| `asymmetric::ec_elgamal`    | EC-ElGamal (point-on-curve homomorphic) | — |

### Key derivation / MAC

| Module          | Algorithm                       | Standard   |
|-----------------|---------------------------------|------------|
| `kdf::hkdf`     | HMAC-SHA256, HKDF extract+expand| RFC 5869   |
| `kdf::pbkdf2`   | PBKDF2-HMAC-SHA256              | RFC 8018   |
| `kdf::hmac_drbg`| HMAC-DRBG deterministic RNG     | NIST SP 800-90A |

### Post-quantum (educational)

| Module                       | Algorithm                                | Notes                              |
|------------------------------|------------------------------------------|------------------------------------|
| `pqc::ml_kem`                | ML-KEM-512/768/1024 (Kyber)              | FIPS 203; NIST ACVP KATs pass      |
| `pqc::ml_dsa`                | ML-DSA-65 (Dilithium)                    | FIPS 204; NIST ACVP KATs pass      |
| `pqc::slh_dsa`               | SLH-DSA (SPHINCS+)                       | FIPS 205                           |
| `pqc::sqisign`               | SQIsign (isogeny-based signature)        | Round-3 on-ramp; toy p = 431, BFS in place of KLPT |
| `pqc::uov`                   | UOV (unbalanced oil & vinegar)           | Round-3 on-ramp; toy GF(256) params |
| `pqc::mayo`                  | MAYO (whipped UOV)                       | Round-3 on-ramp; toy GF(16) params |
| `pqc::qr_uov`                | QR-UOV (quotient-ring UOV)               | Round-3 on-ramp; F_31³ blocks, toy |
| `pqc::snova`                 | SNOVA (UOV over a matrix ring)           | Round-3 on-ramp; M₂(GF(16)), toy   |
| `pqc::hawk`                  | HAWK (lattice-isomorphism signature)     | Round-3 on-ramp; toy dim 32        |
| `pqc::fn_dsa`                | FN-DSA / Falcon (NTRU hash-and-sign)     | Draft FIPS 206; toy n = 16, Babai for ffSampling |
| `pqc::sdith`                 | SDitH (syndrome decoding in the head)    | Round-3 on-ramp; MPC-in-the-head, toy |
| `pqc::mqom`                  | MQOM (MQ on my mind)                     | Round-3 on-ramp; MPC-in-the-head, toy |
| `pqc::faest`                 | FAEST (AES in the head)                  | Round-3 on-ramp; MPC-in-the-head, toy |
| `pqc::mpcith`                | Shared MPC-in-the-head proof engine      | Underlies SDitH / MQOM / FAEST     |
| `pqc::kyber`                 | Kyber / ML-KEM (simplified toy)          | Not FIPS-203 compatible            |
| `pqc::frodo`                 | FrodoKEM (LWE-based)                     | Conservative simplified            |
| `pqc::ntru`                  | NTRU lattice cryptosystem                | NTRU encrypt 1996                  |
| `pqc::ntru_prime`            | NTRU Prime (Streamlined / NTRU LPRime)   | NTRU Prime spec                    |
| `pqc::mceliece`              | Educational binary-Goppa McEliece (toy params) | Original 1978 |
| `pqc::classic_mceliece`      | Classic McEliece (NIST round-3) skeleton | NIST round-3 spec                  |
| `pqc::hqc`                   | HQC (code-based KEM)                     | NIST round-3 / round-4             |
| `pqc::bike`                  | BIKE (QC-MDPC code-based)                | NIST round-3                       |
| `pqc::csidh`                 | CSIDH (isogeny-based) — group action     | Castryck–Lange–Martindale–Panny–Renes 2018 |
| `pqc::x_wing`                | X-Wing hybrid (X25519 + ML-KEM)          | draft-connolly-cfrg-xwing-kem      |
| `pqc::fast::ml_kem`          | ML-KEM-512/768/1024, speed-oriented      | Differentially tested against `pqc::ml_kem` |
| `pqc::fast::ml_dsa`          | ML-DSA-65, speed-oriented                | Differentially tested against `pqc::ml_dsa` |
| `pqc::fast::keccak`          | Unrolled Keccak-f[1600] and its sponges  | 2.4x the reference SHA3-256        |
| `pqc::fast::isogeny`         | SQIsign-scale F_p² and 2-power isogenies | p = 3·2³²⁴ − 1, 326 bits; a 2³²⁴-isogeny in 1.8 ms |

Every scheme in the table is reachable from the command line:

```bash
# What is here, and what parameters each one runs at
crypto pqc list

# Run one scheme end to end (keygen, use, and a negative check), or all 25
crypto pqc run --scheme ml-kem-768
crypto pqc run --scheme all

# The guided tour, with commentary
crypto pqc

# The speed-oriented implementations: self-tests, benchmarks, a real
# 2^324-isogeny walk
crypto pqc-fast selftest
crypto pqc-fast bench
crypto pqc-fast isogeny --op chain
```

`pqc run` is a check, not a demo: a KEM must agree on the shared secret *and*
give a different one for a tampered ciphertext; a signature must verify its own
output *and* reject a tampered one. Measured costs for the `fast` modules are in
[`docs/pqc-speed.md`](docs/pqc-speed.md).

### Cryptanalysis of ML-KEM and ML-DSA

Lattice-attack estimates at real parameters, working sieves, and the
implementation attacks that actually recover keys. Full write-up in
[`docs/mlwe-cryptanalysis.md`](docs/mlwe-cryptanalysis.md).

| Module                          | What it does                                                            |
|---------------------------------|-------------------------------------------------------------------------|
| `cryptanalysis::mlwe::cost`     | BKZ profiles (GSA, q-ary z-shape, simulator) and five SVP cost models   |
| `cryptanalysis::mlwe::primal`   | Primal uSVP, two independent conditions; the ML-DSA MSIS forgery side   |
| `cryptanalysis::mlwe::dual`     | Dual, MATZOV-style dual, Ducas–Pulles and Pouly–Shen regime diagnostics |
| `cryptanalysis::mlwe::hybrid`   | Guessing hybrids — and why they do not pay at these parameters          |
| `cryptanalysis::mlwe::sieve`    | Gauss, Nguyen–Vidick and bucketed sieves; progressive BKZ               |
| `cryptanalysis::ml_kem_pco`     | **Full ML-KEM key recovery** from decapsulation leakage, in 8 queries   |
| `cryptanalysis::ml_dsa_leakage` | **Full ML-DSA-65 `s1` recovery** from 4 leaky signatures, then a forgery |
| `cryptanalysis::ml_dsa_fault`   | **Full `s1` recovery** from one faulted signature; three faults         |

```bash
crypto mlwe margins                  # every set against its NIST category floor
crypto mlwe estimate --scheme ml-kem-768 --model gate-count --tours
crypto mlwe kem-pco --param 512      # recover an ML-KEM key and decapsulate with it
crypto mlwe dsa-leak --per-poly 64   # recover ML-DSA s1 and forge
crypto mlwe dsa-fault --fault all    # three faults on the rejection loop
```

Two things the estimators are arranged to prevent, both of which produced wrong
numbers before they were caught: quoting a dual cost without its
contradictory-regime diagnostic, and pairing one attack's cost model with
another's success condition. The doc explains both.

### Zero-knowledge / commitments

| Module                | Algorithm                                       |
|-----------------------|-------------------------------------------------|
| `zk::schnorr_zkp`     | Schnorr identification proof                    |
| `zk::pedersen`        | Pedersen commitments                            |
| `zk::chaum_pedersen`  | Chaum-Pedersen equality-of-discrete-logs proof  |
| `zk::merkle`          | Merkle tree commitments                         |
| `zk::polynomial`      | Polynomial arithmetic over a field              |
| `zk::kzg`             | KZG polynomial commitments                      |
| `zk::bulletproofs`    | Bulletproofs range proofs                       |
| `zk::groth16`         | Groth16 zk-SNARK skeleton: R1CS/QAP layers only; **`verify` is non-functional** (the pairing rejects honest proofs too) |
| `zk::plonk`           | PLONK polynomial IOP: **educational, unaudited, not zero-knowledge, not succinct** (polynomials sent in the clear in place of KZG commitments). `verify` checks each sent evaluation against its polynomial and the gate, copy-constraint and `Z(1) = 1` identity at `ζ` |
| `zk::stark` / `stark_v2` | STARK-style proof skeletons: **educational, unaudited, not zero-knowledge, no grinding, no stated security level; must not back a research claim**. `stark_v2` is DEEP-ALI: coset LDE, composition polynomial, an out-of-domain sample `z` (the AIR checked at `z`) and degree-bounded FRI on the DEEP quotient; its conjectured soundness (Reed–Solomon proximity conjecture, random-oracle Fiat–Shamir) is `ε ≲ β^(−q) + ε_field + ε_hash`, capped at 128 bits by SHA-256 Merkle commitments, over the BLS12-381 scalar field. `stark` is a spot-check argument, not a STARK (see the module docs) |

---

## Cryptanalysis suite

This is the part of the repo that's grown the most.  Every implemented algorithm
above has at least one attack module that *targets it*, plus a generic framework
that makes adding new attacks cheap.

### Core analytical primitives

| Module                                   | What it gives you |
|------------------------------------------|-------------------|
| `cryptanalysis::sbox`                    | DDT, LAT, BCT (Cid et al. 2018), DLCT (Bar-On et al. 2019), truncated DDT, differential / linear uniformity, nonlinearity, algebraic degree, boomerang uniformity |
| `cryptanalysis::boolean`                 | Walsh-Hadamard, ANF, algebraic degree |
| `cryptanalysis::avalanche`               | Full avalanche matrix, SAC, BIC |
| `cryptanalysis::statistical`             | Chi-squared, monobit, runs test |
| `cryptanalysis::lattice`                 | LLL, BKZ reduction |

### Symmetric-cipher attacks

| Module                                      | Attack                              |
|---------------------------------------------|-------------------------------------|
| `cryptanalysis::boomerang`                  | **Generic boomerang distinguisher + rectangle + sandwich + branch-and-bound trail search** (Wagner FSE 1999 → Cid et al. EUROCRYPT 2018) |
| `cryptanalysis::aes::reduced`               | Reduced-round AES-128 with configurable `Nr` |
| `cryptanalysis::aes::square`                | Square / integral attack (Daemen-Knudsen-Rijmen 1997) |
| `cryptanalysis::aes::differential`          | Differential cryptanalysis on 2-round AES |
| `cryptanalysis::aes::linear`                | Matsui-style linear cryptanalysis |
| `cryptanalysis::aes::impossible`            | Biham-Biryukov-Shamir impossible differential |
| `cryptanalysis::aes::mixture`               | Grassi-Rechberger-Rønjom mixture quadruples |
| `cryptanalysis::aes::boomerang`             | AES-specific boomerang + BCT |
| `cryptanalysis::aes::yoyo`                  | Rønjom-Bardeh-Helleseth yoyo distinguisher |
| `cryptanalysis::aes::mitm`                  | Demirci-Selçuk MITM on 2-round AES |
| `cryptanalysis::aes::small_scale`           | Cid-Murphy-Robshaw SR(n,2,2,4) variant |
| `cryptanalysis::aes::algebraic`             | Multivariate quadratic equations |
| `cryptanalysis::aes::milp`                  | MILP encoding of differential trail search |
| `cryptanalysis::aes::quantum_grover`        | Grover-style quantum key-recovery cost model |
| `cryptanalysis::aes::biclique`              | Biclique attack skeleton |
| `cryptanalysis::aes::related_key`           | **Related-key attack framework** — key-schedule difference propagation, related-key avalanche, related-key boomerang, Biryukov-Khovratovich local-collision demo |
| `cryptanalysis::aes::truncated_diff`        | **Truncated differential cryptanalysis** (Knudsen FSE 1994) — active-byte trail propagation, MixColumns branch-number filter, minimum-active-S-box bounds |
| `cryptanalysis::aes::higher_order`          | **Higher-order differential** (Lai 1994, Knudsen 1995) — d-th derivatives, 3-round Square integral distinguisher, algebraic-degree probe |
| `cryptanalysis::aes::dfa`                   | **DFA / Piret-Quisquater fault attack** (FDTC 2003) — single-byte fault before round 9 reduces last-round-key entropy by ~29 bits per fault |
| `cryptanalysis::aes::visualize`             | **ASCII visualization helpers** — state grids, active-byte patterns, trail diagrams, heat maps, bar charts, boomerang quartet diagrams, recovery progress bars |
| `cryptanalysis::md5_differential`           | MD5 differential collision (Wang et al. 2004) |
| `cryptanalysis::sha1_differential`          | SHA-1 differential analysis (Wang-Yin-Yu 2005) |

### Discrete-log / ECDLP attacks

| Module                                       | Attack                                                  |
|----------------------------------------------|---------------------------------------------------------|
| `cryptanalysis::pollard_rho`                 | Pollard ρ for DLP / ECDLP, multi-shard, distinguished-points |
| `cryptanalysis::pollard_collab`              | **Collaborative p2p rho**: indexed work units, self-verifying DP check-ins, CRDT merge, mailbox + TCP gossip + [cairn](https://github.com/aburan28/cairn) piecework transports — [design](docs/POLLARD_COLLAB_DESIGN.md) |
| `cryptanalysis::preprocessing_rho`           | Bernstein-Lange precomputation rho                       |
| `cryptanalysis::ml_rho_walks`                | Pollard ρ walks under learned partition functions       |
| `cryptanalysis::aut_folded_rho`              | Automorphism-folded rho (CM curves)                      |
| `cryptanalysis::ec_index_calculus`           | Semaev S₃ index calculus on prime-field curves          |
| `cryptanalysis::residual_walk`               | Partial-decomposition residual walks: rho-style collision search over factor-base decompositions, measured against plain rho |
| `cryptanalysis::gaudry_cubic`                | Gaudry-style index calculus on E(F_{p³}) with the subspace factor base {x ∈ F_p}: Weil-restricted S₃ pair test, meet-in-the-middle triple oracle, Gaudry's O(1) three-unknown S₄ solve (symmetrised Weil restriction, Macaulay matrix with row selection, eigenvalues of the multiplication matrix), sparse Wiedemann linear algebra with singleton filtering, the double-large-prime variation, rho reference |
| `cryptanalysis::ec_index_calculus_j0`        | ζ-orbit-reduced IC on j=0 curves + Eisenstein-smooth FB |
| `cryptanalysis::koblitz_index_calculus`      | Frobenius-invariant-factor-base IC on Koblitz curves (GGMP) |
| `cryptanalysis::koblitz_groebner`            | Semaev S₃ Weil restriction + matrix-F4 decomposition oracle |
| `cryptanalysis::semaev_sat`                  | CNF encoding of Semaev systems for the CDCL solver (3rd oracle) |
| `cryptanalysis::koblitz_bench`               | Scaling harness: system profiles, first fall degree, oracle cost |
| `cryptanalysis::koblitz_index_calculus`      | …also: cyclotomic classification of invariant factor bases |
| `cryptanalysis::coordinate_search`           | Algorithmic search for point coordinates that shrink the decomposition system (symmetry detection, linearising frames, interpolated summation polynomials) |
| `cryptanalysis::koblitz_symmetrised`         | The symmetrised (Artin–Schreier frame) Koblitz decomposition systems in the F4 and SAT oracles, with a paired, enumeration-gated benchmark |
| `cryptanalysis::coordinate_quotients`        | Invariants of any finite group of point maps by orbit sums: relation subgroup by experiment, minimal-degree relation, exact collapse |
| `cryptanalysis::f4_fp`                        | Degree-bounded F4 over a small prime field: normal strategy, symbolic preprocessing, parallel dense row reduction mod `p`, cooperative deadline, solving by root finding and substitution — the engine that times the coordinate-descent systems at `m = 3` |
| `cryptanalysis::coordinate_descent`          | Fixed-target quotient systems over `F_{p^k}` descended to `F_p` (Gaudry's setting) and timed on the F_p Gröbner engine; `Chart` puts frames on the `y`-line, where 3-torsion translations of `j = 0` curves are Möbius maps (`examples/three_torsion.rs`) |
| `cryptanalysis::j0_twists`                   | 6-twist enumeration on j=0 curves + smoothness flagging |
| `cryptanalysis::canonical_lift`              | Smart attack on anomalous curves (canonical lifting)    |
| `cryptanalysis::cm_canonical_lift`           | CM-curve canonical lift + p-adic logarithm              |
| `cryptanalysis::nonanom_formal_log`          | Non-anomalous formal-group logarithm                    |
| `cryptanalysis::mazur_tate_sigma`            | Mazur-Tate σ function for anomalous curves              |
| `cryptanalysis::coleman_integration`         | Coleman integration on hyperelliptic curves             |
| `cryptanalysis::ai_schoof`                   | AI-assisted Schoof's algorithm                          |
| `cryptanalysis::ecm`                         | Lenstra elliptic-curve method (ECM) factoring           |
| `cryptanalysis::modular_polynomial`          | Modular polynomials Φ_ℓ(X, Y)                            |
| `cryptanalysis::hilbert_class_poly`          | Hilbert class polynomial computation                    |
| `cryptanalysis::fght_snfs`                   | FGHT toy SNFS pipeline for trapdoored F_p* DLP          |
| `cryptanalysis::shor`                        | Shor's quantum factoring + order-finding                |
| `cryptanalysis::quantum_estimator`           | Resource estimator for quantum attacks                  |

### Signature / nonce attacks

| Module                                      | Attack                                              |
|---------------------------------------------|-----------------------------------------------------|
| `cryptanalysis::hnp_ecdsa`                  | Hidden Number Problem on ECDSA (LLL / BKZ)          |
| `cryptanalysis::multi_key_hnp`              | Multi-key HNP master-key recovery                   |
| `cryptanalysis::bleichenbacher`             | Bleichenbacher direct bias attack (Mironov / Tibouchi) |
| `cryptanalysis::ecdsa_audit`                | ECDSA transcript audit + quick-bias score           |
| `cryptanalysis::signature_corpus`           | Corpus-level reuse / weak-RNG / bias detection      |
| `cryptanalysis::orbit_homology`             | Orbit-homology bias detector                        |
| `cryptanalysis::cga_hnc`                    | Continued-fraction / HNC variant                    |
| `cryptanalysis::b_seed_profile`             | Behaviour-seed profile attack                       |

### Structural / curve-specific

| Module                                | Attack                                                  |
|---------------------------------------|---------------------------------------------------------|
| `cryptanalysis::p256_attacks`         | P-256 specific attack vectors                            |
| `cryptanalysis::p256_structural`      | P-256 structural / lattice attacks                       |
| `cryptanalysis::p256_isogeny_cover`   | Isogeny-cover speculation targeting P-256                |
| `cryptanalysis::solinas_correlations` | Solinas-prime reduction correlations                     |

### Binary-curve Weil descent (GHS / Hess scaffold)

A bounded structural scaffold for Weil descent on binary-field elliptic
curves. The only end-to-end path is the toy `m=1` case, where the descended
Jacobian is the elliptic curve itself and generic bounded search is used.
The `m=2` construction remains symbolic and lacks a smooth hyperelliptic
model; higher-genus descent, explicit isogeny lifting, and Jacobian index
calculus are not implemented.

| Module                                | Role                                                     |
|---------------------------------------|----------------------------------------------------------|
| `cryptanalysis::ghs_descent`          | Frey-Rück / GHS cover-curve construction                 |
| `cryptanalysis::ghs_full_attack`      | Bounded orchestrator producing structured `AttackReport`    |
| `cryptanalysis::ec_trapdoor`          | EC trapdoor / weak-curve detection                       |
| `cryptanalysis::ghs_screen`           | Exact all-factorisation GHS structural screen and cover towers |
| `cryptanalysis::binary_isogeny`       | Vélu-style isogeny computation in characteristic 2       |
| `binary_ecc::hyperelliptic`           | Genus-`g` hyperelliptic curve arithmetic over F_{2^m}    |
| `binary_ecc::poly_f2m`                | Polynomial arithmetic over F_{2^m}                        |
| `prime_hyperelliptic`                 | Hyperelliptic curves over prime fields (Coleman / odd-char descent) |

Toy end-to-end `m=1` ECDLP recovery runs on F_{2^6}; `m=2` type-II symbolic
descent is demonstrated on F_{2^6} with (n=3, ℓ=2), without a smooth model
or HCDLP run. See `examples/ghs_attack_demo.rs` for the bounded scenarios. The
native [`ghs_screen`](docs/GHS_SCREEN.md) CLI validates a polynomial-basis
field and reports every GHS tower without fixed-width genus truncation; that
screen is not a higher-genus solver.

### Auto-attack framework + CLI

| Module                                | Purpose                                                 |
|---------------------------------------|---------------------------------------------------------|
| `cryptanalysis::cipher_registry`      | Named-cipher catalog (`toyspn-{1..4}r`, `aes-{1..4}r`, `toyspn-present-2r`) with `BlockCipher` adapter |
| `cryptanalysis::auto_attack`          | Discovers and runs every applicable attack against a named cipher; emits Markdown report |
| `cryptanalysis::research_bench`       | Falsifiable-hypothesis bench with `linear_fit` / `log_log_fit`, polynomial and exponential scaling regimes |

---

## Cryptanalysis CLI

```bash
# Discover what's registered
crypto cryptanalysis list-ciphers

# Run every applicable attack against a named cipher
crypto cryptanalysis auto --cipher toyspn-2r
crypto cryptanalysis auto --cipher aes-2r --boomerang-pairs 4096

# Drive individual attacks
crypto cryptanalysis sbox      --cipher aes-2r
crypto cryptanalysis boomerang --cipher toyspn-3r --pairs 65536
crypto cryptanalysis boomerang --cipher toyspn-2r --alpha 0100 --delta 0100
crypto cryptanalysis rectangle --cipher toyspn-1r --pool 2048

# Run the full research bench
crypto cryptanalysis bench
cargo build --release --bin ecbench
ECBENCH=target/release/ecbench bash research/ecbench_rootfind_engine_20261012/run.sh
```
