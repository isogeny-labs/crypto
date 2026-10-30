-- ecdb 0006: polynomials attached to curves and maps — division polynomials,
-- modular polynomials, Semaev summation polynomials, class polynomials.
-- Kernel polynomials live on ec.isogeny, Weil polynomials on
-- ec.isogeny_class_fq, defining polynomials on ec.field / ec.number_field.

CREATE TABLE ec.polynomial (
  id                 bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  sha256             ec.sha256_hex NOT NULL UNIQUE,  -- of the canonical coefficient JSON
  coefficient_ring   text NOT NULL CHECK (coefficient_ring IN ('Z','Q','field')),
  coefficient_field_id bigint REFERENCES ec.field(id),
  variables          text[] NOT NULL,
  representation     text NOT NULL CHECK (representation IN ('dense_univariate','dense_bivariate','sparse')),
  degrees            integer[] NOT NULL,              -- degree in each variable
  total_degree       integer,
  term_count         bigint,
  max_coefficient_bits integer,
  coefficients       jsonb,                           -- inline when small
  artifact_id        bigint REFERENCES ec.artifact(id), -- large polynomials live outside the row
  is_irreducible     boolean,
  factorization      jsonb,
  status             ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id      bigint REFERENCES ec.provenance(id),
  CHECK (coefficients IS NOT NULL OR artifact_id IS NOT NULL),
  CHECK (coefficient_ring <> 'field' OR coefficient_field_id IS NOT NULL),
  CHECK (cardinality(degrees) = cardinality(variables))
);
COMMENT ON TABLE ec.polynomial IS
  'Content-addressed polynomial store. Dense: ascending coefficients (nested for bivariate). Sparse: [[exponents...], coefficient]. Large ones in an artifact.';

CREATE TABLE ec.division_polynomial (
  curve_id       bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  n              integer NOT NULL CHECK (n >= 1),
  variant        text NOT NULL CHECK (variant IN ('psi','f_x_only','psi_squared')),
  polynomial_id  bigint NOT NULL REFERENCES ec.polynomial(id),
  factor_degrees integer[],                 -- degrees of the irreducible factors over the base field
  PRIMARY KEY (curve_id, n, variant)
);
COMMENT ON TABLE ec.division_polynomial IS 'psi_n (and its x-only form) of a genus-1 curve; factor degrees give the torsion fields of definition.';

CREATE TABLE ec.modular_polynomial (
  id             bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  level          integer NOT NULL CHECK (level >= 2),
  kind           text NOT NULL CHECK (kind IN ('classical','atkin','weber','canonical','eta','igusa_genus2','other')),
  field_id       bigint REFERENCES ec.field(id),  -- NULL: over Z; otherwise reduced into this field
  polynomial_id  bigint NOT NULL REFERENCES ec.polynomial(id),
  UNIQUE NULLS NOT DISTINCT (level, kind, field_id)
);
COMMENT ON TABLE ec.modular_polynomial IS 'Phi_l(X, Y) and its variants: roots of Phi_l(j(E), Y) are the j-invariants of l-isogenous curves.';

CREATE TABLE ec.summation_polynomial (
  id             bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  model          ec.curve_model NOT NULL,
  m              integer NOT NULL CHECK (m >= 2),
  curve_id       bigint REFERENCES ec.curve(id),  -- NULL: generic in the model coefficients
  symmetrized    boolean NOT NULL DEFAULT false,  -- expressed in elementary symmetric polynomials
  polynomial_id  bigint NOT NULL REFERENCES ec.polynomial(id),
  UNIQUE NULLS NOT DISTINCT (model, m, curve_id, symmetrized)
);
COMMENT ON TABLE ec.summation_polynomial IS 'Semaev summation polynomials S_m (index calculus on elliptic curves).';

CREATE TABLE ec.class_polynomial (
  id             bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  discriminant   ec.zint NOT NULL CHECK (discriminant < 0),
  kind           text NOT NULL CHECK (kind IN ('hilbert','weber','ramanujan','double_eta','atkin','igusa_genus2')),
  field_id       bigint REFERENCES ec.field(id),
  class_number   ec.pos,
  polynomial_id  bigint NOT NULL REFERENCES ec.polynomial(id),
  UNIQUE NULLS NOT DISTINCT (discriminant, kind, field_id)
);
COMMENT ON TABLE ec.class_polynomial IS 'Hilbert and alternative class polynomials (CM method); roots mod p are j-invariants with CM by the order of discriminant D.';
