# Frozen independent trace-2 quotient model replay

16 November 2026. The invariant coordinates for
`C2/F5: y^11=x(x-1)/(x-2)` yield the candidate genus-five
hyperelliptic model

`H2: W²=(v-2)*(D11(v,1)-1)`

where `D11(v,1)=v^11-11v^9+44v^7-77v^5+55v^3-11v`.
The rational map is

`v=y+1/y`, `u=x+2/x`,
`W=(2u-D11(v,1)-2)/P5(v)`, with
`P5=v^5+v^4+v^3+2v^2+3v+1`.

PARI/GP will check the factorization
`D11-2=(v-2)*P5²` modulo five, that `H2` is squarefree, and compute
`hyperellcharpoly(H2)`. The factorization's integer correction is
`(D11-2-(v-2)*P5²)/5 = -2v^9+9v^7+2v^6-13v^5+2v^4+14v^3+4v^2`.
The earlier `IDC_Q2` certifies the other rational identity needed to
derive the quotient equation. A second evaluated certificate for this
factorization will be issued separately. The GP result is an independent
zeta replay of this exact hyperelliptic model. It does not construct
an elliptic-to-Jacobian isogeny.

Freeze the GP script and this protocol before replay. Retain stdout,
stderr, version, and source hash. Any correction is an amendment.
