# Next discriminating calculation: avoid the power-order bottleneck

The30-second bnfinit(H,1) attempt returned no result. Do not repeat it unchanged.
The candidate remains p263=(263,u+1), and a generator must be totally positive
and generate this specific maximal-order ideal, not just have norm263.

An explicit-basis route can use F=Q(zeta131+zeta131^-1), degree65, and
K+=F(u), u=alpha*zeta+beta*zeta^-1. Put s=zeta+zeta^-1. Then

    u²+s*u+2*s²-7=0,
    relative discriminant =7*(4-s²).

The basis O_F + u*O_F has discriminant7^65*131^129, matching the known
maximal-order field discriminant, so it is a candidate explicit maximal
integral basis once the written field/basis identities are checked.
Use O_F basis1,s1,...,s64, where sj=zeta^j+zeta^-j.

A direct low-denominator embedding of s into Q(u) follows from the two
Lucas recurrences. Let A0=0,A1=1,A_(j+2)=u*A_(j+1)-2*A_j. Let U be the
source trace-1 recurrence retained in the parent. Then

    s = (2*A131 + u*(U130-A130))/U131  modulo H.

For j>=1, using Dj=pi^j+(2/pi)^j,

    sj = (Uj*(s*Aj+A_(j-1))-U_(j-1)*Dj)/2^(j-1).

Derive and check these identities before feeding a basis to PARI; passing
an incorrect basis as maximal would invalidate class-group output.
The official nfinit documentation accepts [H,Basis,[7,131]]. The bnfinit
entry accepts an initialized number field in its implementation; verify
that contract on a small known field before using it on degree130.
Retain source, exact basis discriminant, field embedding checks and any
failure. This is a proposed computational route, not a completed basis
or a new principal-generator result. [pcc:unverified]

If a provisional generator appears, check its exact principal ideal,
norm, integrality and all real signs independently before promoting the
polarization condition. A nonzero class vector alone is conditional on
PARI's class-group assumptions; use bnfcertify plus narrow conditions for
a negative statement. The curve/Torelli and executable-transfer conditions
remain even if the ideal test succeeds.
