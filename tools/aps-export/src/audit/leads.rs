//! Independent structural and coverage checks for the opt-in APS v3 records.
//! This does not attest execution or infer missing costs from diagnostics.
use super::*;

pub(super) fn version(r: &Value) -> Result<u8> {
    let v = match r["detail"]["schema"].as_str() {
        Some("ecbench.aps/v1") => 1,
        Some("ecbench.aps/v2") => 2,
        Some("ecbench.aps/v3") => 3,
        _ => 0,
    };
    if v >= 1 {
        require(
            r["method"]["id"] == format!("ic.aps_v{v}"),
            "APS schema and method version disagree",
        )?;
    } else if v == 0
        && r["method"]["id"]
            .as_str()
            .is_some_and(|s| s.starts_with("ic.aps_"))
    {
        return Err("unsupported APS schema cannot bypass audit".into());
    }
    Ok(v)
}

pub(super) fn configuration(p: &Value, v: u8) -> Result<Vec<usize>> {
    if v != 3 {
        super::configuration(p)?;
        return Ok(vec![]);
    }
    require(
        p["subset_size"] == "0" && p["roots"] == "exact",
        "v3 requires explicit profile and exact traversal",
    )?;
    let chart = p["encoding"] == "line_chart3";
    let enc = if chart {
        "line3"
    } else {
        text(&p["encoding"])?
    };
    let arity = if matches!(enc, "line3" | "unordered3" | "partitioned3") {
        3
    } else {
        4
    };
    let profile = text(&p["subset_profile"])?
        .split(',')
        .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>>>()?;
    require(
        profile.len() == arity && profile.iter().all(|s| (1..=4).contains(s)),
        "invalid v3 profile",
    )?;
    let mut ordinary = p.clone();
    ordinary["encoding"] = json!(enc);
    ordinary["subset_size"] = json!(profile[0].to_string());
    super::configuration(&ordinary)?;
    require(
        matches!(text(&p["trace"])?, "off" | "certified"),
        "invalid trace policy",
    )?;
    require(
        (1..=16).contains(&integer(&p["trace_entries"])?)
            && (1..=1_000_000).contains(&integer(&p["trace_words"])?),
        "invalid trace storage bounds",
    )?;
    let coordinate = matches!(enc, "partitioned3" | "expanded4" | "split4");
    if p["guess"] != "none" {
        require(
            coordinate && integer(&p["guess"])? < arity as u64,
            "invalid guessed variable",
        )?;
    }
    require(
        matches!(text(&p["growth"])?, "none" | "retained_full" | "frontier"),
        "invalid growth policy",
    )?;
    let ceiling = integer(&p["growth_ceiling"])? as usize;
    require((1..=4).contains(&ceiling), "invalid growth ceiling")?;
    if p["growth"] != "none" {
        require(
            coordinate && p["orbit"] == "none" && profile.iter().all(|s| *s <= ceiling),
            "unsupported growth configuration",
        )?;
    }
    if p["orbit"] == "cubic" {
        require(
            profile.iter().all(|s| *s <= 3),
            "orbit profile exceeds bound",
        )?;
    }
    Ok(profile)
}

pub(super) fn metrics(r: &Value) -> Result<()> {
    for (counter, field) in [
        ("aps_core_scalar_mults_uncharged", "scalar_mults"),
        ("aps_core_combines_uncharged", "combines"),
        ("aps_core_point_adds_uncharged", "point_adds"),
        ("aps_core_square_roots_uncharged", "lifts"),
        (
            "aps_core_expression_multiplications_uncharged",
            "expression_multiplications",
        ),
    ] {
        require(
            integer(&r["counters"][counter])? == integer(&r["detail"]["core_work"][field])?,
            "core diagnostic/source snapshot mismatch",
        )?;
    }
    let fields = [
        "learned",
        "replay_attempts",
        "accepted",
        "rejected",
        "fallbacks",
        "certification_reductions",
        "certification_pairs",
        "certification_multiplications",
        "certification_term_steps",
        "storage_refusals",
    ];
    let mut previous = BTreeMap::from(fields.map(|s| (s, 0u64)));
    for a in array(&r["detail"]["attempts"])? {
        let lead = &a["lead"];
        for field in fields {
            let before = integer(&lead["trace_before"][field])?;
            let after = integer(&lead["trace_after"][field])?;
            require(
                before == previous[field] && after >= before,
                "trace counter history is discontinuous",
            )?;
            previous.insert(field, after);
        }
        if !a["algebra"].is_null() {
            let calls = previous["learned"] + previous["replay_attempts"]
                - integer(&lead["trace_before"]["learned"])?
                - integer(&lead["trace_before"]["replay_attempts"])?;
            if r["method"]["params"]["trace"] == "certified" {
                require(
                    calls == integer(&a["algebra"]["solver_calls"])?,
                    "learning/replay/fallback calls omitted",
                )?;
            }
        }
    }
    for field in fields {
        require(
            integer(&r["detail"]["trace_pool"][field])? == previous[field],
            "final trace aggregate mismatch",
        )?;
    }
    require(
        integer(&r["counters"]["aps_trace_certification_term_steps_uncharged"])?
            == previous["certification_term_steps"],
        "trace certification work omitted",
    )?;
    require(
        previous["accepted"] + previous["rejected"] == previous["replay_attempts"]
            && previous["fallbacks"] <= previous["rejected"],
        "invalid trace outcome partition",
    )?;
    require(
        integer(&r["detail"]["trace_pool"]["retained_words"])?
            <= integer(&r["method"]["params"]["trace_words"])?,
        "trace storage exceeds configured bound",
    )
}

fn parts(v: &Value) -> Result<Vec<Vec<u64>>> {
    array(v)?
        .iter()
        .map(|row| array(row)?.iter().map(integer).collect())
        .collect()
}

pub(super) fn attempt(
    r: &Value,
    i: usize,
    c: Curve,
    base: &Base,
    initial: &[usize],
) -> Result<Option<Base>> {
    let rows = array(&r["detail"]["attempts"])?;
    let a = &rows[i];
    let p = &r["method"]["params"];
    let lead = &a["lead"];
    require(
        lead["retained_base"].is_boolean() && lead["sampling_complete"].is_boolean(),
        "missing lead history flags",
    )?;
    let growth = p["growth"] != "none";
    let ceiling = integer(&p["growth_ceiling"])? as usize;
    let prior = rows[..i]
        .iter()
        .rev()
        .find(|a| a["lead"]["sampling_complete"] == true);
    let mut expected = initial.to_vec();
    let mut skip = None;
    let mut old_parts = None;
    if lead["retained_base"] == true {
        require(growth, "retained base without growth")?;
        let prior = prior.ok_or("retained base without a complete sampling stage")?;
        let old = Base::parse(&prior["base"], c)?;
        require(
            old.parts.iter().any(|part| part.len() < ceiling),
            "base retained beyond its growth ceiling",
        )?;
        expected = old
            .parts
            .iter()
            .map(|part| (part.len() + 1).min(ceiling))
            .collect();
        for (before, after) in old.parts.iter().zip(&base.parts) {
            require(
                after.starts_with(before),
                "growth changed old partition order or membership",
            )?;
        }
        for x in old.entries.keys() {
            require(
                a["base"]["points"][x.to_string()] == prior["base"]["points"][x.to_string()],
                "growth changed an old point or label",
            )?;
        }
        require(
            integer(&a["base"]["draws"])?
                == integer(&prior["base"]["draws"])?
                    .checked_add(integer(&lead["new_draws"])?)
                    .ok_or("draw counter overflow")?,
            "growth draw total mismatch",
        )?;
        let complete = prior["error"].is_null()
            && prior["recovery"].is_null()
            && prior["algebra"]["complete"] == true
            && prior["algebra"]["timed_out"] == false
            && prior["algebra"]["stopped_early"] == false;
        if complete && p["growth"] == "frontier" {
            require(
                parts(&lead["skipped_parts"])? == old.parts,
                "wrong skipped region",
            )?;
            old_parts = Some(old.parts.clone());
            skip = Some(old);
        } else {
            require(
                lead["skipped_parts"].is_null(),
                "partial/failed search used as a coverage certificate",
            )?;
        }
    } else {
        require(
            lead["skipped_parts"].is_null(),
            "fresh base skips an old region",
        )?;
        require(
            integer(&a["base"]["draws"])? == integer(&lead["new_draws"])?,
            "fresh draw total mismatch",
        )?;
    }
    let actual: Vec<_> = array(&lead["profile"])?
        .iter()
        .map(|x| integer(x).map(|n| n as usize))
        .collect::<Result<_>>()?;
    require(
        actual == expected && base.parts.len() == expected.len(),
        "lead profile differs from frozen policy",
    )?;
    let multiplier = if p["orbit"] == "cubic" && a["error_stage"] != "orbit" {
        3
    } else {
        1
    };
    let sampled = lead["sampling_complete"] == true;
    require(
        base.parts.iter().zip(&expected).all(|(part, n)| {
            if sampled {
                part.len() == n * multiplier
            } else {
                part.len() <= *n
            }
        }),
        "sampled partition size differs from profile",
    )?;
    require(
        integer(&lead["new_draws"])? <= integer(&p["sampling_draws"])?,
        "draw cap exceeded",
    )?;
    if !sampled {
        require(
            a["error_stage"] == "sampling"
                && !a["error"].is_null()
                && a["algebra"].is_null()
                && a["recovery"].is_null()
                && array(&a["visited_coordinate_sets"])?.is_empty()
                && array(&lead["cells"])?.is_empty()
                && a["independent_witness_verified"] == false,
            "partial draw promoted into solver output",
        )?;
        return Ok(None);
    }
    if a["system"].is_null() {
        require(
            a["algebra"].is_null() && a["recovery"].is_null() && !a["error"].is_null(),
            "missing plan without construction refusal",
        )?;
        return Ok(skip);
    }
    let enc = text(&p["encoding"])?;
    let chart = enc == "line_chart3";
    if chart {
        require(
            base.entries.values().all(|e| e.point.y != 0),
            "zero ordinate in line chart",
        )?;
    }
    let guess = if p["guess"] == "none" {
        None
    } else {
        Some(integer(&p["guess"])? as usize)
    };
    let mut partition_cells = vec![base.parts.clone()];
    if let Some(old) = old_parts {
        partition_cells.clear();
        for j in 0..base.parts.len() {
            let delta: Vec<_> = base.parts[j]
                .iter()
                .filter(|x| !old[j].contains(x))
                .copied()
                .collect();
            if !delta.is_empty() {
                partition_cells.push(
                    (0..base.parts.len())
                        .map(|k| {
                            if k < j {
                                old[k].clone()
                            } else if k == j {
                                delta.clone()
                            } else {
                                base.parts[k].clone()
                            }
                        })
                        .collect(),
                );
            }
        }
    }
    let dimensions = match enc {
        "partitioned3" | "unordered3" => 3,
        "line3" => {
            if p["prune"] == "true" {
                3
            } else {
                2
            }
        }
        "expanded4" => 4,
        _ => 5,
    };
    let expected_cells: Vec<(usize, Vec<u64>)> = if chart {
        if p["prune"] == "true" {
            vec![(2, vec![])]
        } else {
            vec![(2, vec![]), (1, vec![])]
        }
    } else if let Some(j) = guess {
        partition_cells
            .iter()
            .flat_map(|parts| parts[j].iter().map(|x| (dimensions - 1, vec![*x])))
            .collect()
    } else {
        partition_cells
            .iter()
            .map(|_| (dimensions, vec![]))
            .collect()
    };
    let cells = array(&lead["cells"])?;
    require(
        cells.len() == expected_cells.len(),
        "plan cell population differs from policy",
    )?;
    require(
        lead["plan_sha256"] == hash(&canonical(&lead["cells"])?),
        "plan digest mismatch",
    )?;
    for (cell, (vars, fixed)) in cells.iter().zip(expected_cells) {
        let s = &cell["system"];
        require(
            cell["system_sha256"] == hash(&canonical(s)?)
                && integer(&s["p"])? == c.p
                && array(&s["variables"])?.len() == vars
                && s["ordering"] == "grevlex"
                && cell["fixed_coordinates"] == json!(fixed),
            "cell identity, dimension or guessed coordinate mismatch",
        )?;
        for poly in array(&s["equations"])? {
            for term in array(poly)? {
                let pair = array(term)?;
                require(
                    pair.len() == 2
                        && array(&pair[0])?.len() == vars
                        && integer(&pair[1])? > 0
                        && integer(&pair[1])? < c.p,
                    "malformed literal equation",
                )?;
            }
        }
    }
    require(
        cells.first().is_some_and(|s| {
            s["system"] == a["system"] && s["system_sha256"] == a["system_sha256"]
        }),
        "primary system differs from first cell",
    )?;
    if a["algebra"]["complete"] == true {
        require(
            a["algebra"]["stopped_early"] == false,
            "stopped traversal claims completeness",
        )?;
    }
    Ok(skip)
}
