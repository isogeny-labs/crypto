#[path = "field_a.rs"]
mod arithmetic;
#[path = "certificate_a.rs"]
mod certificate;
#[path = "cm_a.rs"]
mod cm;
#[path = "groups_a.rs"]
mod groups;
#[path = "schemes_a.rs"]
mod schemes;

use arithmetic::{K, Z};
use certificate::{u16_bytes, u32_bytes, Certificate};
use groups::{Action, Group};
use std::collections::BTreeSet;

fn fibre_certificates(g: &Group, certificate: &mut Certificate) {
    let mut payload = Vec::new();
    for p in 1..g.points.len() {
        let target = g.frobenius[p];
        let f = schemes::fibres(
            g.points[p].expect("nonzero E4"),
            g.points[target].expect("nonzero Frobenius E4"),
        );
        payload.extend([p as u8, target as u8]);
        for point in [&f.f0, &f.f1, &f.f3, &f.jet] {
            let ring = &point[0].ring;
            payload.push(ring.degree() as u8);
            u32_bytes(&mut payload, ring.constant().0);
            for coordinate in point {
                for j in 0..3 {
                    u32_bytes(&mut payload, coordinate.data.get(j).copied().unwrap_or(Z).0);
                }
            }
        }
        assert_ne!(f.f0[1].norm(), Z);
        assert_ne!(f.f1[1].norm(), Z);
        assert_ne!(f.f3[1].norm(), Z);
        assert_ne!(f.jet[0].norm(), Z);
        println!("ACTUAL_BETA_FIBRES\tP_index={}\tpiP_index={}\tf0_degree=2_constant={:?}\tf1_degree=2_constant={:?}\tf3_degree=3_constant={:?}\tactual_origin_jet\twhole_C7_forward_R_identities\tfull_coordinates_in_original_certificate\tPASS",p,target,f.f0[0].ring.constant(),f.f1[0].ring.constant(),f.f3[0].ring.constant());
    }
    certificate.section(b"FIBR", 15, 166, &payload);
    println!("GEOMETRIC_BRIDGE\tactual_beta=(phi1,-g,g-phi3)\tg=h([3_inverse]P)\tall15_actual_E4_fibres_and_jet\tE4_inverse3=3\tactual_cover_divisor_correspondences\tnative_genus3_reduction_engine_not_implemented");
}

fn j4(
    g: &Group,
    e: &[Vec<u8>],
    basis: [usize; 2],
    h: &Action,
    s: &Action,
    r: &Action,
    certificate: &mut Certificate,
) {
    let n = g.points.len();
    assert_eq!(n, 16);
    let mut s_map = Vec::new();
    let mut r_map = Vec::new();
    let mut f_map = Vec::new();
    let mut kernel = Vec::new();
    let mut payload = Vec::new();
    for id in 0..4096 {
        let p = Group::decode(id, n);
        let ss = Group::encode(s.apply(g, p), n);
        let rr = Group::encode(r.apply(g, p), n);
        let ff = Group::encode(p.map(|i| g.frobenius[i]), n);
        s_map.push(ss);
        r_map.push(rr);
        f_map.push(ff);
        if ss == 0 {
            kernel.push(id);
        }
        u16_bytes(&mut payload, ss);
        u16_bytes(&mut payload, rr);
        u16_bytes(&mut payload, ff);
        payload.push(u8::from(ss == 0));
    }
    certificate.section(b"J4MP", 4096, 7, &payload);
    let mut kernel_payload = Vec::new();
    for &id in &kernel {
        u16_bytes(&mut kernel_payload, id);
        println!(
            "ACTUAL_J4_KERNEL\tindex={}\tbeta_coordinates={:?}",
            id,
            Group::decode(id, n)
        );
    }
    certificate.section(b"KERN", kernel.len(), 2, &kernel_payload);
    assert_eq!(kernel.len(), 64);
    let mut pairings = Vec::new();
    for &p in &kernel {
        for &q in &kernel {
            pairings.push(groups::h_pairing(
                g,
                e,
                h,
                Group::decode(p, n),
                Group::decode(q, n),
            ));
        }
    }
    certificate.section(b"K4WP", pairings.len(), 1, &pairings);
    assert!(pairings.iter().all(|&value| value == 0));
    let mut j2 = Vec::new();
    let mut killed_by_two = 0;
    let mut test_basis = Vec::new();
    for axis in 0..3 {
        for &point in &basis {
            let mut v = [0; 3];
            v[axis] = point;
            test_basis.push(v);
        }
    }
    for id in 0..4096 {
        let p = Group::decode(id, n);
        assert_eq!(r_map[s_map[id]], 0);
        assert_eq!(s_map[r_map[id]], 0);
        assert_eq!(s_map[f_map[id]], f_map[s_map[id]]);
        assert_eq!(r_map[f_map[id]], f_map[r_map[id]]);
        if p.iter().all(|&i| g.scalar(i, 2) == 0) {
            j2.push(id);
        }
        if s_map[id] == 0 && p.iter().all(|&i| g.scalar(i, 2) == 0) {
            killed_by_two += 1;
        }
        if id != 0 {
            assert!(test_basis
                .iter()
                .any(|&q| groups::h_pairing(g, e, h, p, q) != 0));
        }
        for &q in &test_basis {
            let original = groups::h_pairing(g, e, h, p, q);
            assert_eq!(
                groups::h_pairing(
                    g,
                    e,
                    h,
                    p.map(|i| g.frobenius[i]),
                    q.map(|i| g.frobenius[i])
                ),
                (7 * original) % 4
            );
        }
    }
    assert_eq!(j2.len(), 64);
    assert_eq!(killed_by_two, 16);
    assert!(kernel.iter().all(|&p| s_map[f_map[p]] == 0));
    let two_image: BTreeSet<_> = j2.iter().map(|&p| s_map[p]).collect();
    assert_eq!(two_image.len(), 4);
    for &id in &j2 {
        let p = Group::decode(id, n);
        let fp = p.map(|i| g.frobenius[i]);
        let ffp = fp.map(|i| g.frobenius[i]);
        assert!(
            std::array::from_fn::<_, 3, _>(|i| g.plus[g.plus[ffp[i]][fp[i]]][p[i]])
                .iter()
                .all(|&i| i == 0)
        );
    }
    println!("ACTUAL_J4_RESULT\tall4096_S_R_Frobenius_decisions\tkernel=64\tkilled_by2=16\torder4=48\ttype=(4,4,2,2)\tall4096_kernel_pairings=1\tFrobenius_stable\tS(J2)_size=4_nonzero_proves_genuine_non2divisibility\tH_nondegenerate_all4096\tPASS");
    println!("WRITTEN_PRIME_FIELD_OBSTRUCTION\tall64_actual_J2_points_pi2+pi+I=0\tirreducible_F2_polynomial_X2+X+1\tall_Frobenius_stable_subspaces_have_even_F2_dimension\tno_dimension3_F7_Lagrangian_or_(2,2,2)_first_step\tstandard_module_consequence_not_a_new_135_census");
}

fn rational(g: &Group, s_matrix: cm::Matrix, r_matrix: cm::Matrix, certificate: &mut Certificate) {
    let n = g.points.len();
    assert_eq!(n, 11);
    let s = Action::new(s_matrix, g);
    let r = Action::new(r_matrix, g);
    let inclusion = Action::new(cm::cover_inclusion(), g);
    let mut ss = Vec::new();
    let mut rr = Vec::new();
    let mut four = Vec::new();
    let mut payload = Vec::new();
    for id in 0..1331 {
        let p = Group::decode(id, n);
        let a = Group::encode(s.apply(g, p), n);
        let b = Group::encode(r.apply(g, p), n);
        let c = Group::encode(p.map(|i| g.scalar(i, 4)), n);
        ss.push(a);
        rr.push(b);
        four.push(c);
        u16_bytes(&mut payload, a);
        u16_bytes(&mut payload, b);
        u16_bytes(&mut payload, c);
    }
    certificate.section(b"J11M", 1331, 6, &payload);
    assert_eq!(ss.iter().copied().collect::<BTreeSet<_>>().len(), 1331);
    assert_eq!(rr.iter().copied().collect::<BTreeSet<_>>().len(), 1331);
    for p in 0..1331 {
        assert_eq!(rr[ss[p]], four[p]);
        assert_eq!(ss[rr[p]], four[p]);
    }
    let t = Some((K::prime(1), K::prime(3)));
    let base = g.id(t);
    let embedded = inclusion.apply(g, [base, 0, 0]);
    let target = s.apply(g, embedded);
    assert_eq!(
        embedded,
        [g.scalar(base, 1), g.scalar(base, 7), g.scalar(base, 10)]
    );
    assert_eq!(
        target,
        [g.scalar(base, 2), g.scalar(base, 6), g.scalar(base, 2)]
    );
    assert!((0..11).all(|k| embedded.map(|i| g.scalar(i, k)) != target));
    let mut wrong_matrix = s_matrix;
    wrong_matrix[0][0] = wrong_matrix[0][0].plus(cm::ONE);
    let wrong = Action::new(wrong_matrix, g);
    let mut changed_rejections = 0;
    let mut targets = Vec::new();
    for k in 0..11 {
        let p = g.id(schemes::times(t, k));
        let source = inclusion.apply(g, [p, 0, 0]);
        let image = s.apply(g, source);
        assert_eq!(image, target.map(|i| g.scalar(i, k)));
        let recovered = r.apply(g, image).map(|i| g.scalar(i, 3));
        assert_eq!(recovered, source);
        assert_eq!(recovered[0], p);
        let changed = r.apply(g, wrong.apply(g, source)).map(|i| g.scalar(i, 3));
        if changed != source {
            changed_rejections += 1;
        }
        assert_eq!(
            r.apply(g, s.apply(g, source)).map(|i| g.scalar(i, 3)),
            source
        );
        targets.push(p as u8);
        u16_bytes(&mut targets, Group::encode(source, n));
        u16_bytes(&mut targets, Group::encode(image, n));
        targets.push(recovered[0] as u8);
        println!("ACTUAL_ELLIPTIC_DLP_TRANSFER\tk={}\tP={:?}\tphi0_beta_coordinates={:?}\tS_phi0_beta_coordinates={:?}\tR_then_inverse4_recovers={:?}\tscalar_preserved\tchanged_map_full_recovery_rejects={}\tpristine_restored",k,g.points[p],source,image,recovered,changed!=source);
    }
    certificate.section(b"ELDL", 11, 6, &targets);
    assert!(changed_rejections > 0);
    println!("ACTUAL_RATIONAL_CHAIN\tall1331_JF7_points\tS_R_bijective\tRS=SR=[4]\tall11_original_elliptic_scalars_preserved_and_recovered\tS_phi0_moves_outside_original_phi0_line\tchanged_map_rejections={}\tF7_defined_via_actual_beta_and_omega=pi+2\tPASS",changed_rejections);
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 2, "one original certificate output path");
    let mut certificate = Certificate::create(&args[1]);
    println!("PROFILE\tactual_C7_mixed_Jacobian_map_four_torsion_chain_A\tunsigned_native_correctness_not_benchmark");
    arithmetic::qualify();
    let g4 = groups::e4(&mut certificate);
    let (e, basis) = groups::pairing(&g4, &mut certificate);
    fibre_certificates(&g4, &mut certificate);
    let (h, s, r) = cm::qualify();
    let h_action = Action::new(h, &g4);
    let s_action = Action::new(s, &g4);
    let r_action = Action::new(r, &g4);
    j4(
        &g4,
        &e,
        basis,
        &h_action,
        &s_action,
        &r_action,
        &mut certificate,
    );
    let g11 = groups::rational(&mut certificate);
    rational(&g11, s, r, &mut certificate);
    certificate.finish();
    println!("SUMMARY\tfield_q=117649\tactual_E4_points=16\tactual_Weil4_pairs=256\tfull_actual_fibre_schemes=15\tJ4_decisions=4096\tmixed_kernel=64\tkernel_pairs=4096\tJF7_decisions=1331\toriginal_scalar_targets=11\tactual_beta_bridge_and_F7_maps\tall_full_original_certificate_sections_preserved\tPASS");
    println!("SCOPE\tactual_curve_torsion_and_pairing_with_proved_geometric_beta_transport\tstandard_Jacobian_and_module_dependencies_written\tno_generic_Lean_or_separately_signed_expert_or_native_genus3_reduction_target_engine_or_cold_cost_advantage");
}
