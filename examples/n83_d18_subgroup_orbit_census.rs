//! Exact direct-subgroup census of low-bit x coordinates on the public N83 Koblitz curve.
//! Usage: cargo run --release --example n83_d18_subgroup_orbit_census -- NEW_DIR [dimension]
use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::koblitz_fast_arith::FastBinaryCurve128;
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use serde_json::json;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const K: usize = 16_627;
const SOURCE: &[u8] = include_bytes!("n83_d18_subgroup_orbit_census.rs");

fn orbit_key(curve: &FastBinaryCurve128, x: u128) -> Result<(u128, u32)> {
    let mut current = x;
    let mut least = x;
    for phase in 1..=83 {
        current = curve.gf.sqr(current);
        if current < least {
            least = current;
        }
        if current == x {
            if 83 % phase != 0 {
                return Err("Frobenius orbit length does not divide 83".into());
            }
            return Ok((least, phase));
        }
    }
    Err("Frobenius did not return after 83 phases".into())
}

fn census(output: &Path, dimension: u32) -> Result<()> {
    if !(1..=18).contains(&dimension) {
        return Err("dimension must be 1..=18".into());
    }
    fs::create_dir(output)?;
    let started = Instant::now();
    let kc = KoblitzCurve::known_n83_k0().ok_or("public N83 K0 fixture unavailable")?;
    if kc.label() != CURVE
        || kc.cofactor != 4u8.into()
        || kc.subgroup_order.to_string() != "2417851639230796216685689"
        || kc.n != 83
        || !kc.frobenius_is_endomorphism
    {
        return Err("curve or subgroup identity changed".into());
    }
    let curve =
        FastBinaryCurve128::new(&kc.curve.irreducible, 0).ok_or("wide binary curve unavailable")?;
    let init_ms = started.elapsed().as_secs_f64() * 1000.0;
    let mut rational_x = 0usize;
    let mut subgroup_x = 0usize;
    let mut subgroup_short_orbit_x = 0usize;
    // Key: minimum x in the complete Frobenius orbit. Value: least x from
    // this subspace and number of subspace x words in that orbit.
    let mut classes = BTreeMap::<u128, (u128, u32)>::new();
    for x in 0..(1u128 << dimension) {
        let points = curve.points_with_x(1, x);
        let Some(point) = points.first().copied() else {
            continue;
        };
        rational_x += 1;
        if curve.scalar_mul(point, &kc.subgroup_order).is_some() {
            continue;
        }
        subgroup_x += 1;
        let (key, length) = orbit_key(&curve, x)?;
        if length != 83 {
            subgroup_short_orbit_x += 1;
            continue;
        }
        let entry = classes.entry(key).or_insert((x, 0));
        entry.0 = entry.0.min(x);
        entry.1 += 1;
    }
    let census_ms = started.elapsed().as_secs_f64() * 1000.0 - init_ms;
    let selected: Vec<_> = classes.iter().take(K).collect();
    let selected_count = selected.len();
    let mut generic_sample_checks = 0usize;
    for (_, &(source_x, _)) in selected.iter().step_by(1024) {
        let fast_point = curve.points_with_x(1, source_x)[0].unwrap();
        let point = BinaryPoint::Affine {
            x: curve.element(fast_point.0),
            y: curve.element(fast_point.1),
        };
        if !kc.curve.is_on_curve(&point)
            || kc.mul(&point, &kc.subgroup_order) != BinaryPoint::Infinity
        {
            return Err("generic curve/subgroup sample disagrees with packed census".into());
        }
        let mut phase_x = curve.element(source_x);
        for _ in 0..83 {
            phase_x = kc.frobenius_x(&phase_x);
        }
        if phase_x.to_biguint() != source_x.into() {
            return Err("generic Frobenius sample did not return".into());
        }
        generic_sample_checks += 1;
    }
    let selected_path = output.join("selected-representatives.csv");
    let mut writer = BufWriter::new(File::create_new(&selected_path)?);
    writer.write_all(b"orbit_key_decimal,source_x_decimal,source_x_multiplicity\n")?;
    let mut all_writer = BufWriter::new(File::create_new(output.join("orbit-classes.csv"))?);
    all_writer.write_all(b"orbit_key_decimal,source_x_decimal,source_x_multiplicity\n")?;
    let mut digest = blake3::Hasher::new();
    let mut all_digest = blake3::Hasher::new();
    let mut multiplicity = BTreeMap::<u32, usize>::new();
    for (&key, &(source_x, matches)) in &classes {
        *multiplicity.entry(matches).or_default() += 1;
        let line = format!("{key},{source_x},{matches}\n");
        all_writer.write_all(line.as_bytes())?;
        all_digest.update(line.as_bytes());
        if selected.binary_search_by_key(&key, |(k, _)| **k).is_ok() {
            writer.write_all(line.as_bytes())?;
            digest.update(line.as_bytes());
        }
    }
    writer.flush()?;
    all_writer.flush()?;
    let summary = json!({
        "schema": "n83.direct-subgroup-x-orbit-census/v1",
        "status": if selected_count == K { "SIZE_PREMISE_PASS" } else { "SIZE_PREMISE_FAIL" },
        "curve_id": CURVE,
        "source_x_rule": "polynomial_basis_low_bits_0_through_dimension_minus_1",
        "source_dimension": dimension,
        "enumerated_x": 1u64 << dimension,
        "rational_x": rational_x,
        "subgroup_x": subgroup_x,
        "subgroup_short_orbit_x": subgroup_short_orbit_x,
        "full_signed_frobenius_orbit_classes": classes.len(),
        "selected_columns": selected_count,
        "selected_full_closure_points": selected_count * 166,
        "subspace_x_per_orbit_histogram": multiplicity,
        "selected_representatives_blake3": digest.finalize().to_hex().to_string(),
        "all_orbit_classes_blake3": all_digest.finalize().to_hex().to_string(),
        "generic_curve_subgroup_frobenius_samples": generic_sample_checks,
        "source_blake3": blake3::hash(SOURCE).to_hex().to_string(),
        "curve_initialization_ms": init_ms,
        "census_ms": census_ms,
        "process_wall_ms": started.elapsed().as_secs_f64() * 1000.0,
        "selected_best_total_runtime": serde_json::Value::Null,
    });
    let mut summary_file = BufWriter::new(File::create_new(output.join("summary.json"))?);
    serde_json::to_writer_pretty(&mut summary_file, &summary)?;
    summary_file.write_all(b"\n")?;
    summary_file.flush()?;
    println!("{}", serde_json::to_string(&summary)?);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        return Err("usage: n83_d18_subgroup_orbit_census NEW_DIR [dimension]".into());
    }
    let dimension = args.get(2).map_or(Ok(18), |d| d.parse::<u32>())?;
    census(Path::new(&args[1]), dimension)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frobenius_key_is_phase_invariant() {
        let kc = KoblitzCurve::known_n83_k0().unwrap();
        let curve = FastBinaryCurve128::new(&kc.curve.irreducible, 0).unwrap();
        let x = 2u128;
        let (key, length) = orbit_key(&curve, x).unwrap();
        assert_eq!(length, 83);
        assert_eq!(orbit_key(&curve, curve.gf.sqr(x)).unwrap(), (key, length));
        assert_eq!(
            orbit_key(&curve, curve.gf.sqr_k(x, 82)).unwrap(),
            (key, length)
        );
        assert_eq!(orbit_key(&curve, 1).unwrap(), (1, 1));
    }
}
