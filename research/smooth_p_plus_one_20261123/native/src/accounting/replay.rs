//! Post-run checking only. The algebraic extractor never calls this module.
use super::*;

const INPUT_CAP: u64 = 128 << 20;

pub(super) fn file(curve: Fixture, depth: usize, path: &Path) -> Result<Value, String> {
    if !(4..=5).contains(&depth) {
        return Err("recorded replay is qualified only for depths four and five".into());
    }
    let input = File::open(path).map_err(|e| e.to_string())?;
    if input.metadata().map_err(|e| e.to_string())?.len() > INPUT_CAP {
        return Err("recorded replay input exceeds 128 MiB".into());
    }
    terminated_output(path)?;
    let mut records = vec![];
    for line in BufReader::new(input.take(INPUT_CAP + 1)).split(b'\n') {
        let line = line.map_err(|e| e.to_string())?;
        if line.is_empty() || line.len() + 1 > RECORD_CAP {
            return Err("empty or oversized replay record".into());
        }
        records.push(serde_json::from_slice::<Value>(&line).map_err(|e| e.to_string())?);
    }
    audit(curve, depth, &records)
}

fn audit(curve: Fixture, depth: usize, records: &[Value]) -> Result<Value, String> {
    let base = Base::new(curve, depth);
    let pairs = base.pair_index();
    let mut pending = vec![];
    let mut arms = vec![];
    let mut seen = BTreeSet::new();
    for record in records {
        match record["kind"].as_str() {
            Some("account_attempt" | "account_summary") => pending.push(record.clone()),
            Some("account_process_receipt") => {
                if pending.is_empty() || arms.len() == 8 {
                    return Err("missing child or too many arms".into());
                }
                let summary = pending.last().unwrap();
                let method = match summary["mode"].as_str() {
                    Some("Direct") => None,
                    Some("Refined") => Some(Method::Refined),
                    Some("ProductionCofactorMsolve") => Some(Method::CofactorMsolve),
                    _ => return Err("unsupported replay method".into()),
                };
                let spec = Run {
                    curve,
                    depth,
                    solver: Path::new("/replay-never-invokes-a-solver"),
                    method,
                    seed: summary["seed"].as_u64().ok_or("missing seed")?,
                    draws: summary["max_draws"]
                        .as_u64()
                        .filter(|n| (1..=256).contains(n))
                        .ok_or("invalid draw bound")? as usize,
                    base_lookup: !summary["base_lookup"].is_null(),
                    full_output: true,
                };
                if !seen.insert(spec.label())
                    || record["receipt_valid"] != true
                    || record["exit_code"] != 0
                    || !record["error"].is_null()
                    || record["mode"] != spec.label()
                    || record["prime"] != curve.p
                    || record["depth"] != depth
                    || record["seed"] != spec.seed
                    || record["format_version"] != 3
                    || record["unrun_arms"].as_array().map(Vec::len) != Some(0)
                {
                    return Err("invalid or repeated process receipt".into());
                }
                validate_summary(
                    spec,
                    &pending,
                    record["spawn_through_exit_ns"]
                        .as_u64()
                        .ok_or("missing parent interval")?,
                )?;
                arms.push(replay_arm(spec, &base, &pairs, &pending)?);
                pending.clear();
            }
            _ => return Err("unexpected replay record kind".into()),
        }
    }
    if !pending.is_empty() || arms.is_empty() {
        return Err("missing final process receipt or empty input".into());
    }
    Ok(
        json!({"kind":"account_replay","format_version":1,"prime":curve.p,"depth":depth,
        "arms":arms,"status":"passed","scope":"post-run exhaustive oracle-set comparison and production-consumer replay; local and unsigned",
        "limitations":"shared fixture/reference/consumer source; incomplete oracle outputs are not certified; audit work is separate from recorded recovery intervals"}),
    )
}

fn replay_arm(
    spec: Run<'_>,
    base: &Base,
    pairs: &BTreeMap<Option<Point>, Vec<[i32; 2]>>,
    records: &[Value],
) -> Result<Value, String> {
    let curve = spec.curve;
    let (public_curve, points) = production::public_input(base);
    let mut consumer = RelationAdapter::new(
        public_curve,
        points,
        Some((curve.q.x as u64, curve.q.y as u64)),
    )
    .map_err(|e| e.to_string())?;
    let (mut rng, mut seen) = (spec.seed, BTreeSet::new());
    let (mut complete, mut incomplete, mut returned, mut singletons, mut skipped) = (0, 0, 0, 0, 0);
    for record in &records[..records.len() - 1] {
        let a = next_mask(&mut rng, curve.n);
        let b = next_mask(&mut rng, curve.n);
        let target = curve.add(curve.multiply(a, curve.g), curve.multiply(b, curve.q));
        let fresh = seen.insert(target);
        if record["a"] != a
            || record["b"] != b
            || record["target"] != json!(target.map(|p| [p.x, p.y]))
            || record["rank_before"] != consumer.rank()
        {
            return Err("mask, residual or initial rank mismatch".into());
        }
        let result = &record["result"];
        if target.is_none() || !fresh {
            let reason = if target.is_none() {
                "identity"
            } else {
                "duplicate_target"
            };
            if result["status"] != "skipped" || result["reason"] != reason {
                return Err("wrong skipped-draw classification".into());
            }
            skipped += 1;
        } else {
            let target = target.unwrap();
            let hit = spec.base_lookup.then(|| base.signed.get(&target)).flatten();
            if let Some(&label) = hit {
                if result["status"] != "base_relation" || result["selected"] != json!([label]) {
                    return Err("wrong singleton lookup output".into());
                }
                replay_row(&mut consumer, &[label], a, b, &result["consumer"])?;
                singletons += 1;
            } else if result["status"] == "undetermined" {
                // The reference must not fill in a failed oracle's missing output.
                if !result["returned"].is_null() || !result["consumed"].is_null() {
                    return Err("incomplete query carries an accepted output".into());
                }
                incomplete += 1;
            } else if result["status"] == "complete_query" {
                let expected: BTreeSet<Relation> = base
                    .reference(pairs, target)
                    .into_iter()
                    .filter(|row| spec.method != Some(Method::Refined) || base.separated(row))
                    .collect();
                if result["returned"] != json!(expected) {
                    return Err(format!(
                        "whole oracle output mismatch in {} draw {}",
                        spec.label(),
                        record["draw"]
                    ));
                }
                returned += expected.len();
                complete += 1;
                for item in result["consumed"]
                    .as_array()
                    .ok_or("missing consumed rows")?
                {
                    let row: Relation = serde_json::from_value(item["selected"].clone())
                        .map_err(|e| e.to_string())?;
                    replay_row(&mut consumer, &row, a, b, &item["consumer"])?;
                }
            } else {
                return Err("unexpected residual output classification".into());
            }
        }
        if record["rank_after"] != consumer.rank() {
            return Err("replayed draw rank differs".into());
        }
    }
    let summary = records.last().unwrap();
    if summary["scalar"] != json!(consumer.scalar())
        || summary["consumer_rank"] != consumer.rank()
        || summary["base_size"] != base.unsigned.len()
        || summary["consumer_work"] != json!(consumer.work())
        || summary["consumer_arithmetic"] != json!(consumer.arithmetic_counts())
        || summary["consumer_setup_arithmetic"] != json!(consumer.setup_arithmetic_counts())
        || consumer
            .scalar()
            .is_some_and(|k| curve.multiply(k as i64, curve.g) != Some(curve.q))
    {
        return Err("replayed consumer totals or scalar differ".into());
    }
    Ok(
        json!({"mode":spec.label(),"draws":summary["draws"],"complete_queries_checked":complete,
        "incomplete_queries_preserved":incomplete,"returned_rows_checked":returned,
        "consumed_rows_replayed":consumer.work().submitted,"singleton_rows":singletons,
        "skipped_draws":skipped,"consumer_rank":consumer.rank(),"scalar":consumer.scalar(),
        "recovery_status":summary["status"]}),
    )
}

fn replay_row(
    consumer: &mut RelationAdapter,
    row: &[i32],
    a: i64,
    b: i64,
    recorded: &Value,
) -> Result<(), String> {
    let receipt = consume_labels(consumer, row, a, b).map_err(|e| e.to_string())?;
    if json!(receipt) != *recorded {
        return Err("replayed row receipt differs".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(raw: &str) -> Vec<Value> {
        raw.lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect()
    }

    fn retained() -> Vec<Value> {
        parse(include_str!("../../../evidence/full_output_v1/full.jsonl"))
    }

    #[test]
    fn retained_full_sets_and_consumers_replay_with_both_lookup_policies() {
        for raw in [
            include_str!("../../../evidence/full_output_v1/full.jsonl"),
            include_str!("../../../evidence/full_output_v1/full_lookup.jsonl"),
        ] {
            let checked = audit(FIXTURES[0], 4, &parse(raw)).unwrap();
            assert_eq!(checked["arms"].as_array().unwrap().len(), 3);
            for arm in checked["arms"].as_array().unwrap() {
                assert_eq!(arm["scalar"], 69);
            }
        }
    }

    #[test]
    fn post_run_replay_rejects_consistent_row_omission_and_altered_public_evidence() {
        let original = retained();
        let summary = original
            .iter()
            .position(|r| r["kind"] == "account_summary")
            .unwrap();
        let spec = Run {
            curve: FIXTURES[0],
            depth: 4,
            solver: Path::new("/unused"),
            method: None,
            seed: 781815,
            draws: 30,
            base_lookup: false,
            full_output: true,
        };
        for mutation in 0..6 {
            let mut changed = original.clone();
            match mutation {
                0 => {
                    let result = &mut changed[summary - 1]["result"];
                    assert!(
                        result["returned"].as_array().unwrap().len()
                            > result["consumed"].as_array().unwrap().len()
                    );
                    result["returned"].as_array_mut().unwrap().pop();
                    result["relations"] = json!(result["relations"].as_u64().unwrap() - 1);
                    for key in ["oracle_rows_returned", "oracle_rows_unconsumed"] {
                        changed[summary]["full_output"][key] =
                            json!(changed[summary]["full_output"][key].as_u64().unwrap() - 1);
                    }
                    // The counts and consumption prefix still reconcile. Only
                    // the independent reference set exposes this lost suffix row.
                    validate_summary(
                        spec,
                        &changed[..=summary],
                        changed[summary + 1]["spawn_through_exit_ns"]
                            .as_u64()
                            .unwrap(),
                    )
                    .unwrap();
                }
                1 => changed[0]["a"] = json!(0),
                2 => changed[0]["target"] = json!([1, 3]),
                3 => changed[0]["result"]["consumed"][0]["consumer"]["canonical_terms"] = json!([]),
                4 => changed[summary]["consumer_work"]["group_equalities_checked"] = json!(0),
                5 => changed[summary + 1]["receipt_valid"] = json!(false),
                _ => unreachable!(),
            }
            assert!(
                audit(FIXTURES[0], 4, &changed).is_err(),
                "mutation {mutation}"
            );
        }
    }

    #[test]
    fn failed_oracle_and_skipped_draws_remain_incomplete_during_replay() {
        let spec = Run {
            curve: FIXTURES[0],
            depth: 4,
            solver: Path::new("/no-solver-for-replay-test"),
            method: Some(Method::Refined),
            seed: 781815,
            draws: 256,
            base_lookup: false,
            full_output: true,
        };
        let start = Instant::now();
        let mut bytes = vec![];
        run(spec, start, &mut bytes).unwrap();
        let mut rows = parse(std::str::from_utf8(&bytes).unwrap());
        rows.push(
            json!({"kind":"account_process_receipt","format_version":3,"mode":spec.label(),
            "prime":127,"depth":4,"seed":781815,"exit_code":0,"receipt_valid":true,"error":null,
            "spawn_through_exit_ns":nanos(start.elapsed()),"unrun_arms":[]}),
        );
        let checked = audit(FIXTURES[0], 4, &rows).unwrap();
        let arm = &checked["arms"][0];
        assert_eq!(arm["complete_queries_checked"], 0);
        assert!(arm["incomplete_queries_preserved"].as_u64().unwrap() > 0);
        assert!(arm["skipped_draws"].as_u64().unwrap() > 0);
        assert_eq!(arm["recovery_status"], "incomplete");
        assert_eq!(arm["scalar"], Value::Null);
    }
}
