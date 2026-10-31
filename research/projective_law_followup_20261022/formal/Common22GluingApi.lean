import Common22PrincipalOpenSpec
import Mathlib.AlgebraicGeometry.Gluing

/-! Pinned API inventory for the next scheme-gluing gate. These checks do not
construct a GlueData object or identify a projective completion. -/

#check AlgebraicGeometry.Scheme.GlueData
#check AlgebraicGeometry.Scheme.GlueData.glued
#check AlgebraicGeometry.Scheme.GlueData.ι
#check AlgebraicGeometry.Scheme.GlueData.ι_isOpenImmersion
#check AlgebraicGeometry.Scheme.GlueData.vPullbackConeIsLimit
#check CategoryTheory.GlueData.t_fac
#check CategoryTheory.GlueData.cocycle

#check Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding
#check Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding
#check Common22PrincipalOpenSpec.principalOpenSpecIso

#print axioms Common22PrincipalOpenSpec.principalOpenSpecIso
