use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const RESIDUAL_QUERIES: u128 = 27_370;
const LIMIT_TENTHS: u128 = 16_000_000;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Operations {
    add: u128,
    sub: u128,
    mul: u128,
    square: u128,
    inverse: u128,
}

impl Operations {
    fn plus(self, right: Self) -> Self {
        Self {
            add: self.add + right.add,
            sub: self.sub + right.sub,
            mul: self.mul + right.mul,
            square: self.square + right.square,
            inverse: self.inverse + right.inverse,
        }
    }

    fn unweighted(self) -> u128 {
        self.add + self.sub + self.mul + self.square + self.inverse
    }

    fn fe_tenths(self, panel: Panel) -> u128 {
        10 * (self.add + self.sub + self.mul)
            + panel.square_tenths * self.square
            + 10 * panel.inverse_price * self.inverse
    }
}

#[derive(Clone, Copy)]
struct Panel {
    name: &'static str,
    square_tenths: u128,
    inverse_price: u128,
}

const PANELS: [Panel; 3] = [
    Panel {
        name: "sq0.8-inv50",
        square_tenths: 8,
        inverse_price: 50,
    },
    Panel {
        name: "sq1.0-inv100",
        square_tenths: 10,
        inverse_price: 100,
    },
    Panel {
        name: "sq1.0-inv200",
        square_tenths: 10,
        inverse_price: 200,
    },
];

#[derive(Clone, Debug)]
struct QueryCounters {
    norm: Operations,
    recovery: Operations,
    direct: Operations,
}

#[derive(Clone, Debug)]
struct DepthCounters {
    depth: usize,
    full_roots: usize,
    signed_leaves: usize,
    setup: Operations,
    finite_queries: Vec<QueryCounters>,
}

#[derive(Clone, Debug)]
struct DepthSummary {
    depth: usize,
    full_roots: usize,
    signed_leaves: usize,
    max_poly_work: u128,
    setup_tenths: [u128; 3],
    norm_max_tenths: [u128; 3],
    direct_max_tenths: [u128; 3],
    min_speedup: [f64; 3],
    speedup_pass: [bool; 3],
}

fn fields(line: &str) -> BTreeMap<&str, &str> {
    line.split_whitespace()
        .skip(1)
        .map(|field| field.split_once('=').expect("malformed key=value field"))
        .collect()
}

fn value(map: &BTreeMap<&str, &str>, key: &str) -> u128 {
    map.get(key)
        .unwrap_or_else(|| panic!("missing counter {key}"))
        .parse()
        .unwrap_or_else(|_| panic!("invalid counter {key}"))
}

fn operations(map: &BTreeMap<&str, &str>, prefix: &str) -> Operations {
    Operations {
        add: value(map, &format!("{prefix}_add")),
        sub: value(map, &format!("{prefix}_sub")),
        mul: value(map, &format!("{prefix}_mul")),
        square: value(map, &format!("{prefix}_square")),
        inverse: value(map, &format!("{prefix}_inv")),
    }
}

fn read_candidate(path: &str) -> Vec<DepthCounters> {
    let file = File::open(path).expect("cannot open candidate transcript");
    let mut depths = BTreeMap::<usize, DepthCounters>::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("cannot read candidate transcript");
        if line.starts_with("DEPTH ") {
            let map = fields(&line);
            let depth = value(&map, "depth") as usize;
            let root = operations(&map, "root");
            let algebra = operations(&map, "algebra_setup");
            let previous = depths.insert(
                depth,
                DepthCounters {
                    depth,
                    full_roots: value(&map, "full_roots") as usize,
                    signed_leaves: value(&map, "signed_leaves") as usize,
                    setup: root.plus(algebra),
                    finite_queries: Vec::new(),
                },
            );
            assert!(previous.is_none(), "duplicate depth row");
        } else if line.starts_with("QUERY ") {
            let map = fields(&line);
            if value(&map, "target_inf") == 1 {
                continue;
            }
            let depth = value(&map, "depth") as usize;
            depths
                .get_mut(&depth)
                .expect("query appeared before depth row")
                .finite_queries
                .push(QueryCounters {
                    norm: operations(&map, "norm"),
                    recovery: operations(&map, "recovery"),
                    direct: operations(&map, "direct"),
                });
        }
    }
    let output: Vec<_> = depths.into_values().collect();
    assert!(!output.is_empty());
    for (index, depth) in output.iter().enumerate() {
        assert_eq!(depth.depth, index + 1);
        assert_eq!(depth.full_roots, 1_usize << depth.depth);
        assert_eq!(depth.finite_queries.len(), 10);
    }
    output
}

fn verifier_passed(path: &str, expected_depths: usize) -> bool {
    let file = File::open(path).expect("cannot open verifier transcript");
    let mut accepted = false;
    for line in BufReader::new(file).lines() {
        let line = line.expect("cannot read verifier transcript");
        if line.starts_with("VERIFY_SUMMARY ") {
            let map = fields(&line);
            accepted = map.get("verdict") == Some(&"pass")
                && value(&map, "depths") as usize == expected_depths
                && value(&map, "queries") as usize == 11 * expected_depths;
        }
    }
    accepted
}

fn summarize(depth: &DepthCounters) -> DepthSummary {
    let max_poly_work = depth
        .finite_queries
        .iter()
        .map(|query| query.norm.unweighted())
        .max()
        .unwrap();
    let mut setup_tenths = [0; 3];
    let mut norm_max_tenths = [0; 3];
    let mut direct_max_tenths = [0; 3];
    let mut min_speedup = [f64::INFINITY; 3];
    let mut speedup_pass = [true; 3];
    for (panel_index, panel) in PANELS.iter().copied().enumerate() {
        setup_tenths[panel_index] = depth.setup.fe_tenths(panel);
        for query in &depth.finite_queries {
            let norm = query.norm.plus(query.recovery).fe_tenths(panel);
            let direct = query.direct.fe_tenths(panel);
            norm_max_tenths[panel_index] = norm_max_tenths[panel_index].max(norm);
            direct_max_tenths[panel_index] = direct_max_tenths[panel_index].max(direct);
            min_speedup[panel_index] = min_speedup[panel_index].min(direct as f64 / norm as f64);
            speedup_pass[panel_index] &= 2 * norm <= direct;
        }
    }
    DepthSummary {
        depth: depth.depth,
        full_roots: depth.full_roots,
        signed_leaves: depth.signed_leaves,
        max_poly_work,
        setup_tenths,
        norm_max_tenths,
        direct_max_tenths,
        min_speedup,
        speedup_pass,
    }
}

fn least_squares_slope(points: &[(f64, f64)]) -> f64 {
    let count = points.len() as f64;
    let mean_x = points.iter().map(|point| point.0).sum::<f64>() / count;
    let mean_y = points.iter().map(|point| point.1).sum::<f64>() / count;
    let numerator = points
        .iter()
        .map(|point| (point.0 - mean_x) * (point.1 - mean_y))
        .sum::<f64>();
    let denominator = points
        .iter()
        .map(|point| (point.0 - mean_x).powi(2))
        .sum::<f64>();
    numerator / denominator
}

fn escaped(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

fn parse_paths() -> (String, String) {
    let mut arguments = std::env::args().skip(1);
    let mut candidate = None;
    let mut verifier = None;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--candidate" => candidate = arguments.next(),
            "--verifier" => verifier = arguments.next(),
            "--help" | "-h" => {
                println!("usage: dickson-norm-gcd-analyze --candidate PATH --verifier PATH");
                std::process::exit(0);
            }
            _ => panic!("unknown argument: {argument}"),
        }
    }
    (
        candidate.expect("--candidate is required"),
        verifier.expect("--verifier is required"),
    )
}

fn main() {
    let (candidate_path, verifier_path) = parse_paths();
    let depths = read_candidate(&candidate_path);
    let summaries: Vec<_> = depths.iter().map(summarize).collect();
    let correctness_pass = verifier_passed(&verifier_path, summaries.len());
    assert!(correctness_pass, "independent verifier did not pass");

    let complete_ladder = summaries.len() == 6;
    let (ratio_4_5, ratio_5_6, slope, growth_pass) = if complete_ladder {
        let ratio_4_5 = summaries[4].max_poly_work as f64 / summaries[3].max_poly_work as f64;
        let ratio_5_6 = summaries[5].max_poly_work as f64 / summaries[4].max_poly_work as f64;
        let points: Vec<_> = summaries[2..6]
            .iter()
            .map(|row| {
                (
                    (row.full_roots as f64).ln(),
                    (row.max_poly_work as f64).ln(),
                )
            })
            .collect();
        let slope = least_squares_slope(&points);
        (
            ratio_4_5,
            ratio_5_6,
            slope,
            ratio_4_5 < 4.0 && ratio_5_6 < 4.0 && slope < 2.0,
        )
    } else {
        (f64::NAN, f64::NAN, f64::NAN, false)
    };

    let depth_six = summaries.last().unwrap();
    let speedup_pass = complete_ladder && depth_six.speedup_pass.iter().all(|&value| value);
    let mut projection_totals = [0_u128; 3];
    let mut projection_passes = [false; 3];
    for panel_index in 0..3 {
        projection_totals[panel_index] = depth_six.setup_tenths[panel_index]
            + RESIDUAL_QUERIES * depth_six.norm_max_tenths[panel_index];
        projection_passes[panel_index] =
            complete_ladder && projection_totals[panel_index] <= LIMIT_TENTHS;
    }
    let primitive_projection_pass = projection_passes.iter().all(|&value| value);
    let composition_status = "absent-two-leaf-primitive-only";
    let composition_pass = false;
    let promote = correctness_pass
        && complete_ladder
        && growth_pass
        && speedup_pass
        && primitive_projection_pass
        && composition_pass;
    let verdict = if promote {
        "promote-to-same-target-batching"
    } else if complete_ladder {
        "stop-primitive-only"
    } else {
        "development-only"
    };

    println!("{{");
    println!("  \"schema\": \"isogeny-labs/pplus-dickson-norm-gcd-analysis/v1\",");
    println!("  \"candidate\": \"{}\",", escaped(&candidate_path));
    println!("  \"verifier\": \"{}\",", escaped(&verifier_path));
    println!("  \"correctness_pass\": {correctness_pass},");
    println!("  \"complete_ladder\": {complete_ladder},");
    println!("  \"residual_queries\": {RESIDUAL_QUERIES},");
    println!("  \"budget_fe_tenths\": {LIMIT_TENTHS},");
    println!("  \"composition_status\": \"{composition_status}\",");
    println!("  \"composition_pass\": {composition_pass},");
    println!("  \"depths\": [");
    for (index, row) in summaries.iter().enumerate() {
        println!("    {{");
        println!("      \"depth\": {},", row.depth);
        println!("      \"full_roots\": {},", row.full_roots);
        println!("      \"signed_leaves\": {},", row.signed_leaves);
        println!("      \"max_poly_work\": {},", row.max_poly_work);
        println!(
            "      \"setup_fe_tenths\": [{}, {}, {}],",
            row.setup_tenths[0], row.setup_tenths[1], row.setup_tenths[2]
        );
        println!(
            "      \"norm_max_fe_tenths\": [{}, {}, {}],",
            row.norm_max_tenths[0], row.norm_max_tenths[1], row.norm_max_tenths[2]
        );
        println!(
            "      \"direct_max_fe_tenths\": [{}, {}, {}],",
            row.direct_max_tenths[0], row.direct_max_tenths[1], row.direct_max_tenths[2]
        );
        println!(
            "      \"minimum_direct_over_norm\": [{:.9}, {:.9}, {:.9}],",
            row.min_speedup[0], row.min_speedup[1], row.min_speedup[2]
        );
        println!(
            "      \"two_x_speedup_pass\": [{}, {}, {}]",
            row.speedup_pass[0], row.speedup_pass[1], row.speedup_pass[2]
        );
        println!(
            "    }}{}",
            if index + 1 == summaries.len() {
                ""
            } else {
                ","
            }
        );
    }
    println!("  ],");
    if complete_ladder {
        println!("  \"growth\": {{");
        println!("    \"ratio_4_to_5\": {ratio_4_5:.9},");
        println!("    \"ratio_5_to_6\": {ratio_5_6:.9},");
        println!("    \"quadratic_ratio\": 4.0,");
        println!("    \"slope_depth_3_to_6\": {slope:.9},");
        println!("    \"pass\": {growth_pass}");
        println!("  }},");
    } else {
        println!("  \"growth\": null,");
    }
    println!("  \"depth_six_two_x_speedup_pass\": {speedup_pass},");
    println!("  \"primitive_projection\": [");
    for panel_index in 0..3 {
        println!("    {{");
        println!("      \"panel\": \"{}\",", PANELS[panel_index].name);
        println!(
            "      \"query_fe_tenths\": {},",
            depth_six.norm_max_tenths[panel_index]
        );
        println!(
            "      \"setup_fe_tenths\": {},",
            depth_six.setup_tenths[panel_index]
        );
        println!(
            "      \"total_fe_tenths\": {},",
            projection_totals[panel_index]
        );
        println!("      \"pass\": {}", projection_passes[panel_index]);
        println!("    }}{}", if panel_index == 2 { "" } else { "," });
    }
    println!("  ],");
    println!("  \"primitive_projection_pass\": {primitive_projection_pass},");
    println!("  \"promote\": {promote},");
    println!("  \"verdict\": \"{verdict}\"");
    println!("}}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_equivalent_panels_are_exact() {
        let operations = Operations {
            add: 3,
            sub: 5,
            mul: 7,
            square: 11,
            inverse: 13,
        };
        assert_eq!(operations.fe_tenths(PANELS[0]), 6_738);
        assert_eq!(operations.fe_tenths(PANELS[1]), 13_260);
        assert_eq!(operations.fe_tenths(PANELS[2]), 26_260);
    }

    #[test]
    fn slope_control_recovers_power() {
        let points: Vec<_> = [8_f64, 16.0, 32.0, 64.0]
            .into_iter()
            .map(|x| (x.ln(), (3.0 * x.powf(1.5)).ln()))
            .collect();
        assert!((least_squares_slope(&points) - 1.5).abs() < 1e-12);
    }

    #[test]
    fn operation_addition_is_componentwise() {
        let left = Operations {
            add: 1,
            sub: 2,
            mul: 3,
            square: 4,
            inverse: 5,
        };
        let right = Operations {
            add: 10,
            sub: 20,
            mul: 30,
            square: 40,
            inverse: 50,
        };
        assert_eq!(
            left.plus(right),
            Operations {
                add: 11,
                sub: 22,
                mul: 33,
                square: 44,
                inverse: 55,
            }
        );
    }
}
