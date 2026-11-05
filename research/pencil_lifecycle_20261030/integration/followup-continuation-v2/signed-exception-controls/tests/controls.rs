use serde_json::{json, Value};
use signed_collector_label_check::verify;
use signed_collector_replay::{parse_expected_header, replay_jsonl, Limits};
use std::{fs, path::PathBuf};
fn fixture() -> (Value, Vec<Value>) {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../signed-replay/fixtures");
    let h = parse_expected_header(&fs::read(p.join("0.header.json")).unwrap()).unwrap();
    let rows = fs::read_to_string(p.join("0.jsonl"))
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str::<Value>(s).unwrap()["event"].clone())
        .collect();
    (h, rows)
}
fn wire(events: &[Value]) -> Vec<u8> {
    events
        .iter()
        .enumerate()
        .map(|(i, e)| serde_json::to_string(&json!({"sequence":i,"event":e})).unwrap() + "\n")
        .collect::<String>()
        .into_bytes()
}
fn terminal(attempts: u64, cont: u64, split: u64, dup: u64, nodes: u64, k: Option<u64>) -> Value {
    json!({"kind":"terminal","attempts":attempts,"continuations":cont,"split_relations":split,"duplicates":dup,"node_count":nodes,"recovered":k,"exhausted":k.is_none(),"source_verified":false})
}
fn choice(cont: u64, attempt: u64, repeated: bool) -> Value {
    json!({"kind":"choice","continuation":cont,"selected_attempt":attempt,"repeated":repeated})
}
fn identity(attempt: u64, cont: u64, r: u64, recover: bool) -> [Value; 2] {
    [
        json!({"kind":"candidate","attempt":attempt,"continuation":cont,"jump_index":0,"divisor":{"degree":0,"u":[1,0,0],"v":[0,0]},"a":if recover{r-2}else{0},"b":if recover{1}else{0}}),
        json!({"kind":"identity","attempt":attempt,"recovered":if recover{Some(2)}else{None}}),
    ]
}
fn restart(reason: &str, cont: u64, after: u64) -> Value {
    json!({"kind":"restart","reason":reason,"continuation":cont,"after_attempt":after,"divisor":{"degree":0,"u":[1,0,0],"v":[0,0]},"a":0,"b":0})
}
fn duplicate(
    c: &Value,
    canonical: &Value,
    attempt: u64,
    cont: u64,
    opposite: bool,
    p: u64,
    r: u64,
) -> [Value; 2] {
    let mut c = c.clone();
    c["attempt"] = json!(attempt);
    c["continuation"] = json!(cont);
    c["jump_index"] = json!(if attempt % 2 == 0 { 1 } else { 0 });
    if opposite {
        for k in ["a", "b"] {
            let x = c[k].as_u64().unwrap();
            c[k] = json!(if x == 0 { 0 } else { r - x });
        }
        for i in 0..2 {
            let x = c["divisor"]["v"][i].as_u64().unwrap();
            c["divisor"]["v"][i] = json!(if x == 0 { 0 } else { p - x });
        }
    }
    [
        c,
        json!({"kind":"duplicate","attempt":attempt,"canonical":canonical,"first_attempt":1}),
    ]
}
fn controls() -> Vec<(&'static str, Vec<Value>)> {
    let (h, original) = fixture();
    let r = h["r"].as_u64().unwrap();
    let p = h["p"].as_u64().unwrap();
    let mut cases = vec![];
    for recover in [true, false] {
        let mut h = h.clone();
        h["candidate_cap"] = json!(1);
        let mut e = vec![h];
        e.extend(identity(1, 1, r, recover));
        if !recover {
            e.push(choice(1, 1, false));
        }
        e.push(terminal(
            1,
            1,
            0,
            0,
            1,
            if recover { Some(2) } else { None },
        ));
        cases.push((
            if recover {
                "identity_recovery"
            } else {
                "identity_zero"
            },
            e,
        ));
    }
    let end = original.iter().position(|e| e["kind"] == "edge").unwrap();
    assert_eq!(original[1]["kind"], "candidate");
    assert_eq!(original[1]["attempt"], 1);
    let nodes = original[end]["node_count"].as_u64().unwrap();
    let canonical = original[end]["canonical"].clone();
    let mut prefix = original[..=end].to_vec();
    prefix[1]["jump_index"] = json!(0);
    prefix.extend(duplicate(&original[1], &canonical, 2, 1, true, p, r));
    prefix.push(choice(1, 1, false));
    let mut cap = prefix.clone();
    cap[0]["candidate_cap"] = json!(2);
    cap.push(terminal(2, 1, 1, 1, nodes, None));
    cases.push(("opposite_duplicate_cap", cap));
    let mut chosen = prefix.clone();
    chosen[0]["candidate_cap"] = json!(4);
    chosen.extend(duplicate(&original[1], &canonical, 3, 2, false, p, r));
    chosen.extend(duplicate(&original[1], &canonical, 4, 2, true, p, r));
    chosen.push(choice(2, 3, true));
    chosen.push(restart("chosen_duplicate", 2, 4));
    chosen.push(terminal(4, 2, 1, 3, nodes, None));
    cases.push(("chosen_duplicate_restart", chosen));
    let mut walk = prefix;
    walk[0]["candidate_cap"] = json!(3);
    walk.push(restart("walk_duplicate", 2, 2));
    walk.extend(identity(3, 2, r, true));
    walk.push(terminal(3, 2, 1, 1, nodes, Some(2)));
    cases.push(("walk_duplicate_restart", walk));
    cases
}
#[test]
fn positive_constructed_controls() {
    for (name, e) in controls() {
        let v = verify(&wire(&e), &e[0], Limits::default()).unwrap();
        println!(
            "EXCEPTION_CONTROL {}",
            json!({"name":name,"provenance":"constructed_non_scheduler_control","native_trace":false,"events":e,"verification":v})
        );
    }
}
#[test]
fn transition_corruptions_reject() {
    let cs = controls();
    let chosen = &cs[3].1;
    let walk = &cs[4].1;
    let cap = &cs[2].1;
    let mut cases = vec![];
    let mut e = chosen.clone();
    e.remove(e.len() - 2);
    cases.push(("missing_required_restart", e));
    let mut e = cap.clone();
    e.insert(e.len() - 1, restart("chosen_duplicate", 1, 2));
    cases.push(("unsolicited_chosen_restart", e));
    let mut e = chosen.clone();
    e.insert(e.len() - 1, e[e.len() - 2].clone());
    cases.push(("repeated_chosen_restart", e));
    let i = walk.iter().position(|e| e["kind"] == "restart").unwrap();
    let mut e = walk.clone();
    e.insert(i, e[i].clone());
    cases.push(("repeated_walk_restart", e));
    let mut e = cap.clone();
    e.insert(e.len() - 1, restart("walk_duplicate", 2, 2));
    cases.push(("walk_restart_after_cap", e));
    let mut e = cap.clone();
    let j = e
        .iter()
        .position(|e| e["kind"] == "candidate" && e["attempt"] == 2)
        .unwrap();
    e[j]["jump_index"] = json!(0);
    cases.push(("repeated_jump_index", e));
    let cut = chosen
        .iter()
        .position(|x| x["kind"] == "candidate" && x["attempt"] == 4)
        .unwrap();
    let mut e = chosen[..cut].to_vec();
    e.push(choice(2, 3, true));
    e.push(restart("chosen_duplicate", 2, 3));
    let mut c4 = chosen[cut].clone();
    c4["continuation"] = json!(3);
    e.push(c4);
    e.push(chosen[cut + 1].clone());
    e.push(choice(3, 4, true));
    e.push(restart("chosen_duplicate", 3, 4));
    let mut t = chosen.last().unwrap().clone();
    t["continuations"] = json!(3);
    e.push(t);
    cases.push(("early_choice_before_cap", e));
    let mut accepted = vec![];
    for (name, e) in cases {
        let result = replay_jsonl(&wire(&e), &e[0], Limits::default());
        let ok = result.is_ok();
        println!(
            "TRANSITION_CONTROL {}",
            json!({"name":name,"accepted":result.is_ok(),"error":result.err()})
        );
        if ok {
            accepted.push(name);
        }
    }
    assert!(accepted.is_empty(), "incorrectly accepted {accepted:?}");
}
#[test]
fn saved_native_streams_still_verify() {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../signed-replay/fixtures");
    let mut n = 0;
    for i in 0..6 {
        let h =
            parse_expected_header(&fs::read(p.join(format!("{i}.header.json"))).unwrap()).unwrap();
        let v = verify(
            &fs::read(p.join(format!("{i}.jsonl"))).unwrap(),
            &h,
            Limits::default(),
        )
        .unwrap();
        n += v.candidate_equations;
    }
    assert_eq!(n, 731);
    println!("SAVED_NATIVE_CHECK 6 streams 731 candidate equations");
}
