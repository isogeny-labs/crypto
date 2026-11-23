use std::collections::BTreeMap;
type FieldPoly = BTreeMap<Vec<usize>, u128>;

#[derive(Clone, Copy)]
struct WitnessField {
    n: usize,
    low: u128,
}
impl WitnessField {
    fn mul(self, mut a: u128, mut b: u128) -> u128 {
        let mask = (1u128 << self.n) - 1;
        assert!(a <= mask && b <= mask);
        let mut r = 0;
        while b != 0 {
            if b & 1 != 0 {
                r ^= a;
            }
            b >>= 1;
            let carry = a >> (self.n - 1);
            a = (a << 1) & mask;
            if carry != 0 {
                a ^= self.low;
            }
        }
        r
    }
}
fn fp_add(p: &mut FieldPoly, m: Vec<usize>, c: u128) {
    if c == 0 {
        return;
    }
    let entry = p.entry(m.clone()).or_default();
    *entry ^= c;
    if *entry == 0 {
        p.remove(&m);
    }
}
fn fp_mul(f: WitnessField, a: &FieldPoly, b: &FieldPoly) -> FieldPoly {
    let mut p = FieldPoly::new();
    // Coefficients repeat heavily in polynomial-basis S3 systems. This cache
    // changes only field multiplication work, never the polynomial operation.
    let mut products = BTreeMap::new();
    for (m, c) in a {
        for (n, d) in b {
            let coefficient = *products.entry((*c, *d)).or_insert_with(|| f.mul(*c, *d));
            let mut monomial = m.clone();
            monomial.extend(n);
            monomial.sort_unstable();
            fp_add(&mut p, monomial, coefficient);
        }
    }
    p
}
fn fp_nf(p: &FieldPoly) -> FieldPoly {
    let mut q = FieldPoly::new();
    for (m, c) in p {
        let mut m = m.clone();
        m.dedup();
        fp_add(&mut q, m, *c);
    }
    q
}
fn fp_boolean_mul(f: WitnessField, a: &FieldPoly, b: &FieldPoly) -> FieldPoly {
    fp_nf(&fp_mul(f, a, b))
}
fn fp_square(f: WitnessField, p: &FieldPoly) -> FieldPoly {
    p.iter().map(|(m, c)| (m.clone(), f.mul(*c, *c))).collect()
}
fn expected_s3(f: WitnessField, x: &FieldPoly, y: &FieldPoly, u: &FieldPoly, b: u128) -> FieldPoly {
    let (xs, ys, us) = (fp_square(f, x), fp_square(f, y), fp_square(f, u));
    let mut p = fp_boolean_mul(f, &xs, &ys);
    for q in [
        fp_boolean_mul(f, &xs, &us),
        fp_boolean_mul(f, &ys, &us),
        fp_boolean_mul(f, &fp_boolean_mul(f, x, y), u),
    ] {
        for (m, c) in q {
            fp_add(&mut p, m, c);
        }
    }
    fp_add(&mut p, vec![], b);
    p
}
fn fp_coordinate(p: &FieldPoly, k: usize) -> Poly {
    p.iter()
        .filter(|(_, c)| (*c >> k) & 1 != 0)
        .map(|(m, _)| m.clone())
        .collect()
}
fn checked_first_s3(
    input: &Value,
    text: &str,
) -> (WitnessField, FieldPoly, Vec<(String, FieldPoly)>) {
    assert_eq!(input["n"], 83);
    assert_eq!(input["m"], 3);
    assert_eq!(input["n_vars"], 164);
    assert_eq!(input["field_modulus"], "z^83+z^45+z^2+z+1");
    let f = WitnessField {
        n: 83,
        low: (1u128 << 45) | 7,
    };
    let bits: Vec<usize> = if input["coordinate_bit_positions"].is_array() {
        input["coordinate_bit_positions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap() as usize)
            .collect()
    } else {
        (0..27).collect()
    };
    assert_eq!(bits.len(), 27);
    assert!(bits.iter().all(|i| *i < 83));
    assert_eq!(bits.iter().collect::<BTreeSet<_>>().len(), 27);
    let linear = |start: usize, positions: &[usize]| -> FieldPoly {
        positions
            .iter()
            .enumerate()
            .map(|(i, b)| (vec![start + i], 1u128 << b))
            .collect()
    };
    let blocks = vec![
        ("X0".into(), linear(0, &bits)),
        ("X1".into(), linear(27, &bits)),
        ("U".into(), linear(81, &(0..83).collect::<Vec<_>>())),
    ];
    let polys = equations(text, 164);
    assert_eq!(
        polys.len(),
        input["input_equations"].as_u64().unwrap() as usize
    );
    let mut actual = FieldPoly::new();
    for (k, (_, p)) in polys.iter().take(83).enumerate() {
        assert!(
            !p.is_empty() && degree(p) == 3,
            "all83 cubic coordinates must exist in saved order"
        );
        for m in p {
            fp_add(&mut actual, m.clone(), 1u128 << k);
        }
    }
    let expected = expected_s3(
        f,
        &blocks[0].1,
        &blocks[1].1,
        &blocks[2].1,
        input["curve_b"].as_str().unwrap().parse().unwrap(),
    );
    assert_eq!(
        actual, expected,
        "complete first S3 source coefficients differ"
    );
    (f, actual, blocks)
}
fn field_poly_bytes(f: WitnessField, p: &FieldPoly) -> Vec<u8> {
    let mut b = b"N83F4P1\n".to_vec();
    b.extend((f.n as u32).to_le_bytes());
    b.extend((p.len() as u64).to_le_bytes());
    for (m, c) in p {
        assert!(m.len() <= 4 && *c != 0 && *c < (1u128 << f.n));
        b.push(m.len() as u8);
        for i in m {
            b.extend((*i as u16).to_le_bytes());
        }
        b.extend(c.to_le_bytes());
    }
    b
}
fn row_bytes(row: &[u64]) -> Vec<u8> {
    row.iter().flat_map(|w| w.to_le_bytes()).collect()
}
struct WitnessSink {
    writer: Option<BufWriter<File>>,
    reader: Option<BufReader<File>>,
}
impl WitnessSink {
    fn new(path: &Path, replay: bool) -> Self {
        if replay {
            Self {
                writer: None,
                reader: Some(BufReader::new(File::open(path).unwrap())),
            }
        } else {
            Self {
                writer: Some(BufWriter::new(File::create(path).unwrap())),
                reader: None,
            }
        }
    }
    fn emit(&mut self, bytes: &[u8]) {
        if let Some(w) = self.writer.as_mut() {
            w.write_all(bytes).unwrap();
        } else {
            let mut got = vec![0; bytes.len()];
            self.reader.as_mut().unwrap().read_exact(&mut got).unwrap();
            assert_eq!(got, bytes, "stored witness bytes differ");
        }
    }
    fn flush(&mut self) {
        if let Some(w) = self.writer.as_mut() {
            w.flush().unwrap();
        }
    }
    fn finish(mut self) {
        self.flush();
        if let Some(r) = self.reader.as_mut() {
            let mut b = [0];
            assert_eq!(r.read(&mut b).unwrap(), 0, "trailing witness bytes");
        }
    }
}
fn witness_file(path: &Path, bytes: &[u8], replay: bool) {
    let mut sink = WitnessSink::new(path, replay);
    sink.emit(bytes);
    sink.finish();
}
fn witness_json(path: &Path, value: &Value, replay: bool) {
    witness_file(
        path,
        (serde_json::to_string_pretty(value).unwrap() + "\n").as_bytes(),
        replay,
    )
}
fn witness_reduce(
    row: &[u64],
    basis: &[Vec<u64>],
    map: &[Option<usize>],
) -> (Vec<u64>, Vec<usize>) {
    let mut r = row.to_vec();
    let mut used = vec![];
    while let Some(p) = pivot(&r) {
        if let Some(id) = map[p] {
            xor(&mut r, &basis[id]);
            used.push(id);
        } else {
            break;
        }
    }
    (r, used)
}
fn check_witness_reduction(
    row: &[u64],
    basis: &[Vec<u64>],
    sp: &Space,
    remainder: &[u64],
    reducers: &[usize],
) {
    assert_eq!(row.len(), sp.words);
    assert_eq!(remainder.len(), sp.words);
    if sp.cols % 64 != 0 {
        assert_eq!(row[sp.words - 1] >> (sp.cols % 64), 0);
        assert_eq!(remainder[sp.words - 1] >> (sp.cols % 64), 0);
    }
    let map = pivot_map(basis, sp);
    let (expected, ids) = witness_reduce(row, basis, &map);
    assert_eq!(remainder, expected);
    assert_eq!(reducers, ids);
    if let Some(p) = pivot(remainder) {
        assert!(map[p].is_none());
    }
}
fn candidate_polynomial(
    f: WitnessField,
    source: &FieldPoly,
    multiplier: &FieldPoly,
) -> (FieldPoly, FieldPoly, usize) {
    assert!(source
        .keys()
        .all(|m| m.len() <= 3 && m.windows(2).all(|w| w[0] < w[1])));
    assert!(multiplier.keys().all(|m| m.len() == 1));
    let ordinary = fp_mul(f, source, multiplier);
    assert!(ordinary.keys().all(|m| m.len() <= 4));
    let mut correction_terms = 0;
    for m in ordinary.keys() {
        let repeats = m.windows(2).filter(|w| w[0] == w[1]).count();
        assert!(
            repeats <= 1,
            "linear times multilinear has at most one repeated variable"
        );
        correction_terms += repeats;
    }
    let normal = fp_nf(&ordinary);
    assert!(
        normal.keys().all(|m| m.len() <= 3),
        "quartic terms did not cancel"
    );
    (ordinary, normal, correction_terms)
}
fn evaluate_witness_family(
    f: WitnessField,
    source: &FieldPoly,
    blocks: &[(String, FieldPoly)],
    basis: &[Vec<u64>],
    sp: &Space,
    out: &Path,
    replay: bool,
) -> Value {
    if !replay {
        fs::create_dir(out).unwrap();
    }
    assert_eq!(sp.cap, 3);
    let map = pivot_map(basis, sp);
    witness_file(
        &out.join("source.fieldpoly"),
        &field_poly_bytes(f, source),
        replay,
    );
    let count = blocks.len() * f.n;
    let mut header = b"N83F4R1\n".to_vec();
    header.extend((sp.n as u32).to_le_bytes());
    header.extend((sp.cols as u64).to_le_bytes());
    header.extend((sp.words as u64).to_le_bytes());
    header.extend((count as u64).to_le_bytes());
    let mut candidates = WitnessSink::new(&out.join("candidates.rows.u64le"), replay);
    let mut remainders = WitnessSink::new(&out.join("remainders.rows.u64le"), replay);
    let mut trace = WitnessSink::new(&out.join("candidates.jsonl"), replay);
    candidates.emit(&header);
    remainders.emit(&header);
    let mut strict = 0;
    let mut first = None;
    let mut rows = vec![];
    let mut block_records = vec![];
    for (block, linear) in blocks {
        println!("candidate_block {block}");
        let (ordinary, normal, corrections) = candidate_polynomial(f, source, linear);
        let ordinary_bytes = field_poly_bytes(f, &ordinary);
        let normal_bytes = field_poly_bytes(f, &normal);
        witness_file(
            &out.join(format!("{block}.ordinary.fieldpoly")),
            &ordinary_bytes,
            replay,
        );
        witness_file(
            &out.join(format!("{block}.normal.fieldpoly")),
            &normal_bytes,
            replay,
        );
        let block_record = json!({"block":block,"multiplier":linear.iter().map(|(m,c)|json!({"variable":m[0],"coefficient":c.to_string()})).collect::<Vec<_>>(),"ordinary_terms":ordinary.len(),"ordinary_max_degree":ordinary.keys().map(Vec::len).max().unwrap_or(0),"normal_terms":normal.len(),"normal_max_degree":normal.keys().map(Vec::len).max().unwrap_or(0),"boolean_correction_terms":corrections,"ordinary_sha256":sha(&ordinary_bytes),"normal_sha256":sha(&normal_bytes)});
        block_records.push(block_record);
        for coordinate in 0..f.n {
            let id = rows.len();
            let candidate = sp.row(&fp_coordinate(&normal, coordinate));
            let (remainder, reducers) = witness_reduce(&candidate, basis, &map);
            let nonmember = pivot(&remainder).is_some();
            if nonmember {
                strict += 1;
                if first.is_none() {
                    first = Some(id);
                }
            }
            let cb = row_bytes(&candidate);
            let rb = row_bytes(&remainder);
            let row = json!({"id":id,"block":block,"coordinate":coordinate,"row_offset_bytes":36+id*sp.words*8,"candidate_degree":sp.deg(&candidate),"candidate_sha256":sha(&cb),"reducers":reducers,"remainder_sha256":sha(&rb),"remainder_pivot":pivot(&remainder),"strict_degree_four_fall":nonmember});
            candidates.emit(&cb);
            remainders.emit(&rb);
            trace.emit((serde_json::to_string(&row).unwrap() + "\n").as_bytes());
            candidates.flush();
            remainders.flush();
            trace.flush();
            rows.push(row);
        }
        let partial = json!({"processed_candidates":rows.len(),"strict_candidates":strict,"first_strict_candidate":first});
        witness_json(
            &out.join(format!("partial-{:03}.json", rows.len())),
            &partial,
            replay,
        );
    }
    candidates.finish();
    remainders.finish();
    trace.finish();
    assert_eq!(rows.len(), count);
    let result = json!({"schema":"n83.strict-degree-four-family/v1","candidates":count,"strict_candidates":strict,"first_strict_candidate":first,"v3_rank":basis.len(),"n_vars":sp.n,"columns":sp.cols,"row_words":sp.words,"blocks":block_records,"strict_fall_found":strict>0,"new_last_fall_lower_bound":if strict>0{Some(4)}else{None},"last_fall_upper_bound":null,"last_fall_degree":null,"complete_v4":false,"null_scope":"No strict witness in these three field-multiplier coordinate families only; not a global upper bound"});
    witness_json(&out.join("family.json"), &result, replay);
    result
}
fn suffix_refs(path: &Path) -> Value {
    let mut refs = serde_json::Map::new();
    for name in [
        "continuation.json",
        "suffix.rows.u64le",
        "suffix.operations.jsonl",
        "closure_state.json",
        "result.json",
    ] {
        let p = path.join(name);
        refs.insert(
            name.into(),
            json!({"sha256":file_hash(&p),"bytes":fs::metadata(p).unwrap().len()}),
        );
    }
    Value::Object(refs)
}
fn require_complete_v3(verdict: &Value, state: &Replayed) {
    assert_eq!(verdict["verified"], true);
    assert_eq!(
        verdict["closure_complete"], true,
        "complete V3 required for nonmembership"
    );
    assert!(state.queue.is_empty());
}
fn degree4_case_state(
    case: &Value,
    input_root: &Path,
    v2: &Path,
    prefix: &Path,
    suffix: &Path,
    out: &Path,
    replay: bool,
) -> (Value, Replayed) {
    let id = case["id"].as_str().unwrap();
    let (input, text) = load_case(input_root, id);
    assert_eq!(
        file_hash(&input_root.join(id).join("input.json")),
        case["input_json_sha256"]
    );
    assert_eq!(sha(text.as_bytes()), case["system_sha256"]);
    assert_eq!(
        input["factor_base_manifest_sha256"],
        case["factor_base_manifest_sha256"]
    );
    assert_eq!(v2_refs(v2), case["v2"]);
    assert_eq!(prefix_refs(prefix), case["parent_files"]);
    assert_eq!(suffix_refs(suffix), case["suffix_files"]);
    println!("replay_complete_v3 {}", case["key"]);
    let (verdict, state) = replay_continuation_rows(&input, &text, v2, prefix, suffix);
    require_complete_v3(&verdict, &state);
    assert_eq!(
        verdict["last_fall_lower_bound"], 3,
        "retained inherited bound must match"
    );
    let (field, source, blocks) = checked_first_s3(&input, &text);
    let mut result = evaluate_witness_family(
        field,
        &source,
        &blocks,
        &state.rows,
        &Space::new(164, 3),
        out,
        replay,
    );
    result["key"] = case["key"].clone();
    result["input"] = input;
    result["parent_replay"] = verdict;
    result["input_binding"] = case.clone();
    result["inherited_last_fall_lower_bound"] = json!(3);
    result["last_fall_lower_bound"] = json!(if result["strict_fall_found"] == true {
        4
    } else {
        3
    });
    result["producer_replay"] = json!(true);
    result["independent_review"] = json!(false);
    witness_json(&out.join("result.json"), &result, replay);
    (result, state)
}
fn degree4_case(
    case: &Value,
    input_root: &Path,
    v2: &Path,
    prefix: &Path,
    suffix: &Path,
    out: &Path,
    replay: bool,
) -> Value {
    degree4_case_state(case, input_root, v2, prefix, suffix, out, replay).0
}

// Independent small-control implementation: squarefree monomials are variable
// masks and rows are u64 bitsets in explicitly sorted graded mask order.
fn tiny_closure(n: usize, cap: usize, initial: &[u64]) -> (Vec<u64>, Vec<usize>) {
    assert!(n <= 6);
    let mut mons: Vec<usize> = (0..1usize << n)
        .filter(|m| m.count_ones() as usize <= cap)
        .collect();
    mons.sort_by_key(|m| (m.count_ones(), *m));
    let degree = |row: u64| -> usize {
        mons.iter()
            .enumerate()
            .filter(|(i, _)| row >> i & 1 != 0)
            .map(|(_, m)| m.count_ones() as usize)
            .max()
            .unwrap_or(0)
    };
    let mut queue: VecDeque<u64> = initial
        .iter()
        .map(|r| {
            mons.iter()
                .enumerate()
                .filter(|(_, m)| r >> **m & 1 != 0)
                .fold(0, |a, (i, _)| a ^ (1u64 << i))
        })
        .collect();
    let mut pivots = vec![0u64; mons.len()];
    while let Some(mut row) = queue.pop_front() {
        while row != 0 {
            let p = 63 - row.leading_zeros() as usize;
            if pivots[p] == 0 {
                break;
            }
            row ^= pivots[p];
        }
        if row == 0 {
            continue;
        }
        let p = 63 - row.leading_zeros() as usize;
        pivots[p] = row;
        if degree(row) < cap {
            for var in 0..n {
                let mut product = 0;
                for (j, m) in mons.iter().enumerate() {
                    if row >> j & 1 != 0 {
                        let at = mons.iter().position(|x| *x == (*m | (1 << var))).unwrap();
                        product ^= 1u64 << at;
                    }
                }
                queue.push_back(product);
            }
        }
    }
    (pivots, mons)
}
fn tiny_member(mask_row: u64, closure: &(Vec<u64>, Vec<usize>)) -> bool {
    let mut r = closure
        .1
        .iter()
        .enumerate()
        .filter(|(_, m)| mask_row >> **m & 1 != 0)
        .fold(0u64, |a, (j, _)| a ^ (1u64 << j));
    while r != 0 {
        let p = 63 - r.leading_zeros() as usize;
        if closure.0[p] == 0 {
            return false;
        }
        r ^= closure.0[p];
    }
    true
}
fn degree4_controls(out: &Path) -> Value {
    fs::create_dir(out).unwrap();
    let f = WitnessField { n: 1, low: 1 };
    let mut records = vec![];
    for (name, constant, strict) in [("xyz_plus_one", true, 3), ("xyz", false, 0)] {
        let dir = out.join(name);
        fs::create_dir(&dir).unwrap();
        let text = format!(
            "I[1]=v(1)*v(2)*v(3){};\nI[2]=v(1)^2-v(1);\nI[3]=v(2)^2-v(2);\nI[4]=v(3)^2-v(3);\n",
            if constant { "+1" } else { "" }
        );
        let input = json!({"n_vars":3,"input_equations":4});
        fs::write(dir.join("system.sing"), &text).unwrap();
        save(&dir.join("input.json"), &input);
        produce(&input, &text, &dir.join("v2"), 2);
        let verdict = run_case(&input, &text, &dir.join("v2"), &dir.join("v3"));
        assert_eq!(verdict["closure_complete"], true);
        let (basis, _) = replay_space(&input, &text, &dir.join("v3"), 3);
        let mut source: FieldPoly = [(vec![0, 1, 2], 1)].into();
        if constant {
            source.insert(vec![], 1);
        }
        let blocks: Vec<_> = (0..3)
            .map(|i| (format!("X{i}"), [(vec![i], 1)].into()))
            .collect();
        let result = evaluate_witness_family(
            f,
            &source,
            &blocks,
            &basis,
            &Space::new(3, 3),
            &dir.join("witness"),
            false,
        );
        evaluate_witness_family(
            f,
            &source,
            &blocks,
            &basis,
            &Space::new(3, 3),
            &dir.join("witness"),
            true,
        );
        assert_eq!(result["strict_candidates"], strict);
        let original = (1u64 << 7) | u64::from(constant);
        let v3 = tiny_closure(3, 3, &[original]);
        let v4 = tiny_closure(3, 4, &[original]);
        for i in 0..3 {
            let candidate = (1u64 << 7) | if constant { 1u64 << (1 << i) } else { 0 };
            assert_eq!(!tiny_member(candidate, &v3), constant);
            assert!(tiny_member(candidate, &v4));
        }
        records.push(json!({"case":name,"strict_candidates":strict,"independent_tiny_v3_v4_passed":true,"fresh_replay_passed":true}));
    }
    let result = json!({"schema":"n83.degree-four-controls/v1","all_passed":true,"cases":records});
    save(&out.join("controls.json"), &result);
    result
}
fn degree4_summary(root: &Path) {
    let protocol = read(&root.join("protocol-degree4-witness.json"));
    let mut rows = vec![];
    for case in protocol["extensions"]["cases"].as_array().unwrap() {
        let key = case["key"].as_str().unwrap();
        let status = root.join("status").join(format!("{key}.json"));
        let mut row = if status.exists() {
            read(&status)
        } else {
            json!({"key":key,"run_exit_code":null,"replay_exit_code":null})
        };
        for name in [
            "arm",
            "dataset",
            "id",
            "family",
            "target_kind",
            "system_sha256",
            "factor_base_manifest_sha256",
        ] {
            row[name] = case[name].clone();
        }
        let path = root.join("cases").join(key).join("result.json");
        if path.exists() {
            let result = read(&path);
            for name in [
                "candidates",
                "strict_candidates",
                "first_strict_candidate",
                "strict_fall_found",
                "v3_rank",
                "last_fall_lower_bound",
                "last_fall_degree",
                "last_fall_upper_bound",
                "complete_v4",
            ] {
                row[name] = result[name].clone();
            }
            row["result_sha256"] = json!(file_hash(&path));
        }
        row["verified"] = json!(row["run_exit_code"] == 0 && row["replay_exit_code"] == 0);
        rows.push(row);
    }
    let verified = rows.iter().filter(|x| x["verified"] == true).count();
    let summary = json!({"schema":"n83.degree-four-witness-summary/v1","planned_cases":12,"verified_cases":verified,"all_verified":verified==12,"rows":rows,"scope":"Strict degree-four lower certificates or finite-family null results only. V4 completeness, global LFD upper bounds, regularity reduction and independent review remain unestablished."});
    save(&root.join("summary.json"), &summary);
}
fn witness_predecessor_main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("summary") => degree4_summary(Path::new(&a[2])),
        Some("selfcheck") => {
            println!("{}", degree4_controls(Path::new(&a[2])));
        }
        Some("run") | Some("replay") => {
            assert_eq!(a.len(), 8);
            let r = degree4_case(
                &read(Path::new(&a[2])),
                Path::new(&a[3]),
                Path::new(&a[4]),
                Path::new(&a[5]),
                Path::new(&a[6]),
                Path::new(&a[7]),
                a[1] == "replay",
            );
            println!(
                "{}",
                json!({"key":r["key"],"candidates":r["candidates"],"strict_candidates":r["strict_candidates"],"last_fall_lower_bound":r["last_fall_lower_bound"]})
            );
        }
        _ => panic!("selfcheck FRESH_OUT | run/replay CASE_JSON INPUT_ROOT V2 PREFIX SUFFIX OUT"),
    }
}
#[cfg(test)]
mod degree4_tests {
    use super::*;
    #[test]
    fn serialized_witness_tampering_rejected() {
        let dir =
            std::env::temp_dir().join(format!("n83-degree4-byte-guards-{}", std::process::id()));
        fs::create_dir(&dir).unwrap();
        let bytes = field_poly_bytes(WitnessField { n: 3, low: 3 }, &[(vec![0, 0, 1], 5)].into());
        let path = dir.join("valid.fieldpoly");
        witness_file(&path, &bytes, false);
        witness_file(&path, &bytes, true);
        for kind in ["header", "coefficient", "extra", "truncated"] {
            let mut changed = bytes.clone();
            match kind {
                "header" => changed[0] ^= 1,
                "coefficient" => *changed.last_mut().unwrap() ^= 1,
                "extra" => changed.push(0),
                _ => {
                    changed.pop();
                }
            }
            let bad = dir.join(format!("{kind}.fieldpoly"));
            fs::write(&bad, changed).unwrap();
            assert!(
                std::panic::catch_unwind(|| witness_file(&bad, &bytes, true)).is_err(),
                "{kind}"
            );
        }
    }
    #[test]
    fn strict_and_null_against_independent_spaces() {
        degree4_controls(
            &std::env::temp_dir().join(format!("n83-degree4-controls-{}", std::process::id())),
        );
    }
    #[test]
    fn degree_and_trace_guards() {
        let f = WitnessField { n: 1, low: 1 };
        let source: FieldPoly = [(vec![0, 1, 2], 1)].into();
        let distinct: FieldPoly = [(vec![3], 1)].into();
        assert!(std::panic::catch_unwind(|| candidate_polynomial(f, &source, &distinct)).is_err());
        let sp = Space::new(3, 3);
        let basis = vec![sp.row(&[vec![0, 1, 2]].into())];
        let candidate = sp.row(&[vec![0, 1, 2], vec![0]].into());
        let (remainder, reducers) = witness_reduce(&candidate, &basis, &pivot_map(&basis, &sp));
        check_witness_reduction(&candidate, &basis, &sp, &remainder, &reducers);
        let mut altered = remainder.clone();
        altered[0] ^= 1;
        assert!(std::panic::catch_unwind(|| check_witness_reduction(
            &candidate, &basis, &sp, &altered, &reducers
        ))
        .is_err());
        assert!(std::panic::catch_unwind(|| check_witness_reduction(
            &candidate,
            &basis,
            &sp,
            &remainder,
            &[]
        ))
        .is_err());
        let state = Replayed {
            rows: basis,
            queue: VecDeque::new(),
            steps: 1,
        };
        assert!(std::panic::catch_unwind(|| require_complete_v3(
            &json!({"verified":true,"closure_complete":false}),
            &state
        ))
        .is_err());
    }
    #[test]
    fn field_multiplication_independent_long_division() {
        let f = WitnessField { n: 3, low: 3 };
        for a in 0u128..8 {
            for b in 0u128..8 {
                let mut product = 0;
                for i in 0..3 {
                    if a >> i & 1 != 0 {
                        product ^= b << i;
                    }
                }
                for i in (3..6).rev() {
                    if product >> i & 1 != 0 {
                        product ^= 11u128 << (i - 3);
                    }
                }
                assert_eq!(f.mul(a, b), product);
            }
        }
        let x: FieldPoly = [(vec![0], 1), (vec![1], 2)].into();
        let y: FieldPoly = [(vec![2], 1), (vec![3], 4)].into();
        let u: FieldPoly = [(vec![4], 1), (vec![5], 2)].into();
        let source = expected_s3(f, &x, &y, &u, 1);
        for multiplier in [&x, &y, &u] {
            let (_, normal, _) = candidate_polynomial(f, &source, multiplier);
            assert!(normal.keys().all(|m| m.len() <= 3));
        }
    }
}
