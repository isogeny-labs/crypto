//! Independent N83 K2048 m5 external-XOR and group replay in Rust.
//! Usage: n83_k2048_m5_native_replay XCNF SOLVER_STDOUT EXPORT_JSON OBJECT_GZ NEW_RECEIPT

use crypto_lib::hash::sha256::Sha256;
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Point = Option<(u128, u128)>;
const N: usize = 83;
const MOD: u128 = (1u128 << N) | (1u128 << 45) | 7;
const ORDER: u128 = 2_417_851_639_230_796_216_685_689;
const OBJECT_SHA256: &str = "aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a";
const POSITIVE_XCNF_SHA256: &str =
    "f46a900495ec1a3594add973ee641de21a318a8fda6c9bfa67bd19057326d44c";

fn sha_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut sha = Sha256::new();
    let mut buffer = [0u8; 1024 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        sha.update(&buffer[..count]);
    }
    Ok(hex::encode(sha.finalize()))
}

fn mul(mut a: u128, mut b: u128) -> u128 {
    let mut out = 0u128;
    while b != 0 {
        if b & 1 != 0 {
            out ^= a;
        }
        b >>= 1;
        a <<= 1;
        if a >> N != 0 {
            a ^= MOD;
        }
    }
    out
}

fn inverse(a: u128) -> Result<u128> {
    if a == 0 {
        return Err("zero field inverse".into());
    }
    let mut power = (1u128 << N) - 2;
    let mut base = a;
    let mut out = 1u128;
    while power != 0 {
        if power & 1 != 0 {
            out = mul(out, base);
        }
        base = mul(base, base);
        power >>= 1;
    }
    if mul(a, out) != 1 {
        return Err("field inverse check failed".into());
    }
    Ok(out)
}

fn on_curve(p: Point) -> bool {
    let Some((x, y)) = p else { return true };
    x < 1u128 << N && y < 1u128 << N && (mul(y, y) ^ mul(x, y) ^ mul(mul(x, x), x) ^ 1) == 0
}

fn add(p: Point, q: Point) -> Result<Point> {
    let (Some((x, y)), Some((u, v))) = (p, q) else {
        return Ok(p.or(q));
    };
    if x == u && y != v {
        if y ^ v != x {
            return Err("same-x points are not inverses".into());
        }
        return Ok(None);
    }
    let (rx, ry) = if x == u {
        if x == 0 {
            return Ok(None);
        }
        let slope = x ^ mul(y, inverse(x)?);
        let rx = mul(slope, slope) ^ slope;
        (rx, mul(x, x) ^ mul(slope ^ 1, rx))
    } else {
        let slope = mul(y ^ v, inverse(x ^ u)?);
        let rx = mul(slope, slope) ^ slope ^ x ^ u;
        (rx, mul(slope, x ^ rx) ^ rx ^ y)
    };
    let result = Some((rx, ry));
    if !on_curve(result) {
        return Err("group addition left curve".into());
    }
    Ok(result)
}

fn scalar(mut p: Point, mut k: u128) -> Result<Point> {
    let mut out = None;
    while k != 0 {
        if k & 1 != 0 {
            out = add(out, p)?;
        }
        p = add(p, p)?;
        k >>= 1;
    }
    Ok(out)
}

fn point(value: &Value) -> Result<Point> {
    let pair = value.as_array().ok_or("point is not a pair")?;
    if pair.len() != 2 {
        return Err("point coordinate count differs".into());
    }
    let x: u128 = pair[0].as_str().ok_or("point x not decimal")?.parse()?;
    let y: u128 = pair[1].as_str().ok_or("point y not decimal")?.parse()?;
    let p = Some((x, y));
    if !on_curve(p) {
        return Err("stored point fails curve equation".into());
    }
    Ok(p)
}

fn model(stdout: &Path, variables: usize) -> Result<Vec<u8>> {
    let mut assignment = vec![0u8; variables + 1];
    let mut statuses = Vec::new();
    for line in BufReader::new(File::open(stdout)?).lines() {
        let line = line?;
        if line.starts_with("s ") {
            statuses.push(line);
        } else if let Some(rest) = line.strip_prefix("v ") {
            for token in rest.split_whitespace() {
                let lit: i32 = token.parse()?;
                if lit == 0 {
                    continue;
                }
                let index = lit.unsigned_abs() as usize;
                if index == 0 || index > variables || assignment[index] != 0 {
                    return Err("model variable range or duplication".into());
                }
                assignment[index] = if lit > 0 { 1 } else { 2 };
            }
        }
    }
    if statuses != ["s SATISFIABLE"] || assignment[1..].contains(&0) {
        return Err("solver model is absent or incomplete".into());
    }
    Ok(assignment)
}

fn check_xcnf(path: &Path, assignment: &[u8]) -> Result<(usize, usize)> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut header = String::new();
    reader.read_line(&mut header)?;
    if header.trim_end() != "p cnf 111478 1146201" || assignment.len() != 111_479 {
        return Err("positive XCNF header or model size differs".into());
    }
    let mut cnf = 0usize;
    let mut xor = 0usize;
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("c ") {
            continue;
        }
        let (is_xor, body) = if let Some(rest) = line.strip_prefix("x ") {
            (true, rest)
        } else {
            (false, line.as_str())
        };
        let literals = body
            .split_whitespace()
            .map(str::parse::<i32>)
            .collect::<std::result::Result<Vec<_>, _>>()?;
        if literals.last() != Some(&0) || literals.len() < 2 {
            return Err("XCNF row lacks terminal zero or literals".into());
        }
        let mut parity = false;
        let mut satisfied = false;
        for &literal in &literals[..literals.len() - 1] {
            let index = literal.unsigned_abs() as usize;
            if index == 0 || index >= assignment.len() {
                return Err("XCNF literal outside model".into());
            }
            let truth = (assignment[index] == 1) == (literal > 0);
            parity ^= truth;
            satisfied |= truth;
        }
        if is_xor {
            xor += 1;
            if !parity {
                return Err("original XOR row fails".into());
            }
        } else {
            cnf += 1;
            if !satisfied {
                return Err("original CNF clause fails".into());
            }
        }
    }
    if (cnf, xor) != (1_141_221, 4_980) {
        return Err("original XCNF row counts differ".into());
    }
    Ok((cnf, xor))
}

fn x_block(assignment: &[u8], block: usize) -> u128 {
    (0..N).fold(0u128, |x, bit| {
        x | ((assignment[block * N + bit + 1] == 1) as u128) << bit
    })
}

#[derive(Clone)]
struct Entry {
    point: Point,
    column: usize,
    coefficient: u128,
}

fn choose_lift(
    groups: &[Vec<Entry>],
    reps: &[Point],
    intermediates: &[u128],
    target: Point,
) -> Result<Vec<Entry>> {
    fn search(
        groups: &[Vec<Entry>],
        reps: &[Point],
        intermediates: &[u128],
        target: Point,
        depth: usize,
        sum: Point,
        chosen: &mut Vec<Entry>,
    ) -> Result<bool> {
        if depth == groups.len() {
            return Ok(sum == target);
        }
        for entry in &groups[depth] {
            if entry.column >= reps.len()
                || entry.coefficient >= ORDER
                || scalar(reps[entry.column], entry.coefficient)? != entry.point
            {
                continue;
            }
            let next = add(sum, entry.point)?;
            if (1..=3).contains(&depth) && next.map(|(x, _)| x) != Some(intermediates[depth - 1]) {
                continue;
            }
            chosen.push(entry.clone());
            if search(groups, reps, intermediates, target, depth + 1, next, chosen)? {
                return Ok(true);
            }
            chosen.pop();
        }
        Ok(false)
    }
    let mut chosen = Vec::new();
    if !search(groups, reps, intermediates, target, 0, None, &mut chosen)? {
        return Err("SAT x blocks have no checked group/row lift".into());
    }
    Ok(chosen)
}

fn replay(xcnf: &Path, stdout: &Path, export: &Path, object: &Path, output: &Path) -> Result<()> {
    if output.exists()
        || sha_file(xcnf)? != POSITIVE_XCNF_SHA256
        || sha_file(object)? != OBJECT_SHA256
    {
        return Err("native replay output or input hash differs".into());
    }
    let export_record: Value = serde_json::from_slice(&fs::read(export)?)?;
    if export_record["status"] != "PASS_XCNF_EXPORT_ONLY"
        || export_record["mode"] != "positive"
        || export_record["xcnf_sha256"] != POSITIVE_XCNF_SHA256
    {
        return Err("native replay export identity differs".into());
    }
    let assignment = model(stdout, 111_478)?;
    let (cnf, xor) = check_xcnf(xcnf, &assignment)?;
    let xs = (0..5)
        .map(|block| x_block(&assignment, block))
        .collect::<Vec<_>>();
    let intermediates = (5..8)
        .map(|block| x_block(&assignment, block))
        .collect::<Vec<_>>();
    let target = point(&export_record["target"])?;
    if scalar(target, ORDER)? != None {
        return Err("native replay target outside subgroup".into());
    }
    let mut reader = BufReader::new(GzDecoder::new(File::open(object)?));
    let mut header_line = String::new();
    reader.read_line(&mut header_line)?;
    let header: Value = serde_json::from_str(&header_line)?;
    if header["curve_a"] != 0
        || header["curve_b"] != "1"
        || header["point_count"] != 339_968
        || header["orbit_columns"] != 2_048
        || header["subgroup_order"] != ORDER.to_string()
    {
        return Err("native replay archive curve or base differs".into());
    }
    let reps = header["representatives"]
        .as_array()
        .ok_or("archive representatives absent")?
        .iter()
        .map(point)
        .collect::<Result<Vec<_>>>()?;
    if reps.len() != 2_048 {
        return Err("archive representative count differs".into());
    }
    let mut found = BTreeMap::<u128, Vec<Entry>>::new();
    for &x in &xs {
        found.entry(x).or_default();
    }
    let mut rows = 0usize;
    for line in reader.lines() {
        let row: Value = serde_json::from_str(&line?)?;
        rows += 1;
        let p = point(&row["point"])?;
        let x = p.ok_or("archive row identity")?.0;
        if let Some(group) = found.get_mut(&x) {
            group.push(Entry {
                point: p,
                column: row["column"].as_u64().ok_or("archive column absent")? as usize,
                coefficient: row["coefficient"]
                    .as_str()
                    .ok_or("archive coefficient absent")?
                    .parse()?,
            });
        }
    }
    if rows != 339_968
        || found
            .values()
            .any(|group| group.is_empty() || group.len() > 2)
    {
        return Err("archive row count or SAT x lifts differ".into());
    }
    let groups = xs.iter().map(|x| found[x].clone()).collect::<Vec<_>>();
    let chosen = choose_lift(&groups, &reps, &intermediates, target)?;
    if chosen.len() != 5 {
        return Err("native replay summand count differs".into());
    }
    let mut row = vec![0u128; 2_048];
    for entry in &chosen {
        if scalar(entry.point, ORDER)? != None {
            return Err("relation point outside subgroup".into());
        }
        row[entry.column] = (row[entry.column] + entry.coefficient) % ORDER;
    }
    let mut row_sum = None;
    for (column, &coefficient) in row.iter().enumerate() {
        if coefficient != 0 {
            row_sum = add(row_sum, scalar(reps[column], coefficient)?)?;
        }
    }
    if row_sum != target {
        return Err("relation row sum differs from target".into());
    }
    let decimals = row.iter().map(ToString::to_string).collect::<Vec<_>>();
    let row_digest = blake3::hash(&serde_json::to_vec(&decimals)?)
        .to_hex()
        .to_string();
    if export_record["positive"]["relation_row_decimal_blake3"] != row_digest {
        return Err("positive relation row digest differs".into());
    }
    let report = json!({
        "schema":"n83.k2048-m5-native-positive-replay/v1",
        "status":"PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY",
        "xcnf_sha256":POSITIVE_XCNF_SHA256,"solver_stdout_sha256":sha_file(stdout)?,
        "object_sha256":OBJECT_SHA256,"export_receipt_sha256":sha_file(export)?,
        "replay_binary_sha256":sha_file(&std::env::current_exe()?)?,
        "checked_cnf_clauses":cnf,"checked_xor_rows":xor,
        "checked_archive_rows":rows,"group_lift_count":chosen.len(),
        "x_coordinates":xs.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "intermediate_x_coordinates":intermediates.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "target":export_record["target"],
        "columns":chosen.iter().map(|entry| entry.column).collect::<Vec<_>>(),
        "coefficients":chosen.iter().map(|entry| entry.coefficient.to_string()).collect::<Vec<_>>(),
        "relation_row_decimal_blake3":row_digest,"selected_best_total_runtime":Value::Null,
    });
    let mut file = File::create_new(output)?;
    serde_json::to_writer_pretty(&mut file, &report)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!(
        "{}",
        json!({"status":report["status"],"cnf":cnf,"xor":xor,"columns":report["columns"]})
    );
    Ok(())
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let result = if args.len() == 6 {
        replay(
            Path::new(&args[1]),
            Path::new(&args[2]),
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
        )
    } else {
        Err("usage: n83_k2048_m5_native_replay XCNF SOLVER_STDOUT EXPORT_JSON OBJECT_GZ NEW_RECEIPT".into())
    };
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_and_group_checks() {
        assert_eq!(mul(1, 2), 2);
        assert_eq!(mul(1 << 82, 2), (1 << 45) | 7);
        let x = 6_322_576_162_995_772_655_031_682;
        let y = 7_602_502_822_968_073_983_643_739;
        let p = Some((x, y));
        assert!(on_curve(p));
        assert_eq!(scalar(p, ORDER).unwrap(), None);
        assert_eq!(add(p, p).unwrap(), scalar(p, 2).unwrap());
        assert_eq!(mul(39, inverse(39).unwrap()), 1);
    }
}
