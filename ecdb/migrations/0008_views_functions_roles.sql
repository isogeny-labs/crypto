-- ecdb 0008: query surface for agents — name resolution, the flat curve
-- summary, the full curve document, isogeny-graph traversal — and the
-- reader / writer roles.

-- ------------------------------------------------- name resolution

CREATE FUNCTION ec.resolve_curve(ref text) RETURNS SETOF bigint
  LANGUAGE sql STABLE PARALLEL SAFE AS $$
  SELECT DISTINCT id FROM (
    SELECT c.id FROM ec.curve c WHERE ref ~ '^[0-9]+$' AND c.id = ref::bigint
    UNION ALL SELECT c.id FROM ec.curve c WHERE c.icv1_slug = ref OR c.icv1 = ref
    UNION ALL SELECT r.curve_id FROM ec.curve_representation r
              WHERE r.ec1_alias = ref OR r.curve_uid = ref OR r.curve_sha256 = lower(ref)
    UNION ALL SELECT a.curve_id FROM ec.curve_alias a WHERE lower(a.alias) = lower(ref)
    UNION ALL SELECT s.curve_id FROM ec.curve_standard s WHERE lower(s.display_name) = lower(ref) OR s.oid = ref
  ) hits
$$;
COMMENT ON FUNCTION ec.resolve_curve(text) IS
  'Curve ids denoted by a reference: numeric id, ICV1 slug or string, EC1 alias, curve_uid, any alias or standard name (case-insensitive), OID. Several ids mean the name is ambiguous.';

-- ------------------------------------------------- flat summary

CREATE VIEW ec.v_curve_summary AS
SELECT
  c.id,
  c.icv1_slug,
  c.icv1,
  c.family,
  c.genus,
  c.model,
  c.form,
  f.id                AS field_id,
  f.kind              AS field_kind,
  f.characteristic,
  f.degree            AS field_degree,
  f.order_q           AS q,
  f.order_bits        AS field_bits,
  f.icv1_field,
  (SELECT array_agg(DISTINCT s.display_name ORDER BY s.display_name)
     FROM ec.curve_standard s WHERE s.curve_id = c.id) AS standard_names,
  g1.j_invariant,
  av.id               AS jacobian_id,
  ic.id               AS isogeny_class_id,
  ic.label            AS isogeny_class_label,
  ic.trace,
  ic.order_value      AS group_order,
  ic.order_bits       AS group_order_bits,
  fr.frobenius_discriminant,
  fr.cm_discriminant,
  fr.conductor_v,
  fr.conductor_bits,
  fr.class_number,
  fr.trace_ratio,
  fr.is_anomalous,
  fr.is_supersingular,
  er.discriminant     AS end_discriminant,
  er.conductor_f      AS end_conductor,
  er.is_maximal       AS end_is_maximal,
  er.status           AS end_status,
  sg.id               AS subgroup_id,
  sg.order_value      AS subgroup_order,
  sg.order_bits       AS subgroup_bits,
  sg.cofactor,
  pd.embedding_degree,
  pd.embedding_lower_bound,
  td.twist_order,
  td.largest_prime_bits AS twist_largest_prime_bits,
  dd.j_field_degree,
  dd.definition_degree,
  (SELECT count(*) FROM ec.isogeny i WHERE i.source_curve_id = c.id OR i.target_curve_id = c.id) AS isogeny_count,
  (SELECT count(*) FROM ec.curve_representation r WHERE r.curve_id = c.id) AS representation_count,
  (SELECT count(*) FROM ec.curve_morphism m WHERE m.target_curve_id = c.id) AS cover_count
FROM ec.curve c
JOIN ec.field f ON f.id = c.field_id
LEFT JOIN ec.genus1_invariants g1 ON g1.curve_id = c.id
LEFT JOIN ec.abelian_variety av ON av.curve_id = c.id
LEFT JOIN ec.isogeny_class_fq ic ON ic.id = av.isogeny_class_id
LEFT JOIN ec.frobenius_genus1 fr ON fr.isogeny_class_id = ic.id
LEFT JOIN ec.endomorphism_ring er ON er.curve_id = c.id
LEFT JOIN LATERAL (
  SELECT s.* FROM ec.subgroup s WHERE s.av_id = av.id AND s.field_id = c.field_id
  ORDER BY s.is_prime_order DESC NULLS LAST, s.order_value DESC LIMIT 1
) sg ON true
LEFT JOIN ec.pairing_data pd ON pd.subgroup_id = sg.id
LEFT JOIN ec.twist_data td ON td.curve_id = c.id AND td.twist_degree = 2 AND td.twist_index = 1
LEFT JOIN ec.descent_data dd ON dd.curve_id = c.id;
COMMENT ON VIEW ec.v_curve_summary IS
  'One row per curve with the most-used invariants. The primary subgroup is the largest prime-order subgroup over the base field.';

CREATE VIEW ec.v_isogeny_edge AS
SELECT i.id, i.external_id, i.degree, i.ell, i.direction, i.formula, i.status,
       i.source_curve_id, sc.icv1_slug AS source_slug,
       i.target_curve_id, tc.icv1_slug AS target_slug,
       i.dimension, i.is_separable, i.kernel_polynomial IS NOT NULL AS has_kernel_polynomial
FROM ec.isogeny i
LEFT JOIN ec.curve sc ON sc.id = i.source_curve_id
LEFT JOIN ec.curve tc ON tc.id = i.target_curve_id;
COMMENT ON VIEW ec.v_isogeny_edge IS 'Isogeny edges with curve slugs.';

-- ------------------------------------------------- isogeny-graph traversal

CREATE FUNCTION ec.isogeny_neighbourhood(root bigint, max_depth integer DEFAULT 2, only_ell numeric DEFAULT NULL)
RETURNS TABLE (isogeny_id bigint, depth integer, source_curve_id bigint, target_curve_id bigint,
               degree numeric, ell numeric, direction ec.volcano_direction, status ec.fact_status)
LANGUAGE sql STABLE AS $$
  WITH RECURSIVE walk(curve_id, depth, path) AS (
    SELECT root, 0, ARRAY[root]
    UNION ALL
    SELECT CASE WHEN i.source_curve_id = w.curve_id THEN i.target_curve_id ELSE i.source_curve_id END,
           w.depth + 1,
           w.path || CASE WHEN i.source_curve_id = w.curve_id THEN i.target_curve_id ELSE i.source_curve_id END
    FROM walk w
    JOIN ec.isogeny i ON (i.source_curve_id = w.curve_id OR i.target_curve_id = w.curve_id)
    WHERE w.depth < max_depth
      AND (only_ell IS NULL OR i.ell = only_ell)
      AND NOT (CASE WHEN i.source_curve_id = w.curve_id THEN i.target_curve_id ELSE i.source_curve_id END = ANY (w.path))
  ), reached AS (
    SELECT curve_id, min(depth) AS depth FROM walk GROUP BY curve_id
  )
  SELECT DISTINCT ON (i.id) i.id, greatest(rs.depth, rt.depth), i.source_curve_id, i.target_curve_id,
         i.degree, i.ell, i.direction, i.status
  FROM ec.isogeny i
  JOIN reached rs ON rs.curve_id = i.source_curve_id
  JOIN reached rt ON rt.curve_id = i.target_curve_id
  WHERE (only_ell IS NULL OR i.ell = only_ell)
  ORDER BY i.id
$$;
COMMENT ON FUNCTION ec.isogeny_neighbourhood(bigint, integer, numeric) IS
  'Isogeny edges among the curves within max_depth steps of root (optionally one prime ell). Traverses recorded edges only.';

-- ------------------------------------------------- full curve document

CREATE FUNCTION ec.curve_document(cid bigint) RETURNS jsonb
  LANGUAGE sql STABLE AS $$
SELECT jsonb_build_object(
  'id', c.id,
  'icv1_slug', c.icv1_slug,
  'icv1', c.icv1,
  'family', c.family,
  'genus', c.genus,
  'model', c.model,
  'form', c.form,
  'coefficients', c.coefficients,
  'equation', c.equation,
  'model_json', c.model_json,
  'model_sha256', c.model_sha256,
  'field', (SELECT to_jsonb(f) - 'provenance_id' FROM ec.field f WHERE f.id = c.field_id),
  'field_ec1_records', (SELECT jsonb_agg(jsonb_build_object('field_sha256', r.field_sha256, 'record', r.record))
                          FROM ec.field_ec1_record r WHERE r.field_id = c.field_id),
  'genus1_invariants', (SELECT to_jsonb(g) - 'curve_id' FROM ec.genus1_invariants g WHERE g.curve_id = c.id),
  'hyperelliptic_model', (SELECT to_jsonb(h) - 'curve_id' FROM ec.hyperelliptic_model h WHERE h.curve_id = c.id),
  'equation_model', (SELECT to_jsonb(m) - 'curve_id' FROM ec.curve_equation_model m WHERE m.curve_id = c.id),
  'geometric_invariants', (SELECT jsonb_agg(to_jsonb(gi) - 'curve_id') FROM ec.geometric_invariant gi WHERE gi.curve_id = c.id),
  'aliases', (SELECT jsonb_agg(jsonb_build_object('namespace', a.namespace, 'alias', a.alias, 'retired', a.is_retired)
                               ORDER BY a.namespace, a.alias)
                FROM ec.curve_alias a WHERE a.curve_id = c.id),
  'standards', (SELECT jsonb_agg(to_jsonb(s) - 'curve_id' - 'id') FROM ec.curve_standard s WHERE s.curve_id = c.id),
  'representations', (SELECT jsonb_agg(jsonb_build_object(
                          'ec1_alias', r.ec1_alias, 'curve_uid', r.curve_uid, 'model_label', r.model_label,
                          'subgroup_order', r.subgroup_order, 'cofactor', r.cofactor, 'generator', r.generator,
                          'target_group', r.target_group, 'identity_verified', r.identity_verified,
                          'sources', r.sources))
                        FROM ec.curve_representation r WHERE r.curve_id = c.id),
  'jacobian', (SELECT to_jsonb(av) - 'curve_id' FROM ec.abelian_variety av WHERE av.curve_id = c.id),
  'isogeny_class', (SELECT to_jsonb(ic) FROM ec.isogeny_class_fq ic
                      JOIN ec.abelian_variety av ON av.isogeny_class_id = ic.id WHERE av.curve_id = c.id),
  'zeta', (SELECT to_jsonb(z) - 'curve_id' FROM ec.curve_zeta z WHERE z.curve_id = c.id),
  'frobenius', (SELECT to_jsonb(fr) FROM ec.frobenius_genus1 fr
                  JOIN ec.abelian_variety av ON av.isogeny_class_id = fr.isogeny_class_id WHERE av.curve_id = c.id),
  'frobenius_ell', (SELECT jsonb_agg(to_jsonb(fe) - 'isogeny_class_id' ORDER BY fe.ell) FROM ec.frobenius_ell fe
                      JOIN ec.abelian_variety av ON av.isogeny_class_id = fe.isogeny_class_id WHERE av.curve_id = c.id),
  'ell_torsion', (SELECT jsonb_agg(to_jsonb(t) - 'curve_id' ORDER BY t.ell) FROM ec.curve_ell_torsion t WHERE t.curve_id = c.id),
  'endomorphism_ring', (SELECT to_jsonb(er) - 'curve_id' FROM ec.endomorphism_ring er WHERE er.curve_id = c.id),
  'volcano_levels', (SELECT jsonb_agg(to_jsonb(el) - 'curve_id' ORDER BY el.ell) FROM ec.endomorphism_level el WHERE el.curve_id = c.id),
  'automorphism_group', (SELECT to_jsonb(ag) - 'curve_id' FROM ec.automorphism_group ag WHERE ag.curve_id = c.id),
  'endomorphisms', (SELECT jsonb_agg(to_jsonb(e) - 'curve_id') FROM ec.endomorphism e WHERE e.curve_id = c.id),
  'scalar_decompositions', (SELECT jsonb_agg(to_jsonb(sd) - 'curve_id') FROM ec.scalar_decomposition sd WHERE sd.curve_id = c.id),
  'tau_adic', (SELECT to_jsonb(t) - 'curve_id' FROM ec.tau_adic_data t WHERE t.curve_id = c.id),
  'descent', (SELECT to_jsonb(d) - 'curve_id' FROM ec.descent_data d WHERE d.curve_id = c.id),
  'twists', (SELECT jsonb_agg(to_jsonb(t) - 'curve_id') FROM ec.twist_data t WHERE t.curve_id = c.id),
  'group_structure', (SELECT jsonb_agg(to_jsonb(gs) - 'av_id' ORDER BY gs.extension_degree) FROM ec.group_structure gs
                        JOIN ec.abelian_variety av ON av.id = gs.av_id WHERE av.curve_id = c.id),
  'subgroups', (SELECT jsonb_agg(jsonb_build_object(
                    'id', s.id, 'order', s.order_value, 'order_bits', s.order_bits, 'cofactor', s.cofactor,
                    'is_prime_order', s.is_prime_order, 'status', s.status,
                    'generator', (SELECT p.coordinates FROM ec.point p WHERE p.id = s.generator_point_id),
                    'pairing', (SELECT to_jsonb(pd) - 'subgroup_id' FROM ec.pairing_data pd WHERE pd.subgroup_id = s.id),
                    'security', (SELECT jsonb_agg(to_jsonb(sa) - 'subgroup_id') FROM ec.security_assessment sa WHERE sa.subgroup_id = s.id)))
                  FROM ec.subgroup s JOIN ec.abelian_variety av ON av.id = s.av_id WHERE av.curve_id = c.id),
  'p_adic', (SELECT to_jsonb(pa) - 'av_id' FROM ec.p_adic_invariants pa
               JOIN ec.abelian_variety av ON av.id = pa.av_id WHERE av.curve_id = c.id),
  'global', (SELECT to_jsonb(gd) - 'curve_id' FROM ec.curve_global_data gd WHERE gd.curve_id = c.id),
  'generation', (SELECT to_jsonb(gp) - 'curve_id' FROM ec.generation_provenance gp WHERE gp.curve_id = c.id),
  'relations', (SELECT jsonb_agg(jsonb_build_object('id', r.id, 'kind', r.kind, 'status', r.status,
                    'other_curve_id', CASE WHEN r.source_curve_id = c.id THEN r.target_curve_id ELSE r.source_curve_id END,
                    'role', CASE WHEN r.source_curve_id = c.id THEN 'source' ELSE 'target' END))
                  FROM ec.curve_relation r WHERE r.source_curve_id = c.id OR r.target_curve_id = c.id),
  'isogenies', (SELECT jsonb_agg(jsonb_build_object('id', i.id, 'degree', i.degree, 'ell', i.ell,
                    'direction', i.direction, 'status', i.status,
                    'other_curve_id', CASE WHEN i.source_curve_id = c.id THEN i.target_curve_id ELSE i.source_curve_id END,
                    'role', CASE WHEN i.source_curve_id = c.id THEN 'source' ELSE 'target' END) ORDER BY i.degree, i.id)
                  FROM ec.isogeny i WHERE i.source_curve_id = c.id OR i.target_curve_id = c.id),
  'covers', (SELECT jsonb_agg(jsonb_build_object('id', m.id, 'kind', m.kind, 'construction', m.construction,
                 'degree', m.degree, 'cover_genus', m.cover_genus, 'cover_curve_id', m.source_curve_id,
                 'status', m.status, 'subgroup_transfer', m.subgroup_transfer))
               FROM ec.curve_morphism m WHERE m.target_curve_id = c.id),
  'covered_curves', (SELECT jsonb_agg(jsonb_build_object('id', m.id, 'kind', m.kind, 'degree', m.degree,
                         'target_curve_id', m.target_curve_id, 'status', m.status))
                       FROM ec.curve_morphism m WHERE m.source_curve_id = c.id),
  'transfers', (SELECT jsonb_agg(jsonb_build_object('id', t.id, 'route_kind', t.route_kind, 'label', t.label,
                    'target', t.target, 'target_genus', t.target_genus, 'verdict', t.verdict,
                    'log2_total_cost', t.log2_total_cost, 'log2_reference_cost', t.log2_reference_cost,
                    'ratio_log2', t.ratio_log2, 'status', t.status) ORDER BY t.route_index)
                  FROM ec.dlp_transfer t WHERE t.source_curve_id = c.id),
  'division_polynomials', (SELECT jsonb_agg(jsonb_build_object('n', d.n, 'variant', d.variant,
                               'polynomial_sha256', p.sha256, 'factor_degrees', d.factor_degrees) ORDER BY d.n)
                             FROM ec.division_polynomial d JOIN ec.polynomial p ON p.id = d.polynomial_id
                            WHERE d.curve_id = c.id),
  'certificates', (SELECT jsonb_agg(jsonb_build_object('id', ce.id, 'kind', ce.kind, 'uid', ce.certificate_uid, 'status', ce.status))
                     FROM ec.certificate ce WHERE ce.subject_table = 'curve' AND ce.subject_id = c.id),
  'external_labels', (SELECT jsonb_agg(jsonb_build_object('catalog', x.catalog, 'label', x.label, 'match_status', x.match_status))
                        FROM ec.external_label x WHERE x.object_table = 'curve' AND x.object_id = c.id),
  'evidence', (SELECT jsonb_agg(jsonb_build_object('kind', e.kind, 'repository', e.repository, 'path', e.path,
                   'sha256', e.sha256, 'validation_status', e.validation_status))
                 FROM ec.evidence_link e WHERE e.curve_id = c.id),
  'trait_schemas', (SELECT jsonb_agg(DISTINCT t.schema) FROM ec.trait_record t WHERE t.curve_id = c.id)
)
FROM ec.curve c WHERE c.id = cid
$$;
COMMENT ON FUNCTION ec.curve_document(bigint) IS 'Everything recorded about one curve, as one JSON document (the API''s GET /v1/curves/{ref}).';

-- ------------------------------------------------- roles

DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'ecdb_reader') THEN
    CREATE ROLE ecdb_reader NOLOGIN;
  END IF;
  IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'ecdb_writer') THEN
    CREATE ROLE ecdb_writer NOLOGIN;
  END IF;
END $$;

GRANT USAGE ON SCHEMA ec TO ecdb_reader, ecdb_writer;
GRANT SELECT ON ALL TABLES IN SCHEMA ec TO ecdb_reader, ecdb_writer;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA ec TO ecdb_reader, ecdb_writer;
GRANT INSERT, UPDATE ON ALL TABLES IN SCHEMA ec TO ecdb_writer;
GRANT USAGE ON ALL SEQUENCES IN SCHEMA ec TO ecdb_writer;
REVOKE INSERT, UPDATE, DELETE ON ec.schema_migrations FROM ecdb_writer;
ALTER DEFAULT PRIVILEGES IN SCHEMA ec GRANT SELECT ON TABLES TO ecdb_reader, ecdb_writer;
ALTER DEFAULT PRIVILEGES IN SCHEMA ec GRANT INSERT, UPDATE ON TABLES TO ecdb_writer;
ALTER DEFAULT PRIVILEGES IN SCHEMA ec GRANT USAGE ON SEQUENCES TO ecdb_writer;
COMMENT ON ROLE ecdb_reader IS 'Read-only: the API read path and POST /v1/sql run as a member of this role.';
COMMENT ON ROLE ecdb_writer IS 'Insert/update without delete: importers and the API write path.';
