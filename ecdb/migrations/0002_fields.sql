-- ecdb 0002: base fields.
--
-- A field row is an exact *representation* (modulus / basis / encoding), not
-- an isomorphism class: GF(2^163) under two moduli is two rows, because a
-- coefficient, a factor base or a log table means nothing under another
-- modulus (docs/curves/ICV1.md, "The modulus is part of the model").

CREATE TABLE ec.number_field (
  id                     bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  lmfdb_label            text UNIQUE,            -- e.g. 2.0.7.1
  degree                 integer NOT NULL CHECK (degree >= 1),
  signature_r1           integer CHECK (signature_r1 >= 0),
  signature_r2           integer CHECK (signature_r2 >= 0),
  defining_polynomial    ec.zint[] NOT NULL,     -- ascending coefficients, monic
  discriminant           ec.zint,
  discriminant_factorization jsonb,             -- [[p, e], ...]
  fundamental_discriminant ec.zint,             -- quadratic fields: d_K
  class_number           ec.pos,
  class_group            ec.pos[],               -- invariant factors
  regulator              numeric,
  is_galois              boolean,
  galois_group_label     text,                   -- transitive group label, e.g. 2T1
  is_cm                  boolean,
  is_imaginary_quadratic boolean GENERATED ALWAYS AS (degree = 2 AND signature_r1 = 0) STORED,
  integral_basis         jsonb,                  -- basis of O_K in the power basis
  status                 ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id          bigint REFERENCES ec.provenance(id),
  CHECK (cardinality(defining_polynomial) = degree + 1),
  CHECK (signature_r1 IS NULL OR signature_r2 IS NULL OR signature_r1 + 2 * signature_r2 = degree),
  UNIQUE (defining_polynomial)
);
COMMENT ON TABLE ec.number_field IS
  'Number fields: CM fields Q(pi) of Frobenius, endomorphism algebras, base fields of global curves, splitting fields.';
CREATE INDEX number_field_fdisc_idx ON ec.number_field (fundamental_discriminant) WHERE fundamental_discriminant IS NOT NULL;

CREATE TABLE ec.field (
  id                   bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  kind                 ec.field_kind NOT NULL,
  characteristic       ec.nat NOT NULL,            -- 0 for characteristic zero
  degree               integer CHECK (degree >= 1),-- absolute degree over the prime field (finite) or Q
  order_q              ec.pos,                     -- q = p^degree for finite fields
  order_bits           integer,                    -- ceil(log2 q): bit length of q - 1
  base_field_id        bigint REFERENCES ec.field(id), -- towers: this = base[t]/(defining_polynomial)
  relative_degree      integer CHECK (relative_degree >= 1),
  representation       ec.field_representation NOT NULL,
  defining_polynomial  ec.zint[],                  -- ascending, monic, over the prime field (or base field residues)
  modulus_exponents    integer[],                  -- sparse binary modulus: exponents of nonzero terms, descending (EC1 field record)
  modulus_hex          text,                       -- binary modulus as ICV1 writes it, e.g. 0x20000000009
  polynomial_shape     text CHECK (polynomial_shape IN ('trinomial','pentanomial','all_one','sparse','dense')),
  normal_basis_type    integer,                    -- Gaussian normal basis type T
  element_encoding     ec.fe_encoding NOT NULL,
  number_field_id      bigint REFERENCES ec.number_field(id),
  icv1_field           text UNIQUE,                -- f2m-<m>-<modhash8> | fp-<p> | fpk-<p>-<k>-<modhash8>
  label                text,                       -- human label, e.g. GF(2^163)
  irreducibility_status ec.fact_status NOT NULL DEFAULT 'unknown',
  primality_status     ec.fact_status NOT NULL DEFAULT 'unknown', -- of the characteristic
  provenance_id        bigint REFERENCES ec.provenance(id),
  created_at           timestamptz NOT NULL DEFAULT now(),
  CHECK (kind NOT IN ('prime','binary','prime_power') OR (characteristic >= 2 AND degree IS NOT NULL AND order_q IS NOT NULL)),
  CHECK (kind <> 'prime' OR degree = 1),
  CHECK (kind <> 'binary' OR characteristic = 2),
  CHECK (kind <> 'prime_power' OR (characteristic > 2 AND degree >= 2)),
  CHECK (kind NOT IN ('rational','number_field','real','complex','p_adic') OR characteristic = 0),
  CHECK (order_q IS NULL OR order_q = power(characteristic::numeric, degree)),
  CHECK (kind <> 'number_field' OR number_field_id IS NOT NULL)
);
COMMENT ON TABLE ec.field IS 'Exact field representations (characteristic, degree, modulus or basis, element encoding).';
COMMENT ON COLUMN ec.field.icv1_field IS 'Field part of the ICV1 identity (docs/curves/ICV1.md).';
CREATE INDEX field_kind_idx ON ec.field (kind, characteristic, degree);
CREATE INDEX field_q_idx ON ec.field (order_q);

-- One ICV1 field may be serialized by several producers as EC1 field
-- records (different element_encoding prose, for instance). Each record keeps
-- its exact canonical bytes so its field_sha256 stays checkable.
CREATE TABLE ec.field_ec1_record (
  id             bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  field_id       bigint NOT NULL REFERENCES ec.field(id),
  field_sha256   ec.sha256_hex NOT NULL UNIQUE,
  canonical_json text NOT NULL,     -- sorted-key compact UTF-8 JSON, the hash preimage
  record         jsonb NOT NULL,
  CHECK (ec.sha256_text(canonical_json) = field_sha256)
);
COMMENT ON TABLE ec.field_ec1_record IS 'EC1 field records (docs/curve-identities.md) with their exact hash preimages; the CHECK re-verifies every digest.';

CREATE TABLE ec.field_embedding (
  id               bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  subfield_id      bigint NOT NULL REFERENCES ec.field(id),
  superfield_id    bigint NOT NULL REFERENCES ec.field(id),
  relative_degree  integer NOT NULL CHECK (relative_degree >= 1),
  generator_image  ec.fe,           -- image of the subfield generator t, in the superfield encoding
  is_frobenius_twisted integer,     -- k when the embedding is composed with Frob^k
  status           ec.fact_status NOT NULL DEFAULT 'unknown',
  provenance_id    bigint REFERENCES ec.provenance(id),
  CHECK (subfield_id <> superfield_id),
  UNIQUE NULLS NOT DISTINCT (subfield_id, superfield_id, generator_image)
);
COMMENT ON TABLE ec.field_embedding IS
  'Explicit field embeddings K -> L (image of the generator). Subfield descent, base change and Weil restriction refer to one; without it "E over GF(2^k) inside GF(2^n)" is ambiguous.';
