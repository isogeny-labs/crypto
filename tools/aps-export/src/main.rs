use aps_measurement_export::{audit, package, write_new, EvidenceRoot, Result};
use std::path::Path;

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str){
        Some("build") if args.len()==6=>{
            let root=EvidenceRoot::new(Path::new(&args[2]))?;
            let export=package::build(&root,&args[3])?;
            if args[4]!="--out"{return Err("expected --out".into());}
            write_new(Path::new(&args[5]),&export)?;
        }
        Some("check") if args.len()==4=>{let root=EvidenceRoot::new(Path::new(&args[2]))?;let export=root.json(&args[3])?;package::check(&root,&export)?;}
        Some("audit-record") if args.len()==4=>{
            let root=EvidenceRoot::new(Path::new(&args[2]))?;let bytes=root.bytes(&args[3])?;
            let line=std::str::from_utf8(&bytes).map_err(|e|e.to_string())?.trim_end_matches('\n');
            let r=audit::record_line(line)?;let report=serde_json::json!({"metrics":audit::metrics(&r)?,"mathematical":audit::mathematical(&r,audit::MAX_ORACLE_CANDIDATES)?});
            println!("{}",serde_json::to_string_pretty(&report).map_err(|e|e.to_string())?);
        }
        Some("audit-inclusive") if args.len()==5=>{
            let root=EvidenceRoot::new(Path::new(&args[2]))?;
            let bytes=root.bytes(&args[3])?;
            let r=audit::record_line(std::str::from_utf8(&bytes).map_err(|e|e.to_string())?.trim_end_matches('\n'))?;
            let (report,_)=aps_measurement_export::inclusive::read(&root,&args[4],&r)?;
            println!("{}",serde_json::to_string_pretty(&report).map_err(|e|e.to_string())?);
        }
        _=>return Err("usage: aps-measurement-export build ROOT REQUEST --out NEW_FILE | check ROOT EXPORT | audit-record ROOT RECORD | audit-inclusive ROOT RECORD PACKET".into()),
    }
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("aps-export: {e}");
        std::process::exit(1);
    }
}
