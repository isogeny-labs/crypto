//! CLI demo for the crypto library.
//!
//! Commands:
//!   hash    <algorithm> [message|--hex HEX|--file PATH] — Compute a hash
//!   ecc     keygen                     — Generate an ECC key pair
//!   ecdsa   sign <privkey-hex> <msg>   — ECDSA sign
//!   ecdsa   verify <pubkey-hex> <msg> <r-hex> <s-hex>
//!   ecdh    <privkey-hex> <pubkey-hex> — ECDH shared secret
//!   rsa     keygen                     — Generate an RSA key pair (slow)
//!   rsa     sign <priv-json> <msg>     — RSA sign
//!   aes     encrypt <key-hex> <nonce-hex> <msg>
//!   chacha  encrypt <key-hex> <nonce-hex> <msg>
//!   hkdf    <ikm-hex> <salt-hex> <info> <length>
//!   pqc                                — PQC demos (ML-KEM, ML-DSA, SQIsign, toy Kyber)
//!   pqc-fast <module> --op ...         — the speed-oriented implementations
//!   mlwe    <attack>  ...              — ML-KEM / ML-DSA cryptanalysis

mod cli_mlwe;
mod cli_aes_observed;
mod cli_pqc;
mod cli_pqc_fast;

use clap::{Parser, Subcommand};
use crypto_lib::{
    asymmetric::rsa::{rsa_sign, rsa_verify, RsaKeyPair},
    ecc::{
        curve::CurveParams,
        ecdh::ecdh,
        ecdsa::{sign, verify, EcdsaSignature},
        field::FieldElement,
        keys::{EccKeyPair, EccPublicKey},
        point::Point,
    },
    hash::{
        blake3_hash, sha224, sha256, sha384, sha3_224, sha3_256, sha3_384, sha3_512, sha512,
        sha512_224, sha512_256, shake128, shake256,
    },
    kdf::hkdf::hkdf,
    pqc::kyber::{kyber_decapsulate, kyber_encapsulate, kyber_keygen},
    pqc::ml_dsa::{ml_dsa_65_keygen, ml_dsa_65_sign, ml_dsa_65_verify},
    pqc::ml_kem::{ml_kem_decaps, ml_kem_encaps, ml_kem_keygen, ML_KEM_768},
    pqc::sqisign::{sqisign_keygen, sqisign_sign, sqisign_verify},
    symmetric::{
        aes::{aes_gcm_decrypt, aes_gcm_encrypt, AesKey},
        chacha20::{chacha20_poly1305_decrypt, chacha20_poly1305_encrypt},
    },
    utils::encoding::{bigint_to_bytes_be, from_hex, to_hex},
};
use num_bigint::BigUint;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "crypto", about = "Educational cryptography library demo")]
struct Cli {
    /// Color output mode: `auto` (default), `always`, or `never`.
    /// `auto` enables colors only on TTY.  Respected by every visual
    /// renderer.
    #[arg(long, global = true, default_value = "auto")]
    color: String,

    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
enum Cmd {
    /// Hash a message with SHA-2, SHA-3, SHAKE, or BLAKE3.
    Hash {
        algorithm: String,
        /// UTF-8 message; use --hex or --file for arbitrary bytes.
        #[arg(conflicts_with_all = ["hex", "file"])]
        message: Option<String>,
        /// Hex-encoded message bytes.
        #[arg(long, conflicts_with = "file")]
        hex: Option<String>,
        /// Read message bytes from a file.
        #[arg(long)]
        file: Option<PathBuf>,
        /// Required output length in bytes for SHAKE128 and SHAKE256.
        #[arg(long)]
        output_bytes: Option<usize>,
    },
    /// ECC operations (keygen)
    Ecc {
        #[command(subcommand)]
        op: EccOp,
    },
    /// ECDSA sign / verify
    Ecdsa {
        #[command(subcommand)]
        op: EcdsaOp,
    },
    /// ECDH shared secret demo
    Ecdh,
    /// RSA operations
    Rsa {
        #[command(subcommand)]
        op: RsaOp,
    },
    /// AES-GCM encrypt / decrypt demo
    Aes,
    /// ChaCha20-Poly1305 encrypt / decrypt demo
    Chacha,
    /// HKDF key derivation demo
    Hkdf,
    /// Post-quantum schemes: `list` them, `run` any one of them, or the guided
    /// `demo`. Bare `crypto pqc` is the demo.
    Pqc {
        #[command(subcommand)]
        op: Option<cli_pqc::PqcOp>,
    },
    /// Run all demos
    Demo,
    /// Cryptanalysis: run cipher-attack techniques (S-box analysis,
    /// boomerang distinguisher, rectangle attack, trail search, …)
    Cryptanalysis {
        #[command(subcommand)]
        op: CryptanalysisOp,
    },
    /// **Visual demos** for every part of the library — symmetric
    /// ciphers, hash constructions, ECC.
    Visual {
        #[command(subcommand)]
        op: VisualOp,
    },
    /// **Isogeny / CM research module** — class groups, Vélu's
    /// formulas, ℓ-isogeny volcanoes, secp256k1 case study, and
    /// experimental ECDLP sweeps across an isogeny class.
    Isogeny {
        #[command(subcommand)]
        op: IsogenyOp,
    },
    /// Cryptopals Set 7 (Hashes) challenges 49–56.  Pass an integer
    /// to run one (`cryptopals 49`) or `all` for every challenge.
    Cryptopals {
        /// Challenge number 49..=56, or the literal `all`.
        challenge: String,
    },
    /// The speed-oriented PQC implementations in `pqc::fast`:
    /// ML-KEM, ML-DSA, Keccak and SQIsign-scale isogeny arithmetic.
    PqcFast {
        #[command(subcommand)]
        op: cli_pqc_fast::PqcFastOp,
    },
    /// Cryptanalysis of ML-KEM and ML-DSA: lattice-attack estimates, working
    /// sieves, and the implementation attacks that actually break deployments.
    Mlwe {
        #[command(subcommand)]
        op: cli_mlwe::MlweOp,
    },
    /// Large files (factor bases, relation tables): push to, pull from, and index
    /// the object store. See docs/store/README.md.
    Store(crypto_lib::store::cli::StoreArgs),
}

#[derive(Subcommand)]
enum IsogenyOp {
    /// Plan, enumerate, resume, verify or solve on an ordinary isogeny class.
    /// Example: crypto isogeny class build --curve curve.json --out graph.sqlite --steps all
    #[command(disable_help_flag = true)]
    Class {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Map the ℓ-volcano of a small curve.
    /// `<curve>` is a built-in name: `toy-a`, `toy-b`, `toy-j0`.
    Volcano {
        #[arg(long, default_value = "toy-b")]
        curve: String,
        #[arg(long, default_value_t = 2)]
        ell: u64,
        #[arg(long, default_value_t = 4)]
        depth: usize,
    },
    /// Build and display the S-isogeny graph for a small curve.
    /// `<ell-list>` is comma-separated, e.g. `2,3,5`.
    Graph {
        #[arg(long, default_value = "toy-b")]
        curve: String,
        #[arg(long, default_value = "2,3")]
        ell_list: String,
        #[arg(long, default_value_t = 64)]
        max_nodes: usize,
    },
    /// Compute class-group data for a negative discriminant `D`.
    Cm {
        /// Discriminant (must be negative, e.g. `-23`).
        #[arg(long, allow_hyphen_values = true)]
        discriminant: i64,
    },
    /// Run the experimental harness: random curves, isogeny
    /// sweep, attack suite, JSON report.
    Experiment {
        #[arg(long, default_value_t = 10)]
        bits: u32,
        #[arg(long, default_value_t = 2)]
        trials: u32,
        #[arg(long, default_value = "2,3")]
        ell_list: String,
        /// Override the Pollard-rho iteration cap.  Defaults to the
        /// `8·2^{bits/2}` heuristic below.  Frozen artifacts under
        /// `experiments/` were produced with explicit caps (2^14, 2^18),
        /// so reproducing them needs this.
        #[arg(long)]
        rho_cap: Option<u64>,
    },
    /// secp256k1 case study: GLV constants, twist analysis, MOV
    /// embedding-degree certificate, small-degree-isogeny survey.
    Secp256k1,
}

#[derive(Subcommand)]
enum VisualOp {
    /// Symmetric ciphers: ECB-penguin, mode dataflows, AES round
    /// trace, ChaCha state, avalanche heat-map.
    Symmetric {
        /// Sub-target: `ecb-penguin`, `modes`, `aes-trace`, `chacha`,
        /// `avalanche`, `all`.
        #[arg(long, default_value = "all")]
        target: String,
    },
    /// Hash functions: Merkle-Damgård / sponge / BLAKE3 tree diagrams,
    /// SHA-256 round trace, hash avalanche.
    Hash {
        #[arg(long, default_value = "all")]
        target: String,
    },
    /// Elliptic curves: toy-curve scatter, point operations,
    /// scalar-multiplication ladder, ECDSA dataflow.
    Ecc {
        #[arg(long, default_value = "all")]
        target: String,
    },
    /// Everything at once — emits the full visual bundle.
    All,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
enum CryptanalysisOp {
    /// List all registered ciphers known to the auto-attack runner.
    ListCiphers,
    /// Run every applicable attack against a registered cipher and
    /// emit a Markdown report.
    ///
    /// Example: `cargo run -- cryptanalysis auto --cipher toyspn-2r`
    Auto {
        /// Cipher identifier (use `list-ciphers` to see options).
        #[arg(long)]
        cipher: String,
        /// Number of plaintext pairs for the boomerang.
        #[arg(long, default_value_t = 65_536)]
        boomerang_pairs: usize,
        /// Pool size for the rectangle attack.
        #[arg(long, default_value_t = 2048)]
        rectangle_pool: usize,
        /// Negative log₂ of the differential-trail probability
        /// threshold.  `12` here means threshold = `2⁻¹²`.
        #[arg(long, default_value_t = 12)]
        trail_threshold_neg_log2: u32,
        /// Max number of trails to render.
        #[arg(long, default_value_t = 5)]
        trail_top_k: usize,
    },
    /// Run just the boomerang distinguisher against a registered cipher.
    Boomerang {
        #[arg(long)]
        cipher: String,
        /// α and δ in hex (low byte first), e.g. `0100`.  Defaults to
        /// the cipher's canonical pair.
        #[arg(long)]
        alpha: Option<String>,
        #[arg(long)]
        delta: Option<String>,
        #[arg(long, default_value_t = 65_536)]
        pairs: usize,
    },
    /// Run just the rectangle attack against a registered cipher.
    Rectangle {
        #[arg(long)]
        cipher: String,
        #[arg(long)]
        alpha: Option<String>,
        #[arg(long)]
        delta: Option<String>,
        #[arg(long, default_value_t = 2048)]
        pool: usize,
    },
    /// Print the S-box differential / linear / boomerang report for a
    /// registered cipher.
    Sbox {
        #[arg(long)]
        cipher: String,
    },
    /// Run a bounded AES-128 differential, linear, or integral recovery.
    AesAttack {
        /// `differential` (2 rounds), `linear` (1 round), or `square` (4 rounds).
        #[arg(long)]
        technique: String,
        /// 16-byte analysis key as 32 hex digits.
        #[arg(long, default_value = "914f6da1c78305bb2957ee104d826cfa")]
        key_hex: String,
        /// Chosen pairs for differential, known pairs for linear.
        #[arg(long, default_value_t = 24)]
        samples: usize,
        #[arg(long, default_value_t = 0xdead_beef_cafe)]
        seed: u64,
        /// Input difference at plaintext byte zero (differential only).
        #[arg(long, default_value_t = 0x73)]
        alpha: u8,
    },
    /// Measure a bounded AES impossible, yoyo, mixture, or S-box zero-correlation distinguisher.
    AesDistinguisher {
        /// `impossible` (4 rounds), `yoyo` (2 vs 3), `mixture` (1 vs 2), or `zero-correlation` (S-box).
        #[arg(long)]
        technique: String,
        /// 16-byte analysis key as 32 hex digits (unused for S-box statistics).
        #[arg(long, default_value = "914f6da1c78305bb2957ee104d826cfa")]
        key_hex: String,
        /// Chosen pairs, yoyo trials, or mixture quadruples.
        #[arg(long, default_value_t = 4096)]
        samples: usize,
        #[arg(long, default_value_t = 0xdead_beef_cafe)]
        seed: u64,
        /// Input difference at plaintext byte zero for `impossible`.
        #[arg(long, default_value_t = 0x73)]
        alpha: u8,
    },
    /// Recover an AES-128 key candidate from measured first-round S-box power traces.
    AesCpa {
        /// JSON with plaintexts and sixteen aligned trace arrays.
        #[arg(long)]
        input: PathBuf,
        /// Ciphertext for the first plaintext, to verify the recovered key.
        #[arg(long)]
        known_ciphertext_hex: Option<String>,
    },
    /// Estimate AES-128 key-byte high nibbles from sixteen isolated lookup timing channels.
    AesCacheTiming {
        /// JSON with plaintexts and one aligned timing channel per byte.
        #[arg(long)]
        input: PathBuf,
        /// Minimum observations for each plaintext high-nibble bucket.
        #[arg(long, default_value_t = 4)]
        min_samples_per_bucket: usize,
    },
    /// Analyze caller-supplied chosen pairs for the two-round AES differential model.
    AesDifferentialObserved {
        /// JSON with alpha and chosen plaintext/ciphertext pairs.
        #[arg(long)]
        input: PathBuf,
    },
    /// Analyze caller-supplied known pairs for the one-round AES linear model.
    AesLinearObserved {
        /// JSON with known pairs and optional independent verification pairs.
        #[arg(long)]
        input: PathBuf,
    },
    /// Reverse an established AES-128 10th-round key to the master key.
    AesDfaSchedule {
        /// Last-round AES-128 key as 32 hex digits, in column-major byte order.
        #[arg(long)]
        last_round_key_hex: String,
    },
    /// Recover AES-128 from observed single-byte start-of-round-9 fault pairs.
    AesDfa {
        /// JSON file or checked store reference containing plaintext/ciphertext fault pairs.
        #[arg(long)]
        input: PathBuf,
    },
    /// Run the full research bench and emit the Markdown report.
    /// (Equivalent to `cargo test --lib --release bench_demo --
    /// --ignored --nocapture` but accessible from the CLI.)
    Bench,
    /// Run applicable length-extension, truncated birthday, and
    /// differential-bias checks for md4, md5, sha1, sha256, or sha3-256.
    HashAuto {
        /// Hash identifier: md4, md5, sha1, sha256, or sha3-256.
        #[arg(long)]
        hash: String,
    },
    /// Length-extension attack demo against a Merkle-Damgård hash.
    LengthExtension {
        /// Hash identifier: md4, md5, sha1, or sha256.
        #[arg(long)]
        hash: String,
        /// Length (in bytes) of the unknown secret prefix.
        #[arg(long)]
        secret_len: usize,
        /// Original message body (hex-encoded).
        #[arg(long)]
        message_hex: String,
        /// Suffix to append (hex-encoded).
        #[arg(long)]
        suffix_hex: String,
        /// Digest of `secret || message_hex` (hex-encoded).
        #[arg(long)]
        digest_hex: String,
    },
    /// **AES related-key attack** — propagate a key-difference
    /// through the schedule + run avalanche + run a 4-round local
    /// collision demo.  Emits a Markdown report.
    AesRelatedKey {
        /// Key size in bits: 128 or 256.
        #[arg(long, default_value_t = 128)]
        key_bits: u32,
        /// Key-difference ΔK in hex (length = key_bits / 8 bytes).
        /// Defaults to a single-active-byte difference at position 0.
        #[arg(long)]
        delta_k_hex: Option<String>,
        /// Number of avalanche trials.
        #[arg(long, default_value_t = 1024)]
        avalanche_trials: usize,
        /// Number of rounds to study (default = full).  Sets the
        /// avalanche and local-collision round count.
        #[arg(long, default_value_t = 0)]
        rounds: usize,
    },
    /// **AES visual demos** — emit ASCII diagrams for the major
    /// AES attack visualizations: truncated-diff trail, 3-round
    /// integral distinguisher, DFA single-byte-fault propagation,
    /// related-key schedule diffusion.
    AesVisualDemo {
        /// Which demo to run: `truncated-diff`, `integral`, `dfa`,
        /// `key-schedule`, or `all` (default).
        #[arg(long, default_value = "all")]
        demo: String,
    },
    /// **Visual demos for every non-AES attack** — DDT/LAT heat-maps
    /// on S-boxes, Walsh-Hadamard spectra, Pollard ρ trajectories,
    /// HNP recovery curves, Bleichenbacher bias histograms,
    /// length-extension diagrams, Joux multicollision trees, j=0
    /// twist factorisations, birthday-paradox curves.
    VisualAll {
        /// Optional single-demo target: `sbox-ddt`, `sbox-lat`,
        /// `walsh`, `pollard-rho`, `hnp`, `bleichenbacher`,
        /// `length-extension`, `joux`, `j0-twists`, `birthday`,
        /// or `all` (default).
        #[arg(long, default_value = "all")]
        target: String,
    },
    /// **Collaborative (peer-to-peer) Pollard rho** — divide one
    /// ECDLP among many machines with self-verifying distinguished-
    /// point check-ins.  See `docs/POLLARD_COLLAB_DESIGN.md`.
    RhoCollab {
        #[command(subcommand)]
        op: RhoCollabOp,
    },
}

#[derive(Subcommand)]
enum RhoCollabOp {
    /// Write a job document that every participant shares.
    Init {
        /// Built-in curve: `demo-small`, `demo-mid`, `demo-32`,
        /// `demo-40`, `secp256k1`.
        #[arg(long, default_value = "demo-32")]
        curve: String,
        /// Plant a secret (hex): the target becomes `secret·P`.
        #[arg(long)]
        secret: Option<String>,
        /// Target point as `x:y` (hex).  Required unless `--secret`.
        #[arg(long)]
        target: Option<String>,
        /// Job label (part of the job id).
        #[arg(long, default_value = "collab")]
        name: String,
        /// Distinguished-point bits; default ≈ ¼·log₂ n.
        #[arg(long)]
        dp_bits: Option<u8>,
        /// Number of r-adding branches.
        #[arg(long, default_value_t = 32)]
        branches: u32,
        /// Use the negation map (x-only DP keys, ≈√2 faster).
        #[arg(long)]
        negation: bool,
        /// Walkers per work unit.
        #[arg(long, default_value_t = 256)]
        unit_size: u64,
        /// Seed mixed into every derivation (re-run with a new one).
        #[arg(long, default_value_t = 0)]
        seed: u64,
        /// Where to write the job document.
        #[arg(long, default_value = "job.json")]
        out: std::path::PathBuf,
        /// Also write it as `<mailbox>/job.json`.
        #[arg(long)]
        mailbox: Option<std::path::PathBuf>,
    },
    /// Walk units, check in, and exchange check-ins with peers.
    Work {
        /// Job document (default: `<mailbox>/job.json`).
        #[arg(long)]
        job: Option<std::path::PathBuf>,
        /// This node's name; lanes are `<node>.<n>`.
        #[arg(long, default_value = "node")]
        node: String,
        /// Worker lanes (threads).
        #[arg(long, default_value_t = 1)]
        threads: usize,
        /// Shared directory transport.
        #[arg(long)]
        mailbox: Option<std::path::PathBuf>,
        /// TCP address to listen on, e.g. `0.0.0.0:7000`.
        #[arg(long)]
        listen: Option<String>,
        /// TCP peers to gossip with (repeatable).
        #[arg(long = "peer")]
        peers: Vec<String>,
        /// Walkers per check-in.
        #[arg(long, default_value_t = 64)]
        checkin_every: u64,
        /// Seconds a silent claim stays live before others take it over.
        #[arg(long, default_value_t = 120)]
        lease_secs: u64,
        /// Seconds between mailbox/peer syncs and status lines.
        #[arg(long, default_value_t = 5)]
        sync_secs: u64,
        /// Stop after this many seconds (0 = until solved).
        #[arg(long, default_value_t = 0)]
        max_seconds: u64,
        /// Stop each lane after this many walkers (0 = until solved).
        #[arg(long, default_value_t = 0)]
        max_walkers: u64,
        /// A cairn node (`http://host:port`, running `cairn serve
        /// --queue`).  Every distinguished point is committed and revealed
        /// as a claim on `--objective`, and the objective's log is merged
        /// back as the shared DP table.
        #[arg(long, requires = "objective")]
        cairn: Option<String>,
        /// The cairn piecework objective id the points are claims on.
        #[arg(long)]
        objective: Option<String>,
        /// Nickname to submit under (default: the node name).  Ignored
        /// when `--identity` is given.
        #[arg(long)]
        submitter: Option<String>,
        /// A cairn identity file (`cairn identity --out FILE`): submit
        /// under its Ed25519 key and sign every record.
        #[arg(long)]
        identity: Option<std::path::PathBuf>,
        /// The cairn objective that pays for `k` itself.  Once the search
        /// solves, `{"k": …}` is committed there and revealed next epoch.
        #[arg(long)]
        answer_objective: Option<String>,
        /// The cairn node's epoch length in seconds (its
        /// `CAIRN_EPOCH_SECONDS`; 600 unless the operator changed it).
        #[arg(long, default_value_t = 600)]
        cairn_epoch_secs: u64,
        /// Where pending commitments are kept between runs (default:
        /// `<node>.cairn.json` next to the job).
        #[arg(long)]
        cairn_state: Option<std::path::PathBuf>,
    },
    /// Show merged progress from a mailbox, peers, and/or a cairn log.
    Status {
        #[arg(long)]
        job: Option<std::path::PathBuf>,
        #[arg(long)]
        mailbox: Option<std::path::PathBuf>,
        #[arg(long = "peer")]
        peers: Vec<String>,
        /// Read the objective's accepted points from this cairn node.
        #[arg(long, requires = "objective")]
        cairn: Option<String>,
        #[arg(long)]
        objective: Option<String>,
        /// Emit JSON instead of text.
        #[arg(long)]
        json: bool,
        #[arg(long, default_value_t = 120)]
        lease_secs: u64,
    },
}

#[derive(Subcommand)]
enum EccOp {
    Keygen,
}

#[derive(Subcommand)]
enum EcdsaOp {
    Sign {
        message: String,
    },
    Verify {
        message: String,
        pubkey_hex: String,
        r_hex: String,
        s_hex: String,
    },
}

#[derive(Subcommand)]
enum RsaOp {
    /// Generate a 1024-bit RSA key pair (faster for demo purposes)
    Keygen,
    /// Sign + verify demo
    Demo,
}

fn main() {
    let cli = Cli::parse();
    // Honour the --color flag before any visual code runs.
    match cli.color.as_str() {
        "always" | "1" | "yes" | "on" => crypto_lib::visualize::color::set_enabled(true),
        "never" | "0" | "no" | "off" => crypto_lib::visualize::color::set_enabled(false),
        _ => {} // "auto": leave to env-var + TTY detection
    }

    match cli.command {
        Cmd::Hash {
            algorithm,
            message,
            hex,
            file,
            output_bytes,
        } => match load_hash_input(message.as_deref(), hex.as_deref(), file.as_deref())
            .and_then(|bytes| cmd_hash(&algorithm, &bytes, output_bytes))
        {
            Ok(digest) => println!("{digest}"),
            Err(error) => {
                eprintln!("hash: {error}");
                std::process::exit(2);
            }
        },
        Cmd::Ecc { op: EccOp::Keygen } => cmd_ecc_keygen(),
        Cmd::Ecdsa { op } => cmd_ecdsa(op),
        Cmd::Ecdh => cmd_ecdh(),
        Cmd::Rsa { op } => cmd_rsa(op),
        Cmd::Aes => cmd_aes(),
        Cmd::Chacha => cmd_chacha(),
        Cmd::Hkdf => cmd_hkdf(),
        Cmd::Pqc { op } => cli_pqc::run(op),
        Cmd::Demo => cmd_demo(),
        Cmd::Cryptanalysis { op } => cmd_cryptanalysis(op),
        Cmd::Visual { op } => cmd_visual(op),
        Cmd::Isogeny { op } => cmd_isogeny(op),
        Cmd::Cryptopals { challenge } => cmd_cryptopals(&challenge),
        Cmd::PqcFast { op } => cli_pqc_fast::run(op),
        Cmd::Mlwe { op } => cli_mlwe::run(op),
        Cmd::Store(args) => {
            if let Err(e) = crypto_lib::store::cli::run(&args) {
                eprintln!("store: {e}");
                std::process::exit(1);
            }
        }
    }
}

fn cmd_cryptopals(arg: &str) {
    use crypto_lib::cryptopals;
    let reports = if arg == "all" {
        cryptopals::run_all()
    } else if arg == "set7" {
        (49..=56).map(|n| cryptopals::run(n).unwrap()).collect()
    } else if arg == "set8" {
        (57..=66).map(|n| cryptopals::run(n).unwrap()).collect()
    } else if arg == "set9" {
        (67..=74).map(|n| cryptopals::run(n).unwrap()).collect()
    } else {
        match arg.parse::<u32>() {
            Ok(n) => match cryptopals::run(n) {
                Some(r) => vec![r],
                None => {
                    eprintln!("Unknown challenge {n} — valid range is 1..=74.");
                    std::process::exit(2);
                }
            },
            Err(_) => {
                eprintln!("Pass a challenge number (1..=74), or `set7`/`set8`/`set9`/`all`.");
                std::process::exit(2);
            }
        }
    };
    let mut any_fail = false;
    for r in &reports {
        println!(
            "── Cryptopals #{} — {} {}──",
            r.challenge,
            r.title,
            if r.success { "[OK] " } else { "[FAIL] " }
        );
        println!("{}", r.transcript);
        if !r.success {
            any_fail = true;
        }
    }
    if any_fail {
        std::process::exit(1);
    }
}

// ── Collaborative rho ────────────────────────────────────────────────────────

fn cmd_rho_collab(op: RhoCollabOp) {
    use crypto_lib::cryptanalysis::pollard_collab::cairn::{
        wall_clock, CairnConfig, CairnTransport, Submitter,
    };
    use crypto_lib::cryptanalysis::pollard_collab::{
        demo_curve, run_lane, sync_with_peer, JobSpec, LaneOptions, Mailbox, PeerServer,
        SharedState, DEMO_CURVES,
    };
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    fn die(msg: impl std::fmt::Display) -> ! {
        eprintln!("error: {msg}");
        std::process::exit(1);
    }
    fn parse_hex(s: &str) -> BigUint {
        BigUint::parse_bytes(s.trim().trim_start_matches("0x").as_bytes(), 16)
            .unwrap_or_else(|| die(format!("bad hex `{s}`")))
    }
    fn load_spec(
        job: &Option<std::path::PathBuf>,
        mailbox: &Option<std::path::PathBuf>,
    ) -> JobSpec {
        match (job, mailbox) {
            (Some(p), _) => {
                let s = std::fs::read_to_string(p)
                    .unwrap_or_else(|e| die(format!("{}: {e}", p.display())));
                JobSpec::from_json(&s).unwrap_or_else(|e| die(e))
            }
            (None, Some(d)) => Mailbox::read_job(d).unwrap_or_else(|e| die(e)),
            (None, None) => die("pass --job <file> or --mailbox <dir>"),
        }
    }

    /// Open the cairn transport the flags describe, if `--cairn` was given.
    #[allow(clippy::too_many_arguments)]
    fn open_cairn(
        url: &Option<String>,
        objective: &Option<String>,
        submitter: &Option<String>,
        identity: &Option<std::path::PathBuf>,
        answer_objective: &Option<String>,
        epoch_secs: u64,
        state_path: Option<std::path::PathBuf>,
        default_name: &str,
    ) -> Option<CairnTransport> {
        let url = url.as_ref()?;
        let objective_id = objective
            .clone()
            .unwrap_or_else(|| die("--cairn needs --objective <id>"));
        let who = match identity {
            Some(path) => Submitter::from_identity_file(path).unwrap_or_else(|e| die(e)),
            None => Submitter::Nickname(
                submitter
                    .clone()
                    .unwrap_or_else(|| default_name.to_string()),
            ),
        };
        let transport = CairnTransport::open(CairnConfig {
            url: url.trim_end_matches('/').to_string(),
            objective_id: objective_id.clone(),
            submitter: who,
            answer_objective: answer_objective.clone(),
            epoch_secs,
            state_path,
            clock: wall_clock(),
        })
        .unwrap_or_else(|e| die(e));
        let short = if objective_id.len() > 23 {
            format!("{}…", &objective_id[..23])
        } else {
            objective_id
        };
        eprintln!(
            "[cairn] {url} · objective {short} · submitting as {}{}",
            transport.submitter(),
            if transport.pending() > 0 {
                format!(
                    " · {} commitment(s) pending from a previous run",
                    transport.pending()
                )
            } else {
                String::new()
            }
        );
        Some(transport)
    }

    match op {
        RhoCollabOp::Init {
            curve,
            secret,
            target,
            name,
            dp_bits,
            branches,
            negation,
            unit_size,
            seed,
            out,
            mailbox,
        } => {
            let params = demo_curve(&curve).unwrap_or_else(|| {
                die(format!(
                    "unknown curve `{curve}`; try {}",
                    DEMO_CURVES.join(", ")
                ))
            });
            let q = match (&secret, &target) {
                (Some(s), _) => {
                    let x = parse_hex(s) % &params.n;
                    params.generator().scalar_mul(&x, &params.a_fe())
                }
                (None, Some(t)) => {
                    let (xs, ys) = t
                        .split_once(':')
                        .unwrap_or_else(|| die("--target must be `x:y` hex"));
                    Point::Affine {
                        x: params.fe(parse_hex(xs)),
                        y: params.fe(parse_hex(ys)),
                    }
                }
                (None, None) => die("pass --secret <hex> (planted demo) or --target x:y"),
            };
            let mut spec = JobSpec::new(&params, &q, &name, seed).unwrap_or_else(|e| die(e));
            if let Some(b) = dp_bits {
                spec.dp_bits = b;
            }
            spec.num_branches = branches;
            spec.negation_map = negation;
            spec.unit_size = unit_size;
            let ctx = spec.build().unwrap_or_else(|e| die(e));
            std::fs::write(&out, spec.to_json())
                .unwrap_or_else(|e| die(format!("{}: {e}", out.display())));
            if let Some(d) = &mailbox {
                let mb = Mailbox::open(d).unwrap_or_else(|e| die(e));
                mb.write_job(&spec).unwrap_or_else(|e| die(e));
            }
            println!("job id:          {}", ctx.job_id);
            println!(
                "curve:           {} ({} bits)",
                params.name,
                params.n.bits()
            );
            println!(
                "dp_bits:         {}  (mean trail 2^{})",
                spec.dp_bits, spec.dp_bits
            );
            println!("unit size:       {} walkers", spec.unit_size);
            println!("expected steps:  {:.3e}", ctx.expected_steps());
            println!("expected DPs:    {:.3e}", ctx.expected_dps());
            println!("written:         {}", out.display());
            if let Some(d) = mailbox {
                println!("mailbox:         {}", d.join("job.json").display());
            }
        }

        RhoCollabOp::Work {
            job,
            node,
            threads,
            mailbox,
            listen,
            peers,
            checkin_every,
            lease_secs,
            sync_secs,
            max_seconds,
            max_walkers,
            cairn,
            objective,
            submitter,
            identity,
            answer_objective,
            cairn_epoch_secs,
            cairn_state,
        } => {
            let spec = load_spec(&job, &mailbox);
            let ctx = Arc::new(spec.build().unwrap_or_else(|e| die(e)));
            let state = Arc::new(Mutex::new(SharedState::new(&ctx)));
            let cairn_state = cairn_state.or_else(|| {
                cairn.as_ref().map(|_| {
                    let dir = job
                        .as_ref()
                        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
                        .or_else(|| mailbox.clone())
                        .unwrap_or_else(|| std::path::PathBuf::from("."));
                    dir.join(format!("{node}.cairn.json"))
                })
            });
            let cairn = open_cairn(
                &cairn,
                &objective,
                &submitter,
                &identity,
                &answer_objective,
                cairn_epoch_secs,
                cairn_state,
                &node,
            )
            .map(|t| Arc::new(Mutex::new(t)));
            if let Some(c) = &cairn {
                let mut st = state.lock().unwrap();
                match c.lock().unwrap().sync(&ctx, &mut st) {
                    Ok(r) => eprintln!(
                        "[cairn] log read: {} points merged, {} rejected, {} revealed",
                        r.accepted_dps, r.rejected_dps, r.revealed
                    ),
                    Err(e) => eprintln!("[cairn] first sync failed: {e}"),
                }
            }
            let mbox = mailbox.as_ref().map(|d| {
                let mut mb = Mailbox::open(d).unwrap_or_else(|e| die(e));
                let (n, _) = mb
                    .sync(&ctx, &mut state.lock().unwrap())
                    .unwrap_or_else(|e| die(e));
                eprintln!("[collab] mailbox {}: merged {n} check-ins", d.display());
                Arc::new(Mutex::new(mb))
            });
            let _server = listen.as_ref().map(|addr| {
                let s = PeerServer::start(addr, Arc::clone(&ctx), Arc::clone(&state))
                    .unwrap_or_else(|e| die(format!("listen {addr}: {e}")));
                eprintln!("[collab] listening on {}", s.local_addr());
                s
            });
            let do_sync = |verbose: bool| {
                for p in &peers {
                    match sync_with_peer(p.as_str(), &ctx, &state) {
                        Ok(r) => {
                            if verbose && (r.received > 0 || r.sent > 0) {
                                eprintln!(
                                    "[collab] {p}: received {} sent {} rejected {}/{}",
                                    r.received, r.sent, r.rejected_here, r.rejected_there
                                );
                            }
                        }
                        Err(e) => {
                            if verbose {
                                eprintln!("[collab] {p}: sync failed: {e}");
                            }
                        }
                    }
                }
                if let Some(mb) = &mbox {
                    let mut mb = mb.lock().unwrap();
                    let mut st = state.lock().unwrap();
                    if let Ok((n, rej)) = mb.sync(&ctx, &mut st) {
                        if verbose && (n > 0 || rej > 0) {
                            eprintln!("[collab] mailbox: +{n} check-ins, {rej} DPs rejected");
                        }
                    }
                    // Bridge: anything learned over TCP goes into the
                    // directory too.
                    if let Err(e) = mb.publish_missing(&st) {
                        eprintln!("[collab] mailbox relay failed: {e}");
                    }
                }
                if let Some(c) = &cairn {
                    let mut st = state.lock().unwrap();
                    let mut c = c.lock().unwrap();
                    match c.sync(&ctx, &mut st) {
                        Ok(r) => {
                            if verbose && (r.accepted_dps > 0 || r.revealed > 0 || r.refused > 0) {
                                eprintln!(
                                    "[cairn] +{} points from the log ({} rejected), {} revealed, {} refused",
                                    r.accepted_dps, r.rejected_dps, r.revealed, r.refused
                                );
                            }
                        }
                        Err(e) => {
                            if verbose {
                                eprintln!("[cairn] sync failed: {e}");
                            }
                        }
                    }
                }
            };
            do_sync(true);

            eprintln!(
                "[collab] job {} · node {node} · {threads} lane(s) · expected {:.3e} steps",
                &ctx.job_id[..16],
                ctx.expected_steps()
            );
            let stop = Arc::new(AtomicBool::new(false));
            let start = Instant::now();
            let handles: Vec<_> = (0..threads.max(1))
                .map(|lane| {
                    let ctx = Arc::clone(&ctx);
                    let state = Arc::clone(&state);
                    let stop = Arc::clone(&stop);
                    let mbox = mbox.clone();
                    let cairn = cairn.clone();
                    let opts = LaneOptions {
                        checkin_every,
                        lease_secs,
                        max_walkers,
                        ..LaneOptions::new(&format!("{node}.{lane}"))
                    };
                    std::thread::spawn(move || {
                        run_lane(
                            &ctx,
                            &state,
                            &opts,
                            &mut |ci| {
                                if let Some(mb) = &mbox {
                                    if let Err(e) = mb.lock().unwrap().publish(ci) {
                                        eprintln!("[collab] publish failed: {e}");
                                    }
                                }
                                if let Some(c) = &cairn {
                                    match c.lock().unwrap().publish(&ctx, ci) {
                                        Ok(r) if r.committed > 0 => eprintln!(
                                            "[cairn] committed {} point(s); reveal follows next epoch",
                                            r.committed
                                        ),
                                        Ok(_) => {}
                                        Err(e) => eprintln!("[cairn] commit failed: {e}"),
                                    }
                                }
                            },
                            &|| stop.load(Ordering::Relaxed),
                        )
                    })
                })
                .collect();

            let tick = Duration::from_secs(sync_secs.max(1));
            let mut last_sync = Instant::now();
            loop {
                std::thread::sleep(Duration::from_millis(250));
                let lanes_done = handles.iter().all(|h| h.is_finished());
                let solved = state.lock().unwrap().solution.is_some();
                let timed_out = max_seconds > 0 && start.elapsed().as_secs() >= max_seconds;
                let due = last_sync.elapsed() >= tick;
                if due || solved || lanes_done || timed_out {
                    last_sync = Instant::now();
                    do_sync(true);
                    // Rate-limited inside: asked every tick, posts every
                    // HEARTBEAT_SECS. A failure is one line and the next
                    // tick tries again; a node without the route is told
                    // once and then left alone.
                    if let Some(c) = &cairn {
                        if let Err(e) = c.lock().unwrap().heartbeat() {
                            eprintln!("[cairn] heartbeat failed: {e}");
                        }
                    }
                    let st = state.lock().unwrap();
                    let p = st.progress(
                        &ctx,
                        crypto_lib::cryptanalysis::pollard_collab::state::now_secs(),
                        lease_secs,
                    );
                    eprintln!(
                        "[collab] {:>6.1}s  steps {:>10}  ({:>5.1}% of expected)  DPs {:>7}  units done {} active {}  peers {}  rejected {}",
                        start.elapsed().as_secs_f64(),
                        p.steps,
                        100.0 * p.fraction,
                        p.dps_stored,
                        p.units_completed,
                        p.units_active,
                        p.peers,
                        p.rejected_dps,
                    );
                    if let Some(c) = &cairn {
                        let c = c.lock().unwrap();
                        let s = c.stats();
                        eprintln!(
                            "[cairn]  committed {}  revealed {}  pending {}  refused {}  paid {} unit(s) = {}  rejected {}  log points {}  heartbeats {}",
                            s.committed, s.revealed, c.pending(), s.refused, s.paid_units, s.paid_total, s.rejected, s.log_dps, s.heartbeats,
                        );
                    }
                }
                if solved || lanes_done || timed_out {
                    break;
                }
            }
            stop.store(true, Ordering::Relaxed);
            for h in handles {
                let _ = h.join();
            }
            // Final exchange so peers learn the outcome.
            do_sync(false);
            let st = state.lock().unwrap();
            if let Some(c) = &cairn {
                let c = c.lock().unwrap();
                if c.pending() > 0 {
                    eprintln!(
                        "[cairn] {} commitment(s) still wait for the epoch to turn; run `work` again \
                         (same node name and state file) to reveal them",
                        c.pending()
                    );
                }
            }
            match &st.solution {
                Some(x) => {
                    println!("solution: {}", x.to_str_radix(16));
                    println!("verified: {}", ctx.g.scalar_mul(x, &ctx.a) == ctx.q);
                }
                None => {
                    println!(
                        "solution: not found (stopped after {:.1}s)",
                        start.elapsed().as_secs_f64()
                    );
                    std::process::exit(2);
                }
            }
        }

        RhoCollabOp::Status {
            job,
            mailbox,
            peers,
            cairn,
            objective,
            json,
            lease_secs,
        } => {
            let spec = load_spec(&job, &mailbox);
            let ctx = spec.build().unwrap_or_else(|e| die(e));
            let state = Mutex::new(SharedState::new(&ctx));
            if let Some(d) = &mailbox {
                let mut mb = Mailbox::open(d).unwrap_or_else(|e| die(e));
                mb.sync(&ctx, &mut state.lock().unwrap())
                    .unwrap_or_else(|e| die(e));
            }
            if let Some(mut c) =
                open_cairn(&cairn, &objective, &None, &None, &None, 600, None, "status")
            {
                match c.sync(&ctx, &mut state.lock().unwrap()) {
                    Ok(r) => eprintln!(
                        "[cairn] {} accepted point(s) in the log, {} rejected here",
                        r.accepted_dps, r.rejected_dps
                    ),
                    Err(e) => eprintln!("[cairn] {e}"),
                }
            }
            for p in &peers {
                if let Err(e) = sync_with_peer(p.as_str(), &ctx, &state) {
                    eprintln!("[collab] {p}: {e}");
                }
            }
            let st = state.lock().unwrap();
            let now = crypto_lib::cryptanalysis::pollard_collab::state::now_secs();
            let p = st.progress(&ctx, now, lease_secs);
            if json {
                println!("{}", serde_json::to_string_pretty(&p).unwrap());
                return;
            }
            println!("job id:        {}", ctx.job_id);
            println!("peers seen:    {}   check-ins: {}", p.peers, p.checkins);
            println!(
                "steps:         {}   ({:.1}% of expected {:.3e})",
                p.steps,
                100.0 * p.fraction,
                p.expected_steps
            );
            println!(
                "DPs stored:    {}   (expected ≈ {:.3e}; {} rejected)",
                p.dps_stored, p.expected_dps, p.rejected_dps
            );
            println!(
                "units:         {} completed, {} active",
                p.units_completed, p.units_active
            );
            match &p.solution {
                Some(x) => println!("solution:      {x}"),
                None => println!("solution:      not yet"),
            }
            let units = st.units(now, lease_secs);
            if !units.is_empty() {
                println!();
                println!(
                    "{:>8} {:>10} {:>10} {:>6} {:>5}  owner",
                    "unit", "walkers", "steps", "dps", "dead"
                );
                for u in units.iter().take(40) {
                    println!(
                        "{:>8} {:>10} {:>10} {:>6} {:>5}  {}",
                        u.unit,
                        format!("{}/{}", u.walkers_done, ctx.spec.unit_size),
                        u.steps,
                        u.dps,
                        u.dead_trails,
                        if u.completed {
                            "done".to_string()
                        } else {
                            u.owner.clone().unwrap_or_else(|| "(lease expired)".into())
                        }
                    );
                }
                if units.len() > 40 {
                    println!("… {} more units", units.len() - 40);
                }
            }
        }
    }
}

// ── Isogeny subcommands ───────────────────────────────────────────────────────

fn parse_isogeny_curve(name: &str) -> crypto_lib::isogeny::SmallCurve {
    use crypto_lib::isogeny::{toy_curve_a, toy_curve_b, toy_curve_j0};
    match name {
        "toy-a" | "toy-2003" => toy_curve_a(),
        "toy-b" | "toy-101" => toy_curve_b(),
        "toy-j0" | "toy-j0-103" => toy_curve_j0(),
        _ => {
            eprintln!("unknown curve `{}`; try one of: toy-a, toy-b, toy-j0", name);
            std::process::exit(1);
        }
    }
}

fn parse_ell_list(s: &str) -> Vec<u64> {
    s.split(',')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| {
            t.parse::<u64>().unwrap_or_else(|_| {
                eprintln!("bad ell value `{}` in ell-list", t);
                std::process::exit(1);
            })
        })
        .collect()
}

fn cmd_isogeny(op: IsogenyOp) {
    use crypto_lib::isogeny::{
        class_group::{class_above_prime, class_number, enumerate_reduced_forms, ClassGroup},
        cm::{cm_discriminant, fundamental_discriminant_and_conductor},
        experiment::{run_experiment, to_json, ExperimentConfig},
        graph::build_graph,
        secp256k1_analysis::{
            beta, embedding_degree_secp256k1, lambda, run_secp256k1_suite,
            small_degree_isogeny_survey, twist_pairing_friendly_check,
        },
        volcano::{j_invariant, map_volcano, position},
    };
    use num_bigint::BigInt;

    match op {
        IsogenyOp::Class { args } => {
            let (value, code) = match volcano_cli::run(&args) {
                Ok(result) => result,
                Err(error) => (serde_json::json!({"status":"ERROR","error":error}), 2),
            };
            println!("{}", serde_json::to_string_pretty(&value).unwrap());
            std::process::exit(code);
        }
        IsogenyOp::Volcano { curve, ell, depth } => {
            let c = parse_isogeny_curve(&curve);
            let m = map_volcano(&c, ell, depth, 256);
            let pos = position(&c, ell);
            println!("# ℓ-isogeny volcano for `{}` at ℓ = {}", curve, ell);
            println!();
            println!(
                "Curve:           y² = x³ + {}x + {}  over  F_{}",
                c.a, c.b, c.p
            );
            println!("j-invariant:     {}", j_invariant(&c));
            println!("Frobenius trace: {}", m.start_cm.trace);
            println!("#E(F_p):         {}", m.start_cm.order);
            println!(
                "Z[π] disc:       {}   (fundamental {}, Frobenius-order conductor {})",
                m.start_cm.frobenius_disc,
                m.start_cm.fundamental_disc,
                m.start_cm.frobenius_order_conductor,
            );
            let label =
                |v: Option<i64>| v.map(|n| n.to_string()).unwrap_or_else(|| "unknown".into());
            println!(
                "End(E) disc:     {}   (conductor {}; evidence {:?})",
                label(m.start_cm.endomorphism_disc),
                label(m.start_cm.endomorphism_conductor),
                m.start_cm.endomorphism_evidence,
            );
            println!(
                "Position:        depth {}, max rational depth {}, crater_size {}, on_crater = {}",
                label(pos.depth.map(i64::from)),
                label(pos.max_depth.map(i64::from)),
                label(pos.crater_size.map(i64::from)),
                pos.on_crater
                    .map(|b| b.to_string())
                    .unwrap_or_else(|| "unknown".into())
            );
            println!(
                "Neighborhood:    BFS distances; rational-kernel enumeration complete = {}",
                m.rational_kernel_enumeration_complete
            );
            println!();
            for level in &m.levels {
                let n = level.j_invariants.len();
                println!(
                    "BFS distance {:>2}: {:>3} vertices  j = {:?}",
                    level.bfs_distance, n, level.j_invariants
                );
            }
        }
        IsogenyOp::Graph {
            curve,
            ell_list,
            max_nodes,
        } => {
            let c = parse_isogeny_curve(&curve);
            let primes = parse_ell_list(&ell_list);
            let g = build_graph(&c, &primes, max_nodes);
            println!(
                "# {}-isogeny graph for `{}`",
                primes
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                curve
            );
            println!();
            println!("Vertices: {}", g.order());
            println!("Edges:    {}", g.size());
            let comps = g.connected_components();
            println!("Components: {}", comps.len());
            let diam = g.diameter();
            println!("Diameter: {}", diam);
            println!("Has cycle of length ≥ 3: {}", g.has_long_cycle());
            println!();
            for node in g.nodes.iter().take(32) {
                let neigh: Vec<String> = g.adj[node.index]
                    .iter()
                    .map(|&(n, l)| format!("{}(ℓ={})", n, l))
                    .collect();
                println!(
                    "  [{:>3}] j={} curve=(p={}, a={}, b={}) -> {}",
                    node.index,
                    node.j,
                    node.curve.0,
                    node.curve.1,
                    node.curve.2,
                    neigh.join(", "),
                );
            }
            if g.order() > 32 {
                println!("  ... ({} more vertices omitted)", g.order() - 32);
            }
        }
        IsogenyOp::Cm { discriminant } => {
            if discriminant >= 0 {
                eprintln!("discriminant must be negative");
                std::process::exit(1);
            }
            let d = BigInt::from(discriminant);
            let (fund, cond) = fundamental_discriminant_and_conductor(discriminant);
            let h = class_number(&d);
            println!("# CM data for D = {}", discriminant);
            println!();
            println!("Fundamental disc:  {}", fund);
            println!("Conductor:         {}", cond);
            println!("Class number h(D): {}", h);
            println!();
            println!("Reduced forms (representatives of Cl(O)):");
            for f in enumerate_reduced_forms(&d) {
                println!("  {}", f);
            }
            if h <= 64 {
                let cg = ClassGroup::new(&d);
                println!();
                println!("Identity (principal form): {}", cg.identity());
                if let Some(t) = &cg.mul_table {
                    println!();
                    println!("Multiplication table (indices):");
                    for row in t {
                        let cells: Vec<String> = row.iter().map(|i| format!("{:>2}", i)).collect();
                        println!("  {}", cells.join(" "));
                    }
                }
            }
            println!();
            println!("Small-prime ideal classes above ℓ:");
            for ell in [2u64, 3, 5, 7, 11, 13] {
                if let Some(f) = class_above_prime(&d, ell) {
                    println!("  ℓ = {:>2}:  {}", ell, f);
                } else {
                    println!("  ℓ = {:>2}:  inert / ramified (no split prime form)", ell);
                }
            }
        }
        IsogenyOp::Experiment {
            bits,
            trials,
            ell_list,
            rho_cap,
        } => {
            let mut cfg = ExperimentConfig::default_for_bits(bits);
            cfg.num_curves = trials.max(1);
            cfg.primes = parse_ell_list(&ell_list);
            cfg.max_graph_nodes = 16;
            // Cap scales with √n.  Hasse interval is centred at p, so
            // √n ≈ √p = 2^{bits/2}.  Allow 8× headroom for the
            // geometric-distribution tail.
            let half_bits = cfg.bits.min(60) / 2;
            cfg.rho_max_iters =
                rho_cap.unwrap_or_else(|| 8u64.checked_shl(half_bits).unwrap_or(u64::MAX));
            eprintln!(
                "# Running isogeny experiment: bits={}, trials={}, ell={:?}",
                cfg.bits, cfg.num_curves, cfg.primes,
            );
            let report = run_experiment(&cfg);
            println!("{}", to_json(&report));
            eprintln!(
                "[summary] {} starting curves, {} isogenous vertices, mean rho iters = {:.1}",
                report.aggregate.total_curves,
                report.aggregate.total_isogenous_vertices,
                report.aggregate.overall_mean_rho_iters,
            );
        }
        IsogenyOp::Secp256k1 => {
            let _ = cm_discriminant;
            let _ = map_volcano;
            let _ = twist_pairing_friendly_check;
            let (beta_ok, lambda_ok, mov_inf, twist_safe) = run_secp256k1_suite();
            println!("# secp256k1 case study");
            println!();
            println!("β (cube root of 1 in F_p):");
            println!("  hex = {:x}", beta());
            println!("  β³ ≡ 1 (mod p):       {}", beta_ok);
            println!();
            println!("λ (eigenvalue of φ on the n-torsion):");
            println!("  hex = {:x}", lambda());
            println!("  λ² + λ + 1 ≡ 0 (mod n): {}", lambda_ok);
            println!();
            println!("MOV / Frey–Rück feasibility:");
            println!(
                "  certificate: k > 200, embedding degree ≥ {}",
                embedding_degree_secp256k1(),
            );
            println!("  → MOV infeasible: {}", mov_inf);
            println!();
            println!("Quadratic-twist pairing-friendly survey (k ≤ 32):");
            println!("  → twist pairing-friendly: {}", !twist_safe);
            println!();
            println!("Small-degree-isogeny structural survey (Z[ω], disc = -3):");
            for (ell, status) in small_degree_isogeny_survey(&[2, 3, 5, 7, 11, 13, 17, 19, 23]) {
                println!("  ℓ = {:>2}:  {}", ell, status);
            }
        }
    }
}

// ── Cryptanalysis subcommands ─────────────────────────────────────────────────

fn cmd_aes_dfa(input: &Path) -> Result<(), String> {
    use crypto_lib::cryptanalysis::aes::dfa::{dfa_full_recovery, DfaFaultPair};
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct PairInput {
        plaintext_hex: String,
        correct_ciphertext_hex: String,
        faulted_ciphertext_hex: String,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct DfaInput {
        pairs: Vec<PairInput>,
    }

    let resolved = crypto_lib::store::resolve_path(input)?;
    let bytes =
        std::fs::read(&resolved).map_err(|e| format!("cannot read {}: {e}", input.display()))?;
    let data: DfaInput =
        serde_json::from_slice(&bytes).map_err(|e| format!("invalid AES DFA input JSON: {e}"))?;
    if !(1..=64).contains(&data.pairs.len()) {
        return Err("AES DFA requires 1..=64 fault pairs".into());
    }
    let parse_block = |text: &str, label: &str| -> Result<[u8; 16], String> {
        let bytes = from_hex(text).map_err(|e| format!("invalid {label} hex: {e}"))?;
        bytes
            .try_into()
            .map_err(|_| format!("{label} must be exactly 16 bytes"))
    };
    let pairs: Vec<DfaFaultPair> = data
        .pairs
        .iter()
        .enumerate()
        .map(|(index, pair)| {
            Ok(DfaFaultPair {
                plaintext: parse_block(&pair.plaintext_hex, &format!("pair {index} plaintext"))?,
                correct_ciphertext: parse_block(
                    &pair.correct_ciphertext_hex,
                    &format!("pair {index} correct ciphertext"),
                )?,
                faulted_ciphertext: parse_block(
                    &pair.faulted_ciphertext_hex,
                    &format!("pair {index} faulted ciphertext"),
                )?,
            })
        })
        .collect::<Result<_, String>>()?;
    let report = dfa_full_recovery(&pairs)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": 1,
            "operation": "aes-dfa",
            "fault_model": "single-byte-before-round-9-subbytes",
            "pairs_used": report.pairs_used,
            "faulted_pairs_replayed": report.faulted_pairs_replayed,
            "pairs_by_column": report.pairs_by_column,
            "candidate_tuples_by_column": report.candidates_by_column,
            "last_round_key_hex": report.last_round_key.map(|key| to_hex(&key)),
            "master_key_hex": report.master_key.map(|key| to_hex(&key)),
            "verified": report.verified,
            "status": if report.verified { "complete" } else { "bounded_ambiguous" },
        }))
        .map_err(|e| e.to_string())?
    );
    if !report.verified {
        return Err("AES DFA fault pairs did not determine a unique verified key".into());
    }
    Ok(())
}

fn cmd_aes_dfa_schedule(last_round_key_hex: &str) -> Result<(), String> {
    use crypto_lib::cryptanalysis::aes::{dfa, ReducedAes128};

    let bytes =
        from_hex(last_round_key_hex).map_err(|e| format!("invalid AES last-round key hex: {e}"))?;
    let last_round: [u8; 16] = bytes
        .try_into()
        .map_err(|_| "AES-128 last-round key must be exactly 16 bytes".to_string())?;
    let master = dfa::invert_aes128_schedule_from_last_round_key(&last_round);
    let expanded = ReducedAes128::new(&master, 10, false).round_key(10);
    let reexpanded_matches = expanded
        .iter()
        .enumerate()
        .all(|(column, word)| word.as_slice() == &last_round[column * 4..column * 4 + 4]);
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": 1,
            "operation": "aes-dfa-schedule",
            "last_round_key_hex": to_hex(&last_round),
            "master_key_hex": to_hex(&master),
            "round_key_reexpansion_matches": reexpanded_matches,
            "status": if reexpanded_matches { "schedule_inverted" } else { "verification_failed" },
        }))
        .map_err(|e| e.to_string())?
    );
    if !reexpanded_matches {
        return Err("AES-128 inverse key schedule failed re-expansion".into());
    }
    Ok(())
}

fn cmd_aes_cpa(input: &Path, known_ciphertext_hex: Option<&str>) -> Result<(), String> {
    use crypto_lib::cryptanalysis::aes::{dpa, ReducedAes128};
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct TraceInput {
        plaintexts: Vec<String>,
        traces_per_byte: [Vec<f64>; 16],
    }

    let bytes =
        std::fs::read(input).map_err(|e| format!("cannot read {}: {e}", input.display()))?;
    let trace_input: TraceInput =
        serde_json::from_slice(&bytes).map_err(|e| format!("invalid AES CPA trace JSON: {e}"))?;
    let count = trace_input.plaintexts.len();
    if !(16..=100_000).contains(&count) {
        return Err("AES CPA requires 16..=100000 aligned traces".into());
    }
    let mut plaintexts = Vec::with_capacity(count);
    for (index, encoded) in trace_input.plaintexts.iter().enumerate() {
        let value = from_hex(encoded)
            .map_err(|e| format!("invalid plaintext hex at index {index}: {e}"))?;
        let block: [u8; 16] = value
            .try_into()
            .map_err(|_| format!("plaintext at index {index} must be 16 bytes"))?;
        plaintexts.push(block);
    }
    let mut recovered = [0u8; 16];
    let mut peak_correlation = [0.0f64; 16];
    let mut runner_up_correlation = [0.0f64; 16];
    for byte in 0..16 {
        let traces = &trace_input.traces_per_byte[byte];
        if traces.len() != count || traces.iter().any(|v| !v.is_finite()) {
            return Err(format!(
                "trace channel {byte} must contain {count} finite samples"
            ));
        }
        if plaintexts.iter().all(|p| p[byte] == plaintexts[0][byte])
            || traces.iter().all(|&value| value == traces[0])
        {
            return Err(format!("trace channel {byte} has no usable variation"));
        }
        let (candidate, correlations) = dpa::cpa_recover_byte(&plaintexts, traces, byte);
        recovered[byte] = candidate;
        peak_correlation[byte] = correlations[candidate as usize].abs();
        runner_up_correlation[byte] = correlations
            .iter()
            .enumerate()
            .filter(|(guess, _)| *guess != candidate as usize)
            .map(|(_, value)| value.abs())
            .fold(0.0, f64::max);
    }
    let known_ciphertext = known_ciphertext_hex
        .map(|encoded| {
            let value = from_hex(encoded).map_err(|e| format!("invalid ciphertext hex: {e}"))?;
            let block: [u8; 16] = value
                .try_into()
                .map_err(|_| "known ciphertext must be 16 bytes".to_string())?;
            Ok::<_, String>(block)
        })
        .transpose()?;
    let verified = known_ciphertext.map(|ciphertext| {
        ReducedAes128::new(&recovered, 10, false).encrypt(&plaintexts[0]) == ciphertext
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": 1,
            "operation": "aes-cpa",
            "model": "first-round-sbox-hamming-weight",
            "samples_per_byte": count,
            "candidate_key_hex": to_hex(&recovered),
            "peak_abs_correlation": peak_correlation,
            "runner_up_abs_correlation": runner_up_correlation,
            "verified": verified,
            "status": if verified == Some(true) { "complete" } else if verified == Some(false) { "verification_failed" } else { "candidate" },
        }))
        .map_err(|e| e.to_string())?
    );
    if verified == Some(false) {
        return Err("AES CPA candidate failed the supplied ciphertext check".into());
    }
    Ok(())
}

fn cmd_aes_cache_timing(input: &Path, min_samples_per_bucket: usize) -> Result<(), String> {
    use crypto_lib::cryptanalysis::aes::cache_timing::recover_isolated_lookup_high_nibbles;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct TimingInput {
        plaintexts: Vec<String>,
        timings_per_byte: [Vec<f64>; 16],
    }

    let resolved = crypto_lib::store::resolve_path(input)?;
    let bytes =
        std::fs::read(&resolved).map_err(|e| format!("cannot read {}: {e}", input.display()))?;
    let data: TimingInput = serde_json::from_slice(&bytes)
        .map_err(|e| format!("invalid AES cache timing JSON: {e}"))?;
    if !(64..=100_000).contains(&data.plaintexts.len()) {
        return Err("AES cache timing requires 64..=100000 aligned observations".into());
    }
    if !(1..=data.plaintexts.len() / 16).contains(&min_samples_per_bucket) {
        return Err("minimum samples per bucket is outside the observation count".into());
    }
    let mut plaintexts = Vec::with_capacity(data.plaintexts.len());
    for (index, encoded) in data.plaintexts.iter().enumerate() {
        let value = from_hex(encoded)
            .map_err(|e| format!("invalid plaintext hex at index {index}: {e}"))?;
        let block: [u8; 16] = value
            .try_into()
            .map_err(|_| format!("plaintext at index {index} must be 16 bytes"))?;
        plaintexts.push(block);
    }
    let result = recover_isolated_lookup_high_nibbles(
        &plaintexts,
        &data.timings_per_byte,
        min_samples_per_bucket,
    )?;
    let resolved_bytes = result.high_nibbles.iter().filter(|v| v.is_some()).count();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "schema_version": 1,
            "operation": "aes-cache-timing",
            "model": "instrumented-isolated-first-lookup-cache-line",
            "observations": plaintexts.len(),
            "min_samples_per_bucket": min_samples_per_bucket,
            "candidate_high_nibbles": result.high_nibbles,
            "resolved_bytes": resolved_bytes,
            "minimum_bucket_count": result.minimum_bucket_count,
            "runner_up_mean_gap": result.runner_up_mean_gap,
            "status": if resolved_bytes == 16 { "candidate" } else { "ambiguous" },
        }))
        .map_err(|e| e.to_string())?
    );
    Ok(())
}

fn cmd_aes_attack(
    technique: &str,
    key_hex: &str,
    samples: usize,
    seed: u64,
    alpha: u8,
) -> Result<(), String> {
    use crypto_lib::cryptanalysis::aes::{differential, linear, square, ReducedAes128};

    let key_vec = from_hex(key_hex).map_err(|e| format!("invalid AES key hex: {e}"))?;
    let key: [u8; 16] = key_vec
        .try_into()
        .map_err(|_| "AES-128 key must be exactly 16 bytes".to_string())?;
    if !(1..=1_000_000).contains(&samples) {
        return Err("samples must be in 1..=1000000".into());
    }
    match technique {
        "differential" => {
            if alpha == 0 {
                return Err("alpha must be nonzero".into());
            }
            let cipher = ReducedAes128::new(&key, 2, false);
            let pairs = differential::build_pairs(&cipher, alpha, samples, seed);
            let ddt = differential::aes_sbox_ddt();
            let report = differential::key_recovery_two_round(alpha, &pairs, &ddt)
                .map_err(|e| format!("bounded differential recovery: {e}"))?;
            let actual = cipher.round_key(2);
            let verified = report
                .key_bytes
                .iter()
                .all(|&(pos, byte)| actual[pos / 4][pos % 4] == byte);
            println!("# AES-128 two-round differential recovery");
            println!(
                "pairs: {}; trials: {}",
                report.pairs_used, report.trial_count
            );
            for (pos, byte) in report.key_bytes {
                println!("last-round key byte {pos}: {byte:02x}");
            }
            println!("verified against supplied analysis key: {verified}");
            if !verified {
                return Err("recovered bytes failed key-schedule verification".into());
            }
        }
        "linear" => {
            let cipher = ReducedAes128::new(&key, 1, false);
            let pairs = linear::known_plaintext_corpus(&cipher, samples, seed);
            let lat = linear::aes_sbox_lat();
            let mut recovered = [0u8; 16];
            for c in 0..4 {
                for r in 0..4 {
                    let byte = linear::recover_key_byte_one_round(&pairs, (c, r), &lat)
                        .ok_or_else(|| format!("bounded linear recovery at ({c},{r})"))?;
                    recovered[4 * ((c + r) % 4) + r] = byte;
                }
            }
            let verified = recovered == key;
            println!("# AES-128 one-round linear recovery");
            println!("known pairs: {samples}");
            println!("recovered input key: {}", to_hex(&recovered));
            println!("verified against supplied analysis key: {verified}");
            if !verified {
                return Err("recovered key failed verification".into());
            }
        }
        "square" => {
            let cipher = ReducedAes128::new(&key, 4, false);
            let report = square::key_recovery_four_round(|p| cipher.encrypt(p));
            let verified = report.master_key == key;
            println!("# AES-128 four-round integral recovery");
            println!(
                "lambda sets: {}; S-box lookups: {}",
                report.lambda_sets_used, report.sbox_lookups
            );
            println!("recovered master key: {}", to_hex(&report.master_key));
            println!("verified against supplied analysis key: {verified}");
            if !verified {
                return Err("recovered key failed verification".into());
            }
        }
        other => {
            return Err(format!(
                "unknown AES technique '{other}'; use differential, linear, or square"
            ))
        }
    }
    Ok(())
}

fn cmd_aes_distinguisher(
    technique: &str,
    key_hex: &str,
    samples: usize,
    seed: u64,
    alpha: u8,
) -> Result<(), String> {
    use crypto_lib::cryptanalysis::aes::{
        impossible, mixture, yoyo, zero_correlation, ReducedAes128,
    };

    if !(1..=1_000_000).contains(&samples) {
        return Err("samples must be in 1..=1000000".into());
    }
    let key_vec = from_hex(key_hex).map_err(|e| format!("invalid AES key hex: {e}"))?;
    let key: [u8; 16] = key_vec
        .try_into()
        .map_err(|_| "AES-128 key must be exactly 16 bytes".to_string())?;
    match technique {
        "impossible" => {
            if alpha == 0 {
                return Err("alpha must be nonzero".into());
            }
            let cipher = ReducedAes128::new(&key, 4, false);
            let report = impossible::verify_4_round_impossibility(&cipher, alpha, samples, seed);
            println!("# AES-128 four-round impossible differential");
            println!("pairs tested: {}", report.pairs_tested);
            println!(
                "one-active-byte output pairs: {}",
                report.one_active_output_pairs
            );
            println!(
                "low-activity output pairs: {}",
                report.low_activity_output_pairs
            );
            let verified = report.one_active_output_pairs == 0;
            println!("structural check: {verified}");
            if !verified {
                return Err(
                    "observed a pair contradicting the specified impossible differential".into(),
                );
            }
        }
        "yoyo" => {
            if samples < 64 {
                return Err("yoyo comparison requires at least 64 trials".into());
            }
            let two = yoyo::yoyo_distinguisher(&ReducedAes128::new(&key, 2, false), samples, seed);
            let three =
                yoyo::yoyo_distinguisher(&ReducedAes128::new(&key, 3, false), samples, seed);
            println!("# AES-128 yoyo distinguisher (2 vs 3 rounds)");
            println!("trials: {samples}");
            println!("two-round structure preserved: {}", two.structure_preserved);
            println!(
                "three-round structure preserved: {}",
                three.structure_preserved
            );
            let verified = two.structure_preserved == samples
                && three.structure_preserved < two.structure_preserved;
            println!("observed round distinction: {verified}");
            if !verified {
                return Err("configured trial panel did not separate the two round counts".into());
            }
        }
        "mixture" => {
            if samples < 64 {
                return Err("mixture comparison requires at least 64 quadruples".into());
            }
            let one = ReducedAes128::new(&key, 1, false);
            let two = ReducedAes128::new(&key, 2, false);
            let report = mixture::column_confined_subspace_check(&one, &two, samples, seed);
            let one_identity = mixture::algebraic_identity_holds(&one, samples, seed);
            let two_identity = mixture::algebraic_identity_holds(&two, samples, seed);
            println!("# AES-128 mixture identity distinguisher (1 vs 2 rounds)");
            println!("quadruples: {}", report.quadruples);
            println!("one-round mixture identity: {one_identity}");
            println!("two-round mixture identity: {two_identity}");
            println!(
                "one-round column confined: {}",
                report.one_round_column_confined
            );
            println!(
                "two-round column confined: {}",
                report.two_round_column_confined
            );
            let verified = one_identity && !two_identity && report.two_round_column_confined == 0;
            println!("observed round distinction: {verified}");
            if !verified {
                return Err("configured mixture panel did not separate the round counts".into());
            }
        }
        "zero-correlation" => {
            let pairs = zero_correlation::enumerate_zero_correlation_pairs(1e-12);
            let verified = !pairs.is_empty() && pairs.iter().all(|(_, _, c)| c.abs() < 1e-12);
            println!("# AES S-box zero-correlation mask pairs");
            println!(
                "nonzero-mask pairs with exact zero correlation: {}",
                pairs.len()
            );
            if let Some((input_mask, output_mask, correlation)) = pairs.first() {
                println!("first pair: {input_mask:02x} -> {output_mask:02x} ({correlation:.6})");
            }
            println!("S-box correlation check: {verified}");
            if !verified {
                return Err("S-box zero-correlation enumeration failed its exact check".into());
            }
        }
        other => {
            return Err(format!(
                "unknown AES distinguisher '{other}'; use impossible, yoyo, mixture, or zero-correlation"
            ));
        }
    }
    Ok(())
}

fn cmd_cryptanalysis(op: CryptanalysisOp) {
    use crypto_lib::cryptanalysis::auto_attack::{auto_attack_with, AutoAttackOptions};
    use crypto_lib::cryptanalysis::boomerang::{boomerang_distinguisher, rectangle_attack};
    use crypto_lib::cryptanalysis::cipher_registry::{list_ciphers, RegisteredCipher};
    use crypto_lib::cryptanalysis::research_bench::run_full_bench;
    match op {
        CryptanalysisOp::ListCiphers => {
            println!("Registered ciphers:");
            for name in list_ciphers() {
                if let Some(c) = RegisteredCipher::from_name(name) {
                    let e = c.entry();
                    println!(
                        "  {:<22} {} block={}B rounds={}",
                        name, e.description, e.block_bytes, e.rounds,
                    );
                }
            }
        }
        CryptanalysisOp::Auto {
            cipher,
            boomerang_pairs,
            rectangle_pool,
            trail_threshold_neg_log2,
            trail_top_k,
        } => {
            let opts = AutoAttackOptions {
                boomerang_pairs,
                rectangle_pool,
                trail_threshold: 2f64.powi(-(trail_threshold_neg_log2 as i32)),
                trail_top_k,
            };
            match auto_attack_with(&cipher, opts) {
                Ok(r) => {
                    println!("{}", r.markdown);
                    eprintln!(
                        "[auto-attack] {} sections in {} ms",
                        r.sections_run, r.elapsed_ms,
                    );
                }
                Err(e) => {
                    eprintln!("error: {}", e);
                    eprintln!("Try one of: {}", list_ciphers().join(", "));
                    std::process::exit(1);
                }
            }
        }
        CryptanalysisOp::Boomerang {
            cipher,
            alpha,
            delta,
            pairs,
        } => {
            let c = match RegisteredCipher::from_name(&cipher) {
                Some(c) => c,
                None => {
                    eprintln!("error: unknown cipher: {}", cipher);
                    std::process::exit(1);
                }
            };
            let entry = c.entry();
            let alpha_bytes = match alpha {
                Some(h) => from_hex(&h).expect("bad alpha hex"),
                None => entry.canonical_alpha.clone(),
            };
            let delta_bytes = match delta {
                Some(h) => from_hex(&h).expect("bad delta hex"),
                None => entry.canonical_delta.clone(),
            };
            if alpha_bytes.len() != entry.block_bytes || delta_bytes.len() != entry.block_bytes {
                eprintln!(
                    "error: alpha/delta must be {} bytes for this cipher",
                    entry.block_bytes
                );
                std::process::exit(1);
            }
            let t0 = std::time::Instant::now();
            let r = boomerang_distinguisher(&c, &alpha_bytes, &delta_bytes, pairs, None);
            let elapsed = t0.elapsed().as_millis();
            println!("# Boomerang distinguisher on `{}`", cipher);
            println!();
            println!("- elapsed:       {} ms", elapsed);
            println!("- right quartets: {} / {}", r.right_quartets, r.n_pairs);
            println!("- empirical p:   {:.4e}", r.empirical_probability);
            println!("- random baseline: {:.4e}", r.random_baseline);
            println!(
                "- distinguishes from random (10×): {}",
                if r.distinguishes_from_random(10.0) {
                    "yes"
                } else {
                    "no"
                },
            );
        }
        CryptanalysisOp::Rectangle {
            cipher,
            alpha,
            delta,
            pool,
        } => {
            let c = match RegisteredCipher::from_name(&cipher) {
                Some(c) => c,
                None => {
                    eprintln!("error: unknown cipher: {}", cipher);
                    std::process::exit(1);
                }
            };
            let entry = c.entry();
            let alpha_bytes = match alpha {
                Some(h) => from_hex(&h).expect("bad alpha hex"),
                None => entry.canonical_alpha.clone(),
            };
            let delta_bytes = match delta {
                Some(h) => from_hex(&h).expect("bad delta hex"),
                None => entry.canonical_delta.clone(),
            };
            let t0 = std::time::Instant::now();
            let r = rectangle_attack(&c, &alpha_bytes, &delta_bytes, pool, None);
            let elapsed = t0.elapsed().as_millis();
            println!("# Rectangle attack on `{}`", cipher);
            println!();
            println!("- elapsed:           {} ms", elapsed);
            println!("- pool size:         {}", r.pool_size);
            println!("- right rectangles:  {}", r.right_quartets);
        }
        CryptanalysisOp::Sbox { cipher } => {
            let c = match RegisteredCipher::from_name(&cipher) {
                Some(c) => c,
                None => {
                    eprintln!("error: unknown cipher: {}", cipher);
                    std::process::exit(1);
                }
            };
            match c.sbox() {
                Some(sbox) => {
                    let r = sbox.report();
                    println!("# S-box report for `{}`", cipher);
                    println!();
                    println!("- bits in / out:                 {} / {}", r.n_in, r.n_out);
                    println!("- bijective:                     {}", r.bijective);
                    println!("- balanced:                      {}", r.balanced);
                    println!(
                        "- differential uniformity:       {}",
                        r.differential_uniformity
                    );
                    println!(
                        "- max differential probability:  {:.4}",
                        r.max_differential_probability
                    );
                    println!("- max linear bias:               {:.4}", r.max_linear_bias);
                    println!("- nonlinearity:                  {}", r.nonlinearity);
                    println!("- algebraic degree:              {}", r.algebraic_degree);
                    println!(
                        "- boomerang uniformity:          {}",
                        r.boomerang_uniformity
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "—".into()),
                    );
                    println!("- max DLCT bias:                 {:.4}", r.max_dlct_bias);
                }
                None => {
                    eprintln!("error: cipher {} has no exposed S-box", cipher);
                    std::process::exit(1);
                }
            }
        }
        CryptanalysisOp::AesAttack {
            technique,
            key_hex,
            samples,
            seed,
            alpha,
        } => {
            if let Err(e) = cmd_aes_attack(&technique, &key_hex, samples, seed, alpha) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        CryptanalysisOp::AesDistinguisher {
            technique,
            key_hex,
            samples,
            seed,
            alpha,
        } => {
            if let Err(e) = cmd_aes_distinguisher(&technique, &key_hex, samples, seed, alpha) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        CryptanalysisOp::AesCpa {
            input,
            known_ciphertext_hex,
        } => {
            if let Err(e) = cmd_aes_cpa(&input, known_ciphertext_hex.as_deref()) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        CryptanalysisOp::AesCacheTiming {
            input,
            min_samples_per_bucket,
        } => {
            if let Err(e) = cmd_aes_cache_timing(&input, min_samples_per_bucket) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        CryptanalysisOp::AesDifferentialObserved { input } => {
            if let Err(e) = cli_aes_observed::differential(&input) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        CryptanalysisOp::AesLinearObserved { input } => {
            if let Err(e) = cli_aes_observed::linear(&input) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        CryptanalysisOp::AesDfaSchedule { last_round_key_hex } => {
            if let Err(e) = cmd_aes_dfa_schedule(&last_round_key_hex) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        CryptanalysisOp::AesDfa { input } => {
            if let Err(e) = cmd_aes_dfa(&input) {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        CryptanalysisOp::Bench => {
            eprintln!("Running full research bench (this takes ~15-30 s in release mode)...");
            let report = run_full_bench(1);
            println!("{}", report);
        }
        CryptanalysisOp::RhoCollab { op } => cmd_rho_collab(op),
        CryptanalysisOp::HashAuto { hash } => {
            use crypto_lib::cryptanalysis::hash_attacks::auto_hash_attack;
            match auto_hash_attack(&hash) {
                Ok(md) => println!("{}", md),
                Err(e) => {
                    eprintln!("error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        CryptanalysisOp::LengthExtension {
            hash,
            secret_len,
            message_hex,
            suffix_hex,
            digest_hex,
        } => {
            use crypto_lib::cryptanalysis::hash_attacks::{
                length_extension_attack, Md4, Md5, MerkleDamgardHash, Sha1, Sha256,
            };
            let parse = |label: &str, value: &str| {
                from_hex(value).unwrap_or_else(|e| {
                    eprintln!("length-extension: invalid {label} hex: {e}");
                    std::process::exit(2);
                })
            };
            let message = parse("message", &message_hex);
            let suffix = parse("suffix", &suffix_hex);
            let digest = parse("digest", &digest_hex);
            let prefix_len = secret_len.checked_add(message.len()).unwrap_or_else(|| {
                eprintln!("length-extension: secret and message lengths overflow");
                std::process::exit(2);
            });
            fn report_lex<H: MerkleDamgardHash>(
                hash: &H,
                digest: &[u8],
                prefix_len: usize,
                message: &[u8],
                suffix: &[u8],
            ) {
                if digest.len() != hash.output_bytes() {
                    eprintln!(
                        "length-extension: {} requires a {}-byte digest, got {} bytes",
                        hash.name(),
                        hash.output_bytes(),
                        digest.len()
                    );
                    std::process::exit(2);
                }
                let (forged, glue) = length_extension_attack(hash, digest, prefix_len, suffix);
                println!("# Length-extension attack on `{}`", hash.name());
                println!();
                println!("Forged digest: {}", to_hex(&forged));
                println!("Glue padding bytes ({} B): {}", glue.len(), to_hex(&glue));
                println!();
                println!("Effective extended message (without secret):");
                println!(
                    "  message ({}B) || glue ({}B) || suffix ({}B)",
                    message.len(),
                    glue.len(),
                    suffix.len()
                );
                let mut combined = Vec::new();
                combined.extend_from_slice(message);
                combined.extend_from_slice(&glue);
                combined.extend_from_slice(suffix);
                println!("Hex: {}", to_hex(&combined));
            }
            match hash.as_str() {
                "md4" => report_lex(&Md4, &digest, prefix_len, &message, &suffix),
                "md5" => report_lex(&Md5, &digest, prefix_len, &message, &suffix),
                "sha1" => report_lex(&Sha1, &digest, prefix_len, &message, &suffix),
                "sha256" | "sha-256" => report_lex(&Sha256, &digest, prefix_len, &message, &suffix),
                other => {
                    eprintln!(
                        "error: unknown hash '{}' (try md4, md5, sha1, sha256)",
                        other
                    );
                    std::process::exit(1);
                }
            }
        }
        CryptanalysisOp::AesRelatedKey {
            key_bits,
            delta_k_hex,
            avalanche_trials,
            rounds,
        } => {
            use crypto_lib::cryptanalysis::aes::related_key::{
                biryukov_khovratovich_4round_demo, format_key_schedule_diff,
                key_schedule_difference, related_key_avalanche,
            };
            use crypto_lib::symmetric::aes::AesKey;
            let key_bytes = match key_bits {
                128 => 16,
                256 => 32,
                _ => {
                    eprintln!("error: --key-bits must be 128 or 256");
                    std::process::exit(1);
                }
            };
            let default_rounds = if key_bits == 128 { 10 } else { 14 };
            let rounds = if rounds == 0 { default_rounds } else { rounds };
            let delta_k: Vec<u8> = match &delta_k_hex {
                Some(h) => {
                    let v = from_hex(h).expect("bad --delta-k-hex");
                    if v.len() != key_bytes {
                        eprintln!(
                            "error: --delta-k-hex must be {} bytes (got {})",
                            key_bytes,
                            v.len()
                        );
                        std::process::exit(1);
                    }
                    v
                }
                None => {
                    let mut d = vec![0u8; key_bytes];
                    d[0] = 0x01;
                    d
                }
            };
            // 1. Key-schedule difference propagation.
            let zero_key_bytes = vec![0u8; key_bytes];
            let zero_key = AesKey::new(&zero_key_bytes).unwrap();
            let diff = key_schedule_difference(&zero_key, &delta_k);
            println!(
                "# AES-{} related-key attack against ΔK = 0x{}",
                key_bits,
                to_hex(&delta_k)
            );
            println!();
            println!("## Key-schedule difference propagation\n");
            println!("{}", format_key_schedule_diff(&diff));
            // 2. Related-key avalanche (AES-128 only — the avalanche
            // function uses ReducedAes128 which is 16-byte-key only).
            if key_bits == 128 {
                println!(
                    "## Related-key avalanche ({} rounds, {} trials)\n",
                    rounds, avalanche_trials
                );
                let zero_key_16 = [0u8; 16];
                let mut dk16 = [0u8; 16];
                dk16.copy_from_slice(&delta_k);
                let r = related_key_avalanche(&zero_key_16, &dk16, rounds, avalanche_trials);
                println!("- mean Hamming distance: {:.2} bits", r.mean_distance_bits);
                println!("- ideal-cipher baseline: {:.2} bits", r.ideal_baseline);
                println!("- bias from ideal:       {:.2} bits", r.bias);
                println!();
                println!("## Biryukov-Khovratovich-style local collision\n");
                println!("{} trials at {} rounds:", avalanche_trials, rounds);
                let r = biryukov_khovratovich_4round_demo(avalanche_trials, rounds);
                println!(
                    "- low-Hamming-distance hits: {} / {}",
                    r.collisions, r.n_trials
                );
                println!(
                    "- empirical hit rate:        {:.4}",
                    r.empirical_probability
                );
                println!("- random baseline (Hd≤16 in 128 bits): ≈ 2⁻⁷⁹");
            } else {
                println!("(Related-key avalanche + local-collision demo only available for AES-128; key-schedule diffusion table above already shows the AES-256 weakness.)");
            }
        }
        CryptanalysisOp::AesVisualDemo { demo } => {
            use crypto_lib::cryptanalysis::aes::{
                dfa::{dfa_per_column_candidates, format_dfa_visualization, inject_fault_round_9},
                higher_order::{integral_distinguisher_3_round, render_integral_visualization},
                reduced::ReducedAes128,
                related_key::{format_key_schedule_diff, key_schedule_difference},
                truncated_diff::{
                    propagate_truncated_round, render_trail_diagram, TruncatedPattern,
                },
            };
            use crypto_lib::symmetric::aes::AesKey;
            let run_truncated = |x: &str| -> bool { x == "all" || x == "truncated-diff" };
            let run_integral = |x: &str| -> bool { x == "all" || x == "integral" };
            let run_dfa = |x: &str| -> bool { x == "all" || x == "dfa" };
            let run_ks = |x: &str| -> bool { x == "all" || x == "key-schedule" };
            if run_truncated(&demo) {
                println!("\n## (1) Truncated-differential trail on AES (single active byte)\n");
                let mut current = TruncatedPattern(1);
                let mut steps = Vec::new();
                for _ in 0..4 {
                    let step = propagate_truncated_round(current);
                    let mut next = 0u16;
                    for c in 0..4 {
                        let n = *step.mc_output_counts[c].iter().min().unwrap();
                        for r in 0..n as usize {
                            next |= 1u16 << (4 * c + r);
                        }
                    }
                    current = TruncatedPattern(next);
                    steps.push(step);
                }
                println!("{}", render_trail_diagram(&steps));
            }
            if run_integral(&demo) {
                println!("\n## (2) Square / integral distinguisher on 3-round AES\n");
                let key = [0u8; 16];
                let other = [0u8; 15];
                let xor_sum = integral_distinguisher_3_round(&key, &other);
                println!("{}", render_integral_visualization(&xor_sum));
            }
            if run_dfa(&demo) {
                println!("\n## (3) DFA / Piret-Quisquater single-byte fault on AES-128\n");
                let key_bytes = [
                    0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09,
                    0xcf, 0x4f, 0x3c,
                ];
                let cipher = ReducedAes128::new(&key_bytes, 10, false);
                let pt = [0x6bu8; 16];
                let (correct, faulted) = inject_fault_round_9(&cipher, &pt, 0, 0xCC);
                let cands = dfa_per_column_candidates(&correct, &faulted);
                println!("{}", format_dfa_visualization(&correct, &faulted, &cands));
            }
            if run_ks(&demo) {
                println!("\n## (4) Related-key schedule diffusion (AES-256 vs AES-128)\n");
                for &(bits, label) in &[(128u32, "AES-128"), (256u32, "AES-256")] {
                    let key_bytes = vec![0u8; (bits / 8) as usize];
                    let key = AesKey::new(&key_bytes).unwrap();
                    let mut delta = vec![0u8; (bits / 8) as usize];
                    delta[0] = 0x01;
                    let diff = key_schedule_difference(&key, &delta);
                    println!("\n### {} (ΔK = single byte at position 0)\n", label);
                    println!("{}", format_key_schedule_diff(&diff));
                }
            }
        }
        CryptanalysisOp::VisualAll { target } => {
            use crypto_lib::cryptanalysis::visual_demos::{
                demo_birthday_paradox, demo_bleichenbacher_bias, demo_hnp_recovery_curve,
                demo_j0_twist_factors, demo_joux_multicollision, demo_length_extension_diagram,
                demo_pollard_rho_path, demo_sbox_ddt_heatmap, demo_sbox_lat_heatmap,
                demo_walsh_spectrum, run_all_visual_demos,
            };
            let out: String = match target.as_str() {
                "sbox-ddt" => demo_sbox_ddt_heatmap(),
                "sbox-lat" => demo_sbox_lat_heatmap(),
                "walsh" => demo_walsh_spectrum(),
                "pollard-rho" => demo_pollard_rho_path(),
                "hnp" => demo_hnp_recovery_curve(),
                "bleichenbacher" => demo_bleichenbacher_bias(),
                "length-extension" => demo_length_extension_diagram(),
                "joux" => demo_joux_multicollision(),
                "j0-twists" => demo_j0_twist_factors(),
                "birthday" => demo_birthday_paradox(),
                "all" => run_all_visual_demos(),
                other => {
                    eprintln!("error: unknown target '{}'; try `all` or one of: sbox-ddt, sbox-lat, walsh, pollard-rho, hnp, bleichenbacher, length-extension, joux, j0-twists, birthday", other);
                    std::process::exit(1);
                }
            };
            println!("{}", out);
        }
    }
}

// ── Command implementations ───────────────────────────────────────────────────

fn load_hash_input(
    message: Option<&str>,
    hex: Option<&str>,
    file: Option<&Path>,
) -> Result<Vec<u8>, String> {
    match (message, hex, file) {
        (Some(message), None, None) => Ok(message.as_bytes().to_vec()),
        (None, Some(hex), None) => from_hex(hex).map_err(|e| format!("invalid input hex: {e}")),
        (None, None, Some(file)) => std::fs::read(file)
            .map_err(|e| format!("cannot read input file {}: {e}", file.display())),
        (None, None, None) => Err("provide a message, --hex, or --file".into()),
        _ => Err("provide exactly one of message, --hex, or --file".into()),
    }
}

fn cmd_hash(alg: &str, bytes: &[u8], output_bytes: Option<usize>) -> Result<String, String> {
    let is_xof = matches!(alg, "shake128" | "shake256");
    let length = if is_xof {
        match output_bytes {
            Some(length @ 1..=1_048_576) => length,
            _ => return Err("SHAKE requires --output-bytes in 1..=1048576".into()),
        }
    } else {
        if output_bytes.is_some() {
            return Err("--output-bytes applies only to SHAKE128 and SHAKE256".into());
        }
        0
    };
    let digest = match alg {
        "sha224" | "sha-224" => to_hex(&sha224(bytes)),
        "sha256" | "sha-256" => to_hex(&sha256(bytes)),
        "sha384" | "sha-384" => to_hex(&sha384(bytes)),
        "sha512" | "sha-512" => to_hex(&sha512(bytes)),
        "sha512-224" | "sha-512-224" => to_hex(&sha512_224(bytes)),
        "sha512-256" | "sha-512-256" => to_hex(&sha512_256(bytes)),
        "sha3-224" => to_hex(&sha3_224(bytes)),
        "sha3" | "sha3-256" => to_hex(&sha3_256(bytes)),
        "sha3-384" => to_hex(&sha3_384(bytes)),
        "sha3-512" => to_hex(&sha3_512(bytes)),
        "shake128" => to_hex(&shake128(bytes, length)),
        "shake256" => to_hex(&shake256(bytes, length)),
        "blake3" => to_hex(&blake3_hash(bytes)),
        _ => return Err(format!("unknown algorithm {alg:?}; use sha224, sha256, sha384, sha512, sha512-224, sha512-256, sha3-224, sha3-256, sha3-384, sha3-512, shake128, shake256, or blake3")),
    };
    Ok(digest)
}

fn cmd_ecc_keygen() {
    let curve = CurveParams::secp256k1();
    let kp = EccKeyPair::generate(&curve);

    println!("=== secp256k1 Key Pair ===");
    println!(
        "Private key: {}",
        to_hex(&bigint_to_bytes_be(&kp.private.scalar, 32))
    );
    if let Some(pub_bytes) = kp.public.to_sec1_uncompressed() {
        println!("Public key (uncompressed): {}", to_hex(&pub_bytes));
    }
}

fn cmd_ecdsa(op: EcdsaOp) {
    let curve = CurveParams::secp256k1();
    match op {
        EcdsaOp::Sign { message } => {
            let kp = EccKeyPair::generate(&curve);
            let sig = sign(message.as_bytes(), &kp.private, &curve);
            println!("Message: {message}");
            println!("r: {}", to_hex(&bigint_to_bytes_be(&sig.r, 32)));
            println!("s: {}", to_hex(&bigint_to_bytes_be(&sig.s, 32)));
            if let Some(pub_bytes) = kp.public.to_sec1_uncompressed() {
                println!("Public key: {}", to_hex(&pub_bytes));
            }
            let valid = verify(message.as_bytes(), &kp.public, &sig, &curve);
            println!("Verified: {valid}");
        }
        EcdsaOp::Verify {
            message,
            pubkey_hex,
            r_hex,
            s_hex,
        } => {
            let pub_bytes = from_hex(&pubkey_hex).expect("invalid pubkey hex");
            if pub_bytes.len() != 65 || pub_bytes[0] != 0x04 {
                eprintln!("Expected 65-byte uncompressed public key (04 || X || Y)");
                return;
            }
            let x = BigUint::from_bytes_be(&pub_bytes[1..33]);
            let y = BigUint::from_bytes_be(&pub_bytes[33..65]);
            let p = CurveParams::secp256k1().p;
            let pub_key = EccPublicKey {
                point: Point::Affine {
                    x: FieldElement::new(x, p.clone()),
                    y: FieldElement::new(y, p),
                },
                curve_name: "secp256k1".to_string(),
            };
            let r = BigUint::from_bytes_be(&from_hex(&r_hex).expect("bad r"));
            let s = BigUint::from_bytes_be(&from_hex(&s_hex).expect("bad s"));
            let sig = EcdsaSignature { r, s };
            let valid = verify(message.as_bytes(), &pub_key, &sig, &curve);
            println!("Signature valid: {valid}");
        }
    }
}

fn cmd_ecdh() {
    let curve = CurveParams::secp256k1();
    println!("=== ECDH Key Exchange (secp256k1) ===");

    let alice = EccKeyPair::generate(&curve);
    let bob = EccKeyPair::generate(&curve);

    let sa = ecdh(&alice.private, &bob.public, &curve).unwrap();
    let sb = ecdh(&bob.private, &alice.public, &curve).unwrap();

    println!(
        "Alice private: {}",
        to_hex(&bigint_to_bytes_be(&alice.private.scalar, 32))
    );
    println!(
        "Bob   private: {}",
        to_hex(&bigint_to_bytes_be(&bob.private.scalar, 32))
    );
    println!("Alice shared:  {}", to_hex(&sa));
    println!("Bob   shared:  {}", to_hex(&sb));
    println!("Match: {}", sa == sb);
}

fn cmd_rsa(op: RsaOp) {
    match op {
        RsaOp::Keygen => {
            println!("Generating 1024-bit RSA key pair…");
            let kp = RsaKeyPair::generate(1024);
            println!(
                "n (256-bit prefix): {}…",
                &to_hex(&bigint_to_bytes_be(&kp.public.n, 128))[..64]
            );
            println!("e: {}", kp.public.e);
        }
        RsaOp::Demo => {
            println!("Generating 1024-bit RSA key pair for demo…");
            let kp = RsaKeyPair::generate(1024);
            let msg = b"RSA demo message";
            let sig = rsa_sign(msg, &kp.private);
            let ok = rsa_verify(msg, &sig, &kp.public);
            println!("Message: {:?}", std::str::from_utf8(msg).unwrap());
            println!(
                "Signature (first 16 bytes): {}",
                to_hex(&bigint_to_bytes_be(&sig, 16))
            );
            println!("Verified: {ok}");
        }
    }
}

fn cmd_aes() {
    println!("=== AES-256-GCM ===");
    let key_bytes = [0x42u8; 32];
    let key = AesKey::Aes256(key_bytes);
    let nonce = [0x00u8; 12];
    let pt = b"Hello, AES-GCM!";
    let aad = b"authenticated header";

    let ct = aes_gcm_encrypt(pt, &key, &nonce, aad);
    println!("Plaintext:  {}", std::str::from_utf8(pt).unwrap());
    println!("Ciphertext: {}", to_hex(&ct));

    let recovered = aes_gcm_decrypt(&ct, &key, &nonce, aad).unwrap();
    println!("Decrypted:  {}", std::str::from_utf8(&recovered).unwrap());
}

fn cmd_chacha() {
    println!("=== ChaCha20-Poly1305 ===");
    let key = [0x42u8; 32];
    let nonce = [0x00u8; 12];
    let pt = b"Hello, ChaCha20-Poly1305!";
    let aad = b"additional data";

    let ct = chacha20_poly1305_encrypt(pt, &key, &nonce, aad);
    println!("Plaintext:  {}", std::str::from_utf8(pt).unwrap());
    println!("Ciphertext: {}", to_hex(&ct));

    let recovered = chacha20_poly1305_decrypt(&ct, &key, &nonce, aad).unwrap();
    println!("Decrypted:  {}", std::str::from_utf8(&recovered).unwrap());
}

fn cmd_hkdf() {
    println!("=== HKDF-SHA256 ===");
    let ikm = b"input keying material";
    let salt = b"random salt";
    let info = b"application context";
    let okm = hkdf(Some(salt), ikm, info, 32);
    println!("IKM:  {:?}", std::str::from_utf8(ikm).unwrap());
    println!("OKM:  {}", to_hex(&okm));
}

pub(crate) fn cmd_pqc() {
    println!("=== ML-KEM-768 (NIST FIPS 203) ===");
    let (ek, dk) = ml_kem_keygen(&ML_KEM_768);
    println!(
        "Key pair generated: ek = {} bytes, dk = {} bytes.",
        ek.0.len(),
        dk.0.len()
    );
    let (ct, ss_enc) = ml_kem_encaps(&ML_KEM_768, &ek).expect("fresh key is valid");
    println!(
        "Encapsulated ({} bytes). Shared secret (encaps): {}",
        ct.len(),
        to_hex(&ss_enc)
    );
    let ss_dec = ml_kem_decaps(&ML_KEM_768, &dk, &ct).expect("well-formed ciphertext");
    println!(
        "Decapsulated.             Shared secret (decaps): {}",
        to_hex(&ss_dec)
    );
    println!("Secrets match: {}", ss_enc == ss_dec);

    println!("\n=== ML-DSA-65 (NIST FIPS 204) ===");
    let mut seed = [0u8; 32];
    crypto_lib::utils::random::random_bytes(&mut seed);
    let (pk, sk) = ml_dsa_65_keygen(&seed);
    let msg = b"post-quantum signatures";
    let sig = ml_dsa_65_sign(&sk, msg, &[0u8; 32]);
    println!(
        "Signed {} bytes -> {} byte signature.",
        msg.len(),
        sig.len()
    );
    println!("Verify: {}", ml_dsa_65_verify(&pk, msg, &sig));

    println!("\n=== SQIsign (toy p = 431 educational demo) ===");
    let (spk, ssk) = sqisign_keygen();
    println!(
        "Public key: j(E_A) = {:?} on the supersingular 2-isogeny graph.",
        spk.j
    );
    let ssig = sqisign_sign(&spk, &ssk, msg);
    println!(
        "Signature: commitment j(E_1) = {:?}, response path of {} steps.",
        ssig.commitment,
        ssig.response.len() - 1
    );
    println!("Verify: {}", sqisign_verify(&spk, msg, &ssig));

    println!("\n=== NIST on-ramp additional signatures (toy educational params) ===");
    cmd_pqc_onramp(msg);

    println!("\n=== Kyber KEM (pre-standard toy, kept for comparison) ===");
    let sk = kyber_keygen();
    println!("Key pair generated.");

    let (ct, ss_enc) = kyber_encapsulate(&sk.public);
    println!("Encapsulated. Shared secret (encaps): {}", to_hex(&ss_enc));

    let ss_dec = kyber_decapsulate(&sk, &ct);
    println!("Decapsulated. Shared secret (decaps): {}", to_hex(&ss_dec));
    println!("Secrets match: {}", ss_enc == ss_dec);
}

/// Sign + verify `msg` under each NIST additional-signatures (on-ramp)
/// round-3 candidate, at this library's toy educational parameters.
fn cmd_pqc_onramp(msg: &[u8]) {
    use crypto_lib::pqc::{
        faest_keygen, faest_sign, faest_verify, fn_dsa_keygen, fn_dsa_sign, fn_dsa_verify,
        hawk_keygen, hawk_sign, hawk_verify, mayo_keygen, mayo_sign, mayo_verify, mqom_keygen,
        mqom_sign, mqom_verify, qr_uov_keygen, qr_uov_sign, qr_uov_verify, sdith_keygen,
        sdith_sign, sdith_verify, snova_keygen, snova_sign, snova_verify, uov_keygen, uov_sign,
        uov_verify,
    };

    macro_rules! demo {
        ($name:expr, $family:expr, $ok:expr) => {
            println!("  {:<8} ({:<16}) sign+verify: {}", $name, $family, $ok);
        };
    }

    let (pk, sk) = uov_keygen();
    demo!(
        "UOV",
        "multivariate",
        uov_verify(&pk, msg, &uov_sign(&sk, msg))
    );
    let (pk, sk) = mayo_keygen();
    demo!(
        "MAYO",
        "multivariate",
        mayo_verify(&pk, msg, &mayo_sign(&sk, msg))
    );
    let (pk, sk) = qr_uov_keygen();
    demo!(
        "QR-UOV",
        "multivariate",
        qr_uov_verify(&pk, msg, &qr_uov_sign(&sk, msg))
    );
    let (pk, sk) = snova_keygen();
    demo!(
        "SNOVA",
        "multivariate",
        snova_verify(&pk, msg, &snova_sign(&sk, msg))
    );
    let (pk, sk) = hawk_keygen();
    demo!(
        "HAWK",
        "lattice (LIP)",
        hawk_verify(&pk, msg, &hawk_sign(&sk, msg))
    );
    let (pk, sk) = fn_dsa_keygen();
    demo!(
        "FN-DSA",
        "lattice (NTRU)",
        fn_dsa_verify(&pk, msg, &fn_dsa_sign(&sk, msg))
    );
    let (pk, sk) = sdith_keygen();
    demo!(
        "SDitH",
        "MPC-in-the-head",
        sdith_verify(&pk, msg, &sdith_sign(&pk, &sk, msg))
    );
    let (pk, sk) = mqom_keygen();
    demo!(
        "MQOM",
        "MPC-in-the-head",
        mqom_verify(&pk, msg, &mqom_sign(&pk, &sk, msg))
    );
    let (pk, sk) = faest_keygen();
    demo!(
        "FAEST",
        "MPC-in-the-head",
        faest_verify(&pk, msg, &faest_sign(&pk, &sk, msg))
    );
}

fn cmd_demo() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  Comprehensive Cryptography Library Demo");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Hashes
    let msg = b"The quick brown fox jumps over the lazy dog";
    println!("[Hash functions]");
    println!("  SHA-256:  {}", to_hex(&sha256(msg)));
    println!("  SHA3-256: {}", to_hex(&sha3_256(msg)));
    println!("  BLAKE3:   {}", to_hex(&blake3_hash(msg)));

    // ECC
    println!("\n[ECC — secp256k1]");
    let curve = CurveParams::secp256k1();
    let kp = EccKeyPair::generate(&curve);
    let sig = sign(msg, &kp.private, &curve);
    println!("  Sign + verify: {}", verify(msg, &kp.public, &sig, &curve));
    let bob = EccKeyPair::generate(&curve);
    let ss = ecdh(&kp.private, &bob.public, &curve).unwrap();
    println!("  ECDH secret:   {}", to_hex(&ss));

    // AES
    println!("\n[AES-256-GCM]");
    let aes_key = AesKey::Aes256([0x13u8; 32]);
    let nonce = [0u8; 12];
    let ct = aes_gcm_encrypt(msg, &aes_key, &nonce, b"");
    let ok = aes_gcm_decrypt(&ct, &aes_key, &nonce, b"").is_ok();
    println!("  Encrypt + decrypt: {ok}");

    // ChaCha20
    println!("\n[ChaCha20-Poly1305]");
    let cha_key = [0x37u8; 32];
    let ct2 = chacha20_poly1305_encrypt(msg, &cha_key, &nonce, b"");
    let ok2 = chacha20_poly1305_decrypt(&ct2, &cha_key, &nonce, b"").is_ok();
    println!("  Encrypt + decrypt: {ok2}");

    // HKDF
    println!("\n[HKDF-SHA256]");
    let okm = hkdf(Some(b"salt"), b"secret", b"ctx", 32);
    println!("  Derived key: {}", to_hex(&okm));

    // Kyber
    println!("\n[Kyber KEM (simplified)]");
    let sk = kyber_keygen();
    let (ct3, ss3_enc) = kyber_encapsulate(&sk.public);
    let ss3_dec = kyber_decapsulate(&sk, &ct3);
    println!("  KEM secrets match: {}", ss3_enc == ss3_dec);

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

// ── Visual subcommands ────────────────────────────────────────────────────────

fn cmd_visual(op: VisualOp) {
    use crypto_lib::ecc::visualize as ecc_v;
    use crypto_lib::hash::visualize as hash_v;
    use crypto_lib::symmetric::visualize as sym_v;
    match op {
        VisualOp::Symmetric { target } => match target.as_str() {
            "ecb-penguin" => println!("{}", sym_v::demo_ecb_penguin(32, 16)),
            "modes" => println!("{}", sym_v::demo_mode_dataflows()),
            "aes-trace" => {
                println!(
                    "{}",
                    sym_v::demo_aes_state_trace(b"YELLOW SUBMARINE", &[0x42u8; 16])
                )
            }
            "chacha" => println!("{}", sym_v::demo_chacha20_state()),
            "avalanche" => {
                for nr in [1usize, 2, 3, 5, 10] {
                    println!("{}", sym_v::demo_avalanche_matrix(nr));
                }
            }
            "all" => println!("{}", sym_v::run_all_symmetric_demos()),
            other => {
                eprintln!(
                    "error: unknown symmetric target '{}'; try one of: ecb-penguin, modes, aes-trace, chacha, avalanche, all",
                    other
                );
                std::process::exit(1);
            }
        },
        VisualOp::Hash { target } => match target.as_str() {
            "md" => println!("{}", hash_v::demo_merkle_damgard_diagram()),
            "sponge" => println!("{}", hash_v::demo_sponge_diagram()),
            "blake3" => println!("{}", hash_v::demo_blake3_tree_diagram()),
            "sha256-trace" => println!("{}", hash_v::demo_sha256_round_trace(b"abc")),
            "avalanche" => println!("{}", hash_v::demo_hash_avalanche()),
            "all" => println!("{}", hash_v::run_all_hash_demos()),
            other => {
                eprintln!(
                    "error: unknown hash target '{}'; try: md, sponge, blake3, sha256-trace, avalanche, all",
                    other
                );
                std::process::exit(1);
            }
        },
        VisualOp::Ecc { target } => match target.as_str() {
            "curve" => println!("{}", ecc_v::demo_toy_curve_scatter(31, 1, 1)),
            "ops" => println!("{}", ecc_v::demo_point_operations()),
            "ladder" => println!("{}", ecc_v::demo_scalar_mul_ladder(13)),
            "ecdsa" => println!("{}", ecc_v::demo_ecdsa_flow()),
            "all" => println!("{}", ecc_v::run_all_ecc_demos()),
            other => {
                eprintln!(
                    "error: unknown ecc target '{}'; try: curve, ops, ladder, ecdsa, all",
                    other
                );
                std::process::exit(1);
            }
        },
        VisualOp::All => {
            println!("{}", sym_v::run_all_symmetric_demos());
            println!("\n\n========================================\n\n");
            println!("{}", hash_v::run_all_hash_demos());
            println!("\n\n========================================\n\n");
            println!("{}", ecc_v::run_all_ecc_demos());
        }
    }
}

#[cfg(test)]
mod hash_cli_tests {
    use super::{cmd_hash, load_hash_input};

    #[test]
    fn sha_variants_match_independent_abc_vectors() {
        let vectors = [
            ("sha224", "23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7"),
            ("sha256", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
            ("sha384", "cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7"),
            ("sha512", "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"),
            ("sha512-224", "4634270f707b6a54daae7530460842e20e37ed265ceee9a43e8924aa"),
            ("sha512-256", "53048e2681941ef99b2e29b76b4c7dabe4c2d0c634fc6d46e0e2f13107e7af23"),
            ("sha3-224", "e642824c3f8cf24ad09234ee7d3c766fc9a3a5168d0c94ad73b46fdf"),
            ("sha3", "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"),
            ("sha3-384", "ec01498288516fc926459f58e2c6ad8df9b473cb0fc08c2596da7cf0e49be4b298d88cea927ac7f539f1edf228376d25"),
            ("sha3-512", "b751850b1a57168a5693cd924b6b096e08f621827444f70d884f5d0240d2712e10e116e9192af3c91a7ec57647e3934057340b4cf408d5a56592f8274eec53f0"),
        ];
        for (name, expected) in vectors {
            assert_eq!(cmd_hash(name, b"abc", None).unwrap(), expected, "{name}");
        }
        assert_eq!(
            cmd_hash("shake128", b"abc", Some(16)).unwrap(),
            "5881092dd818bf5cf8a3ddb793fbcba7"
        );
        assert_eq!(
            cmd_hash("shake256", b"abc", Some(16)).unwrap(),
            "483366601360a8771c6863080cc4114d"
        );
    }

    #[test]
    fn output_length_and_name_errors_are_explicit() {
        assert!(cmd_hash("shake128", b"abc", None).is_err());
        assert!(cmd_hash("shake128", b"abc", Some(0)).is_err());
        assert!(cmd_hash("sha256", b"abc", Some(32)).is_err());
        assert!(cmd_hash("unknown", b"abc", None).is_err());
    }

    #[test]
    fn binary_input_modes_hash_exact_bytes() {
        let expected = cmd_hash("sha3-256", &[0, 255, 1, 128], None).unwrap();
        let hex = load_hash_input(None, Some("00ff0180"), None).unwrap();
        assert_eq!(cmd_hash("sha3-256", &hex, None).unwrap(), expected);
        let path = std::env::temp_dir().join(format!(
            "crypto-hash-input-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("unnamed")
        ));
        std::fs::write(&path, &hex).unwrap();
        let file = load_hash_input(None, None, Some(&path)).unwrap();
        std::fs::remove_file(&path).unwrap();
        assert_eq!(cmd_hash("sha3-256", &file, None).unwrap(), expected);
        assert!(load_hash_input(None, None, None).is_err());
        assert!(load_hash_input(Some("abc"), Some("616263"), None).is_err());
        assert!(load_hash_input(None, Some("0"), None).is_err());
    }
}
