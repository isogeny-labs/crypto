//! Dynamic constant-time audit of the `crypto` crate's secret-handling paths (2026-11-02).
//!
//! Two host-independent instruments, both driven by Valgrind client requests issued from this binary:
//!
//! * `taint <target>` — run under Memcheck. The secret bytes are marked *undefined*
//!   (`VALGRIND_MAKE_MEM_UNDEFINED`); Memcheck then reports every conditional jump/move or memory address
//!   that depends on them. One process per target, `CALLS` calls with independent secrets.
//! * `ir <target>` — run under Callgrind with `--collect-atstart=no`. Each call is bracketed by
//!   `CALLGRIND_TOGGLE_COLLECT` and followed by `CALLGRIND_DUMP_STATS_AT(label)`, so every dump holds the exact
//!   instruction count (Ir) of one call. Classes: `aa` (one fixed secret repeated), `random`, `short` (scalar
//!   below 2^32: the bit-length class of the 2026-10-22 leak) and `lowhw` (Hamming weight 8).
//! * `analyze <raw dir> <out dir>` — parse the Memcheck XML and Callgrind dumps into `rows.jsonl`,
//!   `summary.json` and `fig_ir_spread.svg`, applying the decision rule frozen in PROTOCOL.md.
//!
//! Secrets are derived from a fixed seed (PROTOCOL.md §4); nothing derived from a secret is printed.
use std::collections::BTreeMap;
use std::fs;
use std::hint::black_box;
use std::path::Path;

use crypto::ecc::curve::CurveParams;
use crypto::ecc::keys::EccPrivateKey;
use num_bigint::BigUint;
use rand::{rngs::StdRng, Rng, SeedableRng};

// ------------------------------------------------------------------ Valgrind client requests (x86_64)

const fn tool_base(a: u8, b: u8) -> u64 {
    ((a as u64) << 24) | ((b as u64) << 16)
}
const MC_MAKE_MEM_UNDEFINED: u64 = tool_base(b'M', b'C') + 1;
const MC_MAKE_MEM_DEFINED: u64 = tool_base(b'M', b'C') + 2;
const CG_DUMP_STATS_AT: u64 = tool_base(b'C', b'T') + 3;
const CG_TOGGLE_COLLECT: u64 = tool_base(b'C', b'T') + 2;
const RUNNING_ON_VALGRIND: u64 = 0x1001;

/// The documented client-request sequence from valgrind.h. Natively it is a no-op returning `default`.
#[inline(always)]
fn vg(default: u64, req: u64, a1: u64, a2: u64) -> u64 {
    let args: [u64; 6] = [req, a1, a2, 0, 0, 0];
    let mut res = default;
    unsafe {
        core::arch::asm!(
            "rol rdi, 3", "rol rdi, 13", "rol rdi, 61", "rol rdi, 51", "xchg rbx, rbx",
            inout("rdx") res, in("rax") args.as_ptr(), inout("rdi") 0u64 => _,
            options(nostack)
        );
    }
    res
}
fn undefine(bytes: &mut [u8]) {
    vg(0, MC_MAKE_MEM_UNDEFINED, bytes.as_mut_ptr() as u64, bytes.len() as u64);
}
fn define(bytes: &mut [u8]) {
    vg(0, MC_MAKE_MEM_DEFINED, bytes.as_mut_ptr() as u64, bytes.len() as u64);
}

// ------------------------------------------------------------------ targets

const TARGETS: &[(&str, &str)] = &[
    ("ctrl_pos_branch", "control: branch + table index on a secret byte (must be flagged)"),
    ("ctrl_neg_masked", "control: fixed-trip masked u64 arithmetic, no secret branch or index (must be clean)"),
    ("x25519", "ecc::x25519::x25519, secret scalar, u = 9"),
    ("smul_p256", "ecc::ct::scalar_mul_secret on P-256 (dedicated projective ladder)"),
    ("smul_secp256k1", "ecc::ct::scalar_mul_secret on secp256k1 (dedicated projective ladder)"),
    ("smul_sm2_generic", "ecc::ct::scalar_mul_secret on SM2 (generic affine ladder, fixed 2026-10-22)"),
    ("inv_prime_ct", "utils::mod_inverse_prime_ct(k, n_P-256): ECDSA's k^-1"),
    ("inv_euclid", "utils::mod_inverse(1 + d, n_SM2): SM2 signing's private-key inverse"),
    ("ed25519_sign", "ecc::ed25519::ed25519_sign, secret seed, fixed message"),
    ("ecdsa_p256_sign", "ecc::ecdsa::sign on P-256, secret d, fixed message (scalar blinding draws OsRng)"),
    ("sm2_sign", "ecc::sm2::sign, secret d (nonce drawn from OsRng)"),
];

const SEED: u64 = 0x5ca1_ab1e_c7d1_1102; // PROTOCOL.md §4
const MSG: &[u8] = b"ct-dynamic-audit-20261102 fixed message";

struct Ctx {
    p256: CurveParams,
    secp: CurveParams,
    sm2: CurveParams,
    table: [u64; 256],
}

/// Secret material as raw bytes (big-endian scalar or seed), so it can be marked undefined before use.
fn secret_bytes(target: &str, class: &str, rng: &mut StdRng, ctx: &Ctx) -> Vec<u8> {
    let n = match target {
        "smul_secp256k1" => &ctx.secp.n,
        "smul_sm2_generic" | "inv_euclid" | "sm2_sign" => &ctx.sm2.n,
        _ => &ctx.p256.n,
    };
    let mut b = [0u8; 32];
    match class {
        "short" => rng.fill(&mut b[28..]),
        "lowhw" => {
            let mut set = 0;
            while set < 8 {
                let i = rng.gen_range(8..256usize);
                if b[31 - i / 8] & (1 << (i % 8)) == 0 {
                    b[31 - i / 8] |= 1 << (i % 8);
                    set += 1;
                }
            }
        }
        _ => rng.fill(&mut b),
    }
    let raw_targets = ["ctrl_pos_branch", "ctrl_neg_masked", "x25519", "ed25519_sign"];
    if raw_targets.contains(&target) {
        return b.to_vec();
    }
    // scalars: reduce into [1, n-1] deterministically (never zero)
    let k = BigUint::from_bytes_be(&b) % (n - 1u32) + 1u32;
    let mut out = k.to_bytes_be();
    while out.len() < 32 {
        out.insert(0, 0);
    }
    out
}

#[inline(never)]
fn ctrl_pos_branch(s: &[u8], table: &[u64; 256]) -> u64 {
    let mut acc = 0u64;
    if s[31] & 1 == 1 {
        for i in 0..64u64 {
            acc = acc.wrapping_mul(6364136223846793005).wrapping_add(i);
        }
    }
    acc ^ table[s[0] as usize]
}

#[inline(never)]
fn ctrl_neg_masked(s: &[u8]) -> u64 {
    let mut acc = 0u64;
    for &byte in s {
        let bit = (byte & 1) as u64;
        let mask = bit.wrapping_neg();
        acc = acc.rotate_left(7) ^ ((mask & 0x9e37_79b9_7f4a_7c15) | (!mask & 0x2545_f491_4f6c_dd1d)) ^ (byte as u64);
    }
    acc
}

/// One call of `target` on the secret bytes. The result goes to `black_box` and is never inspected.
#[inline(never)]
fn call(target: &str, s: &[u8], ctx: &Ctx) {
    let big = || BigUint::from_bytes_be(s);
    match target {
        "ctrl_pos_branch" => { black_box(ctrl_pos_branch(s, &ctx.table)); }
        "ctrl_neg_masked" => { black_box(ctrl_neg_masked(s)); }
        "x25519" => {
            let mut k = [0u8; 32];
            k.copy_from_slice(s);
            let mut u = [0u8; 32];
            u[0] = 9;
            black_box(crypto::ecc::x25519::x25519(&k, &u));
        }
        "smul_p256" => { black_box(crypto::ecc::ct::scalar_mul_secret(&ctx.p256.generator(), &big(), &ctx.p256)); }
        "smul_secp256k1" => { black_box(crypto::ecc::ct::scalar_mul_secret(&ctx.secp.generator(), &big(), &ctx.secp)); }
        "smul_sm2_generic" => { black_box(crypto::ecc::ct::scalar_mul_secret(&ctx.sm2.generator(), &big(), &ctx.sm2)); }
        "inv_prime_ct" => { black_box(crypto::utils::mod_inverse_prime_ct(&big(), &ctx.p256.n)); }
        "inv_euclid" => {
            let one_plus_d = (BigUint::from(1u32) + big()) % &ctx.sm2.n;
            black_box(crypto::utils::mod_inverse(&one_plus_d, &ctx.sm2.n));
        }
        "ed25519_sign" => {
            let mut seed = [0u8; 32];
            seed.copy_from_slice(s);
            black_box(crypto::ecc::ed25519::ed25519_sign(MSG, &seed));
        }
        "ecdsa_p256_sign" => {
            let key = EccPrivateKey { scalar: big(), curve_name: ctx.p256.name.to_string() };
            black_box(crypto::ecc::ecdsa::sign(MSG, &key, &ctx.p256));
        }
        "sm2_sign" => {
            // The public key is derived outside the tainted region (see `taint`), so only signing is inspected.
            let d = big();
            let pa = crypto::ecc::ct::scalar_mul_secret(&ctx.sm2.generator(), &d, &ctx.sm2);
            black_box(crypto::ecc::sm2::sign(MSG, &d, &pa, b"ALICE123@YAHOO.COM", &ctx.sm2));
        }
        other => panic!("unknown target {other}"),
    }
}

fn ctx() -> Ctx {
    let mut table = [0u64; 256];
    for (i, t) in table.iter_mut().enumerate() {
        *t = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }
    Ctx { p256: CurveParams::p256(), secp: CurveParams::secp256k1(), sm2: CurveParams::sm2(), table }
}

fn target_seed(target: &str, class: &str) -> u64 {
    // FNV-1a over "target/class", mixed with the frozen seed: per-arm streams fixed before any data.
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in target.bytes().chain(*b"/").chain(class.bytes()) {
        h = (h ^ b as u64).wrapping_mul(0x100_0000_01b3);
    }
    h ^ SEED
}

const CALLS_TAINT: usize = 4;
const CLASSES: &[(&str, usize)] = &[("aa", 16), ("random", 32), ("short", 16), ("lowhw", 16)];

fn run_taint(target: &str) {
    let ctx = ctx();
    let mut rng = StdRng::seed_from_u64(target_seed(target, "taint"));
    for _ in 0..CALLS_TAINT {
        let mut s = secret_bytes(target, "random", &mut rng, &ctx);
        if target == "sm2_sign" {
            // derive the public key while the secret is still defined; only `sign` is inspected
            let d = BigUint::from_bytes_be(&s);
            let pa = crypto::ecc::ct::scalar_mul_secret(&ctx.sm2.generator(), &d, &ctx.sm2);
            let mut sec = s.clone();
            undefine(&mut sec);
            let d_t = BigUint::from_bytes_be(&sec);
            black_box(crypto::ecc::sm2::sign(MSG, &d_t, &pa, b"ALICE123@YAHOO.COM", &ctx.sm2));
            define(&mut sec);
            continue;
        }
        undefine(&mut s);
        call(target, &s, &ctx);
        define(&mut s);
    }
    eprintln!("taint {target}: {CALLS_TAINT} calls (running on valgrind: {})", vg(0, RUNNING_ON_VALGRIND, 0, 0));
}

fn run_ir(target: &str) {
    let ctx = ctx();
    call(target, &secret_bytes(target, "random", &mut StdRng::seed_from_u64(1), &ctx), &ctx); // warm-up, not collected
    for &(class, n) in CLASSES {
        let mut rng = StdRng::seed_from_u64(target_seed(target, class));
        let fixed = secret_bytes(target, "random", &mut rng, &ctx);
        for i in 0..n {
            let s = if class == "aa" { fixed.clone() } else { secret_bytes(target, class, &mut rng, &ctx) };
            vg(0, CG_TOGGLE_COLLECT, 0, 0);
            call(target, &s, &ctx);
            vg(0, CG_TOGGLE_COLLECT, 0, 0);
            let label = format!("{target}|{class}|{i}|{}\0", s.iter().map(|b| b.count_ones()).sum::<u32>());
            vg(0, CG_DUMP_STATS_AT, label.as_ptr() as u64, 0);
        }
    }
    eprintln!("ir {target}: dumped {} calls", CLASSES.iter().map(|c| c.1).sum::<usize>());
}

// ------------------------------------------------------------------ analysis (decision rule: PROTOCOL.md §6)

fn between<'a>(s: &'a str, a: &str, b: &str) -> Option<&'a str> {
    let i = s.find(a)? + a.len();
    let j = s[i..].find(b)? + i;
    Some(&s[i..j])
}

/// Memcheck XML -> list of (kind, first frame inside the crypto crate or harness, file:line).
fn parse_memcheck(xml: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    for err in xml.split("<error>").skip(1) {
        let err = err.split("</error>").next().unwrap_or("");
        let kind = between(err, "<kind>", "</kind>").unwrap_or("?").to_string();
        let mut site = ("?".to_string(), "?".to_string());
        for frame in err.split("<frame>").skip(1) {
            let func = between(frame, "<fn>", "</fn>").unwrap_or("");
            let dir = between(frame, "<dir>", "</dir>").unwrap_or("");
            let file = between(frame, "<file>", "</file>").unwrap_or("");
            let line = between(frame, "<line>", "</line>").unwrap_or("");
            let in_scope = dir.contains("/crypto/src") || dir.contains("ct_dynamic_audit") || dir.contains("num-bigint");
            if site.0 == "?" || (in_scope && !site.1.contains("/crypto/src") && !site.1.contains("ct_dynamic_audit")) {
                let rel = dir.rsplit_once("/crypto/").map(|x| x.1).unwrap_or(dir);
                site = (func.replace("&lt;", "<").replace("&gt;", ">").replace("&amp;", "&"), format!("{rel}/{file}:{line}"));
                if dir.contains("/crypto/src") || dir.contains("ct_dynamic_audit") {
                    break;
                }
            }
        }
        out.push((kind, site.0, site.1));
    }
    out
}

/// Callgrind dump -> (label, Ir).
fn parse_dump(text: &str) -> Option<(String, u64)> {
    let label = text.lines().find_map(|l| l.strip_prefix("desc: Trigger: Client Request: "))?.to_string();
    let ir = text.lines().find_map(|l| l.strip_prefix("totals: ").or_else(|| l.strip_prefix("summary: ")))?;
    Some((label, ir.split_whitespace().next()?.parse().ok()?))
}

fn json_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn analyze(raw: &Path, out: &Path) {
    fs::create_dir_all(out).unwrap();
    let mut rows = String::new();
    let mut taint: BTreeMap<String, Vec<(String, String, String)>> = BTreeMap::new();
    let mut ir: BTreeMap<String, BTreeMap<String, Vec<u64>>> = BTreeMap::new();
    let mut entries: Vec<_> = fs::read_dir(raw).unwrap().flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        let name = p.file_name().unwrap().to_string_lossy().to_string();
        let text = fs::read_to_string(&p).unwrap_or_default();
        if let Some(t) = name.strip_prefix("memcheck-").and_then(|n| n.strip_suffix(".xml")) {
            let errs = parse_memcheck(&text);
            for (kind, func, at) in &errs {
                rows += &format!("{{\"instrument\":\"memcheck\",\"target\":{},\"kind\":{},\"fn\":{},\"at\":{}}}\n",
                                 json_str(t), json_str(kind), json_str(func), json_str(at));
            }
            taint.insert(t.to_string(), errs);
        } else if name.starts_with("callgrind.") {
            if let Some((label, count)) = parse_dump(&text) {
                let parts: Vec<&str> = label.split('|').collect();
                if parts.len() == 4 {
                    rows += &format!("{{\"instrument\":\"callgrind\",\"target\":{},\"class\":{},\"i\":{},\"secret_popcount\":{},\"ir\":{}}}\n",
                                     json_str(parts[0]), json_str(parts[1]), parts[2], parts[3], count);
                    ir.entry(parts[0].into()).or_default().entry(parts[1].into()).or_default().push(count);
                }
            }
        }
    }
    fs::write(out.join("rows.jsonl"), &rows).unwrap();

    let mut summary = Vec::new();
    let mut fig = Vec::new();
    for (t, desc) in TARGETS {
        let errs = taint.get(*t);
        let mut sites: Vec<String> = errs.map(|e| e.iter().map(|x| format!("{} @ {} [{}]", x.1, x.2, x.0)).collect()).unwrap_or_default();
        sites.sort();
        sites.dedup();
        let classes = ir.get(*t);
        let stat = |c: &str| classes.and_then(|m| m.get(c)).map(|v| (*v.iter().min().unwrap(), *v.iter().max().unwrap(), v.len()));
        let aa = stat("aa");
        let others: Vec<u64> = classes.map(|m| m.iter().filter(|(k, _)| k.as_str() != "aa").flat_map(|(_, v)| v.clone()).collect()).unwrap_or_default();
        let (omin, omax) = (others.iter().min().copied(), others.iter().max().copied());
        let ir_verdict = match (aa, omin, omax) {
            (Some((a0, a1, _)), Some(_), Some(_)) if a0 != a1 => format!("not applicable: A/A not deterministic (spread {})", a1 - a0),
            (Some(_), Some(lo), Some(hi)) if lo == hi => "no Ir variation observed across classes".into(),
            (Some(_), Some(lo), Some(hi)) => format!("Ir depends on the secret (spread {} = {:.4}% of min)", hi - lo, 100.0 * (hi - lo) as f64 / lo as f64),
            _ => "missing".into(),
        };
        let taint_verdict = match errs {
            None => "missing".to_string(),
            Some(e) if e.is_empty() => "no secret-dependent branch or address reported".into(),
            Some(_) => format!("{} secret-dependent sites", sites.len()),
        };
        let cls = |c: &str| stat(c).map(|(a, b, n)| format!("{{\"min\":{a},\"max\":{b},\"n\":{n}}}")).unwrap_or("null".into());
        summary.push(format!(
            "  {{\"target\":{},\"description\":{},\"taint_errors\":{},\"taint_sites\":[{}],\"taint_verdict\":{},\"ir\":{{\"aa\":{},\"random\":{},\"short\":{},\"lowhw\":{}}},\"ir_verdict\":{}}}",
            json_str(t), json_str(desc), errs.map(|e| e.len()).unwrap_or(0), sites.iter().map(|s| json_str(s)).collect::<Vec<_>>().join(","),
            json_str(&taint_verdict), cls("aa"), cls("random"), cls("short"), cls("lowhw"), json_str(&ir_verdict)));
        if let (Some(lo), Some(hi)) = (omin, omax) {
            fig.push((t.to_string(), classes.cloned().unwrap_or_default(), lo, hi, sites.len()));
        }
    }
    fs::write(out.join("summary.json"), format!("[\n{}\n]\n", summary.join(",\n"))).unwrap();
    fs::write(out.join("fig_ir_spread.svg"), svg(&fig)).unwrap();
    println!("rows: {} lines; summary: {} targets", rows.lines().count(), summary.len());
}

/// Relative Ir spread per target and class (dot strip), log-ish scale with an explicit zero lane.
fn svg(fig: &[(String, BTreeMap<String, Vec<u64>>, u64, u64, usize)]) -> String {
    let (w, row_h, left, right) = (900.0, 30.0, 190.0, 120.0);
    let h = 70.0 + row_h * fig.len() as f64;
    let colors = [("aa", "#8a939d"), ("random", "#2a78d6"), ("short", "#eb6834"), ("lowhw", "#1baf7a")];
    // x: relative deviation from the target's minimum Ir, in percent, sqrt scale 0..max
    let maxdev = fig.iter().map(|f| 100.0 * (f.3 - f.2) as f64 / f.2 as f64).fold(0.0f64, f64::max).max(0.01);
    let x = |d: f64| left + (d / maxdev).sqrt() * (w - left - right);
    let mut s = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" font-family=\"system-ui,sans-serif\" font-size=\"12\">\n<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>\n");
    s += &format!("<text x=\"{left}\" y=\"20\" font-size=\"14\" fill=\"#1b1f24\">Instruction count per call: deviation from each target's minimum (%, square-root scale)</text>\n");
    for t in [0.0, 0.001, 0.01, 0.1, 1.0, 10.0, 100.0] {
        if t <= maxdev {
            s += &format!("<line x1=\"{0:.1}\" x2=\"{0:.1}\" y1=\"34\" y2=\"{1:.1}\" stroke=\"#eceff3\"/><text x=\"{0:.1}\" y=\"{2:.1}\" text-anchor=\"middle\" fill=\"#5c6773\" font-size=\"10\">{t}%</text>\n", x(t), h - 26.0, h - 12.0);
        }
    }
    for (i, (t, classes, lo, _hi, nsites)) in fig.iter().enumerate() {
        let y = 50.0 + row_h * i as f64;
        s += &format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" fill=\"#1b1f24\">{}</text>\n", left - 10.0, y + 4.0, t);
        for (ci, (c, col)) in colors.iter().enumerate() {
            for v in classes.get(*c).into_iter().flatten() {
                let d = 100.0 * (*v - *lo) as f64 / *lo as f64;
                s += &format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"{col}\" fill-opacity=\"0.75\"><title>{t} {c}: Ir {v} (+{d:.5}%)</title></circle>\n", x(d), y - 6.0 + 4.0 * ci as f64);
            }
        }
        s += &format!("<text x=\"{:.1}\" y=\"{:.1}\" fill=\"#5c6773\" font-size=\"11\">taint sites: {nsites}</text>\n", w - right + 10.0, y + 4.0);
    }
    let mut lx = left;
    for (c, col) in colors {
        s += &format!("<circle cx=\"{lx:.1}\" cy=\"{:.1}\" r=\"4\" fill=\"{col}\"/><text x=\"{:.1}\" y=\"{:.1}\" fill=\"#5c6773\">{c}</text>\n", h - 44.0 + 40.0, lx + 8.0, h - 40.0 + 40.0);
        lx += 90.0;
    }
    s + "</svg>\n"
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("list") => TARGETS.iter().for_each(|(t, d)| println!("{t}\t{d}")),
        Some("taint") => run_taint(&args[2]),
        Some("ir") => run_ir(&args[2]),
        Some("analyze") => analyze(Path::new(&args[2]), Path::new(&args[3])),
        _ => eprintln!("usage: ct_dynamic_audit list | taint <target> | ir <target> | analyze <raw> <out>"),
    }
}
