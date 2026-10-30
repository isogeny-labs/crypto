//! Explicit mixed-data lineage; never converts preparation into scientific approval.
use super::*;
pub(super) const KIND: &str = "full1260-disjoint-p739/v1";
pub(super) fn selected(p: &V) -> bool {
    p["extensions"]["analysis_scope"]["kind"] == KIND
}
pub(super) fn admit_protocol(p: &V) -> Result<()> {
    check(
        p["extensions"]["analysis_scope"]
            == pencil_accounting_price_draft::disjoint::full1260_disjoint_p739_scope(),
        "exact disjoint scope required",
    )?;
    check(
        p["sample_size"]["instances"] == 1260,
        "disjoint denominator must be 1260",
    )?;
    check(
        p["analysis_plan"]["frozen_before_data"] == false,
        "mixed-data continuation must not claim frozen before all data",
    )?;
    let declaration = &p["extensions"]["disjoint_enrollment"];
    check(
        declaration["known_prior_gate_data"] == true
            && declaration["freeze_scope"] == "prospective_continuation_only",
        "explicit known-data and prospective freeze scope required",
    )?;
    let amendment_id = s(&declaration["known_data_amendment_id"])?;
    check(
        !amendment_id.trim().is_empty(),
        "known-data amendment ID absent",
    )?;
    let mut found = false;
    for amendment in arr(&p["amendments"])? {
        if amendment["id"] == amendment_id {
            check(
                !found && amendment["filed_before_affected_data"] == false,
                "known-data amendment must be unique and truthfully post-data",
            )?;
            found = true;
        } else {
            check(
                amendment["filed_before_affected_data"] == true,
                "unrelated post-data amendment is not admitted by enrollment",
            )?;
        }
    }
    check(found, "known-data amendment missing")?;
    check(
        !p["extensions"]["analysis_provenance_erratum"].is_object(),
        "disjoint scientific enrollment cannot use metadata erratum admission",
    )?;
    Ok(())
}
pub(super) struct Context {
    pub lineage: V,
    pub protocols: BTreeMap<String, V>,
}
fn attach_nested(v: &V, files: &mut BTreeMap<PathBuf, V>) -> Result<()> {
    if v["path"].is_string() && v["sha256"].is_string() {
        attach_bound(files, v, "receipt")?;
    } else if let Some(object) = v.as_object() {
        for child in object.values() {
            attach_nested(child, files)?;
        }
    } else if let Some(array) = v.as_array() {
        for child in array {
            attach_nested(child, files)?;
        }
    }
    Ok(())
}
impl Context {
    pub fn history(&self, files: &mut BTreeMap<PathBuf, V>, binary: &V) -> Result<V> {
        let mut records = Vec::new();
        for entry in arr(&self.lineage["plans"])? {
            if entry["execution_history"].is_null() {
                check(entry["records"].is_null(), "observed plan requires history")?;
                continue;
            }
            let h = load(&attach_bound(
                files,
                &entry["execution_history"],
                "receipt",
            )?)?;
            check(
                h["schema"] == "pencil-execution-history/v1"
                    && h["protocol_sha256"] == entry["data_protocol"]["protocol_sha256"]
                    && h["binary_sha256"] == *binary,
                "per-plan history identity mismatch",
            )?;
            for ev in arr(&h["records"])? {
                check(
                    ev["session_id"] == entry["session_id"],
                    "history session differs from lineage",
                )?;
                records.push(ev.clone());
            }
        }
        Ok(json!({"records":records}))
    }
    pub fn record_protocol(&self, session: &V) -> Result<&V> {
        let entry = arr(&self.lineage["plans"])?
            .iter()
            .find(|e| !session.is_null() && e["session_id"] == *session)
            .ok_or("history session absent from lineage")?;
        self.protocols
            .get(s(&entry["name"])?)
            .ok_or_else(|| "plan protocol absent".into())
    }
}
pub(super) fn load_context(p: &V, m: &V, files: &mut BTreeMap<PathBuf, V>) -> Result<Context> {
    check(
        selected(p) && m["analysis_scope"] == p["extensions"]["analysis_scope"],
        "disjoint scope differs",
    )?;
    let lineage = pencil_accounting_price_draft::aggregate::validate_disjoint_manifest(m)?;
    check(
        p["extensions"]["disjoint_enrollment"]["certificate"] == m["enrollment"]["certificate"],
        "continuation must pin exact enrollment certificate",
    )?;
    for key in ["request", "certificate", "protocol_lineage"] {
        attach_bound(files, &m["enrollment"][key], "receipt")?;
    }
    for key in ["binary", "source"] {
        attach_bound(files, &m["enrollment"]["verifier"][key], "analysis")?;
    }
    let request = load(&bound(&m["enrollment"]["request"])?)?;
    attach_nested(&request, files)?;
    let certificate = load(&bound(&m["enrollment"]["certificate"])?)?;
    let output = Path::new(s(&m["enrollment"]["output_directory"])?);
    for (section, name, digest) in [
        (
            "mapping",
            "workload_mapping_file",
            "workload_mapping_sha256",
        ),
        ("mapping", "slot_mapping_file", "slot_mapping_sha256"),
        ("derived_remainder", "spec_name", "spec_sha256"),
        ("derived_remainder", "plan_name", "plan_sha256"),
    ] {
        attach_bound(
            files,
            &json!({"path":output.join(s(&certificate[section][name])?),"sha256":certificate[section][digest]}),
            "receipt",
        )?;
    }
    let continuation = load(&attach_bound(
        files,
        &lineage["continuation_protocol"]["file"],
        "protocol",
    )?)?;
    check(continuation == *p, "lineage continuation protocol differs")?;
    let known = &lineage["known_data_declaration"];
    check(
        known["existing_gate_data_known_at_design"] == true,
        "known gate data declaration required",
    )?;
    for key in ["gate_observation_receipt", "scientific_reuse_admission"] {
        attach_bound(files, &known[key], "receipt")?;
    }
    let reuse = load(&bound(&known["scientific_reuse_admission"])?)?;
    if reuse["independent_review"].is_object() {
        attach_bound(files, &reuse["independent_review"], "report")?;
    }
    let mut protocols = BTreeMap::new();
    for entry in arr(&lineage["plans"])? {
        let name = s(&entry["name"])?;
        let review = &entry["coverage_review"];
        for b in review["source_files"].as_array().into_iter().flatten() {
            attach_bound(files, b, "other")?;
        }
        if review["independent_report"].is_object() {
            attach_bound(files, &review["independent_report"], "report")?;
        }
        let protocol = load(&attach_bound(
            files,
            &entry["data_protocol"]["file"],
            "protocol",
        )?)?;
        check(
            protocol_hash(&protocol)? == s(&entry["data_protocol"]["protocol_sha256"])?,
            "data protocol hash differs",
        )?;
        if entry["origin"] == "prior_gate" {
            basic_protocol_admission(&protocol, false)?;
        } else {
            check(
                entry["origin"] == "prospective_continuation" && protocol == *p,
                "unexpected prospective protocol",
            )?;
        }
        check(
            protocols.insert(name.to_owned(), protocol).is_none(),
            "duplicate lineage plan",
        )?;
    }
    Ok(Context { lineage, protocols })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn draft() -> V {
        json!({"sample_size":{"instances":1260},"analysis_plan":{"frozen_before_data":false},"extensions":{"analysis_scope":pencil_accounting_price_draft::disjoint::full1260_disjoint_p739_scope(),"disjoint_enrollment":{"known_prior_gate_data":true,"freeze_scope":"prospective_continuation_only","known_data_amendment_id":"known-gate"}},"amendments":[{"id":"known-gate","filed_before_affected_data":false}]})
    }
    #[test]
    fn explicit_known_data_scope_is_not_blanket_predata() {
        admit_protocol(&draft()).unwrap();
        let mut p = draft();
        p["analysis_plan"]["frozen_before_data"] = json!(true);
        assert!(admit_protocol(&p).is_err());
    }
    #[test]
    fn unrelated_post_data_changes_rejected() {
        let mut p = draft();
        p["amendments"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":"other","filed_before_affected_data":false}));
        assert!(admit_protocol(&p).is_err());
    }
    #[test]
    fn missing_or_duplicate_known_data_amendment_rejected() {
        let mut p = draft();
        p["amendments"] = json!([]);
        assert!(admit_protocol(&p).is_err());
        let mut p = draft();
        let a = p["amendments"][0].clone();
        p["amendments"].as_array_mut().unwrap().push(a);
        assert!(admit_protocol(&p).is_err());
    }
    #[test]
    fn historical_metadata_exception_cannot_admit_scientific_reuse() {
        let mut p = draft();
        p["extensions"]["analysis_provenance_erratum"] = json!({});
        assert!(admit_protocol(&p).is_err());
    }
    #[test]
    fn unknown_data_declaration_rejected() {
        let mut p = draft();
        p["extensions"]["disjoint_enrollment"]["known_prior_gate_data"] = json!(false);
        assert!(admit_protocol(&p).is_err());
    }
    #[test]
    fn histories_keep_gate_identity_and_future_absence() {
        let d = tempfile::tempdir().unwrap();
        let path = d.path().join("history.json");
        let h = json!({"schema":"pencil-execution-history/v1","protocol_sha256":"old-v5","binary_sha256":"instrument","records":[{"record_id":"original-record","session_id":"original-session","seq":42,"attempts":1,"resumed":0}]});
        fs::write(&path, canonical(&h).unwrap()).unwrap();
        let binding = json!({"path":path,"sha256":digest(&path).unwrap()});
        let mut context = Context {
            lineage: json!({"plans":[{"name":"gate","session_id":"original-session","records":{},"execution_history":binding,"data_protocol":{"protocol_sha256":"old-v5"}},{"name":"future","session_id":null,"records":null,"execution_history":null}]}),
            protocols: BTreeMap::new(),
        };
        let output = context
            .history(&mut BTreeMap::new(), &json!("instrument"))
            .unwrap();
        assert_eq!(output["records"], h["records"]);
        context.lineage["plans"][0]["data_protocol"]["protocol_sha256"] = json!("new-continuation");
        assert!(context
            .history(&mut BTreeMap::new(), &json!("instrument"))
            .is_err());
        context.lineage["plans"][0]["data_protocol"]["protocol_sha256"] = json!("old-v5");
        context.lineage["plans"][0]["session_id"] = json!("relabelled-session");
        assert!(context
            .history(&mut BTreeMap::new(), &json!("instrument"))
            .is_err());
    }
}
