mod evidence_tool;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("RR5 development qualifier: use the frozen native test protocol.");
        println!("Metadata only: export ROOT STUDY_REL OUT_REL");
        println!("Unsigned registration: register ROOT OBS_REL PRODUCER_COMMIT UTC_TIMESTAMP");
        println!("No performance campaign is performed by this entry point.");
        return;
    }
    match evidence_tool::run(&args) {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).unwrap()),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}
