//! Native rendering of qualified component records, without recomputing a solve.
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};
fn comma(x: u128) -> String {
    let s = x.to_string();
    let mut o = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            o.push(',');
        }
        o.push(c);
    }
    o
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: REPORT PACKET NEW_REPORT_DIRECTORY".into());
    }
    let packet = Path::new(&a[0]);
    let summary: Value = serde_json::from_slice(&fs::read(packet.join("summary.json"))?)?;
    let deletion: Value = serde_json::from_slice(&fs::read(packet.join("deletion.json"))?)?;
    if summary["schema"] != "cover.batched-sparse-kernel.qualified/v1"
        || summary["source_revision"] != "effca7bbb646a3642e1c278247546646c83b4dbe"
        || summary["batched_scalar_nonwall_equality"] != true
        || summary["complete_cost"] != false
        || deletion["delete_http"] != 204
        || deletion["independent_get_http"] != 404
    {
        return Err("registered qualified/custody record".into());
    }
    let mut rows = summary["rows"].as_array().ok_or("rows")?.clone();
    if rows.len() != 8 {
        return Err("eight cells".into());
    }
    rows.sort_by_key(|r| {
        (
            usize::from(r["corpus"] != "full-smooth"),
            [
                "counted-rref",
                "library-wiedemann",
                "randomized-lanczos",
                "batched-lanczos",
            ]
            .iter()
            .position(|m| r["checked"]["method"] == *m)
            .unwrap_or(99),
        )
    });
    let mut body = String::new();
    let mut table=String::from("| Corpus | Solver | Observed incomplete primitive subtotal | Inner wall (ms) |\n|---|---|---:|---:|\n");
    let colors = ["#2e5d90", "#538651", "#bd6530", "#8054a2"];
    for (i, r) in rows.iter().enumerate() {
        let c = &r["checked"];
        if c["verified_kernel"] != true || c["rank"] != 508 {
            return Err("actual rank/kernel gate".into());
        }
        let cost = c["observed_incomplete_subtotal"]
            .as_str()
            .ok_or("cost")?
            .parse::<u128>()?;
        let ns = c["inner_wall_ns"].as_str().ok_or("wall")?.parse::<u128>()?;
        let method = c["method"].as_str().ok_or("method")?;
        let corpus = r["corpus"].as_str().ok_or("corpus")?;
        let y = 151 + i * 49;
        body.push_str(&format!(r##"<rect x="25" y="{}" width="1050" height="41" rx="4" fill="#f1f4f8"/><text x="38" y="{y}" font-size="15">{corpus}</text><text x="250" y="{y}" font-size="17">{method}</text><text x="485" y="{y}" font-size="17">{}</text><rect x="690" y="{}" width="{:.2}" height="19" fill="{}"/><text x="955" y="{y}" font-size="18">{:.3}</text>"##,y-26,comma(cost),y-17,cost as f64/700_000_000_000f64*230f64,colors[i%4],ns as f64/1e6));
        table.push_str(&format!(
            "| {corpus} | {method} | {} | {:.3} |\n",
            comma(cost),
            ns as f64 / 1e6
        ));
    }
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="750" viewBox="0 0 1100 750"><rect width="1100" height="750" fill="white"/><g font-family="sans-serif" fill="#172b4d"><text x="25" y="40" font-size="28" font-weight="700">Batched IC kernels45 — identical work, lower elapsed time</text><text x="25" y="74" font-size="17">Public Rust sparse API | two exposed 509column matrices | four cold processes | exact non-wall equality</text><text x="38" y="111" font-size="17">Corpus</text><text x="250" y="111" font-size="17">Solver</text><text x="485" y="111" font-size="17">Observed incomplete subtotal</text><text x="955" y="111" font-size="17">Inner ms</text>{body}<rect x="25" y="568" width="1050" height="76" rx="5" fill="#fff3e2"/><text x="40" y="595" font-size="18">Batching reduces Lanczos elapsed time in this trial; counted RREF remains faster on both matrices.</text><text x="40" y="622" font-size="17">Every non-wall worker field matches scalar Lanczos exactly. Complete costs and ratios remain null.</text><text x="25" y="678" font-size="16">All eight kernels / Lanczos traces replay; iterative methods succeed on first attempt. Data-informed follow-up.</text><text x="25" y="705" font-size="15">Collected before owned-pod DELETE204 / GET404 at 2026-11-17T02:53:31Z. No owned paid runner remains.</text><text x="25" y="730" font-size="15">Source effca7bbb646 | seed443 | one cold round, zero warmups/retries/exclusions | unsigned producer evidence</text></g></svg>"##
    );
    let out = Path::new(&a[1]);
    fs::create_dir(out)?;
    fs::write(out.join("overview.svg"), svg)?;
    fs::write(out.join("table.md"), table)?;
    fs::write(out.join("report.typ"),"#set page(paper: \"a4\", flipped: true, margin: 10mm)\n#set text(size: 8pt)\n#image(\"overview.svg\", width: 100%, height: 170mm, fit: \"contain\")\nRecorded homogeneous component only. Excludes relation collection/filtering, input admission and independent verification. Native integer/setup/control coverage is incomplete. The independent software verifier supplies mathematical and partial-counter replay; it is not a signed execution proof.\n")?;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.join("report-metadata.json"))?;
    serde_json::to_writer_pretty(
        &mut f,
        &json!({"schema":"cover.batched-sparse-kernel.report/v1","summary":summary,"deletion":deletion,"complete_cost":false,"complete_total":null,"ratios":null,"interpretation":"Batching reduces Lanczos single elapsed time with identical non-wall outputs; RREF still faster; counting/setup coverage differs; no complete-pipeline composition"}),
    )?;
    f.write_all(b"\n")?;
    Ok(())
}
