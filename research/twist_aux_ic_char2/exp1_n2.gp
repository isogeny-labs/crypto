\\ EXP1: n=2, exhaustive over all q(q+1) F_q-lines in K=F_{q^2}, several curves per q.
\\ Prediction (theorem, q >= 37): all-lift line  <=>  f_L in F_q[w] and c1 square in K.
\\ Converse also checked: every descending line with c1 square is all-lift; with c1 nonsquare is all-twist.
read("common.gp");
setrand(20261105);
run(q, ncurves, kind) = {
  my(t=ffgen(q^2,'t), one=t^0, Fq, dirs, nall=0, ndesc=0, mism=0, nalltw=0, maxd=0, res=List());
  \\ directions: c1 in {1} U {s + t : s in F_q}  (representatives of K^*/F_q^*)
  dirs = concat([one], vector(q, s, (s-1)*one + t));
  for(cv=1, ncurves,
    my(c, j);
    if(kind=="generic",
       until(j^q != j, c=[0, random(t), random(t)]; if(4*c[2]^3+27*c[3]^2==0, next); j = c[2]^3/(4*c[2]^3+27*c[3]^2)),
       \\ "subtwist": affine image of a curve over F_q, with random c1, c0  (j in F_q)
       my(a=random(q)*one, b=random(q)*one, s1=random(t), s0=random(t));
       until(4*a^3+27*b^2 != 0 && s1 != 0, a=random(q)*one; b=random(q)*one; s1=random(t));
       \\ E: y^2 = s1^3 f0((x-s0)/s1) = x^3 -3 s0 x^2 + (3 s0^2 + a s1^2) x + (-s0^3 - a s1^2 s0 + b s1^3)
       c=[-3*s0, 3*s0^2 + a*s1^2, -s0^3 - a*s1^2*s0 + b*s1^3]);
    my(cAll=0, cDesc=0, cMis=0, cTw=0, cmax=0);
    for(i=1, #dirs, my(c1=dirs[i]);
      for(k=0, q-1, my(c0 = k*t*(i==1) + k*one*(i>1));  \\ coset reps of K / c1 F_q
        my(L = linelift(c, c0, c1, q, one), good = q - L[2], d = L[1]/good, isd = desc(c,c0,c1,q,1));
        if(d > cmax, cmax = d);
        my(all = (L[1]==good), none = (L[1]==0));
        if(all, cAll++); if(none, cTw++);
        if(isd, cDesc++);
        \\ prediction
        my(pred_all = isd && issquare(c1), pred_none = isd && !issquare(c1));
        if(all != pred_all || (none != pred_none), cMis++)));
    nall += cAll; ndesc += cDesc; mism += cMis; nalltw += cTw; maxd = max(maxd, cmax);
    listput(res, [cAll, cTw, cDesc, cMis, cmax*1.]));
  printf("q=%d kind=%s curves=%d lines/curve=%d | all-lift=%d all-twist=%d descending=%d MISMATCH=%d max density=%.4f (Weil cap for nondeg: %.4f)\n",
    q, kind, ncurves, q*(q+1), nall, nalltw, ndesc, mism, maxd, 0.5+ (5*sqrt(q)+6)/(2*(q-6)));
  Vec(res);
}
for(i=1,1, foreach([37,41,43,47], q, run(q, 6, "generic"); run(q, 6, "subtwist")));
\\ below threshold: sporadic non-descending all-lift lines expected to be possible
foreach([7,11,13], q, run(q, 30, "generic"));
