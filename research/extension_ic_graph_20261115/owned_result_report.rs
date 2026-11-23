//! Native rendering of the original copied-qualifier output; no repricing.
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
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: REPORT PACKET NEW_REPORT_DIR".into());
    }
    let p = Path::new(&args[0]);
    let s: Value = serde_json::from_slice(&fs::read(p.join("summary.json"))?)?;
    let repetition: Value = serde_json::from_slice(&fs::read(p.join("repetition.json"))?)?;
    if s["schema"] != "cover.anchored-descent.qualified/v1"
        || s["source_revision"] != "f2691d803a97ffb20d0f218b00a469ffba6c4a47"
        || s["complete_cost"] != false
        || repetition["old_capture_still_rejected"] != true
    {
        return Err("qualified component registration".into());
    }
    let workers = s["workers"].as_array().ok_or("workers")?;
    if workers.len() != 3 {
        return Err("worker count".into());
    }
    let mut body = String::new();
    let mut prices = Vec::new();
    for (i, w) in workers.iter().enumerate() {
        let mode = w["mode"].as_str().ok_or("mode")?;
        if !["original-nagao", "full-only", "anchored"].contains(&mode)
            || w["rank"] != 2
            || w["residuals"] != 1506
            || w["accepted_rows"] != 2
            || w["elliptic_target_replayed"] != true
        {
            return Err("endpoint".into());
        }
        let price = w["observed_incomplete_primitive_subtotal"]
            .as_str()
            .ok_or("subtotal")?
            .parse::<u128>()?;
        prices.push(price);
        let y = 166 + i * 58;
        body.push_str(&format!(r##"<rect x="30" y="{}" width="940" height="49" rx="5" fill="#f2f5f8"/><text x="45" y="{y}">{mode}</text><text x="280" y="{y}">1506</text><text x="410" y="{y}">{}</text><text x="545" y="{y}">{}</text><text x="680" y="{y}" font-size="18">{}</text>"##,y-30,w["partial_rows"],w["matched_partials"],comma(price)));
    }
    let excess = prices[2]
        .checked_sub(prices[1])
        .ok_or("registered result direction")?;
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="600" viewBox="0 0 1000 600"><rect width="1000" height="600" fill="white"/><g font-family="sans-serif" font-size="19" fill="#172b4d"><text x="30" y="40" font-size="29" font-weight="700">Owned descent43 — valid replay, no matched anchors</text><text x="30" y="74" font-size="17">Supplied homogeneous state | seed 401 | three cold arms | complete totals / ratios null</text><text x="45" y="118" font-weight="700">Worker</text><text x="265" y="118">Residuals</text><text x="405" y="118">Partials</text><text x="535" y="118">Matches</text><text x="675" y="118">Observed incomplete subtotal</text>{body}<rect x="30" y="340" width="940" height="75" rx="6" fill="#edf6ef"/><text x="48" y="368">Copied qualifier and geometry-repeat checker both exit 0; all worker foreign flags 0.</text><text x="48" y="393" font-size="17">All 1506common prefixes and every 42→43 geometric/affine endpoint match. Old costs stay rejected.</text><rect x="30" y="435" width="940" height="82" rx="6" fill="#fff4e5"/><text x="48" y="463">26 valid partials, zero matches among 11,206 retained classes; two full rows still needed.</text><text x="48" y="490">Anchored extra observed subtotal: {}. No descent gain on this workload.</text><text x="30" y="553" font-size="15">Source f2691d803a | one supplied-state component | original archive/local hashes retained</text><text x="30" y="579" font-size="15">Collected before owned-pod DELETE204 / GET404 at 2026-11-17T00:42:47Z. No owned paid runner remains.</text></g></svg>"##,
        comma(excess)
    );
    let out = Path::new(&args[1]);
    fs::create_dir(out)?;
    fs::write(out.join("overview.svg"), svg)?;
    fs::write(out.join("report.typ"),"#set page(paper: \"a4\", flipped: true, margin: 12mm)\n#set text(size: 9pt)\n#image(\"overview.svg\", width: 100%)\nUnsigned result/custody assertion PCC-20261117-4f2958. Component software replay only; native coverage remains incomplete. Do not compose these costs with prior cold-pipeline trials.\n")?;
    let metadata = json!({"schema":"cover.owned-descent.report/v1","source_revision":s["source_revision"],"workers":workers,"anchored_extra_observed_subtotal":excess.to_string(),"rho_comparison_in_this_component":false,"complete_total":null,"ratios":null,"old42costs_admitted":false,"scope":"Fixed supplied-state descent workload; no matched anchors, no shorter residual prefix; no full-pipeline gain inferred."});
    let mut f = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(out.join("report-metadata.json"))?;
    serde_json::to_writer_pretty(&mut f, &metadata)?;
    f.write_all(b"\n")?;
    Ok(())
}
