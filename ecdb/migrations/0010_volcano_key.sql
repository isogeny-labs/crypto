-- 0010: a natural key for isogeny_volcano, so a derivation can insert components
-- and their vertices in bulk and re-run idempotently.
ALTER TABLE ec.isogeny_volcano ADD COLUMN IF NOT EXISTS component_key text UNIQUE;
COMMENT ON COLUMN ec.isogeny_volcano.component_key IS
  'Producer key of the component, e.g. ell:3:min_curve:<least curve id> for volcanoes derived from stored isogenies.';
