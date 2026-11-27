use aps_collection_architectures::{partials, partitions, recursive, Fixture, FixtureId, Work};
use serde_json::{json, Value};
use std::{fs::OpenOptions, io::Write, process::ExitCode};

fn arm(index: usize) -> Result<Value, String> {
    let mut panels = vec![];
    for id in [FixtureId::P17, FixtureId::P101] {
        let f = Fixture::new(id);
        match index {
            0 | 2 => {
                let ks = if index == 0 { vec![7] } else { vec![7, 11, 16] };
                let qs: Vec<_> = ks.iter().map(|&k| f.point(k)).collect();
                let pool: Vec<_> = (9..15).map(|k| f.point(k)).collect();
                for two in [false, true] {
                    let mut w = Work::default();
                    let r = partials::collect(f, &qs, &pool, two, 512, &mut w)?;
                    if r.recovered.as_ref() != Some(&ks) {
                        return Err(format!("incomplete recovery {id:?}"));
                    }
                    panels.push(
                        json!({"fixture":id,"targets":qs,"known_answer_check_only":ks,
                        "large_point_pool":pool,"two_large_points":two,"work":w,"result":r}),
                    );
                }
            }
            1 => {
                let base: Vec<_> = [0, 1, 2, 3, 4, 5, 6, f.order() - 1]
                    .map(|k| f.point(k))
                    .into();
                let mut setup = Work::default();
                let table = recursive::PairTable::build(f, &base, &mut setup)?;
                let mut queries = vec![];
                for k in [7, 11, 16] {
                    let q = f.point(k);
                    let mut online = Work::default();
                    let mut coldwork = Work::default();
                    let mut directwork = Work::default();
                    let shared = table.query(f, &base, q, &mut online)?;
                    let cold = recursive::PairTable::build(f, &base, &mut coldwork)?.query(
                        f,
                        &base,
                        q,
                        &mut coldwork,
                    )?;
                    let expected = recursive::direct(f, &base, q, &mut directwork);
                    if shared != expected || cold != expected {
                        return Err("root set mismatch".into());
                    }
                    queries.push(json!({"target":q,"roots":shared,"cold_work":coldwork,
                        "shared_query_work":online,"direct_work":directwork}));
                }
                panels.push(json!({"fixture":id,"base":base,"shared_setup_work":setup,
                    "shared_setup_charged_once_to_batch_only":true,"queries":queries}));
            }
            3 => {
                let pilot = [(f.point(3), 11), (f.point(5), 13)];
                let mut training = Work::default();
                let model = partitions::FrozenRule::train(f, &pilot, &mut training)?;
                let mut heldout = vec![];
                for k in [7, 11, 16] {
                    let mut work = Work::default();
                    let result = model.test(f, f.point(k), 17, &mut work)?;
                    let mut baseline_work = Work::default();
                    let baseline = partitions::evaluate(f, f.point(k), 17, 0, &mut baseline_work)?;
                    heldout.push(json!({"selected":result,"work":work,
                        "fixed_index_rule_baseline":baseline,"baseline_work":baseline_work}));
                }
                panels.push(json!({"fixture":id,"pilot":pilot,"model":model,
                    "training_work":training,"heldout":heldout}));
            }
            _ => return Err("unknown arm".into()),
        }
    }
    Ok(json!(panels))
}

fn dispatch(transport: bool) -> Value {
    let names = [
        "large_point_partials",
        "recursive_shared_pairs",
        "coupled_targets",
        "pilot_frozen_partitions",
    ];
    let mut arms = vec![];
    let mut completed = 0;
    for (i, name) in names.iter().enumerate() {
        let result = if transport {
            Err("intentional wrapper transport-only failure".into())
        } else {
            arm(i)
        };
        match result {
            Ok(value) => {
                completed += 1;
                arms.push(json!({"name":name,"complete":true,"panels":value}));
            }
            Err(e) => arms.push(json!({"name":name,"complete":false,"error":e})),
        }
    }
    json!({"schema":"aps.collection-architecture-qualification/v1",
        "producer_standing":"unverified","transport_only":transport,
        "performance":false,"total_cost_priced":false,"production_aps_f4":false,
        "flow":{"planned":4,"attempted":4,"completed":completed,"excluded":0},"arms":arms})
}

fn run() -> Result<bool, String> {
    let mut args = std::env::args().skip(1);
    let output = args
        .next()
        .ok_or("usage: aps-collection-architectures OUTPUT [--transport-control]")?;
    let transport = match args.next().as_deref() {
        None => false,
        Some("--transport-control") => true,
        _ => return Err("unknown option".into()),
    };
    if args.next().is_some() {
        return Err("extra argument".into());
    }
    // Own raw path before doing any scientific work; stdout is a separate receipt.
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output)
        .map_err(|e| e.to_string())?;
    let report = dispatch(transport);
    serde_json::to_writer_pretty(&mut file, &report).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    println!(
        "{}",
        json!({"output":output,"transport_only":transport,"flow":report["flow"]})
    );
    Ok(transport || report["flow"]["completed"] == 4)
}
fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
