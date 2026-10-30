//! Native additive fixture registration. Preserve all existing registry rows:
//! sparse historical source material is not evidence that an identity vanished.
use super::*;
use serde::Serialize;

fn pretty(value: &Value, indent: &'static [u8]) -> String {
    let mut bytes = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(
        &mut bytes,
        serde_json::ser::PrettyFormatter::with_indent(indent),
    );
    value.serialize(&mut serializer).unwrap();
    String::from_utf8(bytes).unwrap()
}

fn sort_key(row: &Value) -> (u8, u64, num_bigint::BigUint, u64, String) {
    let family = match row["family"].as_str().unwrap_or("") {
        "koblitz" => 0,
        "subfield" => 1,
        "binary" => 2,
        "prime" => 3,
        _ => 4,
    };
    let n = integer(&row["params"]["n"])
        .or_else(|_| integer(&row["params"]["m"]))
        .unwrap_or(0);
    let p = row["params"]["p"]
        .as_str()
        .unwrap_or("0")
        .parse()
        .unwrap_or_default();
    let a = if family == 0 {
        integer(&row["params"]["a"]).unwrap_or(0)
    } else {
        0
    };
    (family, n, p, a, row["slug"].as_str().unwrap().to_string())
}

pub(super) fn register(study: &Path, repo: &Path) -> Result<(), String> {
    let path = repo.join("docs/curves/registry.json");
    let original = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let raw: BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(&original).map_err(|e| e.to_string())?;
    let elements: Vec<Box<serde_json::value::RawValue>> =
        serde_json::from_str(raw["curves"].get()).map_err(|e| e.to_string())?;
    let mut rows: Vec<(Value, String)> = elements
        .iter()
        .map(|v| (serde_json::from_str(v.get()).unwrap(), v.get().to_string()))
        .collect();
    let fixtures = read(&study.join("fixtures.json"))?;
    for source in fixtures["curves"].as_array().unwrap() {
        let p = integer(&source["p"])?;
        let a = integer(&source["a"])?;
        let b = integer(&source["b"])?;
        let order = integer(&source["group_order"])?;
        let r = integer(&source["subgroup_order"])?;
        let h = integer(&source["cofactor"])?;
        let gx = integer(&source["generator"]["x"])?;
        let gy = integer(&source["generator"]["y"])?;
        let instance = prime(&json!({"kind":"prime_explicit", "p":p,"a":a,"b":b,
            "group_order":order,"r":r,"gx":gx,"gy":gy}))?;
        let id = instance.curve_id();
        let field = json!({"characteristic":p,"degree":1,"representation":"prime",
            "element_encoding":"hex integer modulo the characteristic"});
        let curve = json!({"model":"short Weierstrass","coefficients":[0,0,0,a,b],
            "subgroup_order":r.to_string(),"cofactor":h,
            "generator":[format!("0x{gx:x}"),format!("0x{gy:x}")],"target_group":"prime-order subgroup"});
        let digest =
            sha256_hex(&serde_json::to_vec(&json!({"field":field,"curve":curve})).unwrap());
        let rep = json!({"ec1":format!("EC1P{}Cfph{}",64-p.leading_zeros(),&digest[..12]),
            "curve_uid":format!("urn:ec-record:1:sha256:{digest}"),
            "field_sha256":sha256_hex(&serde_json::to_vec(&field).unwrap()),
            "field":field,"curve":curve,"sources":["docs/curves/sources/generated.json"]});
        let index = match rows.iter().position(|(v, _)| v["slug"] == id.slug) {
            Some(index) => index,
            None => {
                let parts: Vec<_> = id.icv1.split(':').collect();
                let trace: i64 = parts[2].parse().map_err(|_| "trace exceeds i64")?;
                let row = json!({"slug":id.slug,"icv1":id.icv1,"family":"prime",
                    "params":{"p":p.to_string(),"a":a.to_string(),"b":b.to_string(),
                        "generator_call":source["generator_call"]},
                    "model_json":id.model_json,"trace":trace,"order":order.to_string(),
                    "j":parts[4],"end":parts[5],"aliases":[id.slug],"standard_names":[],
                    "sources":["docs/curves/sources/generated.json"],"representations":[]});
                rows.push((row, String::new()));
                rows.len() - 1
            }
        };
        let reps = rows[index].0["representations"]
            .as_array_mut()
            .ok_or("missing representations")?;
        if !reps.iter().any(|v| v["curve_uid"] == rep["curve_uid"]) {
            reps.push(rep);
            reps.sort_by_key(|v| v["curve_uid"].as_str().unwrap().to_string());
            rows[index].1 = pretty(&rows[index].0, b" ").replace('\n', "\n  ");
        }
    }
    rows.sort_by_key(|(value, _)| sort_key(value));
    let array = format!(
        "[\n  {}\n ]",
        rows.iter()
            .map(|(_, raw)| raw.as_str())
            .collect::<Vec<_>>()
            .join(",\n  ")
    );
    fs::write(&path, original.replacen(raw["curves"].get(), &array, 1))
        .map_err(|e| e.to_string())?;
    let aliases_path = repo.join("src/cryptanalysis/curve_aliases.json");
    let alias_text = fs::read_to_string(&aliases_path).map_err(|e| e.to_string())?;
    let alias_raw: BTreeMap<String, Box<serde_json::value::RawValue>> =
        serde_json::from_str(&alias_text).map_err(|e| e.to_string())?;
    let mut aliases: Value = serde_json::from_str(alias_raw["aliases"].get()).unwrap();
    for (row, _) in &rows {
        for key in ["slug", "icv1"] {
            aliases[crypto_lib::cryptanalysis::curve_id::normalise(row[key].as_str().unwrap())] =
                row["slug"].clone();
        }
    }
    fs::write(
        aliases_path,
        alias_text.replacen(alias_raw["aliases"].get(), &pretty(&aliases, b""), 1),
    )
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserved_array_layout_round_trips_existing_rows_exactly() {
        let text = "{\n \"curves\": [\n  {\n   \"slug\": \"a\"\n  },\n  {\n   \"slug\": \"b\"\n  }\n ]\n}\n";
        let raw: BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(text).unwrap();
        let rows: Vec<Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw["curves"].get()).unwrap();
        let rebuilt = format!(
            "[\n  {}\n ]",
            rows.iter()
                .map(|r| r.get())
                .collect::<Vec<_>>()
                .join(",\n  ")
        );
        assert_eq!(raw["curves"].get(), rebuilt);
    }
}
