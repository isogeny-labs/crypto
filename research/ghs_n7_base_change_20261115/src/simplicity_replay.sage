R.<X> = PolynomialRing(QQ)

for sign in [-9, 9]:
    K.<alpha> = NumberField(X^2 - sign*X + 128)
    assert K.discriminant() == -431
    assert alpha.norm() == 128
    # Every integer of K has norm (u^2 + 431*v^2)/4.
    assert all(u*u + 431*v*v != 8 for u in range(-3, 4) for v in range(-1, 2))
    print(f"sign={sign} discriminant={K.discriminant()} norm_alpha={alpha.norm()} norm_two_absent=true")

    prev2, prev1 = 2, sign
    for s in range(1, 11):
        if s == 1:
            trace = prev1
        else:
            trace = sign*prev1 - 128*prev2
            prev2, prev1 = prev1, trace
        restriction_poly = X^14 - trace*X^7 + 2^(7*s)
        if s % 7 == 0:
            assert s == 7
            base_change_poly = (X^2 - sign*X + 128)^7
            assert restriction_poly != base_change_poly
            assert not restriction_poly.is_irreducible()
        else:
            base_change_poly = restriction_poly
        factors = [(f.degree(), multiplicity) for f, multiplicity in base_change_poly.factor()]
        assert base_change_poly.is_irreducible() == (s % 7 != 0)
        print(f"sign={sign} s={s} trace={trace} base_change_factors={factors} restriction_factors={[(f.degree(), e) for f, e in restriction_poly.factor()]}")
