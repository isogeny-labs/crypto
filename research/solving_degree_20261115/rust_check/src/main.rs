// sd_static: static solving degree sd = delta + maxwt(supp g^{-1}) for the prime chain systems.
// No external crates. All results are [pcc:unverified].
use std::time::Instant;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn pw(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        e >>= 1;
    }
    r
}
fn inv(a: u64, p: u64) -> u64 {
    pw(a, p - 2, p)
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b) % p
}

// Tonelli-Shanks
fn sqrt_mod(n: u64, p: u64) -> Option<u64> {
    let n = n % p;
    if n == 0 {
        return Some(0);
    }
    if pw(n, (p - 1) / 2, p) != 1 {
        return None;
    }
    let mut q = p - 1;
    let mut s = 0;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while pw(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let mut m = s;
    let mut c = pw(z, q, p);
    let mut t = pw(n, q, p);
    let mut r = pw(n, (q + 1) / 2, p);
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = tt * tt % p;
            i += 1;
        }
        let b = pw(c, 1u64 << (m - i - 1), p);
        m = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }
    Some(r)
}

// Semaev S_3 as quadratic in x3: (a2, a1, a0)
fn s3c(x1: u64, x2: u64, a: u64, b: u64, p: u64) -> (u64, u64, u64) {
    let d = sub(x1, x2, p);
    let a2 = d * d % p;
    let s = (x1 + x2) % p;
    let q = x1 * x2 % p;
    let inner = (s * ((q + a) % p) % p + 2 * b) % p;
    let a1 = (p - 2 * inner % p) % p;
    let qa = sub(q, a, p);
    let a0 = sub(qa * qa % p, 4 * b % p * s % p, p);
    (a2, a1, a0)
}
fn eval_q(c: (u64, u64, u64), x: u64, p: u64) -> u64 {
    (c.0 * (x * x % p) % p + c.1 * x % p + c.2) % p
}
fn res2(a: (u64, u64, u64), b: (u64, u64, u64), p: u64) -> u64 {
    let (a2, a1, a0) = a;
    let (b2, b1, b0) = b;
    let t1 = sub(a2 * b0 % p, a0 * b2 % p, p);
    let t2 = sub(a2 * b1 % p, a1 * b2 % p, p);
    let t3 = sub(a1 * b0 % p, a0 * b1 % p, p);
    sub(t1 * t1 % p, t2 * t3 % p, p)
}

fn fft(buf: &mut [u64], tw: &[u64], p: u64) {
    let n = buf.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j |= bit;
        if i < j {
            buf.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let half = len / 2;
        let step = n / len;
        let mut i = 0;
        while i < n {
            for jj in 0..half {
                let u = buf[i + jj];
                let v = buf[i + jj + half] * tw[jj * step] % p;
                let s = u + v;
                buf[i + jj] = if s >= p { s - p } else { s };
                buf[i + jj + half] = if u >= v { u - v } else { u + p - v };
            }
            i += len;
        }
        len <<= 1;
    }
}

// y_e = sum_x data_x r^{x e} along each axis; r of order n.
fn transform(data: &mut [u32], n: usize, m: usize, r: u64, p: u64) {
    let tw: Vec<u64> = {
        let mut v = Vec::with_capacity(n / 2 + 1);
        let mut c = 1u64;
        for _ in 0..(n / 2).max(1) {
            v.push(c);
            c = c * r % p;
        }
        v
    };
    let total = data.len();
    let mut buf = vec![0u64; n];
    for axis in 0..m {
        let stride = n.pow((m - 1 - axis) as u32);
        for base in 0..total {
            // base must have digit 'axis' equal to 0
            if (base / stride) % n != 0 {
                continue;
            }
            for i in 0..n {
                buf[i] = data[base + i * stride] as u64;
            }
            fft(&mut buf, &tw, p);
            for i in 0..n {
                data[base + i * stride] = buf[i] as u32;
            }
        }
    }
}

fn naive_dft(x: &[u64], r: u64, p: u64) -> Vec<u64> {
    let n = x.len();
    (0..n)
        .map(|e| {
            let mut s = 0;
            for j in 0..n {
                s = (s + x[j] * pw(r, (j * e) as u64, p)) % p;
            }
            s
        })
        .collect()
}

fn batch_inv(v: &mut [u32], p: u64) {
    for ch in v.chunks_mut(4096) {
        let mut pre = Vec::with_capacity(ch.len());
        let mut acc = 1u64;
        for &x in ch.iter() {
            acc = acc * x as u64 % p;
            pre.push(acc);
        }
        let mut ia = inv(acc, p);
        for i in (0..ch.len()).rev() {
            let x = ch[i] as u64;
            let prev = if i > 0 { pre[i - 1] } else { 1 };
            ch[i] = (ia * prev % p) as u32;
            ia = ia * x % p;
        }
    }
}

fn weight(mut e: usize, n: usize, m: usize) -> usize {
    let mut w = 0;
    for _ in 0..m {
        w += (e % n).count_ones() as usize;
        e /= n;
    }
    w
}

struct Out {
    unit: bool,
    zeros: usize,
    ginv: Vec<u32>,
    gc: Vec<u32>, // coefficients of g (empty if not computed)
    maxwt: usize,
    gweight: Option<usize>,
    top_zero: bool,
    zcw: Vec<usize>,
}

fn find_w(p: u64, k: u32, rng: &mut Rng) -> u64 {
    loop {
        let h = 2 + rng.below(p - 3);
        let w = pw(h, (p - 1) >> k, p);
        if pw(w, 1u64 << (k - 1), p) == p - 1 {
            return w;
        }
    }
}

fn solve(p: u64, k: u32, m: usize, a: u64, b: u64, t: u64, w: u64, want_gc: bool) -> Out {
    let n = 1usize << k;
    let total = n.pow(m as u32);
    let pws: Vec<u64> = (0..n).map(|j| pw(w, j as u64, p)).collect();
    let mut g = vec![0u32; total];
    if m == 2 {
        for i in 0..n {
            for j in 0..n {
                g[i * n + j] = eval_q(s3c(pws[i], pws[j], a, b, p), t, p) as u32;
            }
        }
    } else {
        let right: Vec<(u64, u64, u64)> = (0..n).map(|l| s3c(pws[l], t, a, b, p)).collect();
        for i in 0..n {
            for j in 0..n {
                let left = s3c(pws[i], pws[j], a, b, p);
                for l in 0..n {
                    g[(i * n + j) * n + l] = res2(left, right[l], p) as u32;
                }
            }
        }
    }
    let zeros = g.iter().filter(|&&x| x == 0).count();
    if zeros > 0 {
        return Out { unit: false, zeros, ginv: vec![], gc: vec![], maxwt: 0, gweight: None, top_zero: false, zcw: vec![] };
    }
    let winv = inv(w, p);
    let scale = inv(pw(n as u64, m as u64, p), p);
    let finish = |d: &mut Vec<u32>| {
        transform(d, n, m, winv, p);
        for x in d.iter_mut() {
            *x = (*x as u64 * scale % p) as u32;
        }
    };
    let mut gc = vec![];
    let mut gweight = None;
    if want_gc {
        gc = g.clone();
        finish(&mut gc);
        gweight = Some((0..total).filter(|&e| gc[e] != 0).map(|e| weight(e, n, m)).max().unwrap_or(0));
    }
    batch_inv(&mut g, p);
    finish(&mut g);
    let mut maxwt = 0;
    let mut zcw = vec![0usize; m * k as usize + 1];
    for e in 0..total {
        let wt = weight(e, n, m);
        if g[e] != 0 {
            maxwt = maxwt.max(wt);
        } else {
            zcw[wt] += 1;
        }
    }
    let top_zero = g[total - 1] == 0;
    Out { unit: true, zeros: 0, ginv: g, gc, maxwt, gweight, top_zero, zcw }
}

fn json(p: u64, k: u32, m: usize, a: u64, b: u64, t: u64, w: u64, o: &Out, ms: u128) -> String {
    if !o.unit {
        return format!(
            "{{\"p\":{},\"k\":{},\"m\":{},\"w\":{},\"a\":{},\"b\":{},\"t\":{},\"unit\":false,\"note\":\"proper, |Z| = {}\",\"Z\":{},\"ms\":{}}}",
            p, k, m, w, a, b, t, o.zeros, o.zeros, ms
        );
    }
    let delta = if m == 2 { 2 } else { o.gweight.unwrap() };
    let zs: Vec<String> = o.zcw.iter().map(|x| x.to_string()).collect();
    let gw = match o.gweight {
        Some(x) => x.to_string(),
        None => "null".into(),
    };
    let n = 1usize << k;
    let coeffs = if n.pow(m as u32) <= 16 {
        let v: Vec<String> = o.ginv.iter().map(|x| x.to_string()).collect();
        format!(",\"ginv_coeffs\":[{}]", v.join(","))
    } else {
        String::new()
    };
    format!(
        "{{\"p\":{},\"k\":{},\"m\":{},\"w\":{},\"a\":{},\"b\":{},\"t\":{},\"unit\":true,\"maxwt\":{},\"delta\":{},\"gweight\":{},\"sd_static\":{},\"top_coeff_zero\":{},\"zero_count_by_weight\":[{}]{},\"ms\":{}}}",
        p, k, m, w, a, b, t, o.maxwt, delta, gw, delta + o.maxwt, o.top_zero, zs.join(","), coeffs, ms
    )
}

// ---------- group law (self-test) ----------
fn padd(p1: (u64, u64), p2: (u64, u64), a: u64, p: u64) -> Option<(u64, u64)> {
    let lam = if p1.0 == p2.0 {
        if (p1.1 + p2.1) % p == 0 {
            return None;
        }
        (3 * (p1.0 * p1.0 % p) + a) % p * inv(2 * p1.1 % p, p) % p
    } else {
        sub(p2.1, p1.1, p) * inv(sub(p2.0, p1.0, p), p) % p
    };
    let x3 = sub(sub(lam * lam % p, p1.0, p), p2.0, p);
    let y3 = sub(lam * sub(p1.0, x3, p) % p, p1.1, p);
    Some((x3, y3))
}
fn rpoint(a: u64, b: u64, p: u64, rng: &mut Rng) -> (u64, u64) {
    loop {
        let x = rng.below(p);
        let r = (pw(x, 3, p) + a * x + b) % p;
        if let Some(y) = sqrt_mod(r, p) {
            if y != 0 {
                return (x, y);
            }
        }
    }
}

fn selftest() {
    let mut rng = Rng(12345);
    // 1. S_3 / S_4 vs group law
    for &p in &[12289u64, 97, 2147483489u64 / 2 * 0 + 12289] {
        let (a, b) = (17 % p, 72 % p);
        let (mut ok3, mut bad3, mut ok4, mut bad4, mut spur) = (0, 0, 0, 0, 0);
        for _ in 0..2000 {
            let p1 = rpoint(a, b, p, &mut rng);
            let p2 = rpoint(a, b, p, &mut rng);
            if p1.0 == p2.0 {
                continue;
            }
            let c = s3c(p1.0, p2.0, a, b, p);
            let n2 = (p2.0, (p - p2.1) % p);
            for q in [padd(p1, p2, a, p), padd(p1, n2, a, p)] {
                if let Some(q) = q {
                    if eval_q(c, q.0, p) == 0 { ok3 += 1 } else { bad3 += 1 }
                }
            }
            let rx = rng.below(p);
            if eval_q(c, rx, p) == 0 { spur += 1 }
            let p3 = rpoint(a, b, p, &mut rng);
            if let Some(s12) = padd(p1, p2, a, p) {
                if let Some(q) = padd(s12, p3, a, p) {
                    let r = res2(s3c(p1.0, p2.0, a, b, p), s3c(p3.0, q.0, a, b, p), p);
                    if r == 0 { ok4 += 1 } else { bad4 += 1 }
                }
            }
        }
        println!("selftest group-law p={} a={} b={}: S3 vanishes ok={} bad={} (random-x zeros {}); S4 ok={} bad={}", p, a, b, ok3, bad3, spur, ok4, bad4);
    }
    // 2. FFT vs naive
    {
        let p = 12289u64;
        let k = 4;
        let n = 1usize << k;
        let w = find_w(p, k, &mut rng);
        let x: Vec<u64> = (0..n).map(|_| rng.below(p)).collect();
        let nv = naive_dft(&x, w, p);
        let tw: Vec<u64> = (0..n / 2).map(|j| pw(w, j as u64, p)).collect();
        let mut y = x.clone();
        fft(&mut y, &tw, p);
        println!("selftest fft==naive (n=16): {}", y == nv);
    }
    // 3. k=2, m=2 and m=3 (k=2) g*g^{-1}=1 by direct group-algebra convolution
    for &(m, p) in &[(2usize, 17u64), (2, 12289), (3, 17), (3, 12289)] {
        let k = 2u32;
        let n = 1usize << k;
        let (mut tested, mut good) = (0, 0);
        for _ in 0..20 {
            let a = rng.below(p);
            let b = rng.below(p);
            let t = rng.below(p);
            if (4 * pw(a, 3, p) + 27 * b % p * b) % p == 0 {
                continue;
            }
            let w = find_w(p, k, &mut rng);
            let o = solve(p, k, m, a, b, t, w, true);
            if !o.unit {
                continue;
            }
            let total = n.pow(m as u32);
            let mut prod = vec![0u64; total];
            for e1 in 0..total {
                if o.gc[e1] == 0 {
                    continue;
                }
                for e2 in 0..total {
                    // add exponents coordinatewise mod n
                    let (mut x, mut y, mut mul, mut s) = (e1, e2, 1usize, 0usize);
                    for _ in 0..m {
                        s += ((x % n + y % n) % n) * mul;
                        x /= n;
                        y /= n;
                        mul *= n;
                    }
                    prod[s] = (prod[s] + o.gc[e1] as u64 * o.ginv[e2] as u64) % p;
                }
            }
            tested += 1;
            if prod[0] == 1 && prod[1..].iter().all(|&v| v == 0) {
                good += 1;
            }
        }
        println!("selftest k=2 m={} p={}: g*g^-1==1 in K[G] on {}/{} unit instances", m, p, good, tested);
    }
    // 4. k=1 reproduction
    {
        let o = solve(101, 1, 2, 17, 72, 97, 100, true);
        println!("selftest k=1 p=101 a=17 b=72 t=97: unit={} ginv(0,0),(0,1),(1,0),(1,1)={:?} maxwt={} sd={}", o.unit, o.ginv, o.maxwt, 2 + o.maxwt);
    }
}

fn selftest_w() {
    let mut rng = Rng(777);
    let p = 12289u64;
    for &(k, m) in &[(3u32, 2usize), (5, 2), (2, 3), (3, 3)] {
        let (mut tested, mut same) = (0, 0);
        while tested < 5 {
            let a = rng.below(p);
            let b = rng.below(p);
            let t = rng.below(p);
            if (4 * pw(a, 3, p) + 27 * (b * b % p)) % p == 0 {
                continue;
            }
            let w1 = find_w(p, k, &mut rng);
            let s = 2 * rng.below(1u64 << (k - 1)) + 1; // odd, generates a different w
            let w2 = pw(w1, s, p);
            let o1 = solve(p, k, m, a, b, t, w1, true);
            let o2 = solve(p, k, m, a, b, t, w2, true);
            if o1.unit != o2.unit {
                println!("MISMATCH unit flag");
                continue;
            }
            if !o1.unit {
                continue;
            }
            tested += 1;
            if o1.ginv == o2.ginv && o1.gc == o2.gc && o1.maxwt == o2.maxwt {
                same += 1;
            }
        }
        println!("selftest-w p={} k={} m={}: full g^-1 and g coefficient arrays equal under w -> w^s (s odd) on {}/{} instances", p, k, m, same, tested);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut get = std::collections::HashMap::new();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--selftest-w" {
            selftest_w();
            return;
        }
        if args[i] == "--selftest" {
            selftest();
            return;
        }
        if args[i].starts_with("--") && i + 1 < args.len() {
            get.insert(args[i][2..].to_string(), args[i + 1].clone());
            i += 2;
        } else {
            eprintln!("bad arg {}", args[i]);
            std::process::exit(2);
        }
    }
    let u = |k: &str, d: Option<u64>| -> u64 {
        match get.get(k) {
            Some(v) => v.parse().expect("number"),
            None => d.unwrap_or_else(|| {
                eprintln!("missing --{}", k);
                std::process::exit(2)
            }),
        }
    };
    let m = u("m", Some(2)) as usize;
    let k = u("k", None) as u32;
    let p = u("p", None);
    assert!(p < (1 << 31) && (p - 1) % (1u64 << k) == 0 && (m == 2 || m == 3));
    let mut rng = Rng(u("seed", Some(1)));
    let given = get.contains_key("a");
    let count = if given { 1 } else { u("count", Some(1)) };
    for _ in 0..count {
        let (a, b, t) = if given {
            (u("a", None) % p, u("b", None) % p, u("t", None) % p)
        } else {
            loop {
                let a = rng.below(p);
                let b = rng.below(p);
                let t = rng.below(p);
                if (4 * pw(a, 3, p) + 27 * (b * b % p)) % p != 0 {
                    break (a, b, t);
                }
            }
        };
        let w = if get.contains_key("w") { u("w", None) } else { find_w(p, k, &mut rng) };
        let st = Instant::now();
        let want_gc = m == 3 || k <= 10;
        let o = solve(p, k, m, a, b, t, w, want_gc);
        println!("{}", json(p, k, m, a, b, t, w, &o, st.elapsed().as_millis()));
    }
}
