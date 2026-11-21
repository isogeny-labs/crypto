\\ EXP2: usefulness of all-lift lines.  "dead" = line point whose projection onto the
\\ largest-prime-order component (order l) is O; such a point is a zero column for the l-DLP.
\\ Odd char, n=4, descent case: PREDICTION  dead(w)  <=>  side(w) != side(l),
\\   side(w) = [f_L(w) is a square in K'=F_{q^2}] , side(l) = [l | #E_L(K')].
\\ Char 2, N=15 (Q=32,n=3) twisted line of the lift-dense study: no dead points predicted.
read("common.gp");
lproj(E, N, l, x) = my(P=ellordinate(E,x)); if(#P==0, -1, ellmul(E,[x,P[1]], N/l)==[0]);
bigl(N) = my(f=factor(N)[,1]); f[#f];
oddn4(q, trials, seed) = {
  setrand(seed);
  my(t=ffgen(q^4,'t), one=t^0, g=ffprimroot(t)^((q^4-1)/(q^2-1)), tot=[0,0,0,0,0]);
  for(tr=1, trials,
    my(a=0, b=0, s1=0, s0, c, E, N, l, aK, NL, sideL);
    while(a^q==a || 4*a^3+27*b^2==0, a=g^random(q^2-1); b=g^random(q^2-1));
    while(s1==0 || !issquare(s1), s1=random(t)); s0=random(t);
    c=[-3*s0, 3*s0^2 + a*s1^2, -s0^3 - a*s1^2*s0 + b*s1^3];
    E=ellinit([0,c[1],0,c[2],c[3]], t); N=ellcard(E); l=bigl(N);
    if(l < 50, tr--; next);
    \\ #E_L(K') from #E(K) = (Q'+1)^2 - a'^2 with a' = trace over K': N = #EL(K') * #ELtw(K')
    my(Qp=q^2, ap, nL);
    \\ decide which factor l divides: count points of E_L over K' by brute enumeration of x in K'
    my(cnt=1); for(i=0, Qp-2, my(x=g^i, z=x^3+a*x+b); cnt += if(z==0,1, if(z^((Qp-1)/2)==1, 2, 0))); my(x=0*one, z=b); cnt += if(z==0,1,if(z^((Qp-1)/2)==1,2,0));
    sideL = (cnt % l == 0);
    my(lift=0, dead=0, mis=0);
    for(w=0, q-1, my(x=s0 + s1*w*one, fx=fval(c,x)); if(fx==0, next);
      my(r=lproj(E,N,l,x)); if(r==-1, next); lift++; if(r, dead++);
      my(fl = fx/s1^3, sw = (fl^((Qp-1)/2)==1)); if(r != (sw != sideL), mis++));
    printf("odd q=%d n=4 #%d: #E(K)=%d = #EL(K')=%d * %d, l=%d (in E_L(K'): %d) | descends=%d | line lifts %d/%d, dead %d, prediction mismatches %d\n",
       q, tr, N, cnt, N/cnt, l, sideL, desc(c,s0,s1,q,2), lift, q - #select(w->fval(c,s0+s1*w*one)==0, [0..q-1]), dead, mis);
    tot += [1, lift, dead, mis, 0]);
  printf("  TOTAL q=%d: %d curves, %d lifting line points, %d dead (%.3f), %d prediction mismatches\n", q, tot[1], tot[2], tot[3], tot[3]/tot[2]*1., tot[4]);
}
oddn4ctl(q, trials, seed) = {
  setrand(seed); my(t=ffgen(q^4,'t), one=t^0, L=0, D=0);
  for(tr=1,trials, my(c=[0,random(t),random(t)], E=ellinit([0,0,0,c[2],c[3]],t), N=ellcard(E), l=bigl(N));
    if(l<50, tr--; next);
    for(w=0,q-1, my(r=lproj(E,N,l,w*one)); if(r==-1,next); L++; if(r,D++)));
  printf("  CONTROL generic n=4 q=%d, standard line x in F_q: %d curves, %d lifting points (density %.3f), %d dead\n", q, trials, L, L/(trials*q)*1., D);
}
bin(trials, seed) = {
  setrand(seed);
  my(u=ffgen(2^15,'u), one=u^0, h=ffprimroot(u)^((2^15-1)/31), FQ=concat([0*one], vector(31,i,h^i)), T=(z->z + z^32 + z^1024), done=0, nob=0, TL=0,TD=0,SL=0,SD=0);
  while(done < trials,
    my(b=random(u), E, N, l, c, v=0, found=0);
    if(b==0, next);
    E=ellinit([1,0,0,0,b],u); N=ellcard(E); l=bigl(N); if(l<1000, next);
    c=b^(2^14);
    for(k=1, 4000, my(z=random(u)); v=z^32+z; if(v!=0 && T(c/v)==0, found=1; break));
    if(!found, nob++; next);
    done++;
    my(tl=0,td=0,sl=0,sd=0);
    for(i=2,32, my(r=lproj(E,N,l,v*FQ[i])); if(r!=-1, tl++; if(r,td++));
                my(r2=lproj(E,N,l,FQ[i])); if(r2!=-1, sl++; if(r2,sd++)));
    TL+=tl; TD+=td; SL+=sl; SD+=sd;
    printf("char2 N=15 #%d: #E=%d l=%d | twisted line: lifts %d/31 dead %d | standard F_32: lifts %d/31 dead %d\n", done, N, l, tl, td, sl, sd));
  printf("  TOTAL char2: twisted lifts %d dead %d | standard lifts %d dead %d | curves skipped for lack of valid u: %d\n", TL, TD, SL, SD, nob);
}
