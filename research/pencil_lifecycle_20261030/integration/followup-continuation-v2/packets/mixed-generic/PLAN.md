# Frozen generic mixed-degree field construction

Sources: retained gate add12_weighted_generic lines232-292 (SHA5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0), sealed mixed cancellation packet manifest83302b9abece91d047ce715d250ddbe34bbfbeaa951cad6d4353b2b0851e74e3 and its source/literature plan; Ozbay Gurler-Hisil §§3.3,4.2 weighted generic formula (primary read previous task). No new formula novelty claimed.

Target: exact generic weighted1+2 output normalized validity and nonzero output weights from original degree1/degree2 valid inputs, nonzero four weights, rawK!=0 and common monic quintic f=X5+a3X3+a2X2+a1X+a0. No output coefficient identities or validity assumed. Field-level only in this packet, no new UInt64 bridge/counters/class-sum.

Construction: affine k=u(x)!=0; m=(y-v(x))/k; h=v+m*u. Derive (X-x)*u divides f-h² using original validity plus Bezout resultant k. Explicit q'=x-q-m² and r'=a3+q²-r-m*(2s+m*q)+x*q'. Show f-h²=(X-x)*u*(X²+q'X+r') by vanishing degree<=2 residual divisible by degree3 support. Set v'=-h mod u'; literal s'=m*q'-(s+m*q), t'=m*r'-(t+m*r). Then u' divides f-v'². Weighted source decode must derive these normalized identities from actual numerator trees and rawK scale, not impose them.

Freeze controls before control compilation: rational nonunit disjoint case with f=X5+1, point(0,1), u=X²-1,v=1 plus validity? This candidate requires checking input validity before use: f-v²=X5 not divisibleX²-1, so do NOT use as valid fixture. Select an explicit fixture algebraically by constructing f=h²+((X-x)*u)*u' with monic quintic/noX4, and verify original input divisibilities. A full simple choice: x=0,y=1,q=0,r=1,s=0,t=0, k1,m1; q'=-1; choose a3=0 gives r'=-1. Then f=h²+X*(X²+1)*(X²-X-1)=X5-X3+2X²-X+1 (a3=-1, not0), so recompute coefficients exactly before freezing final fixture. Retain this arithmetic design correction, no trial execution.

Bounds: each Leancompile120s pinned existing dependencies, no installs/downloads/root edits/scientific runs. Preserve attempts. Separate readback/PDF/SVG/JSON and independent review before integration.
