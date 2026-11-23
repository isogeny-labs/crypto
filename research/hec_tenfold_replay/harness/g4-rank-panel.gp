\\ Take one coefficient per cube-root-of-unity scaling orbit.
\\ Test the largest prime divisor when its valuation is one.
panel(p) = {
  my(used=vector(p),f,n,F,l);
  for(c=0,p-1,
    my(i=1+lift(Mod(c,p)^3));
    if(used[i],next); used[i]=1;
    f=Mod(1,p)*(x^9+x^3+c*x+1);
    if(poldegree(gcd(f,deriv(f)))>0,print(p," ",c," singular");next);
    n=subst(hyperellcharpoly(f),x,1);
    F=factor(n); l=F[matsize(F)[1],1];
    if(l<1000 || n%(l*l)==0,print(p," ",c," excluded ",n," ",l);next);
    print(p," ",c," selected ",n," ",l);
  );
};
forprime(p=31,67,panel(p))
