\\ e3_curves.gp -- curve selection (PREREGISTRATION section 7). Writes out/curves.csv (odd family) and out/curves_even.csv.
default(parisizemax,2000000000);
genv(n,def)={my(v=getenv(n)); if(type(v)!="t_STR" || v=="", def, v)}
OUTDIR=genv("E3OUT","out");
PL=eval(genv("E3PS","[101,211,401]"));
PE=eval(genv("E3PE","[101,211]"));
nonres(p) = {my(d=2); while(kronecker(d,p)!=-1, d++); d}
\\ the c-th curve of the stream seeded 1000*p+c satisfying the odd-order rules
pick(p,c) = {
  my(d=nonres(p), cnt=0, a, b, E, n, t, np, ET);
  setrand(1000*p+c);
  while(1,
    a=random(p); b=random(p);
    if((4*a^3+27*b^2)%p==0, next);
    E=ellinit([a,b],p); n=ellcard(E);
    if(!isprime(n) || n==p, next);
    t=p+1-n; np=p+1+t;
    if(np==p || np%2==0 || gcd(n,np)!=1, next);
    ET=ellinit([a*d^2%p,b*d^3%p],p);
    if(#ellgroup(ET)!=1, next);
    cnt++;
    if(cnt==c, return([p,c,a,b,n,np,t])));
}
\\ even-order curve: same seed rule with c=0, requires #E even (equivalently a rational 2-torsion point)
pickeven(p) = {
  my(a,b,E,n,t,np,r);
  setrand(1000*p+0);
  while(1,
    a=random(p); b=random(p);
    if((4*a^3+27*b^2)%p==0, next);
    E=ellinit([a,b],p); n=ellcard(E);
    if(n%2!=0, next);
    r=polrootsmod(x^3+a*x+b,p);
    if(#r<1, next);
    t=p+1-n; np=p+1+t;
    return([p,0,a,b,n,np,t,#r]));
}
fn=Str(OUTDIR,"/curves.csv"); system(Str("rm -f ",fn));
write(fn,"p,c,a,b,l,lp,t");
foreach(PL,p, for(c=1,2, my(r=pick(p,c)); write(fn,r[1],",",r[2],",",r[3],",",r[4],",",r[5],",",r[6],",",r[7]); print("odd curve p=",r[1]," c=",r[2]," a=",r[3]," b=",r[4]," #E=",r[5]," #E'=",r[6]," t=",r[7])));
fn=Str(OUTDIR,"/curves_even.csv"); system(Str("rm -f ",fn));
write(fn,"p,c,a,b,l,lp,t,nroots");
foreach(PE,p, my(r=pickeven(p)); write(fn,r[1],",",r[2],",",r[3],",",r[4],",",r[5],",",r[6],",",r[7],",",r[8]); print("even curve p=",r[1]," a=",r[3]," b=",r[4]," #E=",r[5]," #E'=",r[6]," t=",r[7]," rational 2-torsion roots=",r[8]));
quit;
