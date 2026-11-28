\\ EXP16c: affine char-2 lines c0 + u*F_Q. Exhaustive check of every F_Q-line (through 0 or not) inside the
\\ plane P = sqrt(b)*F_{Q^2} (a 2-dim F_Q-subspace of K = F_{Q^4}), with lifting and class computed from actual
\\ points (ellordinate + (#E/2)P test), not from traces. Prediction (Weil/Hilbert 90 argument, see note in the
\\ exp16c result): on P, Tr(b/x^2) = 0 for every x, so lifting <=> Tr(x) = Tr(a); the unique direction with
\\ T(u) = 0 is u in (sqrt(b)/tau)*F_Q, tau = Tr_{K/F_{Q^2}}(sqrt b); its affine cosets with Tr(c0) = Tr(a)
\\ are lift-dense (all Q points lift, all in 2E): Q/2-1 such lines if Tr(a)=0, Q/2 if Tr(a)=1.
\\ Also: through-0 D lines (T(u)=T(sqrt b/u)=0) have no lifts at all when Tr(a)=1.
cls2(E, NE, P) = ellmul(E, P, NE/2) != [0];
run(d, ncurves, seed, ta) = {
  setrand(seed); my(N=4*d, Q=2^d, t=ffgen(2^N,'t), one=t^0, g=ffprimroot(t), h=g^((2^N-1)/(Q-1)),
    h2=g^((2^N-1)/(Q^2-1)), FQ=concat([0*one], vector(Q-1,i,h^i)),
    T=(z->sum(i=0,3,z^(Q^i))), Tr=(z->lift(trace(z))), done=0);
  \\ F_{Q^2} = F_Q + h2*F_Q (h2 generates F_{Q^2}, degree 2 over F_Q since Q^2-1 does not divide Q-1)
  while(done < ncurves, my(b=random(t), a, E, NE, sb, tau, hist=Map(), maxp=0, nfull=0, fullok=1, ndense0=0);
    if(b==0, next);
    if(ta==0, a=0*one, until(Tr(a)%2==1, a=random(t)));
    sb=sqrt(b); tau=sb+sb^(Q^2); if(tau==0, next); done++;
    E=ellinit([1,a,0,0,b],t); NE=ellcard(E);
    \\ directions of P: u = sb*(h2 + l) (l in F_Q) and u = sb (projective line over F_Q, Q+1 points)
    my(dirs=concat(vector(Q,i,sb*(h2+FQ[i])), [sb]));
    foreach(dirs, u,
      my(Tu=T(u), istarget=(Tu==0));
      \\ cosets of u*F_Q in P: c0 = sb*v with v ranging over a complement; P/uF_Q ~ F_Q: c0 = c*v0, v0 in P \ uF_Q
      my(v0 = if(u==sb, sb*h2, sb));
      for(ci=1,Q, my(c0=FQ[ci]*v0, n=[0,0], cnt);
        for(wi=1,Q, my(x=c0+u*FQ[wi]); if(x==0, next); my(Y=ellordinate(E,x));
          if(#Y, my(P=[x,Y[1]], c=cls2(E,NE,P)); n[c+1]++));
        cnt=vecmax(n);
        my(v); if(mapisdefined(hist,cnt,&v), mapput(hist,cnt,v+1), mapput(hist,cnt,1));
        if(ci>1 && cnt>maxp, maxp=cnt);
        if(ci>1 && cnt==Q, nfull++; if(!istarget || Tr(c0)%2 != Tr(a)%2 || n[1]!=Q, fullok=0));
        if(ci>1 && istarget && Tr(c0)%2==Tr(a)%2 && cnt!=Q, fullok=0)));
    \\ through-0 D lines on this curve (random search), lift count
    my(Dl=0, dl=-1);
    for(k=1,20000, my(z=random(t), uu=z^Q+z); if(uu!=0 && T(sb/uu)==0 && (sb/uu^2)^Q!=sb/uu^2, Dl=uu; break));
    if(Dl, dl=sum(i=2,Q, #ellordinate(E,Dl*FQ[i])>0));
    printf("d=%d Q=%d curve %d Tr(a)=%d #E=%d v2=%d | lines in P: Q(Q+1)=%d, pool histogram %s\n", d, Q, done, Tr(a)%2, NE, valuation(NE,2), Q*(Q+1), Vec(Mat(hist)~));
    printf("     affine max pool %d, #affine lines with pool Q = %d (predicted %d), prediction-consistent=%d | through-0 D line lifts %d/%d\n",
      maxp, nfull, if(Tr(a)%2, Q/2, Q/2-1), fullok, dl, Q-1));
}
run(5, 3, 1601, 0);
run(5, 3, 1602, 1);
run(6, 2, 1603, 0);
run(6, 2, 1604, 1);
run(7, 2, 1605, 0);
run(7, 2, 1606, 1);
run(8, 1, 1607, 0);
run(8, 1, 1608, 1);
