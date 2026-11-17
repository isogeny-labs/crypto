-- ecdb 0003: curves of every genus and model, their invariants, names,
-- EC1 representations, points and subgroups.
--
-- Three identities are kept apart and never inferred from one another:
--   curve                 an exact model over an exact field representation
--                         (ICV1 model JSON; docs/curves/ICV1.md)
--   curve_representation  curve + subgroup + generator + target group
--                         (EC1 curve_uid; docs/curve-identities.md)
--   isomorphism / isogeny classes are separate rows reached through explicit,
--   sourced relations (0005). Equal j, order or degree never joins two curves.

CREATE TYPE ec.curve_model AS ENUM (
  -- genus 1, odd characteristic or general
  'short_weierstrass',        -- y^2 = x^3 + a x + b
  'long_weierstrass',         -- y^2 + a1 xy + a3 y = x^3 + a2 x^2 + a4 x + a6
  'montgomery',               -- B y^2 = x^3 + A x^2 + x
  'edwards',                  -- x^2 + y^2 = c^2 (1 + d x^2 y^2)
  'twisted_edwards',          -- a x^2 + y^2 = 1 + d x^2 y^2
  'legendre',                 -- y^2 = x (x - 1)(x - lambda)
  'hessian',                  -- x^3 + y^3 + z^3 = 3 d xyz
  'twisted_hessian',          -- a x^3 + y^3 + z^3 = d xyz
  'jacobi_quartic',           -- y^2 = x^4 + 2 a x^2 + 1 (or d x^4 + 2 a x^2 + 1)
  'jacobi_intersection',      -- s^2 + c^2 = 1, a s^2 + d^2 = 1
  'huff',                     -- a x (y^2 - 1) = b y (x^2 - 1)
  'doche_icart_kohel',        -- y^2 = x^3 + 3a(x+1)^2 and y^2 = x(x^2 + a x + 16a)
  'tate_normal',              -- y^2 + (1 - c) xy - b y = x^3 - b x^2
  'intersection_of_quadrics', -- genus-1 curve in P^3
  -- genus 1, characteristic 2
  'binary_weierstrass',       -- ordinary: y^2 + xy = x^3 + a x^2 + b
  'binary_supersingular',     -- y^2 + c y = x^3 + a x + b
  'binary_edwards',           -- d1(x + y) + d2(x^2 + y^2) = xy + xy(x + y) + x^2 y^2
  'binary_huff',
  -- characteristic 3
  'ternary_weierstrass',      -- y^2 = x^3 + a x^2 + b (ordinary) / x^3 + a x + b (supersingular)
  -- genus >= 2
  'hyperelliptic',            -- y^2 + h(x) y = f(x)
  'superelliptic',            -- y^n = f(x)
  'picard',                   -- y^3 = f(x), deg f = 4
  'c_ab',                     -- C_ab curve
  'plane_curve',              -- F(x, y, z) = 0, e.g. smooth plane quartics (non-hyperelliptic genus 3)
  'canonical_model',          -- canonical embedding, ideal generators
  'ideal'                     -- general: generators of an ideal in a (weighted) projective space
);

CREATE TABLE ec.curve (
  id               bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  field_id         bigint NOT NULL REFERENCES ec.field(id),
  genus            integer NOT NULL CHECK (genus >= 0),
  model            ec.curve_model NOT NULL,
  form             text NOT NULL,          -- ICV1 form string, e.g. y^2=x^3+a*x+b
  coefficients     jsonb NOT NULL,         -- named coefficients in the field encoding, e.g. {"a": "...", "b": "..."}
  equation         text,                   -- human-readable equation
  family           text,                   -- producer family: koblitz, binary, prime, extension, subfield, ...
  model_json       text,                   -- exact ICV1 model JSON (hash preimage)
  model_sha256     ec.sha256_hex,
  icv1             text UNIQUE,            -- full ICV1 string
  icv1_slug        text UNIQUE,            -- icv1-<field tag>-t<trace>-<model8>
  is_smooth        boolean,
  smoothness_status ec.fact_status NOT NULL DEFAULT 'unknown',
  geometrically_irreducible boolean,
  description      text,
  provenance_id    bigint REFERENCES ec.provenance(id),
  created_at       timestamptz NOT NULL DEFAULT now(),
  updated_at       timestamptz NOT NULL DEFAULT now(),
  CHECK (model_json IS NULL OR model_sha256 = ec.sha256_text(model_json)),
  CHECK (icv1_slug IS NULL OR icv1_slug ~ '^icv1-[a-z0-9]+-tm?[0-9]+-[0-9a-f]{8}$'),
  CHECK (icv1_slug IS NULL OR model_sha256 IS NULL OR right(icv1_slug, 8) = left(model_sha256, 8)),
  CHECK (model NOT IN ('short_weierstrass','long_weierstrass','montgomery','edwards','twisted_edwards',
                       'legendre','hessian','twisted_hessian','jacobi_quartic','jacobi_intersection',
                       'huff','doche_icart_kohel','tate_normal','intersection_of_quadrics',
                       'binary_weierstrass','binary_supersingular','binary_edwards','binary_huff',
                       'ternary_weierstrass') OR genus = 1),
  UNIQUE (field_id, model, coefficients)
);
COMMENT ON TABLE ec.curve IS
  'An exact curve model over an exact field representation, of any genus. ICV1 identity when the model is ICV1-nameable.';
COMMENT ON COLUMN ec.curve.model_json IS 'Exact ICV1 model JSON; model_sha256 is re-verified by CHECK and the slug suffix must match it.';
CREATE INDEX curve_field_idx ON ec.curve (field_id);
CREATE INDEX curve_genus_model_idx ON ec.curve (genus, model);
CREATE INDEX curve_family_idx ON ec.curve (family);
CREATE INDEX curve_coeff_gin ON ec.curve USING gin (coefficients jsonb_path_ops);
CREATE TRIGGER curve_touch BEFORE UPDATE ON ec.curve FOR EACH ROW EXECUTE FUNCTION ec.touch_updated_at();

-- ------------------------------------------------- genus-1 invariants

CREATE TABLE ec.genus1_invariants (
  curve_id        bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  a1 ec.fe, a2 ec.fe, a3 ec.fe, a4 ec.fe, a6 ec.fe,      -- long Weierstrass model (when one is attached)
  b2 ec.fe, b4 ec.fe, b6 ec.fe, b8 ec.fe,
  c4 ec.fe, c6 ec.fe,
  discriminant    ec.fe,
  j_invariant     ec.fe,
  j_is_0          boolean,
  j_is_1728       boolean,                 -- equal to j_is_0 in characteristic 2 and 3
  j_field_degree  integer,                 -- least k with j in the subfield of degree k
  to_weierstrass  jsonb,                   -- birational map from this model to the long Weierstrass model
  from_weierstrass jsonb,
  status          ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id   bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.genus1_invariants IS
  'a-, b-, c-invariants, discriminant and j of a genus-1 curve. j is shared by all twists: never an identity.';
CREATE INDEX genus1_j_idx ON ec.genus1_invariants (j_invariant);

-- ------------------------------------------------- higher-genus models

CREATE TABLE ec.hyperelliptic_model (
  curve_id        bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  f               jsonb NOT NULL,          -- ascending coefficients of f(x)
  h               jsonb NOT NULL DEFAULT '[]',  -- ascending coefficients of h(x); [] for h = 0
  deg_f           integer NOT NULL,
  deg_h           integer,
  infinity_type   text CHECK (infinity_type IN ('ramified','split','inert')), -- imaginary / real / unusual model
  points_at_infinity integer CHECK (points_at_infinity BETWEEN 0 AND 2),
  discriminant    ec.fe,
  weierstrass_points jsonb,                -- known Weierstrass points
  is_simplified   boolean GENERATED ALWAYS AS (h = '[]'::jsonb) STORED,
  status          ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id   bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.hyperelliptic_model IS
  'y^2 + h(x) y = f(x). Imaginary (ramified at infinity), real (split) or unusual (inert) model; genus = ceil(max(deg f, 2 deg h)/2) - 1.';

CREATE TABLE ec.curve_equation_model (
  curve_id        bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  ambient         text NOT NULL,           -- 'A^2', 'P^2', 'P^3', 'P(1,1,2)', ...
  variables       text[] NOT NULL,
  weights         integer[],               -- weighted projective space
  generators      jsonb NOT NULL,          -- list of sparse polynomials: [[[exponents...], coeff], ...]
  degree          integer,
  is_complete_intersection boolean,
  singular_points jsonb,
  status          ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id   bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.curve_equation_model IS
  'Plane, superelliptic, Picard, C_ab, canonical and general ideal models: explicit generators of the defining ideal.';

CREATE TYPE ec.invariant_system AS ENUM (
  'j',                     -- genus 1
  'igusa_clebsch',         -- genus 2 (I2, I4, I6, I10)
  'igusa',                 -- genus 2 (J2, J4, J6, J8, J10), valid in every characteristic
  'g2_cardona_quer',       -- absolute genus-2 invariants
  'absolute_igusa',
  'shioda',                -- genus-3 hyperelliptic
  'dixmier_ohno',          -- plane quartics
  'rosenhain',             -- Rosenhain form parameters
  'theta_constants',
  'cardona_nart_pujolas',  -- genus-2 invariants in characteristic 2
  'other'
);

CREATE TABLE ec.geometric_invariant (
  curve_id        bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  system          ec.invariant_system NOT NULL,
  invariant_values jsonb NOT NULL,         -- array of field elements
  weights         integer[],               -- weighted-projective weights; NULL for absolute invariants
  is_absolute     boolean NOT NULL,
  normalization   text,
  status          ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id   bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (curve_id, system)
);
COMMENT ON TABLE ec.geometric_invariant IS
  'Invariants that determine the curve up to isomorphism over the algebraic closure. Twists share them.';
CREATE INDEX geometric_invariant_values_gin ON ec.geometric_invariant USING gin (invariant_values jsonb_path_ops);

-- ------------------------------------------------- names

CREATE TYPE ec.alias_namespace AS ENUM (
  'icv1_slug', 'icv1', 'ec1', 'curve_uid', 'standard', 'oid', 'registry_alias',
  'retired', 'lmfdb', 'cremona', 'safecurves', 'challenge', 'other'
);

CREATE TABLE ec.curve_alias (
  id          bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  curve_id    bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  namespace   ec.alias_namespace NOT NULL,
  alias       text NOT NULL,
  is_retired  boolean NOT NULL DEFAULT false,
  source_id   bigint REFERENCES ec.source(id),
  UNIQUE (namespace, alias, curve_id)
);
COMMENT ON TABLE ec.curve_alias IS
  'Every spelling that denotes a curve. Retired forms (docs/curves/ICV1.md) resolve but are flagged; never write them in new text.';
CREATE INDEX curve_alias_lower_idx ON ec.curve_alias (lower(alias));
CREATE INDEX curve_alias_trgm ON ec.curve_alias USING gin (alias gin_trgm_ops);

CREATE TABLE ec.curve_standard (
  id                   bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  curve_id             bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  display_name         text NOT NULL,       -- sect163k1, P-256, brainpoolP256r1, ...
  category             text NOT NULL,       -- secg, nist, brainpool, x962, gost, bn, bls, mnt, ...
  oid                  text,
  source_ref           text,                -- e.g. other/M-221
  adoption_status      text,
  parameter_validation text,
  sources              jsonb,               -- [{name, url}, ...]
  provenance_id        bigint REFERENCES ec.provenance(id),
  UNIQUE NULLS NOT DISTINCT (curve_id, display_name, source_ref)
);
COMMENT ON TABLE ec.curve_standard IS 'Publication of a curve by a standards body or challenge, with OID and validation notes.';
CREATE INDEX curve_standard_name_idx ON ec.curve_standard (lower(display_name));

-- ------------------------------------------------- abelian varieties
-- Declared here because points, subgroups and group structures live on the
-- Jacobian; for genus 1 the curve is its own Jacobian.

CREATE TYPE ec.av_kind AS ENUM (
  'jacobian',           -- Jac(C); for genus 1, Jac(E) = E
  'product',            -- A_1 x ... x A_n (components in ec.av_component)
  'weil_restriction',   -- Res_{L/K} A
  'prym',               -- Prym variety of a double cover
  'quotient',           -- A / G for a finite subgroup G
  'isogeny_factor',     -- simple factor up to isogeny
  'generic'
);

CREATE TABLE ec.abelian_variety (
  id                    bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  field_id              bigint NOT NULL REFERENCES ec.field(id),
  dimension             integer NOT NULL CHECK (dimension >= 1),
  kind                  ec.av_kind NOT NULL,
  curve_id              bigint UNIQUE REFERENCES ec.curve(id) ON DELETE CASCADE, -- kind = jacobian
  isogeny_class_id      bigint,              -- FK added in 0004 (ec.isogeny_class_fq)
  polarization_type     ec.pos[],            -- (d_1 | d_2 | ... | d_g); {1,...,1} principal
  is_principally_polarized boolean,
  polarization_status   ec.fact_status NOT NULL DEFAULT 'unknown',
  is_simple             boolean,
  is_geometrically_simple boolean,
  simplicity_status     ec.fact_status NOT NULL DEFAULT 'unknown',
  description           text,
  provenance_id         bigint REFERENCES ec.provenance(id),
  CHECK (kind <> 'jacobian' OR curve_id IS NOT NULL)
);
COMMENT ON TABLE ec.abelian_variety IS
  'Abelian varieties: Jacobians (every curve of genus >= 1 has exactly one), products, Weil restrictions, Pryms, quotients.';
CREATE INDEX abelian_variety_class_idx ON ec.abelian_variety (isogeny_class_id);

CREATE TABLE ec.av_component (
  av_id            bigint NOT NULL REFERENCES ec.abelian_variety(id) ON DELETE CASCADE,
  component_av_id  bigint NOT NULL REFERENCES ec.abelian_variety(id),
  multiplicity     integer NOT NULL DEFAULT 1 CHECK (multiplicity >= 1),
  relation         text NOT NULL CHECK (relation IN ('product_factor','isogeny_factor','geometric_isogeny_factor')),
  map              jsonb,                 -- explicit projection / isogeny when known
  status           ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id    bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (av_id, component_av_id, relation)
);
COMMENT ON TABLE ec.av_component IS 'Decompositions: A = prod A_i (product) or A ~ prod A_i^e_i (up to isogeny, over the base or the closure).';

CREATE TABLE ec.weil_restriction (
  av_id            bigint PRIMARY KEY REFERENCES ec.abelian_variety(id) ON DELETE CASCADE,
  restricted_av_id bigint NOT NULL REFERENCES ec.abelian_variety(id),
  embedding_id     bigint NOT NULL REFERENCES ec.field_embedding(id),  -- k -> K; restriction from K down to k
  provenance_id    bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.weil_restriction IS 'Res_{K/k} A: the target of GHS / Weil descent.';

-- ------------------------------------------------- EC1 representations

CREATE TABLE ec.curve_representation (
  id                 bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  curve_id           bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  ec1_alias          text UNIQUE,          -- EC1N<m>C<tag>h<12hex> | EC1P<b>C... | EC1Q<p>D<n>C...
  curve_uid          text UNIQUE,          -- urn:ec-record:1:sha256:<64hex>
  curve_sha256       ec.sha256_hex UNIQUE,
  field_record_id    bigint REFERENCES ec.field_ec1_record(id),
  canonical_preimage text,                 -- {"curve":...,"field":...} sorted compact JSON; NULL when the producer's preimage is not on file
  curve_record       jsonb,                -- EC1 curve part
  model_label        text,                 -- 'short Weierstrass', 'binary Weierstrass', 'Montgomery', ...
  subgroup_order     ec.pos,
  cofactor           ec.pos,
  generator          jsonb,                -- coordinates exactly as recorded
  target_group       text,
  identity_verified  boolean GENERATED ALWAYS AS (canonical_preimage IS NOT NULL) STORED,
  sources            text[] NOT NULL DEFAULT '{}',
  provenance_id      bigint REFERENCES ec.provenance(id),
  CHECK (curve_uid IS NULL OR curve_uid = 'urn:ec-record:1:sha256:' || curve_sha256),
  CHECK (canonical_preimage IS NULL OR ec.sha256_text(canonical_preimage) = curve_sha256),
  CHECK (ec1_alias IS NULL OR ec1_alias ~ '^EC1(N[0-9]+|P[0-9]+|Q[0-9]+D[0-9]+)C[a-z][a-z0-9]*h[0-9a-f]{12,64}$'),
  CHECK (ec1_alias IS NULL OR curve_sha256 IS NULL OR curve_sha256 LIKE (substring(ec1_alias FROM 'h([0-9a-f]+)$') || '%'))
);
COMMENT ON TABLE ec.curve_representation IS
  'EC1 representation identity: curve + subgroup + generator + target group. curve_sha256 is re-verified against the stored preimage. Join across repositories on curve_uid, never on the compact alias.';
CREATE INDEX curve_representation_curve_idx ON ec.curve_representation (curve_id);

-- ------------------------------------------------- points and subgroups

CREATE TYPE ec.point_representation AS ENUM (
  'affine', 'projective', 'jacobian_coordinates', 'lopez_dahab', 'extended_edwards',
  'montgomery_xz', 'x_only', 'compressed', 'mumford', 'formal_sum', 'infinity'
);

CREATE TABLE ec.point (
  id               bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  curve_id         bigint REFERENCES ec.curve(id) ON DELETE CASCADE,
  av_id            bigint REFERENCES ec.abelian_variety(id) ON DELETE CASCADE,
  field_id         bigint NOT NULL REFERENCES ec.field(id),  -- field of definition of the coordinates (may be an extension)
  representation   ec.point_representation NOT NULL,
  coordinates      jsonb NOT NULL,       -- [x, y] / [X, Y, Z] / {"u": [...], "v": [...]} (Mumford)
  is_identity      boolean NOT NULL DEFAULT false,
  order_value      ec.pos,
  order_factorization jsonb,
  order_status     ec.fact_status NOT NULL DEFAULT 'unknown',
  role             text,                 -- 'generator', 'torsion_basis', 'weierstrass_point', 'base_point', 'certificate', ...
  label            text,
  provenance_id    bigint REFERENCES ec.provenance(id),
  CHECK (curve_id IS NOT NULL OR av_id IS NOT NULL),
  UNIQUE NULLS NOT DISTINCT (curve_id, av_id, field_id, representation, coordinates)
);
COMMENT ON TABLE ec.point IS
  'Points on curves and elements of abelian varieties (Mumford divisors for hyperelliptic Jacobians). For genus 1 both curve_id and av_id are set.';
CREATE INDEX point_curve_idx ON ec.point (curve_id);
CREATE INDEX point_av_idx ON ec.point (av_id);

CREATE TABLE ec.subgroup (
  id                 bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  av_id              bigint NOT NULL REFERENCES ec.abelian_variety(id) ON DELETE CASCADE,
  field_id           bigint NOT NULL REFERENCES ec.field(id),  -- the group A(field) it sits in
  order_value        ec.pos NOT NULL,
  order_bits         integer NOT NULL,
  is_prime_order     boolean,
  primality_status   ec.fact_status NOT NULL DEFAULT 'unknown',
  cofactor           ec.pos,              -- #A(field) / order
  generator_point_id bigint REFERENCES ec.point(id),
  structure          ec.pos[],            -- invariant factors when not cyclic
  representation_id  bigint REFERENCES ec.curve_representation(id),
  status             ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id      bigint REFERENCES ec.provenance(id),
  UNIQUE NULLS NOT DISTINCT (av_id, field_id, order_value, generator_point_id)
);
COMMENT ON TABLE ec.subgroup IS 'Subgroups of A(F_q^k), normally the prime-order target group of the DLP.';
CREATE INDEX subgroup_av_idx ON ec.subgroup (av_id);
CREATE INDEX subgroup_bits_idx ON ec.subgroup (order_bits);

-- Every curve of genus >= 1 gets its Jacobian row automatically.
CREATE FUNCTION ec.ensure_jacobian() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
  IF NEW.genus >= 1 THEN
    INSERT INTO ec.abelian_variety (field_id, dimension, kind, curve_id,
                                    polarization_type, is_principally_polarized, polarization_status)
    VALUES (NEW.field_id, NEW.genus, 'jacobian', NEW.id,
            array_fill(1::numeric, ARRAY[NEW.genus])::ec.pos[], true, 'proved')
    ON CONFLICT (curve_id) DO NOTHING;
  END IF;
  RETURN NEW;
END $$;
CREATE TRIGGER curve_jacobian AFTER INSERT ON ec.curve FOR EACH ROW EXECUTE FUNCTION ec.ensure_jacobian();
COMMENT ON FUNCTION ec.ensure_jacobian() IS
  'Creates Jac(C) with its canonical principal polarization for every new curve of genus >= 1.';
