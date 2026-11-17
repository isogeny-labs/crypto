//! Command ecdlscale prints the shape of the low-degree gadget networks for
//! field multiplication, complete point addition, and the per-point validity
//! and membership checks as the field size grows:
//! edges per bit should level off (O(b) edges) and max degree / b^(3/4)
//! should stay bounded. Networks are built but never evaluated.
//!
//! Flags (Go `flag` syntax: `-bits 64,256` or `-bits=64,256`):
//!   -bits LIST  comma-separated field sizes, default "64,256,1024,4096,16384"

use ecdl_tensor_ic::scale::ScaleRow;

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
    println!("{}", ScaleRow::header());
    for s in sizes.split(',') {
        let row = match s.trim().parse::<i64>() {
            Ok(v) if v >= 4 => ScaleRow::new(v as usize),
            _ => None,
        };
        let Some(row) = row else {
            println!("bad size: {}", s);
            continue;
        };
        for st in [&row.mul, &row.add, &row.point] {
            if !st.is_graph {
                println!("warning: not a graph at b = {}", row.b);
            }
        }
        println!("{}", row.line());
    }
}
