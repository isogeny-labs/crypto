//! Native Rust orchestration/receipt comparisons; all map arithmetic is ecbench.
use serde_json::{json, Value};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).expect("receipt bytes")).expect("receipt JSON")
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 5, "binary library SHA out");
    let (binary, library, sha) = (&args[1], &args[2], &args[3]);
    let out = PathBuf::from(&args[4]);
    fs::create_dir(&out).expect("fresh qualification directory");
    let study = Path::new("research/prime_conductor_gap80_20261031/twist-transport-20261119");
    let mut commands = Vec::new();
    let mut points = 0usize;
    let mut cases = 0usize;
    let mut call = |label: &str,
                    input: &Path,
                    scale: &str,
                    block: usize,
                    window: usize,
                    extra: &[&str],
                    expected: i32|
     -> Value {
        let result = out.join(format!("{label}.json"));
        let mut argv = vec![
            "isogeny-evaluate".to_string(),
            "--input".into(),
            input.to_string_lossy().into_owned(),
            "--out".into(),
            result.to_string_lossy().into_owned(),
            "--native-library".into(),
            library.clone(),
            "--native-sha256".into(),
            sha.clone(),
            "--twist-scale".into(),
            scale.into(),
            "--max-window".into(),
            window.to_string(),
            "--block".into(),
            block.to_string(),
            "--projective-batch".into(),
            "7".into(),
        ];
        if extra.len() == 2 && extra[0] == "--native-sha256" {
            let position = argv.iter().position(|x| x == "--native-sha256").unwrap();
            argv[position + 1] = extra[1].to_string();
        } else {
            argv.extend(extra.iter().map(|x| x.to_string()));
        }
        let status = Command::new(binary)
            .args(&argv)
            .stdout(Stdio::from(
                File::create(out.join(format!("{label}.stdout"))).unwrap(),
            ))
            .stderr(Stdio::from(
                File::create(out.join(format!("{label}.stderr"))).unwrap(),
            ))
            .status()
            .expect("native Rust child");
        fs::write(
            out.join(format!("{label}.exit")),
            format!("{}\n", status.code().unwrap_or(-1)),
        )
        .unwrap();
        commands.push(json!({"label":label,"binary":binary,"argv":argv,"exit":status.code(),"expected_exit":expected}));
        // Persist even if the following control fails.
        fs::write(
            out.join("commands.json"),
            serde_json::to_vec_pretty(&commands).unwrap(),
        )
        .unwrap();
        assert_eq!(status.code(), Some(expected), "{label}");
        if expected == 0 {
            assert!(
                fs::read(out.join(format!("{label}.stderr")))
                    .unwrap()
                    .is_empty(),
                "{label}"
            );
            read(result)
        } else {
            assert!(!result.exists(), "failed control emitted quotient: {label}");
            Value::Null
        }
    };
    let fixtures = [
        (71, 1),
        (71, 2),
        (71, 3),
        (71, 4),
        (71, 5),
        (71, 7),
        (11, 1),
        (11, 2),
        (11, 4),
    ];
    for (p, d) in fixtures {
        let name = format!("p{p}-d{d}");
        let input = study.join("inputs").join(format!("{name}.txt"));
        let reference = read(
            study
                .join("qualification-a")
                .join(format!("{name}-prepared.stdout")),
        );
        assert_eq!(reference["pari_direct_reference"], true);
        for block in [1, 64] {
            let label = format!("{name}-block{block}");
            let row = call(&label, &input, &d.to_string(), block, 64, &[], 0);
            assert_eq!(row["implementation"], "rust_complete_twist_v1");
            for field in [
                "status",
                "p",
                "source",
                "ell",
                "codomain",
                "baby_count",
                "giant_count",
                "kernel_pairs_covered",
                "images",
            ] {
                assert_eq!(row[field], reference[field], "{label}: {field}");
            }
            points += row["images"].as_array().unwrap().len();
            cases += 1;
        }
    }
    let nonsquare = study.join("inputs/p71-d7.txt");
    for (label, scale, window, extra) in [
        ("zero", "0", 64, vec![]),
        ("equal-p", "71", 64, vec![]),
        ("wrong-source", "1", 64, vec![]),
        ("window", "7", 1, vec![]),
        ("baby-window", "7", 64, vec!["--baby-window", "100"]),
        (
            "wrong-library-hash",
            "7",
            64,
            vec![
                "--native-sha256",
                "0000000000000000000000000000000000000000000000000000000000000000",
            ],
        ),
    ] {
        call(
            &format!("reject-{label}"),
            &nonsquare,
            scale,
            1,
            window,
            &extra,
            2,
        );
    }
    let source = fs::read_to_string(&nonsquare).unwrap();
    let extra = out.join("extra-input.txt");
    fs::write(&extra, format!("{source}\nextra\n")).unwrap();
    call("reject-extra-input", &extra, "7", 1, 64, &[], 2);
    let mut lines: Vec<_> = source.lines().collect();
    lines.pop();
    let short = out.join("short-input.txt");
    fs::write(&short, lines.join("\n")).unwrap();
    call("reject-short-input", &short, "7", 1, 64, &[], 2);
    assert_eq!(cases, 18);
    assert_eq!(points, 470);
    let summary = json!({"scope":"Rust evaluator correctness only; no timed performance claim","Rust_complete_map_controls":cases,"PARI_reference_point_inputs":points,"fields":[11,71],"square_and_nonsquare_twists":true,"blocked_and_full_rectangles":true,"negative_controls":8,"full_80bit_crossing_complete":false});
    fs::write(
        out.join("summary.json"),
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    println!("{summary}");
}
