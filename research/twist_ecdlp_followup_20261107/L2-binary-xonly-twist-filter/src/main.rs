mod analyze;
mod cells;
mod controls;
mod full;
mod rho;
mod gf2;
mod model;
mod solve;
mod util;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use util::*;

pub fn par_map<T: Send + 'static, F: Fn(usize) -> T + Send + Sync + 'static>(n: usize, threads: usize, f: F) -> Vec<T> {
    let f = Arc::new(f);
    let next = Arc::new(AtomicUsize::new(0));
    let out: Arc<Mutex<Vec<Option<T>>>> = Arc::new(Mutex::new((0..n).map(|_| None).collect()));
    let mut hs = vec![];
    for _ in 0..threads.max(1) {
        let (f, next, out) = (f.clone(), next.clone(), out.clone());
        hs.push(std::thread::spawn(move || loop {
            let i = next.fetch_add(1, Ordering::SeqCst);
            if i >= n {
                break;
            }
            let r = f(i);
            out.lock().unwrap()[i] = Some(r);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    let mut g = out.lock().unwrap();
    g.iter_mut().map(|o| o.take().unwrap()).collect()
}

fn threads() -> usize {
    std::env::var("L2THREADS").ok().and_then(|v| v.parse().ok()).unwrap_or(3)
}
fn scale() -> f64 {
    std::env::var("L2SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0)
}
fn sc(n: usize) -> usize {
    ((n as f64 * scale()).round() as usize).max(1)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = std::env::var("L2DIR").unwrap_or_else(|_| ".".to_string());
    let (bc, pc) = read_curves(&dir);
    let bc = Arc::new(bc);
    let pc = Arc::new(pc);
    std::fs::create_dir_all(format!("{}/results/samples", dir)).unwrap();
    let nth = threads();
    match args.get(1).map(|s| s.as_str()) {
        Some("arms") => {
            // arms <B|P> <m> <k>
            let f = args[2].clone();
            let m: usize = args[3].parse().unwrap();
            let k: u32 = args[4].parse().unwrap();
            let t0 = std::time::Instant::now();
            // registered sizes
            let (ncurves, nt) = if f == "B" {
                match (m, k) {
                    (2, 17) | (2, 19) | (2, 23) | (3, 17) | (3, 19) => (20, 1000),
                    (2, 29) => (20, 500),
                    (2, 31) => (20, 300),
                    (3, 23) => (20, 400),
                    (3, 29) => (10, 200),
                    _ => panic!("cell not registered"),
                }
            } else {
                match (m, k) {
                    (2, 17) | (2, 19) | (2, 23) | (3, 17) | (3, 19) => (10, 1000),
                    (2, 29) => (10, 500),
                    (2, 31) => (10, 300),
                    (3, 23) => (10, 400),
                    _ => panic!("cell not registered"),
                }
            };
            let ncurves = if std::env::var("L2NCURVES").is_ok() { std::env::var("L2NCURVES").unwrap().parse().unwrap() } else { ncurves };
            let nt = sc(nt);
            let census_n: usize = if f == "B" { std::env::var("L2CENSUS").ok().and_then(|v| v.parse().ok()).unwrap_or(200) } else { std::env::var("L2PCENSUS").ok().and_then(|v| v.parse().ok()).unwrap_or(50) };
            let (bc2, pc2, f2) = (bc.clone(), pc.clone(), f.clone());
            let outs = par_map(ncurves, nth, move |ci| {
                if f2 == "B" {
                    let c = bc2.iter().find(|c| c.k == k && c.idx == ci).unwrap().clone();
                    cells::binary_job(m, &c, ci, nt, census_n)
                } else {
                    let c = pc2.iter().find(|c| c.k == k && c.idx == ci).unwrap().clone();
                    cells::prime_job(m, &c, ci, nt, census_n)
                }
            });
            let recs: Vec<Rec> = outs.iter().map(|o| o.rec.clone()).collect();
            write_tsv(&format!("{}/results/arms_{}_m{}_k{}.tsv", dir, f, m, k), &recs);
            let mut sm = String::new();
            for o in &outs {
                for l in &o.samples {
                    sm.push_str(l);
                    sm.push('\n');
                }
            }
            std::fs::write(format!("{}/results/samples/{}_m{}_k{}.txt", dir, f, m, k), sm).unwrap();
            eprintln!("arms {} m={} k={} done in {:?}", f, m, k, t0.elapsed());
        }
        Some("controls") => {
            let t0 = std::time::Instant::now();
            let ra = controls::pca(&dir, &bc, &pc);
            write_tsv(&format!("{}/results/pca.tsv", dir), &ra);
            // PC-B
            let mut jobs = vec![];
            for field in 0..2u32 {
                for &(m, nt) in &[(2usize, 1000usize), (3, 300)] {
                    for &k in &[17u32, 19, 23] {
                        for ci in 0..10usize {
                            jobs.push((field, m, nt, k, ci));
                        }
                    }
                }
            }
            let (b2, p2) = (bc.clone(), pc.clone());
            let jb = jobs.clone();
            let outs = par_map(jobs.len(), nth, move |i| {
                let (field, m, nt, k, ci) = jb[i];
                controls::pcb_job(field, m, k, ci, sc(nt), &b2, &p2)
            });
            write_tsv(&format!("{}/results/pcb.tsv", dir), &outs.iter().map(|o| o.0.clone()).collect::<Vec<_>>());
            let mut sm = String::new();
            for o in &outs {
                for l in &o.1 {
                    sm.push_str(l);
                    sm.push('\n');
                }
            }
            std::fs::write(format!("{}/results/samples/pcb.txt", dir), sm).unwrap();
            let rc = controls::pcc(&bc, &pc);
            write_tsv(&format!("{}/results/pcc.tsv", dir), &rc);
            let mut jobs = vec![];
            for field in 0..2u32 {
                for &k in &[17u32, 19, 23] {
                    for ci in 0..10usize {
                        jobs.push((field, k, ci));
                    }
                }
            }
            let (b2, p2) = (bc.clone(), pc.clone());
            let jb = jobs.clone();
            let outs = par_map(jobs.len(), nth, move |i| {
                let (field, k, ci) = jb[i];
                controls::pcd_job(field, k, ci, &b2, &p2)
            });
            write_tsv(&format!("{}/results/pcd.tsv", dir), &outs);
            eprintln!("controls done in {:?}", t0.elapsed());
        }
        Some("rho") => {
            let k: u32 = args[2].parse().unwrap();
            let b2 = bc.clone();
            let ninst = sc(50);
            let outs = par_map(10, nth, move |ci| {
                let c = b2.iter().find(|c| c.k == k && c.idx == ci).unwrap().clone();
                rho::rho_job(&c, ci, ninst)
            });
            write_tsv(&format!("{}/results/rho_k{}.tsv", dir, k), &outs);
        }
        Some("full") => {
            // full <m> <k> <ncurves> <ninst>
            let m: usize = args[2].parse().unwrap();
            let k: u32 = args[3].parse().unwrap();
            let ncur: usize = args[4].parse().unwrap();
            let ninst: usize = args[5].parse().unwrap();
            let b2 = bc.clone();
            let outs = par_map(ncur * ninst, nth, move |i| {
                let (ci, inst) = (i / ninst, i % ninst);
                let c = b2.iter().find(|c| c.k == k && c.idx == ci).unwrap().clone();
                full::full_solve(m, &c, ci, inst)
            });
            write_tsv(&format!("{}/results/full_m{}_k{}.tsv", dir, m, k), &outs);
        }
        Some("analyze") => {
            analyze::analyze(&dir);
        }
        Some("test") => {
            let m: usize = args[2].parse().unwrap();
            let k: u32 = args[3].parse().unwrap();
            let nt: usize = args[4].parse().unwrap();
            let which = args[5].as_str();
            let t0 = std::time::Instant::now();
            let o = if which == "B" {
                let c = bc.iter().find(|c| c.k == k && c.idx == 0).unwrap().clone();
                cells::binary_job(m, &c, 0, nt, 3)
            } else {
                let c = pc.iter().find(|c| c.k == k && c.idx == 0).unwrap().clone();
                cells::prime_job(m, &c, 0, nt, 3)
            };
            for (n, v) in &o.rec.0 {
                println!("{} {}", n, fmtv(*v));
            }
            println!("elapsed {:?}", t0.elapsed());
        }
        _ => eprintln!("usage: arms|controls|rho|full|test"),
    }
}
