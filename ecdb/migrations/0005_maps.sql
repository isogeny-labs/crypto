-- ecdb 0005: maps — isogenies and isogeny routes, volcanoes, typed curve
-- relations (isomorphisms, twists, base change, model changes), covers and
-- Weil descent, DLP transfers with their cost accounting, certificates.
--
-- Typed links stay separate (docs/curves/ic/curve-links/README.md): a twist
-- or a field/model change is a curve_relation, never an isogeny; an isogeny
-- needs an explicit kernel or map; no base-field log transport is inferred
-- from a shared j-invariant.

CREATE TYPE ec.isogeny_formula AS ENUM (
  'velu', 'kohel', 'elkies', 'sqrt_velu', 'radical', 'modular_polynomial',
  'richelot', 'lubicz_robert', 'cosset_robert', 'kani', 'frobenius', 'verschiebung',
  'dual', 'composition', 'cm_action', 'other', 'unknown'
);

CREATE TYPE ec.volcano_direction AS ENUM ('horizontal', 'ascending', 'descending', 'unknown');

CREATE TABLE ec.isogeny (
  id                  bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  external_id         text UNIQUE,         -- producer id, e.g. e3h1_d64cad762368
  source_av_id        bigint NOT NULL REFERENCES ec.abelian_variety(id),
  target_av_id        bigint NOT NULL REFERENCES ec.abelian_variety(id),
  source_curve_id     bigint REFERENCES ec.curve(id),   -- set for Jacobians of curves (always for genus 1)
  target_curve_id     bigint REFERENCES ec.curve(id),
  dimension           integer NOT NULL,
  degree              ec.pos NOT NULL,
  degree_factorization jsonb,
  ell                 ec.pos,              -- prime when the degree is a power of one prime
  kernel_structure    ec.pos[],            -- invariant factors of the kernel: {l} cyclic, {l,l} for (l,l)
  is_separable        boolean,
  inseparable_degree  ec.pos,
  field_of_definition_id bigint REFERENCES ec.field(id),
  formula             ec.isogeny_formula NOT NULL DEFAULT 'unknown',
  kernel_polynomial   jsonb,               -- ascending field elements (genus 1)
  kernel_generators   jsonb,               -- coordinates or Mumford divisors spanning the kernel
  kernel_ideal        jsonb,               -- CM / quaternion ideal generating the kernel
  codomain_raw        jsonb,               -- codomain as the formula produced it, before normalization
  post_isomorphism    jsonb,               -- e.g. {"u2": ...} or {"u","r","s","t"} onto the stored target model
  rational_map        jsonb,               -- explicit map; large maps go to an artifact
  rational_map_artifact_id bigint REFERENCES ec.artifact(id),
  is_normalized       boolean,
  direction           ec.volcano_direction,
  dual_isogeny_id     bigint REFERENCES ec.isogeny(id),
  is_endomorphism     boolean GENERATED ALWAYS AS (source_av_id = target_av_id) STORED,
  certificate         text,                -- what was checked, e.g. kernel divides psi_ell, closed under [g]
  status              ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id       bigint REFERENCES ec.provenance(id),
  created_at          timestamptz NOT NULL DEFAULT now(),
  CHECK (dimension >= 1),
  CHECK (kernel_polynomial IS NULL OR dimension = 1)
);
COMMENT ON TABLE ec.isogeny IS
  'Isogenies between abelian varieties (elliptic-curve isogenies are dimension 1; Richelot / (l,l) for surfaces). Explicit kernel or map required for status proved.';
CREATE INDEX isogeny_source_idx ON ec.isogeny (source_curve_id, degree);
CREATE INDEX isogeny_target_idx ON ec.isogeny (target_curve_id, degree);
CREATE INDEX isogeny_source_av_idx ON ec.isogeny (source_av_id);
CREATE INDEX isogeny_target_av_idx ON ec.isogeny (target_av_id);
CREATE INDEX isogeny_ell_idx ON ec.isogeny (ell, direction);

CREATE TABLE ec.isogeny_route (
  id               bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  external_id      text UNIQUE,            -- IW1... route id
  source_curve_id  bigint REFERENCES ec.curve(id),
  target_curve_id  bigint REFERENCES ec.curve(id),
  source_av_id     bigint NOT NULL REFERENCES ec.abelian_variety(id),
  target_av_id     bigint NOT NULL REFERENCES ec.abelian_variety(id),
  total_degree     ec.pos,
  length           integer NOT NULL,
  search_prime     ec.pos,
  status           ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id    bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.isogeny_route IS 'Ordered chain of isogenies (IW1 routes); the destination keeps its own identity.';

CREATE TABLE ec.isogeny_route_step (
  route_id    bigint NOT NULL REFERENCES ec.isogeny_route(id) ON DELETE CASCADE,
  step_index  integer NOT NULL CHECK (step_index >= 0),
  isogeny_id  bigint NOT NULL REFERENCES ec.isogeny(id),
  PRIMARY KEY (route_id, step_index)
);

CREATE TABLE ec.isogeny_volcano (
  id                bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  isogeny_class_id  bigint NOT NULL REFERENCES ec.isogeny_class_fq(id),
  ell               ec.pos NOT NULL,
  depth             integer NOT NULL,
  crater_size       integer,
  crater_shape      text CHECK (crater_shape IN ('single','pair','cycle')),
  vertex_count      integer,
  is_complete       boolean NOT NULL DEFAULT false,  -- every vertex of this component recorded
  status            ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id     bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.isogeny_volcano IS 'One connected component of the ell-isogeny graph of an ordinary isogeny class.';

CREATE TABLE ec.volcano_vertex (
  volcano_id  bigint NOT NULL REFERENCES ec.isogeny_volcano(id) ON DELETE CASCADE,
  curve_id    bigint NOT NULL REFERENCES ec.curve(id),
  level       integer NOT NULL,
  PRIMARY KEY (volcano_id, curve_id)
);

-- ------------------------------------------------- typed curve relations

CREATE TYPE ec.relation_kind AS ENUM (
  'isomorphism',              -- (x, y) -> (u^2 x + r, u^3 y + s u^2 x + t) over the stated field
  'quadratic_twist', 'cubic_twist', 'quartic_twist', 'sextic_twist',
  'base_change',              -- same equation over an extension (field_embedding_id)
  'frobenius_conjugate',      -- E^(p^k): apply Frobenius to the coefficients
  'birational_model_change',  -- Montgomery <-> Weierstrass, Edwards <-> Montgomery, ...
  'reduction_mod_p',          -- global curve -> its reduction
  'cm_lift',                  -- finite-field curve -> a CM lift
  'quotient_by_automorphism',
  'other'
);

CREATE TABLE ec.curve_relation (
  id                  bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  source_curve_id     bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  target_curve_id     bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  kind                ec.relation_kind NOT NULL,
  field_of_definition_id bigint REFERENCES ec.field(id),
  field_embedding_id  bigint REFERENCES ec.field_embedding(id),
  twist_parameter     ec.fe,
  map                 jsonb,                -- u, r, s, t or explicit rational maps
  prime               ec.pos,               -- reduction_mod_p
  status              ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id       bigint REFERENCES ec.provenance(id),
  CHECK (source_curve_id <> target_curve_id OR kind = 'quotient_by_automorphism'),
  UNIQUE NULLS NOT DISTINCT (source_curve_id, target_curve_id, kind, field_embedding_id)
);
COMMENT ON TABLE ec.curve_relation IS
  'Typed non-isogeny relations. A twist shares j but not the group; base change and model changes keep separate identities.';
CREATE INDEX curve_relation_target_idx ON ec.curve_relation (target_curve_id, kind);

-- ------------------------------------------------- covers and Weil descent

CREATE TYPE ec.cover_kind AS ENUM (
  'ghs_weil_descent',          -- Gaudry–Hess–Smart
  'generalized_ghs',           -- Hess; Momose–Chao
  'diem_descent',
  'isogeny_then_ghs',          -- Galbraith–Hess–Smart / Menezes–Teske–Weng isogeny to a weak curve first
  'hyperelliptic_pullback',    -- explicit H -> E by substitution (docs/curves/COVERS.md)
  'scholten',                  -- genus-2 cover of E over F_{p^2}
  'gluing_2_2',                -- (2,2)-gluing of two elliptic curves (Kani; Howe–Leprévost–Poonen)
  'frey_kani',
  'satoh',
  'quotient',                  -- C -> C / <sigma>
  'other'
);

CREATE TABLE ec.curve_morphism (
  id                 bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  source_curve_id    bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,  -- the cover C (higher genus)
  target_curve_id    bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,  -- E
  kind               ec.cover_kind NOT NULL,
  construction       text,                -- producer construction id, e.g. prime_quadratic_pullback_v1
  degree             ec.pos,
  source_field_id    bigint REFERENCES ec.field(id),  -- field the cover is defined over
  target_field_id    bigint REFERENCES ec.field(id),
  descent_embedding_id bigint REFERENCES ec.field_embedding(id),
  descent_degree     integer,             -- n = [K : k]
  ghs_magic_number   integer,             -- m = dim span{(1, sigma^i(sqrt b))}
  cover_genus        integer,
  map                jsonb,               -- e.g. {"x": [...], "y_v": [...], "y_0": [...]} : x = x(u), y = y_v(u) v + y_0(u)
  map_artifact_id    bigint REFERENCES ec.artifact(id),
  map_direction      text,
  field_relation     text,                -- same_field, subfield, extension
  exists_over_base   boolean,
  minimal_degree     ec.pos,
  minimal_genus      integer,
  subfield_descent   text,
  subgroup_transfer  text,                -- whether the DLP subgroup is carried; 'not_tested' when unchecked
  claim_contract     jsonb,               -- per-claim statuses (docs/curves/cover-transfer-assessments.json)
  status             ec.fact_status NOT NULL DEFAULT 'unknown',
  method             text,
  provenance_id      bigint REFERENCES ec.provenance(id),
  UNIQUE NULLS NOT DISTINCT (source_curve_id, target_curve_id, kind, construction)
);
COMMENT ON TABLE ec.curve_morphism IS
  'Non-constant morphisms of curves C -> E: covers, GHS / Weil descent, gluings. Existence, explicit construction, subgroup action and DLP advantage are separate claims.';
CREATE INDEX curve_morphism_target_idx ON ec.curve_morphism (target_curve_id, kind);
CREATE INDEX curve_morphism_source_idx ON ec.curve_morphism (source_curve_id);

CREATE TYPE ec.transfer_target AS ENUM (
  'jacobian',                    -- Jac(C) of a cover / descended curve
  'weil_restriction',
  'finite_field_multiplicative', -- MOV / Frey–Rück via a pairing
  'additive_group',              -- Smart / Satoh–Araki / Semaev, anomalous curves
  'isogenous_curve',
  'extension_field_curve',       -- the same curve over an extension (summation polynomials)
  'subgroup_factor'              -- Pohlig–Hellman reduction
);

CREATE TABLE ec.dlp_transfer (
  id                  bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  source_curve_id     bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  source_subgroup_id  bigint REFERENCES ec.subgroup(id),
  route_index         integer,
  route_kind          text NOT NULL,       -- ghs_weil_descent, same_field_cover, base_change_descent, mov, smart_anomalous, ...
  label               text,
  target              ec.transfer_target NOT NULL,
  target_av_id        bigint REFERENCES ec.abelian_variety(id),
  target_field_id     bigint REFERENCES ec.field(id),
  target_genus        ec.pos,
  target_field_bits   numeric,
  morphism_id         bigint REFERENCES ec.curve_morphism(id),
  isogeny_route_id    bigint REFERENCES ec.isogeny_route(id),
  evidence            text,                -- exact_invariant, construction, formula, measurement
  transport           text,                -- carried, carried_if_coprime_to_degree, degenerate, ...
  invariants          jsonb,
  cost                jsonb,               -- full producer cost record (search, transfer, destination solver)
  log2_transfer_cost  numeric,
  log2_target_dlp_cost numeric,
  log2_total_cost     numeric,
  log2_reference_cost numeric,             -- matched rho reference
  ratio_log2          numeric,             -- reference - total; positive is a formula advantage
  verdict             text,                -- formula_advantage, formula_marginal, boundary, not_carried, ...
  notes               text[],
  status              ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id       bigint REFERENCES ec.provenance(id),   -- its parameters hold the cost model
  UNIQUE NULLS NOT DISTINCT (source_curve_id, route_kind, route_index)
);
COMMENT ON TABLE ec.dlp_transfer IS
  'Routes that move a DLP instance to another group, with the full cost accounting. A formula advantage is a lead, not a measured speedup.';
CREATE INDEX dlp_transfer_source_idx ON ec.dlp_transfer (source_curve_id);
CREATE INDEX dlp_transfer_kind_idx ON ec.dlp_transfer (route_kind, verdict);

-- ------------------------------------------------- certificates

CREATE TABLE ec.certificate (
  id               bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  certificate_uid  text UNIQUE,
  kind             text NOT NULL,          -- order, volcano_level, kernel, jacobian_gluing, cover, primality, irreducibility, ...
  subject_table    text NOT NULL CHECK (subject_table IN ('curve','curve_representation','isogeny','curve_morphism',
                                                          'abelian_variety','field','subgroup','dlp_transfer','none')),
  subject_id       bigint,
  payload          jsonb NOT NULL,
  required_hypotheses text[],
  artifact_id      bigint REFERENCES ec.artifact(id),
  status           ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id    bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.certificate IS 'Machine-checkable certificates (order, volcano level, kernel, Jacobian gluing, cover) and the hypotheses they rest on.';
CREATE INDEX certificate_subject_idx ON ec.certificate (subject_table, subject_id);
