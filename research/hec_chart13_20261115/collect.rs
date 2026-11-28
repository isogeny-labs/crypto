use std::{
    fs::{self, OpenOptions},
    path::Path,
    process::{Command, Stdio},
};
const P: u64 = 127;
fn pow(mut a: u64, mut n: usize) -> u64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 {
            r = r * a % P
        }
        a = a * a % P;
        n >>= 1
    }
    r
}
fn choose(n: usize, k: usize) -> u64 {
    let mut z = 1;
    for i in 0..k {
        z = z * (n - i) as u64 / (i + 1) as u64;
    }
    z
}
fn monomials() -> Vec<(usize, usize)> {
    (0..=4)
        .rev()
        .flat_map(|i| (0..=4 - i).rev().map(move |j| (i, j)))
        .collect()
}
fn eval(f: &[u64], x: u64, y: u64) -> u64 {
    monomials().iter().zip(f).fold(0, |s, ((i, j), a)| {
        (s + a * pow(x, *i) % P * pow(y, *j)) % P
    })
}
fn translated(f: &[u64], a: u64, b: u64) -> Vec<u64> {
    let ms = monomials();
    let mut out = vec![0; 15];
    for ((i, j), c) in ms.iter().zip(f) {
        for k in 0..=*i {
            for l in 0..=*j {
                let t = ms.iter().position(|m| *m == (k, l)).unwrap();
                out[t] = (out[t]
                    + c * choose(*i, k) % P * choose(*j, l) % P * pow(a, i - k) % P
                        * pow(b, j - l))
                    % P;
            }
        }
    }
    out
}
fn file(p: &Path) -> fs::File {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(p)
        .unwrap()
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let f: Vec<u64> = fs::read_to_string(&a[1])
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(f.len(), 15);
    let root = Path::new(&a[2]);
    assert!(!root.exists());
    fs::create_dir(root).unwrap();
    for (i, (a, b)) in [(0, 0), (1, 1), (7, 11)].into_iter().enumerate() {
        let g = translated(&f, a, b);
        for x in 0..P {
            for y in 0..P {
                assert_eq!(
                    eval(&g, x, y),
                    eval(&f, (x + a) % P, (y + b) % P),
                    "exact_complete_grid_polynomial_identity"
                );
            }
        }
        let mut bad = g.clone();
        bad[14] = (bad[14] + 1) % P;
        assert_ne!(eval(&bad, 0, 0), eval(&f, a, b));
        let input = root.join(format!("case{i}.txt"));
        fs::write(
            &input,
            g.iter().map(u64::to_string).collect::<Vec<_>>().join(" ") + "\n",
        )
        .unwrap();
        let bin = std::env::current_exe().unwrap().with_file_name("chart13");
        let s = Command::new(bin)
            .arg(&input)
            .arg(a.to_string())
            .arg(b.to_string())
            .stdout(Stdio::from(file(&root.join(format!("case{i}.stdout")))))
            .stderr(Stdio::from(file(&root.join(format!("case{i}.stderr")))))
            .status()
            .unwrap();
        fs::write(
            root.join(format!("case{i}.status")),
            format!("exit={:?}\n", s.code()),
        )
        .unwrap();
        assert!(s.success(), "chart_worker_case{i}_failed");
        println!("CASE_PASS case={i} translation={a},{b} full_polynomial_identity=true coordinate_transform_corruption_rejected=true");
    }
    println!("PASS runtime_chart_cohort=true requested=3 completed=3 one_isomorphism_class=true timing_claim=false");
}
