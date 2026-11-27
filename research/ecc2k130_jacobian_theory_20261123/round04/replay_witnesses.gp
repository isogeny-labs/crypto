default(parisize,268435456);
default(parisizemax,536870912);
checkwitness(d,F,g) = {
  my(T);
  if(poldegree(F)!=d || !polisirreducible(F) || polsturm(F)!=d,error("field check"));
  T=charpoly(g);
  if(poldegree(T)!=d || pollead(T)!=1 || denominator(T)!=1,error("integrality check"));
  if((-1)^d*polcoef(T,0)!=263,error("norm check"));
  if(polsturm(T,[-oo,0])!=0,error("total positivity check"));
  print("degree=",d,"; irreducible_real_field=pass; integral=pass; norm=263; totally_positive=pass");
};
iferr(read("witnesses.gp"),E,print("PARI_ERROR=",E);quit(2));
quit;
