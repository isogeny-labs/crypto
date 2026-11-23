# Basis parser amendment

The first frozen validator (`validation_protocol.json` SHA-256 `c56e497c4d4abd8687f3dbb64664dd0b737832694c93631505a8253e7c985298`) failed while parsing the query-4 control basis: msolve returned 33 polynomials rather than the 30-polynomial triangular form observed on the earlier residuals. The original parser asserted a 30-polynomial basis.

The amended parser reduces the Boolean-linear basis equations, enumerates all assignments of their remaining free variables (capped at eight), and tests every reported basis polynomial. It therefore handles both the prior triangular form and the 33-polynomial form without assuming a particular root count. The independent field S3 enumeration and full group-point replay remain separate checks. The validator source hash is updated before rerunning; solver inputs, limits, outputs, and run order are unchanged.
