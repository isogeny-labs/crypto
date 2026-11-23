use isolab_eks_recovery::{cloud, contract::Request, io::*, supervisor};
use std::{os::unix::process::CommandExt, path::Path, process::Command};
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    match args.get(1).map(String::as_str){
  Some("validate") if args.len()==3=>{let req=Request::parse(&std::fs::read(&args[2]).map_err(error)?)?;println!("{}",serde_json::to_string_pretty(&req).map_err(error)?);Ok(())},
  Some("supervise") if args.len()==4=>supervisor::supervise(Request::parse(&std::fs::read(&args[2]).map_err(error)?)?,args[3].parse().map_err(error)?),
  Some("rpc") if args.len()==4=>supervisor::rpc(args[2].parse().map_err(error)?,args[3].as_bytes()),
  Some("status") if args.len()==3=>{println!("{}",serde_json::to_string(&supervisor::status(args[2].parse().map_err(error)?)?).map_err(error)?);Ok(())},
  Some("submit") if args.len() == 4 => { let r=cloud::submit(Request::parse(&std::fs::read(&args[2]).map_err(error)?)?,Path::new(&args[3]))?; println!("{}",r); Ok(()) },
        Some("wait") if args.len() == 4 => { let r=cloud::wait(Request::parse(&std::fs::read(&args[2]).map_err(error)?)?,Path::new(&args[3]))?; println!("{}",r); Ok(()) },
        Some("remote-status") if args.len() == 4 => { let r=cloud::remote_status(&Request::parse(&std::fs::read(&args[2]).map_err(error)?)?,Path::new(&args[3]))?; println!("{}",r); Ok(()) },
        Some("controller") if args.len()==4=>cloud::run(Request::parse(&std::fs::read(&args[2]).map_err(error)?)?,Path::new(&args[3])),
  Some("archive") if args.len()==3=>{let n:u64=args[2].parse().map_err(error)?;let err=Command::new("/opt/isolab/recovery/tar").env("LD_LIBRARY_PATH","/opt/isolab/recovery/lib").args(["-C","/work","-cf","-","compute",&format!("generations/g{n}")]).exec();Err(error(err))},
  Some("archive-computation") if args.len()==2=>{let err=Command::new("/opt/isolab/recovery/tar").env("LD_LIBRARY_PATH","/opt/isolab/recovery/lib").args(["-C","/work","-cf","-","compute","control"]).exec();Err(error(err))},
  _=>Err("usage: isolab-eks-recovery validate REQUEST | controller REQUEST RECEIPTS | supervise REQUEST ATTEMPT | rpc ATTEMPT JSON | status ATTEMPT | archive GENERATION | archive-computation".into())
 }
}
fn main() {
    if let Err(e) = run() {
        eprintln!("isolab-eks-recovery: {e}");
        std::process::exit(1)
    }
}
