// p1_holo.rs -- pre-registered probe for proposal P1 (HOLO-ORBIT).
//
// STATUS: NOT COMPILED, NOT RUN.  The authoring session had no execution tool
// (no shell).  Everything below is a specification in code form.  [pcc:unverified]
// Language: Rust, per AGENTS.md "Implementation language: no Python".
// Thin orchestration only:  rustc -O p1_holo.rs -o p1_holo && ./p1_holo
//
// Question: does u_i = x(iP) (i = 1..M, M = N-1) satisfy a linear recurrence
//   sum_{a=0..d} c_a(i) * u_{i+a} = 0,   deg c_a <= r,
// with unknown count U = (d+1)(r+1) much smaller than the number of equations
// E = M - d?  For a sequence without such structure a nontrivial kernel appears
// only when U > E.  We test at U ~ E/2 and report the kernel dimension.
//
// Positive control: u_i = i! mod p      -> kernel >= 1 at (d,r) = (1,1).
// Negative control: pseudo-random u_i   -> kernel 0 at U ~ E/2.
// Requires p = 3 mod 4 and p < 2^31 (plain u64 products do not overflow).

fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 { r = r * a % p; }
        a = a * a % p;
        e >>= 1;
    }
    r
}
fn inv(a: u64, p: u64) -> u64 { powm(a, p - 2, p) }
fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut i = 2;
    while i * i <= n { if n % i == 0 { return false; } i += 1; }
    true
}

type Pt = Option<(u64, u64)>;
fn add(p1: Pt, p2: Pt, a: u64, p: u64) -> Pt {
    match (p1, p2) {
        (None, q) => q,
        (q, None) => q,
        (Some((x1, y1)), Some((x2, y2))) => {
            let l;
            if x1 == x2 {
                if (y1 + y2) % p == 0 { return None; }
                let num = (3 * (x1 * x1 % p) + a) % p;
                l = num * inv(2 * y1 % p, p) % p;
            } else {
                l = (y2 + p - y1) % p * inv((x2 + p - x1) % p, p) % p;
            }
            let x3 = ((l * l) % p + 2 * p - x1 - x2) % p;
            let y3 = (l * ((x1 + p - x3) % p) % p + p - y1) % p;
            Some((x3, y3))
        }
    }
}

/// #E(F_p) for y^2 = x^3 + a x + b (naive Legendre sum, p small).
fn count_points(a: u64, b: u64, p: u64) -> u64 {
    let mut n = p + 1;
    for x in 0..p {
        let f = (x * x % p * x + a * x + b) % p;
        let c = powm(f, (p - 1) / 2, p);
        if c == 1 { n += 1; } else if c == p - 1 { n -= 1; }
    }
    n
}

fn some_point(a: u64, b: u64, p: u64) -> Option<(u64, u64)> {
    for x in 0..p {
        let f = (x * x % p * x + a * x + b) % p;
        if powm(f, (p - 1) / 2, p) == 1 {
            let y = powm(f, (p + 1) / 4, p);
            if y * y % p == f { return Some((x, y)); }
        }
    }
    None
}

/// dimension of the kernel of the E x U matrix of the order-d, degree-r guess.
fn kernel_dim(u: &[u64], d: usize, r: usize, p: u64) -> usize {
    let m = u.len();                // u[i-1] = u_i
    let e = m - d;                  // rows i = 1..=e
    let cols = (d + 1) * (r + 1);
    let mut mat = vec![vec![0u64; cols]; e];
    for row in 0..e {
        let i = (row as u64 + 1) % p;
        let mut pw = 1u64;
        for j in 0..=r {
            for a in 0..=d {
                mat[row][a * (r + 1) + j] = pw * u[row + a] % p;
            }
            pw = pw * i % p;
        }
    }
    // Gaussian elimination mod p, rank only.
    let mut rank = 0usize;
    for c in 0..cols {
        let mut piv = None;
        for rr in rank..e { if mat[rr][c] != 0 { piv = Some(rr); break; } }
        let Some(pr) = piv else { continue };
        mat.swap(rank, pr);
        let iv = inv(mat[rank][c], p);
        for rr in (rank + 1)..e {
            if mat[rr][c] != 0 {
                let f = mat[rr][c] * iv % p;
                for cc in c..cols {
                    let t = f * mat[rank][cc] % p;
                    mat[rr][cc] = (mat[rr][cc] + p - t) % p;
                }
            }
        }
        rank += 1;
        if rank == e { break; }
    }
    cols - rank
}

fn probe(name: &str, u: &[u64], p: u64) {
    let m = u.len();
    for &d in &[1usize, 2, 3, 4, 6, 8, 12, 16] {
        let e = m - d;
        let r = ((e / 2) / (d + 1)).saturating_sub(1);   // U <= E/2
        let kd = kernel_dim(u, d, r, p);
        println!("{name}: d={d} r={r} U={} E={e} kernel_dim={kd}", (d + 1) * (r + 1));
    }
}

fn main() {
    // Controls on one prime.
    let p0: u64 = 1999 + 2; // adjust until prime and = 3 mod 4 (checked below)
    let mut p = p0;
    while !(is_prime(p) && p % 4 == 3) { p += 1; }
    let m = 900usize;
    let fact: Vec<u64> = {
        let mut v = Vec::with_capacity(m);
        let mut f = 1u64;
        for i in 1..=m as u64 { f = f * (i % p) % p; v.push(f); }
        v
    };
    println!("positive control (i! mod {p}), expect kernel>=1 at d=1,r=1:");
    println!("  d=1 r=1 kernel_dim={}", kernel_dim(&fact, 1, 1, p));
    let mut s = 0x9E3779B97F4A7C15u64;
    let rnd: Vec<u64> = (0..m).map(|_| { s ^= s << 13; s ^= s >> 7; s ^= s << 17; s % p }).collect();
    probe("negative control (xorshift, seed fixed)", &rnd, p);

    // Curves: first five (a,b) with prime N in [900,2000] over primes p = 3 mod 4 near 2^10..2^11.
    let mut found = 0;
    let mut pp = 1009u64;
    'outer: while pp < 2100 && found < 5 {
        pp += 1;
        if !(is_prime(pp) && pp % 4 == 3) { continue; }
        for a in 1..20u64 {
            for b in 1..20u64 {
                if (4 * a * a % pp * a + 27 * b * b) % pp == 0 { continue; }
                let n = count_points(a, b, pp);
                if !(is_prime(n) && n >= 900 && n <= 2200) { continue; }
                let Some(g) = some_point(a, b, pp) else { continue };
                let mut u = Vec::new();
                let mut q: Pt = Some(g);
                for _ in 1..n { // i = 1..N-1
                    if let Some((x, _)) = q { u.push(x); } else { break; }
                    q = add(q, Some(g), a, pp);
                }
                if (u.len() as u64) != n - 1 { continue; }
                println!("curve p={pp} a={a} b={b} N={n}");
                probe("  orbit x(iP)", &u, pp);
                found += 1;
                continue 'outer;
            }
        }
    }
}
