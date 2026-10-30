//! Command ecdldemo generates a random toy prime-order curve and recovers a
//! random discrete logarithm by index calculus with tensor-contraction
//! relation finding.
//!
//! Flags (Go `flag` syntax: `-bits 14`, `-bits=14`, `--bits 14`; booleans are
//! bare `-q` / `-gadgets` or `-q=true`):
//!   -bits N     field size b in bits (4..26), default 14
//!   -m N        number of factor bases, default 2
//!   -seed N     random seed, default current time in nanoseconds
//!   -q          suppress progress output
//!   -gadgets    use low-degree arithmetic gadgets (slow; keep -bits small)
//!   -window K   constrain x < p>>K in every factor base (0 = off; works with or without -gadgets)
//!   -mitm       find relations by meeting in the middle (~sqrt(2r) additions per attempt)
//!   -threads N  relation-search workers with -mitm (0 = one per core, 1 = sequential)

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rand::rngs::StdRng;
use rand::SeedableRng;

use ecdl_tensor_ic::field::rand_below;
use ecdl_tensor_ic::{random_prime_order_curve, DFSCounter, Solver, SparseContractor};

/// Parsed command line (Go: the four flag variables).
#[derive(Debug, Clone)]
struct Flags {
    bits: i64,
    m: i64,
    seed: i64,
    quiet: bool,
    gadgets: bool,
    window: i64,
    mitm: bool,
    threads: i64,
}

/// Marker error text for -h / -help (Go: flag.ErrHelp).
const HELP: &str = "flag: help requested";

fn usage() {
    eprintln!("Usage of ecdldemo:");
    eprintln!("  -bits int");
    eprintln!("    \tfield size b in bits (4..26) (default 14)");
    eprintln!("  -gadgets");
    eprintln!("    \tuse low-degree arithmetic gadgets (slow; keep -bits small)");
    eprintln!("  -mitm");
    eprintln!("    \tfind relations by meeting in the middle (~sqrt(2r) additions per attempt)");
    eprintln!("  -threads int");
    eprintln!("    \trelation-search workers with -mitm (0 = one per core, 1 = sequential)");
    eprintln!("  -m int");
    eprintln!("    \tnumber of factor bases (default 2)");
    eprintln!("  -q\tsuppress progress output");
    eprintln!("  -window int");
    eprintln!(
        "    \tconstrain x < p>>k in every factor base (0 = off; works with or without -gadgets)"
    );
    eprintln!("  -seed int");
    eprintln!("    \trandom seed (default <current time in ns>)");
}

/// Go: strconv.ParseInt(s, 0, 64) (base prefixes and underscores allowed).
fn parse_int(s: &str) -> Option<i64> {
    let (neg, body) = match s.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, s.strip_prefix('+').unwrap_or(s)),
    };
    if body.is_empty() {
        return None;
    }
    let lower = body.to_ascii_lowercase();
    let (radix, digits) = if let Some(r) = lower.strip_prefix("0x") {
        (16, r.to_string())
    } else if let Some(r) = lower.strip_prefix("0b") {
        (2, r.to_string())
    } else if let Some(r) = lower.strip_prefix("0o") {
        (8, r.to_string())
    } else if lower.len() > 1 && lower.starts_with('0') {
        (8, lower[1..].to_string())
    } else {
        (10, lower.clone())
    };
    let digits = if radix == 10 && !lower.contains('_') {
        digits
    } else {
        digits.replace('_', "")
    };
    let v = i128::from_str_radix(&digits, radix).ok()?;
    let v = if neg { -v } else { v };
    i64::try_from(v).ok()
}

/// Go: strconv.ParseBool.
fn parse_bool(s: &str) -> Option<bool> {
    match s {
        "1" | "t" | "T" | "TRUE" | "true" | "True" => Some(true),
        "0" | "f" | "F" | "FALSE" | "false" | "False" => Some(false),
        _ => None,
    }
}

/// Hand-written parser for the flags above. Returns `Err(message)` on an
/// unknown flag or bad value (the caller prints it and exits with status 2,
/// like Go's flag package).
fn parse_flags(args: &[String]) -> Result<Flags, String> {
    let seed_default = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0);
    let mut f = Flags {
        bits: 14,
        m: 2,
        seed: seed_default,
        quiet: false,
        gadgets: false,
        window: 0,
        mitm: false,
        threads: 0,
    };
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        // Go stops at the first non-flag argument ("-" alone is one) and
        // consumes a lone "--".
        if arg.len() < 2 || !arg.starts_with('-') {
            break;
        }
        let mut name = &arg[1..];
        if let Some(rest) = name.strip_prefix('-') {
            if rest.is_empty() {
                break;
            }
            name = rest;
        }
        if name.starts_with('-') || name.starts_with('=') {
            return Err(format!("bad flag syntax: {}", arg));
        }
        i += 1;
        let (name, inline) = match name.split_once('=') {
            Some((n, v)) => (n, Some(v.to_string())),
            None => (name, None),
        };
        match name {
            "q" | "gadgets" | "mitm" => {
                let v = match inline {
                    None => true,
                    Some(v) => parse_bool(&v).ok_or_else(|| {
                        format!("invalid boolean value {:?} for -{}: parse error", v, name)
                    })?,
                };
                match name {
                    "q" => f.quiet = v,
                    "gadgets" => f.gadgets = v,
                    _ => f.mitm = v,
                }
            }
            "bits" | "m" | "seed" | "window" | "threads" => {
                let v = match inline {
                    Some(v) => v,
                    None => {
                        if i < args.len() {
                            i += 1;
                            args[i - 1].clone()
                        } else {
                            return Err(format!("flag needs an argument: -{}", name));
                        }
                    }
                };
                let n = parse_int(&v).ok_or_else(|| {
                    format!("invalid value {:?} for flag -{}: parse error", v, name)
                })?;
                match name {
                    "bits" => f.bits = n,
                    "m" => f.m = n,
                    "window" => f.window = n,
                    "threads" => f.threads = n,
                    _ => f.seed = n,
                }
            }
            "h" | "help" => return Err(HELP.to_string()),
            _ => return Err(format!("flag provided but not defined: -{}", name)),
        }
    }
    Ok(f)
}

/// Formats a Duration like Go's Duration.String (used after rounding to ms).
fn go_duration(d: Duration) -> String {
    let total_ms = d.as_millis();
    if total_ms == 0 {
        return "0s".to_string();
    }
    if total_ms < 1000 {
        return format!("{}ms", total_ms);
    }
    let secs_total = total_ms / 1000;
    let ms = total_ms % 1000;
    let (h, m, s) = (secs_total / 3600, (secs_total / 60) % 60, secs_total % 60);
    let mut sec = format!("{}", s);
    if ms != 0 {
        let frac = format!("{:03}", ms);
        sec = format!("{}.{}", s, frac.trim_end_matches('0'));
    }
    if h > 0 {
        format!("{}h{}m{}s", h, m, sec)
    } else if m > 0 {
        format!("{}m{}s", m, sec)
    } else {
        format!("{}s", sec)
    }
}

/// Go: Duration.Round(time.Millisecond) (half away from zero).
fn round_ms(d: Duration) -> Duration {
    Duration::from_millis(((d.as_nanos() + 500_000) / 1_000_000) as u64)
}

/// Go: log.Fatal (without the timestamp prefix).
fn fatal(msg: impl std::fmt::Display) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flags = match parse_flags(&args) {
        Ok(f) => f,
        Err(e) if e == HELP => {
            usage();
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("{}", e);
            usage();
            std::process::exit(2);
        }
    };

    let mut rng = StdRng::seed_from_u64(flags.seed as u64);
    let bits = usize::try_from(flags.bits).unwrap_or(usize::MAX);
    let c = random_prime_order_curve(&mut rng, bits).unwrap_or_else(|e| fatal(e));
    let xi = rand_below(&mut rng, c.order);
    let q = c.mul(xi, c.g);
    println!(
        "curve  y^2 = x^3 + {} x + {} over F_{}, prime order r = {}",
        c.a, c.b, c.p, c.order
    );
    println!("P = ({}, {}), Q = ({}, {})", c.g.x, c.g.y, q.x, q.y);

    let m = usize::try_from(flags.m).unwrap_or(0);
    let mut s = Solver::new(c, m, Box::new(SparseContractor::default()), rng);
    if flags.gadgets {
        s.gadgets = true;
        s.counter = Box::new(DFSCounter);
    }
    s.window_bits = usize::try_from(flags.window).unwrap_or(0);
    s.mitm = flags.mitm;
    s.threads = usize::try_from(flags.threads).unwrap_or(0);
    if !flags.quiet {
        s.logf = Some(Box::new(|msg: &str| eprintln!("{}", msg)));
    }
    let (got, st) = s.solve(q).unwrap_or_else(|e| fatal(e));
    if st.network.edges == 0 {
        println!("network: meet-in-the-middle (no contraction)");
    } else {
        println!(
            "network: {} edges, {} vertices, max degree {}, graph={}",
            st.network.edges, st.network.vertices, st.network.max_degree, st.network.is_graph
        );
    }
    println!(
        "N = {}, relations = {}, attempts = {}, restarts = {}, time = {}",
        st.factor_base_size,
        st.relations,
        st.attempts,
        st.restarts,
        go_duration(round_ms(st.elapsed))
    );
    println!(
        "recovered xi = {} (secret {}) — verified [xi]P == Q: {}",
        got,
        xi,
        c.mul(got, c.g) == q
    );
}
