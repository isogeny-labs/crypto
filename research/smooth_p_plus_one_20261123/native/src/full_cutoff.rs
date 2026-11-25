//! Bounded algebraic extraction over every qualified fine-image multiset.
//! Fixed-panel interface; the direct reference is used only by the caller.
use super::*;

const CAPTURE_BYTES: usize = 262_144;

#[derive(Clone, Copy)]
enum Presentation {
    Pushforward,
    Factorwise,
}

fn retain(record: &mut Value, key: &str, raw: &str, captured: &mut usize) -> Result<(), String> {
    // Charge escaped JSON bytes so raw strings cannot silently exceed the spool
    // record size through escaping. A missing required capture stops this query.
    let encoded = serde_json::to_string(raw).map_err(|e| e.to_string())?.len();
    record[format!("{key}_bytes")] = json!(raw.len());
    if encoded > CAPTURE_BYTES.saturating_sub(*captured) {
        record[format!("{key}_capture")] = json!("omitted_query_capture_cap");
        return Err("full-cutoff capture allowance exhausted".into());
    }
    *captured += encoded;
    record[key] = json!(raw);
    Ok(())
}

pub(super) fn discover(
    base: &Base,
    target: Point,
    solver: &Path,
    scratch: &Scratch,
    deadline: Instant,
) -> Discovery {
    discover_using(
        base,
        target,
        solver,
        scratch,
        deadline,
        Presentation::Pushforward,
    )
}

pub(super) fn discover_factorwise(
    base: &Base,
    target: Point,
    solver: &Path,
    scratch: &Scratch,
    deadline: Instant,
) -> Discovery {
    discover_using(
        base,
        target,
        solver,
        scratch,
        deadline,
        Presentation::Factorwise,
    )
}

fn discover_using(
    base: &Base,
    target: Point,
    solver: &Path,
    scratch: &Scratch,
    deadline: Instant,
    presentation: Presentation,
) -> Discovery {
    let mut outcome = Discovery {
        result: Err("oracle budget exhausted".into()),
        planned: 0,
        completed: 0,
        systems: vec![],
    };
    if Instant::now() >= deadline {
        return outcome;
    }
    let charts = match cutoff::qualified_catalog(base) {
        Ok(charts) => charts,
        Err(error) => {
            outcome.result = Err(error);
            return outcome;
        }
    };
    outcome.planned = charts.len();
    let (mut union, mut captured) = (BTreeSet::new(), 0);
    for (index, chart) in charts.iter().enumerate() {
        if Instant::now() >= deadline {
            return outcome;
        }
        let system = match presentation {
            Presentation::Pushforward => {
                System::at_cutoff(base.curve, target, base.depth - 3, chart.polynomial)
            }
            Presentation::Factorwise => {
                match System::factorwise(base.curve, target, base.depth - 3, chart.images) {
                    Ok(system) => system,
                    Err(error) => {
                        outcome.result = Err(error);
                        return outcome;
                    }
                }
            }
        };
        let (variables, equations, terms, degree) = system.metrics();
        let mut record = json!({
            "chart":index,"fine_images":chart.images,"coefficients":chart.polynomial,
            "variables":variables+1,"equations":equations+1,"terms_before_form":terms,
            "degree":degree,"solver_args":["-t","1","-c","0"],
            "form_limit":16,"query_capture_limit_bytes":CAPTURE_BYTES,"attempts":[],
        });
        if matches!(presentation, Presentation::Factorwise) {
            record["presentation"] = json!("factorwise");
        }
        let result = (|| -> Result<BTreeSet<Relation>, String> {
            if terms > 20_000 {
                return Err("full-cutoff construction term allowance exhausted".into());
            }
            let rendered = system.render(base.curve.p);
            let (names, rest) = rendered.split_once('\n').ok_or("invalid system render")?;
            let prefix = format!("{names},A\n{}", rest.trim_end());
            retain(&mut record, "input_prefix", &prefix, &mut captured)?;
            let mut rows = Err("separating-form allowance exhausted".to_string());
            for j in 0..16i64 {
                if Instant::now() >= deadline {
                    return Err("oracle budget exhausted".into());
                }
                let p = base.curve.p;
                let linear = format!(
                    "1*A+{}*t0+{}*a1+{}*a0",
                    p - 1,
                    (-j).rem_euclid(p),
                    (-j * j).rem_euclid(p)
                );
                let input = format!("{prefix},\n{linear}\n");
                let mut attempt = json!({"form":j,"linear_equation":linear});
                let candidate = run_solver(solver, &input, deadline, scratch, &["-c", "0"])
                    .and_then(|raw| {
                        retain(&mut attempt, "rur", &raw, &mut captured)?;
                        extract(&raw, base, target, Mode::Quadratic)
                    });
                let another = candidate.as_ref().err().is_some_and(|error| {
                    error.starts_with("nonseparating RUR:")
                        || error == "nonseparating staircase (msolve -c 0)"
                });
                attempt["status"] = json!(if candidate.is_ok() {
                    "complete"
                } else {
                    "incomplete"
                });
                attempt["reason"] = json!(candidate.as_ref().err());
                record["attempts"].as_array_mut().unwrap().push(attempt);
                rows = candidate;
                if !another {
                    break;
                }
            }
            let rows = rows?;
            if Instant::now() >= deadline {
                return Err("oracle budget exhausted".into());
            }
            if rows
                .iter()
                .any(|row| !chart.contains(base, row) || union.contains(row))
            {
                return Err("overlapping or incorrect full-cutoff chart".into());
            }
            Ok(rows)
        })();
        record["status"] = json!(if result.is_ok() {
            "complete"
        } else {
            "incomplete"
        });
        record["reason"] = json!(result.as_ref().err());
        record["rows"] = json!(result.as_ref().ok());
        outcome.systems.push(record);
        match result {
            Ok(rows) => {
                union.extend(rows);
                outcome.completed += 1;
            }
            Err(error) => {
                outcome.result = Err(error);
                return outcome;
            }
        }
    }
    if Instant::now() < deadline {
        outcome.result = Ok(union);
    }
    outcome
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn stub(scratch: &Scratch, body: &str) -> PathBuf {
        let path = scratch.0.join("solver.sh");
        fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
        path
    }

    #[test]
    fn factorwise_dispatch_serializes_the_qualified_variables_and_visits_all_charts() {
        let base = Base::new(FIXTURES[0], 4);
        let scratch = Scratch::new().unwrap();
        let solver = stub(&scratch,
            "IFS= read -r first < \"$2\"\n[ \"$first\" = 'q0,q1,q2,q3,a0,a1,t0,A' ] || exit 91\nprintf '[-1]:' > \"$4\"");
        let result = super::super::discover(
            &base,
            base.curve.q,
            Method::FactorwiseCutoff,
            &solver,
            &scratch,
            Duration::from_secs(3),
        );
        assert!(result.result.unwrap().is_empty());
        assert_eq!(
            result.completed,
            cutoff::qualified_catalog(&base).unwrap().len()
        );
        for record in result.systems {
            assert_eq!(record["presentation"], "factorwise");
            assert_eq!(record["variables"], 8);
            assert_eq!(record["equations"], 9);
            assert_eq!(record["attempts"].as_array().unwrap().len(), 1);
        }
        let expired = discover_factorwise(
            &base,
            base.curve.q,
            Path::new("/missing"),
            &scratch,
            Instant::now(),
        );
        assert!(expired.result.is_err());
        assert_eq!(expired.completed, 0);
        assert!(expired.systems.is_empty());
        assert!(mode("factorwise-cutoff") == Some(Method::FactorwiseCutoff));
        assert_eq!(Method::FactorwiseCutoff.encoding(), Mode::Quadratic);
    }

    #[test]
    fn complete_empty_union_visits_every_chart_and_expired_budget_calls_none() {
        let base = Base::new(FIXTURES[0], 4);
        let scratch = Scratch::new().unwrap();
        let solver = stub(&scratch, "printf '[-1]:' > \"$4\"");
        let empty = discover(
            &base,
            base.curve.q,
            &solver,
            &scratch,
            Instant::now() + Duration::from_secs(3),
        );
        assert_eq!(
            empty.planned,
            cutoff::qualified_catalog(&base).unwrap().len()
        );
        assert_eq!(empty.completed, empty.planned);
        assert!(empty.result.unwrap().is_empty());
        assert!(empty
            .systems
            .iter()
            .all(|s| s["attempts"].as_array().unwrap().len() == 1));
        let expired = discover(
            &base,
            base.curve.q,
            Path::new("/missing"),
            &scratch,
            Instant::now(),
        );
        assert_eq!(expired.completed, 0);
        assert!(expired.systems.is_empty());
        assert_eq!(expired.result.unwrap_err(), "oracle budget exhausted");
    }

    #[test]
    fn completed_prefix_is_not_accepted_after_later_chart_failure() {
        let base = Base::new(FIXTURES[0], 4);
        let scratch = Scratch::new().unwrap();
        let solver = stub(
            &scratch,
            "if [ -e \"$2.seen\" ]; then exit 9; fi\n: > \"$2.seen\"\nprintf '[-1]:' > \"$4\"",
        );
        let result = discover(
            &base,
            base.curve.q,
            &solver,
            &scratch,
            Instant::now() + Duration::from_secs(3),
        );
        assert_eq!(result.completed, 1);
        assert_eq!(result.systems.len(), 2);
        assert_eq!(result.systems[0]["status"], "complete");
        assert_eq!(result.systems[1]["status"], "incomplete");
        assert!(result.result.unwrap_err().starts_with("solver exit"));
    }

    #[test]
    fn only_known_nonseparation_advances_forms_and_capture_limit_stops() {
        let base = Base::new(FIXTURES[0], 4);
        let scratch = Scratch::new().unwrap();
        let solver = stub(&scratch, "if [ -e \"$2.seen\" ]; then exit 9; fi\n: > \"$2.seen\"\n: > \"$4\"\nprintf '%s\\n' '=====> Computation failed <=====' 'Try to add a random linear form with a new variable' >&2");
        let result = discover(
            &base,
            base.curve.q,
            &solver,
            &scratch,
            Instant::now() + Duration::from_secs(3),
        );
        assert_eq!(result.completed, 0);
        assert_eq!(result.systems[0]["attempts"].as_array().unwrap().len(), 2);
        assert_eq!(
            result.systems[0]["attempts"][0]["reason"],
            "nonseparating staircase (msolve -c 0)"
        );
        assert!(result.result.unwrap_err().starts_with("solver exit"));
        let mut record = json!({});
        let mut captured = CAPTURE_BYTES;
        assert!(retain(&mut record, "rur", "[-1]:", &mut captured).is_err());
        assert_eq!(record["rur_capture"], "omitted_query_capture_cap");
        assert!(record.get("rur").is_none());
    }
}
