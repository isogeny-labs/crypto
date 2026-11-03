//! Export exact public synthetic descended ANFs for the disk row-reducer.
//! Usage: ideas_bounded_memory_export OUT N ELL M SCALAR
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_groebner::{build_decomposition_system, FieldStructure};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use num_bigint::BigUint;
use serde_json::json;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 6 { return Err("usage: OUT N ELL M SCALAR".into()); }
    let n: u32 = args[2].parse()?;
    let ell: usize = args[3].parse()?;
    let m: usize = args[4].parse()?;
    let scalar = BigUint::parse_bytes(args[5].as_bytes(), 10).ok_or("scalar")?;
    if ell == 0 || ell > n as usize { return Err("invalid subspace dimension".into()); }
    let kc = KoblitzCurve::new(1, n).ok_or("unsupported curve")?;
    let point = kc.mul(kc.generator(), &scalar);
    let BinaryPoint::Affine { x, y } = &point else { return Err("target is infinity".into()); };
    let basis: Vec<_> = (0..ell).map(|i| F2mElement::from_bit_positions(&[i as u32], n)).collect();
    let st = FieldStructure::new(n, &kc.curve.irreducible);
    let sys = build_decomposition_system(&basis, x, &kc.curve.b, m, &st).ok_or("system exceeds 64-variable monomial representation")?;
    let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&args[1])?;
    writeln!(file, "BM-ANF-1 {} {}", sys.n_vars, sys.equations.len())?;
    for p in &sys.equations {
        write!(file, "{}", p.terms.len())?;
        for term in &p.terms { write!(file, " {}", term.mask)?; }
        writeln!(file)?;
    }
    let meta = json!({"schema":"bounded-macaulay-input/v1", "curve":"K_1", "n":n,"m":m,"ell":ell,"scalar":args[5],"subgroup_order":kc.subgroup_order.to_str_radix(16),"target_x_words":x.raw_bits(),"target_y_words":y.raw_bits(),"basis":"1,z,...,z^(ell-1)","n_vars":sys.n_vars,"n_equations":sys.equations.len(),"field_structure":st,"system":sys});
    let mut metadata = std::fs::OpenOptions::new().write(true).create_new(true).open(format!("{}.json",args[1]))?;
    writeln!(metadata,"{}",serde_json::to_string_pretty(&meta)?)?;
    println!("exported {} variables {} equations to {}",sys.n_vars,sys.equations.len(),args[1]);
    Ok(())
}
