mod accounting;
mod cubic;
#[path = "inputs/cryptanalysis1964/divisor_a.rs"]
mod divisor_a;
#[path = "inputs/cryptanalysis1964/elliptic_a.rs"]
mod elliptic_a;
#[path = "inputs/cryptanalysis1964/field49_a.rs"]
mod field49_a;
mod field_d;
#[path = "inputs/crypto5114/field_a.rs"]
mod field_reference;
#[path = "inputs/crypto5114/poly_d.rs"]
mod poly_d;
mod rational;
mod rho;

use cubic::{eval_point, CURVE};
use divisor_a::{Div, H};
use elliptic_a::EP;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

const SEEDS: [u64; 4] = [
    0x243f6a8885a308d3,
    0x13198a2e03707344,
    0xa4093822299f31d0,
    0x082efa98ec4e6c89,
];
fn file(path: &Path) -> BufWriter<std::fs::File> {
    BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .unwrap(),
    )
}
fn point(p: EP) -> String {
    p.map_or_else(|| "O".to_owned(), |(x, y)| format!("{x}:{y}"))
}
fn parse_point(text: &str) -> EP {
    if text == "O" {
        None
    } else {
        let (x, y) = text.split_once(':').unwrap();
        Some((x.parse().unwrap(), y.parse().unwrap()))
    }
}
fn input_and_instrument_controls() {
    for a in 0..7 {
        for b in 0..7 {
            assert_eq!(field_d::add(a, b), field_reference::add(a, b));
            assert_eq!(field_d::mul(a, b), field_reference::mul(a, b));
            assert_eq!(field_d::sub(a, b), field_reference::sub(a, b));
        }
        if a != 0 {
            assert_eq!(field_d::inv(a), field_reference::inv(a));
        }
    }
    let (_, c) = accounting::sample(|| {
        assert_eq!(field_d::add(2, 3), 5);
        assert_eq!(field_d::mul(2, 3), 6);
        assert_eq!(field_d::inv(2), 4);
        let mut bytes = Vec::<u8>::with_capacity(73);
        bytes.push(1);
        std::hint::black_box(&bytes);
        drop(bytes);
    });
    assert_eq!((c.fp_add, c.fp_mul, c.fp_inv), (1, 6, 1));
    assert_eq!(
        (
            c.allocation_calls,
            c.allocated_bytes,
            c.freed_bytes,
            c.peak_new_heap_bytes
        ),
        (1, 73, 73, 73)
    );
    println!("INSTRUMENT_CONTROL\t{c:?}\twhole-inversion-includes-five-multiplications=true");
    let points = elliptic_a::points();
    assert_eq!(points.len(), 11);
    for &p in &points {
        assert!(rho::ec_valid(p));
        for &q in &points {
            assert_eq!(rho::ec_add(p, q), elliptic_a::add_base(p, q));
        }
        for n in 0..=11 {
            assert_eq!(rho::ec_mul(p, n), elliptic_a::mul_base(p, n));
        }
    }
    println!("DIRECT_E_BASELINE_CONTROL\tall121-additions-and132-multiplications-match-published-independent-field-route");
}
fn all_class_pushes(out: &Path) {
    let old = include_str!("inputs/cryptanalysis1964/classes.csv");
    let mut records = file(&out.join("class_pushes.csv"));
    writeln!(
        records,
        "id,actual_H3_push,predicted_6_times_p1_plus_3p2,root_type,old_push1,old_push2"
    )
    .unwrap();
    let mut count = 0;
    for line in old.lines().skip(1) {
        let cells: Vec<_> = line.split(',').collect();
        assert_eq!(cells.len(), 15);
        let value = |n: usize| cells[n].parse::<u8>().unwrap();
        let d = match value(1) {
            0 => divisor_a::identity(),
            2 => Div {
                u: vec![value(2), value(3), 1],
                v: poly_d::trim(vec![value(4), value(5)]),
            },
            _ => panic!("unexpected original degree"),
        };
        assert!(divisor_a::valid(&d));
        let packet = divisor_a::push(&d);
        assert_eq!(packet.pair, (parse_point(cells[6]), parse_point(cells[7])));
        assert_eq!(packet.kind, cells[8]);
        let actual = cubic::push(&d);
        let predicted = rho::ec_mul(rho::ec_add(packet.pair.0, rho::ec_mul(packet.pair.1, 3)), 6);
        assert_eq!(
            actual, predicted,
            "actual cubic root evaluation at original class {count}"
        );
        assert_eq!(cells[0].parse::<usize>().unwrap(), count);
        writeln!(
            records,
            "{count},{},{},{},{},{}",
            point(actual),
            point(predicted),
            packet.kind,
            point(packet.pair.0),
            point(packet.pair.1)
        )
        .unwrap();
        count += 1;
    }
    assert_eq!(count, 121);
    records.flush().unwrap();
    println!("ALL_PRIOR_CLASSES_NEW_CUBIC_PUSH\t121-actual-class-pushes\tindependent-old-class-inputs-retained\tno-old-Cayley-reexecution");
}
fn all_target_controls(out: &Path, g: EP) {
    let mut rows = file(&out.join("target_controls.csv"));
    writeln!(rows,"driver_oracle_only,source,canonical_u,canonical_v,push1,push2,H3_push,fibre_u,fibre_v,fibre_repeated_degree").unwrap();
    let base = cubic::pull(g);
    for oracle in 0..11 {
        let q = elliptic_a::mul_base(g, oracle);
        let (mapped_g, mapped_q) = eval_point(&CURVE, g, q);
        assert_eq!(mapped_g, base);
        assert_eq!(mapped_q, divisor_a::gamma(q));
        assert_eq!(mapped_q, divisor_a::scalar(&base, oracle));
        let pair = divisor_a::push(&mapped_q).pair;
        assert_eq!(pair, (q, rho::ec_mul(q, 9)));
        let push3 = cubic::push(&mapped_q);
        assert_eq!(push3, rho::ec_mul(q, 3));
        let fibre = cubic::fibre(q);
        let repeated = poly_d::bezout(&fibre.u, &poly_d::derivative(&fibre.u))
            .0
            .len()
            - 1;
        let poly = |v: &[u8]| v.iter().map(u8::to_string).collect::<Vec<_>>().join(":");
        writeln!(
            rows,
            "{oracle},{},{},{},{},{},{},{},{},{repeated}",
            point(q),
            poly(&mapped_q.u),
            poly(&mapped_q.v),
            point(pair.0),
            point(pair.1),
            point(push3),
            poly(&fibre.u),
            poly(&fibre.v)
        )
        .unwrap();
        rows.flush().unwrap();
        println!("ALL_COORDINATE_INPUTS\tdriver-oracle-only={oracle}\tQ={q:?}\tactual-cubic-fibre={fibre:?}\tcanonical={mapped_q:?}\tpush1={:?}\tH3push={push3:?}\trepeated-fibre-degree={repeated}",pair.0);
    }
}
fn cost_line(
    rows: &mut BufWriter<std::fs::File>,
    oracle: usize,
    master: u64,
    seed: u64,
    arm: &str,
    r: rho::Outcome,
    c: accounting::Cost,
) {
    write!(rows, "{oracle},{master},{seed},{arm}").unwrap();
    for value in [
        u64::try_from(r.scalar).unwrap(),
        u64::try_from(r.restarts).unwrap(),
        u64::try_from(r.walk_steps).unwrap(),
        u64::try_from(r.degenerate_collisions).unwrap(),
        c.fp_add,
        c.fp_mul,
        c.fp_inv,
        c.scalar_add,
        c.scalar_mul,
        c.hash_mix,
        c.coordinate_compare,
        c.allocation_calls,
        c.allocated_bytes,
        c.freed_bytes,
        c.peak_new_heap_bytes,
    ] {
        write!(rows, ",{value}").unwrap();
    }
    write!(rows, ",").unwrap();
    for (i, stream) in r.used_streams.iter().take(r.restarts + 1).enumerate() {
        if i > 0 {
            write!(rows, ":").unwrap();
        }
        write!(rows, "{stream:016x}").unwrap();
    }
    writeln!(rows).unwrap();
    rows.flush().unwrap();
}
fn cold_operation_controls(out: &Path, g: EP) {
    let mut rows = file(&out.join("cold_operation_controls.csv"));
    writeln!(rows,"driver_oracle_only,master_seed,effective_walk_seed,arm,recovered,restarts,walk_steps,degenerate_collisions,fp_add,fp_mul,fp_inv_included_in_mul,scalar_add,scalar_mul,hash_mix_blocks,coordinate_compares,allocation_calls,allocated_bytes,freed_bytes,peak_new_heap_bytes,actual_restart_streams_hex").unwrap();
    let mut completed = 0;
    for oracle in 1..11 {
        let q = elliptic_a::mul_base(g, oracle);
        for master in SEEDS {
            let seed = rho::target_seed(master, q);
            // Target generation and CSV I/O are outside every arm. Each
            // measured mathematical arm is cold and contains its full setup.
            let (direct, dc) = accounting::sample(|| {
                cubic::validate_inputs(&CURVE, g, q);
                let walk = rho::target_seed(master, q);
                rho::solve::<rho::Elliptic>(&g, &q, 11, walk)
                    .expect("fixed direct rho control exhausted")
            });
            assert_eq!(direct.scalar, oracle);
            cost_line(&mut rows, oracle, master, seed, "direct_E_rho", direct, dc);
            let (new, nc) = accounting::sample(|| {
                let (base, target) = eval_point(&CURVE, g, q);
                let walk = rho::target_seed(master, q);
                let answer = rho::solve::<rho::Jacobian>(&base, &target, 11, walk)
                    .expect("fixed cubic rho control exhausted");
                assert_eq!(rho::ec_mul(g, answer.scalar), q);
                answer
            });
            assert_eq!(new.scalar, oracle);
            cost_line(
                &mut rows,
                oracle,
                master,
                seed,
                "cubic_pullback_J_rho",
                new,
                nc,
            );
            let (old, oc) = accounting::sample(|| {
                cubic::validate_inputs(&CURVE, g, q);
                let (base, target) = (divisor_a::gamma(g), divisor_a::gamma(q));
                let walk = rho::target_seed(master, q);
                let answer = rho::solve::<rho::Jacobian>(&base, &target, 11, walk)
                    .expect("fixed prior Gamma rho control exhausted");
                assert_eq!(rho::ec_mul(g, answer.scalar), q);
                answer
            });
            assert_eq!(old.scalar, oracle);
            cost_line(
                &mut rows,
                oracle,
                master,
                seed,
                "prior_Gamma_J_rho",
                old,
                oc,
            );
            completed += 3;
        }
    }
    assert_eq!(completed, 120);
    println!("COLD_SOURCE_OPERATION_CONTROLS\t120-records\t10-nonzero-targets*4-frozen-seeds*3-arms\tall-recovered-and-source-replayed\tall-setup-included-per-target\tno-walltime-or-bitop-speedup-claim\tstructural-word-and-OS-costs-remain-symbolic-whole-call-terms");
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 2, "one fresh dataset directory");
    let out = Path::new(&args[1]);
    std::fs::create_dir(out).unwrap();
    println!("PROFILE\tCM19-actual-cubic-cover-coordinate-controls-A\tfixed-object-correctness-and-source-operation-accounting\tunsigned");
    input_and_instrument_controls();
    let (g, s, t) = poly_d::bezout(&H, &poly_d::derivative(&H));
    assert_eq!(g, vec![1]);
    println!("ORIGINAL_MODEL\tH={H:?}\tsquarefree-s={s:?}\tt={t:?}");
    cubic::global_certificate();
    let generator = Some((1, 3));
    let mut logical_streams = std::collections::BTreeSet::new();
    for d in 1..11 {
        let q = elliptic_a::mul_base(generator, d);
        for master in SEEDS {
            for restart in 0..32 {
                assert!(
                    logical_streams
                        .insert(rho::restart_stream(rho::target_seed(master, q), restart)),
                    "duplicate logical control stream"
                );
            }
        }
    }
    assert_eq!(logical_streams.len(), 1280);
    println!("SEED_CONTROL\t1280-distinct-domain-separated-target-restart-streams\tintentional-three-arm-reuse\tactual-used-streams-retained-in-each-row");
    all_target_controls(out, generator);
    all_class_pushes(out);
    cold_operation_controls(out, generator);
    println!("PRISTINE_COMPLETE\tactual-cubic-curve-and-divisor-maps\tall-original-controls-retained\tfull-generic-formalization-and-signed-verification-open");
}
