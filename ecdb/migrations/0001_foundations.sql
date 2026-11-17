-- ecdb 0001: foundations.
--
-- Everything lives in schema `ec`. Conventions used by every later migration:
--
--   * Exact integers of any size are `ec.zint` (numeric with no fractional
--     part). Group orders of 571-bit curves, Weil-polynomial coefficients and
--     discriminants never pass through bigint or floating point.
--   * A field element is `ec.fe`: text in the canonical encoding its field
--     declares (`ec.field.element_encoding`): decimal residue for GF(p),
--     lower-case `0x` bitset for polynomial-basis GF(2^m), comma-joined
--     decimal coefficients `e_0,...,e_{k-1}` for GF(p^k) (the ICV1
--     j-invariant convention), comma-joined rationals for number fields.
--   * A polynomial over a field is a jsonb array of `ec.fe` strings in
--     ascending degree (the convention of the isogeny-walk kernel polynomials
--     and of docs/curves/covers.json).
--   * Every derived value carries an `ec.fact_status` (the vocabulary of
--     docs/curves/traits.json, extended by heuristic/conjectural/refuted) and
--     a provenance row. Unknown is NULL with status `unknown`; it is never 0,
--     'none' or false.

CREATE SCHEMA IF NOT EXISTS ec;
COMMENT ON SCHEMA ec IS
  'Elliptic curves, curves of higher genus, abelian varieties, their invariants and the maps between them (ecdb).';

CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS btree_gin;

CREATE TABLE IF NOT EXISTS ec.schema_migrations (
  version     integer PRIMARY KEY,
  name        text NOT NULL,
  sha256      char(64) NOT NULL,
  applied_at  timestamptz NOT NULL DEFAULT now()
);
COMMENT ON TABLE ec.schema_migrations IS
  'Applied migrations. The runner refuses to start when an applied file''s sha256 changed: migrations are immutable once applied.';

-- ---------------------------------------------------------------- domains

CREATE DOMAIN ec.zint AS numeric CHECK (VALUE = trunc(VALUE));
COMMENT ON DOMAIN ec.zint IS 'Exact integer of unbounded size.';
CREATE DOMAIN ec.nat AS numeric CHECK (VALUE = trunc(VALUE) AND VALUE >= 0);
COMMENT ON DOMAIN ec.nat IS 'Exact non-negative integer of unbounded size.';
CREATE DOMAIN ec.pos AS numeric CHECK (VALUE = trunc(VALUE) AND VALUE >= 1);
COMMENT ON DOMAIN ec.pos IS 'Exact positive integer of unbounded size.';
CREATE DOMAIN ec.fe AS text CHECK (length(VALUE) > 0 AND VALUE !~ '\s');
COMMENT ON DOMAIN ec.fe IS 'Field element in the canonical encoding of its field (see ec.field.element_encoding).';
CREATE DOMAIN ec.sha256_hex AS char(64) CHECK (VALUE ~ '^[0-9a-f]{64}$');

-- ---------------------------------------------------------------- enums

CREATE TYPE ec.fact_status AS ENUM (
  'proved',          -- established by proof or deterministic certified computation
  'probable',        -- e.g. rests on a probable-prime test
  'bounded',         -- only a bound is established (see the *_at_most / lower_bound columns)
  'heuristic',       -- follows from a heuristic argument or unproved standard assumption
  'conjectural',     -- conditional on a named conjecture (BSD, GRH, ...)
  'refuted',         -- a claim that was checked and is false; kept as a record
  'unknown',         -- nobody has determined it
  'not_evaluated',   -- the producing tool does not support this case
  'not_applicable'   -- the notion does not apply to this object
);
COMMENT ON TYPE ec.fact_status IS
  'How well a value is established. Vocabulary of docs/curves/traits.json plus heuristic/conjectural/refuted.';

CREATE TYPE ec.field_kind AS ENUM (
  'rational',        -- Q
  'number_field',    -- finite extension of Q
  'prime',           -- GF(p)
  'binary',          -- GF(2^m), m >= 1
  'prime_power',     -- GF(p^k), p odd, k >= 2
  'p_adic',          -- Q_p or a finite extension
  'real',
  'complex',
  'function_field'   -- F_q(t) and finite extensions
);

CREATE TYPE ec.field_representation AS ENUM (
  'prime',                 -- residues mod p
  'polynomial',            -- F_p[t]/(f)
  'normal',                -- normal basis
  'optimal_normal_type1',
  'optimal_normal_type2',
  'gaussian_normal',       -- X9.62 Gaussian normal basis of type T
  'tower',                 -- relative extension of base_field_id
  'power_basis',           -- number field Q[t]/(f)
  'abstract'               -- no element encoding (e.g. R, C, Q_p)
);

CREATE TYPE ec.fe_encoding AS ENUM (
  'decimal',           -- GF(p): canonical residue in [0, p)
  'hex_bitset',        -- GF(2^m) polynomial basis: 0x..., bit i = coefficient of t^i
  'decimal_vector',    -- GF(p^k): e_0,...,e_{k-1} in basis 1, t, ..., t^{k-1}
  'rational_vector',   -- number field: c_0,...,c_{n-1} rationals a/b in the power basis
  'normal_bitset',     -- GF(2^m) normal basis bitset
  'opaque'             -- declared by the source; not interpreted by ecdb
);

CREATE TYPE ec.provenance_class AS ENUM ('recalled', 'retrieved', 'kb', 'internal');
COMMENT ON TYPE ec.provenance_class IS
  'Citation provenance of crypto-autoresearcher AGENTS.md rule 9: a recalled reference is a pointer, never support.';

CREATE TYPE ec.source_kind AS ENUM (
  'repository_file', 'standard', 'paper', 'database', 'computation', 'manual', 'agent'
);

-- ---------------------------------------------------------------- provenance

CREATE TABLE ec.source (
  id               bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  kind             ec.source_kind NOT NULL,
  title            text NOT NULL,
  uri              text,
  repository       text,           -- e.g. isogeny-labs/crypto
  path             text,           -- repository-relative path
  git_commit       text,
  sha256           ec.sha256_hex,
  blake3           text,
  citation         jsonb,          -- bibliographic record for papers and standards
  provenance_class ec.provenance_class NOT NULL DEFAULT 'internal',
  retrieved_at     timestamptz,
  created_at       timestamptz NOT NULL DEFAULT now(),
  UNIQUE NULLS NOT DISTINCT (kind, repository, path, sha256, uri)
);
COMMENT ON TABLE ec.source IS 'Where a record came from: a pinned repository file, a standard, a paper, an external database, a computation.';

CREATE TABLE ec.artifact (
  id            bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  sha256        ec.sha256_hex NOT NULL UNIQUE,
  blake3        text,
  size_bytes    bigint CHECK (size_bytes >= 0),
  media_type    text,
  storage_uri   text,             -- s3://bucket/key for large objects
  repo_path     text,
  description   text,
  created_at    timestamptz NOT NULL DEFAULT now()
);
COMMENT ON TABLE ec.artifact IS
  'Content-addressed blob (certificate, large polynomial, map formulas) stored outside the row; the row keeps its hash.';

CREATE TABLE ec.provenance (
  id            bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  source_id     bigint REFERENCES ec.source(id),
  artifact_id   bigint REFERENCES ec.artifact(id),
  method        text,             -- how the value was obtained (algorithm / check)
  tool          text,             -- producing program
  tool_version  text,
  command       text,
  computed_at   timestamptz,
  verified_by   text[] NOT NULL DEFAULT '{}',
  parameters    jsonb,            -- tool parameters, cost models, budgets
  notes         text,
  created_at    timestamptz NOT NULL DEFAULT now(),
  UNIQUE NULLS NOT DISTINCT (source_id, artifact_id, method, tool, tool_version)
);
COMMENT ON TABLE ec.provenance IS 'One provenance record; fact rows point here.';
CREATE INDEX provenance_source_idx ON ec.provenance (source_id);

-- ---------------------------------------------------------------- helpers

CREATE FUNCTION ec.sha256_text(t text) RETURNS text
  LANGUAGE sql IMMUTABLE STRICT PARALLEL SAFE
  AS $$ SELECT encode(sha256(convert_to(t, 'UTF8')), 'hex') $$;
COMMENT ON FUNCTION ec.sha256_text(text) IS 'Hex SHA-256 of the UTF-8 bytes of t (EC1 / ICV1 identity checks).';

-- floor(log2) through numeric log can round across a power of two, so the
-- estimate is corrected exactly.
CREATE FUNCTION ec.bits(n numeric) RETURNS integer
  LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE AS $$
DECLARE
  a numeric := abs(trunc(n));
  b integer;
BEGIN
  IF a < 1 THEN RETURN 0; END IF;
  b := floor(log(2::numeric, a))::integer + 1;
  -- exact correction: 2^(b-1) <= a < 2^b
  WHILE power(2::numeric, b - 1) > a LOOP b := b - 1; END LOOP;
  WHILE power(2::numeric, b) <= a LOOP b := b + 1; END LOOP;
  RETURN b;
END $$;
COMMENT ON FUNCTION ec.bits(numeric) IS 'Bit length of |n| (0 for 0).';

CREATE FUNCTION ec.isqrt(n numeric) RETURNS numeric
  LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE AS $$
DECLARE
  x numeric;
  y numeric;
BEGIN
  IF n < 0 THEN RAISE EXCEPTION 'isqrt of negative %', n; END IF;
  IF n < 2 THEN RETURN trunc(n); END IF;
  x := trunc(sqrt(n));
  -- Newton correction to the exact floor
  LOOP
    y := trunc((x + trunc(n / x)) / 2);
    EXIT WHEN y >= x;
    x := y;
  END LOOP;
  WHILE x * x > n LOOP x := x - 1; END LOOP;
  WHILE (x + 1) * (x + 1) <= n LOOP x := x + 1; END LOOP;
  RETURN x;
END $$;
COMMENT ON FUNCTION ec.isqrt(numeric) IS 'Exact floor square root of a non-negative integer.';

CREATE FUNCTION ec.touch_updated_at() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN NEW.updated_at := now(); RETURN NEW; END $$;
