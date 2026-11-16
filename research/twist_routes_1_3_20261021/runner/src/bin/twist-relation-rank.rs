//! Exact target identifiability in archived relation matrices, without point logs.
#![allow(dead_code)]
#[path = "../../../../../src/cryptanalysis/twist_ic_field.rs"]
mod field;
#[path = "../../../../../src/hash/sha256.rs"]
mod source_sha256;
use field::{inv, Quad, Q};
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn sha(p: impl AsRef<Path>) -> Result<String> {
    Ok(hex::encode(source_sha256::sha256(&fs::read(p)?)))
}
fn analyze(rows: &[Vec<u64>], n: usize) -> (usize, Vec<usize>, Vec<u64>, Vec<Vec<u64>>) {
    let mut a = rows.to_vec();
    let mut pivots = vec![];
    let mut r = 0;
    for c in 0..n {
        let Some(k) = (r..a.len()).find(|k| a[*k][c] != 0) else {
            continue;
        };
        a.swap(k, r);
        let z = inv(a[r][c], 823);
        for x in &mut a[r] {
            *x = *x * z % 823;
        }
        for i in 0..a.len() {
            if i != r {
                let z = a[i][c];
                for j in 0..=n {
                    a[i][j] = (a[i][j] + 823 - z * a[r][j] % 823) % 823;
                }
            }
        }
        pivots.push(c);
        r += 1;
    }
    assert!(a
        .iter()
        .all(|x| x[..n].iter().any(|x| *x != 0) || x[n] == 0));
    let free: Vec<_> = (0..n).filter(|c| !pivots.contains(c)).collect();
    let mut s = vec![0; n];
    for (i, c) in pivots.iter().enumerate() {
        s[*c] = a[i][n];
    }
    let directions: Vec<_> = free
        .iter()
        .map(|c| {
            let mut v = vec![0; n];
            v[*c] = 1;
            for (i, p) in pivots.iter().enumerate() {
                v[*p] = (823 - a[i][*c]) % 823;
            }
            v
        })
        .collect();
    let dot = |r: &[u64], v: &[u64]| r.iter().zip(v).fold(0, |s, (x, y)| (s + x * y) % 823);
    for row in rows {
        assert_eq!(dot(&row[..n], &s), row[n]);
        for d in &directions {
            assert_eq!(dot(&row[..n], d), 0);
        }
    }
    (r, free, s, directions)
}
fn produce(input: &Path, out: &Path) -> Result<()> {
    if out.exists() {
        return Err("fresh output required".into());
    }
    fs::create_dir_all(out)?;
    let v: Value = serde_json::from_slice(&fs::read(input)?)?;
    let mut cases = vec![];
    let mut gp = String::from("ck(c,s)=if(!c,error(s));p=823;\n");
    let mut unique_count = 0;
    let mut ambiguous_count = 0;
    for case in v["cases"].as_array().unwrap() {
        let mut anchor = vec![0; 10];
        anchor[0] = 1;
        anchor[9] = 1;
        let mut rows = vec![anchor];
        for q in case["queries"].as_array().unwrap() {
            let j = q["offset"].as_u64().unwrap();
            for w in q["witnesses"].as_array().unwrap() {
                let mut row = vec![0; 10];
                for (i, k) in w["indices"].as_array().unwrap().iter().enumerate() {
                    let k = k.as_u64().unwrap() as usize;
                    row[k] = (row[k]
                        + if w["mask"].as_u64().unwrap() & (1 << i) == 0 {
                            1
                        } else {
                            822
                        })
                        % 823;
                }
                row[8] = 822;
                row[9] = j;
                rows.push(row);
            }
        }
        let (rank, free, solution, directions) = analyze(&rows, 9);
        let unique = directions.iter().all(|d| d[8] == 0);
        let (mi, bi) = (
            case["model"].as_u64().unwrap() as usize,
            case["base"].as_u64().unwrap(),
        );
        let (a, b) = if mi == 0 {
            (1, 15)
        } else {
            (
                v["maps"][mi - 1]["map"]["a"].as_u64().unwrap(),
                v["maps"][mi - 1]["map"]["b"].as_u64().unwrap(),
            )
        };
        let e = Quad::new(827, a, b);
        let point = |x: &Value| Some((Q(x[0].as_u64().unwrap(), 0), Q(x[1].as_u64().unwrap(), 0)));
        let gg = point(&case["generator"]);
        let qq = point(&case["target"]);
        if unique {
            unique_count += 1;
            assert_eq!(e.pm(gg, solution[8] as u128), qq);
        } else {
            ambiguous_count += 1;
            let d = directions.iter().find(|d| d[8] != 0).unwrap();
            let other: Vec<_> = solution.iter().zip(d).map(|(a, b)| (a + b) % 823).collect();
            assert_ne!(solution[8], other[8]);
            for row in &rows {
                assert_eq!(
                    row[..9]
                        .iter()
                        .zip(&other)
                        .fold(0, |s, (a, b)| (s + a * b) % 823),
                    row[9]
                );
            }
        }
        gp.push_str(&format!("M=Mat({}~)*Mod(1,p);A=M[,1..9];b=M[,10];K=matker(A);ck(matrank(A)=={},\"rank\");ck((sum(i=1,matsize(K)[2],K[9,i]!=0)==0)=={},\"target uniqueness\");ss=matinverseimage(A,b);ck(A*ss==b,\"particular solution\");",serde_json::to_string(&rows)?,rank,if unique{1}else{0}));
        if unique {
            gp.push_str(&format!("ck(lift(ss[9])=={},\"scalar\");E=ellinit([Mod({},827),Mod({},827)]);ck(ellmul(E,Mod(1,827)*{},lift(ss[9]))==Mod(1,827)*{},\"point verification\");",solution[8],a,b,serde_json::to_string(&case["generator"])?,serde_json::to_string(&case["target"])?));
        }
        gp.push('\n');
        cases.push(json!({"model":mi,"base":bi,"rank":rank,"unknowns":9,"free_columns":free,"particular_solution":solution,"nullspace_basis":directions,"target_identifiable":unique,"recovered_scalar":if unique{json!(solution[8])}else{Value::Null},"scalar_verified":unique}));
    }
    gp.push_str("print(\"ALL_TWIST_RELATION_RANKS_OK 18\");\n");
    fs::write(out.join("verify.gp"), gp)?;
    let result = json!({"schema":"twist.relation-rank/v1","input_sha256":sha(input)?,"cases":cases,"summary":{"systems":18,"target_identifiable":unique_count,"target_ambiguous":ambiguous_count},"no_point_logs_or_fixture_scalar_in_linear_solver":true,"performance_claim":false});
    fs::write(
        out.join("RESULTS.json"),
        serde_json::to_string_pretty(&result)? + "\n",
    )?;
    println!("{}", result["summary"]);
    Ok(())
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("INPUT_RESULTS FRESH_OUT".into());
    }
    produce(Path::new(&a[0]), Path::new(&a[1]))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deficient_but_target_unique() {
        let (r, _, s, k) = analyze(&[vec![1, 0, 0, 7], vec![0, 0, 1, 13]], 3);
        assert_eq!(r, 2);
        assert_eq!(s[2], 13);
        assert!(k.iter().all(|v| v[2] == 0));
    }
    #[test]
    fn deficient_and_target_ambiguous() {
        let (r, _, _, k) = analyze(&[vec![1, 0, 7]], 2);
        assert_eq!(r, 1);
        assert!(k.iter().any(|v| v[1] != 0));
    }
}
