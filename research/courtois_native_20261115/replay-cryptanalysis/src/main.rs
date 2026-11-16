//! Cross-repository field replay using cryptanalysis's actual Rust Field.
use hashed_ecdl_tensor::{binary::Field, BigUint, Limits, Meter};
use serde_json::{json, Value};
fn replay(path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let rows: Vec<Value> = serde_json::from_slice(&std::fs::read(path)?)?;
    let mut meter = Meter::new(Limits::default());
    let mut checked = 0;
    for row in rows {
        let n = row["n"].as_u64().ok_or("degree")? as usize;
        let modulus = row["modulus"].as_u64().ok_or("modulus")?;
        let field = Field::new(n, BigUint::from(modulus), &mut meter)?;
        let x = BigUint::from(row["x"].as_u64().ok_or("x")?);
        let y = BigUint::from(row["y"].as_u64().ok_or("y")?);
        if field.mul(&x, &y, &mut meter)?
            != BigUint::from(row["product"].as_u64().ok_or("product")?)
        {
            return Err("native product disagrees".into());
        }
        if let Some(inverse) = row["inverse"].as_u64() {
            if field.inverse(&x, &mut meter)? != BigUint::from(inverse) {
                return Err("native inverse disagrees".into());
            }
        }
        checked += 1;
    }
    Ok(
        json!({"schema":"courtois.cross-repo-field-replay/v1","vectors":checked,
        "reference":"cryptanalysis/bindings/rust/hashed-ecdl-tensor/src/binary.rs",
        "status":"producer_cross_check_passed","claim_standing":"pcc:unverified","meter_steps":meter.used}),
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("pass the native field-vector JSON path")?;
    println!("{}", replay(&path)?);
    Ok(())
}

#[test]
fn cross_repository_native_vectors() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../evidence/field-vectors.json"
    );
    let report = replay(path).unwrap();
    assert_eq!(report["vectors"], 256);
    println!("{report}");
}
