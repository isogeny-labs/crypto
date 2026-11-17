-- ecbench database schema, version 1 (SQLite 3.24+; PostgreSQL with minor
-- type changes).
--
-- This database is an INDEX, rebuilt from committed session directories by
--   ecbench db sql <session dirs...> | sqlite3 ecbench.db
-- It is never the record of a measurement: the session directory (sealed
-- records, host capsule, hashes) is, and `ecbench verify` audits it.  Every
-- statement `ecbench db sql` emits is idempotent, so loading a session twice
-- changes nothing.
--
-- Conventions
--   * Curves are keyed by ICV1 slug (AGENTS.md §11, docs/curves/ICV1.md).
--     EC1 aliases and curve UIDs are joined per representation, never inferred.
--   * 64-bit unsigned values that may exceed 2^63 (seeds, planted scalars,
--     group orders) are TEXT in decimal; counts that cannot are INTEGER.
--   * S = total group-addition equivalents / sqrt(r), over the whole method,
--     cold, one target.  A row whose `lower_bound` is 1 left work unpriced.
--   * Only runs with status = 'verified' and warmup = 0 are results, and
--     only runs of a `complete` session: the views below read nothing from
--     an `interrupted` or `running` one (a prefix of its plan, however
--     favourable), which stays in the tables with its status.
--   * `ecbench db sql` audits a session directory before it emits a row of
--     it, so a tampered or unsealed record never reaches this index; the
--     identity guards below turn a row that disagrees with the one already
--     loaded into an error.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS schema_info (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

-- ── What is measured on ──────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS curves (
  slug                    TEXT PRIMARY KEY,           -- icv1-...
  icv1                    TEXT NOT NULL UNIQUE,       -- full ICV1 string
  family                  TEXT NOT NULL CHECK (family IN ('prime', 'koblitz', 'binary', 'extension', 'fp4', 'hyperelliptic')),
  field_degree            INTEGER,                    -- m for GF(2^m)
  field_bits              INTEGER,                    -- bits of p for GF(p)
  group_order             TEXT NOT NULL,
  r                       TEXT NOT NULL,              -- prime subgroup order
  cofactor                TEXT NOT NULL,
  log2_r                  REAL NOT NULL,
  automorphisms_available INTEGER NOT NULL,           -- 2, or 2m on a Koblitz curve
  floor_s                 REAL NOT NULL,              -- sqrt(pi / 2A)
  registered              INTEGER NOT NULL CHECK (registered IN (0, 1))
);

-- One generator of one subgroup: the exact representation measured.
CREATE TABLE IF NOT EXISTS curve_representations (
  slug        TEXT NOT NULL REFERENCES curves(slug),
  generator_x TEXT NOT NULL,
  generator_y TEXT NOT NULL,
  ec1         TEXT,                                   -- registry EC1 alias, when it has this one
  curve_uid   TEXT,
  PRIMARY KEY (slug, generator_x, generator_y)
);

-- The constructor calls that produced a curve (a search, the roster, explicit).
CREATE TABLE IF NOT EXISTS curve_constructions (
  slug         TEXT NOT NULL REFERENCES curves(slug),
  construction TEXT NOT NULL,
  PRIMARY KEY (slug, construction)
);

-- One single-target workload: curve, generator, target point.
CREATE TABLE IF NOT EXISTS workloads (
  workload_id     TEXT PRIMARY KEY,                   -- W + 12 hex
  workload_sha256 TEXT NOT NULL UNIQUE,
  curve_slug      TEXT NOT NULL REFERENCES curves(slug),
  generator_x     TEXT NOT NULL,
  generator_y     TEXT NOT NULL,
  target_x        TEXT NOT NULL,
  target_y        TEXT NOT NULL,
  target_law      TEXT NOT NULL,
  target_seed     TEXT NOT NULL,
  target_index    INTEGER NOT NULL,
  planted         TEXT                                -- the known answer; NULL for a public target
);

-- ── What is measured ─────────────────────────────────────────────────

-- An algorithm configuration: a registry method with its parameters,
-- defaults written out.  Rho, BSGS, kangaroo and index calculus share it.
CREATE TABLE IF NOT EXISTS algorithms (
  method_id     TEXT PRIMARY KEY,                     -- ECM1h + 12 hex
  method_sha256 TEXT NOT NULL UNIQUE,
  method        TEXT NOT NULL,                        -- e.g. rho.negation, ic.pipeline
  family        TEXT NOT NULL CHECK (family IN ('rho', 'bsgs', 'kangaroo', 'claw', 'ic')),
  params_json   TEXT NOT NULL,
  entry         TEXT NOT NULL                         -- the code path it runs
);

-- A factor base, identified by its points.
CREATE TABLE IF NOT EXISTS factor_bases (
  fb_id         TEXT PRIMARY KEY,                     -- FB1h + 12 hex
  fb_sha256     TEXT NOT NULL UNIQUE,
  curve_slug    TEXT NOT NULL REFERENCES curves(slug),
  family        TEXT NOT NULL,                        -- prime-abscissa, koblitz-orbit, ...
  params_json   TEXT NOT NULL,
  description   TEXT,
  signed_points INTEGER NOT NULL,
  abscissae     INTEGER NOT NULL,
  columns       INTEGER NOT NULL,
  dimension     INTEGER,
  points_sha256 TEXT NOT NULL                         -- sha256 of the sorted point keys
);

-- The points themselves, when a factor-base dump (`ecbench fb`) is loaded.
CREATE TABLE IF NOT EXISTS factor_base_points (
  fb_id  TEXT NOT NULL REFERENCES factor_bases(fb_id),
  idx    INTEGER NOT NULL,
  x      TEXT NOT NULL,
  y      TEXT NOT NULL,
  col    INTEGER NOT NULL,                            -- relation-matrix column
  coef   TEXT NOT NULL,                               -- coefficient of the point in its column
  PRIMARY KEY (fb_id, idx)
);

-- FBM1: measurement configuration identity (curve × FB1 × oracle × solver ×
-- linalg × instrument). Results attach to cells; they are never part of the hash.
CREATE TABLE IF NOT EXISTS factor_base_measurements (
  fbm_id        TEXT PRIMARY KEY,                     -- FBM1h + 12 hex
  fbm_sha256    TEXT NOT NULL UNIQUE,
  curve_slug    TEXT NOT NULL REFERENCES curves(slug),
  fb_id         TEXT NOT NULL REFERENCES factor_bases(fb_id),
  fb_sha256     TEXT NOT NULL,
  family        TEXT NOT NULL,
  fb_params_json TEXT NOT NULL,
  oracle        TEXT NOT NULL,
  solver        TEXT NOT NULL,
  linalg        TEXT NOT NULL,
  instrument    TEXT NOT NULL                          -- e.g. ecbench.ic_pipeline/v1, sage.factor_base_dor/v1
);

-- ── Where and how ────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS hosts (
  env_class_id     TEXT PRIMARY KEY,                  -- ECBENV2h (or v1 ECBENV1h) + 12 hex of the stable facts
  env_class_sha256 TEXT NOT NULL UNIQUE,
  os               TEXT NOT NULL,
  arch             TEXT NOT NULL,
  kernel_release   TEXT,
  cpu_model        TEXT,
  features         TEXT NOT NULL,                     -- space-separated
  logical_cpus     INTEGER NOT NULL,
  physical_cores   INTEGER,
  numa_nodes       INTEGER NOT NULL,
  virtual          INTEGER,                           -- 1, 0, or NULL when unknown
  rustc            TEXT,
  capsule_json     TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
  session_id       TEXT PRIMARY KEY,                  -- ECBS1h + 12 hex
  label            TEXT NOT NULL,
  spec_id          TEXT NOT NULL,                     -- ECS1h + 12 hex
  spec_sha256      TEXT NOT NULL,
  env_class_id     TEXT NOT NULL REFERENCES hosts(env_class_id),
  git_commit       TEXT,
  git_dirty        INTEGER,
  binary_sha256    TEXT,
  cpu_request      TEXT NOT NULL,
  run_cpu          INTEGER,
  reserved_cpus    TEXT,
  numa_node        INTEGER,
  preflight_quiet  INTEGER,
  started_unix_ms  INTEGER NOT NULL,
  finished_unix_ms INTEGER,
  status           TEXT NOT NULL,                     -- complete | interrupted | running
  records_sha256   TEXT,
  path             TEXT NOT NULL                      -- where the session directory was loaded from
);

CREATE TABLE IF NOT EXISTS arms (
  session_id TEXT NOT NULL REFERENCES sessions(session_id),
  arm        TEXT NOT NULL,
  role       TEXT NOT NULL CHECK (role IN ('reference', 'baseline', 'candidate', 'control')),
  method_id  TEXT NOT NULL REFERENCES algorithms(method_id),
  PRIMARY KEY (session_id, arm)
);

-- ── What happened ────────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS runs (
  record_id            TEXT PRIMARY KEY,              -- ECR1h + 12 hex, seals the record
  session_id           TEXT NOT NULL REFERENCES sessions(session_id),
  run_id               TEXT NOT NULL,                 -- <method_id><workload_id>R<n>
  seq                  INTEGER NOT NULL,
  round                INTEGER NOT NULL,
  warmup               INTEGER NOT NULL CHECK (warmup IN (0, 1)),
  arm                  TEXT NOT NULL,
  method_id            TEXT NOT NULL REFERENCES algorithms(method_id),
  workload_id          TEXT NOT NULL REFERENCES workloads(workload_id),
  fb_id                TEXT REFERENCES factor_bases(fb_id),
  fbm_id               TEXT REFERENCES factor_base_measurements(fbm_id),
  algorithm_seed       TEXT NOT NULL,
  status               TEXT NOT NULL CHECK (status IN ('verified', 'wrong_answer', 'exhausted', 'error', 'timeout', 'crashed', 'interrupted', 'spawn_error')),
  recovered            TEXT,
  total_gae            REAL,
  s                    REAL,
  ratio_to_floor       REAL,
  automorphisms_used   INTEGER,
  lower_bound          INTEGER NOT NULL,
  unpriced             TEXT NOT NULL,                 -- space-separated counter names
  deterministic        INTEGER NOT NULL,
  solve_wall_ns        INTEGER,
  process_wall_ns      INTEGER NOT NULL,
  user_ns              INTEGER NOT NULL,
  sys_ns               INTEGER NOT NULL,
  max_rss_kib          INTEGER NOT NULL,
  involuntary_switches INTEGER NOT NULL,
  run_delay_ns         INTEGER,
  isolation_level      TEXT NOT NULL CHECK (isolation_level IN ('L0', 'L1', 'L2', 'L3')),
  foreign_busy_ticks   INTEGER,
  steal_ticks          INTEGER,
  error                TEXT,
  UNIQUE (session_id, seq),
  UNIQUE (session_id, run_id)
);

CREATE TABLE IF NOT EXISTS phases (
  record_id     TEXT NOT NULL REFERENCES runs(record_id),
  ord           INTEGER NOT NULL,
  phase         TEXT NOT NULL,
  adds          INTEGER NOT NULL,
  doubles       INTEGER NOT NULL,
  scalar_mults  INTEGER NOT NULL,
  gae           REAL NOT NULL,
  wall_ns       INTEGER,
  native_json   TEXT NOT NULL,
  PRIMARY KEY (record_id, ord)
);

CREATE TABLE IF NOT EXISTS run_counters (
  record_id TEXT NOT NULL REFERENCES runs(record_id),
  name      TEXT NOT NULL,
  value     INTEGER NOT NULL,
  PRIMARY KEY (record_id, name)
);

-- The decomposition solver's statistics for an algebraic or SAT
-- index-calculus run (the record's `solver` block; ICMS pdp_metrics).
-- Absent for table oracles and generic methods.
CREATE TABLE IF NOT EXISTS run_solver (
  record_id            TEXT PRIMARY KEY REFERENCES runs(record_id),
  name                 TEXT NOT NULL,
  calls                INTEGER NOT NULL,
  ops                  INTEGER NOT NULL,
  op_unit              TEXT NOT NULL,
  wall_ns              INTEGER NOT NULL,
  budget_exceeded      INTEGER NOT NULL,
  n_vars               INTEGER,
  n_equations          INTEGER,
  semi_regular_degree  INTEGER,
  solving_degree_mean  REAL,
  solving_degree_max   INTEGER,
  macaulay_rows        INTEGER,
  macaulay_columns     INTEGER,
  macaulay_degree      INTEGER,
  macaulay_rank        INTEGER,
  sat_variables        INTEGER,
  sat_clauses          INTEGER,
  sat_conflicts        INTEGER,
  sat_decisions        INTEGER,
  sat_propagations     INTEGER,
  extra_json           TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS isolation_blockers (
  record_id TEXT NOT NULL REFERENCES runs(record_id),
  ord       INTEGER NOT NULL,
  blocker   TEXT NOT NULL,                            -- "L2: 3 ticks of hypervisor steal ..."
  PRIMARY KEY (record_id, ord)
);

-- One measured cell under an FBM1 id (yield / DoR / cost). Source may be an
-- ecbench record or a Sage DoR join stamped with the same FBM1.
CREATE TABLE IF NOT EXISTS factor_base_measurement_cells (
  cell_id              TEXT PRIMARY KEY,              -- optional stable id; else fbm_id||':'||record_id
  fbm_id               TEXT NOT NULL REFERENCES factor_base_measurements(fbm_id),
  record_id            TEXT REFERENCES runs(record_id),
  session_id           TEXT,
  s                    REAL,
  total_gae            REAL,
  trials               INTEGER,
  relations            INTEGER,
  yield_per_trial      REAL,
  solving_degree_mean  REAL,
  solving_degree_max   INTEGER,
  semi_regular_degree  INTEGER,
  macaulay_rows        INTEGER,
  macaulay_columns     INTEGER,
  status               TEXT,
  extra_json           TEXT NOT NULL DEFAULT '{}'
);

CREATE TABLE IF NOT EXISTS comparisons (
  comparison_id     TEXT PRIMARY KEY,                 -- ECC1h + 12 hex
  session_a         TEXT NOT NULL,
  arm_a             TEXT NOT NULL,
  session_b         TEXT NOT NULL,
  arm_b             TEXT NOT NULL,
  ops_status        TEXT NOT NULL,                    -- ok | incomplete | empty
  ops_ratio         REAL,
  ops_ci_low        REAL,
  ops_ci_high       REAL,
  ops_bounded       INTEGER NOT NULL,
  wall_status       TEXT NOT NULL,                    -- admitted | descriptive | refused
  wall_ratio        REAL,
  wall_ci_low       REAL,
  wall_ci_high      REAL,
  verdict           TEXT NOT NULL,
  json              TEXT NOT NULL
);

-- vs_rho claims (ecbench claim build): one IC run against one strong-rho
-- run on one public target, keyed by the repository's IC1 run id.  A
-- claim rebuilt with an independent receipt replaces its earlier form;
-- the identity trigger keeps the run id on one candidate, workload and
-- session.
CREATE TABLE IF NOT EXISTS claims (
  run_id                     TEXT PRIMARY KEY,                -- <candidate_id>W<workload_id>R<n>
  candidate_id               TEXT NOT NULL,                   -- IC1 label
  candidate_manifest_sha256  TEXT NOT NULL,
  claim_workload_id          TEXT NOT NULL,                   -- IC1 workload id (12 hex), not ecbench's W…
  workload_manifest_sha256   TEXT NOT NULL,
  session_id                 TEXT NOT NULL,
  ic_record_id               TEXT NOT NULL,
  rho_record_id              TEXT NOT NULL,
  n_or_bits                  INTEGER NOT NULL,
  ic_online_wall_ms          REAL NOT NULL,
  rho_online_wall_ms         REAL NOT NULL,
  online_speedup             REAL NOT NULL,                   -- rho / IC
  independent_validation     INTEGER NOT NULL,
  replay_certificate_sha256  TEXT,                            -- NULL until a receipt is cited
  check_status               TEXT NOT NULL,                   -- PASS | FAIL, the vs_rho checker at load time
  verdict                    TEXT NOT NULL,
  json                       TEXT NOT NULL
);

-- ── Identity guards ──────────────────────────────────────────────────
--
-- An insert tolerates a conflict on its primary key only when the row is
-- the same row again.  These triggers turn a key that arrives with a
-- different identity (a slug on another ICV1 string, a workload id on
-- another target, a method id on other parameters) into an error instead
-- of a silently kept first row.  They fire before conflict resolution, so
-- a true duplicate still passes to `ON CONFLICT ... DO NOTHING`.

CREATE TRIGGER IF NOT EXISTS curves_identity BEFORE INSERT ON curves
WHEN EXISTS (SELECT 1 FROM curves WHERE slug = NEW.slug AND icv1 <> NEW.icv1)
BEGIN SELECT RAISE(ABORT, 'curve slug already loaded with a different ICV1 string'); END;

CREATE TRIGGER IF NOT EXISTS workloads_identity BEFORE INSERT ON workloads
WHEN EXISTS (SELECT 1 FROM workloads WHERE workload_id = NEW.workload_id AND workload_sha256 <> NEW.workload_sha256)
BEGIN SELECT RAISE(ABORT, 'workload id already loaded with a different workload hash'); END;

CREATE TRIGGER IF NOT EXISTS algorithms_identity BEFORE INSERT ON algorithms
WHEN EXISTS (SELECT 1 FROM algorithms WHERE method_id = NEW.method_id AND method_sha256 <> NEW.method_sha256)
BEGIN SELECT RAISE(ABORT, 'method id already loaded with a different method hash'); END;

CREATE TRIGGER IF NOT EXISTS factor_bases_identity BEFORE INSERT ON factor_bases
WHEN EXISTS (SELECT 1 FROM factor_bases WHERE fb_id = NEW.fb_id AND fb_sha256 <> NEW.fb_sha256)
BEGIN SELECT RAISE(ABORT, 'factor base id already loaded with a different factor-base hash'); END;

CREATE TRIGGER IF NOT EXISTS hosts_identity BEFORE INSERT ON hosts
WHEN EXISTS (SELECT 1 FROM hosts WHERE env_class_id = NEW.env_class_id AND env_class_sha256 <> NEW.env_class_sha256)
BEGIN SELECT RAISE(ABORT, 'host class id already loaded with a different class hash'); END;

-- A run loaded again must be the same run: the same session and sequence
-- number, and the same figures.  (Dropped first so a database built under
-- an older schema gets the stricter guard.)
DROP TRIGGER IF EXISTS runs_identity;
CREATE TRIGGER runs_identity BEFORE INSERT ON runs
WHEN EXISTS (SELECT 1 FROM runs WHERE record_id = NEW.record_id
             AND (session_id <> NEW.session_id OR seq <> NEW.seq OR run_id <> NEW.run_id
                  OR method_id <> NEW.method_id OR workload_id <> NEW.workload_id
                  OR status <> NEW.status OR total_gae IS NOT NEW.total_gae OR s IS NOT NEW.s))
BEGIN SELECT RAISE(ABORT, 'record id already loaded as a different run'); END;

-- A session loaded again must be the same session, and a complete one
-- never reverts or changes its records.
DROP TRIGGER IF EXISTS sessions_identity;
CREATE TRIGGER sessions_identity BEFORE INSERT ON sessions
WHEN EXISTS (SELECT 1 FROM sessions WHERE session_id = NEW.session_id
             AND (spec_sha256 <> NEW.spec_sha256 OR env_class_id <> NEW.env_class_id
                  OR label <> NEW.label OR started_unix_ms <> NEW.started_unix_ms
                  OR binary_sha256 IS NOT NEW.binary_sha256
                  OR (status = 'complete'
                      AND (NEW.status <> 'complete' OR records_sha256 IS NOT NEW.records_sha256))))
BEGIN SELECT RAISE(ABORT, 'session id already loaded as a different session, or as complete with other records'); END;

-- An arm name keeps its role and method for the whole session.
CREATE TRIGGER IF NOT EXISTS arms_identity BEFORE INSERT ON arms
WHEN EXISTS (SELECT 1 FROM arms WHERE session_id = NEW.session_id AND arm = NEW.arm
             AND (role <> NEW.role OR method_id <> NEW.method_id))
BEGIN SELECT RAISE(ABORT, 'arm already loaded with a different role or method'); END;

-- A comparison id keeps its content: a differing row is an error, never a
-- replacement.
CREATE TRIGGER IF NOT EXISTS comparisons_identity BEFORE INSERT ON comparisons
WHEN EXISTS (SELECT 1 FROM comparisons WHERE comparison_id = NEW.comparison_id AND json <> NEW.json)
BEGIN SELECT RAISE(ABORT, 'comparison id already loaded with different content'); END;

CREATE TRIGGER IF NOT EXISTS claims_identity BEFORE INSERT ON claims
WHEN EXISTS (SELECT 1 FROM claims WHERE run_id = NEW.run_id
             AND (candidate_manifest_sha256 <> NEW.candidate_manifest_sha256
                  OR workload_manifest_sha256 <> NEW.workload_manifest_sha256
                  OR session_id <> NEW.session_id))
BEGIN SELECT RAISE(ABORT, 'claim run id already loaded for a different candidate, workload or session'); END;

-- ── Views ────────────────────────────────────────────────────────────

-- The views read complete sessions only.  (Dropped and recreated on every
-- load, so a database built under an older schema picks up the filter.)
DROP VIEW IF EXISTS arm_workload_summary;
DROP VIEW IF EXISTS method_by_curve;
DROP VIEW IF EXISTS ic_phase_split;
DROP VIEW IF EXISTS ic_yield;
DROP VIEW IF EXISTS fb_measurement_ledger;

-- Every arm on every workload: the table AGENTS.md §2 asks for, one unit.
CREATE VIEW arm_workload_summary AS
SELECT r.session_id, r.arm, a.method, a.family, r.method_id,
       w.curve_slug, c.log2_r, r.workload_id,
       COUNT(*)                                          AS measured,
       SUM(r.status = 'verified')                        AS verified,
       AVG(CASE WHEN r.status = 'verified' THEN r.s END) AS mean_s,
       MIN(CASE WHEN r.status = 'verified' THEN r.s END) AS min_s,
       MAX(CASE WHEN r.status = 'verified' THEN r.s END) AS max_s,
       c.floor_s,
       AVG(CASE WHEN r.status = 'verified' THEN r.s END) / c.floor_s AS mean_ratio_to_floor,
       MAX(r.lower_bound)                                AS any_lower_bound,
       MIN(r.isolation_level)                            AS worst_level
FROM runs r
JOIN sessions ss ON ss.session_id = r.session_id AND ss.status = 'complete'
JOIN algorithms a ON a.method_id = r.method_id
JOIN workloads w ON w.workload_id = r.workload_id
JOIN curves c ON c.slug = w.curve_slug
WHERE r.warmup = 0
GROUP BY r.session_id, r.arm, r.workload_id;

-- Each method's mean S per curve across sessions, against the curve's floor.
CREATE VIEW method_by_curve AS
SELECT a.method, a.family, r.method_id, w.curve_slug, c.log2_r, c.floor_s,
       COUNT(*)                     AS verified_runs,
       AVG(r.s)                     AS mean_s,
       AVG(r.s) / c.floor_s         AS mean_ratio_to_floor,
       MAX(r.lower_bound)           AS any_lower_bound
FROM runs r
JOIN sessions ss ON ss.session_id = r.session_id AND ss.status = 'complete'
JOIN algorithms a ON a.method_id = r.method_id
JOIN workloads w ON w.workload_id = r.workload_id
JOIN curves c ON c.slug = w.curve_slug
WHERE r.warmup = 0 AND r.status = 'verified'
GROUP BY r.method_id, w.curve_slug;

-- Index-calculus runs with their factor base and phase split.
CREATE VIEW ic_phase_split AS
SELECT r.record_id, r.session_id, r.arm, w.curve_slug, f.fb_id, f.family AS fb_family,
       f.signed_points, f.columns, p.phase, p.gae, r.total_gae,
       p.gae / r.total_gae AS share
FROM runs r
JOIN sessions ss ON ss.session_id = r.session_id AND ss.status = 'complete'
JOIN phases p ON p.record_id = r.record_id
JOIN workloads w ON w.workload_id = r.workload_id
LEFT JOIN factor_bases f ON f.fb_id = r.fb_id
WHERE r.fb_id IS NOT NULL;

-- The yield ledger: what each index-calculus run yielded on one target,
-- with its base, oracle, matrix rank and solver statistics.  `yield` is
-- relations per trial.  ic.pipeline writes `relations`, ic.shared_rank
-- writes `hits`; the view reads either.
CREATE VIEW ic_yield AS
SELECT r.record_id, r.session_id, r.arm, r.round, r.status, r.run_id,
       w.curve_slug, w.target_index, r.fb_id, r.fbm_id, f.family AS fb_family, f.columns AS fb_columns,
       f.signed_points AS fb_signed_points, a.method AS method,
       json_extract(a.params_json, '$.oracle') AS oracle,
       json_extract(a.params_json, '$.solver') AS solver_param,
       json_extract(p.native_json, '$.trials') AS trials,
       COALESCE(json_extract(p.native_json, '$.relations'), json_extract(p.native_json, '$.hits')) AS relations,
       CAST(COALESCE(json_extract(p.native_json, '$.relations'), json_extract(p.native_json, '$.hits')) AS REAL)
         / NULLIF(json_extract(p.native_json, '$.trials'), 0) AS yield,
       json_extract(p.native_json, '$.lookups') AS lookups,
       json_extract(p.native_json, '$.lift_failures') AS lift_failures,
       json_extract(l.native_json, '$.rows') AS matrix_rows,
       json_extract(l.native_json, '$.rank') AS matrix_rank,
       p.gae AS relations_gae, r.total_gae, r.s, r.deterministic,
       s.name AS solver, s.calls AS solver_calls, s.ops AS solver_ops, s.op_unit AS solver_op_unit,
       s.budget_exceeded, s.n_vars, s.n_equations, s.semi_regular_degree, s.solving_degree_max,
       s.macaulay_rows, s.macaulay_columns, s.macaulay_degree, s.macaulay_rank,
       s.sat_variables, s.sat_clauses, s.sat_conflicts, s.sat_decisions, s.sat_propagations
FROM runs r
JOIN sessions ss ON ss.session_id = r.session_id AND ss.status = 'complete'
JOIN algorithms a ON a.method_id = r.method_id
JOIN workloads w ON w.workload_id = r.workload_id
LEFT JOIN factor_bases f ON f.fb_id = r.fb_id
LEFT JOIN phases p ON p.record_id = r.record_id AND p.phase = 'relations'
LEFT JOIN phases l ON l.record_id = r.record_id AND l.phase = 'linear_algebra'
LEFT JOIN run_solver s ON s.record_id = r.record_id
WHERE a.family = 'ic' AND r.warmup = 0;

-- The FBM1 ledger: every measurement cell with its identity and DoR/yield fields.
-- Cite yield or DoR only with both fb_id and fbm_id present.
CREATE VIEW fb_measurement_ledger AS
SELECT m.fbm_id, m.fbm_sha256, m.curve_slug, m.fb_id, m.family AS fb_family,
       m.oracle, m.solver, m.linalg, m.instrument,
       c.cell_id, c.record_id, c.session_id, c.s, c.total_gae,
       c.trials, c.relations, c.yield_per_trial,
       c.solving_degree_mean, c.solving_degree_max, c.semi_regular_degree,
       c.macaulay_rows, c.macaulay_columns, c.status, c.extra_json
FROM factor_base_measurements m
LEFT JOIN factor_base_measurement_cells c ON c.fbm_id = m.fbm_id
  AND (c.session_id IS NULL
       OR c.session_id IN (SELECT session_id FROM sessions WHERE status = 'complete'));

CREATE TRIGGER IF NOT EXISTS factor_base_measurements_identity
BEFORE INSERT ON factor_base_measurements
WHEN EXISTS (
  SELECT 1 FROM factor_base_measurements
  WHERE fbm_id = NEW.fbm_id AND fbm_sha256 <> NEW.fbm_sha256
)
BEGIN
  SELECT RAISE(ABORT, 'fbm_id collision: same short id, different sha256');
END;

-- Version 4: identity guards for arms, sessions, runs and comparisons; views
-- over complete sessions only; run statuses `interrupted` and `spawn_error`
-- (a database created under version 3 keeps its older CHECK and must be
-- rebuilt to load sessions that have them).
-- ── The artifact store ───────────────────────────────────────────────
-- Large files kept in object storage and named in the tracked index
-- `store/index.json` (docs/store/README.md).  `ecbench db sql store/index.json`
-- REPLACES these rows with the index's current content; the index in Git is
-- the record, and `crypto store verify --remote` audits the objects.
-- `kind`, `curve_slug` and `fb_id` are read from the object's NAME by the
-- convention `factor-bases/<ICV1 slug>/<FB1 id>.json`; they are NULL when a
-- name does not follow it.
CREATE TABLE IF NOT EXISTS store_objects (
  name        TEXT PRIMARY KEY,
  sha256      TEXT NOT NULL,                          -- of the stored bytes
  bytes       INTEGER NOT NULL,
  kind        TEXT NOT NULL,                          -- first segment of the name
  curve_slug  TEXT,                                   -- no foreign key: the curve need not be loaded
  fb_id       TEXT,
  description TEXT,
  provenance  TEXT
);

CREATE TABLE IF NOT EXISTS store_tags (
  name TEXT NOT NULL REFERENCES store_objects(name),
  tag  TEXT NOT NULL,
  PRIMARY KEY (name, tag)
);

-- What a name pointed at before `store push --replace` moved it.
CREATE TABLE IF NOT EXISTS store_superseded (
  name   TEXT NOT NULL REFERENCES store_objects(name),
  sha256 TEXT NOT NULL,
  bytes  INTEGER NOT NULL,
  PRIMARY KEY (name, sha256)
);

-- Stored factor-base dumps, and whether their points are loaded here.
CREATE VIEW IF NOT EXISTS store_factor_bases AS
SELECT o.name, o.sha256, o.bytes, o.curve_slug, o.fb_id,
       f.fb_id IS NOT NULL AS loaded, f.family, f.signed_points, f.fb_sha256
FROM store_objects o
LEFT JOIN factor_bases f ON f.fb_id = o.fb_id
WHERE o.kind = 'factor-bases';

INSERT OR REPLACE INTO schema_info (key, value) VALUES ('ecbench_schema_version', '4');

BEGIN;
INSERT INTO hosts VALUES ('ECBENV3h4ed1141c63b6', '4ed1141c63b6f6b310b66bf00b232ea6e294808ec797715bc5f90a7f6db574df', 'linux', 'x86_64', '6.18.57', 'INTEL(R) XEON(R) PLATINUM 8573C', 'adx aes avx avx2 avx512bw avx512dq avx512f avx512ifma avx512vbmi avx512vl bmi1 bmi2 fma gfni pclmulqdq popcnt sha sse4.2 vaes vpclmulqdq', 5, 5, 1, 1, 'rustc 1.99.1 (58cf8f9c3 2026-10-13)', '{"schema":"ecbench.host/v1","env_class_id":"ECBENV3h4ed1141c63b6","env_class_sha256":"4ed1141c63b6f6b310b66bf00b232ea6e294808ec797715bc5f90a7f6db574df","stable":{"os":"linux","arch":"x86_64","kernel_release":"6.18.57","cpu_vendor":"GenuineIntel","cpu_model":"INTEL(R) XEON(R) PLATINUM 8573C","cpu_signature":"6/207/2","microcode":"0x1","features":["adx","aes","avx","avx2","avx512bw","avx512dq","avx512f","avx512ifma","avx512vbmi","avx512vl","bmi1","bmi2","fma","gfni","pclmulqdq","popcnt","sha","sse4.2","vaes","vpclmulqdq"],"flags_sha256":"16a7227851bbd0d54c1a76b00030c349f39b11fe7860f6e2de2e0b7e1261be4c","logical_cpus":5,"physical_cores":5,"packages":1,"numa_nodes":1,"topology":{"cpus":[{"cpu":0,"core_id":0,"package":0,"node":0,"siblings":[0]},{"cpu":1,"core_id":1,"package":0,"node":0,"siblings":[1]},{"cpu":2,"core_id":2,"package":0,"node":0,"siblings":[2]},{"cpu":3,"core_id":3,"package":0,"node":0,"siblings":[3]},{"cpu":4,"core_id":4,"package":0,"node":0,"siblings":[4]}],"nodes":[{"node":0,"cpus":[0,1,2,3,4],"mem_total_kib":18440100,"distances":[10]}]},"perf_levels":[],"mem_total_kib":18440100,"cpu_settings":{"0":{"governor":null,"no_turbo":null,"boost":null,"in_isolcpus":false,"in_nohz_full":false,"smt_control":"notsupported"},"1":{"governor":null,"no_turbo":null,"boost":null,"in_isolcpus":false,"in_nohz_full":false,"smt_control":"notsupported"},"2":{"governor":null,"no_turbo":null,"boost":null,"in_isolcpus":false,"in_nohz_full":false,"smt_control":"notsupported"},"3":{"governor":null,"no_turbo":null,"boost":null,"in_isolcpus":false,"in_nohz_full":false,"smt_control":"notsupported"},"4":{"governor":null,"no_turbo":null,"boost":null,"in_isolcpus":false,"in_nohz_full":false,"smt_control":"notsupported"}},"transparent_hugepages":"always [madvise] never","virtualization":{"cpu_flag":"hypervisor","systemd_detect_virt_vm":"kvm"},"rustc_vv":"rustc 1.99.1 (58cf8f9c3 2026-10-13)\nbinary: rustc\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\ncommit-date: 2026-10-13\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\nLLVM version: 23.1.1"},"build":{"binary_path":"/workspace/scratch/ecbench-rho-frozen-61030c23","binary_sha256":"61030c23b4e95c0ed56b86a65b980c28e19ed09ba643719248734e13218d718d","git_commit":null,"git_dirty":null,"allocator":"glibc 2.41","debug_assertions":false,"target_os":"linux","target_arch":"x86_64","build_env":{}},"process_affinity":[0,1,2,3,4],"captured_unix_ms":1794522323194}') ON CONFLICT (env_class_id) DO NOTHING;
INSERT INTO sessions VALUES ('ECBS1h9cd21a0011c9', 'Separate one-target preflight for fixed repository rho-count audit', 'ECS1hbc386ee698fd', 'bc386ee698fd5cd6870d4753aab1a7153dfd2406e54480d4d9406882463d6e83', 'ECBENV3h4ed1141c63b6', NULL, NULL, '61030c23b4e95c0ed56b86a65b980c28e19ed09ba643719248734e13218d718d', '"none"', NULL, NULL, NULL, NULL, 1794522323199, 1794522323308, 'complete', '160d23d10ceff70f8d3fc6a7ff7fcd886a1b6db1d593fb178403f2117b079bc1', 'research/rho_constant_audit_20261112/sessions/preflight') ON CONFLICT (session_id) DO UPDATE SET finished_unix_ms = excluded.finished_unix_ms, status = excluded.status, records_sha256 = excluded.records_sha256, path = excluded.path;
INSERT INTO curves VALUES ('icv1-fp14-tm43-158b6914', 'ICV1:fp-16187:-43:16231:12429:unk:unk:r:158b69140d4b', 'prime', NULL, 14, '16231', '16231', '1', 1.3986464267334126e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp14-tm43-158b6914', '0x6', '0x3d1f', 'EC1P14Cfph4f360f746508', 'urn:ec-record:1:sha256:4f360f746508dc06057c047d86463d426d802a68c7d719cc464b309780e4b2e4') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp14-tm43-158b6914', 'roster_prime_instance(14)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W592e6a3d460b', '592e6a3d460bec5e458b9bd9ecb0382c98d89244a66a8b39b155c934d6879aa5', 'icv1-fp14-tm43-158b6914', '0x6', '0x3d1f', '0x27c9', '0x2e47', 'uniform_scalar_sha256_v1', '20261112001', 0, '5089') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp16-t351-d433f24e', 'ICV1:fp-65353:351:65003:20412:unk:unk:r:d433f24edf51', 'prime', NULL, 16, '65003', '65003', '1', 1.5988218682078955e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp16-t351-d433f24e', '0x1', '0x3b3d', 'EC1P16Cfph25cb80ed53b2', 'urn:ec-record:1:sha256:25cb80ed53b2ef56b40393ed5c53f586b07b919cfa6250d138c946abd917f6b3') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp16-t351-d433f24e', 'roster_prime_instance(16)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W37c608457bbd', '37c608457bbddf5aa676bb1a9202bb10463f9d085ca20bd3794dbc956b9a0e8a', 'icv1-fp16-t351-d433f24e', '0x1', '0x3b3d', '0xbb87', '0x47ad', 'uniform_scalar_sha256_v1', '20261112001', 0, '49105') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp18-t175-45770ebc', 'ICV1:fp-261847:175:261673:864:unk:unk:r:45770ebcfcb2', 'prime', NULL, 18, '261673', '261673', '1', 1.7997405546030816e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp18-t175-45770ebc', '0x2', '0x4', 'EC1P18Cfphf3346c488da6', 'urn:ec-record:1:sha256:f3346c488da69d51303dc3f0c3de3b8de9358a8645a80f33e75388e02ce2d50b') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp18-t175-45770ebc', 'roster_prime_instance(18)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W6f4baf109b00', '6f4baf109b00f3dbe1f724d54e8b46ca617ed0f43bbaba88016d53b0444a6d7f', 'icv1-fp18-t175-45770ebc', '0x2', '0x4', '0x23c67', '0x389c8', 'uniform_scalar_sha256_v1', '20261112001', 0, '190523') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp20-t1293-3c2999ba', 'ICV1:fp-1048291:1293:1046999:326892:unk:unk:r:3c2999ba43b0', 'prime', NULL, 20, '1046999', '1046999', '1', 1.999782863366001e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp20-t1293-3c2999ba', '0x1', '0x1e07c', 'EC1P20Cfphbc31d4165da7', 'urn:ec-record:1:sha256:bc31d4165da7e5ea9b09a2a803cb545e74d20cccd9d011d8eb3e112037a95cdf') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp20-t1293-3c2999ba', 'roster_prime_instance(20)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W7b411114689e', '7b411114689e4e396c995095802c918d4f8dd3fe2b4d541a6d315b834e81a656', 'icv1-fp20-t1293-3c2999ba', '0x1', '0x1e07c', '0x7b0e', '0x34c87', 'uniform_scalar_sha256_v1', '20261112001', 0, '592974') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hefea4e0ebe79', 'efea4e0ebe79dadfedeb75106dab92e25fc26bfe282155ee0326380d4184e995', 'rho.negation', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp14-tm43-158b6914', 'ICV1:fp-16187:-43:16231:12429:unk:unk:r:158b69140d4b', 'prime', NULL, 14, '16231', '16231', '1', 1.3986464267334126e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp14-tm43-158b6914', '0x6', '0x3d1f', 'EC1P14Cfph4f360f746508', 'urn:ec-record:1:sha256:4f360f746508dc06057c047d86463d426d802a68c7d719cc464b309780e4b2e4') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp14-tm43-158b6914', 'roster_prime_instance(14)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W592e6a3d460b', '592e6a3d460bec5e458b9bd9ecb0382c98d89244a66a8b39b155c934d6879aa5', 'icv1-fp14-tm43-158b6914', '0x6', '0x3d1f', '0x27c9', '0x2e47', 'uniform_scalar_sha256_v1', '20261112001', 0, '5089') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-neg', 'reference', 'ECM1hefea4e0ebe79') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1h706c8b20594b', 'ECBS1h9cd21a0011c9', 'ECM1hefea4e0ebe79W592e6a3d460bR1', 0, 0, 0, 'rho-neg', 'ECM1hefea4e0ebe79', 'W592e6a3d460b', NULL, NULL, '15135734312211569705', 'verified', '5089', 3.55e2, 2.786478613681826e0, 3.1442044172357577e0, 2, 1, 'canonicalisations_uncharged', 1, 205370, 6237576, 0, 5999000, 12872, 0, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h706c8b20594b', 0, 'setup', 25, 32, 0, 5.7e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h706c8b20594b', 1, 'search', 0, 0, 6, 2.8e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h706c8b20594b', 2, 'internal_verification', 0, 0, 0, 1.8e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'canonicalisations_uncharged', 281) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'cycle_escape_doublings', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'cycles_length_4', 3) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'cycles_other_length', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'distinguished_point_bits', 3) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'distinguished_points', 33) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'jump_coefficient_bits', 6) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'jumps', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'look_ahead_additions', 36) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'setup_additions', 25) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'setup_doublings', 32) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'start_additions', 32) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'steps', 208) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'verification_operations', 18) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'walk_operations', 248) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'walks', 33) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h706c8b20594b', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1h706c8b20594b', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hb367476fce6c', 'b367476fce6cfebab6ee78e1374f1a40fa7749badf80f6d83a5ec65b18f1bba4', 'rho.plain', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(PointClasses, RhoWalk::plain())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp14-tm43-158b6914', 'ICV1:fp-16187:-43:16231:12429:unk:unk:r:158b69140d4b', 'prime', NULL, 14, '16231', '16231', '1', 1.3986464267334126e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp14-tm43-158b6914', '0x6', '0x3d1f', 'EC1P14Cfph4f360f746508', 'urn:ec-record:1:sha256:4f360f746508dc06057c047d86463d426d802a68c7d719cc464b309780e4b2e4') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp14-tm43-158b6914', 'roster_prime_instance(14)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W592e6a3d460b', '592e6a3d460bec5e458b9bd9ecb0382c98d89244a66a8b39b155c934d6879aa5', 'icv1-fp14-tm43-158b6914', '0x6', '0x3d1f', '0x27c9', '0x2e47', 'uniform_scalar_sha256_v1', '20261112001', 0, '5089') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-plain', 'baseline', 'ECM1hb367476fce6c') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1h78d5e41627c6', 'ECBS1h9cd21a0011c9', 'ECM1hb367476fce6cW592e6a3d460bR1', 1, 0, 0, 'rho-plain', 'ECM1hb367476fce6c', 'W592e6a3d460b', NULL, NULL, '15135734312211569705', 'verified', '5089', 2.41e2, 1.8916657630910425e0, 2.134516238179768e0, 1, 0, '', 1, 147516, 4464445, 0, 4255000, 12848, 1, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h78d5e41627c6', 0, 'setup', 25, 32, 0, 5.7e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h78d5e41627c6', 1, 'search', 0, 0, 6, 1.66e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h78d5e41627c6', 2, 'internal_verification', 0, 0, 0, 1.8e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'canonicalisations_uncharged', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'cycle_escape_doublings', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'cycles_length_4', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'cycles_other_length', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'distinguished_point_bits', 3) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'distinguished_points', 22) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'jump_coefficient_bits', 6) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'jumps', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'look_ahead_additions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'setup_additions', 25) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'setup_doublings', 32) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'start_additions', 21) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'steps', 145) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'verification_operations', 18) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'walk_operations', 145) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'walks', 22) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h78d5e41627c6', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1h78d5e41627c6', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hefea4e0ebe79', 'efea4e0ebe79dadfedeb75106dab92e25fc26bfe282155ee0326380d4184e995', 'rho.negation', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp14-tm43-158b6914', 'ICV1:fp-16187:-43:16231:12429:unk:unk:r:158b69140d4b', 'prime', NULL, 14, '16231', '16231', '1', 1.3986464267334126e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp14-tm43-158b6914', '0x6', '0x3d1f', 'EC1P14Cfph4f360f746508', 'urn:ec-record:1:sha256:4f360f746508dc06057c047d86463d426d802a68c7d719cc464b309780e4b2e4') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp14-tm43-158b6914', 'roster_prime_instance(14)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W592e6a3d460b', '592e6a3d460bec5e458b9bd9ecb0382c98d89244a66a8b39b155c934d6879aa5', 'icv1-fp14-tm43-158b6914', '0x6', '0x3d1f', '0x27c9', '0x2e47', 'uniform_scalar_sha256_v1', '20261112001', 0, '5089') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-neg-aa', 'control', 'ECM1hefea4e0ebe79') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1h869078019d91', 'ECBS1h9cd21a0011c9', 'ECM1hefea4e0ebe79W592e6a3d460bR2', 2, 0, 0, 'rho-neg-aa', 'ECM1hefea4e0ebe79', 'W592e6a3d460b', NULL, NULL, '15135734312211569705', 'verified', '5089', 3.55e2, 2.786478613681826e0, 3.1442044172357577e0, 2, 1, 'canonicalisations_uncharged', 1, 249195, 4484525, 3525000, 352000, 12968, 2, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h869078019d91', 0, 'setup', 25, 32, 0, 5.7e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h869078019d91', 1, 'search', 0, 0, 6, 2.8e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h869078019d91', 2, 'internal_verification', 0, 0, 0, 1.8e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'canonicalisations_uncharged', 281) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'cycle_escape_doublings', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'cycles_length_4', 3) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'cycles_other_length', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'distinguished_point_bits', 3) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'distinguished_points', 33) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'jump_coefficient_bits', 6) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'jumps', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'look_ahead_additions', 36) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'setup_additions', 25) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'setup_doublings', 32) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'start_additions', 32) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'steps', 208) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'verification_operations', 18) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'walk_operations', 248) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'walks', 33) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h869078019d91', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1h869078019d91', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hefea4e0ebe79', 'efea4e0ebe79dadfedeb75106dab92e25fc26bfe282155ee0326380d4184e995', 'rho.negation', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp16-t351-d433f24e', 'ICV1:fp-65353:351:65003:20412:unk:unk:r:d433f24edf51', 'prime', NULL, 16, '65003', '65003', '1', 1.5988218682078955e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp16-t351-d433f24e', '0x1', '0x3b3d', 'EC1P16Cfph25cb80ed53b2', 'urn:ec-record:1:sha256:25cb80ed53b2ef56b40393ed5c53f586b07b919cfa6250d138c946abd917f6b3') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp16-t351-d433f24e', 'roster_prime_instance(16)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W37c608457bbd', '37c608457bbddf5aa676bb1a9202bb10463f9d085ca20bd3794dbc956b9a0e8a', 'icv1-fp16-t351-d433f24e', '0x1', '0x3b3d', '0xbb87', '0x47ad', 'uniform_scalar_sha256_v1', '20261112001', 0, '49105') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-neg', 'reference', 'ECM1hefea4e0ebe79') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1h0235d35ed25f', 'ECBS1h9cd21a0011c9', 'ECM1hefea4e0ebe79W37c608457bbdR1', 3, 0, 0, 'rho-neg', 'ECM1hefea4e0ebe79', 'W37c608457bbd', NULL, NULL, '325128915932787194', 'verified', '49105', 3.72e2, 1.4590703749617833e0, 1.6463846144331145e0, 2, 1, 'canonicalisations_uncharged', 1, 175410, 5825130, 1231000, 3958000, 12780, 2, 4975, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h0235d35ed25f', 0, 'setup', 54, 75, 0, 1.29e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h0235d35ed25f', 1, 'search', 0, 0, 10, 2.18e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h0235d35ed25f', 2, 'internal_verification', 0, 0, 0, 2.5e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'canonicalisations_uncharged', 219) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'cycle_escape_doublings', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'cycles_length_4', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'cycles_other_length', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'distinguished_point_bits', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'distinguished_points', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'jump_coefficient_bits', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'jumps', 8) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'look_ahead_additions', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'setup_additions', 54) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'setup_doublings', 75) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'start_additions', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'steps', 198) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'verification_operations', 25) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'walk_operations', 209) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'walks', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h0235d35ed25f', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1h0235d35ed25f', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hb367476fce6c', 'b367476fce6cfebab6ee78e1374f1a40fa7749badf80f6d83a5ec65b18f1bba4', 'rho.plain', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(PointClasses, RhoWalk::plain())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp16-t351-d433f24e', 'ICV1:fp-65353:351:65003:20412:unk:unk:r:d433f24edf51', 'prime', NULL, 16, '65003', '65003', '1', 1.5988218682078955e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp16-t351-d433f24e', '0x1', '0x3b3d', 'EC1P16Cfph25cb80ed53b2', 'urn:ec-record:1:sha256:25cb80ed53b2ef56b40393ed5c53f586b07b919cfa6250d138c946abd917f6b3') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp16-t351-d433f24e', 'roster_prime_instance(16)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W37c608457bbd', '37c608457bbddf5aa676bb1a9202bb10463f9d085ca20bd3794dbc956b9a0e8a', 'icv1-fp16-t351-d433f24e', '0x1', '0x3b3d', '0xbb87', '0x47ad', 'uniform_scalar_sha256_v1', '20261112001', 0, '49105') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-plain', 'baseline', 'ECM1hb367476fce6c') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1h2536a70d0f45', 'ECBS1h9cd21a0011c9', 'ECM1hb367476fce6cW37c608457bbdR1', 4, 0, 0, 'rho-plain', 'ECM1hb367476fce6c', 'W37c608457bbd', NULL, NULL, '325128915932787194', 'verified', '49105', 2.54e2, 9.962469764524003e-1, 1.1241443335107824e0, 1, 0, '', 1, 108953, 4860796, 3133000, 1001000, 13020, 1, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h2536a70d0f45', 0, 'setup', 54, 75, 0, 1.29e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h2536a70d0f45', 1, 'search', 0, 0, 10, 1e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h2536a70d0f45', 2, 'internal_verification', 0, 0, 0, 2.5e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'canonicalisations_uncharged', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'cycle_escape_doublings', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'cycles_length_4', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'cycles_other_length', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'distinguished_point_bits', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'distinguished_points', 2) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'jump_coefficient_bits', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'jumps', 8) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'look_ahead_additions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'setup_additions', 54) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'setup_doublings', 75) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'start_additions', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'steps', 99) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'verification_operations', 25) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'walk_operations', 99) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'walks', 2) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h2536a70d0f45', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1h2536a70d0f45', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hefea4e0ebe79', 'efea4e0ebe79dadfedeb75106dab92e25fc26bfe282155ee0326380d4184e995', 'rho.negation', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp16-t351-d433f24e', 'ICV1:fp-65353:351:65003:20412:unk:unk:r:d433f24edf51', 'prime', NULL, 16, '65003', '65003', '1', 1.5988218682078955e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp16-t351-d433f24e', '0x1', '0x3b3d', 'EC1P16Cfph25cb80ed53b2', 'urn:ec-record:1:sha256:25cb80ed53b2ef56b40393ed5c53f586b07b919cfa6250d138c946abd917f6b3') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp16-t351-d433f24e', 'roster_prime_instance(16)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W37c608457bbd', '37c608457bbddf5aa676bb1a9202bb10463f9d085ca20bd3794dbc956b9a0e8a', 'icv1-fp16-t351-d433f24e', '0x1', '0x3b3d', '0xbb87', '0x47ad', 'uniform_scalar_sha256_v1', '20261112001', 0, '49105') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-neg-aa', 'control', 'ECM1hefea4e0ebe79') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1h26c54d0a0777', 'ECBS1h9cd21a0011c9', 'ECM1hefea4e0ebe79W37c608457bbdR2', 5, 0, 0, 'rho-neg-aa', 'ECM1hefea4e0ebe79', 'W37c608457bbd', NULL, NULL, '325128915932787194', 'verified', '49105', 3.72e2, 1.4590703749617833e0, 1.6463846144331145e0, 2, 1, 'canonicalisations_uncharged', 1, 1065598, 6489480, 0, 5948000, 11344, 0, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h26c54d0a0777', 0, 'setup', 54, 75, 0, 1.29e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h26c54d0a0777', 1, 'search', 0, 0, 10, 2.18e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h26c54d0a0777', 2, 'internal_verification', 0, 0, 0, 2.5e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'canonicalisations_uncharged', 219) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'cycle_escape_doublings', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'cycles_length_4', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'cycles_other_length', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'distinguished_point_bits', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'distinguished_points', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'jump_coefficient_bits', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'jumps', 8) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'look_ahead_additions', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'setup_additions', 54) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'setup_doublings', 75) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'start_additions', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'steps', 198) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'verification_operations', 25) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'walk_operations', 209) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'walks', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h26c54d0a0777', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1h26c54d0a0777', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hefea4e0ebe79', 'efea4e0ebe79dadfedeb75106dab92e25fc26bfe282155ee0326380d4184e995', 'rho.negation', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp18-t175-45770ebc', 'ICV1:fp-261847:175:261673:864:unk:unk:r:45770ebcfcb2', 'prime', NULL, 18, '261673', '261673', '1', 1.7997405546030816e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp18-t175-45770ebc', '0x2', '0x4', 'EC1P18Cfphf3346c488da6', 'urn:ec-record:1:sha256:f3346c488da69d51303dc3f0c3de3b8de9358a8645a80f33e75388e02ce2d50b') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp18-t175-45770ebc', 'roster_prime_instance(18)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W6f4baf109b00', '6f4baf109b00f3dbe1f724d54e8b46ca617ed0f43bbaba88016d53b0444a6d7f', 'icv1-fp18-t175-45770ebc', '0x2', '0x4', '0x23c67', '0x389c8', 'uniform_scalar_sha256_v1', '20261112001', 0, '190523') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-neg', 'reference', 'ECM1hefea4e0ebe79') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1hc99452da43c7', 'ECBS1h9cd21a0011c9', 'ECM1hefea4e0ebe79W6f4baf109b00R1', 6, 0, 0, 'rho-neg', 'ECM1hefea4e0ebe79', 'W6f4baf109b00', NULL, NULL, '17850166523046510890', 'verified', '190523', 3.87e2, 7.56539326139634e-1, 8.536632147044406e-1, 2, 1, 'canonicalisations_uncharged', 1, 137498, 5401487, 4860000, 0, 12936, 0, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1hc99452da43c7', 0, 'setup', 56, 80, 0, 1.36e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1hc99452da43c7', 1, 'search', 0, 0, 10, 2.25e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1hc99452da43c7', 2, 'internal_verification', 0, 0, 0, 2.6e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'canonicalisations_uncharged', 226) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'cycle_escape_doublings', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'cycles_length_2', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'cycles_length_4', 2) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'cycles_other_length', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'distinguished_point_bits', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'distinguished_points', 11) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'jump_coefficient_bits', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'jumps', 8) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'look_ahead_additions', 13) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'setup_additions', 56) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'setup_doublings', 80) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'start_additions', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'steps', 198) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'verification_operations', 26) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'walk_operations', 215) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'walks', 11) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hc99452da43c7', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1hc99452da43c7', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hb367476fce6c', 'b367476fce6cfebab6ee78e1374f1a40fa7749badf80f6d83a5ec65b18f1bba4', 'rho.plain', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(PointClasses, RhoWalk::plain())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp18-t175-45770ebc', 'ICV1:fp-261847:175:261673:864:unk:unk:r:45770ebcfcb2', 'prime', NULL, 18, '261673', '261673', '1', 1.7997405546030816e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp18-t175-45770ebc', '0x2', '0x4', 'EC1P18Cfphf3346c488da6', 'urn:ec-record:1:sha256:f3346c488da69d51303dc3f0c3de3b8de9358a8645a80f33e75388e02ce2d50b') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp18-t175-45770ebc', 'roster_prime_instance(18)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W6f4baf109b00', '6f4baf109b00f3dbe1f724d54e8b46ca617ed0f43bbaba88016d53b0444a6d7f', 'icv1-fp18-t175-45770ebc', '0x2', '0x4', '0x23c67', '0x389c8', 'uniform_scalar_sha256_v1', '20261112001', 0, '190523') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-plain', 'baseline', 'ECM1hb367476fce6c') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1hd875abad6a20', 'ECBS1h9cd21a0011c9', 'ECM1hb367476fce6cW6f4baf109b00R1', 7, 0, 0, 'rho-plain', 'ECM1hb367476fce6c', 'W6f4baf109b00', NULL, NULL, '17850166523046510890', 'verified', '190523', 5.35e2, 1.0458618591336026e0, 1.180128733506139e0, 1, 0, '', 1, 151488, 6226735, 4567000, 1112000, 13072, 1, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1hd875abad6a20', 0, 'setup', 56, 80, 0, 1.36e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1hd875abad6a20', 1, 'search', 0, 0, 10, 3.73e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1hd875abad6a20', 2, 'internal_verification', 0, 0, 0, 2.6e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'canonicalisations_uncharged', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'cycle_escape_doublings', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'cycles_length_4', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'cycles_other_length', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'distinguished_point_bits', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'distinguished_points', 23) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'jump_coefficient_bits', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'jumps', 8) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'look_ahead_additions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'setup_additions', 56) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'setup_doublings', 80) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'start_additions', 22) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'steps', 351) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'verification_operations', 26) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'walk_operations', 351) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'walks', 23) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hd875abad6a20', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1hd875abad6a20', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hefea4e0ebe79', 'efea4e0ebe79dadfedeb75106dab92e25fc26bfe282155ee0326380d4184e995', 'rho.negation', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp18-t175-45770ebc', 'ICV1:fp-261847:175:261673:864:unk:unk:r:45770ebcfcb2', 'prime', NULL, 18, '261673', '261673', '1', 1.7997405546030816e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp18-t175-45770ebc', '0x2', '0x4', 'EC1P18Cfphf3346c488da6', 'urn:ec-record:1:sha256:f3346c488da69d51303dc3f0c3de3b8de9358a8645a80f33e75388e02ce2d50b') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp18-t175-45770ebc', 'roster_prime_instance(18)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W6f4baf109b00', '6f4baf109b00f3dbe1f724d54e8b46ca617ed0f43bbaba88016d53b0444a6d7f', 'icv1-fp18-t175-45770ebc', '0x2', '0x4', '0x23c67', '0x389c8', 'uniform_scalar_sha256_v1', '20261112001', 0, '190523') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-neg-aa', 'control', 'ECM1hefea4e0ebe79') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1h8bf978029605', 'ECBS1h9cd21a0011c9', 'ECM1hefea4e0ebe79W6f4baf109b00R2', 8, 0, 0, 'rho-neg-aa', 'ECM1hefea4e0ebe79', 'W6f4baf109b00', NULL, NULL, '17850166523046510890', 'verified', '190523', 3.87e2, 7.56539326139634e-1, 8.536632147044406e-1, 2, 1, 'canonicalisations_uncharged', 1, 172739, 5216259, 4571000, 0, 12752, 0, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h8bf978029605', 0, 'setup', 56, 80, 0, 1.36e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h8bf978029605', 1, 'search', 0, 0, 10, 2.25e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h8bf978029605', 2, 'internal_verification', 0, 0, 0, 2.6e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'canonicalisations_uncharged', 226) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'cycle_escape_doublings', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'cycles_length_2', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'cycles_length_4', 2) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'cycles_other_length', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'distinguished_point_bits', 4) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'distinguished_points', 11) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'jump_coefficient_bits', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'jumps', 8) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'look_ahead_additions', 13) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'setup_additions', 56) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'setup_doublings', 80) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'start_additions', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'steps', 198) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'verification_operations', 26) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'walk_operations', 215) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'walks', 11) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h8bf978029605', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1h8bf978029605', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hefea4e0ebe79', 'efea4e0ebe79dadfedeb75106dab92e25fc26bfe282155ee0326380d4184e995', 'rho.negation', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp20-t1293-3c2999ba', 'ICV1:fp-1048291:1293:1046999:326892:unk:unk:r:3c2999ba43b0', 'prime', NULL, 20, '1046999', '1046999', '1', 1.999782863366001e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp20-t1293-3c2999ba', '0x1', '0x1e07c', 'EC1P20Cfphbc31d4165da7', 'urn:ec-record:1:sha256:bc31d4165da7e5ea9b09a2a803cb545e74d20cccd9d011d8eb3e112037a95cdf') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp20-t1293-3c2999ba', 'roster_prime_instance(20)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W7b411114689e', '7b411114689e4e396c995095802c918d4f8dd3fe2b4d541a6d315b834e81a656', 'icv1-fp20-t1293-3c2999ba', '0x1', '0x1e07c', '0x7b0e', '0x34c87', 'uniform_scalar_sha256_v1', '20261112001', 0, '592974') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-neg', 'reference', 'ECM1hefea4e0ebe79') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1h25aed6eabd4b', 'ECBS1h9cd21a0011c9', 'ECM1hefea4e0ebe79W7b411114689eR1', 9, 0, 0, 'rho-neg', 'ECM1hefea4e0ebe79', 'W7b411114689e', NULL, NULL, '8287508584014093421', 'verified', '592974', 2.561e3, 2.5028593513176975e0, 2.824174350197078e0, 2, 1, 'canonicalisations_uncharged', 1, 420817, 4439244, 0, 3845000, 13052, 1, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h25aed6eabd4b', 0, 'setup', 57, 77, 0, 1.34e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h25aed6eabd4b', 1, 'search', 0, 0, 10, 2.401e3, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h25aed6eabd4b', 2, 'internal_verification', 0, 0, 0, 2.6e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'canonicalisations_uncharged', 2402) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'cycle_escape_doublings', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'cycles_length_4', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'cycles_other_length', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'distinguished_point_bits', 5) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'distinguished_points', 61) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'jump_coefficient_bits', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'jumps', 8) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'look_ahead_additions', 139) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'setup_additions', 57) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'setup_doublings', 77) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'start_additions', 60) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'steps', 2192) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'verification_operations', 26) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'walk_operations', 2341) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'walks', 61) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h25aed6eabd4b', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1h25aed6eabd4b', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hb367476fce6c', 'b367476fce6cfebab6ee78e1374f1a40fa7749badf80f6d83a5ec65b18f1bba4', 'rho.plain', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(PointClasses, RhoWalk::plain())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp20-t1293-3c2999ba', 'ICV1:fp-1048291:1293:1046999:326892:unk:unk:r:3c2999ba43b0', 'prime', NULL, 20, '1046999', '1046999', '1', 1.999782863366001e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp20-t1293-3c2999ba', '0x1', '0x1e07c', 'EC1P20Cfphbc31d4165da7', 'urn:ec-record:1:sha256:bc31d4165da7e5ea9b09a2a803cb545e74d20cccd9d011d8eb3e112037a95cdf') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp20-t1293-3c2999ba', 'roster_prime_instance(20)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W7b411114689e', '7b411114689e4e396c995095802c918d4f8dd3fe2b4d541a6d315b834e81a656', 'icv1-fp20-t1293-3c2999ba', '0x1', '0x1e07c', '0x7b0e', '0x34c87', 'uniform_scalar_sha256_v1', '20261112001', 0, '592974') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-plain', 'baseline', 'ECM1hb367476fce6c') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1h6074c0d38f7b', 'ECBS1h9cd21a0011c9', 'ECM1hb367476fce6cW7b411114689eR1', 10, 0, 0, 'rho-plain', 'ECM1hb367476fce6c', 'W7b411114689e', NULL, NULL, '8287508584014093421', 'verified', '592974', 2.348e3, 2.294694946073391e0, 2.5892859719885752e0, 1, 0, '', 1, 443196, 4620723, 4048000, 0, 13256, 0, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h6074c0d38f7b', 0, 'setup', 57, 77, 0, 1.34e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h6074c0d38f7b', 1, 'search', 0, 0, 10, 2.188e3, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1h6074c0d38f7b', 2, 'internal_verification', 0, 0, 0, 2.6e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'canonicalisations_uncharged', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'cycle_escape_doublings', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'cycles_length_4', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'cycles_other_length', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'distinguished_point_bits', 5) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'distinguished_points', 67) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'jump_coefficient_bits', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'jumps', 8) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'look_ahead_additions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'setup_additions', 57) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'setup_doublings', 77) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'start_additions', 66) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'steps', 2122) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'verification_operations', 26) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'walk_operations', 2122) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'walks', 67) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1h6074c0d38f7b', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1h6074c0d38f7b', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO algorithms VALUES ('ECM1hefea4e0ebe79', 'efea4e0ebe79dadfedeb75106dab92e25fc26bfe282155ee0326380d4184e995', 'rho.negation', 'rho', '{"cap_multiple":"64"}', 'ic_boundary::rho_walk_with(NegationClasses, RhoWalk::negation())') ON CONFLICT (method_id) DO NOTHING;
INSERT INTO curves VALUES ('icv1-fp20-t1293-3c2999ba', 'ICV1:fp-1048291:1293:1046999:326892:unk:unk:r:3c2999ba43b0', 'prime', NULL, 20, '1046999', '1046999', '1', 1.999782863366001e1, 2, 8.862269254527579e-1, 1) ON CONFLICT (slug) DO NOTHING;
INSERT INTO curve_representations VALUES ('icv1-fp20-t1293-3c2999ba', '0x1', '0x1e07c', 'EC1P20Cfphbc31d4165da7', 'urn:ec-record:1:sha256:bc31d4165da7e5ea9b09a2a803cb545e74d20cccd9d011d8eb3e112037a95cdf') ON CONFLICT (slug, generator_x, generator_y) DO NOTHING;
INSERT INTO curve_constructions VALUES ('icv1-fp20-t1293-3c2999ba', 'roster_prime_instance(20)') ON CONFLICT (slug, construction) DO NOTHING;
INSERT INTO workloads VALUES ('W7b411114689e', '7b411114689e4e396c995095802c918d4f8dd3fe2b4d541a6d315b834e81a656', 'icv1-fp20-t1293-3c2999ba', '0x1', '0x1e07c', '0x7b0e', '0x34c87', 'uniform_scalar_sha256_v1', '20261112001', 0, '592974') ON CONFLICT (workload_id) DO NOTHING;
INSERT INTO arms VALUES ('ECBS1h9cd21a0011c9', 'rho-neg-aa', 'control', 'ECM1hefea4e0ebe79') ON CONFLICT (session_id, arm) DO NOTHING;
INSERT INTO runs (record_id, session_id, run_id, seq, round, warmup, arm, method_id, workload_id, fb_id, fbm_id, algorithm_seed, status, recovered, total_gae, s, ratio_to_floor, automorphisms_used, lower_bound, unpriced, deterministic, solve_wall_ns, process_wall_ns, user_ns, sys_ns, max_rss_kib, involuntary_switches, run_delay_ns, isolation_level, foreign_busy_ticks, steal_ticks, error) VALUES ('ECR1hf6145c505376', 'ECBS1h9cd21a0011c9', 'ECM1hefea4e0ebe79W7b411114689eR2', 11, 0, 0, 'rho-neg-aa', 'ECM1hefea4e0ebe79', 'W7b411114689e', NULL, NULL, '8287508584014093421', 'verified', '592974', 2.561e3, 2.5028593513176975e0, 2.824174350197078e0, 2, 1, 'canonicalisations_uncharged', 1, 1892376, 9121798, 0, 5974000, 13128, 0, 0, 'L0', NULL, NULL, NULL) ON CONFLICT (record_id) DO NOTHING;
INSERT INTO phases VALUES ('ECR1hf6145c505376', 0, 'setup', 57, 77, 0, 1.34e2, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1hf6145c505376', 1, 'search', 0, 0, 10, 2.401e3, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO phases VALUES ('ECR1hf6145c505376', 2, 'internal_verification', 0, 0, 0, 2.6e1, NULL, '{}') ON CONFLICT (record_id, ord) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'canonicalisations_uncharged', 2402) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'cycle_escape_doublings', 10) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'cycles_length_2', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'cycles_length_4', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'cycles_other_length', 1) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'distinguished_point_bits', 5) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'distinguished_points', 61) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'jump_coefficient_bits', 9) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'jumps', 8) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'look_ahead_additions', 139) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'setup_additions', 57) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'setup_doublings', 77) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'start_additions', 60) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'steps', 2192) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'useless_collisions', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'verification_operations', 26) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'walk_operations', 2341) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'walks', 61) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'walks_abandoned_in_a_cycle', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO run_counters VALUES ('ECR1hf6145c505376', 'walks_capped', 0) ON CONFLICT (record_id, name) DO NOTHING;
INSERT INTO isolation_blockers VALUES ('ECR1hf6145c505376', 0, 'L1: no CPU reservation (--cpus none, or no affinity control on this OS)') ON CONFLICT (record_id, ord) DO NOTHING;
COMMIT;

-- Synthetic analysis negative control only; no canonical session is changed.
UPDATE run_counters SET value=value+1 WHERE rowid=(SELECT q.rowid FROM run_counters q JOIN runs r USING(record_id) WHERE r.arm='rho-neg-aa' ORDER BY q.record_id,q.name LIMIT 1);
