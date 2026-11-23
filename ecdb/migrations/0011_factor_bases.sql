-- 0011: factor bases of the index-calculus work, the measurement
-- configurations they were run under, and what each one yielded.
--
-- Identity follows the repository's own conventions (docs/ecbench/
-- FACTOR_BASE_PROTOCOL.md), so an id cited in a PR or a leaderboard row is
-- the key here:
--   fb1        FB1h<12 hex>: SHA-256 of the canonical {schema
--              "ecbench.factor_base/v1", curve, family, params, columns,
--              signed_points, points_sha256}. Names a point set.
--   fbarchive  the cryptanalysis archive's factor_base_sha256
--              (experiments/fb-archive/index.csv), keyed by EC1.
--   spec       FBS1h<12 hex>, minted by ecdb for a base known only by its
--              construction: SHA-256 of the canonical {schema
--              "ecdb.factor_base_spec/v1", curve, family, params}. Used for
--              result files that never materialised an FB1 id (large-prime
--              cells, SECG small-x). Two different point sets can share a
--              spec id only if the construction is not deterministic.
-- FBM1h<12 hex> names a measurement configuration (curve x FB1 x oracle x
-- solver x linalg x instrument); results are never part of either hash.
-- The importer recomputes both digests and records whether they matched.

CREATE TABLE ec.factor_base (
  id                    bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  scheme                text NOT NULL CHECK (scheme IN ('fb1', 'fbarchive', 'spec')),
  key                   text NOT NULL,             -- FB1h..., archive factor_base_sha256, FBS1h...
  digest                text,                      -- the full SHA-256 behind the key
  digest_verified       boolean,                   -- recomputed from the preimage; NULL when not recomputable
  curve_id              bigint REFERENCES ec.curve(id),
  curve_slug            text,                      -- ICV1 slug as the producer wrote it
  ec1                   text,
  curve_name            text,                      -- any other curve name the producer used (secp112r1, ...)
  family                text NOT NULL,             -- prime-abscissa, koblitz-orbit, binary-subspace, large-prime, ...
  params                jsonb NOT NULL DEFAULT '{}',
  description           text,
  signed_points         bigint,
  abscissae             bigint,
  columns               bigint,                    -- relation-matrix columns
  dimension             int,
  effective_columns     bigint,
  strict_points         bigint,
  quotient_rule         text,
  points_sha256         text,                      -- FB1: SHA-256 of the sorted big-endian point keys
  enumerated_set_sha256 text,                      -- fbarchive: digest of the enumerated point set
  store_path            text,                      -- store:factor-bases/<slug>/<FB1>.json or an archive path
  record                jsonb NOT NULL,
  provenance_id         bigint NOT NULL REFERENCES ec.provenance(id),
  UNIQUE (scheme, key)
);
CREATE INDEX factor_base_curve_idx ON ec.factor_base (curve_id);
CREATE INDEX factor_base_family_idx ON ec.factor_base (family);
CREATE INDEX factor_base_points_idx ON ec.factor_base (points_sha256);
CREATE INDEX factor_base_enumerated_idx ON ec.factor_base (enumerated_set_sha256);
COMMENT ON TABLE ec.factor_base IS 'A factor base of the IC work, under the id its producer gave it (FB1, the cryptanalysis archive digest) or an ecdb spec id (FBS1) when only its construction is known. digest_verified says whether ecdb recomputed the id from its preimage.';

CREATE TABLE ec.factor_base_alias (
  factor_base_id bigint NOT NULL REFERENCES ec.factor_base(id),
  kind           text NOT NULL,                    -- plug_in, catalog_id, ps1_label, archive_alias, file_sha256
  value          text NOT NULL,
  PRIMARY KEY (kind, value, factor_base_id)
);
CREATE INDEX factor_base_alias_value_idx ON ec.factor_base_alias (value);
COMMENT ON TABLE ec.factor_base_alias IS 'Other names of a factor base: the ecbench plug-in string, a catalog id, a PS1 label, a cited digest the archive reconciles.';

CREATE TABLE ec.factor_base_point (
  factor_base_id bigint NOT NULL REFERENCES ec.factor_base(id),
  idx            int NOT NULL,
  x              text NOT NULL,                    -- as dumped (decimal or 0x hex)
  y              text NOT NULL,
  col            int NOT NULL,                     -- relation-matrix column
  coef           text NOT NULL,                    -- coefficient of the point in its column
  PRIMARY KEY (factor_base_id, idx)
);
COMMENT ON TABLE ec.factor_base_point IS 'The points of a factor base, where a dump (ecbench fb) exists.';

CREATE TABLE ec.factor_base_measurement (
  id              bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  fbm_id          text NOT NULL UNIQUE,            -- FBM1h + 12 hex
  fbm_sha256      text,
  digest_verified boolean,
  factor_base_id  bigint NOT NULL REFERENCES ec.factor_base(id),
  curve_slug      text,
  oracle          text,
  solver          text,
  linalg          text,
  instrument      text,
  config          jsonb NOT NULL,
  provenance_id   bigint NOT NULL REFERENCES ec.provenance(id)
);
CREATE INDEX factor_base_measurement_fb_idx ON ec.factor_base_measurement (factor_base_id);
COMMENT ON TABLE ec.factor_base_measurement IS 'FBM1: one measurement configuration of a factor base (curve x FB1 x oracle x solver x linalg x instrument). Yield and degree of regularity are cited with both ids.';

-- Results that do not live in ec.ecbench_run. ecbench runs stay where they
-- are and are linked by ec.ecbench_run.factor_base_id / fbm_id;
-- ec.v_factor_base_result is the union of both.
CREATE TABLE ec.factor_base_result (
  id                  bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  factor_base_id      bigint NOT NULL REFERENCES ec.factor_base(id),
  measurement_id      bigint REFERENCES ec.factor_base_measurement(id),
  source              text NOT NULL CHECK (source IN ('large_prime_cell', 'secg_small_x', 'fbm_cell', 'other')),
  source_key          text NOT NULL,               -- repository path (and cell) of the result
  arm                 text,
  seed                bigint,
  status              text,
  solved              boolean,                     -- the planted / target log was recovered and verified
  trials              bigint,
  relations           bigint,
  yield_per_trial     double precision,            -- relations per decomposition trial (measured)
  yield_log2          double precision,            -- exact or model yield, log2, where the producer computed one
  matrix_rank         bigint,
  independent_rows    bigint,
  s                   double precision,
  total_gae           double precision,
  cpu_ns              bigint,
  solving_degree_mean double precision,
  solving_degree_max  int,
  semi_regular_degree int,
  macaulay_rows       bigint,
  macaulay_columns    bigint,
  counters            jsonb,
  extra               jsonb NOT NULL DEFAULT '{}',
  record              jsonb NOT NULL,
  provenance_id       bigint NOT NULL REFERENCES ec.provenance(id),
  UNIQUE (source, source_key)
);
CREATE INDEX factor_base_result_fb_idx ON ec.factor_base_result (factor_base_id);
COMMENT ON TABLE ec.factor_base_result IS 'A measured result of a factor base outside ecbench (large-prime receipts, SECG small-x yield). Kept as the producer reported it; nothing is re-priced.';

ALTER TABLE ec.ecbench_run ADD COLUMN IF NOT EXISTS factor_base_id bigint REFERENCES ec.factor_base(id);
ALTER TABLE ec.ecbench_run ADD COLUMN IF NOT EXISTS fb_id text;
ALTER TABLE ec.ecbench_run ADD COLUMN IF NOT EXISTS fbm_id text;
CREATE INDEX IF NOT EXISTS ecbench_run_factor_base_idx ON ec.ecbench_run (factor_base_id);
ALTER TABLE ec.ic_board_row ADD COLUMN IF NOT EXISTS factor_base_id bigint REFERENCES ec.factor_base(id);

CREATE VIEW ec.v_factor_base_result AS
SELECT 'ecbench_run'::text AS source, r.record_id AS source_key, r.factor_base_id,
       m.id AS measurement_id, r.fbm_id, r.session_id, r.arm, r.outcome AS status,
       (r.outcome = 'verified' AND r.matches_target IS TRUE) AS solved,
       COALESCE((r.counters->>'targets_tried')::bigint, (r.counters->>'target_attempts')::bigint) AS trials,
       (r.counters->>'relations_found')::bigint AS relations,
       NULL::double precision AS yield_log2,
       (r.counters->>'matrix_rank')::bigint AS matrix_rank,
       r.s, r.total_gae, r.counters
  FROM ec.ecbench_run r
  LEFT JOIN ec.factor_base_measurement m ON m.fbm_id = r.fbm_id
 WHERE r.factor_base_id IS NOT NULL
UNION ALL
SELECT x.source, x.source_key, x.factor_base_id, x.measurement_id, m.fbm_id, NULL, x.arm, x.status, x.solved,
       x.trials, x.relations, x.yield_log2, x.matrix_rank, x.s, x.total_gae, x.counters
  FROM ec.factor_base_result x
  LEFT JOIN ec.factor_base_measurement m ON m.id = x.measurement_id;
COMMENT ON VIEW ec.v_factor_base_result IS 'Every measured result of a factor base: ecbench runs (linked through FB1/FBM1) and the other result files.';

CREATE VIEW ec.v_factor_base_summary AS
SELECT fb.id, fb.scheme, fb.key, fb.digest_verified, fb.curve_id, c.icv1_slug, fb.curve_name, fb.family, fb.params,
       fb.columns, fb.signed_points, fb.dimension,
       count(r.*) AS results,
       count(*) FILTER (WHERE r.solved) AS solved,
       min(r.s) FILTER (WHERE r.solved) AS best_s_solved,
       percentile_cont(0.5) WITHIN GROUP (ORDER BY r.s) FILTER (WHERE r.solved) AS median_s_solved,
       max(r.relations) AS max_relations,
       max(r.matrix_rank) AS max_rank,
       max(r.yield_log2) AS best_yield_log2,
       (SELECT count(*) FROM ec.factor_base_measurement m WHERE m.factor_base_id = fb.id) AS measurements,
       (SELECT count(*) FROM ec.factor_base_point p WHERE p.factor_base_id = fb.id) AS points_stored
  FROM ec.factor_base fb
  LEFT JOIN ec.curve c ON c.id = fb.curve_id
  LEFT JOIN ec.v_factor_base_result r ON r.factor_base_id = fb.id
 GROUP BY fb.id, c.icv1_slug;
COMMENT ON VIEW ec.v_factor_base_summary IS 'One row per factor base: its construction and size, how often it was measured, and its best solved cost (S, the same unit as the leaderboard) and largest rank.';

GRANT SELECT ON ec.factor_base, ec.factor_base_alias, ec.factor_base_point, ec.factor_base_measurement,
                ec.factor_base_result, ec.v_factor_base_result, ec.v_factor_base_summary TO ecdb_reader, ecdb_writer;
