-- 0012: factor-base design grids and the IC tool program's claims.
--
-- A design grid (research/koblitz_n83_factor_base_sweep_20261008/design.json)
-- is a planned Cartesian product of factor-base specs and solver settings.
-- Its base specs are loaded as `spec` factor bases (FBS1 ids, one per curve
-- model and construction) and grouped under ec.factor_base_design with their
-- ordinals. The design's per-tuple disposition counts say why tuples were not
-- run; they are design coverage, not experimental coverage, and the table keeps
-- the design's own wording of that.
--
-- The IC tool program (research/ic_tool_program/rule/<version>/claims/) seals
-- one claim per curve x target x round: the IC candidate's single-target online
-- wall time against matched rho on the same target, with the reusable set-up
-- reported beside it. A candidate's factor base is named by the digest of its
-- usable point set (scheme `pointset`), so the same base under several rule
-- versions is one row.

ALTER TABLE ec.factor_base DROP CONSTRAINT IF EXISTS factor_base_scheme_check;
ALTER TABLE ec.factor_base ADD CONSTRAINT factor_base_scheme_check
  CHECK (scheme IN ('fb1', 'fbarchive', 'spec', 'pointset'));
COMMENT ON COLUMN ec.factor_base.scheme IS 'fb1: ecbench FB1 id. fbarchive: cryptanalysis archive digest. spec: ecdb FBS1 id of a construction. pointset: SHA-256 of a usable point set as an IC tool-program manifest records it.';

CREATE TABLE ec.factor_base_design (
  id                  bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  study               text NOT NULL UNIQUE,
  schema              text NOT NULL,
  path                text NOT NULL,
  curve_models        text[] NOT NULL,                -- index = the Koblitz a of a base spec
  base_spec_count     int NOT NULL,
  solver_axes         jsonb,
  solver_radices      int[],
  solver_tuple_count  bigint,
  total_tuple_count   bigint,
  disposition_counts  jsonb,                          -- tuples per reason they were not (or were) run
  coverage            text,
  outside_grid        text,
  objective           jsonb,
  runtime_claim_gate  text,
  record              jsonb NOT NULL,
  provenance_id       bigint NOT NULL REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.factor_base_design IS 'A planned factor-base x solver grid. Counts are design coverage (how many tuples exist and why each was or was not run), never results.';

CREATE TABLE ec.factor_base_design_member (
  design_id      bigint NOT NULL REFERENCES ec.factor_base_design(id),
  ordinal        int NOT NULL,                        -- position in the design's base_specs (mixed-radix address)
  factor_base_id bigint NOT NULL REFERENCES ec.factor_base(id),
  spec           jsonb NOT NULL,                      -- the base spec verbatim
  PRIMARY KEY (design_id, ordinal)
);
CREATE INDEX factor_base_design_member_fb_idx ON ec.factor_base_design_member (factor_base_id);

CREATE TABLE ec.ic_claim (
  id                       bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  program                  text NOT NULL,             -- ic_tool_program
  rule_version             text NOT NULL,             -- v3, v6, v7, v8, v8c
  row_key                  text NOT NULL,             -- <curve dir>/T<target>-R<round>
  run_id                   text NOT NULL,
  candidate_id             text NOT NULL,             -- IC1 label
  candidate_manifest_sha256 text,
  factor_base_id           bigint REFERENCES ec.factor_base(id),
  curve_id                 bigint REFERENCES ec.curve(id),
  curve_slug               text,
  ec1                      text,
  n                        int,
  a                        int,
  log2_r                   double precision,
  workload_id              text,
  target_index             int,
  round                    int,
  timing_class             text,
  record_class             text,
  ic_online_wall_ms        double precision,
  rho_online_wall_ms       double precision,
  online_speedup           double precision,          -- rho / IC, online interval only
  s_ic_online              double precision,
  s_rho_online             double precision,
  s_setup                  double precision,          -- IC reusable set-up, outside the online interval
  s_ic_cold                double precision,          -- set-up + online: the cost the success criterion compares
  s_rho_cold               double precision,
  cold_ratio_ic_over_rho   double precision,          -- > 1: rho is cheaper cold; the distance remaining
  ic_online_operations     bigint,
  rho_online_operations    bigint,
  ic_scalar_verified       boolean,
  rho_scalar_verified      boolean,
  verdict                  text,
  validation_status        text,                      -- the claim's fail-closed validator: PASS / FAIL
  independent_validation   boolean,
  ic_online_phase_ms       jsonb,
  rho_online_phase_ms      jsonb,
  units                    jsonb,
  claim_boundary           text,
  report_path              text,
  source_path              text NOT NULL,
  record                   jsonb NOT NULL,
  provenance_id            bigint NOT NULL REFERENCES ec.provenance(id),
  UNIQUE (program, rule_version, row_key)
);
CREATE INDEX ic_claim_curve_idx ON ec.ic_claim (curve_id);
CREATE INDEX ic_claim_fb_idx ON ec.ic_claim (factor_base_id);
COMMENT ON TABLE ec.ic_claim IS 'A sealed single-target claim of the IC tool program: IC online wall time against matched rho on the same target. online_speedup covers the online interval only; the reusable set-up (s_setup) is outside it. cold_ratio_ic_over_rho (the producer''s, set-up included) is the comparable figure. Not a leaderboard row.';

CREATE VIEW ec.v_ic_claim_summary AS
SELECT program, rule_version, candidate_id, curve_slug, factor_base_id,
       count(*) AS claims,
       count(*) FILTER (WHERE validation_status = 'PASS') AS passed,
       count(*) FILTER (WHERE online_speedup > 1) AS online_faster,
       exp(avg(ln(online_speedup)) FILTER (WHERE online_speedup > 0)) AS geomean_online_speedup,
       min(online_speedup) AS min_online_speedup,
       max(online_speedup) AS max_online_speedup,
       exp(avg(ln(cold_ratio_ic_over_rho)) FILTER (WHERE cold_ratio_ic_over_rho > 0)) AS geomean_cold_ratio_ic_over_rho,
       min(cold_ratio_ic_over_rho) AS min_cold_ratio_ic_over_rho,
       avg(s_setup) AS mean_s_setup,
       avg(s_ic_online) AS mean_s_ic_online,
       avg(s_rho_online) AS mean_s_rho_online
  FROM ec.ic_claim
 GROUP BY program, rule_version, candidate_id, curve_slug, factor_base_id;
COMMENT ON VIEW ec.v_ic_claim_summary IS 'Per rule version and candidate: claim counts, validator passes, the online speedup (geometric mean; set-up excluded) and the cold ratio IC / rho (set-up included; above 1 means rho is cheaper, and the value is the distance remaining).';

GRANT SELECT ON ec.factor_base_design, ec.factor_base_design_member, ec.ic_claim, ec.v_ic_claim_summary
  TO ecdb_reader, ecdb_writer;
