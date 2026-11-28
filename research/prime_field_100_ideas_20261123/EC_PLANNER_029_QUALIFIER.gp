\\ Independent scalar-congruence replay of #29 V2 exact EC rewards/ranks.
B=16;H=8;
pointlog(P)={my(k=0);while(k<r,k++;if(ellmul(E,G,k)==P,return(k)));error("point log anchor")};
setup(pp,bb,rr,gg,expectedTargets)={my(n=0,xx=1,q);
  p=pp;b=bb;r=rr;E=ellinit([Mod(1,p),Mod(b,p)]);G=gg;
  if(ellcard(E)!=r || ellorder(E,G)!=r,error("qualifier group guard"));
  fb=vector(B);targets=vector(H);targetx=vector(H);
  while(n<B+H && xx<p,
    q=Mod(xx^3+xx+b,p);
    if(issquare(q),n++;if(n<=B,fb[n]=[Mod(xx,p),sqrt(q)],targets[n-B]=[Mod(xx,p),sqrt(q)];targetx[n-B]=xx));xx++;
  );
  if(n!=B+H || targetx!=expectedTargets,error("qualifier target sequence"));
  flog=vector(B,i,pointlog(fb[i]));tlog=vector(H,i,pointlog(targets[i]));
};
scalarhit(i,j,k,m,u)=lift(Mod(if(m%2,-flog[i],flog[i])+if((m\2)%2,-flog[j],flog[j])+if((m\4)%2,-flog[k],flog[k])-tlog[u],r))==0;
row_for(i,j,k,m,u)={my(v=vector(B+H,t,0));v[i]+=if(m%2,-1,1);v[j]+=if((m\2)%2,-1,1);v[k]+=if((m\4)%2,-1,1);v[B+u]=-1;v};
count_block(u,block,keep)={my(n=0,v);for(i=4*block-3,4*block,for(j=1,B,for(k=1,B,for(m=0,7,
  if(scalarhit(i,j,k,m,u),n++;if(keep,v=row_for(i,j,k,m,u);listput(~rows,v)));
))));n};
rank_of(rows)={my(V=Vec(rows),M);M=matrix(#V,B+H,ii,jj,Mod(V[ii][jj],r));matrank(M)};
run()={my(tot=vector(4,t,0),choices,expectedHits=[54,54,61,64],expectedRanks=[20,21,22,22],n,rew,cover);
  setup(1009,14,1013,[Mod(0,1009),Mod(425,1009)],[42,43,44,45,48,49,54,55]);
  for(u=1,H,for(block=1,4,tot[block]+=count_block(u,block,0)));
  if(tot!=[37,37,51,49],error("qualifier train totals"));
  print("qualifier_train curve_order=",r," block_totals=",tot," point_logs_anchored=",B+H);
  setup(1019,20,1033,[Mod(0,1019),Mod(64,1019)],[38,39,41,46,49,53,54,56]);
  for(arm=1,4,
    choices=if(arm==1,[3,4,1,1,1,1,1,1],if(arm==2,vector(H,t,3),if(arm==3,vector(H,t,1),[1,1,3,1,4,1,1,1])));
    rows=List();n=0;cover=0;rew=vector(H);
    for(u=1,H,rew[u]=count_block(u,choices[u],1);n+=rew[u];if(rew[u]>0,cover++));
    if(n!=expectedHits[arm] || rank_of(rows)!=expectedRanks[arm] || cover!=7,error("qualifier heldout totals/rank"));
    print("qualifier_arm=",arm," choices=",choices," rewards=",rew," hits=",n," coverage=",cover," rank=",rank_of(rows));
  );
  print("qualifier_heldout curve_order=",r," point_logs_anchored=",B+H," all_scalar_relations_valid=true");
};
run();print("complete independent EC planner scalar replay");
