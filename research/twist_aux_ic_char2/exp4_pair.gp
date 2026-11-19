\\ EXP4 (counterexample test to the naive Prop 3): n=4. Roots of f_L = {alpha, alpha^q, gamma}.
\\ arm A: alpha of degree 4 (alpha^{q^2} != alpha), gamma in F_q  -> norm is square, NO descent predicted.
\\ arm B: pair {alpha, alpha^{q^2}} , gamma in F_q                -> descent to F_{q^2}.
read("common.gp");
lproj(E, N, l, x) = my(P=ellordinate(E,x)); if(#P==0, -1, ellmul(E,[x,P[1]], N/l)==[0]);
bigl(N) = my(f=factor(N)[,1]); f[#f];
arm(q, trials, seed, B) = {
  setrand(seed); my(t=ffgen(q^4,'t), one=t^0, tot=[0,0,0,0,0,0]);
  while(tot[1] < trials,
    my(al=random(t), ga=random(q)*one, s1=0, s0=random(t), e, c, E, N, l, j);
    if(al^(q^2)==al, next);
    while(s1==0 || !issquare(s1), s1=random(t));
    e = if(B, [al, al^(q^2), ga], [al, al^q, ga]); e = vector(3,i, s0 + s1*e[i]);
    \\ f = (x-e1)(x-e2)(x-e3) = x^3 + c1 x^2 + c2 x + c3
    c = [-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]];
    E = ellinit([0,c[1],0,c[2],c[3]], t); N=ellcard(E); l=bigl(N); if(l<100, next);
    j = E.j;
    my(lift=0, dead=0, good=0);
    for(w=0,q-1, my(x=s0+s1*w*one); if(fval(c,x)==0, next); good++; my(r=lproj(E,N,l,x)); if(r==-1,next); lift++; if(r,dead++));
    tot += [1, good, lift, dead, desc(c,s0,s1,q,2), (j^(q^2)==j)];
    printf("q=%d arm=%s #E=%d l=%d  desc_to_Fq2=%d  j_in_Fq2=%d  lifts %d/%d dead %d\n", q, if(B,"B(desc)","A(pair)"), N, l, desc(c,s0,s1,q,2), j^(q^2)==j, lift, good, dead));
  printf("TOTAL q=%d arm %s: curves %d, lifts %d/%d, dead %d, descending %d, j in F_q^2 %d\n", q, if(B,"B","A"), tot[1], tot[3], tot[2], tot[4], tot[5], tot[6]);
}
arm(11,6,1,0); arm(13,6,2,0); arm(17,5,3,0); arm(11,4,4,1); arm(13,4,5,1);
