use cubic_native_measure::{
    command::{self, Config},
    isolation::CpuRequest,
};
use std::path::PathBuf;
use std::time::Duration;

fn main() {
    let result = (|| -> Result<(), String> {
        let args: Vec<_> = std::env::args().skip(1).collect();
        let split = args.iter().position(|s| s == "--").ok_or(
            "usage: cubic-native-measure OUTDIR WORKDIR LIBRARY CPUS TIMEOUT_SECONDS [--diagnostic] [--input PUBLIC_FILE]... -- ABSOLUTE_EXE ARGS..."
        )?;
        if split < 5 {
            return Err("five parameters required".into());
        }
        let mut diagnostic = false;
        let mut public_inputs = Vec::new();
        let mut i = 5;
        while i < split {
            match args[i].as_str() {
                "--diagnostic" if !diagnostic => {
                    diagnostic = true;
                    i += 1;
                }
                "--input" if i + 1 < split => {
                    public_inputs.push(PathBuf::from(&args[i + 1]));
                    i += 2;
                }
                _ => return Err("unknown or incomplete measurement option".into()),
            }
        }
        let record = command::run(&Config {
            output: PathBuf::from(&args[0]),
            directory: PathBuf::from(&args[1]),
            library: PathBuf::from(&args[2]),
            public_inputs,
            cpus: CpuRequest::parse(&args[3])?,
            timeout: Duration::from_secs(args[4].parse().map_err(|_| "invalid timeout")?),
            diagnostic,
            argv: args[split + 1..].to_vec(),
        })?;
        if record["outcome"]["completed"] != true {
            return Err("process failed; raw outputs and record retained".into());
        }
        if !diagnostic && record["isolation_wall_eligible"] != true {
            return Err("process did not earn L2; record retained".into());
        }
        Ok(())
    })();
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
