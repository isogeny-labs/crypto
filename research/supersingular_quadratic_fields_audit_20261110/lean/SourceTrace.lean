import QuadraticModels

namespace FiniteFieldSourceTraceAudit
open SupersingularTraceClassesAudit

/-- Exact FST2010 finite-field source definition, with q the actual field
cardinality and trace q+1 minus the actual rational point-group cardinality.
The geometric equivalence with endomorphism or p-rank definitions is separate. -/
def SourceSupersingular {K : Type*} [Field K] [Fintype K]
    (E : WeierstrassCurve.Affine K) : Prop :=
  Int.gcd (fieldTrace E) (Fintype.card K)≠1

end FiniteFieldSourceTraceAudit
