\\ Height above the floor by BFS on the Phi_l graph: distance to the nearest vertex with one rational root (with multiplicity).
nb(j,l,p,P)=my(f=subst(subst(P,x,Mod(j,p)),y,'Y), r=polrootsmod(f,p)); [lift(r), #r];
height(j0,l,p)={my(P=polmodular(l), seen=Map(), front=[j0], d=0);
  mapput(seen,j0,1);
  while(d<=200,
    my(nxt=List());
    for(i=1,#front, my(v=nb(front[i],l,p,P)); if(v[2]==1, return(d));
      for(k=1,#v[1], my(u=v[1][k]); if(!mapisdefined(seen,u), mapput(seen,u,1); listput(nxt,u))));
    front=Vec(nxt); d++); -1};
vv(t,p,l)=my(D=t^2-4*p, s=0); while(D%(l^2)==0 && (D/l^2)%4<2, D/=l^2; s++); s;
