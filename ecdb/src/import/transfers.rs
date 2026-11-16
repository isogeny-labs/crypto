//! docs/curves/jacobian-transfers.json: every DLP transfer route the
//! repository priced for each registered curve (GHS / Weil descent, covers,
//! base-change descent, extension-field index calculus), with the producer's
//! cost record and verdict. The cost model is stored once, as the provenance
//! parameters. A formula advantage is a lead, never a measured speedup.

use super::{bulk, s, Ctx, Report};
use anyhow::Result;
use serde_json::json;

const TRANSFER_SQL: &str = "
INSERT INTO ec.dlp_transfer (source_curve_id, source_subgroup_id, route_index, route_kind, label, target, target_genus,
       target_field_bits, morphism_id, evidence, transport, invariants, cost, log2_transfer_cost, log2_target_dlp_cost,
       log2_total_cost, log2_reference_cost, ratio_log2, verdict, notes, status, provenance_id)
SELECT c.id, sg.id, r.route_index, r.route_kind, r.label, r.target::ec.transfer_target, r.target_genus,
       r.target_field_bits, m.id, r.evidence, r.transport, r.invariants, r.cost, r.log2_transfer, r.log2_target,
       r.log2_total, r.reference_log2, r.ratio_log2, r.verdict, r.notes, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, subgroup_order numeric, route_index int, route_kind text, label text,
     target text, target_genus numeric, target_field_bits numeric, evidence text, transport text, invariants jsonb,
     cost jsonb, log2_transfer numeric, log2_target numeric, log2_total numeric, reference_log2 numeric,
     ratio_log2 numeric, verdict text, notes text[], status text)
JOIN ec.curve c ON c.icv1_slug = r.key
LEFT JOIN ec.abelian_variety av ON av.curve_id = c.id
LEFT JOIN LATERAL (SELECT s.id FROM ec.subgroup s WHERE s.av_id = av.id AND s.order_value = r.subgroup_order
                   ORDER BY s.id LIMIT 1) sg ON true
LEFT JOIN LATERAL (SELECT mm.id FROM ec.curve_morphism mm
                   WHERE mm.target_curve_id = c.id AND r.route_kind = 'same_field_cover'
                   ORDER BY mm.id LIMIT 1) m ON true
ON CONFLICT DO NOTHING";

fn status(evidence: &str) -> &'static str {
    match evidence {
        "exact_invariant" | "construction" => "proved",
        "heuristic" => "heuristic",
        "proposed" => "conjectural",
        _ => "unknown",
    }
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let input = ctx.read("docs/curves/jacobian-transfers.json")?;
    let mut report = Report::new(&input);
    let src = ctx.source(&input.rel_path, &input.sha256, "Jacobian / descent transfer catalogue").await?;
    let params = json!({ "schema": input.doc["schema"], "cost_model": input.doc["cost_model"],
                         "factor_budget": input.doc["factor_budget"] });
    let prov = ctx
        .provenance(src, "transfer routes and formula costs (docs/curves/JACOBIAN_TRANSFERS.md); status describes the route invariants, costs are model estimates", Some(&params))
        .await?;
    let mut rows = vec![];
    for c in input.doc["curves"].as_array().into_iter().flatten() {
        for (i, r) in c["routes"].as_array().into_iter().flatten().enumerate() {
            let object = r["destination"]["object"].as_str().unwrap_or_default();
            let target = if object.starts_with("Jac") { "jacobian" } else { "extension_field_curve" };
            let cost = &r["cost"];
            rows.push(json!({
                "key": c["slug"], "subgroup_order": s(&c["subgroup"]["order"]), "route_index": i + 1,
                "route_kind": r["kind"], "label": r["label"], "target": target,
                "target_genus": s(&r["destination"]["genus"]), "target_field_bits": r["destination"]["field_bits"],
                "evidence": r["evidence"], "transport": r["transport"], "invariants": r["invariants"], "cost": cost,
                "log2_transfer": cost["log2_transfer"], "log2_target": cost["destination"]["log2_cost"],
                "log2_total": cost["log2_total"], "reference_log2": c["reference_log2"], "ratio_log2": cost["ratio_log2"],
                "verdict": r["class"], "notes": r["notes"], "status": status(r["evidence"].as_str().unwrap_or_default()),
            }));
        }
    }
    report.count("transfers", bulk(ctx.client, TRANSFER_SQL, &rows, prov).await?);
    Ok(report)
}
