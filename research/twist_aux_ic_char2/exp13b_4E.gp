\\ EXP13b: is there concentration beyond E/2E? With a = 0 the D, P1, P2 points lie in 2E; the 2-part of E(K) is cyclic,
\\ so P in 2E is in 4E iff (#E/4)P = O (needs v2(#E) >= 2). Count per line type; R counts only its 2E points.
read("exp13_char2.gp");
fourE(d, ncurves, seed) = {
  setrand(seed); my(N=4*d, Q=2^d, t=ffgen(2^N,'t), h=ffprimroot(t)^((2^N-1)/(Q-1)), FQ=vector(Q-1,i,h^i), T=(z->sum(i=0,3,z^(Q^i))), done=0);
  while(done < ncurves, my(b=random(t), E, NE, sb, z, u, D=0, P1=0, P2=0); if(b==0, next);
    E=ellinit([1,0,0,0,b],t); NE=ellcard(E); if(valuation(NE,2)<2, next); sb=sqrt(b);
    for(k=1,20000, z=random(t); u=z^Q+z; if(u==0, next);
      if(!D && T(sb/u)==0 && (sb/u^2)^Q != sb/u^2, D=u); if(!P1 && T(sb/u)!=0, P1=u));
    if(!D, next); done++;
    for(k=1,20000, u=random(t); if(u!=0 && T(u)!=0 && T(sb/u)==0, P2=u; break));
    my(s=Str("d=",d," curve ",done," v2=",valuation(NE,2),":"));
    foreach([["D",D],["P1",P1],["P2",P2],["R",random(t)]], ln, my(in2=0, in4=0);
      for(i=1,Q-1, my(x=ln[2]*FQ[i], Y=ellordinate(E,x)); if(#Y==0, next); my(P=[x,Y[1]]);
        if(ellmul(E,P,NE/2)==[0], in2++; if(ellmul(E,P,NE/4)==[0], in4++)));
      s=Str(s, "  ", ln[1], " in4E/in2E=", in4, "/", in2));
    print(s)); }
fourE(7, 6, 7001); fourE(8, 6, 8001);
