const PROPAGATION_ROWS: usize = 65536;
const PROPAGATION_OPERATIONS: usize = 262144;
const AFFINE_FREE_CAP: usize = 12;

fn load_witness_rows(path: &Path, sp: &Space, count: usize) -> Vec<Vec<u64>> {
    let mut f = BufReader::new(File::open(path).unwrap());
    let mut header = vec![0; 36];
    f.read_exact(&mut header).unwrap();
    let mut expected = b"N83F4R1\n".to_vec();
    expected.extend((sp.n as u32).to_le_bytes());
    for x in [sp.cols, sp.words, count] {
        expected.extend((x as u64).to_le_bytes());
    }
    assert_eq!(header, expected, "witness row header mismatch");
    let mut rows = Vec::with_capacity(count);
    for _ in 0..count {
        let mut row = vec![0; sp.words];
        for w in &mut row {
            let mut bytes = [0; 8];
            f.read_exact(&mut bytes).unwrap();
            *w = u64::from_le_bytes(bytes);
        }
        if sp.cols % 64 != 0 {
            assert_eq!(row[sp.words - 1] >> (sp.cols % 64), 0);
        }
        rows.push(row);
    }
    let mut tail = [0];
    assert_eq!(f.read(&mut tail).unwrap(), 0, "extra witness bytes");
    rows
}

fn propagation_affine(rows: &[Vec<u64>], sp: &Space, polys: &[(usize, Poly)]) -> Value {
    let unit = rows.iter().position(|r| pivot(r) == Some(0));
    let mut linear: Vec<_> = rows
        .iter()
        .enumerate()
        .filter(|(_, r)| sp.deg(r) == 1)
        .collect();
    linear.sort_by_key(|(_, r)| pivot(r));
    let pivots: BTreeSet<_> = linear.iter().map(|(_, r)| pivot(r).unwrap() - 1).collect();
    let free: Vec<_> = (0..sp.n).filter(|v| !pivots.contains(v)).collect();
    let mut assignments = vec![];
    let mut all_valid = None;
    if unit.is_none() && free.len() <= AFFINE_FREE_CAP {
        let mut valid = true;
        for assignment in 0..(1usize << free.len()) {
            let mut values = vec![false; sp.n];
            for (j, v) in free.iter().enumerate() {
                values[*v] = assignment >> j & 1 != 0;
            }
            for (_, r) in &linear {
                let p = pivot(r).unwrap();
                values[p - 1] = bit(r, 0)
                    ^ (1..p)
                        .filter(|c| bit(r, *c))
                        .fold(false, |a, c| a ^ values[c - 1]);
            }
            let failed: Vec<_> = polys
                .iter()
                .filter(|(_, p)| evaluate(p, &values))
                .map(|(id, _)| *id)
                .collect();
            valid &= failed.is_empty();
            assignments.push(json!({"free_assignment":assignment,"values_hex_words":values.chunks(64).map(|v|format!("{:016x}",v.iter().enumerate().fold(0u64,|a,(i,b)|a|((*b as u64)<<i)))).collect::<Vec<_>>(),"failed_input_equations":failed,"satisfies_all_input_equations":failed.is_empty()}));
        }
        all_valid = Some(valid);
    }
    json!({"contains_one":unit.is_some(),"unit_row_id":unit,"linear_rank":linear.len(),"linear_basis_row_ids":linear.iter().map(|(i,_)| *i).collect::<Vec<_>>(),"free_variables_zero_based":free,"affine_free_variable_cap":AFFINE_FREE_CAP,"exhaustive_affine_completion":all_valid,"assignments_checked":assignments.len(),"assignments":assignments})
}

fn propagation_header(sp: &Space, parent_rows: usize, seeds: usize) -> Vec<u8> {
    let mut b = b"N83F4S1\n".to_vec();
    b.extend((sp.n as u32).to_le_bytes());
    for x in [sp.cols, sp.words, parent_rows, seeds] {
        b.extend((x as u64).to_le_bytes());
    }
    assert_eq!(b.len(), 44);
    b
}

// These rows have membership level4, but stored degree at most3. An empty
// restricted queue is never used to infer a global upper bound.
fn propagate_rows(
    input: &Value,
    text: &str,
    mut rows: Vec<Vec<u64>>,
    seeds: &[Vec<u64>],
    out: &Path,
    max_rows: usize,
    max_ops: usize,
    inherited_lower: Option<usize>,
    replay: bool,
) -> Value {
    let n = input["n_vars"].as_u64().unwrap() as usize;
    let sp = Space::new(n, 3);
    let polys = equations(text, n);
    let first_row = rows.len();
    assert!(max_rows >= first_row && max_ops > 0);
    assert!(inherited_lower.is_none() || inherited_lower == Some(4));
    for row in seeds {
        assert_eq!(row.len(), sp.words);
    }
    if !replay {
        fs::create_dir(out).unwrap();
    }
    let mut data = WitnessSink::new(&out.join("added.rows.u64le"), replay);
    let mut ops = WitnessSink::new(&out.join("operations.jsonl"), replay);
    let mut progress = WitnessSink::new(&out.join("progress.jsonl"), replay);
    data.emit(&propagation_header(&sp, first_row, seeds.len()));
    let mut map = pivot_map(&rows, &sp);
    let mut queue: VecDeque<Value> = (0..seeds.len())
        .map(|i| json!({"witness_candidate":i}))
        .collect();
    let mut steps = 0;
    let mut seeds_processed = 0;
    let mut seed_insertions = 0;
    let mut added_by_degree = [0usize; 4];
    while !queue.is_empty() && rows.len() < max_rows && steps < max_ops && map[0].is_none() {
        let source = queue.pop_front().unwrap();
        let seed = source["witness_candidate"].as_u64();
        let mut row = if let Some(i) = seed {
            seeds_processed += 1;
            seeds.get(i as usize).expect("seed index").clone()
        } else {
            let id = source["parent"].as_u64().unwrap() as usize;
            let v = source["variable"].as_u64().unwrap() as usize;
            assert!(
                id >= first_row && id < rows.len(),
                "only new admitted rows are queued"
            );
            assert!(
                sp.deg(&rows[id]) <= 2,
                "cubic multiplication is outside this procedure"
            );
            sp.mul(&rows[id], v)
        };
        let original_hash = sha(&row_bytes(&row));
        let mut reducers = vec![];
        while let Some(p) = pivot(&row) {
            if let Some(id) = map[p] {
                xor(&mut row[..=p / 64], &rows[id][..=p / 64]);
                reducers.push(id);
            } else {
                break;
            }
        }
        let remainder_hash = sha(&row_bytes(&row));
        let degree = sp.deg(&row);
        let inserted = if let Some(p) = pivot(&row) {
            let id = rows.len();
            map[p] = Some(id);
            data.emit(&row_bytes(&row));
            append_multiples(&mut queue, &row, id, &sp);
            rows.push(row);
            added_by_degree[degree] += 1;
            if seed.is_some() {
                seed_insertions += 1;
            }
            Some(id)
        } else {
            None
        };
        let record = json!({"step":steps,"source":source,"source_row_sha256":original_hash,"xor_basis_rows":reducers,"remainder_sha256":remainder_hash,"remainder_degree":degree,"inserted_row":inserted,"membership_degree_bound":4});
        ops.emit((serde_json::to_string(&record).unwrap() + "\n").as_bytes());
        steps += 1;
        if steps % 128 == 0 {
            data.flush();
            ops.flush();
            let p = json!({"steps":steps,"rank":rows.len(),"added_rows":rows.len()-first_row,"pending":queue.len(),"seeds_processed":seeds_processed,"contains_one":map[0].is_some()});
            progress.emit((serde_json::to_string(&p).unwrap() + "\n").as_bytes());
            progress.flush();
        }
    }
    let reason = if map[0].is_some() {
        "unit_certificate"
    } else if queue.is_empty() {
        "restricted_queue_exhausted"
    } else if rows.len() == max_rows {
        "row_cap"
    } else {
        "operation_cap"
    };
    assert!(map[0].is_some() || queue.is_empty() || rows.len() == max_rows || steps == max_ops);
    let state = json!({"restricted_queue_exhausted":queue.is_empty(),"unrestricted_v4_computed":false,"pending_candidates":queue.len(),"next_source":queue.front(),"max_rows":max_rows,"max_operations":max_ops,"processed_operations":steps,"parent_rows":first_row,"rank":rows.len(),"added_rows":rows.len()-first_row,"seeds":seeds.len(),"seeds_processed":seeds_processed,"independent_seed_rows":seed_insertions,"added_rows_by_degree":added_by_degree,"stop_reason":reason});
    progress.emit((serde_json::to_string(&state).unwrap() + "\n").as_bytes());
    data.finish();
    ops.finish();
    progress.finish();
    witness_json(&out.join("state.json"), &state, replay);
    // Preserve the whole pending queue, not just its length or first element.
    witness_json(&out.join("pending.json"), &json!(queue), replay);
    let affine = propagation_affine(&rows, &sp, &polys);
    witness_json(&out.join("affine.json"), &affine, replay);
    let upper = affine["contains_one"] == true || affine["exhaustive_affine_completion"] == true;
    let upper_arg = if affine["contains_one"] == true {
        "Verified1 in V4; multiplication gives every polynomial in Vmax(4,degree)."
    } else if upper {
        "All Boolean assignments satisfying the admitted linear equations satisfy every original equation. Their linear-plus-field ideal equals the original ideal; degree-compatible substitution gives global upper4."
    } else {
        "No global upper certificate; a restricted stall or cap gives no new higher-fall lower bound."
    };
    let result = json!({"schema":"n83.degree-four-propagation/v1","working_degree":3,"membership_degree_bound":4,"state":state,"linear_rank":affine["linear_rank"],"contains_one":affine["contains_one"],"exhaustive_affine_completion":affine["exhaustive_affine_completion"],"assignments_checked":affine["assignments_checked"],"inherited_last_fall_lower_bound":inherited_lower,"last_fall_lower_bound":inherited_lower,"last_fall_upper_bound":if upper{Some(4)}else{None},"last_fall_degree":if upper&&inherited_lower==Some(4){Some(4)}else{None},"upper_bound_argument":upper_arg,"unrestricted_v4_computed":false,"independent_review":false});
    witness_json(&out.join("propagation.json"), &result, replay);
    result
}

fn witness_parent_refs(dir: &Path) -> Value {
    let names = [
        "source.fieldpoly",
        "X0.ordinary.fieldpoly",
        "X0.normal.fieldpoly",
        "X1.ordinary.fieldpoly",
        "X1.normal.fieldpoly",
        "U.ordinary.fieldpoly",
        "U.normal.fieldpoly",
        "candidates.rows.u64le",
        "remainders.rows.u64le",
        "candidates.jsonl",
        "family.json",
        "result.json",
        "partial-083.json",
        "partial-166.json",
        "partial-249.json",
    ];
    let mut refs = serde_json::Map::new();
    for name in names {
        let p = dir.join(name);
        refs.insert(
            name.into(),
            json!({"sha256":file_hash(&p),"bytes":fs::metadata(p).unwrap().len()}),
        );
    }
    Value::Object(refs)
}

fn propagation_case(
    case: &Value,
    input_root: &Path,
    v2: &Path,
    prefix: &Path,
    suffix: &Path,
    witness: &Path,
    out: &Path,
    replay: bool,
) -> Value {
    assert_eq!(witness_parent_refs(witness), case["witness_files"]);
    let (parent, state) = degree4_case_state(
        &case["witness_case"],
        input_root,
        v2,
        prefix,
        suffix,
        witness,
        true,
    );
    assert!(state.queue.is_empty());
    assert_eq!(parent["last_fall_lower_bound"], 4);
    assert_eq!(parent["strict_candidates"], 249);
    assert_eq!(parent["candidates"], 249);
    let (input, text) = load_case(input_root, case["id"].as_str().unwrap());
    let seeds = load_witness_rows(
        &witness.join("candidates.rows.u64le"),
        &Space::new(164, 3),
        249,
    );
    let mut result = propagate_rows(
        &input,
        &text,
        state.rows,
        &seeds,
        out,
        PROPAGATION_ROWS,
        PROPAGATION_OPERATIONS,
        Some(4),
        replay,
    );
    result["key"] = case["key"].clone();
    result["input_binding"] = case.clone();
    result["witness_parent_result_sha256"] = json!(file_hash(&witness.join("result.json")));
    result["verified_parent_strict_candidates"] = parent["strict_candidates"].clone();
    witness_json(&out.join("result.json"), &result, replay);
    result
}

fn propagation_control_fixture(
    root: &Path,
    constant: bool,
) -> (Value, String, Vec<Vec<u64>>, Vec<Vec<u64>>) {
    fs::create_dir(root).unwrap();
    let text = control_text(
        &[if constant {
            "v(1)*v(2)*v(3)+1"
        } else {
            "v(1)*v(2)*v(3)"
        }],
        3,
    );
    let input = json!({"n_vars":3,"input_equations":4});
    fs::write(root.join("system.sing"), &text).unwrap();
    save(&root.join("input.json"), &input);
    produce(&input, &text, &root.join("v2"), 2);
    run_case(&input, &text, &root.join("v2"), &root.join("v3"));
    let (basis, _) = replay_space(&input, &text, &root.join("v3"), 3);
    let mut source: FieldPoly = [(vec![0, 1, 2], 1)].into();
    if constant {
        source.insert(vec![], 1);
    }
    let blocks: Vec<_> = (0..3)
        .map(|i| (format!("X{i}"), [(vec![i], 1)].into()))
        .collect();
    evaluate_witness_family(
        WitnessField { n: 1, low: 1 },
        &source,
        &blocks,
        &basis,
        &Space::new(3, 3),
        &root.join("witness"),
        false,
    );
    let seeds = load_witness_rows(
        &root.join("witness/candidates.rows.u64le"),
        &Space::new(3, 3),
        3,
    );
    (input, text, basis, seeds)
}

fn propagation_controls(out: &Path) -> Value {
    fs::create_dir(out).unwrap();
    let mut cases = vec![];
    for (name, constant) in [("xyz_plus_one", true), ("xyz", false)] {
        let root = out.join(name);
        let (input, text, basis, seeds) = propagation_control_fixture(&root, constant);
        let lower = if constant { Some(4) } else { None };
        let result = propagate_rows(
            &input,
            &text,
            basis.clone(),
            &seeds,
            &root.join("propagation"),
            64,
            1024,
            lower,
            false,
        );
        assert_eq!(
            result,
            propagate_rows(
                &input,
                &text,
                basis,
                &seeds,
                &root.join("propagation"),
                64,
                1024,
                lower,
                true
            )
        );
        assert_eq!(
            result["last_fall_degree"],
            if constant { json!(4) } else { Value::Null }
        );
        let original = (1u64 << 7) | u64::from(constant);
        let v3 = tiny_closure(3, 3, &[original]);
        let v4 = tiny_closure(3, 4, &[original]);
        for i in 0..3 {
            let linear = 1u64 | (1u64 << (1 << i));
            if constant {
                assert!(!tiny_member(linear, &v3));
                assert!(tiny_member(linear, &v4));
            }
        }
        if constant {
            assert_eq!(result["linear_rank"], 3);
            assert_eq!(result["exhaustive_affine_completion"], true);
        } else {
            assert_eq!(result["exhaustive_affine_completion"], false);
            assert!(result["last_fall_upper_bound"].is_null());
        }
        cases.push(json!({"case":name,"result":result,"independent_tiny_v3_v4_passed":true,"fresh_replay_passed":true}));
    }
    let result =
        json!({"schema":"n83.degree-four-propagation-controls/v1","all_passed":true,"cases":cases});
    save(&out.join("controls.json"), &result);
    result
}

fn propagation_summary(root: &Path) {
    let protocol = read(&root.join("protocol-degree4-propagation.json"));
    let mut rows = vec![];
    for case in protocol["extensions"]["cases"].as_array().unwrap() {
        let key = case["key"].as_str().unwrap();
        let status_path = root.join("status").join(format!("{key}.json"));
        let status = if status_path.exists() {
            read(&status_path)
        } else {
            json!({"run_exit_code":null,"replay_exit_code":null})
        };
        let ok = status["run_exit_code"] == 0 && status["replay_exit_code"] == 0;
        let mut row = json!({"key":key,"arm":case["arm"],"dataset":case["dataset"],"family":case["family"],"target_kind":case["target_kind"],"verified":ok,"run_exit_code":status["run_exit_code"],"replay_exit_code":status["replay_exit_code"]});
        if ok {
            let result = read(&root.join("cases").join(key).join("result.json"));
            for field in [
                "linear_rank",
                "contains_one",
                "exhaustive_affine_completion",
                "assignments_checked",
                "inherited_last_fall_lower_bound",
                "last_fall_lower_bound",
                "last_fall_upper_bound",
                "last_fall_degree",
                "unrestricted_v4_computed",
            ] {
                row[field] = result[field].clone();
            }
            row["state"] = result["state"].clone();
        }
        rows.push(row);
    }
    save(
        &root.join("summary.json"),
        &json!({"schema":"n83.degree-four-propagation-summary/v1","verified_cases":rows.iter().filter(|r|r["verified"]==true).count(),"rows":rows,"scope":"Restricted degree3 propagation of verified V4 consequences; global upper only from unit or exhaustive affine completion; inherited lower4; no higher-fall inference from caps or stalls"}),
    );
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("run") | Some("replay") => {
            assert_eq!(a.len(), 9);
            let result = propagation_case(
                &read(Path::new(&a[2])),
                Path::new(&a[3]),
                Path::new(&a[4]),
                Path::new(&a[5]),
                Path::new(&a[6]),
                Path::new(&a[7]),
                Path::new(&a[8]),
                a[1] == "replay",
            );
            println!(
                "{}",
                json!({"key":result["key"],"added_rows":result["state"]["added_rows"],"stop_reason":result["state"]["stop_reason"],"last_fall_lower_bound":result["last_fall_lower_bound"],"last_fall_upper_bound":result["last_fall_upper_bound"],"last_fall_degree":result["last_fall_degree"]})
            );
        }
        Some("selfcheck") => {
            assert_eq!(a.len(), 3);
            println!("{}", propagation_controls(Path::new(&a[2])));
        }
        Some("summary") => {
            assert_eq!(a.len(), 3);
            propagation_summary(Path::new(&a[2]));
        }
        _ => panic!(
            "run|replay CASE INPUT_ROOT V2 PREFIX SUFFIX WITNESS OUT; selfcheck OUT; summary ROOT"
        ),
    }
}

#[cfg(test)]
mod propagation_tests {
    use super::*;
    fn tmp(tag: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("n83-propagation-{}-{tag}", std::process::id()))
    }
    #[test]
    fn global_upper_positive_and_restricted_negative() {
        propagation_controls(&tmp("controls"));
    }
    #[test]
    fn caps_replay_and_affine_certificate_with_pending_queue() {
        for (tag, rows, ops, upper, reason) in [
            ("rows", 2, 100, false, "row_cap"),
            ("ops", 64, 1, false, "operation_cap"),
            ("affine_cap", 4, 100, true, "row_cap"),
        ] {
            let root = tmp(tag);
            let (i, t, b, s) = propagation_control_fixture(&root, true);
            let out = root.join("propagation");
            let r = propagate_rows(&i, &t, b.clone(), &s, &out, rows, ops, Some(4), false);
            assert_eq!(
                r,
                propagate_rows(&i, &t, b, &s, &out, rows, ops, Some(4), true)
            );
            assert_eq!(r["state"]["stop_reason"], reason);
            assert_eq!(r["state"]["restricted_queue_exhausted"], false);
            assert_eq!(
                r["last_fall_upper_bound"],
                if upper { json!(4) } else { Value::Null }
            );
        }
    }
    #[test]
    fn dependent_seeds_and_unit_certificate() {
        let root = tmp("dependent");
        let (i, t, b, mut s) = propagation_control_fixture(&root, true);
        s.push(s[0].clone());
        let r = propagate_rows(
            &i,
            &t,
            b,
            &s,
            &root.join("propagation"),
            64,
            1024,
            Some(4),
            false,
        );
        assert_eq!(r["state"]["seeds_processed"], 4);
        assert_eq!(r["state"]["independent_seed_rows"], 3);
        let sp = Space::new(3, 3);
        let text = control_text(&["1"], 3);
        let row = sp.row(&parse("1", 3));
        let r = propagate_rows(
            &json!({"n_vars":3}),
            &text,
            vec![row],
            &[],
            &tmp("unit"),
            64,
            100,
            None,
            false,
        );
        assert_eq!(r["contains_one"], true);
        assert_eq!(r["last_fall_upper_bound"], 4);
        assert!(r["last_fall_degree"].is_null());
    }
    #[test]
    fn altered_row_operation_pending_and_upper_rejected() {
        for tag in ["row", "operation", "pending", "upper"] {
            let root = tmp(tag);
            let (i, t, b, s) = propagation_control_fixture(&root, true);
            let out = root.join("propagation");
            propagate_rows(&i, &t, b.clone(), &s, &out, 4, 100, Some(4), false);
            match tag {
                "row" => {
                    let p = out.join("added.rows.u64le");
                    let mut bytes = fs::read(&p).unwrap();
                    bytes[44] ^= 1;
                    fs::write(p, bytes).unwrap();
                }
                "operation" => {
                    let p = out.join("operations.jsonl");
                    let mut ls: Vec<Value> = fs::read_to_string(&p)
                        .unwrap()
                        .lines()
                        .map(|x| serde_json::from_str(x).unwrap())
                        .collect();
                    ls[0]["xor_basis_rows"] = json!([]);
                    fs::write(p, ls.iter().map(|x| format!("{x}\n")).collect::<String>()).unwrap();
                }
                "pending" => save(&out.join("pending.json"), &json!([])),
                "upper" => {
                    let p = out.join("propagation.json");
                    let mut r = read(&p);
                    r["last_fall_upper_bound"] = json!(3);
                    save(&p, &r);
                }
                _ => unreachable!(),
            }
            assert!(
                std::panic::catch_unwind(|| propagate_rows(
                    &i,
                    &t,
                    b,
                    &s,
                    &out,
                    4,
                    100,
                    Some(4),
                    true
                ))
                .is_err(),
                "{tag}"
            );
        }
    }
    #[test]
    fn witness_header_tail_and_field_guards() {
        let root = tmp("seed_bytes");
        let (i, t, b, s) = propagation_control_fixture(&root, true);
        let sp = Space::new(3, 3);
        let p = root.join("witness/candidates.rows.u64le");
        let original = fs::read(&p).unwrap();
        for bytes in [
            original[..original.len() - 1].to_vec(),
            [original.clone(), vec![0]].concat(),
            {
                let mut x = original.clone();
                x[0] ^= 1;
                x
            },
        ] {
            fs::write(&p, bytes).unwrap();
            assert!(std::panic::catch_unwind(|| load_witness_rows(&p, &sp, 3)).is_err());
        }
        fs::write(p, original).unwrap();
        let missing = t
            .lines()
            .filter(|l| !l.starts_with("I[4]"))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(std::panic::catch_unwind(|| propagate_rows(
            &i,
            &missing,
            b,
            &s,
            &root.join("bad"),
            64,
            100,
            Some(4),
            false
        ))
        .is_err());
    }
}
