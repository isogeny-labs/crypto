use super::{
    model::Sample, quartic_cells, quartic_oracle, route, route_selected, sampler, select, Report,
};
use std::collections::BTreeSet;

pub(super) fn collect(mut fixture: impl FnMut(u64) -> Result<sampler::Fixture, String>) -> Report {
    let mut samples = Vec::new();
    let mut correct = true;
    let mut attempted = 0;
    let mut completed = 0;
    for seed in sampler::SEEDS {
        let f = fixture(seed);
        let fixture_error = f.as_ref().err().cloned();
        let selection_result = f.as_ref().map_err(Clone::clone).and_then(|f| {
            quartic_cells::plan(f)?
                .iter()
                .enumerate()
                .map(|(i, cell)| select(&cell.parts, i))
                .collect::<Result<Vec<_>, _>>()
        });
        let selection_error = selection_result.as_ref().err().cloned();
        let selections = selection_result.unwrap_or_default();
        let mut routes = Vec::new();
        let mut errors = Vec::new();
        for pairing in 0..3 {
            for shared in [false, true] {
                attempted += 1;
                let result = f
                    .as_ref()
                    .map_err(Clone::clone)
                    .and_then(|f| route(f, seed, pairing, shared));
                match result {
                    Ok(r) => {
                        correct &= r.complete_and_equal;
                        completed += usize::from(r.complete_and_equal);
                        routes.push(r);
                    }
                    Err(error) => {
                        correct = false;
                        errors.push(format!("pairing{pairing}/shared{shared}: {error}"));
                    }
                }
            }
        }
        let mut selected = BTreeSet::new();
        let mut selected_complete = fixture_error.is_none() && selection_error.is_none();
        for s in &selections {
            let cell = routes
                .iter()
                .find(|r| r.pairing == Some(s.selected_pairing) && !r.shared)
                .and_then(|r| r.cells.get(s.cell));
            if let Some(c) = cell {
                selected_complete &=
                    c.complete && !c.timed_out && c.error.is_none() && c.observed == c.expected;
                selected.extend(c.observed.iter().cloned());
            } else {
                selected_complete = false;
            }
        }
        let all_roots = f
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|f| quartic_oracle::roots(&f.base, &quartic_cells::universe(f)));
        match all_roots {
            Ok(reference) => selected_complete &= selected == reference,
            Err(error) => {
                selected_complete = false;
                errors.push(format!("selected reference: {error}"));
            }
        }
        attempted += 1;
        let actual = f
            .as_ref()
            .map_err(Clone::clone)
            .and_then(|f| route_selected(f, seed, &selections));
        let (selected_route, selected_route_error) = match actual {
            Ok(r) => {
                selected_complete &= r.complete_and_equal
                    && r.observed_roots.iter().cloned().collect::<BTreeSet<_>>() == selected;
                completed += usize::from(r.complete_and_equal);
                (Some(r), None)
            }
            Err(error) => {
                selected_complete = false;
                (None, Some(error))
            }
        };
        correct &= selected_complete;
        samples.push(Sample {
            seed,
            fixture_error,
            selection_error,
            selections,
            routes,
            route_errors: errors,
            selected_roots_from_cold_controls: selected.into_iter().collect(),
            selected_cold_replay_complete: selected_complete,
            selected_route,
            selected_route_error,
        });
    }
    Report {
        schema: "aps.pairing-controls/v1",
        heuristic: "min(|Fi|*|Fj|+|Fk|*|Fl|), tie by fixed pairing index",
        selected_route_uses_cold_control_outputs: false,
        actual_selected_route_executed: true,
        performance_trial: false,
        cost_complete: false,
        samples,
        planned_routes: 14,
        planned_control_routes: 12,
        planned_selected_routes: 2,
        attempted_routes: attempted,
        completed_routes: completed,
        failed_or_incomplete_routes: attempted - completed,
        excluded_routes_by_rule: 0,
        complete_and_equal: correct,
    }
}
