-- 0009: measured results and the sibling repositories' records.
--
-- Measurements are kept as their producers recorded them: each table names
-- its unit, its reference and its source file, and a ratio is compared only
-- inside one family / unit (the leaderboard's own rule). Nothing here is
-- recomputed or re-ranked; a row's validation status is the producer's.
-- A curve link (curve_id) is set only when the record's ICV1 slug, EC1 alias
-- or curve_uid resolves exactly; otherwise it stays NULL and the identifier
-- is kept as text.

-- ------------------------------------------------- IC leaderboard (docs/ic/leaderboard.json)

CREATE TABLE ec.ic_family (
  code          text PRIMARY KEY,           -- A, B, C
  title         text NOT NULL,
  section       text,
  status        text,
  targets       text,
  unit          text,
  unit_how      text,
  reference     text,
  floor         text,
  unit_short    text,
  ref_short     text,
  regimes       text[],
  record        jsonb NOT NULL,
  provenance_id bigint NOT NULL REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.ic_family IS 'A table of the IC leaderboard (A Koblitz one target, B prime / random binary ladder, C historical 32-target batch). Ratios compare only within one family.';

CREATE TABLE ec.ic_board_row (
  id                 bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  family             text NOT NULL REFERENCES ec.ic_family(code),
  regime             text NOT NULL,
  curve_slug         text NOT NULL,
  curve_id           bigint REFERENCES ec.curve(id),
  ec1                text,
  legacy_name        text,
  log2_r             double precision,
  cofactor           numeric,
  reference          text,
  reference_s        double precision,
  reference_mean_total_gae double precision,
  floor_s            double precision,
  targets            text,
  operation_unit     text,
  round              text,
  class              text,
  best_variant       text,
  best_label         text,
  best_m             int,
  best_base_points   bigint,
  best_s             double precision,
  best_ratio_rho     double precision,
  best_ratio_rho_ci  double precision[],
  best_ratio_floor   double precision,
  best_mean_total_gae double precision,
  validation_status  text,
  validation         jsonb,
  factor_base        jsonb,
  curve_record       jsonb,
  source_path        text,
  record             jsonb NOT NULL,
  provenance_id      bigint NOT NULL REFERENCES ec.provenance(id),
  UNIQUE (family, regime, curve_slug)
);
CREATE INDEX ic_board_row_curve_idx ON ec.ic_board_row (curve_id);
COMMENT ON TABLE ec.ic_board_row IS 'One priced curve of the IC leaderboard: the best recipe''s cold cost against the matched rho reference of its family. best_ratio_rho is IC / rho (lower is closer; 1 means equal). Not a speedup unless below 1 and validated.';
COMMENT ON COLUMN ec.ic_board_row.validation_status IS 'The validation gate''s standing: validated, or unvalidated (pre-gate).';

CREATE TABLE ec.ic_recipe (
  board_row_id   bigint NOT NULL REFERENCES ec.ic_board_row(id),
  variant        text NOT NULL,
  label          text,
  m              int,
  base_points    bigint,
  s              double precision,
  s_online       double precision,
  s_setup        double precision,
  ratio_rho      double precision,
  ratio_rho_ci   double precision[],
  ratio_floor    double precision,
  online_speedup double precision,
  verified       boolean,
  repetitions    int,
  mean_total_gae double precision,
  phases_s       jsonb,
  is_best        boolean NOT NULL DEFAULT false,
  record         jsonb NOT NULL,
  PRIMARY KEY (board_row_id, variant)
);
COMMENT ON TABLE ec.ic_recipe IS 'Every recipe priced on a leaderboard curve, with its phase split (S per phase).';

CREATE TABLE ec.ic_leaderboard_section (
  key           text PRIMARY KEY,
  record        jsonb NOT NULL,
  provenance_id bigint NOT NULL REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.ic_leaderboard_section IS 'The leaderboard''s other sections verbatim (leaders, diagnostics, audits, tournament pilot), keyed by their JSON key.';

-- ------------------------------------------------- progress timeline (docs/ic/progress-timeline.json)

CREATE TABLE ec.progress_series (
  id            text PRIMARY KEY,
  label         text NOT NULL,
  note          text,
  barrier_test  boolean,
  record        jsonb NOT NULL,
  provenance_id bigint NOT NULL REFERENCES ec.provenance(id)
);

CREATE TABLE ec.progress_point (
  id          bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  series_id   text NOT NULL REFERENCES ec.progress_series(id),
  date        date,
  label       text NOT NULL,
  value       double precision,
  reference   text,
  class       text,
  detail      text,
  source      text,
  anchor      text,
  record      jsonb NOT NULL,
  UNIQUE NULLS NOT DISTINCT (series_id, date, label)
);
COMMENT ON TABLE ec.progress_point IS 'A landed result on the progress timeline: IC / rho in its own unit; reference says whether the rho was matched.';

-- ------------------------------------------------- boundary ledger (docs/ic/boundary_targets.json)

CREATE TABLE ec.boundary_record (
  regime        text NOT NULL,
  stage         text NOT NULL,
  status        text,
  summary       text,
  metrics       jsonb,
  next_target   jsonb,
  evidence      text[],
  history       jsonb,
  regime_record jsonb,
  provenance_id bigint NOT NULL REFERENCES ec.provenance(id),
  PRIMARY KEY (regime, stage)
);
COMMENT ON TABLE ec.boundary_record IS 'The boundary ledger: per regime (binary, koblitz, prime) and stage (factor_base ... vs_rho), the current record and the next target with its acceptance gates.';

-- ------------------------------------------------- benchmarks/ index

CREATE TABLE ec.benchmark_entry (
  dir           text PRIMARY KEY,           -- benchmarks/<YYYYMMDD>-<slug>
  date          date,
  slug          text,
  listed        text NOT NULL,              -- admitted | archive | unlisted
  readme_row    text,
  validation    jsonb,                      -- VALIDATION.json when present
  copies        jsonb,                      -- copies.json when present
  provenance_id bigint NOT NULL REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.benchmark_entry IS 'A benchmarks/ directory: its row in benchmarks/README.md (admitted) or ARCHIVE.md, and its VALIDATION.json / copies.json.';

-- ------------------------------------------------- ecbench sessions and runs

CREATE TABLE ec.ecbench_session (
  session_id        text PRIMARY KEY,
  path              text NOT NULL,          -- repository-relative session directory
  repository        text NOT NULL,
  schema            text,
  label             text,
  spec_id           text,
  spec_sha256       text,
  git_commit        text,
  git_dirty         boolean,
  binary_sha256     text,
  host_sha256       text,
  env_class_id      text,
  status            text,
  started_at        timestamptz,
  finished_at       timestamptz,
  executions_planned int,
  records_written   int,
  records_sha256    text,
  status_counts     jsonb,
  audit             jsonb,                  -- summary of audit-replay-all.json when present
  record            jsonb NOT NULL,
  provenance_id     bigint NOT NULL REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.ecbench_session IS 'A sealed ecbench session (session.json). Its runs are in ecbench_run; audit summarises the replay audit when the directory has one.';

CREATE TABLE ec.ecbench_run (
  record_id       text PRIMARY KEY,
  session_id      text NOT NULL REFERENCES ec.ecbench_session(session_id),
  run_id          text,
  seq             int,
  round           int,
  warmup          boolean,
  arm             text,
  role            text,
  method_id       text,
  method          text,
  method_family   text,
  method_params   jsonb,
  workload_id     text,
  curve_id        bigint REFERENCES ec.curve(id),
  curve_slug      text,
  ec1             text,
  curve_uid       text,
  r               numeric,
  target_index    int,
  outcome         text,
  matches_target  boolean,
  cost_unit       text,
  total_gae       double precision,
  s               double precision,
  lower_bound     boolean,
  unpriced        text[],
  floor_s         double precision,
  ratio_to_floor  double precision,
  phases          jsonb,
  counters        jsonb
);
CREATE INDEX ecbench_run_session_idx ON ec.ecbench_run (session_id);
CREATE INDEX ecbench_run_curve_idx ON ec.ecbench_run (curve_id);
CREATE INDEX ecbench_run_method_idx ON ec.ecbench_run (method);
COMMENT ON TABLE ec.ecbench_run IS 'One ecbench execution: method, workload (curve and target), outcome, and cost in GAE with S = cost / sqrt(r). lower_bound means some native work is unpriced (listed in unpriced), so compare arms only across the AGENTS.md 5a band.';

CREATE TABLE ec.ecbench_comparison (
  comparison_id text NOT NULL,
  session_id    text NOT NULL REFERENCES ec.ecbench_session(session_id),
  path          text NOT NULL,
  a             jsonb,
  b             jsonb,
  record        jsonb NOT NULL,
  PRIMARY KEY (comparison_id, session_id)
);
COMMENT ON TABLE ec.ecbench_comparison IS 'A paired comparison of two arms written by ecbench compare (comparisons/*.json in the session directory).';

CREATE VIEW ec.v_ecbench_arm AS
SELECT r.session_id, s.label AS session_label, s.path AS session_path, r.arm, r.role, r.method, r.method_family,
       r.curve_slug, r.curve_id, r.r,
       count(*)                                        AS runs,
       count(*) FILTER (WHERE r.outcome = 'verified')  AS verified,
       avg(r.s)                                        AS mean_s,
       stddev_samp(r.s) / nullif(sqrt(count(r.s)), 0)  AS sem_s,
       avg(r.ratio_to_floor)                           AS mean_ratio_to_floor,
       bool_or(r.lower_bound)                          AS lower_bound
FROM ec.ecbench_run r JOIN ec.ecbench_session s ON s.session_id = r.session_id
WHERE NOT coalesce(r.warmup, false)
GROUP BY r.session_id, s.label, s.path, r.arm, r.role, r.method, r.method_family, r.curve_slug, r.curve_id, r.r;
COMMENT ON VIEW ec.v_ecbench_arm IS 'Per session, arm and curve: runs, verified runs, mean S (with its standard error) and mean ratio to the floor; warm-up runs excluded.';

-- ------------------------------------------------- sibling repositories

CREATE TABLE ec.external_curve_record (
  id            bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  repository    text NOT NULL,
  path          text NOT NULL,
  record_key    text NOT NULL,
  kind          text NOT NULL,              -- ic_catalog, curve_capsule, traits_snapshot, factor_base, dataset_curve, ...
  icv1_slug     text,
  ec1           text,
  curve_uid     text,
  curve_id      bigint REFERENCES ec.curve(id),
  match         text NOT NULL CHECK (match IN ('ec1', 'curve_uid', 'icv1_slug', 'computed_icv1', 'standard_name', 'none')),
  record        jsonb NOT NULL,
  provenance_id bigint NOT NULL REFERENCES ec.provenance(id),
  UNIQUE (repository, path, record_key)
);
CREATE INDEX external_curve_record_curve_idx ON ec.external_curve_record (curve_id);
COMMENT ON TABLE ec.external_curve_record IS 'A curve or factor-base record from another repository (cryptanalysis, crypto-autoresearcher, cryptanalysis-rl, ml-cryptanalysis), verbatim, with how it was matched to an ecdb curve. match = none keeps the record unlinked: same j or same order is never a match.';

CREATE TABLE ec.measurement (
  id            bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  repository    text NOT NULL,
  dataset       text NOT NULL,              -- repository-relative path of the producing file
  record_key    text NOT NULL,
  curve_id      bigint REFERENCES ec.curve(id),
  curve_slug    text,
  method        text NOT NULL,
  metric        text NOT NULL,
  unit          text,
  value         double precision,
  sem           double precision,
  n             int,
  status        text,
  record        jsonb,
  provenance_id bigint NOT NULL REFERENCES ec.provenance(id),
  UNIQUE (repository, dataset, record_key, method, metric)
);
CREATE INDEX measurement_curve_idx ON ec.measurement (curve_id);
COMMENT ON TABLE ec.measurement IS 'A measured quantity from a sibling dataset (e.g. mean rho iterations of a curve, group operations of a tool-policy arm), in its producer''s unit, with its sample size. Comparable only within one dataset and metric.';

GRANT SELECT ON ALL TABLES IN SCHEMA ec TO ecdb_reader, ecdb_writer;
GRANT INSERT, UPDATE ON ALL TABLES IN SCHEMA ec TO ecdb_writer;
