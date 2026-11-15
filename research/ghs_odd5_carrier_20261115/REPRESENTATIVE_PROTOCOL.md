# Frozen independent full-polynomial representative check

15 November 2026 UTC, after the exhaustive C++ and Sage point-count agreement and before a public Magma call. Select the first positive ordinary trace in the eight simple carrier classes, t=10. The frozen first source equation is `E/F243: y²=x³+x²+(a²+a)` with `a^5+2a+1=0`, order 234. The first F3 genus-five target mask is 2872, namely `Y:y²=x^11+x^7+2x^5+2x^4+x^3+x²+1`; five exact point counts predict `L_Y(T)=1−10T^5+3^5T^10` and `#J_Y(F3)=234`. [pcc:unverified]

Commit the exact Magma source and immutable public runner before calling it. Compute and assert E order, Y genus, entire `EulerFactor(Jacobian(Y))`, and Jacobian order. Preserve source hash, complete response, CPU and client wall time. A matching polynomial certifies the isogeny class by Tate but does not evaluate an isogeny on points; do not call it a subgroup map or attack. Stop and retain any mismatch or API failure. [pcc:unverified]
