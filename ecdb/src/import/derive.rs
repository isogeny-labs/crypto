//! Facts derived inside the database from rows already imported, in SQL so
//! they are reproducible from the stored inputs alone.
//!
//! * Automorphism groups of genus-1 curves over F_q (q = p^n), p >= 5, from j
//!   and q: Aut = C2 unless j = 0 (C6 when q = 1 mod 3) or j = 1728
//!   (C4 when q = 1 mod 4); geometric orders 6 and 4. Characteristic 2 and 3
//!   with j != 0: C2. (Silverman, AEC, III.10.1 and A.1; provenance: recalled.)
//!   j = 0 in characteristic 2 or 3 (supersingular, Aut up to 24 / 12) is left
//!   unknown here: the group over F_q depends on more than j and q.
//! * One security_assessment row per prime-order subgroup with structural
//!   values copied from the trait catalogue, and the textbook rho estimate
//!   0.5 log2(pi r / 4) with its automorphism fold, status heuristic.
//! * The q-power Frobenius of every genus-1 curve with a known Weil
//!   polynomial: purely inseparable, degree q, x^2 - t x + q.
//! * Subfield Frobenius tau for binary curves that descend to GF(2^k) with a
//!   signed base trace t_k (Koblitz curves: k = 1, t_1 = mu = (-1)^(1-a)):
//!   tau^2 - t_k tau + 2^k = 0, V_n = t, and N(delta) = #E(F_{2^n}) / #E(F_{2^k})
//!   for delta = (tau^n - 1)/(tau - 1).

use super::{Ctx, Report};
use anyhow::Result;

const AUT_SQL: &str = "
INSERT INTO ec.automorphism_group (curve_id, group_order, group_label, geometric_order, geometric_label, status, provenance_id)
SELECT c.id,
       CASE WHEN f.characteristic >= 5 AND g.j_is_0 AND mod(f.order_q, 3) = 1 THEN 6
            WHEN f.characteristic >= 5 AND g.j_is_1728 AND mod(f.order_q, 4) = 1 THEN 4
            ELSE 2 END,
       CASE WHEN f.characteristic >= 5 AND g.j_is_0 AND mod(f.order_q, 3) = 1 THEN 'C6'
            WHEN f.characteristic >= 5 AND g.j_is_1728 AND mod(f.order_q, 4) = 1 THEN 'C4'
            ELSE 'C2' END,
       CASE WHEN f.characteristic >= 5 AND g.j_is_0 THEN 6
            WHEN f.characteristic >= 5 AND g.j_is_1728 THEN 4
            ELSE 2 END,
       CASE WHEN f.characteristic >= 5 AND g.j_is_0 THEN 'C6'
            WHEN f.characteristic >= 5 AND g.j_is_1728 THEN 'C4'
            ELSE 'C2' END,
       'proved', $1::bigint
FROM ec.curve c
JOIN ec.field f ON f.id = c.field_id
JOIN ec.genus1_invariants g ON g.curve_id = c.id
WHERE c.genus = 1 AND f.order_q IS NOT NULL AND g.j_invariant IS NOT NULL
  AND (f.characteristic >= 5 OR NOT g.j_is_0)
ON CONFLICT (curve_id) DO NOTHING";

const SECURITY_SQL: &str = "
INSERT INTO ec.security_assessment (subgroup_id, rho_log2, automorphism_fold, rho_log2_folded, pohlig_hellman_bits,
       embedding_degree, is_anomalous, twist_security_bits, cm_discriminant_bits, status, notes, provenance_id)
SELECT s.id,
       round(0.5 * log(2::numeric, pi()::numeric * s.order_value / 4), 3),
       ag.group_order,
       round(0.5 * log(2::numeric, pi()::numeric * s.order_value / 4) - 0.5 * log(2::numeric, coalesce(ag.group_order, 2)), 3),
       CASE WHEN s.is_prime_order THEN s.order_bits END,
       pd.embedding_degree,
       fr.is_anomalous AND f.degree = 1,
       td.largest_prime_bits,
       ec.bits(fr.cm_discriminant),
       'heuristic',
       'rho_log2: expected group operations of parallel rho, 0.5 log2(pi r / 4), random-mapping heuristic; folded by |Aut| only (no Frobenius or GLS fold). Not a measured cost and not a matched ecbench reference.',
       $1::bigint
FROM ec.subgroup s
JOIN ec.abelian_variety av ON av.id = s.av_id
JOIN ec.curve c ON c.id = av.curve_id
JOIN ec.field f ON f.id = c.field_id
LEFT JOIN ec.automorphism_group ag ON ag.curve_id = c.id
LEFT JOIN ec.pairing_data pd ON pd.subgroup_id = s.id
LEFT JOIN ec.frobenius_genus1 fr ON fr.isogeny_class_id = av.isogeny_class_id
LEFT JOIN ec.twist_data td ON td.curve_id = c.id AND td.twist_degree = 2 AND td.twist_index = 1
WHERE c.genus = 1 AND s.is_prime_order AND s.order_value > 1
  AND NOT EXISTS (SELECT 1 FROM ec.security_assessment x WHERE x.subgroup_id = s.id AND x.provenance_id = $1::bigint)";

const FROBENIUS_SQL: &str = "
INSERT INTO ec.endomorphism (curve_id, kind, name, degree, characteristic_polynomial, trace, norm, field_of_definition_id,
       is_separable, formula, status, provenance_id)
SELECT c.id, 'frobenius', 'pi_q', ic.q, ARRAY[ic.q, -ic.trace, 1]::numeric[], ic.trace, ic.q, c.field_id, false,
       '(x, y) -> (x^q, y^q)', ic.status, $1::bigint
FROM ec.curve c
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE c.genus = 1
  AND NOT EXISTS (SELECT 1 FROM ec.endomorphism e WHERE e.curve_id = c.id AND e.kind = 'frobenius')";

const TAU_SQL: &str = "
INSERT INTO ec.tau_adic_data (curve_id, base_degree, extension_degree, koblitz_a, mu, tau_char_poly, base_trace,
       lucas_v_n, delta_norm, status, provenance_id)
SELECT c.id, d.definition_degree, f.degree / d.definition_degree,
       CASE WHEN d.definition_degree = 1 THEN (CASE c.coefficients->>'a' WHEN '0x0' THEN 0 WHEN '0x1' THEN 1 END) END,
       CASE WHEN d.definition_degree = 1 THEN sign(d.base_trace)::smallint END,
       ARRAY[trunc(power(2::numeric, d.definition_degree)), -d.base_trace, 1]::numeric[], d.base_trace, ic.trace,
       CASE WHEN mod(ic.order_value, trunc(power(2::numeric, d.definition_degree)) + 1 - d.base_trace) = 0
            THEN div(ic.order_value, trunc(power(2::numeric, d.definition_degree)) + 1 - d.base_trace) END,
       'proved', $1::bigint
FROM ec.curve c
JOIN ec.field f ON f.id = c.field_id AND f.kind = 'binary'
JOIN ec.descent_data d ON d.curve_id = c.id
JOIN ec.abelian_variety av ON av.curve_id = c.id
JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
WHERE d.status = 'proved' AND d.base_trace IS NOT NULL AND NOT d.base_trace_sign_free
  AND d.definition_degree < f.degree AND mod(f.degree, d.definition_degree) = 0
ON CONFLICT (curve_id) DO NOTHING";

const TAU_ENDO_SQL: &str = "
INSERT INTO ec.endomorphism (curve_id, kind, name, degree, characteristic_polynomial, trace, norm,
       field_of_definition_id, is_separable, formula, status, provenance_id)
SELECT t.curve_id, 'tau', 'tau = Frobenius of GF(2^' || t.base_degree || ')', trunc(power(2::numeric, t.base_degree)),
       t.tau_char_poly, t.base_trace, trunc(power(2::numeric, t.base_degree)), c.field_id, false,
       '(x, y) -> (x^(2^' || t.base_degree || '), y^(2^' || t.base_degree || '))', 'proved', $1::bigint
FROM ec.tau_adic_data t JOIN ec.curve c ON c.id = t.curve_id
WHERE NOT EXISTS (SELECT 1 FROM ec.endomorphism e WHERE e.curve_id = t.curve_id AND e.kind = 'tau')";

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let input = super::Input { rel_path: "(database)".into(), sha256: String::new(), doc: serde_json::Value::Null };
    let mut report = Report::new(&input);
    let src = ctx
        .client
        .query_one(
            "WITH ins AS (INSERT INTO ec.source (kind, title, provenance_class)
                          VALUES ('computation', 'ecdb derived facts', 'internal') ON CONFLICT DO NOTHING RETURNING id)
             SELECT id FROM ins UNION ALL
             SELECT id FROM ec.source WHERE kind = 'computation' AND title = 'ecdb derived facts' AND repository IS NULL LIMIT 1",
            &[],
        )
        .await?
        .get::<_, i64>(0);
    let aut = ctx.provenance(src, "Aut(E) from j and q (Silverman AEC III.10.1, recalled)", None).await?;
    let sec = ctx
        .provenance(src, "structural attack-surface values copied from imported rows; textbook rho formula", None)
        .await?;
    report.count("automorphism_groups", ctx.client.execute(AUT_SQL, &[&aut]).await?);
    report.count("security_assessments", ctx.client.execute(SECURITY_SQL, &[&sec]).await?);
    let frob = ctx
        .provenance(src, "q-power Frobenius from the Weil polynomial (status follows the order check)", None)
        .await?;
    report.count("frobenius_endomorphisms", ctx.client.execute(FROBENIUS_SQL, &[&frob]).await?);
    let tau = ctx
        .provenance(
            src,
            "subfield Frobenius tau from the proved descent degree and signed base trace (docs/curves/TRAITS.md)",
            None,
        )
        .await?;
    report.count("tau_adic", ctx.client.execute(TAU_SQL, &[&tau]).await?);
    report.count("tau_endomorphisms", ctx.client.execute(TAU_ENDO_SQL, &[&tau]).await?);
    Ok(report)
}
