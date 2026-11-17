{lisosD(a4,a6,p,l)=
  my(E=ellinit([a4,a6],p),k=if(l==2,1,(l-1)/2),dv=elldivpol(E,l),fa=factor(dv),fs=List(),res=List(),jE=E.j,bad=0);
  for(i=1,#fa~, listput(fs,fa[i,1]));
  fs=Vec(fs);
  my(cands=subsets(fs,k,1,0,Mod(1,p)*1));
  my(Phi=polmodular(l),fy=lift(subst(subst(Phi,'x,jE),'y,'x)),rts=Set(apply(r->lift(r),Vec(polrootsmod(fy,p)))));
  my(pts=vector(5,i,random(E)));
  for(i=1,#cands,
    my(G=cands[i],iso=iferr(ellisogeny(E,G),e,0));
    if(iso==0,print("isofail ",G);bad++;next);
    my(E2=ellinit(iso[1]),ok=1); if(#E2==0,print("sing");bad++;next);
    for(u=1,#pts, my(Q=iferr(ellisogenyapply(iso[2],pts[u]),e,0)); if(Q==0||!ellisoncurve(E2,Q),print("ptfail ",G," ",pts[u]," ",Q);ok=0;break));
    if(!ok,bad++;next);
    my(j2=lift(E2.j));
    if(!setsearch(rts,j2),print("jfail ",j2);bad++;next);
    listput(res,[shortof(E2,p),ellcard(E2),j2]));
  [Vec(res),bad]
}
print(lisosD(0,5,13,2));
