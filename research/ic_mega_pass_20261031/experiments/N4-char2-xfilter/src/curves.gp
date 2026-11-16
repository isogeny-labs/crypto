\\ find curves y^2+xy=x^3+a x^2+b over F_2^n with #E = h*l, l prime
tobits(e,n)=my(v=Vec(lift(e.pol)));v=Polrev(v);sum(i=0,n-1,polcoef(lift(e.pol),i)*2^i);
mk(n,m,w)=sum(i=0,n-1,bittest(m,i)*w^i);
setrand(20261101);
{
for(n=7,23, if(n%2==0 || (n>7 && n<13) || n==11,next);
  T=ffinit(2,n); w=ffgen(T,'w); pm=subst(lift(T),variable(lift(T)),2);
  \\ Koblitz
  for(a=0,1, E=ellinit([1,a,0,0,1],w); N=ellcard(E); h=if(a,2,4); if(N%h==0 && isprime(N/h), print("{\"n\":",n,",\"poly\":",pm,",\"a\":",a,",\"b\":1,\"h\":",h,",\"l\":",N/h,",\"kind\":\"koblitz\"}")));
  for(a=0,1, h=if(a,2,4); cnt=0; while(cnt< if(a,2,1), bm=random(2^n-2)+2; b=mk(n,bm,w); E=ellinit([1,a,0,0,b],w); N=ellcard(E); if(N%h==0 && isprime(N/h), cnt++; print("{\"n\":",n,",\"poly\":",pm,",\"a\":",a,",\"b\":",bm,",\"h\":",h,",\"l\":",N/h,",\"kind\":\"random",cnt,"\"}"))));
);
}
quit
