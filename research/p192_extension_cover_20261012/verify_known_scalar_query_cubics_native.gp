\\ Independent PARI replay of all frozen known-scalar query cubic roots.
read("research/p192_extension_cover_20261012/type_i_known_scalar_query_cubic_roots_input.gp");
v=varhigher("v",x);
P(z)=sum(i=1,#z,Mod(z[i],p)*x^(i-1));
curve=sum(i=1,#terms,Mod(terms[i][3],p)*x^terms[i][1]*v^terms[i][2]);
{
  my(counts=vector(4,i,0),root_total=0);
  if(#cases!=256,error("query cubic count"));
  for(k=1,#cases,
    my(c=cases[k],h=P(c[1]),yy=P(c[2]),factors,linear=0,point);
    if(poldegree(h,x)!=3 || pollead(h)!=Mod(1,p),error("cubic degree or monic"));
    if(poldegree(gcd(h,deriv(h,x)),x)!=0,error("cubic repeated root"));
    if(subst(curve,v,yy)%h!=0,error("cubic not on quartic"));
    factors=factormod(lift(h),p);
    for(j=1,matsize(factors)[1],
      if(factors[j,2]!=1,error("factor repeated"));
      if(poldegree(factors[j,1],x)==1,linear++);
    );
    if(linear!=#c[3] || (linear!=0 && linear!=1 && linear!=3),
      error("rational root count"));
    for(j=1,#c[3],
      point=c[3][j];
      if(subst(h,x,Mod(point[1],p))!=0,error("listed root absent"));
      if(subst(yy,x,Mod(point[1],p))!=Mod(point[2],p),
        error("listed ordinate mismatch"));
      if(subst(subst(curve,x,Mod(point[1],p)),v,Mod(point[2],p))!=0,
        error("listed point off quartic"));
    );
    counts[linear+1]++;
    root_total+=linear;
  );
  print("QUERY_CUBIC_NATIVE_PASS cases=",#cases,
        " roots=",root_total," counts=",counts);
}
