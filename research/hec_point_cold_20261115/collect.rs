use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
const BUILD: &str = match option_env!("HEC_SOURCE") {
    Some(s) => s,
    None => "UNFROZEN",
};
fn cell(worker: &str, root: &Path, key: &str, args: &[String], cap: u64) -> (String, u128) {
    let out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(format!("{key}.out")))
        .unwrap();
    let err = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(format!("{key}.err")))
        .unwrap();
    let now = Instant::now();
    let mut child = Command::new(worker)
        .args(args)
        .stdout(Stdio::from(out))
        .stderr(Stdio::from(err))
        .spawn()
        .unwrap();
    let pid = root.join(format!("{key}.pid"));
    fs::write(&pid, format!("{}\n{BUILD}\n{args:?}\n", child.id())).unwrap();
    let status = loop {
        if let Some(s) = child.try_wait().unwrap() {
            break format!("exit_{}", s.code().unwrap_or(-1));
        }
        if now.elapsed() >= Duration::from_secs(cap) {
            child.kill().unwrap();
            child.wait().unwrap();
            break "timeout".into();
        }
        thread::sleep(Duration::from_millis(5));
    };
    let wall = now.elapsed().as_nanos();
    fs::remove_file(pid).unwrap();
    let mut r = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(format!("{key}.receipt")))
        .unwrap();
    writeln!(r, "HEC-POINT-COLD-v1 {BUILD} {args:?}\n{status} {wall}").unwrap();
    println!("STATUS {key} {status} {wall}");
    (status, wall)
}
fn main() {
    assert_ne!(BUILD, "UNFROZEN");
    let a: Vec<_> = std::env::args().collect();
    if a[1] == "--self-test" {
        let root = Path::new(&a[2]);
        fs::create_dir(root).unwrap();
        assert_eq!(cell("/usr/bin/true", root, "success", &[], 2).0, "exit_0");
        assert_eq!(
            cell("/usr/bin/false", root, "rejection", &[], 2).0,
            "exit_1"
        );
        assert_eq!(
            cell("/bin/sleep", root, "timeout", &["2".into()], 0).0,
            "timeout"
        );
        println!("CONTROLS success/rejection/owned-timeout=true");
        return;
    }
    let root = Path::new(&a[2]);
    fs::create_dir(root).expect("new root required; inspect any old processes before resuming");
    let origin = Path::new(&a[3]);
    let mut used = 0;
    let mut cells = 0;
    let mut gates = 0;
    for (fi, (g, p)) in [
        (2, 1009),
        (2, 16381),
        (3, 127),
        (3, 1009),
        (4, 31),
        (4, 127),
    ]
    .into_iter()
    .enumerate()
    {
        let fixture = origin.join(format!("select-g{g}-p{p}-f0.out"));
        for (si, seed) in [11, 12, 13].into_iter().enumerate() {
            assert!(
                used + 60_000_000_000 < 1_800_000_000_000u128,
                "aggregate budget"
            );
            let prefix = format!("g{g}-p{p}-s{seed}");
            let fixture = fixture.to_str().unwrap().to_string();
            let (gate, w) = cell(
                &a[1],
                root,
                &format!("gate-{prefix}"),
                &[fixture.clone(), seed.to_string(), "point-gate".into()],
                60,
            );
            used += w;
            gates += 1;
            let mut arms = vec![
                "n1fr",
                "n1fd",
                "n1hr",
                "n1hd",
                "n64fr",
                "n64fd",
                "n64hr",
                "n64hd",
                "t1f",
                "t1h",
                "t64f",
                "t64h",
                "compact",
                "rational-generic",
                "point-dense",
                "point-lazy",
                "point-lazy-aa",
            ];
            arms.rotate_left((fi * 3 + si) % 17);
            if si % 2 == 1 {
                arms.reverse();
            }
            for arm in arms {
                assert!(
                    used + 30_000_000_000 < 1_800_000_000_000u128,
                    "aggregate budget"
                );
                let key = format!("time-{prefix}-{arm}");
                if arm.starts_with("point-") && gate != "exit_0" {
                    fs::write(
                        root.join(format!("{key}.receipt")),
                        format!("HEC-POINT-COLD-v1 {BUILD}\ngate_failed 0\n"),
                    )
                    .unwrap();
                    continue;
                }
                let (_, w) = cell(
                    &a[1],
                    root,
                    &key,
                    &[fixture.clone(), seed.to_string(), arm.into()],
                    30,
                );
                used += w;
                cells += 1;
            }
        }
    }
    println!("TERMINAL gates={gates} timed_cells={cells} child_wall_ns={used}");
}
