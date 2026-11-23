//! Original-top chain certificates. No affine row reduction or base changes.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_string_pretty(v).unwrap() + "\n").unwrap()
}
#[derive(Clone, Copy)]
struct Field {
    n: usize,
    low: u128,
}
impl Field {
    fn mask(self) -> u128 {
        (1u128 << self.n) - 1
    }
    fn mul(self, mut a: u128, mut b: u128) -> u128 {
        assert!(a <= self.mask() && b <= self.mask());
        let mut r = 0;
        while b != 0 {
            if b & 1 != 0 {
                r ^= a;
            }
            b >>= 1;
            let high = a >> (self.n - 1);
            a = (a << 1) & self.mask();
            if high != 0 {
                a ^= self.low;
            }
        }
        r
    }
    fn pow(self, mut a: u128, mut e: u128) -> u128 {
        let mut r = 1;
        while e != 0 {
            if e & 1 != 0 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn slow(self, a: u128, b: u128) -> u128 {
        let mut c = vec![false; 2 * self.n];
        for i in 0..self.n {
            for j in 0..self.n {
                if a >> i & 1 != 0 && b >> j & 1 != 0 {
                    c[i + j] ^= true;
                }
            }
        }
        let f = (1u128 << self.n) | self.low;
        for k in (self.n..2 * self.n).rev() {
            if c[k] {
                for i in 0..=self.n {
                    if f >> i & 1 != 0 {
                        c[k - self.n + i] ^= true;
                    }
                }
            }
        }
        (0..self.n).fold(0, |r, i| r | ((c[i] as u128) << i))
    }
    fn certificate(self) -> Value {
        assert!((2..self.n)
            .filter(|d| d * d <= self.n)
            .all(|d| self.n % d != 0));
        assert_eq!(self.low & 1, 1);
        assert_eq!(((1u128 << self.n) | self.low).count_ones() % 2, 1);
        let mut z = 2;
        let mut ladder = vec![z.to_string()];
        for _ in 0..self.n {
            let q = self.mul(z, z);
            assert_eq!(q, self.slow(z, z));
            z = q;
            ladder.push(z.to_string());
        }
        assert_eq!(z, 2);
        json!({"n":self.n,"modulus_hex":format!("{:x}",(1u128<<self.n)|self.low),"degree_prime":true,"f_at_zero_and_one_nonzero":true,"frobenius_ladder":ladder,"irreducible":true,"criterion":"Rabin: prime degree n; z^(2^n)=z; gcd(f,z^2+z)=1"})
    }
}
fn f83() -> Field {
    Field {
        n: 83,
        low: (1u128 << 45) | 7,
    }
}
fn matrix(f: Field, w: &[u128], rows: usize) -> Vec<Vec<u128>> {
    let mut m = vec![w.to_vec()];
    for k in 1..rows {
        m.push(m[k - 1].iter().map(|x| f.mul(*x, *x)).collect());
    }
    m
}
fn inverse(f: Field, m: &[Vec<u128>]) -> Option<Vec<Vec<u128>>> {
    let n = m.len();
    let mut a = vec![vec![0; 2 * n]; n];
    for i in 0..n {
        assert_eq!(m[i].len(), n);
        a[i][..n].copy_from_slice(&m[i]);
        a[i][n + i] = 1;
    }
    for c in 0..n {
        let p = (c..n).find(|i| a[*i][c] != 0)?;
        a.swap(p, c);
        let inv = f.pow(a[c][c], f.mask() - 1);
        assert_eq!(f.mul(a[c][c], inv), 1);
        for j in 0..2 * n {
            a[c][j] = f.mul(a[c][j], inv);
        }
        for i in 0..n {
            if i != c {
                let q = a[i][c];
                for j in 0..2 * n {
                    a[i][j] ^= f.mul(q, a[c][j]);
                }
            }
        }
    }
    Some(a.iter().map(|r| r[n..].to_vec()).collect())
}
fn check_inverse(f: Field, m: &[Vec<u128>], v: &[Vec<u128>]) -> bool {
    let n = m.len();
    (0..n).all(|i| {
        (0..n).all(|j| {
            let a = (0..n).fold(0, |r, k| r ^ f.mul(m[i][k], v[k][j]));
            let b = (0..n).fold(0, |r, k| r ^ f.mul(v[i][k], m[k][j]));
            a == u128::from(i == j) && b == u128::from(i == j)
        })
    })
}
fn mj(m: &[Vec<u128>]) -> Value {
    json!(m
        .iter()
        .map(|r| r.iter().map(u128::to_string).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}
fn full_weights(f: Field) -> Vec<u128> {
    (0..f.n).map(|i| 1u128 << i).collect()
}
fn weights(spike: bool) -> Vec<u128> {
    (0..27)
        .map(|i| 1u128 << if spike && i == 26 { 40 } else { i })
        .collect()
}
fn linear_map(row: &[u128], bits: u128) -> u128 {
    row.iter()
        .enumerate()
        .filter(|(i, _)| bits >> i & 1 != 0)
        .fold(0, |a, (_, x)| a ^ x)
}
fn transform(f: Field, w: &[u128]) -> Value {
    let m = matrix(f, w, w.len());
    let inv = inverse(f, &m).expect("singular Moore matrix");
    assert!(check_inverse(f, &m, &inv));
    json!({"weights":w.iter().map(u128::to_string).collect::<Vec<_>>(),"matrix":mj(&m),"inverse":mj(&inv),"both_products_identity":true})
}
fn product_table(f: Field, ww: &[Vec<u128>]) -> Vec<u128> {
    let mut v = vec![1];
    for w in ww {
        v = v
            .into_iter()
            .flat_map(|a| w.iter().map(move |b| f.mul(a, *b)))
            .collect();
    }
    v
}
fn cubic(f: Field, ww: &[Vec<u128>]) -> Value {
    assert_eq!(ww.len(), 3);
    let ell = ww.iter().map(Vec::len).min().unwrap();
    let em = matrix(f, &full_weights(f), ell);
    let ms: Vec<_> = ww.iter().map(|w| matrix(f, w, ell)).collect();
    let prod = product_table(f, ww);
    let mut proofs = vec![];
    for k in 0..ell {
        let mut bytes = vec![];
        let mut c = 0;
        for i in 0..ww[0].len() {
            for j in 0..ww[1].len() {
                for l in 0..ww[2].len() {
                    let got = linear_map(&em[k], prod[c]);
                    let want = f.mul(f.mul(ms[0][k][i], ms[1][k][j]), ms[2][k][l]);
                    assert_eq!(got, want);
                    bytes.extend_from_slice(&got.to_le_bytes());
                    c += 1;
                }
            }
        }
        proofs.push(json!({"slot":k,"coordinate_equation_coefficients":em[k].iter().map(u128::to_string).collect::<Vec<_>>(),"coefficients_checked":c,"coefficient_u128le_sha256":sha(&bytes)}));
    }
    let total: usize = ww.iter().map(Vec::len).sum();
    json!({"dimensions":ww.iter().map(Vec::len).collect::<Vec<_>>(),"independent_cubic_slots":ell,"relations":proofs,"component_socle_upper":total-ell,"argument":"Invertible block coordinate changes preserve squares; disjoint cubic slots permit at most2 of3 variables per slot; unused variables contribute at most1 each"})
}
fn quadratic(f: Field, w: &[u128], t: u128) -> Value {
    assert!(t <= f.mask());
    let em = matrix(f, &full_weights(f), f.n);
    let q: Vec<_> = (0..f.n)
        .map(|r| f.mul(1 << r, 1 << r) ^ f.mul(t, 1 << r))
        .collect();
    let mut a = vec![0; f.n];
    let mut c = 1;
    let mut tk = t;
    let mut rows = vec![];
    for k in 0..f.n {
        let mut bytes = vec![];
        for r in 0..f.n {
            let got = (0..f.n)
                .filter(|j| q[r] >> j & 1 != 0)
                .fold(0, |v, j| v ^ a[j]);
            let want = em[k][r] ^ f.mul(c, 1 << r);
            assert_eq!(got, want);
            bytes.extend_from_slice(&got.to_le_bytes());
        }
        rows.push(json!({"embedding":k,"p0_scale":c.to_string(),"original_coordinate_coefficients":a.iter().map(u128::to_string).collect::<Vec<_>>(),"abstract_product_basis_coefficients_checked":f.n,"coefficient_u128le_sha256":sha(&bytes)}));
        for r in 0..f.n {
            a[r] = em[k][r] ^ f.mul(tk, a[r]);
        }
        c = f.mul(tk, c);
        tk = f.mul(tk, tk);
    }
    assert_eq!(c, if t == 0 { 0 } else { 1 });
    // Full product ideal upper: select ell pair slots after verified Moore transforms.
    // For t !=0, P_k=c_k*P_0 modulo Q; P_0 is killed by all U_k and Y_k.
    // For t=0, Q is the invertible Frobenius map of product coordinates.
    assert!(f.n >= 3 && !w.is_empty() && w.len() <= f.n);
    if t != 0 {
        assert!(rows.iter().all(|r| r["p0_scale"].as_str().unwrap() != "0"));
    }
    json!({"target":t.to_string(),"full_width":f.n,"selector_width":w.len(),"embedding_relations":rows,"product_quotient_socle_upper":f.n,"quadratic_quotient_socle_upper":f.n,"degree_two_defect_only":t!=0,"defect_dimension_upper":if t==0{0}else{1},"annihilator_argument":"For nonzero t, all P_k are nonzero scalar multiples of h=P_0. h*U_k=h*Y_k=0 by squares. Full U transform and selector transform invertibility imply every local variable kills h. Adding products changes only degree2. Full-product quotient vanishes above degree n>=3."})
}
// Tiny explicit squarefree Macaulay matrices, independent of structural bound.
fn rank(mut rows: Vec<Vec<u64>>) -> usize {
    let mut piv: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
    for mut r in rows.drain(..) {
        loop {
            let p = r.iter().enumerate().rev().find_map(|(i, x)| {
                if *x == 0 {
                    None
                } else {
                    Some(i * 64 + 63 - x.leading_zeros() as usize)
                }
            });
            let Some(p) = p else { break };
            if let Some(q) = piv.get(&p) {
                for (a, b) in r.iter_mut().zip(q) {
                    *a ^= *b;
                }
            } else {
                piv.insert(p, r);
                break;
            }
        }
    }
    piv.len()
}
fn explicit_hilbert(n: usize, g: &[Vec<usize>], degree: usize) -> Vec<usize> {
    assert!(n <= 12);
    let mut h = vec![];
    for d in 0..=n {
        let cols: Vec<_> = (0usize..1 << n)
            .filter(|x| x.count_ones() as usize == d)
            .collect();
        let map: BTreeMap<_, _> = cols.iter().enumerate().map(|(i, m)| (*m, i)).collect();
        let mut rows = vec![];
        if d >= degree {
            for mon in (0usize..1 << n).filter(|x| x.count_ones() as usize == d - degree) {
                for poly in g {
                    let mut row = vec![0; cols.len().div_ceil(64)];
                    for term in poly {
                        if mon & term == 0 {
                            let j = map[&(mon | term)];
                            row[j / 64] ^= 1u64 << (j % 64);
                        }
                    }
                    rows.push(row);
                }
            }
        }
        h.push(cols.len() - rank(rows));
    }
    h.push(0);
    h
}
fn small_generators(f: Field, ww: &[Vec<u128>], t: Option<u128>) -> (usize, Vec<Vec<usize>>) {
    let mut terms = vec![(0usize, 1u128)];
    let mut start = 0;
    for w in ww {
        terms = terms
            .into_iter()
            .flat_map(|(mask, v)| {
                w.iter()
                    .enumerate()
                    .map(move |(i, x)| (mask | (1 << (start + i)), f.mul(v, *x)))
            })
            .collect();
        start += w.len();
    }
    let mut g = vec![vec![]; f.n];
    for (mon, mut v) in terms {
        if let Some(t) = t {
            v = f.mul(v, v) ^ f.mul(t, v);
        }
        for r in 0..f.n {
            if v >> r & 1 != 0 {
                g[r].push(mon);
            }
        }
    }
    (start, g)
}
fn controls() -> Value {
    let mut out = vec![];
    for f in [Field { n: 3, low: 3 }, Field { n: 5, low: 5 }] {
        f.certificate();
        let fw = full_weights(f);
        for ell in 1..=3 {
            let w = fw[..ell].to_vec();
            for t in [0, 1, 2, 3] {
                let p = quadratic(f, &w, t);
                let (n, g) = small_generators(f, &[fw.clone(), w.clone()], Some(t));
                let h = explicit_hilbert(n, &g, 2);
                assert_eq!(h.iter().position(|v| *v == 0), Some(f.n + 1));
                out.push(json!({"kind":"quadratic","field_degree":f.n,"ell":ell,"target":t,"hilbert":h,"dreg":f.n+1,"structural":p["quadratic_quotient_socle_upper"]}));
            }
        }
        for dims in if f.n == 3 {
            vec![[1, 1, 3], [2, 2, 3], [3, 2, 3]]
        } else {
            vec![[1, 1, 5], [2, 2, 5], [5, 1, 5]]
        } {
            let ww: Vec<_> = dims.iter().map(|n| fw[..*n].to_vec()).collect();
            let proof = cubic(f, &ww);
            let (n, g) = small_generators(f, &ww, None);
            let h = explicit_hilbert(n, &g, 3);
            let expected = n - dims.iter().min().unwrap() + 1;
            assert_eq!(h.iter().position(|v| *v == 0), Some(expected));
            out.push(json!({"kind":"cubic","field_degree":f.n,"dimensions":dims,"hilbert":h,"dreg":expected,"structural":proof["component_socle_upper"]}));
        }
    }
    let f = f83();
    let mut s = 123456789u128;
    for _ in 0..128 {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(17) & f.mask();
        let a = s;
        s = s.wrapping_mul(1442695040888963407).wrapping_add(91) & f.mask();
        assert_eq!(f.mul(a, s), f.slow(a, s));
    }
    json!({"schema":"n83.chain-controls/v1","explicit_cases":out,"field_arithmetic_cross_checks":128,"all_passed":true})
}
fn global() -> Value {
    let f = f83();
    let fw = full_weights(f);
    let mut bases = serde_json::Map::new();
    for (spike, name) in [(false, "baseline"), (true, "spike40")] {
        let w = weights(spike);
        bases.insert(name.into(),json!({"selector_transform":transform(f,&w),"first_cubic":cubic(f,&[w.clone(),w.clone(),fw.clone()]),"middle_cubic":cubic(f,&[fw.clone(),w.clone(),fw.clone()])}));
    }
    json!({"schema":"n83.chain-global/v1","field":f.certificate(),"full_transform":transform(f,&fw),"bases":bases})
}
#[derive(Clone)]
struct Block {
    start: usize,
    w: Vec<u128>,
}
fn chain_blocks(m: usize, spike: bool, k: usize) -> Vec<Block> {
    let x = |j| Block {
        start: j * 27,
        w: weights(spike),
    };
    let u = |j| Block {
        start: m * 27 + j * 83,
        w: full_weights(f83()),
    };
    if k == 0 {
        vec![x(0), x(1), u(0)]
    } else if k == m - 2 {
        vec![u(k - 1), x(k + 1)]
    } else {
        vec![u(k - 1), x(k + 1), u(k)]
    }
}
fn table_counts(v: &[u128]) -> Vec<usize> {
    let mut counts = vec![0; 83];
    for a in v {
        let mut bits = *a;
        while bits != 0 {
            let r = bits.trailing_zeros() as usize;
            counts[r] += 1;
            bits &= bits - 1;
        }
    }
    counts
}
fn variable(s: &str, n: usize) -> usize {
    let i = s
        .strip_prefix("v(")
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert!((1..=n).contains(&i));
    i - 1
}
fn check_top_line(
    line: &str,
    id: usize,
    n: usize,
    degree: usize,
    blocks: &[Block],
    table: &[u128],
    coordinate: usize,
    want: usize,
    lower_keep: &BTreeSet<usize>,
) -> Value {
    let prefix = format!("I[{id}]=");
    let expr = line
        .strip_prefix(&prefix)
        .unwrap()
        .strip_suffix(';')
        .unwrap();
    let mut prev: Option<Vec<usize>> = None;
    let mut count = 0;
    for term in expr.split('+') {
        let mon = if term == "1" {
            vec![]
        } else {
            term.split('*').map(|v| variable(v, n)).collect::<Vec<_>>()
        };
        assert!(mon.len() <= degree);
        assert!(mon.windows(2).all(|w| w[0] < w[1]));
        if let Some(ref p) = prev {
            assert!(*p < mon, "noncanonical or duplicate monomial");
        }
        prev = Some(mon.clone());
        if mon.len() == degree {
            let mut index = 0;
            for b in blocks {
                let local: Vec<_> = mon
                    .iter()
                    .filter(|v| b.start <= **v && **v < b.start + b.w.len())
                    .collect();
                assert_eq!(local.len(), 1);
                index = index * b.w.len() + (*local[0] - b.start);
            }
            assert_eq!(table[index] >> coordinate & 1, 1);
            assert!(
                !mon.iter().all(|v| lower_keep.contains(v)),
                "lower projection violated"
            );
            count += 1;
        }
    }
    assert_eq!(count, want, "missing top terms");
    assert!(count > 0);
    json!({"equation_id":id,"coordinate":coordinate,"top_degree":degree,"top_terms":count,"source_line_sha256":sha(line.as_bytes())})
}
fn suffix(input: &Value, n: usize) -> String {
    if input["target_kind"] == "natural" {
        assert!(input["witness"].is_null());
        return String::new();
    }
    assert_eq!(input["target_kind"], "planted");
    let w = input["witness"].as_array().unwrap();
    assert_eq!(w.len(), n);
    let bits: Vec<_> = w
        .iter()
        .map(|v| {
            let s = v.as_str().unwrap();
            assert!(s == "0" || s == "1");
            s
        })
        .collect();
    format!("map witness=r,{};\nif(size(witness(I))!=0){{print(\"WITNESS_FAILED\");exit;}} print(\"WITNESS_OK\");\n",bits.join(","))
}
fn certify(root: &Path, case: &Value) -> Value {
    let key = case["key"].as_str().unwrap();
    let dir = root.join("inputs").join(key);
    let bytes = fs::read(dir.join("input.json")).unwrap();
    assert_eq!(sha(&bytes), case["input_sha256"]);
    let input: Value = serde_json::from_slice(&bytes).unwrap();
    let text = fs::read_to_string(dir.join("system.sing")).unwrap();
    assert_eq!(sha(text.as_bytes()), case["source_sha256"]);
    assert_eq!(input["source_sha256"], case["source_sha256"]);
    assert_eq!(
        input["factor_base_manifest_sha256"],
        case["factor_base_manifest_sha256"]
    );
    assert_eq!(input["n"], 83);
    assert_eq!(input["ell"], 27);
    let m = input["m"].as_u64().unwrap() as usize;
    assert!((3..=9).contains(&m));
    let n = 27 * m + 83 * (m - 2);
    assert_eq!(input["n_vars"], n);
    assert!(input["family"] == "binary" || input["family"] == "koblitz");
    assert_eq!(input["field_modulus"], "z^83+z^45+z^2+z+1");
    let dataset = case["dataset"].as_str().unwrap();
    assert!(["original", "baseline", "spike40"].contains(&dataset));
    let spike = dataset == "spike40";
    if dataset != "original" {
        assert_eq!(input["arm"], dataset);
        assert_eq!(
            input["coordinate_bit_positions"],
            json!((0..27)
                .map(|i| if spike && i == 26 { 40 } else { i })
                .collect::<Vec<_>>())
        );
    }
    let t = input["target_x"].as_str().unwrap().parse::<u128>().unwrap();
    assert!(t <= f83().mask());
    let expected = 83 * (m - 1) + n;
    assert_eq!(input["input_equations"], expected);
    let keep: BTreeSet<_> = (0..27).chain(m * 27..n).collect();
    let mut lines = text.lines();
    assert_eq!(lines.next().unwrap(), format!("ring r=2,(v(1..{n})),dp;"));
    assert_eq!(lines.next(), Some("ideal I;"));
    let mut groups = vec![];
    let mut selected = vec![];
    let mut used = BTreeSet::new();
    for k in 0..m - 1 {
        let blocks = chain_blocks(m, spike, k);
        let degree = blocks.len();
        let mut table = product_table(
            f83(),
            &blocks.iter().map(|b| b.w.clone()).collect::<Vec<_>>(),
        );
        if degree == 2 {
            for a in &mut table {
                *a = f83().mul(*a, *a) ^ f83().mul(t, *a);
            }
        }
        let counts = table_counts(&table);
        let mut checks = vec![];
        for r in 0..83 {
            checks.push(check_top_line(
                lines.next().unwrap(),
                k * 83 + r + 1,
                n,
                degree,
                &blocks,
                &table,
                r,
                counts[r],
                &keep,
            ));
        }
        let desc = json!({"chain_equation":k,"kind":if degree==2{"final_quadratic"}else if k==0{"first_cubic"}else{"middle_cubic"},"blocks":blocks.iter().map(|b|json!({"start_zero_based":b.start,"width":b.w.len()})).collect::<Vec<_>>(),"coordinates":checks});
        groups.push(desc.clone());
        if k % 2 == 0 {
            for b in &blocks {
                for v in b.start..b.start + b.w.len() {
                    assert!(used.insert(v), "selected blocks overlap");
                }
            }
            selected.push(desc["chain_equation"].clone());
        }
    }
    for i in 0..n {
        assert_eq!(
            lines.next().unwrap(),
            format!("I[{}]=v({})^2-v({});", 83 * (m - 1) + i + 1, i + 1, i + 1)
        );
    }
    let remaining = lines.collect::<Vec<_>>();
    let expected_suffix = suffix(&input, n);
    assert_eq!(remaining.join("\n"), expected_suffix.trim_end_matches('\n'));
    assert_eq!(selected.len(), m / 2);
    let lower = keep.len() + 1;
    let upper = n - 27 * selected.len() + 1;
    assert!(lower <= upper);
    let quad = quadratic(f83(), &weights(spike), t);
    json!({"schema":"n83.chain-case/v1","key":key,"input":input,"source_and_all_top_forms_verified":true,"groups":groups,"selected_chain_equations_zero_based":selected,"selected_variables_zero_based":used,"global_certificate":"global.json","global_sha256":sha(&fs::read(root.join("certificates/global.json")).unwrap()),"global_base":if spike{"spike40"}else{"baseline"},"final_quadratic":quad,"lower_retained_variables_zero_based":keep,"lower_projection_verified":true,"dreg_lower_bound":lower,"dreg_upper_bound":upper,"dreg_exact":if lower==upper{Some(lower)}else{None},"status":"finite_interval","argument":"Select even chain equations on disjoint original variable blocks. Each component drops27 from square-zero socle bound; tensor products add socle bounds. Original ideal contains this subideal. Lower quotient retains first27 selectors and all intermediate coordinates."})
}
fn run(root: &Path, replay: bool) {
    let p = read(&root.join("protocol-chain.json"));
    let cases = p["extensions"]["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 84);
    let out = root.join("certificates");
    if !replay {
        fs::create_dir(&out).unwrap();
    }
    for (name, v) in [("controls", controls()), ("global", global())] {
        if replay {
            assert_eq!(read(&out.join(format!("{name}.json"))), v);
        } else {
            save(&out.join(format!("{name}.json")), &v);
        }
    }
    let mut rows = vec![];
    let mut errors = 0;
    for c in cases {
        let key = c["key"].as_str().unwrap();
        match std::panic::catch_unwind(|| certify(root, c)) {
            Ok(v) => {
                let path = out.join(format!("{key}.json"));
                if replay {
                    assert_eq!(read(&path), v);
                } else {
                    save(&path, &v);
                }
                rows.push(json!({"key":key,"arm":c["arm"],"dataset":c["dataset"],"family":v["input"]["family"],"m":v["input"]["m"],"target_kind":v["input"]["target_kind"],"source_sha256":c["source_sha256"],"factor_base_manifest_sha256":c["factor_base_manifest_sha256"],"dreg_lower_bound":v["dreg_lower_bound"],"dreg_upper_bound":v["dreg_upper_bound"],"dreg_exact":v["dreg_exact"],"selected_blocks":v["selected_chain_equations_zero_based"].as_array().unwrap().len(),"certificate":format!("{key}.json"),"certificate_sha256":sha(&fs::read(path).unwrap()),"verified":true}));
                println!("{} {key}", if replay { "replayed" } else { "certified" });
            }
            Err(_) => {
                errors += 1;
                rows.push(json!({"key":key,"arm":c["arm"],"status":"error","verified":false}));
            }
        }
        if !replay {
            save(
                &out.join(format!("partial-summary-{:03}.json", rows.len())),
                &json!({"rows":rows,"errors":errors}),
            );
        }
    }
    let summary = json!({"schema":"n83.chain-summary/v1","rows":rows,"errors":errors,"all_verified":errors==0,"scope":"Original highest-form degree bounds, not exact full-system values, LFD or base improvement"});
    if replay {
        assert_eq!(read(&out.join("summary.json")), summary);
        save(
            &root.join("replay.json"),
            &json!({"cases":84,"all_passed":errors==0,"summary_sha256":sha(&fs::read(out.join("summary.json")).unwrap())}),
        );
    } else {
        save(&out.join("summary.json"), &summary);
    }
    assert_eq!(errors, 0);
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "controls" => save(Path::new(&a[2]), &controls()),
        "run" | "replay" => run(Path::new(&a[2]), a[1] == "replay"),
        _ => panic!("run ROOT | replay ROOT | controls OUT"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_controls() {
        assert_eq!(controls()["all_passed"], true);
    }
    #[test]
    fn transforms_and_mutation() {
        let f = Field { n: 3, low: 3 };
        let m = matrix(f, &full_weights(f), 3);
        let mut inv = inverse(f, &m).unwrap();
        assert!(check_inverse(f, &m, &inv));
        inv[0][0] ^= 1;
        assert!(!check_inverse(f, &m, &inv));
        assert!(inverse(f, &[vec![1, 1], vec![1, 1]]).is_none());
    }
    #[test]
    fn disjoint_chain_selection() {
        for m in 3..=12 {
            let mut used = BTreeSet::new();
            let mut count = 0;
            for k in (0..m - 1).step_by(2) {
                count += 1;
                for b in chain_blocks(m, false, k) {
                    for v in b.start..b.start + b.w.len() {
                        assert!(used.insert(v));
                    }
                }
            }
            assert_eq!(count, m / 2);
        }
    }
    #[test]
    fn parser_rejects_bad_coefficient_and_duplicate() {
        let b = vec![
            Block {
                start: 0,
                w: vec![1],
            },
            Block {
                start: 1,
                w: vec![1],
            },
        ];
        let keep = BTreeSet::new();
        check_top_line("I[1]=v(1)*v(2);", 1, 2, 2, &b, &[1], 0, 1, &keep);
        assert!(std::panic::catch_unwind(|| check_top_line(
            "I[1]=v(1)*v(2);",
            1,
            2,
            2,
            &b,
            &[0],
            0,
            1,
            &keep
        ))
        .is_err());
        assert!(std::panic::catch_unwind(|| check_top_line(
            "I[1]=v(1)*v(2)+v(1)*v(2);",
            1,
            2,
            2,
            &b,
            &[1],
            0,
            2,
            &keep
        ))
        .is_err());
    }
    #[test]
    fn suffix_rejection() {
        assert_eq!(
            suffix(&json!({"target_kind":"natural","witness":null}), 4),
            ""
        );
        assert!(std::panic::catch_unwind(|| suffix(
            &json!({"target_kind":"planted","witness":["2"]}),
            1
        ))
        .is_err());
    }
}
