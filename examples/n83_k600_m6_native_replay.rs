//! Independent N83 K600 m6 external-XOR, archive and group replay in Rust.
//! Usage: n83_k600_m6_native_replay SOURCE_COMMIT BINARY_SHA256 INPUT_ROOT EXPORT_JSON XCNF SOLVER_RECEIPT SOLVER_STDOUT NEW_RECEIPT

use crypto_lib::hash::sha256::Sha256;
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
type Point = Option<(u128, u128)>;
const N: usize = 83;
const MOD: u128 = (1u128 << N) | (1u128 << 45) | 7;
const ORDER: u128 = 2_417_851_639_230_796_216_685_689;
const CURVE: &str = "icv1-f2m83-tm6151469093347-debefd74";
const OBJECT: &str =
    "objects/b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e.jsonl.gz";
const OBJECT_SHA256: &str = "3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf";
const CORPUS_SHA256: &str = "760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd";

fn sha_bytes(bytes: &[u8]) -> String {
    let mut sha = Sha256::new();
    sha.update(bytes);
    hex::encode(sha.finalize())
}

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

fn decimal(value: &Value) -> Result<u128> {
    if let Some(text) = value.as_str() {
        return Ok(text.parse()?);
    }
    Ok(value.as_u64().ok_or("integer is not decimal or u64")? as u128)
}

fn point(value: &Value) -> Result<Point> {
    let pair = value.as_array().ok_or("point is not a pair")?;
    if pair.len() != 2 {
        return Err("point coordinate count differs".into());
    }
    let x = decimal(&pair[0])?;
    let y = decimal(&pair[1])?;
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

fn check_xcnf(
    path: &Path,
    assignment: &[u8],
    expected_variables: usize,
    expected_cnf: usize,
    expected_xor: usize,
) -> Result<(usize, usize)> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut header = String::new();
    reader.read_line(&mut header)?;
    if header.trim_end() != format!("p cnf {expected_variables} {}", expected_cnf + expected_xor)
        || assignment.len() != expected_variables + 1
    {
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
    if (cnf, xor) != (expected_cnf, expected_xor) {
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
            if (1..=4).contains(&depth) && next.map(|(x, _)| x) != Some(intermediates[depth - 1]) {
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

fn replay(
    source_commit: &str,
    binary_sha256: &str,
    input_root: &Path,
    export_path: &Path,
    xcnf: &Path,
    receipt_path: &Path,
    stdout: &Path,
    output: &Path,
) -> Result<()> {
    if source_commit.len() != 40
        || !source_commit.bytes().all(|c| c.is_ascii_hexdigit())
        || binary_sha256.len() != 64
        || !binary_sha256.bytes().all(|c| c.is_ascii_hexdigit())
    {
        return Err("source commit or binary SHA-256 binding malformed".into());
    }
    let export: Value = serde_json::from_slice(&fs::read(export_path)?)?;
    let receipt: Value = serde_json::from_slice(&fs::read(receipt_path)?)?;
    let mode = export["mode"].as_str().ok_or("export mode absent")?;
    if !["positive-pinned", "positive-unpinned", "natural"].contains(&mode)
        || export["schema"] != "n83.k600-m6-xcnf-export/v1"
        || export["status"] != "PASS_XCNF_EXPORT_ONLY"
        || export["curve_id"] != CURVE
        || export["curve_a"] != 0
        || export["source_commit"] != source_commit
        || export["source_attestation_mode"] != "clean_checkout"
        || export["binary_sha256"] != binary_sha256
        || export["object"] != OBJECT
        || export["object_sha256"] != OBJECT_SHA256
    {
        return Err("K600 export source, curve or object identity differs".into());
    }
    let shape = &export["shape"];
    let variables = shape["variables"].as_u64().ok_or("variable count absent")? as usize;
    let clauses = shape["clauses"].as_u64().ok_or("clause count absent")? as usize;
    let xor_rows = shape["xor_rows"].as_u64().ok_or("XOR count absent")? as usize;
    if shape["summands"] != 6
        || shape["identity_mask"] != 0
        || shape["root_encoding"] != "per_representative"
        || shape["domain_representatives"] != 600
        || shape["summand_ordering"] != "none"
        || shape["positive_x_pins"] != if mode == "positive-pinned" { 913 } else { 0 }
        || variables < 11 * N
    {
        return Err("K600 shape or solver policy differs".into());
    }
    let formula_sha = export["xcnf_sha256"].as_str().ok_or("formula SHA absent")?;
    if formula_sha.len() != 64
        || receipt["exit_code"] != 10
        || !["SAT_PENDING_REPLAY", "SAT_PENDING_XCNF_REPLAY"]
            .iter()
            .any(|status| receipt["status"] == *status)
        || receipt["xcnf_sha256"] != formula_sha
        || sha_file(xcnf)? != formula_sha
        || fs::metadata(xcnf)?.len() != export["xcnf_bytes"].as_u64().ok_or("XCNF bytes absent")?
        || sha_file(stdout)? != receipt["stdout_sha256"]
    {
        return Err("solver SAT receipt or original XCNF bytes differ".into());
    }
    let object = input_root.join(OBJECT);
    if sha_file(&object)? != OBJECT_SHA256 {
        return Err("retained K600 factor-base bytes differ".into());
    }
    let target = point(&export["target"])?;
    if scalar(target, ORDER)?.is_some() {
        return Err("target outside the declared subgroup".into());
    }
    if mode == "natural" {
        if !export["positive"].is_null() || export["public_fixture"] != 0 {
            return Err("natural export fixture or witness differs".into());
        }
        let corpus_path = input_root.join("probe-corpus.json");
        if sha_file(&corpus_path)? != CORPUS_SHA256 {
            return Err("public corpus SHA-256 differs".into());
        }
        let corpus: Value = serde_json::from_slice(&fs::read(corpus_path)?)?;
        let fixtures = corpus["targets"]
            .as_array()
            .ok_or("public targets absent")?
            .iter()
            .filter(|row| row["a"] == 0 && row["fixture"] == 0)
            .collect::<Vec<_>>();
        if fixtures.len() != 1 || point(&fixtures[0]["point"])? != target {
            return Err("natural target differs from frozen public fixture".into());
        }
    } else if export["positive"].is_null() {
        return Err("positive witness absent".into());
    }
    let assignment = model(stdout, variables)?;
    let (cnf, xor) = check_xcnf(xcnf, &assignment, variables, clauses, xor_rows)?;
    let xs = (0..6).map(|i| x_block(&assignment, i)).collect::<Vec<_>>();
    let intermediates = (6..10).map(|i| x_block(&assignment, i)).collect::<Vec<_>>();
    if x_block(&assignment, 10) != target.ok_or("identity target")?.0 {
        return Err("model target x block differs".into());
    }
    let mut reader = BufReader::new(GzDecoder::new(File::open(&object)?));
    let mut header_line = String::new();
    reader.read_line(&mut header_line)?;
    let header: Value = serde_json::from_str(&header_line)?;
    if header["curve_slug"] != CURVE
        || header["curve_a"] != 0
        || header["curve_b"] != "1"
        || header["n"] != N
        || header["closure"] != "signed_frobenius"
        || header["policy"] != "public_x_hash"
        || header["seed"] != 2_026_100_801u64
        || header["point_count"] != 99_600
        || header["orbit_columns"] != 600
        || decimal(&header["subgroup_order"])? != ORDER
    {
        return Err("factor-base archive header differs".into());
    }
    let reps = header["representatives"]
        .as_array()
        .ok_or("representatives absent")?
        .iter()
        .map(point)
        .collect::<Result<Vec<_>>>()?;
    if reps.len() != 600 || reps.iter().any(|&p| scalar(p, ORDER).ok() != Some(None)) {
        return Err("representative count or subgroup differs".into());
    }
    let positive = &export["positive"];
    let indices = if positive.is_null() {
        Vec::new()
    } else {
        positive["indices"]
            .as_array()
            .ok_or("positive indices absent")?
            .iter()
            .map(|v| v.as_u64().ok_or_else(|| "positive index invalid".into()))
            .collect::<Result<Vec<_>>>()?
    };
    if !positive.is_null()
        && (indices.len() != 6
            || indices.iter().any(|&i| i >= 99_600)
            || indices.iter().copied().collect::<BTreeSet<_>>().len() != 6)
    {
        return Err("positive archive indices differ".into());
    }
    let mut found = BTreeMap::<u128, Vec<Entry>>::new();
    for &x in &xs {
        found.entry(x).or_default();
    }
    let mut indexed = BTreeMap::<u64, Point>::new();
    let mut rows = 0u64;
    for line in reader.lines() {
        let row: Value = serde_json::from_str(&line?)?;
        let p = point(&row["point"])?;
        if indices.contains(&rows) {
            indexed.insert(rows, p);
        }
        let x = p.ok_or("identity archive point")?.0;
        if let Some(group) = found.get_mut(&x) {
            let column = row["column"].as_u64().ok_or("archive column absent")? as usize;
            let coefficient = decimal(&row["coefficient"])? % ORDER;
            if column >= reps.len()
                || scalar(p, ORDER)?.is_some()
                || scalar(reps[column], coefficient)? != p
            {
                return Err("archive row/column relation fails".into());
            }
            group.push(Entry {
                point: p,
                column,
                coefficient,
            });
        }
        rows += 1;
    }
    if rows != 99_600
        || found
            .values()
            .any(|group| group.is_empty() || group.len() > 2)
    {
        return Err("archive row count or model x lifts differ".into());
    }
    if !positive.is_null() {
        let points = positive["points"]
            .as_array()
            .ok_or("positive points absent")?;
        let partials = positive["partial_sums"]
            .as_array()
            .ok_or("positive partial sums absent")?;
        if points.len() != 6 || partials.len() != 6 || indexed.len() != 6 {
            return Err("positive witness length differs".into());
        }
        let mut sum = None;
        for i in 0..6 {
            let p = point(&points[i])?;
            if indexed[&indices[i]] != p || scalar(p, ORDER)?.is_some() {
                return Err("positive point differs from subgroup archive row".into());
            }
            sum = add(sum, p)?;
            if sum.is_none() || sum != point(&partials[i])? {
                return Err("positive partial group sum differs".into());
            }
        }
        if sum != target {
            return Err("positive target sum differs".into());
        }
    }
    let groups = xs.iter().map(|x| found[x].clone()).collect::<Vec<_>>();
    let chosen = choose_lift(&groups, &reps, &intermediates, target)?;
    if chosen.len() != 6 {
        return Err("six-point relation lift absent".into());
    }
    let mut relation_row = vec![0u128; 600];
    for entry in &chosen {
        relation_row[entry.column] = (relation_row[entry.column] + entry.coefficient) % ORDER;
    }
    let mut row_sum = None;
    for (column, &coefficient) in relation_row.iter().enumerate() {
        if coefficient != 0 {
            row_sum = add(row_sum, scalar(reps[column], coefficient)?)?;
        }
    }
    if row_sum != target {
        return Err("relation row group replay differs from target".into());
    }
    let report = json!({
        "schema":"n83.k600-m6-native-model-replay/v1",
        "status":"PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY",
        "mode":mode,
        "source_commit":source_commit,
        "binary_sha256":binary_sha256,
        "replay_binary_sha256":sha_file(&std::env::current_exe()?)?,
        "xcnf_sha256":formula_sha,
        "solver_stdout_sha256":receipt["stdout_sha256"],
        "object_sha256":OBJECT_SHA256,
        "model_values_sha256":sha_bytes(&assignment),
        "checked_cnf_clauses":cnf,
        "checked_xor_rows":xor,
        "checked_archive_rows":rows,
        "target":export["target"],
        "x_coordinates":xs.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "intermediate_x_coordinates":intermediates.iter().map(ToString::to_string).collect::<Vec<_>>(),
        "chosen_points":chosen.iter().map(|e| {
            let (x,y)=e.point.expect("chosen archive point");
            vec![x.to_string(),y.to_string()]
        }).collect::<Vec<_>>(),
        "columns":chosen.iter().map(|e| e.column).collect::<Vec<_>>(),
        "coefficients":chosen.iter().map(|e| e.coefficient.to_string()).collect::<Vec<_>>(),
        "relation_row_decimal_blake3":blake3::hash(&serde_json::to_vec(
            &relation_row.iter().map(ToString::to_string).collect::<Vec<_>>())?).to_hex().to_string(),
        "subgroup_order":ORDER.to_string(),
        "selected_best_total_runtime":Value::Null
    });
    let mut file = File::create_new(output)?;
    serde_json::to_writer_pretty(&mut file, &report)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    println!("{}", json!({"status":report["status"],"cnf":cnf,"xor":xor}));
    Ok(())
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let result = if args.len() == 9 {
        replay(
            &args[1],
            &args[2],
            Path::new(&args[3]),
            Path::new(&args[4]),
            Path::new(&args[5]),
            Path::new(&args[6]),
            Path::new(&args[7]),
            Path::new(&args[8]),
        )
    } else {
        Err("usage: n83_k600_m6_native_replay SOURCE_COMMIT BINARY_SHA256 INPUT_ROOT EXPORT_JSON XCNF SOLVER_RECEIPT SOLVER_STDOUT NEW_RECEIPT".into())
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
    fn field_group_and_formula_checks() {
        assert_eq!(mul(1 << 82, 2), (1 << 45) | 7);
        let p = Some((
            6_322_576_162_995_772_655_031_682,
            7_602_502_822_968_073_983_643_739,
        ));
        assert!(on_curve(p));
        assert_eq!(scalar(p, ORDER).unwrap(), None);
        assert_eq!(
            add(p, Some((p.unwrap().0, p.unwrap().0 ^ p.unwrap().1))).unwrap(),
            None
        );
        assert_eq!(mul(39, inverse(39).unwrap()), 1);
        let dir = std::env::temp_dir().join(format!(
            "n83-k600-native-replay-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&dir).unwrap();
        let formula = dir.join("small.xcnf");
        let transcript = dir.join("model.txt");
        fs::write(&formula, "p cnf 3 3\n1 -2 0\n3 0\nx -1 2 3 0\n").unwrap();
        fs::write(&transcript, "s SATISFIABLE\nv 1 -2 3 0\n").unwrap();
        let good = model(&transcript, 3).unwrap();
        assert_eq!(check_xcnf(&formula, &good, 3, 2, 1).unwrap(), (2, 1));
        fs::write(&transcript, "s SATISFIABLE\nv 1 -2 0\n").unwrap();
        assert!(model(&transcript, 3).is_err());
        assert!(check_xcnf(&formula, &[0, 2, 2, 1], 3, 2, 1).is_err());
        assert!(check_xcnf(&formula, &[0, 1, 1, 1], 3, 2, 1).is_err());
        fs::remove_dir_all(dir).unwrap();
    }
}
