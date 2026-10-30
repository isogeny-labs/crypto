//! Certificates: stored volcano-level certificates
//! (docs/curves/level-certificates.json) and Jacobian / gluing certificates
//! (docs/curves/jacobian-certificates.json). Payloads are kept verbatim; a
//! certificate's status is the producer's, mapped onto ec.fact_status.

use super::{bulk, Ctx, Report};
use crate::math::{canonical_json, sha256_hex};
use anyhow::Result;
use serde_json::{json, Value};

const CERT_SQL: &str = "
INSERT INTO ec.certificate (certificate_uid, kind, subject_table, subject_id, payload, required_hypotheses, status, provenance_id)
SELECT r.uid, r.kind, r.subject_table, c.id, r.payload, r.hypotheses, r.status::ec.fact_status, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(uid text, kind text, subject_table text, key text, payload jsonb,
     hypotheses text[], status text)
LEFT JOIN ec.curve c ON c.icv1_slug = r.key
WHERE r.key IS NULL OR c.id IS NOT NULL
ON CONFLICT DO NOTHING";

pub async fn import(ctx: &Ctx<'_>) -> Result<Vec<Report>> {
    let mut reports = vec![];

    let input = ctx.read("docs/curves/level-certificates.json")?;
    let mut report = Report::new(&input);
    let src = ctx.source(&input.rel_path, &input.sha256, "Stored volcano-level certificates").await?;
    let prov = ctx
        .provenance(src, "points found offline (search.gp), verified by curve_traits verify_level_certificate", None)
        .await?;
    let rows: Vec<Value> = input.doc["certificates"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|c| {
            let uid =
                format!("urn:ecdb-level-certificate:1:sha256:{}", sha256_hex(&canonical_json(c).unwrap_or_default()));
            json!({ "uid": uid, "kind": "volcano_level", "subject_table": "curve", "key": c["slug"], "payload": c,
                    "hypotheses": null, "status": "proved" })
        })
        .collect();
    report.count("certificates", bulk(ctx.client, CERT_SQL, &rows, prov).await?);
    reports.push(report);

    let input = ctx.read("docs/curves/jacobian-certificates.json")?;
    let mut report = Report::new(&input);
    let src = ctx.source(&input.rel_path, &input.sha256, "Jacobian certificates").await?;
    let prov = ctx.provenance(src, "jacobian-certificate checker (docs/curves/JACOBIAN_CERTIFICATES.md)", None).await?;
    let rows: Vec<Value> = input.doc["records"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|c| {
            // conditional_existence rests on the listed, unverified hypotheses.
            let status = match c["status"].as_str() {
                Some("verified") | Some("proved") => "proved",
                Some("conditional_existence") => "conjectural",
                _ => "unknown",
            };
            json!({ "uid": c["certificate_uid"], "kind": "jacobian_gluing", "subject_table": "none", "key": null,
                    "payload": c, "hypotheses": c["required_hypotheses"], "status": status })
        })
        .collect();
    report.count("certificates", bulk(ctx.client, CERT_SQL, &rows, prov).await?);
    reports.push(report);
    Ok(reports)
}
