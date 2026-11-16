-- 0015: a relation kind for the Prym complement of a genus-2 cover.
--
-- Additive only. For a genus-2 curve H with an involution whose quotient is
-- the elliptic curve E, Jac(H) is isogenous to E x E', with E' the Prym
-- variety of H -> E (the quotient by the complementary involution). A
-- curve_relation row of kind prym_complement links E (source) to E'
-- (target); its map names H and the explicit maps. It does not transport a
-- discrete logarithm: E and E' are different isogeny classes in general.
-- The decomposition Jac(H) ~ Jac(E) x Jac(E') itself is av_component rows
-- (relation 'isogeny_factor').
--
-- ALTER TYPE ... ADD VALUE runs inside the migration transaction (Postgres
-- 12+); the value is used only after the transaction commits.

ALTER TYPE ec.relation_kind ADD VALUE IF NOT EXISTS 'prym_complement';
