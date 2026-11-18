// Journal writes are outside meter::measure. A complete marker follows synced output.
fn journal_event(file:&mut std::fs::File,ordinal:usize,phase:&str,protocol:&str,input:&str){
 writeln!(file,"{{\"ordinal\":{},\"phase\":\"{}\",\"protocol_sha256\":\"{}\",\"input_sha256\":\"{}\"}}",ordinal,phase,protocol,input).unwrap();
 file.sync_all().unwrap();
}
fn journal_self_test(){
 let path=std::env::temp_dir().join(format!("resultant-journal-{}-{}",std::process::id(),std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
 let mut file=std::fs::OpenOptions::new().write(true).create_new(true).open(&path).unwrap();
 journal_event(&mut file,0,"start","synthetic","synthetic");journal_event(&mut file,0,"complete","synthetic","synthetic");drop(file);
 let text=std::fs::read_to_string(&path).unwrap();assert_eq!(text.lines().count(),2);assert!(text.lines().next().unwrap().contains("\"phase\":\"start\""));assert!(text.lines().nth(1).unwrap().contains("\"phase\":\"complete\""));
 assert!(std::fs::OpenOptions::new().write(true).create_new(true).open(&path).is_err());std::fs::remove_file(path).unwrap();
}
