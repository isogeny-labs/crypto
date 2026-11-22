use ghs_prime_descent::{parse_request, Adapter, MAX_INPUT_BYTES};
use std::io::{Read, Write};
fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let input = args
        .next()
        .ok_or("usage: ghs-prime-descent INPUT.json OUTPUT.json")?;
    let output = args.next().ok_or("missing output path")?;
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let mut bytes = Vec::new();
    std::fs::File::open(&input)
        .map_err(|e| e.to_string())?
        .take(MAX_INPUT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    let req = parse_request(&bytes)?;
    let a = Adapter::new(req.q)?;
    let keys = req
        .points
        .iter()
        .map(|p| a.transfer(p).map(|c| c.key().0))
        .collect::<Result<Vec<_>, _>>()?;
    let body = serde_json::json!({"schema":"ghs-prime-descent-result/v1","q":a.q,"field":a.field_tag,"model":a.model_tag,"jacobian_keys":keys,"count":keys.len()});
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output)
        .map_err(|e| e.to_string())?;
    serde_json::to_writer_pretty(&mut f, &body).map_err(|e| e.to_string())?;
    f.write_all(b"\n").map_err(|e| e.to_string())?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(2);
    }
}
