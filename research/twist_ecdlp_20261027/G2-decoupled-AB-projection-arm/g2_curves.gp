\\ g2_curves.gp -- curve selection (PREREGISTRATION sections 3, 4, 7). Writes OUT/curves.csv, OUT/curves_even.csv, OUT/curves_extreme_scan.csv
\\ kind: 0 = E3 primary (c=1,2), 1 = extension (c=3,4), 2 = extreme-twist (c=9)
default(parisizemax,2000000000);
genv(n,def)={my(v=getenv(n)); if(type(v)!="t_STR" || v=="", def, v)}
OUTDIR=genv("G2OUT","out");
nonres(p) = {my(d=2); while(kronecker(d,p)!=-1, d++); d}
\\ the c-th curve of the stream seeded 1000*p+c satisfying the odd-order rules (rule copied from e3_curves.gp)
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
\\ extreme-twist curve: among the first 400 admissible draws of setrand(31000+p), smallest #E' (first on ties)
pickextreme(p, fs) = {
  my(d=nonres(p), cnt=0, a, b, E, n, t, np, ET, best=0, draws=0);
  setrand(31000+p);
  while(cnt<400,
    a=random(p); b=random(p); draws++;
    if((4*a^3+27*b^2)%p==0, next);
    E=ellinit([a,b],p); n=ellcard(E);
    if(!isprime(n) || n==p, next);
    t=p+1-n; np=p+1+t;
    if(np==p || np%2==0 || gcd(n,np)!=1, next);
    ET=ellinit([a*d^2%p,b*d^3%p],p);
    if(#ellgroup(ET)!=1, next);
    cnt++;
    write(fs,p,",",cnt,",",a,",",b,",",n,",",np,",",t);
    if(best==0 || np<best[6], best=[p,9,a,b,n,np,t]));
  [best,draws]
}
\\ registered E3 curves (PREREGISTRATION section 3)
REG=[[101,1,51,91,97,107],[101,2,2,36,97,107],[211,1,57,128,199,225],[211,2,33,145,193,231],[401,1,170,292,397,407],[401,2,242,53,373,431]];
fn=Str(OUTDIR,"/curves.csv"); system(Str("rm -f ",fn));
write(fn,"p,c,a,b,l,lp,t,kind");
bad=0;
{
foreach([101,211,401],p, for(c=1,4, my(r=pick(p,c));
  if(c<=2, my(k=0); for(i=1,#REG, if(REG[i][1]==p && REG[i][2]==c, k=REG[i]));
     if(k==0 || k[3]!=r[3] || k[4]!=r[4] || k[5]!=r[5] || k[6]!=r[6], bad++; print("MISMATCH with registered E3 curve p=",p," c=",c)));
  write(fn,r[1],",",r[2],",",r[3],",",r[4],",",r[5],",",r[6],",",r[7],",",if(c<=2,0,1));
  print(if(c<=2,"primary","extension")," curve p=",r[1]," c=",r[2]," a=",r[3]," b=",r[4]," #E=",r[5]," #E'=",r[6]," t=",r[7])));
}
if(bad>0, error("registered E3 curves not reproduced: ",bad));
print("registered E3 six curves reproduced exactly (0 mismatches)");
fs=Str(OUTDIR,"/curves_extreme_scan.csv"); system(Str("rm -f ",fs)); write(fs,"p,idx,a,b,l,lp,t");
{
foreach([101,211,401],p, my(r=pickextreme(p,fs), b=r[1]);
  write(fn,b[1],",",b[2],",",b[3],",",b[4],",",b[5],",",b[6],",",b[7],",2");
  print("extreme-twist curve p=",p," c=9 a=",b[3]," b=",b[4]," #E=",b[5]," #E'=",b[6]," t=",b[7]," (smallest #E' among first 400 admissible draws; ",r[2]," raw draws; Hasse lower bound for #E' = ",ceil(p+1-2*sqrt(p)),")"));
}
fe=Str(OUTDIR,"/curves_even.csv"); system(Str("rm -f ",fe));
write(fe,"p,c,a,b,l,lp,t,nroots");
{
foreach([101,211],p, my(r=pickeven(p)); write(fe,r[1],",",r[2],",",r[3],",",r[4],",",r[5],",",r[6],",",r[7],",",r[8]);
  print("even curve p=",r[1]," a=",r[3]," b=",r[4]," #E=",r[5]," #E'=",r[6]," t=",r[7]," roots=",r[8]));
}
quit;
