//! Emit symbolic Appendix D.4 parameter-guard transcripts.

use hashed_ecdl_tensor::contraction::guards::{
    power_of_two_size_guards, scan_power_of_two_lambdas,
};
use hashed_ecdl_tensor::BigUint;
use std::env;
use std::io::{self, Write};

fn usage() -> &'static str {
    "usage: tensor-guards point LAMBDA | tensor-guards scan MIN_LOG2_LAMBDA MAX_LOG2_LAMBDA"
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().skip(1).collect();
    let value = match args.as_slice() {
        [command, lambda] if command == "point" => {
            let lambda = lambda
                .parse::<BigUint>()
                .map_err(|_| "LAMBDA must be a positive decimal integer")?;
            serde_json::to_value(power_of_two_size_guards(&lambda)?)?
        }
        [command, minimum, maximum] if command == "scan" => {
            let minimum = minimum
                .parse::<usize>()
                .map_err(|_| "MIN_LOG2_LAMBDA must be an integer")?;
            let maximum = maximum
                .parse::<usize>()
                .map_err(|_| "MAX_LOG2_LAMBDA must be an integer")?;
            serde_json::to_value(scan_power_of_two_lambdas(minimum, maximum)?)?
        }
        _ => return Err(usage().into()),
    };
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    serde_json::to_writer_pretty(&mut stdout, &value)?;
    writeln!(stdout)?;
    Ok(())
}
