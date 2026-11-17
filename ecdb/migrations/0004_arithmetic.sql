-- ecdb 0004: arithmetic over finite fields — Weil polynomials and isogeny
-- classes, Frobenius, endomorphism rings and volcano levels, automorphisms
-- and explicit endomorphisms (GLV / GLS / tau-adic), group structure, twists,
-- subfield descent, pairings / embedding degree, p-adic invariants.
--
-- Three orders are kept apart (docs/curves/TRAITS.md, "Three orders"):
--   Z[pi]   determined by the Weil polynomial      -> isogeny_class_fq / frobenius_genus1
--   End(E)  between them, a property of the curve  -> endomorphism_ring / endomorphism_level
--   O_K     the maximal order of Q(pi)              -> frobenius_genus1.cm_discriminant

-- ------------------------------------------------- isogeny classes over F_q

CREATE TABLE ec.isogeny_class_fq (
  id                   bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  q                    ec.pos NOT NULL,
  p                    ec.pos NOT NULL,
  r                    integer NOT NULL CHECK (r >= 1),  -- q = p^r
  g                    integer NOT NULL CHECK (g >= 1),
  l_polynomial         ec.zint[] NOT NULL,   -- L(T) = 1 + a_1 T + ... + q^g T^{2g}, ascending (LMFDB "poly" extended to degree 2g)
  label                text NOT NULL UNIQUE, -- LMFDB-style g.q.<a_1>_..._<a_g>, computed for every q
  trace                numeric GENERATED ALWAYS AS (- l_polynomial[2]) STORED,  -- trace of Frobenius
  order_value          ec.pos,               -- #A(F_q) = L(1)
  order_bits           integer,
  point_counts         ec.zint[],            -- #A(F_{q^k}), k = 1..n
  curve_counts         ec.zint[],            -- q^k + 1 - sum alpha_i^k (formal for non-Jacobians)
  real_weil_polynomial ec.zint[],
  p_rank               integer,
  newton_slopes        text[],               -- e.g. {0, 1/2, 1/2, 1}
  is_ordinary          boolean,
  is_supersingular     boolean,
  is_almost_ordinary   boolean,
  is_simple            boolean,
  is_geometrically_simple boolean,
  simple_factor_labels text[],
  simple_multiplicities integer[],
  is_squarefree        boolean,
  angle_rank           integer,
  frobenius_field_id   bigint REFERENCES ec.number_field(id),   -- Q(pi) when simple
  has_jacobian         boolean,
  has_principal_polarization boolean,
  jacobian_count       integer,
  twist_labels         text[],
  lmfdb                jsonb,                -- remaining LMFDB av_fq_isog columns, verbatim
  status               ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id        bigint REFERENCES ec.provenance(id),
  CHECK (cardinality(l_polynomial) = 2 * g + 1),
  CHECK (l_polynomial[1] = 1),
  CHECK (l_polynomial[2 * g + 1] = power(q::numeric, g)),
  CHECK (q = power(p::numeric, r)),
  -- Hasse–Weil for g = 1: t^2 <= 4q
  CHECK (g <> 1 OR l_polynomial[2] * l_polynomial[2] <= 4 * q),
  UNIQUE (q, l_polynomial)
);
COMMENT ON TABLE ec.isogeny_class_fq IS
  'Isogeny classes of abelian varieties over F_q, determined by the Weil polynomial (Tate). For g = 1, L(T) = 1 - tT + qT^2 and #E(F_q) = q + 1 - t.';
COMMENT ON COLUMN ec.isogeny_class_fq.label IS
  'g.q.a_1_..._a_g with each a_i in LMFDB base-26 letters (a = 0, negative values prefixed by a). Computed for any q, not only LMFDB''s range.';
CREATE INDEX isogeny_class_trace_idx ON ec.isogeny_class_fq (q, trace);
CREATE INDEX isogeny_class_g_idx ON ec.isogeny_class_fq (g, p, r);

ALTER TABLE ec.abelian_variety
  ADD CONSTRAINT abelian_variety_isogeny_class_fk FOREIGN KEY (isogeny_class_id) REFERENCES ec.isogeny_class_fq(id);

CREATE TABLE ec.curve_zeta (
  curve_id        bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  isogeny_class_id bigint REFERENCES ec.isogeny_class_fq(id),  -- class of Jac(C)
  point_counts    ec.zint[] NOT NULL,       -- #C(F_{q^k}), k = 1..n
  order_check     ec.fact_status NOT NULL DEFAULT 'unknown',
  order_method    text,                     -- exhaustive count, generator certificate, SEA, CM, ...
  provenance_id   bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.curve_zeta IS 'Point counts of the curve itself; Z(C, T) = L(T) / ((1 - T)(1 - qT)).';

-- ------------------------------------------------- Frobenius (genus 1)

CREATE TABLE ec.frobenius_genus1 (
  isogeny_class_id      bigint PRIMARY KEY REFERENCES ec.isogeny_class_fq(id) ON DELETE CASCADE,
  trace                 ec.zint NOT NULL,
  trace_ratio           numeric,          -- t / 2 sqrt(q), size-free
  trace_mod_p           ec.zint,          -- t mod p: anomalous (Smart / SSSA) when 1 and n = 1; supersingular when 0
  frobenius_discriminant ec.zint NOT NULL,  -- Delta = t^2 - 4q = v^2 d_K
  disc_factorization    jsonb,            -- [[prime, exponent], ...]
  disc_unfactored       ec.zint[],        -- composite cofactors left unsplit
  disc_status           ec.fact_status NOT NULL DEFAULT 'unknown',
  cm_discriminant       ec.zint,          -- d_K, fundamental discriminant of Q(pi)
  cm_field_id           bigint REFERENCES ec.number_field(id),
  conductor_v           ec.pos,           -- v = [O_K : Z[pi]]
  conductor_bits        integer,
  conductor_factorization jsonb,
  conductor_unfactored  ec.zint[],
  conductor_fraction    numeric,          -- log v / log sqrt|Delta|
  class_number          ec.pos,           -- h(d_K)
  class_number_status   ec.fact_status NOT NULL DEFAULT 'unknown',
  is_anomalous          boolean,          -- #E(F_q) divisible by the characteristic (trace = 1 mod p)
  is_supersingular      boolean,
  status                ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id         bigint REFERENCES ec.provenance(id),
  CHECK (cm_discriminant IS NULL OR conductor_v IS NULL OR frobenius_discriminant = conductor_v * conductor_v * cm_discriminant)
);
COMMENT ON TABLE ec.frobenius_genus1 IS
  'Frobenius data of a genus-1 isogeny class: Delta = t^2 - 4q = v^2 d_K, conductor of Z[pi], class number of O_K.';
CREATE INDEX frobenius_cm_idx ON ec.frobenius_genus1 (cm_discriminant);
CREATE INDEX frobenius_class_number_idx ON ec.frobenius_genus1 (class_number);

CREATE TABLE ec.frobenius_ell (
  isogeny_class_id   bigint NOT NULL REFERENCES ec.isogeny_class_fq(id) ON DELETE CASCADE,
  ell                ec.pos NOT NULL,
  volcano_depth      integer,              -- v_ell(v): depth of the ell-isogeny volcano
  splitting          text CHECK (splitting IN ('split','inert','ramified')),  -- of ell in Q(pi)
  rational_isogenies integer,              -- F_q-rational ell-isogenies, only where the class fixes it (depth 0)
  status             ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id      bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (isogeny_class_id, ell)
);
COMMENT ON TABLE ec.frobenius_ell IS
  'Per-prime data the Weil polynomial fixes: volcano depth v_ell(v), splitting of ell in Q(pi), and the rational ell-isogeny count when ell does not divide v. Curve-dependent torsion data is in ec.curve_ell_torsion.';

-- When ell | v, Frobenius on E[ell] is the scalar t/2 or a Jordan block
-- depending on the curve's level in the ell-volcano (docs/curves/TRAITS.md),
-- so the action, the torsion and isogeny fields of definition and the
-- factorization of psi_ell belong to the curve, not the isogeny class.
CREATE TABLE ec.curve_ell_torsion (
  curve_id           bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  ell                ec.pos NOT NULL,
  frobenius_action   text,                 -- on E[ell]: 'scalar','split','inert','ramified','jordan'
  rational_isogenies integer,              -- F_q-rational ell-isogenies of this curve
  torsion_point_degree integer,            -- least k with a point of order ell over F_{q^k}
  full_torsion_degree integer,             -- least k with E[ell] subset E(F_{q^k})
  isogeny_field_degree integer,            -- least k over which an ell-isogeny is defined
  all_isogenies_degree integer,            -- least k over which every ell-isogeny is defined
  division_poly_factor_degrees text,       -- factorization pattern of psi_ell
  status             ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id      bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (curve_id, ell)
);
COMMENT ON TABLE ec.curve_ell_torsion IS
  'Per-curve ell-torsion: Frobenius action on E[ell], torsion and isogeny fields of definition, rational ell-isogenies, psi_ell factor degrees.';

-- ------------------------------------------------- endomorphisms

CREATE TABLE ec.endomorphism_ring (
  curve_id              bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  over_closure          boolean NOT NULL DEFAULT false,  -- End over the algebraic closure rather than the base field
  kind                  text NOT NULL CHECK (kind IN ('imaginary_quadratic_order','quaternion_maximal_order',
                                                       'quaternion_order','rational_integers','cm_order_higher_genus','unknown')),
  discriminant          ec.zint,          -- disc End(E) = f^2 d_K
  conductor_f           ec.pos,           -- [O_K : End(E)]
  conductor_divides     ec.pos,           -- established bound f | f_0
  is_maximal            boolean,
  contains              text[],           -- orders known to lie in End(E), e.g. Z[pi], Z[tau], Z[zeta_3]
  cm_field_id           bigint REFERENCES ec.number_field(id),
  quaternion_ramification ec.pos[],       -- supersingular: primes where B ramifies (p and infinity)
  quaternion_order_basis jsonb,           -- basis of the maximal order in B_{p,inf}
  status                ec.fact_status NOT NULL DEFAULT 'unknown',
  method                text,
  provenance_id         bigint REFERENCES ec.provenance(id),
  CHECK (conductor_f IS NULL OR conductor_divides IS NULL OR mod(conductor_divides, conductor_f) = 0)
);
COMMENT ON TABLE ec.endomorphism_ring IS
  'End(E): conductor f with f | v, maximality, the orders it is known to contain; maximal orders in B_{p,inf} for supersingular curves.';
CREATE INDEX endomorphism_disc_idx ON ec.endomorphism_ring (discriminant);

CREATE TABLE ec.endomorphism_level (
  curve_id        bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  ell             ec.pos NOT NULL,
  depth           integer,                -- v_ell(v)
  level           integer,                -- v_ell(f): 0 on the crater, depth on the floor
  level_at_most   integer,
  method          text,
  status          ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id   bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (curve_id, ell),
  CHECK (level IS NULL OR (level >= 0 AND (depth IS NULL OR level <= depth)))
);
COMMENT ON TABLE ec.endomorphism_level IS 'Position of the curve in each ell-isogeny volcano (Kohel; Fouquet–Morain).';

CREATE TABLE ec.automorphism_group (
  curve_id         bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  group_order      integer NOT NULL,      -- 2, 4, 6 for ordinary genus 1; up to 24 in char 2/3
  group_label      text,                  -- C2, C4, C6, SL(2,3), C3:C4, ...
  geometric_order  integer,
  geometric_label  text,
  generators       jsonb,
  status           ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id    bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.automorphism_group IS 'Aut over the base field and over the closure; drives the rho automorphism fold.';

CREATE TYPE ec.endomorphism_kind AS ENUM (
  'frobenius', 'verschiebung', 'multiplication', 'negation', 'automorphism',
  'cm', 'glv', 'gls', 'tau', 'distortion_map', 'other'
);

CREATE TABLE ec.endomorphism (
  id                  bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  curve_id            bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  kind                ec.endomorphism_kind NOT NULL,
  name                text,
  degree              ec.pos,
  characteristic_polynomial ec.zint[],    -- ascending, over Z
  trace               ec.zint,
  norm                ec.pos,
  field_of_definition_id bigint REFERENCES ec.field(id),
  is_separable        boolean,
  rational_map        jsonb,              -- {"x": {"num": [...], "den": [...]}, "y": ...}
  formula             text,               -- e.g. (x, y) -> (beta x, y)
  eigenvalue          ec.zint,            -- lambda with phi(P) = [lambda] P on the subgroup
  subgroup_id         bigint REFERENCES ec.subgroup(id),
  evaluation_cost     jsonb,              -- field multiplications / squarings / inversions
  status              ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id       bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.endomorphism IS 'Explicit endomorphisms: Frobenius, tau, GLV CM maps, GLS, automorphisms, distortion maps; eigenvalue on the target subgroup.';
CREATE INDEX endomorphism_curve_idx ON ec.endomorphism (curve_id, kind);

CREATE TABLE ec.scalar_decomposition (
  id               bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  curve_id         bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  subgroup_id      bigint REFERENCES ec.subgroup(id),
  endomorphism_ids bigint[] NOT NULL,     -- phi_1, ..., phi_{d-1}
  dimension        integer NOT NULL,      -- 2 GLV, 2 GLS, 4 GLV+GLS, ...
  lattice_basis    jsonb NOT NULL,        -- short basis of {(k_i) : sum k_i lambda_i = 0 mod r}
  max_component_bits integer,
  method           text,
  status           ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id    bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.scalar_decomposition IS 'GLV / GLS decompositions: the lattice and the resulting component sizes.';

CREATE TABLE ec.tau_adic_data (
  curve_id         bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  base_field_id    bigint REFERENCES ec.field(id),  -- the subfield whose Frobenius is tau (GF(2) for Koblitz)
  base_degree      integer NOT NULL,                 -- k, with the curve defined over GF(p^k)
  extension_degree integer NOT NULL,                 -- n = [working field : base field]
  koblitz_a        smallint CHECK (koblitz_a IN (0, 1)),
  mu               smallint CHECK (mu IN (-1, 1)),   -- (-1)^(1-a)
  tau_char_poly    ec.zint[] NOT NULL,               -- ascending: tau^2 - t_k tau + p^k
  base_trace       ec.zint,                          -- t_k
  lucas_v_n        ec.zint,                          -- V_n(t_k, p^k): trace over the working field
  partial_reduction jsonb,                           -- s_0, s_1 with delta = (tau^n - 1)/(tau - 1)
  delta_norm       ec.pos,
  status           ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id    bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.tau_adic_data IS 'Koblitz and subfield curves: Frobenius tau of the subfield, tau-adic (TNAF) reduction constants.';

-- ------------------------------------------------- descent, twists, groups

CREATE TABLE ec.descent_data (
  curve_id             bigint PRIMARY KEY REFERENCES ec.curve(id) ON DELETE CASCADE,
  j_field_degree       integer,          -- least k with j in GF(p^k)
  definition_degree    integer,          -- least k such that the curve is a base change from GF(p^k)
  base_curve_id        bigint REFERENCES ec.curve(id),
  base_trace           ec.zint,
  base_trace_sign_free boolean,          -- n/k even: only |t_k| is an invariant
  status               ec.fact_status NOT NULL DEFAULT 'unknown',
  method               text,
  provenance_id        bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.descent_data IS 'Subfield structure: j can lie in a subfield the curve does not descend to (docs/curves/TRAITS.md, Subfields).';

CREATE TABLE ec.twist_data (
  curve_id             bigint NOT NULL REFERENCES ec.curve(id) ON DELETE CASCADE,
  twist_degree         integer NOT NULL CHECK (twist_degree IN (2, 3, 4, 6)),
  twist_index          integer NOT NULL DEFAULT 1,   -- several non-isomorphic twists of one degree
  twist_curve_id       bigint REFERENCES ec.curve(id),
  twist_order          ec.pos,
  twist_order_factorization jsonb,
  largest_prime_bits   integer,
  cofactor             ec.pos,
  status               ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id        bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (curve_id, twist_degree, twist_index)
);
COMMENT ON TABLE ec.twist_data IS 'Twist orders and their factorization (twist security, invalid-curve exposure of x-only ladders).';

CREATE TABLE ec.group_structure (
  av_id             bigint NOT NULL REFERENCES ec.abelian_variety(id) ON DELETE CASCADE,
  extension_degree  integer NOT NULL DEFAULT 1,     -- A(F_{q^k})
  invariant_factors ec.pos[],                       -- n_1 | n_2 | ... (Z/n_1 x Z/n_2 x ...)
  order_value       ec.pos NOT NULL,
  order_factorization jsonb,
  order_unfactored  ec.zint[],
  is_cyclic         boolean,
  status            ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id     bigint REFERENCES ec.provenance(id),
  PRIMARY KEY (av_id, extension_degree)
);
COMMENT ON TABLE ec.group_structure IS 'Structure of A(F_{q^k}) as an abstract group, with the factorization of its order.';

CREATE TABLE ec.pairing_data (
  subgroup_id          bigint PRIMARY KEY REFERENCES ec.subgroup(id) ON DELETE CASCADE,
  embedding_degree     ec.pos,           -- ord_r(q)
  embedding_lower_bound ec.pos,          -- when only q^k != 1 mod r for k below the bound is known
  complement_bits      integer,          -- bit length of (r - 1)/k
  target_field_bits    integer,          -- k log2 q
  rho_value            numeric,          -- log q / log r
  is_pairing_friendly  boolean,
  pairing_family       text,             -- BN, BLS12, BLS24, KSS18, MNT4, MNT6, supersingular, ...
  distortion_map_id    bigint REFERENCES ec.endomorphism(id),
  status               ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id        bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.pairing_data IS 'Embedding degree (MOV / Frey–Rück transfer target size) and pairing-friendly family data.';
CREATE INDEX pairing_embedding_idx ON ec.pairing_data (embedding_degree);

CREATE TABLE ec.p_adic_invariants (
  av_id               bigint PRIMARY KEY REFERENCES ec.abelian_variety(id) ON DELETE CASCADE,
  p_rank              integer,
  a_number            integer,
  hasse_witt_matrix   jsonb,
  cartier_manin_matrix jsonb,
  ekedahl_oort_type   text,
  is_superspecial     boolean,
  newton_polygon      jsonb,
  status              ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id       bigint REFERENCES ec.provenance(id)
);
COMMENT ON TABLE ec.p_adic_invariants IS 'p-rank, a-number, Hasse–Witt / Cartier–Manin matrices, Ekedahl–Oort type.';
