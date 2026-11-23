# Recovered quotient candidate and quadratic twist

Producer assertions [pcc:unverified]. Freeze7eb6ff0b323 pushed before
single invocation, all hashes match, exit0. Both control and image
quartics have nonsingular tangent conics at the distinguished node and
exactlyone rational marked branchpoint. Quadratic parameterizations satisfy
their conic equations identically; restricted cubics give squarefree
quintics, one branchpoint at parameter infinity.

Sourcecontrol recovered y²=222x⁵+11, qcount681,q²count360423.
Image recovery before twist gave coefficients [545,104,54,436,77,500].
Multiplying by nonsquare7 selected the source-order twist:

    C_candidate: y²=495x⁵+539x⁴+47x³+378x²+127x+209 over F601.

Selected candidate counts:681overF601 and360423overF601²; reconstruct
a1=79,a2=2731,Jacobian order411491. This matches the original source
characteristic polynomial. Raw point counts enumerate both fields using
extension r²=7 and norm quadratic character. Counts are producer checks,
not independent verification or sufficient proof of the actual map lift.

Recorded image conic branchpoint [1,154,322], tangent parameter direction
[0,380,576], complement[0,1,0]. These determine the rational conic
parameterization and are available for coordinate-map reconstruction.
No original target is substituted by this new candidate curve.

Next exact coordinate identity: transform the conic parameterization to
the standard (k1,k2,k3)=(v²,2uv,u²) tangent conic. Solve k4=lambda*t+L(k1,k2,k3)
by comparing cubic polar terms, then check the full quartic identity against
the general Kummer quartic of the recovered polynomial. Only after that
qualify the Jacobian lift/originaltarget transport and charge full costs.
No speed measurement or genus2+10x claim from this recovery gate.
