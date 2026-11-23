\\ EXP12e: "class-pinned" lines. With alpha != beta in F_{q^2}, c1 = (e2-e1)/(alpha-beta), c0 = e1 + c1*alpha:
\\ x-e1 = c1(w+alpha), x-e2 = c1(w+beta), and F_{q^2}* is inside the squares of F_{q^4}, so every x-value of the
\\ line has the same 2-descent class; the third root is generic, so the line is half-lift and not case D.
read("exp12a_classes.gp");
pinned(e, q, t) = { my(g=ffprimroot(t)^((q^4-1)/(q^2-1)), a, b, c1);
  until(a!=b, a=random(q)+random(q)*g; b=random(q)+random(q)*g); c1=(e[2]-e[1])/(a-b); [e[1]+c1*a, c1]; }
{ my(q=1009, t=ffgen(q^4,'t), one=t^0, done=0); setrand(100901);
  while(done<3, my(e=vector(3,i,random(t)), line); if(#Set(e)<3, next);
    line=caseP(e,q,t); if(line==0, next); done++;
    my(cf=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]], N=ellcard(ellinit([0,cf[1],0,cf[2],cf[3]],t)));
    for(i=1,4, my(L=pinned(e,q,t), n=lineclasses(e,L[1],L[2],q,one));
      printf("q=%d curve %d pinned line %d: classes %s, total %d, whole-line yield %.4f (P %.4f)\n", q, done, i, n, vecsum(n), 64.*N0(n)/N, predict(e,line[1],line[2],q,one,N)[2]))) }
{ my(q=43, t=ffgen(q^4,'t), one=t^0, done=0, NT=300); setrand(4302);
  while(done<3, my(e=vector(3,i,random(t)), cf, E, N, L, pts, cnt=vector(4), s=0, h=0); if(#Set(e)<3, next); done++;
    cf=[-(e[1]+e[2]+e[3]), e[1]*e[2]+e[1]*e[3]+e[2]*e[3], -e[1]*e[2]*e[3]]; E=ellinit([0,cf[1],0,cf[2],cf[3]],t); N=ellcard(E);
    L=pinned(e,q,t); pts=linepts(E,L[1],L[2],q,one);
    for(w=1,q, if(#pts[w], cnt[cls(e,pts[w][1][1])]++));
    for(r=1,NT, my(R=ellmul(E,random(E),2), c); if(R==[0], next); c=bf2(E,pts,q,R); s+=c; h+=(c>0));
    printf("q=43 curve %d pinned: classes %s | decomps/target in 2E measured %.4f (hit %.3f) predicted %.4f\n", done, cnt, s*1./NT, h*1./NT, 64.*N0(cnt)/N)) }
