//! Charge every registered branch; incomplete/censored totals stay unknown.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 2, "STUDY RUN");
    let study = Path::new(&args[0]);
    let run = Path::new(&args[1]);
    let input = std::fs::read(study.join("inputs/guess-ell8.jsonl")).unwrap();
    let input_hash = format!("{:x}", Sha256::digest(input));
    let mut attempts = BTreeMap::<(u64, usize, u64), Value>::new();
    let mut completed_new = 0u64;
    let mut load = |dir: &Path, old: bool| {
        let status_path = dir.join("status.json");
        if !status_path.exists() {
            return;
        }
        let status = read(&status_path);
        let command = read(&dir.join("command.json"));
        let argv = command["argv"].as_array().unwrap();
        let arg = |name: &str| {
            argv.windows(2).find(|w| w[0] == name).unwrap()[1]
                .as_str()
                .unwrap()
        };
        let draw = arg("--draw").parse::<u64>().unwrap();
        let branch = arg("--first-branch").parse::<u64>().unwrap();
        assert_eq!(arg("--end-branch").parse::<u64>().unwrap(), branch + 1);
        let vars = match arg("--vars") {
            "none" => vec![],
            s => s
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>(),
        };
        assert_eq!(vars, [7, 15, 23, 24][..vars.len()]);
        let k = vars.len();
        assert!([0, 3, 5, 6].contains(&draw) && k <= 4 && branch < 1 << k);
        let bytes = std::fs::read(dir.join("rows.jsonl")).unwrap();
        let rows: Vec<Value> = std::str::from_utf8(&bytes)
            .unwrap()
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let row = rows.first().cloned().unwrap_or(Value::Null);
        if !row.is_null() {
            assert_eq!(rows.len(), 1);
            assert_eq!(row["input_sha256"], input_hash);
            assert_eq!(row["draw"], draw);
            assert_eq!(row["branch"], branch);
            assert_eq!(row["degree"], 4);
            assert_eq!(row["guessed_variables"], json!(vars));
        }
        let verdict = if status["exit_code"] == 0 {
            match row["status"].as_str() {
                Some("refuted") => "refuted",
                Some("saturated_not_refuted") => "saturated_not_refuted",
                _ => "unknown",
            }
        } else {
            "unknown"
        };
        let words = (verdict != "unknown").then(|| row["closure_word_ops"].as_u64().unwrap());
        let item = json!({"draw":draw,"guesses":k,"branch":branch,"verdict":verdict,"solver_word_ops":words,
            "exit_code":status["exit_code"],"legacy_draw0_k3":old,
            "rows_sha256":format!("{:x}",Sha256::digest(&bytes))});
        assert!(
            attempts.insert((draw, k, branch), item).is_none(),
            "duplicate branch"
        );
        if !old {
            completed_new += 1;
        }
    };
    let mut dirs: Vec<_> = std::fs::read_dir(run)
        .unwrap()
        .map(Result::unwrap)
        .filter(|d| d.file_type().unwrap().is_dir())
        .collect();
    dirs.sort_by_key(|d| d.file_name());
    for dir in dirs {
        if dir.file_name().to_str().unwrap().starts_with("cell-") {
            load(&dir.path(), false);
        }
    }
    for branch in 0..8 {
        load(
            &study.join(format!(
                "runs/guess-balanced-k3-d0-b{branch}-20261023-02/cell-0000"
            )),
            true,
        );
    }
    let mut groups = vec![];
    for draw in [0u64, 3, 5, 6] {
        for k in 0usize..=4 {
            let expected = 1u64 << k;
            let cells: Vec<_> = (0..expected)
                .filter_map(|b| attempts.get(&(draw, k, b)))
                .collect();
            let refuted = cells.iter().filter(|c| c["verdict"] == "refuted").count() as u64;
            let nonrefuting = cells
                .iter()
                .filter(|c| c["verdict"] == "saturated_not_refuted")
                .count() as u64;
            let unknown = cells.iter().filter(|c| c["verdict"] == "unknown").count() as u64;
            let missing = expected - cells.len() as u64;
            let mut known_words = 0u64;
            for cell in &cells {
                if let Some(n) = cell["solver_word_ops"].as_u64() {
                    known_words = known_words.checked_add(n).unwrap();
                }
            }
            let all_refuted = refuted == expected;
            let verdict = if all_refuted {
                "all_branches_refuted"
            } else if nonrefuting > 0 {
                "has_nonrefuting_branch"
            } else {
                "unknown"
            };
            let guessed_variables = [7, 15, 23, 24][..k].to_vec();
            groups.push(json!({"draw":draw,"guesses":k,"guessed_variables":guessed_variables,
                "expected_branches":expected,"recorded_attempts":cells.len(),"refuted":refuted,
                "saturated_nonrefuting":nonrefuting,"unknown_attempts":unknown,"missing_attempts":missing,
                "verdict":verdict,"all_branch_solver_word_ops":(unknown==0&&missing==0).then_some(known_words),
                "known_completed_solver_word_ops":known_words}));
        }
    }
    assert!(completed_new <= 116);
    let ended = run.join("ended.txt").exists();
    if ended {
        assert_eq!(completed_new, 116);
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-balanced-guess-checkpoint/v1",
        "input_sha256":input_hash,"planned_new_attempts":116,"recorded_new_attempts":completed_new,
        "runner_ended":ended,"groups":groups,"attempts":attempts.into_values().collect::<Vec<_>>(),
        "scope":"Fixed balanced sets only. Unknown branches keep complete cost null. Counts exclude construction/splitting/physical prices. No global minimum over alternative sets or degree law."})).unwrap());
}
