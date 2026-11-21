-- 0016: saved research sweeps, factor-base object references, versioned
-- feature observations and reproducible ML dataset definitions.
--
-- A sweep is a plan, not evidence.  FB1/FBM1 identity and measured results
-- remain in migration 0011.  Large objects remain content-addressed ec.artifact
-- rows; factor_base_artifact gives a factor base one or more durable object
-- references without putting the bytes in Postgres.

CREATE TABLE ec.research_sweep (
  id                 bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name               text NOT NULL,
  version            integer NOT NULL CHECK (version >= 1),
  schema_uri         text NOT NULL,
  description        text NOT NULL,
  objective          text NOT NULL,
  selection_policy   jsonb NOT NULL DEFAULT '{}',
  feature_keys       text[] NOT NULL DEFAULT '{}',
  labels             jsonb NOT NULL DEFAULT '{}',
  definition_sha256  ec.sha256_hex NOT NULL,
  lifecycle          text NOT NULL DEFAULT 'active'
                     CHECK (lifecycle IN ('draft','active','complete','retired')),
  provenance_id      bigint NOT NULL REFERENCES ec.provenance(id),
  created_at         timestamptz NOT NULL DEFAULT now(),
  UNIQUE (name, version),
  UNIQUE (definition_sha256),
  CHECK (jsonb_typeof(selection_policy) = 'object'),
  CHECK (jsonb_typeof(labels) = 'object')
);
COMMENT ON TABLE ec.research_sweep IS
  'A versioned, content-bound curve-sweep plan. It states intended coverage and controls, not a measurement result.';

CREATE TABLE ec.research_sweep_cell (
  id                     bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  sweep_id               bigint NOT NULL REFERENCES ec.research_sweep(id) ON DELETE CASCADE,
  cell_key               text NOT NULL,
  ordinal                integer NOT NULL CHECK (ordinal >= 0),
  field_kind             ec.field_kind,
  family                 text,
  field_bits_min         integer CHECK (field_bits_min >= 1),
  field_bits_max         integer CHECK (field_bits_max >= 1),
  field_degree           integer CHECK (field_degree >= 1),
  curve_filter           jsonb NOT NULL,
  target_curves          integer NOT NULL DEFAULT 1 CHECK (target_curves >= 1),
  selection_seed         ec.nat,
  factor_base_selectors  jsonb NOT NULL DEFAULT '[]',
  measurement_plan       jsonb NOT NULL DEFAULT '{}',
  required_features      text[] NOT NULL DEFAULT '{}',
  lifecycle              text NOT NULL DEFAULT 'planned'
                         CHECK (lifecycle IN ('planned','running','complete','blocked','retired')),
  provenance_id          bigint NOT NULL REFERENCES ec.provenance(id),
  UNIQUE (sweep_id, cell_key),
  UNIQUE (sweep_id, ordinal),
  CHECK (field_bits_min IS NULL OR field_bits_max IS NULL OR field_bits_min <= field_bits_max),
  CHECK (jsonb_typeof(curve_filter) = 'object'),
  CHECK (jsonb_typeof(factor_base_selectors) = 'array'),
  CHECK (jsonb_typeof(measurement_plan) = 'object')
);
COMMENT ON TABLE ec.research_sweep_cell IS
  'One saved sweep stratum: an API curve filter, existing factor-base selectors, measurement/control plan and required features.';
CREATE INDEX research_sweep_cell_shape_idx
  ON ec.research_sweep_cell (field_kind, family, field_bits_min, field_bits_max);

CREATE TABLE ec.research_sweep_member (
  cell_id          bigint NOT NULL REFERENCES ec.research_sweep_cell(id) ON DELETE CASCADE,
  curve_id         bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  selection_order  integer CHECK (selection_order >= 0),
  role             text NOT NULL DEFAULT 'candidate'
                   CHECK (role IN ('candidate','control','reference','holdout','excluded')),
  status           ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id    bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (cell_id, curve_id)
);
COMMENT ON TABLE ec.research_sweep_member IS
  'Frozen membership of a sweep cell. Empty membership means its saved filter remains dynamic.';
CREATE INDEX research_sweep_member_curve_idx ON ec.research_sweep_member (curve_id);

CREATE TABLE ec.factor_base_artifact (
  factor_base_id  bigint NOT NULL REFERENCES ec.factor_base(id) ON DELETE CASCADE,
  artifact_id     bigint NOT NULL REFERENCES ec.artifact(id),
  role            text NOT NULL
                  CHECK (role IN ('dump','matrix','relations','log_table','certificate','manifest','dataset','other')),
  is_primary      boolean NOT NULL DEFAULT false,
  description     text,
  provenance_id   bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (factor_base_id, artifact_id, role)
);
COMMENT ON TABLE ec.factor_base_artifact IS
  'Content-addressed objects belonging to an FB1/FBS1/archive factor base. artifact.storage_uri is the S3 or other object-store URI.';
CREATE UNIQUE INDEX factor_base_primary_artifact_idx
  ON ec.factor_base_artifact (factor_base_id, role) WHERE is_primary;

CREATE TABLE ec.feature_definition (
  id                 bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  feature_key        text NOT NULL,
  version            integer NOT NULL CHECK (version >= 1),
  scope              text NOT NULL
                     CHECK (scope IN ('curve','field','subgroup','factor_base','factor_base_measurement','sweep_cell')),
  data_type          text NOT NULL
                     CHECK (data_type IN ('integer','number','boolean','string','category','object','array')),
  unit               text,
  nullable           boolean NOT NULL DEFAULT true,
  description        text NOT NULL,
  semantics_uri      text,
  source_expression  text,
  definition_sha256  ec.sha256_hex NOT NULL,
  provenance_id      bigint NOT NULL REFERENCES ec.provenance(id),
  UNIQUE (feature_key, version),
  UNIQUE (definition_sha256)
);
COMMENT ON TABLE ec.feature_definition IS
  'A versioned semantic contract for one model input, target or metadata value. Changed semantics require a new version.';
CREATE INDEX feature_definition_scope_idx ON ec.feature_definition (scope, feature_key);

CREATE TABLE ec.feature_observation (
  id                          bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  feature_definition_id       bigint NOT NULL REFERENCES ec.feature_definition(id),
  curve_id                    bigint REFERENCES ec.curve(id) ON DELETE CASCADE,
  field_id                    bigint REFERENCES ec.field(id) ON DELETE CASCADE,
  subgroup_id                 bigint REFERENCES ec.subgroup(id) ON DELETE CASCADE,
  factor_base_id              bigint REFERENCES ec.factor_base(id) ON DELETE CASCADE,
  factor_base_measurement_id  bigint REFERENCES ec.factor_base_measurement(id) ON DELETE CASCADE,
  sweep_cell_id               bigint REFERENCES ec.research_sweep_cell(id) ON DELETE CASCADE,
  value                       jsonb,
  missing_reason              text,
  context                     jsonb NOT NULL DEFAULT '{}',
  context_sha256              ec.sha256_hex GENERATED ALWAYS AS (ec.sha256_text(context::text)) STORED,
  status                      ec.fact_status NOT NULL DEFAULT 'unknown',
  observed_at                 timestamptz,
  provenance_id               bigint NOT NULL REFERENCES ec.provenance(id),
  CHECK (num_nonnulls(curve_id, field_id, subgroup_id, factor_base_id,
                      factor_base_measurement_id, sweep_cell_id) = 1),
  CHECK (jsonb_typeof(context) = 'object'),
  CHECK (num_nonnulls(value, missing_reason) <= 1),
  CHECK (value IS NOT NULL OR missing_reason IS NOT NULL OR
         status IN ('unknown','not_evaluated','not_applicable')),
  UNIQUE NULLS NOT DISTINCT
    (feature_definition_id, curve_id, field_id, subgroup_id, factor_base_id,
     factor_base_measurement_id, sweep_cell_id, context_sha256, provenance_id)
);
COMMENT ON TABLE ec.feature_observation IS
  'A typed feature value with one exact subject, context, evidence status and provenance. Missing is never coerced to zero.';
CREATE FUNCTION ec.validate_feature_observation() RETURNS trigger
LANGUAGE plpgsql AS $$
DECLARE
  expected_scope text;
  expected_type text;
  is_nullable boolean;
  actual_scope text;
  scalar text;
BEGIN
  SELECT scope, data_type, nullable
    INTO expected_scope, expected_type, is_nullable
    FROM ec.feature_definition WHERE id = NEW.feature_definition_id;
  actual_scope := CASE
    WHEN NEW.curve_id IS NOT NULL THEN 'curve'
    WHEN NEW.field_id IS NOT NULL THEN 'field'
    WHEN NEW.subgroup_id IS NOT NULL THEN 'subgroup'
    WHEN NEW.factor_base_id IS NOT NULL THEN 'factor_base'
    WHEN NEW.factor_base_measurement_id IS NOT NULL THEN 'factor_base_measurement'
    WHEN NEW.sweep_cell_id IS NOT NULL THEN 'sweep_cell'
  END;
  IF actual_scope IS NULL OR expected_scope <> actual_scope THEN
    RAISE EXCEPTION 'feature definition % expects scope %, not %',
      NEW.feature_definition_id, expected_scope, coalesce(actual_scope, 'no subject');
  END IF;
  IF NEW.value IS NULL THEN
    IF NOT is_nullable AND NEW.missing_reason IS NULL THEN
      RAISE EXCEPTION 'non-nullable feature definition % needs a value or explicit missing reason',
        NEW.feature_definition_id;
    END IF;
    RETURN NEW;
  END IF;
  scalar := NEW.value #>> '{}';
  IF NOT (CASE expected_type
    WHEN 'integer' THEN
      (jsonb_typeof(NEW.value) = 'number' AND NEW.value::text ~ '^-?(0|[1-9][0-9]*)$') OR
      (jsonb_typeof(NEW.value) = 'string' AND scalar ~ '^-?[0-9]+$')
    WHEN 'number' THEN
      jsonb_typeof(NEW.value) = 'number' OR
      (jsonb_typeof(NEW.value) = 'string' AND
       scalar ~ '^[+-]?([0-9]+([.][0-9]*)?|[.][0-9]+)([eE][+-]?[0-9]+)?$')
    WHEN 'boolean' THEN jsonb_typeof(NEW.value) = 'boolean'
    WHEN 'string' THEN jsonb_typeof(NEW.value) = 'string'
    WHEN 'category' THEN jsonb_typeof(NEW.value) = 'string'
    WHEN 'object' THEN jsonb_typeof(NEW.value) = 'object'
    WHEN 'array' THEN jsonb_typeof(NEW.value) = 'array'
    ELSE false
  END) THEN
    RAISE EXCEPTION 'feature definition % expects data type %, got %',
      NEW.feature_definition_id, expected_type, jsonb_typeof(NEW.value);
  END IF;
  RETURN NEW;
END $$;
CREATE TRIGGER feature_observation_contract
  BEFORE INSERT OR UPDATE ON ec.feature_observation
  FOR EACH ROW EXECUTE FUNCTION ec.validate_feature_observation();
CREATE INDEX feature_observation_curve_idx
  ON ec.feature_observation (curve_id, feature_definition_id);
CREATE INDEX feature_observation_factor_base_idx
  ON ec.feature_observation (factor_base_id, feature_definition_id);
CREATE INDEX feature_observation_measurement_idx
  ON ec.feature_observation (factor_base_measurement_id, feature_definition_id);
CREATE INDEX feature_observation_value_gin
  ON ec.feature_observation USING gin (value jsonb_path_ops);

CREATE TABLE ec.ml_dataset (
  id                 bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name               text NOT NULL,
  version            integer NOT NULL CHECK (version >= 1),
  subject_scope      text NOT NULL
                     CHECK (subject_scope IN ('curve','factor_base','factor_base_measurement')),
  description        text NOT NULL,
  selection_query    text NOT NULL,
  split_policy       jsonb NOT NULL,
  seed               ec.nat,
  artifact_id        bigint REFERENCES ec.artifact(id),
  definition_sha256  ec.sha256_hex NOT NULL,
  status             ec.fact_status NOT NULL DEFAULT 'not_evaluated',
  provenance_id      bigint NOT NULL REFERENCES ec.provenance(id),
  created_at         timestamptz NOT NULL DEFAULT now(),
  UNIQUE (name, version),
  UNIQUE (definition_sha256),
  CHECK (jsonb_typeof(split_policy) = 'object')
);
COMMENT ON TABLE ec.ml_dataset IS
  'A versioned dataset recipe. selection_query and split_policy define a reproducible export; artifact_id may pin materialized Parquet/Arrow bytes.';

CREATE TABLE ec.ml_dataset_feature (
  dataset_id            bigint NOT NULL REFERENCES ec.ml_dataset(id) ON DELETE CASCADE,
  feature_definition_id bigint NOT NULL REFERENCES ec.feature_definition(id),
  role                  text NOT NULL CHECK (role IN ('input','target','metadata')),
  ordinal               integer NOT NULL CHECK (ordinal >= 0),
  PRIMARY KEY (dataset_id, feature_definition_id, role),
  UNIQUE (dataset_id, role, ordinal)
);

CREATE TABLE ec.ml_dataset_member (
  id                          bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  dataset_id                  bigint NOT NULL REFERENCES ec.ml_dataset(id) ON DELETE CASCADE,
  sample_key                  text NOT NULL,
  curve_id                    bigint REFERENCES ec.curve(id) ON DELETE CASCADE,
  factor_base_id              bigint REFERENCES ec.factor_base(id) ON DELETE CASCADE,
  factor_base_measurement_id  bigint REFERENCES ec.factor_base_measurement(id) ON DELETE CASCADE,
  split                       text NOT NULL
                              CHECK (split IN ('train','validation','test','holdout','excluded')),
  sample_weight               numeric CHECK (sample_weight IS NULL OR sample_weight >= 0),
  labels                      jsonb NOT NULL DEFAULT '{}',
  provenance_id               bigint REFERENCES ec.provenance(id),
  CHECK (num_nonnulls(curve_id, factor_base_id, factor_base_measurement_id) = 1),
  CHECK (jsonb_typeof(labels) = 'object'),
  UNIQUE (dataset_id, sample_key),
  UNIQUE NULLS NOT DISTINCT (dataset_id, curve_id, factor_base_id, factor_base_measurement_id)
);
COMMENT ON TABLE ec.ml_dataset_member IS
  'Frozen sample membership and leakage-aware split for one dataset version.';
CREATE INDEX ml_dataset_member_curve_idx ON ec.ml_dataset_member (curve_id);
CREATE INDEX ml_dataset_member_factor_base_idx ON ec.ml_dataset_member (factor_base_id);

CREATE FUNCTION ec.validate_ml_dataset_member_subject() RETURNS trigger
LANGUAGE plpgsql AS $$
DECLARE expected text;
BEGIN
  SELECT subject_scope INTO expected FROM ec.ml_dataset WHERE id = NEW.dataset_id;
  IF expected = 'curve' AND NEW.curve_id IS NULL OR
     expected = 'factor_base' AND NEW.factor_base_id IS NULL OR
     expected = 'factor_base_measurement' AND NEW.factor_base_measurement_id IS NULL THEN
    RAISE EXCEPTION 'dataset % expects subject scope %, but member % has another subject',
      NEW.dataset_id, expected, NEW.sample_key;
  END IF;
  RETURN NEW;
END $$;
CREATE TRIGGER ml_dataset_member_subject
  BEFORE INSERT OR UPDATE ON ec.ml_dataset_member
  FOR EACH ROW EXECUTE FUNCTION ec.validate_ml_dataset_member_subject();

CREATE VIEW ec.v_curve_ml_features AS
SELECT
  s.id AS curve_id,
  s.icv1_slug,
  s.family,
  s.genus,
  s.model,
  s.field_id,
  s.field_kind,
  s.characteristic,
  s.field_degree,
  s.field_bits,
  s.isogeny_class_id,
  s.isogeny_class_label,
  s.trace,
  s.trace_ratio,
  s.group_order_bits,
  ic.is_ordinary,
  s.is_supersingular,
  s.is_anomalous,
  s.frobenius_discriminant,
  ec.bits(s.frobenius_discriminant) AS frobenius_discriminant_bits,
  s.cm_discriminant,
  s.conductor_v AS frobenius_conductor,
  s.conductor_bits,
  fr.conductor_fraction,
  s.class_number,
  g1.j_is_0,
  g1.j_is_1728,
  g1.j_field_degree,
  er.kind AS endomorphism_kind,
  s.end_discriminant AS endomorphism_discriminant,
  ec.bits(s.end_discriminant) AS endomorphism_discriminant_bits,
  s.end_conductor AS endomorphism_conductor,
  s.end_is_maximal AS endomorphism_is_maximal,
  er.status AS endomorphism_status,
  ocg.class_number AS endomorphism_class_number,
  ocg.two_rank AS endomorphism_class_group_two_rank,
  cardinality(ocg.class_group) AS endomorphism_class_group_rank,
  ag.group_order AS automorphism_group_order,
  ag.geometric_order AS geometric_automorphism_group_order,
  s.subgroup_id,
  s.subgroup_bits,
  s.cofactor AS subgroup_cofactor,
  s.embedding_degree,
  pd.target_field_bits,
  sa.rho_log2,
  sa.rho_log2_folded,
  s.twist_largest_prime_bits,
  (SELECT count(*) FROM ec.curve c2 JOIN ec.abelian_variety a2 ON a2.curve_id = c2.id
    WHERE a2.isogeny_class_id = s.isogeny_class_id) AS isogeny_class_curve_count,
  (SELECT count(*) FROM ec.curve_relation r
    WHERE r.source_curve_id = s.id OR r.target_curve_id = s.id) AS relation_count,
  (SELECT count(*) FROM ec.curve_relation r
    WHERE (r.source_curve_id = s.id OR r.target_curve_id = s.id) AND r.kind = 'isomorphism') AS isomorphism_relation_count,
  s.isogeny_count,
  (SELECT count(DISTINCT i.ell) FROM ec.isogeny i
    WHERE i.source_curve_id = s.id OR i.target_curve_id = s.id) AS isogeny_prime_count,
  (SELECT count(*) FROM ec.curve_morphism m
    WHERE m.source_curve_id = s.id OR m.target_curve_id = s.id) AS morphism_count,
  (SELECT count(*) FROM ec.endomorphism e WHERE e.curve_id = s.id) AS explicit_endomorphism_count,
  (SELECT count(*) FROM ec.endomorphism_level l WHERE l.curve_id = s.id) AS volcano_level_count,
  (SELECT count(*) FROM ec.factor_base fb WHERE fb.curve_id = s.id) AS factor_base_count,
  (SELECT count(*) FROM ec.v_factor_base_result r JOIN ec.factor_base fb ON fb.id = r.factor_base_id
    WHERE fb.curve_id = s.id) AS factor_base_result_count,
  (SELECT min(r.s) FILTER (WHERE r.solved) FROM ec.v_factor_base_result r
    JOIN ec.factor_base fb ON fb.id = r.factor_base_id WHERE fb.curve_id = s.id) AS best_s_solved,
  (SELECT count(*) FROM ec.feature_observation o WHERE o.curve_id = s.id) AS feature_observation_count,
  (SELECT max(t.ratio_log2) FROM ec.dlp_transfer t WHERE t.source_curve_id = s.id) AS max_transfer_ratio_log2
FROM ec.v_curve_summary s
LEFT JOIN ec.genus1_invariants g1 ON g1.curve_id = s.id
LEFT JOIN ec.isogeny_class_fq ic ON ic.id = s.isogeny_class_id
LEFT JOIN ec.frobenius_genus1 fr ON fr.isogeny_class_id = s.isogeny_class_id
LEFT JOIN ec.endomorphism_ring er ON er.curve_id = s.id
LEFT JOIN ec.order_class_group ocg ON ocg.discriminant = er.discriminant
LEFT JOIN ec.automorphism_group ag ON ag.curve_id = s.id
LEFT JOIN ec.pairing_data pd ON pd.subgroup_id = s.subgroup_id
LEFT JOIN LATERAL (
  SELECT x.* FROM ec.security_assessment x
   WHERE x.subgroup_id = s.subgroup_id ORDER BY x.id DESC LIMIT 1
) sa ON true;
COMMENT ON VIEW ec.v_curve_ml_features IS
  'One model-ready structural row per curve. NULL means missing or not applicable; use source fact statuses or observations before training.';

CREATE VIEW ec.v_factor_base_ml_features AS
SELECT
  fb.id AS factor_base_id,
  fb.key AS factor_base_key,
  fb.scheme,
  fb.family AS factor_base_family,
  fb.params,
  fb.curve_id,
  cv.icv1_slug,
  cv.field_kind,
  cv.field_bits,
  cv.family AS curve_family,
  cv.genus,
  fb.signed_points,
  fb.abscissae,
  fb.columns,
  fb.dimension,
  fb.effective_columns,
  fb.strict_points,
  fb.quotient_rule,
  s.results AS result_count,
  s.solved AS solved_count,
  s.best_s_solved,
  s.median_s_solved,
  s.max_relations,
  s.max_rank,
  s.best_yield_log2,
  s.measurements AS measurement_count,
  s.points_stored,
  (SELECT count(*) FROM ec.factor_base_artifact a WHERE a.factor_base_id = fb.id) AS artifact_count,
  EXISTS (SELECT 1 FROM ec.factor_base_artifact a JOIN ec.artifact x ON x.id = a.artifact_id
           WHERE a.factor_base_id = fb.id AND x.storage_uri IS NOT NULL) AS has_object_store_artifact,
  (SELECT x.storage_uri FROM ec.factor_base_artifact a JOIN ec.artifact x ON x.id = a.artifact_id
    WHERE a.factor_base_id = fb.id ORDER BY a.is_primary DESC, a.role, a.artifact_id LIMIT 1) AS primary_storage_uri,
  (SELECT sum(x.size_bytes) FROM ec.factor_base_artifact a JOIN ec.artifact x ON x.id = a.artifact_id
    WHERE a.factor_base_id = fb.id) AS artifact_bytes,
  (SELECT count(*) FROM ec.feature_observation o WHERE o.factor_base_id = fb.id) AS feature_observation_count
FROM ec.factor_base fb
JOIN ec.v_factor_base_summary s ON s.id = fb.id
LEFT JOIN ec.v_curve_ml_features cv ON cv.curve_id = fb.curve_id;
COMMENT ON VIEW ec.v_factor_base_ml_features IS
  'One model-ready row per factor base: construction and size, curve stratum, outcomes and object-store coverage.';

CREATE VIEW ec.v_factor_base_measurement_ml_features AS
SELECT
  m.id AS factor_base_measurement_id,
  m.fbm_id,
  m.factor_base_id,
  fb.factor_base_key,
  fb.factor_base_family,
  fb.curve_id,
  fb.icv1_slug,
  fb.field_kind,
  fb.field_bits,
  m.oracle,
  m.solver,
  m.linalg,
  m.instrument,
  m.config,
  count(r.*) AS result_count,
  count(*) FILTER (WHERE r.solved) AS solved_count,
  max(r.relations) AS max_relations,
  max(r.matrix_rank) AS max_matrix_rank,
  max(r.yield_log2) AS best_yield_log2,
  min(r.s) FILTER (WHERE r.solved) AS best_s_solved,
  min(r.total_gae) FILTER (WHERE r.solved) AS best_total_gae_solved,
  (SELECT max(x.solving_degree_mean) FROM ec.factor_base_result x WHERE x.measurement_id = m.id) AS solving_degree_mean_max,
  (SELECT max(x.solving_degree_max) FROM ec.factor_base_result x WHERE x.measurement_id = m.id) AS solving_degree_max,
  (SELECT max(x.macaulay_rows) FROM ec.factor_base_result x WHERE x.measurement_id = m.id) AS macaulay_rows_max,
  (SELECT max(x.macaulay_columns) FROM ec.factor_base_result x WHERE x.measurement_id = m.id) AS macaulay_columns_max,
  (SELECT count(*) FROM ec.feature_observation o WHERE o.factor_base_measurement_id = m.id) AS feature_observation_count
FROM ec.factor_base_measurement m
JOIN ec.v_factor_base_ml_features fb ON fb.factor_base_id = m.factor_base_id
LEFT JOIN ec.v_factor_base_result r ON r.measurement_id = m.id
GROUP BY m.id, fb.factor_base_key, fb.factor_base_family, fb.curve_id, fb.icv1_slug, fb.field_kind, fb.field_bits;
COMMENT ON VIEW ec.v_factor_base_measurement_ml_features IS
  'One model-ready row per FBM1 configuration with solver/oracle identity and recorded outcome summaries.';

ALTER FUNCTION ec.curve_document(bigint) RENAME TO curve_document_v8;
CREATE FUNCTION ec.curve_document(cid bigint) RETURNS jsonb LANGUAGE sql STABLE AS $$
SELECT CASE WHEN EXISTS (SELECT 1 FROM ec.curve c WHERE c.id = cid) THEN
  ec.curve_document_v8(cid) || jsonb_build_object(
    'ml_features',
      (SELECT to_jsonb(v) - 'curve_id' FROM ec.v_curve_ml_features v WHERE v.curve_id = cid),
    'factor_bases',
      (SELECT jsonb_agg((to_jsonb(fb) - 'record' - 'provenance_id') || jsonb_build_object(
         'artifacts', (SELECT jsonb_agg(jsonb_build_object(
           'role', a.role, 'is_primary', a.is_primary, 'description', a.description,
           'artifact', to_jsonb(x)) ORDER BY a.is_primary DESC, a.role, a.artifact_id)
           FROM ec.factor_base_artifact a JOIN ec.artifact x ON x.id = a.artifact_id
          WHERE a.factor_base_id = fb.id),
         'ml_features', (SELECT to_jsonb(v) - 'factor_base_id'
           FROM ec.v_factor_base_ml_features v WHERE v.factor_base_id = fb.id))
       ORDER BY fb.key) FROM ec.factor_base fb WHERE fb.curve_id = cid),
    'feature_observations',
      (SELECT jsonb_agg(jsonb_build_object(
         'feature_key', d.feature_key, 'version', d.version, 'value', o.value,
         'missing_reason', o.missing_reason, 'context', o.context,
         'status', o.status, 'observed_at', o.observed_at)
       ORDER BY d.feature_key, d.version, o.id)
       FROM ec.feature_observation o JOIN ec.feature_definition d ON d.id = o.feature_definition_id
       WHERE o.curve_id = cid),
    'sweep_memberships',
      (SELECT jsonb_agg(jsonb_build_object(
         'sweep', s.name, 'version', s.version, 'cell', cell.cell_key,
         'role', m.role, 'status', m.status, 'selection_order', m.selection_order)
       ORDER BY s.name, s.version, cell.ordinal)
       FROM ec.research_sweep_member m
       JOIN ec.research_sweep_cell cell ON cell.id = m.cell_id
       JOIN ec.research_sweep s ON s.id = cell.sweep_id
       WHERE m.curve_id = cid),
    'dataset_memberships',
      (SELECT jsonb_agg(jsonb_build_object(
         'dataset', d.name, 'version', d.version, 'sample_key', m.sample_key,
         'split', m.split, 'sample_weight', m.sample_weight, 'labels', m.labels)
       ORDER BY d.name, d.version)
       FROM ec.ml_dataset_member m JOIN ec.ml_dataset d ON d.id = m.dataset_id
       WHERE m.curve_id = cid)
  ) ELSE NULL END
$$;
COMMENT ON FUNCTION ec.curve_document(bigint) IS
  'Everything recorded about one curve, including model-ready features, factor-base object references, sweep membership and datasets.';

GRANT SELECT ON ec.research_sweep, ec.research_sweep_cell, ec.research_sweep_member,
  ec.factor_base_artifact, ec.feature_definition, ec.feature_observation,
  ec.ml_dataset, ec.ml_dataset_feature, ec.ml_dataset_member,
  ec.v_curve_ml_features, ec.v_factor_base_ml_features,
  ec.v_factor_base_measurement_ml_features TO ecdb_reader, ecdb_writer;
GRANT INSERT, UPDATE ON ec.research_sweep, ec.research_sweep_cell, ec.research_sweep_member,
  ec.factor_base_artifact, ec.feature_definition, ec.feature_observation,
  ec.ml_dataset, ec.ml_dataset_feature, ec.ml_dataset_member TO ecdb_writer;
GRANT USAGE ON ALL SEQUENCES IN SCHEMA ec TO ecdb_writer;
GRANT EXECUTE ON FUNCTION ec.curve_document(bigint), ec.curve_document_v8(bigint),
  ec.validate_feature_observation(), ec.validate_ml_dataset_member_subject()
  TO ecdb_reader, ecdb_writer;
