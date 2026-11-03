// Appended to the unchanged native cubic-field arithmetic and rho implementation.
// The solver reader accepts exactly nine public lines; it has no scalar input.
fn public_source(path: &str) -> (Source, EP, EP, Vec<String>) {
    let data = std::fs::read_to_string(path).expect("public fixture");
    let lines: Vec<String> = data.lines().map(str::to_owned).collect();
    assert_eq!(
        lines.len(),
        9,
        "exactly nine public lines; no planted scalar"
    );
    let p: u64 = lines[0].parse().unwrap();
    let r: u64 = lines[1].parse().unwrap();
    assert!(p > 13 && p <= 4096 && is_prime(p) && is_prime(r));
    assert!(p % 3 == 1 && r % 9 == 4);
    let modulus = coefficients(&lines[3], p);
    assert_eq!(modulus[3], 1);
    let curve: Vec<_> = lines[4].split('(').collect();
    let aa = curve[1].split(')').next().unwrap();
    let bb = curve[2].split(')').next().unwrap();
    let cv = |s: &str| {
        let c = coefficients(s, p);
        F3([c[0], c[1], c[2]])
    };
    let ec = Source {
        f: Fq(p),
        modulus: [modulus[0], modulus[1], modulus[2]],
        a: cv(aa),
        b: cv(bb),
        r,
    };
    let z = F3([0, 1, 0]);
    let q = p.pow(3);
    assert!(
        ec.powf(z, q) == z && ec.powf(z, p) != z,
        "cubic modulus is not irreducible"
    );
    let a3 = ec.mulf(ec.mulf(ec.a, ec.a), ec.a);
    let b2 = ec.mulf(ec.b, ec.b);
    let denominator = ec.addf(ec.mulf(ec.constant(4), a3), ec.mulf(ec.constant(27), b2));
    assert_ne!(denominator, F3([0; 3]), "singular source");
    let j = ec.divf(
        ec.mulf(ec.constant(1728), ec.mulf(ec.constant(4), a3)),
        denominator,
    );
    assert_ne!(ec.powf(j, p), j, "subfield source");
    let mut bound = (4 * q).isqrt();
    if bound * bound < 4 * q {
        bound += 1;
    }
    assert!(r >= q + 1 - bound && r <= q + 1 + bound && 2 * r > q + 1 + bound);
    assert!(
        (q as i128 + 1 - r as i128) % p as i128 != 0,
        "ordinary source required"
    );
    let point = EP {
        x: cv(&lines[5]),
        y: cv(&lines[6]),
        inf: false,
    };
    let target = EP {
        x: cv(&lines[7]),
        y: cv(&lines[8]),
        inf: false,
    };
    assert!(
        ec.on_curve(point)
            && ec.on_curve(target)
            && ec.mulp(r, point).inf
            && ec.mulp(r, target).inf
    );
    (ec, point, target, lines)
}
fn field_text(a: F3) -> String {
    format!("{}*z^2 + {}*z + {}", a.0[2], a.0[1], a.0[0])
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 2, "source_public solve|generate|verify ...");
    match args[1].as_str() {
        "solve" => {
            assert_eq!(args.len(), 5, "solve PUBLIC_FIXTURE rho|rho-neg SEED");
            let neg = match args[3].as_str() {
                "rho" => false,
                "rho-neg" => true,
                _ => panic!("source arm"),
            };
            let seed: u64 = args[4].parse().unwrap();
            let before = snap();
            let (ec, p, q, _) = public_source(&args[2]);
            let (k, steps, restarts, entries) = source_rho(&ec, p, q, seed, neg);
            assert_eq!(ec.mulp(k, p), q);
            let count = diff(snap(), before);
            println!("{{\"status\":\"passed\",\"p\":{},\"r\":{},\"arm\":\"{}\",\"seed\":{seed},\"scalar\":{k},\"steps\":{steps},\"restarts\":{restarts},\"table_entries\":{entries},\"M\":{},\"I\":{},\"NAT\":{},\"LA\":{},\"source_verified\":true,\"source_log_read\":false,\"benchmark\":false}}",ec.f.0,ec.r,args[3],count.m,count.i,count.nat,count.la);
        }
        "generate" => {
            assert_eq!(
                args.len(),
                6,
                "generate PUBLIC_SOURCE OUTPUT_DIR SEED COUNT"
            );
            let seed: u64 = args[4].parse().unwrap();
            let count: usize = args[5].parse().unwrap();
            assert!((1..=64).contains(&count));
            let (ec, p, _, mut lines) = public_source(&args[2]);
            let out = std::path::Path::new(&args[3]);
            std::fs::create_dir_all(out).unwrap();
            let mut rng = Rng::new(seed);
            let mut scalars = std::collections::BTreeSet::new();
            let mut rows = vec![];
            for i in 0..count {
                let k = loop {
                    let k = 2 + rng.below(ec.r - 3);
                    if scalars.insert(k) {
                        break k;
                    }
                };
                let q = ec.mulp(k, p);
                assert!(!q.inf && ec.on_curve(q));
                lines[7] = field_text(q.x);
                lines[8] = field_text(q.y);
                let name = format!("p{}-target{i:02}.public", ec.f.0);
                let path = out.join(&name);
                assert!(!path.exists(), "never overwrite a workload");
                std::fs::write(path, lines.join("\n") + "\n").unwrap();
                rows.push(format!("{{\"file\":\"{name}\",\"scalar\":{k}}}"));
            }
            println!("{{\"schema\":\"cubic-native-known-answer-workloads/v1\",\"p\":{},\"r\":{},\"seed\":{seed},\"count\":{count},\"solver_inputs_contain_scalar\":false,\"workloads\":[{}],\"benchmark\":false}}",ec.f.0,ec.r,rows.join(","));
        }
        "verify" => {
            assert_eq!(args.len(), 4, "verify PUBLIC_FIXTURE RECOVERED_SCALAR");
            let (ec, p, q, _) = public_source(&args[2]);
            let k: u64 = args[3].parse().unwrap();
            assert!(k < ec.r && ec.mulp(k, p) == q);
            println!("{{\"status\":\"passed\",\"p\":{},\"r\":{},\"scalar\":{k},\"independent_cubic_arithmetic\":true,\"benchmark\":false}}",ec.f.0,ec.r);
        }
        _ => panic!("unknown source command"),
    }
}
