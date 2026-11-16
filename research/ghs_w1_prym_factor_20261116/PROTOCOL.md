# Frozen W1 genus-fourteen Frobenius-factor check

16 November 2026. Source: the recorded order-23 `WeilDescent` map at `c=z^94` in `../ghs_n7_explicit_map_20261115/`. Before a new public Magma call, test the specific prediction

`L_C(T)=(1+9T^7+128T^14)(1-9T^7+128T^14)=1+175T^14+16384T^28`.

It predicts `#J_C(F2)=16560`, matching the earlier class-group computation, but that one number alone does not determine the polynomial. The corresponding seven-dimensional factors have orders 138 (W1 source restriction and `Y_-`) and 120 (W0 twist restriction and `Y_+`). For `1<=m<=13`, the predicted `#C(F_(2^m))` is `2^m+1`; at `m=14` it is `18835`.

Run `check_factor.magma` against Magma's `LPolynomial(C)` with a 120-second HTTPS timeout. Preserve the complete response, source hash, and any failure or timeout. Assert the predicted polynomial only after printing the observed polynomial. If the computation fails or times out, retain a conditional theorem-level factorization from Hess Theorem 15 and seek an independent finite count; do not promote the class-number match to a full factorization.

Even a confirmed split does not give an evaluated map from the W1 source to the hyperelliptic `Y_-`: it identifies an isogeny factor. Hess Section 8.3 says the natural intermediate-field norm kills the source image. Any useful projection must be a distinct correspondence or an explicit isogeny from the Prym to `J(Y_-)`.

Primary source: [Hess 2004, Theorem 15 and Section 8.3](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf). Computational API: [Magma LPolynomial documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1471).
