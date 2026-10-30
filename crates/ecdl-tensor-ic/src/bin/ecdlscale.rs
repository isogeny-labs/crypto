//! Command ecdlscale prints the shape of the low-degree gadget networks for
//! field multiplication, complete point addition, and the per-point validity
//! and membership checks as the field size grows:
//! edges per bit should level off (O(b) edges) and max degree / b^(3/4)
//! should stay bounded. Networks are built but never evaluated.
//!
//! Flags (Go `flag` syntax: `-bits 64,256` or `-bits=64,256`):
//!   -bits LIST  comma-separated field sizes, default "64,256,1024,4096,16384"

use num_bigint::BigUint;

use ecdl_tensor_ic::{gadget_shapes, Arith, NetworkStats};

const DEFAULT_BITS: &str = "64,256,1024,4096,16384";

/// Marker error text for -h / -help (Go: flag.ErrHelp).
const HELP: &str = "flag: help requested";

fn usage() {
    eprintln!("Usage of ecdlscale:");
    eprintln!("  -bits string");
    eprintln!(
        "    \tcomma-separated field sizes (default \"{}\")",
        DEFAULT_BITS
    );
}

/// Hand-written parser: returns the value of -bits (the default string when
/// absent). `Err(message)` on an unknown flag.
fn parse_bits_flag(args: &[String]) -> Result<String, String> {
    let mut bits = DEFAULT_BITS.to_string();
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
            "bits" => {
                bits = match inline {
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
            }
            "h" | "help" => return Err(HELP.to_string()),
            _ => return Err(format!("flag provided but not defined: -{}", name)),
        }
    }
    Ok(bits)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let sizes = match parse_bits_flag(&args) {
        Ok(s) => s,
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
    println!(
        "{:>7} {:>5} {:>4} {:>4} | {:>8} {:>8} {:>8} | {:>7} {:>9}",
        "b", "t", "n", "L", "mul e/b", "add e/b", "point e/b", "maxdeg", "deg/b^.75"
    );
    for s in sizes.split(',') {
        let b: usize = match s.trim().parse::<i64>() {
            Ok(v) if v >= 4 => v as usize,
            _ => {
                println!("bad size: {}", s);
                continue;
            }
        };
        let p: BigUint = (BigUint::from(1u32) << (b - 1)) + 1u32;
        let ar = Arith::new(&p);
        let (mul, add, point) = gadget_shapes(&ar);
        let mut maxdeg = 0usize;
        for st in [mul, add, point] {
            if st.max_degree > maxdeg {
                maxdeg = st.max_degree;
            }
            if !st.is_graph {
                println!("warning: not a graph at b = {}", b);
            }
        }
        let per = |st: NetworkStats| st.edges as f64 / b as f64;
        println!(
            "{:>7} {:>5} {:>4} {:>4} | {:>8.1} {:>8.1} {:>8.1} | {:>7} {:>9.1}",
            b,
            ar.t,
            ar.n,
            ar.l,
            per(mul),
            per(add),
            per(point),
            maxdeg,
            maxdeg as f64 / (b as f64).powf(0.75)
        );
    }
}
