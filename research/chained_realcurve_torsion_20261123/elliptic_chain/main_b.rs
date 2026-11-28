mod base27;
#[path = "chart_b.rs"]
mod chart;
#[path = "ec_b.rs"]
mod ec;
#[path = "field_b.rs"]
mod field;
mod inherited;
#[path = "maps_b.rs"]
mod maps;

use chart::{key, Chart, Divisor};
use ec::Point;
use field::{Field, Q};
use inherited::{pmod, pscale, Div};
use maps::{Kernel, G0, G1, G2, ORIGINAL};
use std::collections::BTreeSet;

fn qualify(label: &str, chart: &Chart<'_>, points: &[Point], images: &[Divisor]) {
    assert_eq!(points.len(), 7);
    assert_eq!(images.len(), 7);
    let mut distinct = BTreeSet::new();
    for (i, (&p, image)) in points.iter().zip(images).enumerate() {
        assert!(chart.h.is_valid(image) && image.u.len() <= 3);
        assert!(chart.h.is_identity(&chart.h.mul(image, 7)));
        assert_eq!(chart.frobenius(image), *image);
        assert!(distinct.insert(key(image)));
        assert_eq!(chart.h.is_identity(image), p.infinity);
        println!("IMAGE map={label} point_id={i} point={p:?} divisor={:?} F3_fixed=pass order7_or_identity=pass", key(image));
    }
    assert_eq!(distinct.len(), 7);
    for (i, &p) in points.iter().enumerate() {
        for (j, &q) in points.iter().enumerate() {
            let sum = ec::add(p, q);
            let index = points.iter().position(|&r| r == sum).unwrap();
            assert_eq!(chart.h.add(&images[i], &images[j]), images[index]);
        }
    }
    let generator = points.iter().position(|p| !p.infinity).unwrap();
    for scalar in 0..7 {
        let q = ec::mul(points[generator], scalar);
        let index = points.iter().position(|&r| r == q).unwrap();
        assert_eq!(
            chart.h.mul(&images[generator], u128::from(scalar)),
            images[index]
        );
    }
    println!("MAP_RESULT map={label} distinct=7 addition_pairs=49 scalar_controls=7 F3_fixed=7 injective_on_source_order7=pass");
}

fn main() {
    let f = Field::new();
    println!("FIELD q={Q} tower=F27[b]/(b2-2) F27_modulus=2,2,0,1 generator={} all_pair_addition_and_multiplication_checks={} inverse_and_Fq_checks={Q} square_root_checks={Q}", f.generator, Q * Q);
    let points = ec::points();
    assert!(!ec::on_curve(Point {
        x: 0,
        y: 0,
        infinity: false
    }));
    assert!(!ec::on_curve(Point {
        x: 3,
        y: 1,
        infinity: false
    }));
    for &p in &points {
        assert_eq!(ec::add(p, ec::IDENTITY), p);
        assert_eq!(ec::mul(p, 7), ec::IDENTITY);
        for &q in &points {
            assert!(points.contains(&ec::add(p, q)));
            assert_eq!(ec::add(p, q), ec::add(q, p));
            for &r in &points {
                assert_eq!(ec::add(ec::add(p, q), r), ec::add(p, ec::add(q, r)));
            }
        }
    }
    let generator = *points.iter().find(|p| !p.infinity).unwrap();
    assert_eq!(
        (0..7)
            .map(|n| ec::mul(generator, n))
            .collect::<BTreeSet<_>>()
            .len(),
        7
    );
    println!("ELLIPTIC curve=Y2-X3-2X-1 points={points:?} generator={generator:?} order=7 addition_pairs=49 associativity_triples=343 invalid_point_controls=pass");
    maps::cover_identity(&f);
    let source = Chart::new(&f, &ORIGINAL);
    source.controls();
    println!("CHART label=source roots={:?} r={} scale={} leading={} monic={:?} inverse_cube_exponent=243 basepoint_control=pass", source.roots, source.r, source.scale, source.leading, source.h.h);
    let source_images: Vec<_> = points.iter().map(|&p| maps::pullback(&source, p)).collect();
    let mut scale_mutants_rejected = 0;
    let mut source_infinities = 0;
    for (&p, image) in points.iter().zip(&source_images) {
        assert_eq!(maps::source_fibre_reference(&source, p), *image);
        if !p.infinity {
            let mutated = Div {
                u: image.u.clone(),
                v: pscale(&f, &image.v, &source.scale),
            };
            assert!(!source.h.is_valid(&mutated));
            assert!(source.h.is_valid(&maps::pullback(&source, p)));
            scale_mutants_rejected += 1;
            for (a, b, y) in maps::preimages(&f, p) {
                source_infinities += usize::from(b == 0);
                println!("COVER_PREIMAGE point={p:?} a={a} b={b} y_homogeneous={y} original_source_infinity={}", u8::from(b == 0));
            }
        }
    }
    assert_eq!(source_infinities, 2);
    assert_eq!(scale_mutants_rejected, 6);
    qualify("elliptic_pullback", &source, &points, &source_images);
    assert!(Kernel::new(&f, 1, G1).is_err());
    let kernels = [
        Kernel::new(&f, 0, G0).unwrap(),
        Kernel::new(&f, 2, G2).unwrap(),
    ];
    let mut degree_drop_controls = 0;
    let mut affine_remainder_positive_controls = 0;
    let mut total_chain_maps = 0;
    for kernel in &kernels {
        let target = Chart::new(&f, &kernel.codomain);
        target.controls();
        println!(
            "KERNEL id={} G={:?} H={:?} delta={} codomain={:?}",
            kernel.id, kernel.g, kernel.h, kernel.delta, kernel.codomain
        );
        println!("CHART label=Ric{} roots={:?} r={} scale={} leading={} monic={:?} inverse_cube_exponent=243 basepoint_control=pass", kernel.id, target.roots, target.r, target.scale, target.leading, target.h.h);
        assert!(target.h.is_identity(&maps::identity_fibre(&target, kernel)));
        let images: Vec<_> = points
            .iter()
            .map(|&p| maps::chain(&target, kernel, p))
            .collect();
        let label = format!("elliptic_then_Ric{}", kernel.id);
        qualify(&label, &target, &points, &images);
        total_chain_maps += 1;
        if kernel.id == 0 {
            for (x, y) in [(20, 1), (20, 2), (25, 1), (25, 2)] {
                let (u, numerator) = maps::relation(&f, kernel, x, 1, y);
                assert_eq!(u.len(), 2);
                let correct = maps::convert_ideal(&target, &u, &numerator);
                assert!(target.h.is_valid(&correct));
                let remainder = pmod(&f, &numerator, &u);
                let bad = maps::convert_ideal(&target, &u, &remainder);
                assert!(!target.h.is_valid(&bad));
                assert_eq!(maps::image_point(&target, kernel, x, 1, y), correct);
                degree_drop_controls += 1;
                println!("DEGREE_DROP_CONTROL x={x} y={y} relation={u:?} corrected={:?} early_affine_remainder_rejected=pass restored=pass", key(&correct));
            }
            let (u, numerator) = maps::relation(&f, kernel, 0, 1, 1);
            assert_eq!(u.len(), 3);
            let full = maps::convert_ideal(&target, &u, &numerator);
            let remainder = maps::convert_ideal(&target, &u, &pmod(&f, &numerator, &u));
            assert!(target.h.is_valid(&remainder));
            assert_eq!(full, remainder);
            affine_remainder_positive_controls += 1;
            println!("DEGREE2_POSITIVE_CONTROL x=0 y=1 early_affine_remainder_valid=pass same_class=pass");
        }
    }
    assert_eq!(degree_drop_controls, 4);
    assert_eq!(affine_remainder_positive_controls, 1);
    assert_eq!(total_chain_maps, 2);
    println!("SUMMARY explicit_elliptic_pullback=pass actual_Richelot_chains=2 each_source_group_order=7 each_injective_images=7 each_addition_pairs=49 total_addition_pairs=147 each_F3_fixed=7 source_infinity_preimages=2 double_preimage_u0=covered degree_drop_controls=4 nonvacuous_degree2_positive=1 pullback_scale_mutants_rejected=6 chart_basepoint_controls=3 product_endpoint_rejected=1 scalar_labels_used_by_transfer=0 actual_weil8=not_computed cryptographic_size_solve=not_claimed cost_advantage=not_claimed");
}
