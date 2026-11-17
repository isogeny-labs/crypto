-- 0013: the characteristic polynomial of Frobenius on E[ell], per isogeny class.
--
-- X^2 - t X + q mod ell is fixed by the Weil polynomial, so it belongs to
-- ec.frobenius_ell (one row per class and prime) beside the volcano depth and
-- the splitting of ell in Q(pi). Additive only: two nullable columns and a
-- range check; NULL means not determined (rows written before this migration,
-- or by an importer that does not compute it).

ALTER TABLE ec.frobenius_ell
  ADD COLUMN frobenius_trace_mod_ell integer,
  ADD COLUMN frobenius_det_mod_ell integer;

ALTER TABLE ec.frobenius_ell ADD CONSTRAINT frobenius_ell_charpoly_range CHECK (
  (frobenius_trace_mod_ell IS NULL OR (frobenius_trace_mod_ell >= 0 AND frobenius_trace_mod_ell < ell))
  AND (frobenius_det_mod_ell IS NULL OR (frobenius_det_mod_ell >= 0 AND frobenius_det_mod_ell < ell)));

COMMENT ON COLUMN ec.frobenius_ell.frobenius_trace_mod_ell IS
  't mod ell: Frobenius on E[ell] has characteristic polynomial X^2 - (t mod ell) X + (q mod ell).';
COMMENT ON COLUMN ec.frobenius_ell.frobenius_det_mod_ell IS
  'q mod ell = det of Frobenius on E[ell] (Weil pairing).';
