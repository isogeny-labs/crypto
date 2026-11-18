#![allow(dead_code)]

mod frozen {
    #![allow(dead_code)]
    include!("h116_producer.rs");

    pub mod division_torsor {
        use super::*;
        use std::cmp::Ordering;
        use std::collections::{BTreeMap, HashSet};

        const TARGETS: usize = 1520;
        const REPLICATES: usize = 4096;
        const SCALES: [u64; 3] = [1, 2, 3];

        #[derive(Clone)]
        struct Target { slug: String, parity: u8, labels: [u8; 3] }
        struct Rule { map: u8, matches: usize, statistic: (u128, u128) }

        fn base_cubic(spec: CurveSpec, x: u64) -> u64 {
            (mul_mod_u64(mul_mod_u64(x, x, spec.p), x, spec.p)
                + mul_mod_u64(spec.a, x, spec.p) + spec.b) % spec.p
        }

        fn torsion_field(spec: CurveSpec) -> Result<(Field, Vec<Felt>), String> {
            let base_roots: Vec<u64> = (0..spec.p).filter(|&x| base_cubic(spec, x) == 0).collect();
            let (field, mut roots) = match base_roots.len() {
                3 => {
                    let field = Field::new(spec.p, 1, &[0, 1])?;
                    let roots = base_roots.iter().map(|&root| field.from_base(root)).collect();
                    (field, roots)
                }
                1 => {
                    let root = base_roots[0];
                    let constant = (spec.a + mul_mod_u64(root, root, spec.p)) % spec.p;
                    let modulus = vec![constant, root, 1];
                    if !h116_field::is_irreducible(&modulus, spec.p) {
                        return Err("quadratic two-torsion factor is not irreducible".to_string());
                    }
                    let field = Field::new(spec.p, 2, &modulus)?;
                    let alpha = field.from_index(spec.p);
                    let roots = vec![field.from_base(root), alpha, field.frobenius(alpha)];
                    (field, roots)
                }
                0 => {
                    let modulus = vec![spec.b, spec.a, 0, 1];
                    if !h116_field::is_irreducible(&modulus, spec.p) {
                        return Err("rootless two-torsion cubic is not irreducible".to_string());
                    }
                    let field = Field::new(spec.p, 3, &modulus)?;
                    let alpha = field.from_index(spec.p);
                    let roots = vec![alpha, field.frobenius(alpha), field.frobenius_power(alpha, 2)];
                    (field, roots)
                }
                count => return Err(format!("unexpected base two-torsion root count {count}")),
            };
            roots.sort_by_key(|&root| field.to_index(root));
            roots.dedup();
            if roots.len() != 3 { return Err("two-torsion roots are not distinct".to_string()); }
            for &root in &roots {
                let value = field.add(
                    field.add(field.mul(field.square(root), root), field.mul(field.from_base(spec.a), root)),
                    field.from_base(spec.b));
                if value != field.zero() { return Err("constructed two-torsion root failed cubic".to_string()); }
            }
            Ok((field, roots))
        }

        fn point_key(field: &Field, point: Point) -> Result<(u64, u64), String> {
            let Point::Affine(x, y) = point else { return Err("half unexpectedly infinity".to_string()); };
            Ok((field.to_index(x), field.to_index(y)))
        }

        fn selected_index(field: &Field, halves: &[Point; 4], scale: u64) -> Result<usize, String> {
            let mut best = 0_usize;
            let mut best_key = point_key(field, coordinate_map(field, halves[0], scale))?;
            for (index, &half) in halves.iter().enumerate().skip(1) {
                let key = point_key(field, coordinate_map(field, half, scale))?;
                if key < best_key { best = index; best_key = key; }
            }
            Ok(best)
        }

        fn prediction(map: u8, label: u8) -> u8 { (map >> label) & 1 }
        fn score(map: u8, labels: &[u8], truth: &[u8]) -> usize {
            labels.iter().zip(truth).filter(|(label, bit)| prediction(map, **label) == **bit).count()
        }
        fn statistic(matches: usize, count: usize) -> (u128, u128) {
            let signed = 2_i128 * matches as i128 - count as i128;
            if signed <= 0 { (0, 1) } else { ((signed as u128).pow(2), count as u128) }
        }
        fn compare_ratio(left: (u128, u128), right: (u128, u128)) -> Ordering {
            (left.0 * right.1).cmp(&(right.0 * left.1))
        }
        fn splitmix64(state: &mut u64) -> u64 {
            *state = state.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = *state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^ (z >> 31)
        }
        fn shuffle<T>(values: &mut [T], state: &mut u64) {
            for index in (1..values.len()).rev() {
                let swap = (splitmix64(state) % (index as u64 + 1)) as usize;
                values.swap(index, swap);
            }
        }

        pub fn selftest() -> Result<(), String> {
            let labels = [0_u8, 1, 0, 1];
            let truth = [0_u8, 1, 0, 1];
            assert_eq!(score(0b0010, &labels, &truth), 4);
            assert_eq!(statistic(8, 10), (36, 10));
            assert_eq!((0_u8..16).count(), 16);
            for spec in CURVES {
                let (field, roots) = torsion_field(spec)?;
                assert!((1..=3).contains(&field.d));
                assert_eq!(roots.len(), 3);
            }
            println!("h116 division-torsor Frobenius selftest: pass maps=16");
            Ok(())
        }

        pub fn run(output_path: &Path) -> Result<(), String> {
            let mut targets = Vec::new();
            let mut rows = BTreeMap::<String, Vec<usize>>::new();
            let mut half_checks = 0_usize;
            let mut multiset_checks = 0_usize;
            let mut field_degrees = Vec::new();

            for spec in CURVES {
                let (field, roots) = torsion_field(spec)?;
                field_degrees.push(format!("{{\"slug\":\"{}\",\"degree\":{}}}", spec.slug, field.d));
                let curve = Curve { field: &field, a: field.from_base(spec.a), b: field.from_base(spec.b) };
                let generator = first_base_point(&curve, spec.p)?;
                if generator == Point::Infinity || curve.scalar_mul(spec.order, generator)? != Point::Infinity {
                    return Err("generator/order gate failed".to_string());
                }
                let mut torsion = vec![Point::Infinity];
                torsion.extend(roots.iter().map(|&root| Point::Affine(root, field.zero())));
                for &point in &torsion {
                    if curve.scalar_mul(2, point)? != Point::Infinity {
                        return Err("constructed point is not two-torsion".to_string());
                    }
                }
                let mut fixed_multiset = Vec::new();
                for &point in &torsion {
                    let cocycle = curve.sub(curve.frobenius(point), point)?;
                    let label = torsion.iter().position(|&candidate| candidate == cocycle)
                        .ok_or("torsion cocycle escaped E[2]")?;
                    fixed_multiset.push(label as u8);
                }
                fixed_multiset.sort_unstable();

                let half_scalar = (spec.order + 1) / 2;
                for scalar in 1..spec.order {
                    let target = curve.scalar_mul(scalar, generator)?;
                    let rational_half = curve.scalar_mul(half_scalar, target)?;
                    if curve.scalar_mul(2, rational_half)? != target {
                        return Err("rational half failed".to_string());
                    }
                    let mut halves = [Point::Infinity; 4];
                    let mut cocycle_labels = [0_u8; 4];
                    let mut observed_multiset = Vec::new();
                    let mut unique = HashSet::new();
                    for index in 0..4 {
                        halves[index] = curve.add(rational_half, torsion[index])?;
                        if halves[index] == Point::Infinity || curve.scalar_mul(2, halves[index])? != target {
                            return Err("division-torsor half check failed".to_string());
                        }
                        if !unique.insert(halves[index]) { return Err("duplicate division-torsor half".to_string()); }
                        let cocycle = curve.sub(curve.frobenius(halves[index]), halves[index])?;
                        if curve.scalar_mul(2, cocycle)? != Point::Infinity {
                            return Err("selected cocycle is not two-torsion".to_string());
                        }
                        let label = torsion.iter().position(|&candidate| candidate == cocycle)
                            .ok_or("half cocycle escaped E[2]")? as u8;
                        cocycle_labels[index] = label;
                        observed_multiset.push(label);
                        half_checks += 1;
                    }
                    observed_multiset.sort_unstable();
                    if observed_multiset != fixed_multiset {
                        return Err("target-dependent unselected cocycle multiset".to_string());
                    }
                    multiset_checks += 1;
                    let mut labels = [0_u8; 3];
                    for (scale_index, &scale) in SCALES.iter().enumerate() {
                        labels[scale_index] = cocycle_labels[selected_index(&field, &halves, scale)?];
                    }
                    let index = targets.len();
                    targets.push(Target { slug: spec.slug.to_string(), parity: (scalar % 2) as u8, labels });
                    rows.entry(spec.slug.to_string()).or_default().push(index);
                }
            }
            if targets.len() != TARGETS || rows.len() != 5 || half_checks != TARGETS * 4
                || multiset_checks != TARGETS {
                return Err("torsor census failed".to_string());
            }
            let truth: Vec<u8> = targets.iter().map(|target| target.parity).collect();
            let primary_labels: Vec<u8> = targets.iter().map(|target| target.labels[0]).collect();
            let planted_labels = truth.clone();
            if score(0b0010, &planted_labels, &truth) != TARGETS {
                return Err("planted scorer control failed".to_string());
            }
            let rules: Vec<Rule> = (0_u8..16).map(|map| {
                let matches = score(map, &primary_labels, &truth);
                Rule { map, matches, statistic: statistic(matches, TARGETS) }
            }).collect();
            let selected_index = (0..rules.len()).max_by(|&left, &right| {
                compare_ratio(rules[left].statistic, rules[right].statistic)
                    .then_with(|| rules[right].map.cmp(&rules[left].map))
            }).ok_or("no maps")?;
            let selected = &rules[selected_index];
            let exact_rules = rules.iter().filter(|rule| rule.matches == TARGETS).count();

            let mut exceedances = 0_usize;
            for replicate in 0..REPLICATES {
                let mut shuffled = vec![0_u8; TARGETS];
                for (slug, indices) in &rows {
                    let mut values: Vec<u8> = indices.iter().map(|&index| truth[index]).collect();
                    let mut seed = 0x116_010_u64
                        ^ (replicate as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15)
                        ^ slug.bytes().map(u64::from).sum::<u64>();
                    shuffle(&mut values, &mut seed);
                    for (&index, value) in indices.iter().zip(values) { shuffled[index] = value; }
                }
                let mut maximum = (0_u128, 1_u128);
                for map in 0_u8..16 {
                    let candidate = statistic(score(map, &primary_labels, &shuffled), TARGETS);
                    if compare_ratio(candidate, maximum) == Ordering::Greater { maximum = candidate; }
                }
                exceedances += usize::from(compare_ratio(maximum, selected.statistic) != Ordering::Less);
            }

            let mut per_curve = Vec::new();
            for (slug, indices) in &rows {
                let labels: Vec<u8> = indices.iter().map(|&index| targets[index].labels[0]).collect();
                let labels_truth: Vec<u8> = indices.iter().map(|&index| truth[index]).collect();
                per_curve.push(format!("{{\"slug\":\"{slug}\",\"targets\":{},\"matches\":{}}}",
                    indices.len(), score(selected.map, &labels, &labels_truth)));
            }
            let mut scale_json = Vec::new();
            for scale_index in 0..3 {
                let labels: Vec<u8> = targets.iter().map(|target| target.labels[scale_index]).collect();
                let matches = score(selected.map, &labels, &truth);
                let agreements = labels.iter().zip(&primary_labels).filter(|(left, right)| left == right).count();
                scale_json.push(format!("{{\"scale\":{},\"matches\":{matches},\"selector_label_agreements\":{agreements}}}",
                    SCALES[scale_index]));
            }
            let signed_excess = 2_i64 * selected.matches as i64 - TARGETS as i64;
            let json = format!(concat!("{{\n  \"schema\":\"h116-division-torsor-frobenius/v1\",\n",
                "  \"targets\":{},\n  \"curves\":{},\n  \"maps\":16,\n",
                "  \"half_cocycle_checks\":{},\n  \"multiset_checks\":{},\n",
                "  \"planted_control_matches\":{},\n  \"selected_map\":{},\n",
                "  \"matches\":{},\n  \"signed_excess\":{},\n",
                "  \"statistic_numerator\":{},\n  \"statistic_denominator\":{},\n",
                "  \"exact_rules\":{},\n  \"replicates\":4096,\n",
                "  \"familywise_exceedances\":{},\n  \"familywise_empirical_p\":{:.12},\n",
                "  \"field_degrees\":[{}],\n  \"scale_scores\":[{}],\n",
                "  \"per_curve\":[{}],\n  \"status\":\"measured\"\n}}\n"),
                targets.len(), rows.len(), half_checks, multiset_checks, TARGETS, selected.map,
                selected.matches, signed_excess, selected.statistic.0, selected.statistic.1,
                exact_rules, exceedances, (exceedances + 1) as f64 / (REPLICATES + 1) as f64,
                field_degrees.join(","), scale_json.join(","), per_curve.join(","));
            fs::write(output_path, json).map_err(|error| error.to_string())?;
            println!("division-torsor Frobenius: map={} matches={} exact={} p={:.6}", selected.map,
                selected.matches, exact_rules, (exceedances + 1) as f64 / (REPLICATES + 1) as f64);
            Ok(())
        }
    }
}

use std::path::Path;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let result = match args.as_slice() {
        [_, flag] if flag == "--selftest" => frozen::division_torsor::selftest(),
        [_, flag, output] if flag == "--run" => frozen::division_torsor::run(Path::new(output)),
        _ => Err("usage: h116_division_torsor_frobenius --selftest | --run OUTPUT.json".to_string()),
    };
    if let Err(error) = result { eprintln!("h116 division-torsor Frobenius error: {error}"); std::process::exit(1); }
}
