//! Construction subcommands (Rust ports of the former `build_neighbours.py`, `build_tower.py`,
//! `make_targets.py`).  The number theory runs in libpari 2.17 through dlopen (`pari.rs`, GP source
//! in `gp/construct.gp`); hashing, ICV1 identities, JSON layout and all the bookkeeping are Rust.
//! Outputs keep the key order and layout of `json.dump(.., indent=1)`.
//!
//!   p1_eccp131 build-neighbours [--out neighbours.json] [--lmax 150] [--only 5,7] [--libpari PATH]
//!   p1_eccp131 build-tower      [--out tower.json] [--tmax 5] [--p DEC] [--libpari PATH]   (--p: same construction over another prime)
//!   p1_eccp131 make-targets     [--nb neighbours.json] [--ntargets 16] [--out targets.json] [--libpari PATH]
//!   p1_eccp131 jsoncmp A.json B.json [--skip timing_s,wall_s,build_total_s_sum]

use super::jsonv::{self, flt, int, obj, s, J};
use super::pari::{find_libpari, Pari};
use sha2::{Digest, Sha256};
use std::time::Instant;

const GP_SRC: &str = include_str!("../gp/construct.gp");
const P_HEX: &str = "0x48e1d43f293469e33194c43186b3abc0b";
const A_HEX: &str = "0x41cb121ce2b31f608a76fc8f23d73cb66";
const B_HEX: &str = "0x2f74f717e8dec90991e5ea9b2ff03da58";
const N_HEX: &str = "0x48e1d43f293469e317f7ed728f6b8e6f1";
const TRACE: &str = "29529377007934231835";
const SEED: u64 = 20261102;
const EXPECTED_ELKIES: [(u32, usize); 14] =
    [(5, 2), (7, 2), (13, 2), (17, 2), (29, 2), (37, 2), (61, 2), (67, 1), (73, 2), (83, 2), (89, 2), (107, 2), (131, 2), (149, 2)];

fn opt(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}

fn h256(parts: &[String]) -> String {
    Sha256::digest(parts.join("|").as_bytes()).iter().map(|b| format!("{b:02x}")).collect()
}

fn hh(seed: u64, label: &str) -> String {
    h256(&[seed.to_string(), label.to_string()])
}

/// `0x...` GP literal of the integer SHA-256(seed|label)
fn hlit(label: &str) -> String {
    format!("0x{}", hh(SEED, label))
}

fn hlit_parts(parts: &[String]) -> String {
    format!("0x{}", h256(parts))
}

// ---- decimal -> Python hex() ---------------------------------------------------------------

fn dec_to_hex(d: &str) -> String {
    let mut limbs: Vec<u32> = vec![0];
    for c in d.bytes() {
        assert!(c.is_ascii_digit(), "not a decimal: {d}");
        let mut carry = (c - b'0') as u64;
        for l in limbs.iter_mut() {
            let v = *l as u64 * 10 + carry;
            *l = v as u32;
            carry = v >> 32;
        }
        if carry > 0 {
            limbs.push(carry as u32);
        }
    }
    let mut out = String::new();
    for (i, l) in limbs.iter().rev().enumerate() {
        if i == 0 {
            out.push_str(&format!("{l:x}"));
        } else {
            out.push_str(&format!("{l:08x}"));
        }
    }
    format!("0x{out}")
}

fn hex_to_dec(h: &str) -> String {
    let h = h.trim_start_matches("0x");
    let mut limbs: Vec<u32> = vec![0];
    for c in h.chars() {
        let mut carry = c.to_digit(16).unwrap() as u64;
        for l in limbs.iter_mut() {
            let v = *l as u64 * 16 + carry;
            *l = v as u32;
            carry = v >> 32;
        }
        if carry > 0 {
            limbs.push(carry as u32);
        }
    }
    // repeated division by 1e9
    let mut parts = Vec::new();
    while !(limbs.len() == 1 && limbs[0] == 0) {
        let mut rem = 0u64;
        for l in limbs.iter_mut().rev() {
            let cur = (rem << 32) | *l as u64;
            *l = (cur / 1_000_000_000) as u32;
            rem = cur % 1_000_000_000;
        }
        while limbs.len() > 1 && *limbs.last().unwrap() == 0 {
            limbs.pop();
        }
        parts.push(rem);
    }
    if parts.is_empty() {
        return "0".into();
    }
    let mut o = format!("{}", parts.pop().unwrap());
    while let Some(x) = parts.pop() {
        o.push_str(&format!("{x:09}"));
    }
    o
}

fn hx(d: &str) -> J {
    s(dec_to_hex(d))
}

fn digits(sv: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in sv.chars() {
        if c.is_ascii_digit() {
            cur.push(c);
        } else if !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// ICV1 identity of y^2 = x^3 + a x + b over F_p (scripts/curve_id.py `prime_id`, end/level "unk")
struct Icv1 {
    slug: String,
    icv1: String,
    model_sha256: String,
}

fn prime_id(p_dec: &str, a_dec: &str, b_dec: &str, order_dec: &str, j_dec: &str, trace: &str) -> Icv1 {
    let field = format!("fp-{p_dec}");
    // canonical JSON: sorted keys, compact separators
    let model_json = format!(
        "{{\"a\":\"{a_dec}\",\"b\":\"{b_dec}\",\"field\":\"{field}\",\"form\":\"y^2=x^3+a*x+b\",\"p\":\"{p_dec}\",\"v\":\"1\"}}"
    );
    let model: String = Sha256::digest(model_json.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let hexp = dec_to_hex(p_dec);
    let digs = hexp.trim_start_matches("0x");
    let first = digs.chars().next().unwrap().to_digit(16).unwrap();
    let bits = 4 * (digs.len() - 1) + (32 - first.leading_zeros() as usize);
    let t = if let Some(m) = trace.strip_prefix('-') { format!("tm{m}") } else { format!("t{trace}") };
    Icv1 {
        slug: format!("icv1-fp{bits}-{t}-{}", &model[..8]),
        icv1: format!("ICV1:{field}:{trace}:{order_dec}:{j_dec}:unk:unk:r:{}", &model[..12]),
        model_sha256: model,
    }
}

fn open_pari(args: &[String]) -> Pari {
    let lib = opt(args, "--libpari").unwrap_or_else(find_libpari);
    let pari = Pari::open(&lib, 6usize << 30).expect("libpari");
    eprintln!("PARI {}", pari.version);
    pari.load(GP_SRC);
    pari
}

fn write_out(path: &str, j: &J) {
    std::fs::write(path, j.dump()).unwrap();
}

fn primes_upto(n: u32) -> Vec<u32> {
    (2..=n).filter(|&k| (2..k).take_while(|d| d * d <= k).all(|d| k % d != 0)).collect()
}

fn pt_hex(tokens: &mut std::slice::Iter<&str>) -> J {
    let x = tokens.next().unwrap();
    if *x == "O" {
        return s("O");
    }
    let y = tokens.next().unwrap();
    J::Arr(vec![hx(x), hx(y)])
}

// ================================================================== build-neighbours

pub fn cmd_build_neighbours(args: &[String]) {
    let out_path = opt(args, "--out").unwrap_or("neighbours.json".into());
    let lmax: u32 = opt(args, "--lmax").map(|v| v.parse().unwrap()).unwrap_or(150);
    let only: Vec<u32> = opt(args, "--only").unwrap_or_default().split(',').filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
    let pari = open_pari(args);
    let t_start = Instant::now();
    let (p_dec, a_dec, b_dec, n_dec) = (hex_to_dec(P_HEX), hex_to_dec(A_HEX), hex_to_dec(B_HEX), hex_to_dec(N_HEX));
    let ev = |e: &str| pari.eval_str(e);

    let j0 = ev(&format!("P1init({p_dec},{a_dec},{b_dec},{n_dec},{TRACE})"));
    let id0 = prime_id(&p_dec, &a_dec, &b_dec, &n_dec, &j0, TRACE);
    let k_dlp = ev(&format!("{}%({n_dec}-2)+1", hlit("dlp-k")));
    let pl = ev(&format!("P1plant({},{k_dlp})", hlit("planted-P")));
    let pl: Vec<&str> = pl.split(' ').collect();
    let (p0x, p0y, q0x, q0y) = (pl[0], pl[1], pl[2], pl[3]);

    let mut out = obj(vec![
        ("schema", s("p1.neighbours/v1")),
        ("seed", int(SEED)),
        ("pari", s(pari.version.clone())),
        ("p", s(P_HEX)),
        ("n", s(N_HEX)),
        ("trace", J::Int(TRACE.to_string())),
        ("E0", obj(vec![("a", s(A_HEX)), ("b", s(B_HEX)), ("j", hx(&j0)), ("slug", s(id0.slug.clone())), ("icv1", s(id0.icv1.clone())),
                        ("model_sha256", s(id0.model_sha256.clone())), ("role", s("E0"))])),
        ("planted_dlp", obj(vec![("k", hx(&k_dlp)), ("P", J::Arr(vec![hx(p0x), hx(p0y)])), ("Q", J::Arr(vec![hx(q0x), hx(q0y)])),
            ("rule", s("P = first rational point at x >= SHA256(seed|planted-P); k = SHA256(seed|dlp-k) mod (n-2) + 1"))])),
        ("neighbours", J::Arr(vec![])),
        ("controls", J::Arr(vec![])),
        ("elkies_roots_per_l", obj(vec![])),
        ("timing_s", obj(vec![])),
    ]);

    // ---- isomorphic-model control -----------------------------------------------------------
    let u = ev(&format!("{}%({p_dec}-2)+2", hlit("iso-u")));
    let ic = ev(&format!("P1iso({u})"));
    let ic: Vec<&str> = ic.split(' ').collect();
    let idi = prime_id(&p_dec, ic[0], ic[1], &n_dec, &j0, TRACE);
    let ctl = obj(vec![
        ("role", s("iso_control")), ("a", hx(ic[0])), ("b", hx(ic[1])), ("u", hx(&u)), ("j", hx(&j0)),
        ("slug", s(idi.slug)), ("icv1", s(idi.icv1)), ("model_sha256", s(idi.model_sha256)),
        ("map", s("(x,y) -> (u^2 x, u^3 y)")),
        ("planted_Q_image", J::Arr(vec![hx(ic[2]), hx(ic[3])])),
        ("check_on_curve", J::Bool(ic[4] == "1")),
    ]);
    if let J::Obj(o) = &mut out {
        for (k, v) in o.iter_mut() {
            if k == "controls" {
                *v = J::Arr(vec![ctl.clone()]);
            }
        }
    }

    // ---- Elkies primes ----------------------------------------------------------------------
    let mut roots_by_l: Vec<(u32, Vec<String>)> = Vec::new();
    let t0 = Instant::now();
    for l in primes_upto(lmax) {
        if l <= 3 || (!only.is_empty() && !only.contains(&l)) {
            continue;
        }
        let r = digits(&ev(&format!("P1roots({l})")));
        if !r.is_empty() {
            roots_by_l.push((l, r));
        }
    }
    let t_phi = t0.elapsed().as_secs_f64();
    out.set("timing_s", obj(vec![("phi_roots_all_l", flt(t_phi))]));
    out.set("elkies_roots_per_l", J::Obj(roots_by_l.iter().map(|(l, v)| (l.to_string(), J::Arr(v.iter().map(|x| hx(x)).collect()))).collect()));
    if only.is_empty() {
        let got: Vec<(u32, usize)> = roots_by_l.iter().map(|(l, v)| (*l, v.len())).collect();
        assert_eq!(got, EXPECTED_ELKIES.to_vec(), "Elkies prime pattern");
    }

    // ---- kernel polynomials and isogenies ---------------------------------------------------
    let mut recs: Vec<(u32, u64, J, bool, f64)> = Vec::new();
    for (l, roots) in &roots_by_l {
        let l = *l;
        let tl = Instant::now();
        let lams_s = ev(&format!("P1prep({l})"));
        let shared = tl.elapsed().as_secs_f64();
        let lams: Vec<u64> = digits(&lams_s).iter().map(|x| x.parse().unwrap()).collect();
        let mut js: Vec<String> = Vec::new();
        for (idx, &lam) in lams.iter().enumerate() {
            let tk = Instant::now();
            let deg: u32 = ev(&format!("P1kern({l},{lam},{idx},{})", lams.len())).parse().unwrap();
            assert_eq!(deg, (l - 1) / 2);
            let t_kernel = tk.elapsed().as_secs_f64();
            let tv = Instant::now();
            let vel = ev("P1velu()");
            let t_isog = tv.elapsed().as_secs_f64();
            let vel: Vec<&str> = vel.split(' ').collect();
            let (jp, ap, bp) = (vel[0], vel[1], vel[2]);
            assert!(roots.iter().any(|r| r == jp), "j' not a root: l={l} lam={lam}");
            let coeffs = digits(&ev("P1kcoef()"));
            let tc = Instant::now();
            let xs: Vec<String> = (0..6).flat_map(|i| ["a", "b"].map(|ab| hlit(&format!("hom-{l}-{lam}-{i}-{ab}")))).collect();
            let ver = ev(&format!("P1verify({l},[{}])", xs.join(",")));
            let t_check = tc.elapsed().as_secs_f64();
            let vt: Vec<&str> = ver.split(' ').collect();
            let (dg0, dg1) = (vt[0], vt[1]);
            let flag = |i: usize| vt[i] == "1";
            let mut it = vt[9..].iter();
            let phi_p = pt_hex(&mut it);
            let phi_q = pt_hex(&mut it);
            let idn = prime_id(&p_dec, ap, bp, &n_dec, jp, TRACE);
            let chk_vals = [flag(2), flag(3), flag(4), flag(5), flag(6), flag(7), true, flag(8)];
            let ok = chk_vals.iter().all(|&b| b);
            let chk = obj(vec![
                ("x_map_degrees_f_over_h2", J::Arr(vec![int(dg0), int(dg1)])),
                ("degree_ok", J::Bool(flag(2))),
                ("homomorphism_6_pairs", J::Bool(flag(3))),
                ("images_on_curve", J::Bool(flag(4))),
                ("ellcard_equals_n", J::Bool(flag(5))),
                ("image_generator_order_n", J::Bool(flag(6))),
                ("planted_dlp_transports", J::Bool(flag(7))),
                ("j_prime_is_root_of_Phi_l", J::Bool(true)),
                ("j_prime_not_j0", J::Bool(flag(8))),
            ]);
            let ksha = h256(&["kernel".to_string(), l.to_string(), coeffs.join(",")]);
            let build_total = t_kernel + t_isog + shared / lams.len() as f64;
            let rec = obj(vec![
                ("role", s("neighbour")), ("l", int(l)), ("lambda", int(lam)), ("kernel_degree", int((l - 1) / 2)),
                ("a", hx(ap)), ("b", hx(bp)), ("j", hx(jp)),
                ("slug", s(idn.slug.clone())), ("icv1", s(idn.icv1)), ("model_sha256", s(idn.model_sha256)),
                ("id_note", s("ICV1-style slug from scripts/curve_id.py prime_id; not yet in docs/curves/registry.json")),
                ("kernel_poly_high_to_low", J::Arr(coeffs.iter().map(|c| hx(c)).collect())),
                ("kernel_poly_sha256", s(ksha)),
                ("phi_P", phi_p), ("phi_Q", phi_q), ("checks", chk), ("all_checks_pass", J::Bool(ok)),
                ("timing_s", obj(vec![("kernel_gcd", flt(t_kernel)), ("shared_psi_Xp_per_l", flt(shared)), ("velu_isogeny", flt(t_isog)),
                                      ("verify", flt(t_check)), ("build_total", flt(build_total))])),
            ]);
            println!("l={l:3} lam={lam:3} {} ok={} kernel={t_kernel:.2}s isog={t_isog:.2}s verify={t_check:.2}s shared={shared:.2}s",
                     idn.slug, if ok { "True" } else { "False" });
            js.push(jp.to_string());
            recs.push((l, lam, rec, ok, build_total));
        }
        let mut uniq = js.clone();
        uniq.sort();
        uniq.dedup();
        assert!(uniq.len() == js.len() && js.len() == roots.len(), "distinct j' per l={l}");
        let mut a: Vec<String> = js.clone();
        a.sort_by_key(|x| (x.len(), x.clone()));
        let mut b: Vec<String> = roots.clone();
        b.sort_by_key(|x| (x.len(), x.clone()));
        assert_eq!(a, b, "j' set equals the Phi_l roots, l={l}");
    }
    recs.sort_by_key(|r| (r.0, r.1));
    let all_ok = recs.iter().all(|r| r.3);
    let count = recs.len();
    let bt_sum: f64 = recs.iter().map(|r| r.4).sum();
    out.set("neighbours", J::Arr(recs.into_iter().map(|r| r.2).collect()));
    out.push("summary", obj(vec![("count", int(count)), ("all_checks_pass", J::Bool(all_ok)), ("build_total_s_sum", flt(bt_sum)),
                                  ("wall_s", flt(t_start.elapsed().as_secs_f64()))]));
    write_out(&out_path, &out);
    println!("wrote {out_path} count={count} all_checks_pass={all_ok}");
    if !all_ok || (only.is_empty() && count != 27) {
        std::process::exit(1);
    }
}

// ================================================================== build-tower

pub fn cmd_build_tower(args: &[String]) {
    let out_path = opt(args, "--out").unwrap_or("tower.json".into());
    let tmax: usize = opt(args, "--tmax").map(|v| v.parse().unwrap()).unwrap_or(5);
    let pari = open_pari(args);
    let t_start = Instant::now();
    // `--p DEC` builds the same tower construction over another prime (used by the f4_fp cross-check)
    let p_dec = opt(args, "--p").unwrap_or_else(|| hex_to_dec(P_HEX));
    let (a_dec, b_dec, n_dec) = (hex_to_dec(A_HEX), hex_to_dec(B_HEX), hex_to_dec(N_HEX));
    let _ = (&a_dec, &b_dec, &n_dec);
    let ev = |e: &str| pari.eval_str(e);
    // only p is set: the generator PARI picks in ellgroup(.,.,1) depends on its random state, so no
    // other randomised call (ellcard, isprime) may run before the first ellgroup (the Python builder had none)
    ev(&format!("p={p_dec};0"));

    let mut tried: u64 = 0;
    let (mut aa, mut bb);
    loop {
        aa = ev(&format!("{}%{p_dec}", hlit_parts(&[SEED.to_string(), "aux-A".into(), tried.to_string()])));
        bb = ev(&format!("{}%{p_dec}", hlit_parts(&[SEED.to_string(), "aux-B".into(), tried.to_string()])));
        tried += 1;
        if ev(&format!("P1auxtry({aa},{bb},{tmax})")) == "1" {
            break;
        }
    }
    let info = ev(&format!("P1auxinit({tmax})"));
    let info: Vec<&str> = info.split(' ').collect();
    let (order, d1, d2) = (info[0], info[1], info[2]);
    let mut j = 0u64;
    let rr;
    loop {
        let x = ev(&format!("{}%{p_dec}", hlit_parts(&[SEED.to_string(), "aux-R".into(), j.to_string()])));
        j += 1;
        if ev(&format!("P1tryR({x})")) == "1" {
            rr = ev("Str(Rq[1],\" \",Rq[2])");
            break;
        }
    }
    let rr: Vec<&str> = rr.split(' ').collect();
    let orders = if d2 == "0" { vec![int_hex_dec(d1)] } else { vec![int_hex_dec(d1), int_hex_dec(d2)] };
    let mut out = obj(vec![
        ("schema", s("p1.tower/v1")), ("p", s(dec_to_hex(&p_dec))), ("seed", int(SEED)),
        ("formulation", s("PKM isogeny tower, 2-isogeny chain")),
        ("aux_curve", obj(vec![("A", hx(&aa)), ("B", hx(&bb)), ("order", J::Int(order.to_string())),
                               ("group_orders", J::Arr(orders)), ("curves_tried", int(tried)),
                               ("R", J::Arr(vec![hx(rr[0]), hx(rr[1])])), ("Q_order", int(1u64 << tmax))])),
        ("towers", obj(vec![])),
    ]);
    let mut towers: Vec<(String, J)> = Vec::new();
    for t in 2..=tmax {
        let xs: Vec<String> = (0..t).map(|jj| format!("{}%{p_dec}", hlit_parts(&[SEED.to_string(), "chk".into(), t.to_string(), jj.to_string()]))).collect();
        let r = ev(&format!("P1tower({t},{tmax},[{}])", xs.join(",")));
        let nums = digits(&r);
        assert_eq!(nums.len(), 7 * t + 1 + (1 << t), "tower output size");
        let mut stages = Vec::new();
        for jj in 0..t {
            let o = 7 * jj;
            stages.push(obj(vec![
                ("num_low_to_high", J::Arr(nums[o..o + 3].iter().map(|x| hx(x)).collect())),
                ("den_low_to_high", J::Arr(nums[o + 3..o + 5].iter().map(|x| hx(x)).collect())),
                ("curve", obj(vec![("a", hx(&nums[o + 5])), ("b", hx(&nums[o + 6]))])),
            ]));
        }
        let c = &nums[7 * t];
        let v = &nums[7 * t + 1..];
        println!("t={t} |V|={} ok", v.len());
        towers.push((t.to_string(), obj(vec![("t", int(t)), ("size", int(1u64 << t)), ("c", hx(c)), ("stages", J::Arr(stages)),
                                              ("V_sorted", J::Arr(v.iter().map(|x| hx(x)).collect()))])));
    }
    out.set("towers", J::Obj(towers));
    out.push("wall_s", flt(t_start.elapsed().as_secs_f64()));
    write_out(&out_path, &out);
    println!("wrote {out_path} aux curves tried {tried}");
}

fn int_hex_dec(d: &str) -> J {
    J::Int(d.to_string())
}

// ================================================================== make-targets

pub fn cmd_make_targets(args: &[String]) {
    let nb_path = opt(args, "--nb").unwrap_or("neighbours.json".into());
    let nt: usize = opt(args, "--ntargets").map(|v| v.parse().unwrap()).unwrap_or(16);
    let out_path = opt(args, "--out").unwrap_or("targets.json".into());
    let nb = jsonv::parse(&std::fs::read_to_string(&nb_path).unwrap()).unwrap();
    let pari = open_pari(args);
    let ev = |e: &str| pari.eval_str(e);
    let hexlit = |j: &J| j.as_str().to_string();
    let (p_hex, n_hex) = (nb.get("p").as_str().to_string(), nb.get("n").as_str().to_string());
    let (p_dec, n_dec) = (hex_to_dec(&p_hex), hex_to_dec(&n_hex));
    let seed: u64 = nb.get("seed").as_int_text().parse().unwrap();
    let e0 = nb.get("E0");
    let (a_hex, b_hex) = (e0.get("a").as_str().to_string(), e0.get("b").as_str().to_string());
    ev(&format!("P1init({p_dec},{},{},{n_dec},{})", hexlit(e0.get("a")), hexlit(e0.get("b")), nb.get("trace").as_int_text()));
    let pd = nb.get("planted_dlp");
    let pp = pd.get("P").as_arr();
    let qq = pd.get("Q").as_arr();
    let ok = ev(&format!(
        "P0=[{},{}];Q0=[{},{}];k0={};lift(ellmul(E0,P0,k0))==Q0",
        hexlit(&pp[0]), hexlit(&pp[1]), hexlit(&qq[0]), hexlit(&qq[1]), hexlit(pd.get("k"))
    ));
    assert_eq!(ok, "1", "planted DLP Q = k P");
    let k0_dec = ev("k0");

    // targets
    let mut ks: Vec<String> = Vec::new();
    for tau in 0..nt {
        if tau == 0 {
            ks.push(k0_dec.clone());
        } else {
            ks.push(ev(&format!("{}%({n_dec}-2)+1", hlit_parts(&[seed.to_string(), "target-k".into(), tau.to_string()]))));
        }
    }
    let rs = digits(&ev(&format!("P1tgt([{}])", ks.join(","))));
    assert_eq!(rs.len(), 2 * nt);
    let mut targets: Vec<J> = Vec::new();
    let mut xbyarm: Vec<Vec<(String, J)>> = vec![Vec::new(); nt];
    let r_x: Vec<String> = (0..nt).map(|t| rs[2 * t].clone()).collect();

    let mut arms: Vec<J> = vec![
        obj(vec![("arm", s("E0")), ("role", s("reference")), ("a", s(a_hex.clone())), ("b", s(b_hex.clone())), ("slug", s(e0.get("slug").as_str()))]),
        obj(vec![("arm", s("AA_repeat")), ("role", s("aa_repeat")), ("a", s(a_hex.clone())), ("b", s(b_hex.clone())), ("slug", s(e0.get("slug").as_str())),
                 ("note", s("same instance as E0, separate run slot (determinism and clock noise floor)"))]),
        obj(vec![("arm", s("AA_perm")), ("role", s("aa_variable_order")), ("a", s(a_hex.clone())), ("b", s(b_hex.clone())), ("slug", s(e0.get("slug").as_str())),
                 ("note", s("same instance as E0 with a seeded permutation of the variable order (re-encoding noise floor)"))]),
    ];
    for t in 0..nt {
        for arm in ["E0", "AA_repeat", "AA_perm"] {
            xbyarm[t].push((arm.to_string(), hx(&r_x[t])));
        }
    }
    let c = nb.get("controls").as_arr()[0].clone();
    let u_dec = hex_to_dec(c.get("u").as_str());
    arms.push(obj(vec![("arm", s("iso")), ("role", s("iso_control")), ("a", s(c.get("a").as_str())), ("b", s(c.get("b").as_str())),
                       ("slug", s(c.get("slug").as_str())), ("u", s(c.get("u").as_str()))]));
    for t in 0..nt {
        let xi = ev(&format!("({}*{u_dec}^2)%{p_dec}", r_x[t]));
        xbyarm[t].push(("iso".to_string(), hx(&xi)));
    }

    let mut seen: std::collections::HashMap<String, usize> = Default::default();
    for rec in nb.get("neighbours").as_arr() {
        let l: u64 = rec.get("l").as_int_text().parse().unwrap();
        let idx = *seen.entry(l.to_string()).or_insert(0);
        *seen.get_mut(&l.to_string()).unwrap() += 1;
        let arm = format!("n{l}_{idx}");
        let co: Vec<String> = rec.get("kernel_poly_high_to_low").as_arr().iter().map(|x| hex_to_dec(x.as_str())).collect();
        let res = digits(&ev(&format!(
            "P1arm([{}],{},{},{})", co.join(","), hex_to_dec(rec.get("j").as_str()), hex_to_dec(rec.get("a").as_str()), hex_to_dec(rec.get("b").as_str())
        )));
        // res: phi(P) x y, then one x per target
        let stored_p = rec.get("phi_P").as_arr();
        assert_eq!(dec_to_hex(&res[0]), stored_p[0].as_str());
        assert_eq!(dec_to_hex(&res[1]), stored_p[1].as_str());
        assert_eq!(res.len(), 2 + nt);
        arms.push(obj(vec![("arm", s(arm.clone())), ("role", s("neighbour")), ("l", int(l)), ("slot", int(idx)), ("a", s(rec.get("a").as_str())),
                           ("b", s(rec.get("b").as_str())), ("slug", s(rec.get("slug").as_str())),
                           ("kernel_poly_sha256", s(rec.get("kernel_poly_sha256").as_str()))]));
        for t in 0..nt {
            xbyarm[t].push((arm.clone(), hx(&res[2 + t])));
        }
        println!("{arm} {}", rec.get("slug").as_str());
    }
    for t in 0..nt {
        let rx = &rs[2 * t];
        let ry = &rs[2 * t + 1];
        targets.push(obj(vec![("tau", int(t)), ("k", hx(&ks[t])), ("R", J::Arr(vec![hx(rx), hx(ry)])), ("x_by_arm", J::Obj(std::mem::take(&mut xbyarm[t])))]));
    }
    let out = obj(vec![
        ("schema", s("p1.targets/v1")), ("seed", int(seed)), ("p", s(p_hex)), ("n", s(n_hex)), ("arms", J::Arr(arms.clone())), ("targets", J::Arr(targets)),
        ("rule", s("target 0 = planted Q; target tau = k_tau P0, k_tau = SHA256(seed|target-k|tau) mod (n-2) + 1")),
    ]);
    write_out(&out_path, &out);
    println!("wrote {out_path} {} arms {nt} targets", arms.len());
}

// ================================================================== jsoncmp

pub fn cmd_jsoncmp(args: &[String]) {
    let skip: Vec<String> = opt(args, "--skip").unwrap_or_default().split(',').filter(|x| !x.is_empty()).map(String::from).collect();
    let files: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    // the value after --skip is not a file
    let files: Vec<&&String> = files.iter().filter(|f| Some(f.as_str()) != opt(args, "--skip").as_deref()).collect();
    assert!(files.len() == 2, "usage: jsoncmp A B [--skip k1,k2]");
    let mut canon = Vec::new();
    for f in &files {
        let raw = std::fs::read(f.as_str()).unwrap();
        let raw_sha: String = Sha256::digest(&raw).iter().map(|b| format!("{b:02x}")).collect();
        let j = jsonv::parse(std::str::from_utf8(&raw).unwrap()).unwrap();
        let c = j.canon(&skip);
        let csha: String = Sha256::digest(c.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
        println!("{}  raw_sha256={raw_sha}  canonical_sha256(skip={:?})={csha}", f, skip);
        canon.push((c, raw));
    }
    println!("byte_identical={}", canon[0].1 == canon[1].1);
    let same = canon[0].0 == canon[1].0;
    println!("canonical_identical={same}");
    if !same {
        let (a, b) = (&canon[0].0, &canon[1].0);
        let i = a.bytes().zip(b.bytes()).position(|(x, y)| x != y).unwrap_or(a.len().min(b.len()));
        let lo = i.saturating_sub(60);
        println!("first difference at {i}:\n A: {}\n B: {}", &a[lo..(i + 60).min(a.len())], &b[lo..(i + 60).min(b.len())]);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_dec_round_trip() {
        for h in ["0x0", "0x1", "0xff", P_HEX, N_HEX, "0x10000000000000000", "0xffffffffffffffffffffffff"] {
            assert_eq!(dec_to_hex(&hex_to_dec(h)), h);
        }
        assert_eq!(hex_to_dec(P_HEX), "1550031797834347859248576414813139942411");
    }

    #[test]
    fn icv1_identity_of_e0_matches_scripts_curve_id_py() {
        // values committed in neighbours.json (produced by scripts/curve_id.py prime_id)
        let id = prime_id(
            &hex_to_dec(P_HEX),
            &hex_to_dec(A_HEX),
            &hex_to_dec(B_HEX),
            &hex_to_dec(N_HEX),
            "424712279079156405453899315082070930793",
            TRACE,
        );
        assert_eq!(id.slug, "icv1-fp131-t29529377007934231835-9681b699");
        assert_eq!(id.model_sha256, "9681b699f2f5930f691df383d9b98f9772e19796959cbd68b34f6f8bf47ef4c6");
        assert_eq!(
            id.icv1,
            "ICV1:fp-1550031797834347859248576414813139942411:29529377007934231835:1550031797834347859219047037805205710577:424712279079156405453899315082070930793:unk:unk:r:9681b699f2f5"
        );
    }

    #[test]
    fn json_layout_is_python_indent_1() {
        let j = obj(vec![("a", int(1)), ("b", J::Arr(vec![s("x"), J::Bool(true)])), ("c", obj(vec![])), ("d", J::Arr(vec![]))]);
        assert_eq!(j.dump(), "{\n \"a\": 1,\n \"b\": [\n  \"x\",\n  true\n ],\n \"c\": {},\n \"d\": []\n}");
        assert_eq!(jsonv::parse(&j.dump()).unwrap(), j);
    }
}
