//! MCP over HTTP ("streamable HTTP" transport, JSON responses, stateless):
//! POST /mcp carries one JSON-RPC message. initialize, ping and tools/list
//! need no token; tools/call forwards the caller's Authorization header to
//! the REST route the tool maps to, so the API's own auth applies.

use axum::body::{to_bytes, Body, Bytes};
use axum::http::{header, HeaderMap, Method, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use serde_json::{json, Map, Value};
use tower::ServiceExt;

const PROTOCOL_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];
const MAX_TEXT: usize = 400_000;

const INSTRUCTIONS: &str = "ecdb: exact catalogue of elliptic curves, genus-2/3 cover curves, Jacobians, isogenies, \
covers and priced DLP transfers from the isogeny-labs repositories. Name curves by ICV1 slug, EC1 alias, curve_uid \
or standard name (secp256k1, P-256, sect163k1). Integers beyond 2^53 are decimal strings; field elements are \
strings in the field's encoding. Every derived value has a status (proved, probable, bounded, heuristic, \
conjectural, unknown, ...); unknown is null, never 0. Transfer costs are formula estimates, not measured speedups. \
Measured results (IC leaderboard, progress, boundary ledger, benchmarks, ecbench sessions and runs, sibling-repository \
datasets) keep their producer's unit and reference: ecdb_results, ecdb_ecbench, ecdb_curve_detail part=measurements. \
Start with ecdb_schema or ecdb_search_curves; use ecdb_sql for anything the other tools do not cover.";

fn tools() -> Value {
    let s = |props: Value, required: &[&str]| json!({ "type": "object", "properties": props, "required": required });
    let curve_ref = json!({ "type": "string", "description": "ICV1 slug or string, EC1 alias, curve_uid, standard name (secp256k1, P-256), OID, alias or numeric id" });
    json!([
        {
            "name": "ecdb_schema",
            "description": "Tables and views of schema ec with comments. With `table`, that relation's columns, types, comments, foreign keys and CHECK constraints. Read before writing SQL.",
            "inputSchema": s(json!({ "table": { "type": "string", "description": "e.g. curve, v_curve_summary, isogeny, dlp_transfer" } }), &[]),
        },
        {
            "name": "ecdb_search_curves",
            "description": "Filter and sort curves (view v_curve_summary). Returns total and a page of summaries: slug, standard names, field, genus, model, j, trace, group order, CM discriminant, conductor, class number, End(E), subgroup bits, cofactor, embedding degree, twist security, counts of isogenies and covers.",
            "inputSchema": s(json!({
                "field_kind": { "type": "string", "enum": ["prime", "binary", "prime_power"] },
                "characteristic": { "type": "string" }, "field_degree": { "type": "integer" },
                "field_bits_min": { "type": "integer" }, "field_bits_max": { "type": "integer" },
                "genus": { "type": "integer" },
                "model": { "type": "string", "description": "short_weierstrass, binary_weierstrass, montgomery, edwards, twisted_edwards, hyperelliptic" },
                "family": { "type": "string", "description": "koblitz, binary, prime, extension, subfield, isogeny-walk, cover" },
                "trace": { "type": "string" }, "j_invariant": { "type": "string" },
                "cm_discriminant": { "type": "string" }, "class_number": { "type": "string" }, "class_number_max": { "type": "string" },
                "conductor_bits_max": { "type": "integer" }, "end_discriminant": { "type": "string" }, "end_is_maximal": { "type": "boolean" },
                "subgroup_bits_min": { "type": "integer" }, "subgroup_bits_max": { "type": "integer" }, "cofactor": { "type": "string" },
                "embedding_degree_max": { "type": "string" }, "twist_prime_bits_min": { "type": "integer" },
                "is_anomalous": { "type": "boolean" }, "is_supersingular": { "type": "boolean" }, "definition_degree": { "type": "integer" },
                "standard": { "type": "string", "description": "substring of a standard name" },
                "name": { "type": "string", "description": "substring of any alias or standard name" },
                "isogeny_class": { "type": "string" }, "has_isogenies": { "type": "boolean" }, "has_covers": { "type": "boolean" },
                "sort": { "type": "string", "enum": ["id", "icv1_slug", "field_bits", "group_order_bits", "subgroup_bits", "trace", "class_number", "conductor_bits", "embedding_degree", "cm_discriminant", "isogeny_count", "cover_count"] },
                "order": { "type": "string", "enum": ["asc", "desc"] },
                "limit": { "type": "integer", "minimum": 1, "maximum": 1000 }, "offset": { "type": "integer", "minimum": 0 }
            }), &[]),
        },
        {
            "name": "ecdb_get_curve",
            "description": "Everything recorded about one curve: field, model, invariants, aliases, standards, EC1 representations, Jacobian, isogeny class, Frobenius, per-prime data, End(E) and volcano levels, automorphisms, endomorphisms, GLV lattices, tau-adic data, descent, twists, group structure, subgroups with pairing and attack-surface data, relations, isogenies, covers, transfers, division polynomials, certificates, evidence links.",
            "inputSchema": s(json!({ "ref": curve_ref }), &["ref"]),
        },
        {
            "name": "ecdb_curve_detail",
            "description": "One part of a curve in full: isogenies (kernel polynomials, codomains, certificates), graph (recorded isogeny graph; depth 0-8, optional ell), transfers (priced DLP routes), covers (cover curves and maps), traits (verbatim producer records), relations (twists, isomorphisms, model changes), endomorphisms (with GLV lattices), division-polynomials (coefficients).",
            "inputSchema": s(json!({
                "ref": curve_ref,
                "part": { "type": "string", "enum": ["isogenies", "graph", "transfers", "covers", "traits", "relations", "endomorphisms", "division-polynomials", "measurements"], "description": "measurements: the curve's IC leaderboard rows with recipes, ecbench arm summaries, sibling-dataset measurements and sibling-repository records" },
                "depth": { "type": "integer", "minimum": 0, "maximum": 8 }, "ell": { "type": "string" }
            }), &["ref", "part"]),
        },
        {
            "name": "ecdb_isogeny_class",
            "description": "An isogeny class over F_q (any dimension) by LMFDB-style label g.q.a1_..._ag or id: Weil polynomial, point counts, Frobenius data, per-prime data, member curves.",
            "inputSchema": s(json!({ "label": { "type": "string" } }), &["label"]),
        },
        {
            "name": "ecdb_transfers",
            "description": "DLP transfer routes across all curves, highest formula ratio first. ratio_log2 is a formula estimate under the stored cost model, not a measured speedup.",
            "inputSchema": s(json!({
                "route_kind": { "type": "string", "description": "ghs_weil_descent, same_field_cover, base_change_descent, cover_descent, extension_index_calculus, ghs_isogeny_extension" },
                "verdict": { "type": "string", "description": "formula_advantage, formula_marginal, boundary, not_carried" },
                "min_ratio_log2": { "type": "string" }, "status": { "type": "string" },
                "limit": { "type": "integer" }, "offset": { "type": "integer" }
            }), &[]),
        },
        {
            "name": "ecdb_polynomials",
            "description": "Stored polynomials: classical modular Phi_l (l <= 19; set coefficients=true for the terms), Hilbert class polynomials (class number one), Semaev summation polynomials S_2, S_3.",
            "inputSchema": s(json!({
                "kind": { "type": "string", "enum": ["modular", "class", "summation"] },
                "level": { "type": "integer" }, "coefficients": { "type": "boolean" }
            }), &["kind"]),
        },
        {
            "name": "ecdb_results",
            "description": "Measured results of the crypto repository as frozen there. kind=leaderboard: IC leaderboard rows (best recipe's cold cost over matched rho, per family A/B/C; filters family, regime, validation; recipes=true adds every recipe). kind=progress: the progress timeline (filter series). kind=boundary: the boundary ledger (filters regime, stage). kind=benchmarks: the benchmarks/ index with VALIDATION.json (filters listed=admitted|archive|unlisted, q, limit, offset). kind=claims: IC tool-program claims, IC online against matched rho per target (filters rule_version, curve, candidate_id, validation_status; summary=true for per-candidate geometric means). online_speedup excludes the IC set-up; cold_ratio_ic_over_rho includes it and is the figure to quote. kind=designs: factor-base design grids (n83). Ratios compare only within one family or series.",
            "inputSchema": s(json!({
                "kind": { "type": "string", "enum": ["leaderboard", "progress", "boundary", "benchmarks", "claims", "designs"] },
                "rule_version": { "type": "string" }, "curve": { "type": "string" }, "candidate_id": { "type": "string" },
                "validation_status": { "type": "string" }, "summary": { "type": "boolean" },
                "family": { "type": "string", "enum": ["A", "B", "C"] }, "regime": { "type": "string" },
                "validation": { "type": "string" }, "recipes": { "type": "boolean" },
                "series": { "type": "string" }, "stage": { "type": "string" },
                "listed": { "type": "string", "enum": ["admitted", "archive", "unlisted"] }, "q": { "type": "string" },
                "limit": { "type": "integer" }, "offset": { "type": "integer" }
            }), &["kind"]),
        },
        {
            "name": "ecdb_ecbench",
            "description": "ecbench sessions (sealed measurement runs of rho, BSGS, kangaroo and IC methods). Without session_id: a page of sessions (filters method, curve slug, label, path). With session_id: the session, per-arm summaries (runs, verified, mean S with standard error, mean ratio to the floor) and paired comparisons. S is GAE per sqrt(r); lower_bound means unpriced native work.",
            "inputSchema": s(json!({
                "session_id": { "type": "string" }, "method": { "type": "string" }, "curve": { "type": "string" },
                "label": { "type": "string" }, "path": { "type": "string" },
                "limit": { "type": "integer" }, "offset": { "type": "integer" }
            }), &[]),
        },
        {
            "name": "ecdb_factor_bases",
            "description": "Factor bases of the index-calculus work. Ids: FB1h... (ecbench point set), FBM1h... (measurement configuration), FBS1h... (ecdb id for a base known only by construction), or a cryptanalysis archive digest; plug-in strings, PS1 labels and cited digests resolve too. Without key: a page of bases with sizes, result counts, solved counts and best solved S (filters curve, family, scheme, min_columns, max_columns, solved; sort results|best_s|columns|key). With key: the base with aliases, FBM1 configurations and results by source; results=true pages its results (ecbench runs, large-prime cells, SECG small-x yields; filters source, fbm_id, solved); points=true pages its stored points.",
            "inputSchema": s(json!({
                "key": { "type": "string" }, "curve": { "type": "string" }, "family": { "type": "string" },
                "scheme": { "type": "string", "enum": ["fb1", "fbarchive", "spec", "pointset"] }, "design": { "type": "string" },
                "min_columns": { "type": "integer" }, "max_columns": { "type": "integer" }, "solved": { "type": "boolean" },
                "sort": { "type": "string", "enum": ["results", "best_s", "columns", "key"] },
                "results": { "type": "boolean" }, "points": { "type": "boolean" }, "source": { "type": "string" },
                "fbm_id": { "type": "string" }, "limit": { "type": "integer" }, "offset": { "type": "integer" }
            }), &[]),
        },
        {
            "name": "ecdb_sql",
            "description": "One read-only SQL query over schema ec (search_path ec, public), as role ecdb_reader, with a statement timeout. Rows come back as arrays in column order. Useful functions: resolve_curve(text), curve_document(bigint), isogeny_neighbourhood(bigint, int, numeric).",
            "inputSchema": s(json!({ "sql": { "type": "string" }, "limit": { "type": "integer" } }), &["sql"]),
        },
        {
            "name": "ecdb_record_evidence",
            "description": "Link a curve to a research artifact (needs the write token as bearer). The row is a claim with agent provenance.",
            "inputSchema": s(json!({
                "curve": curve_ref,
                "kind": { "type": "string", "description": "ecbench_session, isogeny_walk, ic_run, rho_run, report, paper" },
                "repository": { "type": "string" }, "path": { "type": "string" }, "sha256": { "type": "string" },
                "summary": { "type": "object" }, "validation_status": { "type": "string" },
                "agent": { "type": "object", "properties": { "name": { "type": "string" }, "method": { "type": "string" }, "notes": { "type": "string" } }, "required": ["name"] }
            }), &["curve", "kind", "agent"]),
        }
    ])
}

fn enc(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

fn query(args: &Map<String, Value>, skip: &[&str]) -> String {
    let parts: Vec<String> = args
        .iter()
        .filter(|(k, v)| !skip.contains(&k.as_str()) && !v.is_null())
        .map(|(k, v)| {
            let val = match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            };
            format!("{}={}", enc(k), enc(&val))
        })
        .collect();
    if parts.is_empty() {
        String::new()
    } else {
        format!("?{}", parts.join("&"))
    }
}

fn arg<'a>(args: &'a Map<String, Value>, k: &str) -> Result<&'a str, String> {
    args.get(k).and_then(Value::as_str).ok_or_else(|| format!("missing string argument {k:?}"))
}

/// (method, path and query, JSON body) of the REST call behind a tool.
fn route(name: &str, args: &Map<String, Value>) -> Result<(Method, String, Option<Value>), String> {
    Ok(match name {
        "ecdb_schema" => (Method::GET, "/v1/schema".into(), None),
        "ecdb_search_curves" => (Method::GET, format!("/v1/curves{}", query(args, &[])), None),
        "ecdb_get_curve" => (Method::GET, format!("/v1/curves/{}", enc(arg(args, "ref")?)), None),
        "ecdb_curve_detail" => {
            let part = arg(args, "part")?;
            const PARTS: &[&str] = &[
                "isogenies",
                "graph",
                "transfers",
                "covers",
                "traits",
                "relations",
                "endomorphisms",
                "division-polynomials",
                "measurements",
            ];
            if !PARTS.contains(&part) {
                return Err(format!("part must be one of {PARTS:?}"));
            }
            (
                Method::GET,
                format!("/v1/curves/{}/{part}{}", enc(arg(args, "ref")?), query(args, &["ref", "part"])),
                None,
            )
        }
        "ecdb_isogeny_class" => (Method::GET, format!("/v1/isogeny-classes/{}", enc(arg(args, "label")?)), None),
        "ecdb_transfers" => (Method::GET, format!("/v1/transfers{}", query(args, &[])), None),
        "ecdb_polynomials" => match arg(args, "kind")? {
            "modular" => (Method::GET, format!("/v1/modular-polynomials{}", query(args, &["kind"])), None),
            "class" => (Method::GET, "/v1/class-polynomials".into(), None),
            "summation" => (Method::GET, "/v1/summation-polynomials".into(), None),
            other => return Err(format!("unknown polynomial kind {other:?}")),
        },
        "ecdb_results" => match arg(args, "kind")? {
            "leaderboard" => (Method::GET, format!("/v1/ic/leaderboard{}", query(args, &["kind"])), None),
            "progress" => (Method::GET, format!("/v1/ic/progress{}", query(args, &["kind"])), None),
            "boundary" => (Method::GET, format!("/v1/ic/boundary{}", query(args, &["kind"])), None),
            "benchmarks" => (Method::GET, format!("/v1/benchmarks{}", query(args, &["kind"])), None),
            "claims" => (Method::GET, format!("/v1/ic/claims{}", query(args, &["kind"])), None),
            "designs" => (Method::GET, "/v1/factor-base-designs".into(), None),
            other => return Err(format!("unknown results kind {other:?}")),
        },
        "ecdb_ecbench" => match args.get("session_id").and_then(Value::as_str) {
            Some(id) => (Method::GET, format!("/v1/ecbench/sessions/{}", enc(id)), None),
            None => (Method::GET, format!("/v1/ecbench/sessions{}", query(args, &["session_id"])), None),
        },
        "ecdb_factor_bases" => {
            let wants_rows = |k: &str| args.get(k).and_then(Value::as_bool) == Some(true);
            match args.get("key").and_then(Value::as_str) {
                Some(k) if wants_rows("results") || wants_rows("points") => (
                    Method::GET,
                    format!("/v1/factor-bases/{}/results{}", enc(k), query(args, &["key", "results"])),
                    None,
                ),
                Some(k) => (Method::GET, format!("/v1/factor-bases/{}", enc(k)), None),
                None => (Method::GET, format!("/v1/factor-bases{}", query(args, &["key", "results", "points"])), None),
            }
        }
        "ecdb_sql" => (Method::POST, "/v1/sql".into(), Some(Value::Object(args.clone()))),
        "ecdb_record_evidence" => (Method::POST, "/v1/evidence".into(), Some(Value::Object(args.clone()))),
        other => return Err(format!("unknown tool {other:?}")),
    })
}

/// Shape the schema answer: the relation list, or one relation in full.
fn schema_view(body: Value, table: Option<&str>) -> Value {
    let rels = body["relations"].as_array().cloned().unwrap_or_default();
    match table {
        Some(t) => rels
            .into_iter()
            .find(|r| r["name"].as_str() == Some(t))
            .unwrap_or_else(|| json!({ "error": format!("no relation {t:?} in schema ec") })),
        None => json!({
            "conventions": body["conventions"],
            "relations": rels.iter().map(|r| json!({ "name": r["name"], "kind": r["kind"], "comment": r["comment"] })).collect::<Vec<_>>(),
            "enums": body["enums"], "functions": body["functions"],
        }),
    }
}

async fn call_tool(api: Router, auth: Option<&str>, params: &Value) -> Value {
    let name = params["name"].as_str().unwrap_or_default();
    let args = params["arguments"].as_object().cloned().unwrap_or_default();
    let tool_error = |msg: String| json!({ "content": [{ "type": "text", "text": msg }], "isError": true });
    let (method, uri, body) = match route(name, &args) {
        Ok(r) => r,
        Err(e) => return tool_error(e),
    };
    let mut req = Request::builder().method(method).uri(&uri);
    if let Some(a) = auth {
        req = req.header(header::AUTHORIZATION, a);
    }
    let req = match body {
        Some(b) => req.header(header::CONTENT_TYPE, "application/json").body(Body::from(b.to_string())),
        None => req.body(Body::empty()),
    };
    let Ok(req) = req else { return tool_error("could not build the request".into()) };
    let res = match api.oneshot(req).await {
        Ok(r) => r,
        Err(e) => return tool_error(format!("request failed: {e}")),
    };
    let status = res.status();
    let bytes = to_bytes(res.into_body(), 64 * 1024 * 1024).await.unwrap_or_default();
    let mut text = String::from_utf8_lossy(&bytes).into_owned();
    if name == "ecdb_schema" && status.is_success() {
        if let Ok(v) = serde_json::from_str::<Value>(&text) {
            text = schema_view(v, args.get("table").and_then(Value::as_str)).to_string();
        }
    }
    if status == StatusCode::UNAUTHORIZED {
        text = format!(
            "{text}\nThe MCP connection needs `Authorization: Bearer <token>`; the token is read_token in AWS Secrets Manager secret ecdb/api (us-west-2)."
        );
    }
    if text.len() > MAX_TEXT {
        let mut cut = MAX_TEXT;
        while !text.is_char_boundary(cut) {
            cut -= 1;
        }
        text.truncate(cut);
        text.push_str("\n... truncated; narrow the request (limit, part, or ecdb_sql).");
    }
    json!({ "content": [{ "type": "text", "text": text }], "isError": !status.is_success() })
}

fn rpc_result(id: &Value, result: Value) -> Response {
    Json(json!({ "jsonrpc": "2.0", "id": id, "result": result })).into_response()
}

fn rpc_error(id: &Value, code: i64, message: &str) -> Response {
    Json(json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })).into_response()
}

pub async fn handle(api: Router, headers: HeaderMap, body: Bytes) -> Response {
    let Ok(msg) = serde_json::from_slice::<Value>(&body) else {
        return rpc_error(&Value::Null, -32700, "parse error");
    };
    if msg.is_array() {
        return rpc_error(&Value::Null, -32600, "batched JSON-RPC is not supported");
    }
    let id = msg.get("id").cloned();
    let method = msg["method"].as_str().unwrap_or_default();
    // Notifications and responses from the client carry no id: acknowledge.
    let Some(id) = id else {
        return StatusCode::ACCEPTED.into_response();
    };
    let params = msg.get("params").cloned().unwrap_or(Value::Null);
    match method {
        "initialize" => {
            let asked = params["protocolVersion"].as_str().unwrap_or_default();
            let version = if PROTOCOL_VERSIONS.contains(&asked) { asked } else { PROTOCOL_VERSIONS[0] };
            rpc_result(
                &id,
                json!({
                    "protocolVersion": version,
                    "capabilities": { "tools": { "listChanged": false } },
                    "serverInfo": { "name": "ecdb", "version": env!("CARGO_PKG_VERSION") },
                    "instructions": INSTRUCTIONS,
                }),
            )
        }
        "ping" => rpc_result(&id, json!({})),
        "tools/list" => rpc_result(&id, json!({ "tools": tools() })),
        "tools/call" => {
            let auth = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok());
            rpc_result(&id, call_tool(api, auth, &params).await)
        }
        "resources/list" => rpc_result(&id, json!({ "resources": [] })),
        "prompts/list" => rpc_result(&id, json!({ "prompts": [] })),
        _ => rpc_error(&id, -32601, &format!("method {method:?} not found")),
    }
}

/// No server-initiated stream: GET /mcp is not offered.
pub async fn get_not_supported() -> Response {
    (StatusCode::METHOD_NOT_ALLOWED, [(header::ALLOW, "POST")]).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tools_map_to_routes() {
        let mut a = Map::new();
        a.insert("ref".into(), json!("256-bit Random ECP Group"));
        a.insert("part".into(), json!("graph"));
        a.insert("depth".into(), json!(2));
        let (m, uri, _) = route("ecdb_curve_detail", &a).unwrap();
        assert_eq!(m, Method::GET);
        assert_eq!(uri, "/v1/curves/256-bit%20Random%20ECP%20Group/graph?depth=2");
        let mut q = Map::new();
        q.insert("family".into(), json!("koblitz"));
        q.insert("end_is_maximal".into(), json!(true));
        assert_eq!(route("ecdb_search_curves", &q).unwrap().1, "/v1/curves?end_is_maximal=true&family=koblitz");
        assert!(route("ecdb_nope", &Map::new()).is_err());
        let mut r = Map::new();
        r.insert("kind".into(), json!("leaderboard"));
        r.insert("family".into(), json!("A"));
        assert_eq!(route("ecdb_results", &r).unwrap().1, "/v1/ic/leaderboard?family=A");
        let mut e = Map::new();
        e.insert("session_id".into(), json!("ECBS1hffaa276d8a5c"));
        assert_eq!(route("ecdb_ecbench", &e).unwrap().1, "/v1/ecbench/sessions/ECBS1hffaa276d8a5c");
        let mut fb = Map::new();
        fb.insert("key".into(), json!("FB1h8e805e4ad71c"));
        assert_eq!(route("ecdb_factor_bases", &fb).unwrap().1, "/v1/factor-bases/FB1h8e805e4ad71c");
        fb.insert("results".into(), json!(true));
        fb.insert("solved".into(), json!(true));
        assert_eq!(route("ecdb_factor_bases", &fb).unwrap().1, "/v1/factor-bases/FB1h8e805e4ad71c/results?solved=true");
        let mut c = Map::new();
        c.insert("kind".into(), json!("claims"));
        c.insert("summary".into(), json!(true));
        assert_eq!(route("ecdb_results", &c).unwrap().1, "/v1/ic/claims?summary=true");
        assert_eq!(tools().as_array().unwrap().len(), 12);
    }
}
