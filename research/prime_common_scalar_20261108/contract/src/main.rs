use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{env, fs, path::Path};

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}
fn validate(schema: &Path, value: &Value) -> Result<(), String> {
    let mut schema: Value = serde_json::from_slice(&fs::read(schema).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    // Canonical schemas use a relative $id. This crate requires an absolute
    // resolution base; all references here are document-local fragments.
    // Change the identifier only, preserving every validation constraint.
    schema["$id"] = json!("https://local.invalid/canonical-schema.json");
    let compiled = jsonschema::JSONSchema::options().with_draft(jsonschema::Draft::Draft202012)
        .compile(&schema).map_err(|e| e.to_string())?;
    let result = compiled.validate(value);
    if let Err(errors) = result {
        return Err(errors.map(|e| format!("{}: {e}", e.instance_path)).collect::<Vec<_>>().join("\n"));
    }
    Ok(())
}
fn read(path: &str) -> Result<Value,String> {
    serde_json::from_slice(&fs::read(path).map_err(|e|e.to_string())?).map_err(|e|e.to_string())
}
fn write(path:&str,v:&Value)->Result<(),String>{fs::write(path,serde_json::to_vec_pretty(v).unwrap()).map_err(|e|e.to_string())}
fn canonical(v:&Value)->String {let mut v=v.clone();v.as_object_mut().unwrap().remove("freeze");digest(&serde_json::to_vec(&v).unwrap())}
fn check(b:bool,s:&str)->Result<(),String>{if b{Ok(())}else{Err(s.into())}}
fn file(path:&str)->Result<Value,String>{
    let bytes=fs::read(path).map_err(|e|e.to_string())?;
    Ok(json!({"path":path,"sha256":digest(&bytes),"bytes":bytes.len()}))
}
fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("freeze") if args.len()==6 => {
            let mut p=read(&args[2])?;
            check(p["freeze"]["status"]!="frozen","already frozen")?;
            check(p["amendments"].as_array().unwrap().is_empty(),"amendments require explicit review")?;
            validate(Path::new(&args[3]),&p)?;
            let hash=canonical(&p);
            p["freeze"]=json!({"status":"frozen","protocol_sha256":hash,"commit":args[4],"frozen_at":args[5],"filed_before_affected_data":true});
            validate(Path::new(&args[3]),&p)?;write(&args[2],&p)?;
            println!("frozen {}",p["freeze"]["protocol_sha256"]);
        }
        Some("export") if args.len()==5 => {
            // Study-scoped adapter: each receipt is one physical process, never
            // a matrix cell or an embedded control disguised as another run.
            let root=&args[2];let p=read(&format!("{root}/protocol.json"))?;
            check(canonical(&p)==p["freeze"]["protocol_sha256"].as_str().unwrap(),"freeze hash")?;
            let summary=read(&format!("{root}/reports/summary.json"))?;
            let mut runs=Vec::new();let mut files=Vec::new();
            for rep in 0..2 {
                let arm=if rep==0 {"paired-panel"} else {"replay"};
                let path=format!("{root}/runs/{rep}");let r=read(&format!("{path}/receipt.json"))?;
                check(r["exit_code"]==0,"unsuccessful run; retain evidence, do not emit success export")?;
                let artifacts=vec![file(&format!("{path}/panel.csv.xz"))?,file(&format!("{path}/receipt.json"))?,file(&format!("{path}/stderr.txt"))?];
                files.extend(artifacts.clone());
                runs.push(json!({"run_id":format!("{arm}/0"),"arm":arm,"instance_id":"0","seed":0,"replicate":0,"warmup":false,"status":"succeeded","outcome_class":"completed","attempts":1,"resumed":0,"checkpoint":null,
                  "measurements":{"counted":{"rows_completed":summary["rows"]},"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":{}},
                  "isolation":{"tier":"L0","contended":null,"blockers":["No timed metric; co-tenants not controlled"]},"verification":{"status":"no_claim","verifier":"Producer exact reconstruction and recount; no independent signature"},"exclusion":{"excluded":false,"rule":null},"artifacts":artifacts,"started_at":r["started_at"],"finished_at":r["finished_at"],"worker":r["worker"],"backend_ref":format!("local-{rep}")}));
            }
            let outputs=vec![file(&format!("{root}/reports/summary.json"))?,file(&format!("{root}/reports/per-level.csv"))?];
            let host=file(&format!("{root}/host.txt"))?;files.push(host.clone());
            let arms:Vec<Value>=p["arms"].as_array().unwrap().iter().map(|a|json!({"id":a["id"],"role":a["role"],"binary_sha256":a["instrument"]["binary_sha256"],"commit":a["instrument"]["commit"]})).collect();
            let e=json!({"schema":"experiment.export/v1","export_id":"prime-common-scalar-20261108","created":args[3],"protocol":{"path":format!("{root}/protocol.json"),"sha256":p["freeze"]["protocol_sha256"],"protocol_id":p["protocol_id"],"version":p["version"]},"producer":{"runtime":"codex","model":null,"session":"common-scalar-20261108","role":"producer"},"source":{"repo":"crypto","commit":p["freeze"]["commit"],"dirty":true},"backend":"local","host":{"path":host["path"],"sha256":host["sha256"],"class_id":null,"summary":{"cpu_model":null,"logical_cpus":null,"memory_bytes":null,"kernel":null,"hypervisor":null,"isolation_tier_max":"L0"}},"arms":arms,"runs":runs,"flow":{"planned":2,"attempted":2,"completed":2,"not_completed":0,"interrupted":0,"resumed":0,"excluded":[]},"files":files,"analysis":{"script":file(&format!("{root}/analyze.cpp"))?,"blinded":false,"blinding_key_sha256":null,"outputs":outputs,"unmasked_at":null},"notes":[{"kind":"observation","text":"Two physical executions. Kernels and controls are embedded. Exact finite census; precision multipliers are proxies, no runtime conclusion.","by":"producer"}],"reviews_dir":"reviews"});
            write(&args[4],&e)?;
        }
        Some("check-export") if args.len()==6 => {
            let p=read(&args[2])?;let e=read(&args[3])?;
            validate(Path::new(&args[4]),&p)?;validate(Path::new(&args[5]),&e)?;
            check(canonical(&p)==p["freeze"]["protocol_sha256"].as_str().unwrap(),"freeze hash")?;
            check(e["protocol"]["sha256"]==p["freeze"]["protocol_sha256"],"export protocol binding")?;
            check(p["sample_size"]["instances"]==1&&p["sample_size"]["replicates"]==1&&p["arms"].as_array().unwrap().len()==2,"physical execution design")?;
            check(e["runs"].as_array().unwrap().len()==2&&e["flow"]["planned"]==2&&e["flow"]["completed"]==2,"physical flow")?;
            check(e["arms"][0]["binary_sha256"]==p["arms"][0]["instrument"]["binary_sha256"],"binary binding")?;
            check(e["analysis"]["script"]["sha256"]==p["analysis_plan"]["script"]["sha256"],"analysis binding")?;
            for (i,r) in e["runs"].as_array().unwrap().iter().enumerate(){
                let arm=if i==0 {"paired-panel"} else {"replay"};
                check(r["replicate"]==0&&r["instance_id"]=="0"&&r["arm"]==arm&&r["seed"]==0,"run identity")?;
                check(r["status"]=="succeeded"&&r["attempts"]==1&&r["resumed"]==0&&r["measurements"]["wall_valid"]==false,"run validity")?;
                check(r["measurements"]["counted"]["rows_completed"]==20928,"complete registered census")?;
                let start=r["started_at"].as_str().ok_or("missing start")?;
                check(start>p["freeze"]["frozen_at"].as_str().unwrap(),"freeze must precede data")?;
            }
            for f in e["files"].as_array().unwrap().iter().chain(e["analysis"]["outputs"].as_array().unwrap()).chain(std::iter::once(&e["analysis"]["script"])) {
                let path=f["path"].as_str().ok_or("missing file path")?;
                let bytes=fs::read(path).map_err(|e|e.to_string())?;
                check(digest(&bytes)==f["sha256"].as_str().unwrap(),&format!("file hash {path}"))?;
                check(f["bytes"].as_u64()==Some(bytes.len() as u64),"file byte length")?;
            }
            println!("Study-scoped native contract PASS: schemas, freeze/analysis/binary/file bindings, physical flow and prospective timestamps. Not a general replacement for the canonical checker; not an independent verdict.");
        }
        Some("schema") if args.len() == 4 => {
            let value: Value = serde_json::from_slice(&fs::read(&args[3]).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
            validate(Path::new(&args[2]), &value)?;
            println!("JSON Schema PASS");
        }
        Some("visual") if args.len()==4 => {
            let v=read(&args[2])?;let groups=v["groups"].as_object().ok_or("groups")?;
            let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1060\" height=\"650\" viewBox=\"0 0 1060 650\"><rect width=\"1060\" height=\"650\" fill=\"white\"/><g font-family=\"sans-serif\" font-size=\"16\" fill=\"#17212b\"><text x=\"20\" y=\"28\" font-size=\"22\">Frozen exact tuple census: every outcome retained</text><text x=\"20\" y=\"54\">Producer evidence, unsigned. Fewer / equal / more candidate tuples; no timing metric.</text>");
            let mut tex=String::from("\\begin{tikzpicture}[x=0.012cm,y=0.72cm]\\small\n");
            for (i,(key,s)) in groups.iter().enumerate(){
                let counts=[s["fewer"].as_u64().unwrap(),s["equal"].as_u64().unwrap(),s["more"].as_u64().unwrap()];
                let n: u64=counts.iter().sum();let y=100+44*i;let mut x=310.0;let mut tx=0.0;
                svg+=&format!("<text x=\"20\" y=\"{}\">{key}</text>",y+20);
                tex+=&format!("\\node[anchor=east] at (-8,-{}){{\\texttt{{{}}}}};\n",i,key.replace('_',"\\_"));
                for (j,c) in counts.iter().enumerate(){let w=500.0*(*c as f64)/(n as f64);let color=["#2c7b58","#9babb7","#c64a4a"][j];
                    svg+=&format!("<rect x=\"{x:.5}\" y=\"{y}\" width=\"{w:.5}\" height=\"28\" fill=\"{color}\"/>");
                    tex+=&format!("\\fill[{}] ({tx:.6},-{}-0.2) rectangle ({:.6},-{}+0.2);\n",["win","tie","loss"][j],i,tx+w,i);
                    x+=w;tx+=w;
                }
                svg+=&format!("<text x=\"830\" y=\"{}\">{} / {} / {} (n={n})</text>",y+20,counts[0],counts[1],counts[2]);
                tex+=&format!("\\node[anchor=west] at (508,-{}){{{} / {} / {} ($n={n}$)}};\n",i,counts[0],counts[1],counts[2]);
            }
            svg+="<text x=\"310\" y=\"570\">0%</text><text x=\"778\" y=\"570\">100%</text><text x=\"20\" y=\"608\">same: equal precision. proxy: entry ceil(4q/3), scalar 2q. These are conservative proxies.</text><text x=\"20\" y=\"635\">study prime-common-scalar-20261108; reports/summary.json; 2 physical executions.</text></g></svg>";
            tex+="\\end{tikzpicture}\n";
            fs::write(format!("{}/census.svg",args[3]),svg).map_err(|e|e.to_string())?;
            fs::write(format!("{}/plot.tex",args[3]),tex).map_err(|e|e.to_string())?;
        }
        Some("hash") if args.len() == 3 => println!("{}", digest(&fs::read(&args[2]).map_err(|e| e.to_string())?)),
        _ => return Err("use schema SCHEMA JSON | hash FILE | freeze PROTOCOL SCHEMA COMMIT UTC | export ROOT UTC OUTPUT | check-export PROTOCOL EXPORT PROTOCOL_SCHEMA EXPORT_SCHEMA".into()),
    }
    Ok(())
}
