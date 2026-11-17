//! `pairing validate <run_dir> <claimed.json>`: recompute the README's
//! headline numbers from the run's raw rows (rows.jsonl, c2pnb.jsonl) and
//! write the pcc `recompute` validator report, with the raw files pinned by
//! SHA-256. Producer-run and unsigned: it binds the numbers to the data, it
//! does not establish independence.

use std::collections::BTreeMap;
use std::fmt::Write as _;

// ---------------------------------------------------------- SHA-256 ----

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub fn sha256_hex(data: &[u8]) -> String {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut msg = data.to_vec();
    let bits = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bits.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[4 * i],
                chunk[4 * i + 1],
                chunk[4 * i + 2],
                chunk[4 * i + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7]
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [
                t1.wrapping_add(t2),
                v[0],
                v[1],
                v[2],
                v[3].wrapping_add(t1),
                v[4],
                v[5],
                v[6],
            ];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

// ------------------------------------------------------- recompute ----

fn g<'a>(l: &'a str, k: &str) -> Option<&'a str> {
    crate::jget(l, k)
}
fn n(l: &str, k: &str) -> u64 {
    crate::jnum(l, k)
}
fn b(l: &str, k: &str) -> bool {
    crate::jbool(l, k)
}

/// rho_vec(role_a) / rho_vec(role_b) in cell (N, d); rho_vec = sum rows_vec / sum targets*D.
fn ratio(lines: &[&str], nf: u64, d: u64, ra: &str, rb: &str) -> f64 {
    let rho = |role: &str| {
        let gs: Vec<&&str> = lines
            .iter()
            .filter(|l| n(l, "N") == nf && n(l, "d") == d && g(l, "role") == Some(role))
            .collect();
        let num: u64 = gs.iter().map(|l| n(l, "rows_vec")).sum();
        let den: u64 = gs.iter().map(|l| n(l, "targets") * n(l, "cols")).sum();
        num as f64 / den as f64
    };
    rho(ra) / rho(rb)
}

fn recompute(dir: &str) -> BTreeMap<String, f64> {
    let rows = std::fs::read_to_string(format!("{dir}/rows.jsonl")).unwrap();
    let c2 = std::fs::read_to_string(format!("{dir}/c2pnb.jsonl")).unwrap();
    let all: Vec<&str> = rows.lines().collect();
    let curves: Vec<&str> = all
        .iter()
        .copied()
        .filter(|l| g(l, "kind") == Some("curve"))
        .collect();
    let lines: Vec<&str> = all
        .iter()
        .copied()
        .filter(|l| g(l, "kind") == Some("line"))
        .collect();
    let prior: Vec<&str> = all
        .iter()
        .copied()
        .filter(|l| g(l, "kind") == Some("prior"))
        .collect();
    let mut r = BTreeMap::new();
    let mut put = |k: &str, v: f64| {
        r.insert(k.to_string(), v);
    };
    put("curves", curves.len() as f64);
    put(
        "curves_with_L_T",
        curves.iter().filter(|l| n(l, "tstable_lines") > 0).count() as f64,
    );
    put(
        "p1_mismatches",
        curves.iter().filter(|l| !b(l, "p1_ok")).count() as f64,
    );
    let lt: Vec<&&str> = lines
        .iter()
        .filter(|l| b(l, "tstable") && n(l, "k") == n(l, "q1"))
        .collect();
    put("L_T_lines", lt.len() as f64);
    put(
        "L_T_lines_pairing_exact",
        lt.iter()
            .filter(|l| n(l, "pair_ok") == n(l, "k") && n(l, "tplus_ok") == n(l, "k"))
            .count() as f64,
    );
    for nn in [2u64, 3, 4] {
        put(
            &format!("L_T_D_eq_half_n{nn}"),
            lt.iter()
                .filter(|l| n(l, "n") == nn && n(l, "cols") == (n(l, "k") - 1) / 2)
                .count() as f64,
        );
    }
    put("prior_rows", prior.len() as f64);
    put(
        "prior_tstable",
        prior.iter().filter(|l| b(l, "tstable")).count() as f64,
    );
    for (nf, d) in [(15u64, 5u64), (21, 7), (18, 6), (16, 4), (20, 5)] {
        put(
            &format!("rho_vec_LT_over_valid_{nf}_{d}"),
            ratio(&lines, nf, d, "L_T", "valid"),
        );
    }
    let lt_std: Vec<f64> = crate::CELLS
        .iter()
        .map(|&(nf, d)| ratio(&lines, nf as u64, d as u64, "L_T", "std"))
        .collect();
    put(
        "rho_vec_LT_over_std_min",
        lt_std.iter().cloned().fold(f64::INFINITY, f64::min),
    );
    put(
        "rho_vec_LT_over_std_max",
        lt_std.iter().cloned().fold(0.0, f64::max),
    );
    for l in c2.lines() {
        let name = g(l, "curve").unwrap().to_string();
        put(&format!("{name}_u_valid"), n(l, "u_valid") as f64);
        put(&format!("{name}_lines"), n(l, "lines") as f64);
        put(&format!("{name}_u_tstable"), n(l, "u_tstable") as f64);
    }
    r
}

pub fn validate(dir: &str, claimed_path: &str, which: &str) {
    let claimed_text = std::fs::read_to_string(claimed_path).unwrap();
    let rec = recompute(dir);
    let mut heads = vec![];
    let mut ok = true;
    for line in claimed_text.lines() {
        // one "key": value per line
        let t = line.trim().trim_end_matches(',');
        let Some((k, v)) = t.split_once(':') else {
            continue;
        };
        let k = k.trim().trim_matches('"');
        let Ok(v) = v.trim().parse::<f64>() else {
            continue;
        };
        let is_c2 = k.starts_with("c2pnb");
        if (which == "c2pnb") != is_c2 {
            continue;
        }
        let got = *rec.get(k).unwrap_or_else(|| panic!("no recompute for {k}"));
        // ratios are claimed to 3 decimals in the README
        let tol = if v.fract() != 0.0 { 0.002 } else { 0.0 };
        if (got - v).abs() > tol * v.abs().max(1.0) {
            ok = false;
        }
        heads.push(format!(
            "    {{\"name\": \"{k}\", \"claimed\": {v}, \"recomputed\": {got:.6}, \"rel_tol\": {tol}}}"
        ));
    }
    let raws: Vec<&str> = if which == "c2pnb" {
        vec!["c2pnb.jsonl"]
    } else {
        vec!["rows.jsonl"]
    };
    let mut raw_js = vec![];
    for f in &raws {
        let data = std::fs::read(format!("{dir}/{f}")).unwrap();
        raw_js.push(format!(
            "    {{\"path\": \"{f}\", \"sha256\": \"{}\"}}",
            sha256_hex(&data)
        ));
    }
    let mut out = String::new();
    writeln!(
        out,
        "{{\n  \"schema\": \"validator-report\",\n  \"status\": \"{}\",",
        if ok { "pass" } else { "fail" }
    )
    .unwrap();
    writeln!(out, "  \"study\": \"research/lift_dense_torsion_pairing_20261106\",\n  \"recomputed_by\": {{\"tool\": \"research/lift_dense_torsion_pairing_20261106/pairing (pairing validate)\", \"command\": \"pairing validate <run> claimed.json {which}\"}},").unwrap();
    writeln!(out, "  \"note\": \"Producer-run recompute of README headline numbers from the pinned raw rows; unsigned, so independence is not established.\",").unwrap();
    writeln!(out, "  \"raw_data\": [\n{}\n  ],", raw_js.join(",\n")).unwrap();
    writeln!(out, "  \"headline\": [\n{}\n  ]\n}}", heads.join(",\n")).unwrap();
    let name = if which == "c2pnb" {
        "validator-report-c2pnb.json"
    } else {
        "validator-report.json"
    };
    std::fs::write(format!("{dir}/{name}"), out).unwrap();
    eprintln!("{name}: {}", if ok { "pass" } else { "FAIL" });
    if !ok {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sha256_vectors() {
        assert_eq!(
            super::sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            super::sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
