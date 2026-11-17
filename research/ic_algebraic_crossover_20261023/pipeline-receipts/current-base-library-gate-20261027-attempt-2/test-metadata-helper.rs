use std::{collections::BTreeSet, error::Error, fs, path::Path, process::Command};
fn strings(s: &BTreeSet<String>) -> String {
    format!("[{}]", s.iter().map(|v| format!("{v:?}")).collect::<Vec<_>>().join(","))
}
fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len()!=3 { return Err("usage: helper test-binary receipt-directory".into()); }
    let dir=Path::new(&a[2]);
    let listed=Command::new(&a[1]).args(["--list","--format","terse"]).output()?;
    if !listed.status.success() { return Err("test discovery failed".into()); }
    fs::write(dir.join("listed-tests.txt"), &listed.stdout)?;
    let list=std::str::from_utf8(&listed.stdout)?;
    let expected: BTreeSet<String> = list.lines().filter_map(|v|v.strip_suffix(": test").map(str::to_owned)).collect();
    if expected.is_empty() || expected.iter().any(|v|!v.is_ascii()) { return Err("unexpected test names".into()); }
    let log=fs::read_to_string(dir.join("cargo-test-release-lib.log"))?;
    let (mut passed,mut ignored,mut failed)=(BTreeSet::new(),BTreeSet::new(),BTreeSet::new());
    for line in log.lines() {
        let Some(t)=line.strip_prefix("test ") else {continue};
        let Some((name,result))=t.split_once(" ... ") else {continue};
        let name=name.strip_suffix(" - should panic").unwrap_or(name);
        if !expected.contains(name) {continue}
        if result=="ok" {passed.insert(name.to_owned());}
        else if result.starts_with("ignored") {ignored.insert(name.to_owned());}
        else if result=="FAILED" {failed.insert(name.to_owned());}
    }
    let finished: BTreeSet<String>=passed.union(&ignored).cloned().chain(failed.iter().cloned()).collect();
    let pending: BTreeSet<String>=expected.difference(&finished).cloned().collect();
    let output=format!("{{\"scope\":\"software test-discovery metadata; terminal command status remains authoritative\",\"listed\":{},\"named_passes\":{},\"named_ignores\":{},\"named_failures\":{},\"finished_names\":{},\"pending\":{},\"failed\":{},\"complete_pass_claim\":false}}\n",expected.len(),passed.len(),ignored.len(),failed.len(),finished.len(),strings(&pending),strings(&failed));
    fs::write(dir.join("named-test-metadata.json"), &output)?;
    print!("{output}");
    Ok(())
}
