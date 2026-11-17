# Exact recovered-curve Kummer identification

Producer assertions [pcc:unverified]. Freeze0ef439d7d09 pushed before
single invocation, all hashes match, exit0. Conicmatrix invertible;
alpha25,lambda502. Derived matrix from old image coordinates to candidate
standard Kummer coordinates:

    534   0  24 485
    120 474   0 551
    432 280 103 151
    559 588 239 358

Matrix rank4. Full exact polynomial identity overF601:

    K_standard(GL*y) = 368 * F_image(y).

Standard quartic uses all coefficients of recovered curve
y²=495x⁵+539x⁴+47x³+378x²+127x+209, via published generalKummer forms.
No sampled fitting or point-count identification is used for this identity.
Four transformed quintic vectors retained in CONTROL.txt. Together with
the earlier exact composition identity, the map lands on this explicit
candidate curve's standard Kummer model as a polynomial identity.

Independent verification, globalbasepoint/degree25/Jacobianlift proof,
originaltarget recovery and fullcost remain separate. Next implement
Mumford lift and check map compatibility with the original rational
subgroup, preserving zero/degree1/repeated-support exceptional cases.
