# Fixed exact development certificates, 2026-11-04

No search, timing, fitted exponent, EKS job or ECDLP execution. Freeze before
the first compile/run:

1. L=4,8,16 and k=1,2,4,8. Form actual rational puncture polynomials and
   multiply even-delta entries by ((1-T)/(1-delta))^k. Check all source node
   values, all coefficients of all scaled charts for integrality, and odd-node
   crossing content against the original sharp content plus k.
2. a=1,...,8 and g=1,2,4. Construct the deterministic equality schedule
   m_i=g+sum_(j>i)(j-i)m_j, from high level to low. Require total mass g*F_(2a).
   Directly evaluate the rational diagonal anchor product at t=-2^i and
   independently compare its valuation to g at each active level. Also
   check the uniform schedule m_i=1 has gain 1-sum_(j>i)(j-i), including losses.
3. Reject the deliberately insufficient schedule formed by decreasing the
   lowest-level multiplicity of each equality schedule by one.

Retain compile/build failures and exact commands. Local units are covered
by the isolab-eks skill's development exception. Polynomial/valuation
certificates do not establish matrix rank compression or recursive runtime.
