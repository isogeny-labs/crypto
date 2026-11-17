-- ecdb 0007: curves over Q and number fields (global arithmetic, local data,
-- Galois images), generation provenance, security assessments, raw trait
-- snapshots, external catalog labels and links to research evidence.

-- ------------------------------------------------- global curves

CREATE TABLE ec.curve_global_data (
  curve_id              bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  conductor             ec.pos,            -- conductor (Q) or its norm (number field)
  conductor_ideal       text,
  conductor_factorization jsonb,
  minimal_discriminant  ec.zint,
  is_minimal_model      boolean,
  is_semistable         boolean,
  has_potential_good_reduction boolean,
  rank                  integer,
  rank_status           ec.fact_status NOT NULL DEFAULT 'unknown',
  analytic_rank         integer,           -- distinct from rank
  rank_bounds           int4range,
  torsion_structure     integer[],
  torsion_generators    jsonb,
  mordell_weil_generators jsonb,
  regulator             numeric,
  real_period           numeric,
  sha_analytic          numeric,
  tamagawa_product      ec.pos,
  root_number           smallint CHECK (root_number IN (-1, 1)),
  faltings_height       numeric,
  stable_faltings_height numeric,
  modular_degree        ec.pos,
  manin_constant        ec.pos,
  cm_discriminant       ec.zint,           -- 0 / NULL when no CM
  isogeny_class_label   text,
  isogeny_matrix        jsonb,
  adelic_image_label    text,
  adelic_image_index    ec.pos,
  bsd_status            ec.fact_status NOT NULL DEFAULT 'unknown',
  status                ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id         bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.curve_global_data IS 'Arithmetic of curves over Q and number fields (conductor, rank, torsion, BSD quantities, CM, isogeny class).';
CREATE INDEX curve_global_conductor_idx ON ec.curve_global_data (conductor);

CREATE TABLE ec.curve_local_data (
  curve_id              bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  prime_label           text NOT NULL,     -- p for Q, a prime-ideal label for number fields
  prime_norm            ec.pos NOT NULL,
  reduction_type        text CHECK (reduction_type IN ('good','split_multiplicative','nonsplit_multiplicative','additive')),
  kodaira_symbol        text,
  conductor_exponent    integer,
  discriminant_valuation integer,
  j_valuation           integer,
  tamagawa_number       ec.pos,
  local_root_number     smallint CHECK (local_root_number IN (-1, 1)),
  a_p                   ec.zint,           -- trace of Frobenius at good primes
  reduced_curve_id      bigint REFERENCES ec.curve(id),
  status                ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id         bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (curve_id, prime_label)
);
COMMENT ON TABLE ec.curve_local_data IS 'Tate''s algorithm per prime: reduction type, Kodaira symbol, Tamagawa number; link to the reduced curve over the residue field.';

CREATE TABLE ec.galois_image (
  curve_id        bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  ell             ec.pos NOT NULL,
  image_label     text,
  image_index     ec.pos,
  is_surjective   boolean,
  status          ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id   bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (curve_id, ell)
);
COMMENT ON TABLE ec.galois_image IS 'Images of the mod-ell / ell-adic Galois representations.';

-- ------------------------------------------------- generation and security

CREATE TABLE ec.generation_provenance (
  curve_id         bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  method           text NOT NULL CHECK (method IN ('verifiably_random_x962','brainpool_seeded','rigid_minimal',
                                                    'cm_method','koblitz','subfield','random','seeded_generator',
                                                    'isogeny_walk','standard_published','pairing_family','unknown')),
  seed             text,
  hash_function    text,
  procedure_ref    text,
  generator_call   text,               -- producer constructor, e.g. KoblitzCurve::new(0, 7)
  is_reproducible  boolean,
  status           ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id    bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.generation_provenance IS 'How the curve was generated (seed, procedure, rigidity).';

CREATE TABLE ec.security_assessment (
  id                        bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  subgroup_id               bigint NOT NULL REFERENCES ec.subgroup(id) ON DELETE CASCADE,
  rho_log2                  numeric,      -- log2 of expected rho group operations, sqrt(pi r / 4)
  automorphism_fold         integer,      -- equivalence classes per orbit used by the matched rho
  rho_log2_folded           numeric,
  pohlig_hellman_bits       integer,      -- largest prime factor of the subgroup order
  embedding_degree          ec.pos,
  is_anomalous              boolean,
  twist_security_bits       integer,
  cm_discriminant_bits      integer,
  cheon_r_minus_1           jsonb,        -- factorization of r - 1
  cheon_r_plus_1            jsonb,
  ghs_susceptible           boolean,
  complete_addition_law     boolean,
  ladder_friendly           boolean,
  elligator                 text CHECK (elligator IN ('elligator1','elligator2','elligator_squared','none')),
  safecurves                jsonb,        -- per-criterion verdicts
  notes                     text,
  status                    ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id             bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.security_assessment IS
  'Known-attack surface per target subgroup. A recorded value is a structural fact, not a security verdict or a measured attack cost.';
CREATE INDEX security_assessment_subgroup_idx ON ec.security_assessment (subgroup_id);

CREATE TABLE ec.trait_record (
  curve_id        bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  schema          text NOT NULL,        -- e.g. curve-traits/2
  source_id       bigint NOT NULL REFERENCES ec.source(id),
  record          jsonb NOT NULL,       -- the producer's record, verbatim
  PRIMARY KEY (curve_id, schema, source_id)
);
COMMENT ON TABLE ec.trait_record IS 'Verbatim producer trait records (DiSSECT families, kn factorizations, Hamming weights, ...), queryable with jsonb operators.';
CREATE INDEX trait_record_gin ON ec.trait_record USING gin (record jsonb_path_ops);

CREATE TABLE ec.external_label (
  id            bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  object_table  text NOT NULL CHECK (object_table IN ('curve','isogeny_class_fq','number_field','field','abelian_variety')),
  object_id     bigint NOT NULL,
  catalog       text NOT NULL,          -- lmfdb, cremona, safecurves, dissect, ecc_challenge, ...
  label         text NOT NULL,
  url           text,
  match_status  ec.fact_status NOT NULL DEFAULT 'unknown',  -- the model match itself must be established
  provenance_id bigint REFERENCES ec.provenance(id),
  UNIQUE (object_table, object_id, catalog, label)
);
COMMENT ON TABLE ec.external_label IS 'Crosswalk to external catalogs. A label does not establish identity until the exact model match is proved.';
CREATE INDEX external_label_lookup_idx ON ec.external_label (catalog, label);

CREATE TABLE ec.evidence_link (
  id                 bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  curve_id           bigint REFERENCES ec.curve(id) ON DELETE CASCADE,
  representation_id  bigint REFERENCES ec.curve_representation(id) ON DELETE CASCADE,
  kind               text NOT NULL,     -- ecbench_session, isogeny_walk, ic_run, rho_run, cover_search, paper, report
  repository         text,
  path               text,
  sha256             ec.sha256_hex,
  summary            jsonb,
  validation_status  text,              -- e.g. validated, unvalidated (pre-gate)
  provenance_id      bigint REFERENCES ec.provenance(id),
  created_at         timestamptz NOT NULL DEFAULT now(),
  CHECK (curve_id IS NOT NULL OR representation_id IS NOT NULL),
  UNIQUE NULLS NOT DISTINCT (curve_id, representation_id, kind, repository, path)
);
COMMENT ON TABLE ec.evidence_link IS 'Pointers from a curve to research runs and reports about it, so agents find prior work before repeating it.';
CREATE INDEX evidence_link_curve_idx ON ec.evidence_link (curve_id);
