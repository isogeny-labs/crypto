\\ Independent re-implementation: non-backtracking descending walks; height = shortest walk to a vertex with one rational root.
roots(j,l,p,P)={my(f=subst(subst(P,x,Mod(j,p)),y,'Y), fa=factormod(f,p), r=List()); for(i=1,#fa~, if(poldegree(fa[i,1])==1, my(z=lift(-polcoef(fa[i,1],0)/polcoef(fa[i,1],1))); for(k=1,fa[i,2], listput(r,z)))); vecsort(Vec(r))};
height(j0,l,p,dep)={my(P=polmodular(l), s=roots(j0,l,p,P), best=dep+1);
  if(#s==1, return(0));
  foreach(Set(s), r, my(prev=j0, cur=r, len=1);
    while(len<best && len<=dep,
      if(cur==0||cur==1728%p, break);
      my(R=roots(cur,l,p,P)); if(#R==1, best=min(best,len); break);
      my(k=select(z->z==prev,R,1)); if(#k==0, error("no dual")); R=vecextract(R, Str("^",k[1]));
      prev=cur; cur=R[1]; len++));
  if(best>dep, -1, best)};
read("real17.gp");
{for(i=1,#C, my(c=C[i], h=height(c[3],c[5],c[2],c[6]));
  my(D=c[4]^2-4*c[2], dK=coredisc(D));
  print(c[1]," dK=",dK," l=",c[5]," depth=",c[6]," height=",h," level=",c[6]-h))}
