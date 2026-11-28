\\ Frozen adaptive finite-state bandit on exact public EC signed relations.
B=16;H=8;seed0=20261125;
setup_curve(pp,bb,rr,expectedTargets)={my(n=0,xx=1,q);
  p=pp;b=bb;r=rr;E=ellinit([Mod(1,p),Mod(b,p)]);
  if(ellcard(E)!=r || !isprime(r),error("curve order guard"));
  fb=vector(B);negfb=vector(B);fbx=vector(B);targets=vector(H);targetx=vector(H);
  while(n<B+H && xx<p,
    q=Mod(xx^3+xx+b,p);
    if(issquare(q),
      n++;
      if(n<=B,fb[n]=[Mod(xx,p),sqrt(q)];fbx[n]=xx;negfb[n]=ellsub(E,[0],fb[n]),targets[n-B]=[Mod(xx,p),sqrt(q)];targetx[n-B]=xx);
    );xx++;
  );
  if(n!=B+H || targetx!=expectedTargets,error("factor base or target rule guard"));
  for(i=1,B,if(!ellisoncurve(E,fb[i]) || ellorder(E,fb[i])!=r,error("factor base point guard")));
  for(i=1,H,if(!ellisoncurve(E,targets[i]) || ellorder(E,targets[i])!=r,error("target point guard")));
};
row_for(i,j,k,m,u)={my(v=vector(B+H,h,0));v[i]+=if(m%2,-1,1);v[j]+=if((m\2)%2,-1,1);v[k]+=if((m\4)%2,-1,1);v[B+u]=-1;v};
signed_sum(i,j,k,m)={my(P1=if(m%2,negfb[i],fb[i]),P2=if((m\2)%2,negfb[j],fb[j]),P3=if((m\4)%2,negfb[k],fb[k]));elladd(E,elladd(E,P1,P2),P3)};
block_hits(u,block,record)={my(hits=0,S,v);
  for(i=4*block-3,4*block,for(j=1,B,for(k=1,B,for(m=0,7,
    S=signed_sum(i,j,k,m);
    if(S==targets[u],hits++;if(record,v=row_for(i,j,k,m,u);listput(~armRows,v)));
  ))));
  hits
};
rank_rows(rows)={my(V=Vec(rows),M);if(#V==0,return(0));M=matrix(#V,B+H,ii,jj,Mod(V[ii][jj],r));matrank(M)};
bestblock(v)={my(k=1);for(i=2,4,if(v[i]>v[k],k=i));k};
train()={my(t0=getabstime(),w0=getwalltime(),hits=matrix(H,4,i,j,0),tot=vector(4,j,0));
  setup_curve(1009,14,1013,[42,43,44,45,48,49,54,55]);
  print("train_curve p=",p," b=",b," order=",r," base_x=",fbx," target_x=",targetx);
  for(u=1,H,for(block=1,4,
    hits[u,block]=block_hits(u,block,0);tot[block]+=hits[u,block];
  ));
  for(u=1,H,print("train_target x=",targetx[u]," block_hits=",[hits[u,1],hits[u,2],hits[u,3],hits[u,4]]));
  trainTotals=tot;trainMeans=vector(4,j,tot[j]/H);trainBest=bestblock(trainMeans);
  print("train_summary target_count=",H," signed_patterns=",H*4*8192," candidate_sum_adds=",H*4*8192*2,
        " target_comparisons=",H*4*8192," block_totals=",trainTotals," q_initial=",trainMeans,
        " best_block=",trainBest," cpu_ms=",getabstime()-t0," wall_ms=",getwalltime()-w0);
};
run_arm(name,mode)={my(t0=getabstime(),w0=getwalltime(),q=vector(4,j,trainMeans[j]),hits=0,covered=0,choice,reward,rng=seed0,
  choices=vector(H),rewards=vector(H));
  armRows=List();
  for(u=1,H,
    if(mode==1,choice=bestblock(q));
    if(mode==2,choice=trainBest);
    if(mode==3,choice=1);
    if(mode==4,rng=(1103515245*rng+12345)%(2^31);choice=1+((rng\65536)%4));
    if(choice<1 || choice>4,error("planner choice guard"));
    reward=block_hits(u,choice,1);choices[u]=choice;rewards[u]=reward;hits+=reward;
    if(reward>0,covered++);
    if(mode==1,q[choice]=(q[choice]+reward)/2);
    print("heldout_step arm=",name," target_index=",u," x=",targetx[u]," block=",choice," exact_hits=",reward," q_after=",if(mode==1,q,"none"));
  );
  print("heldout_summary arm=",name," choices=",choices," rewards=",rewards," hits=",hits," covered_targets=",covered,"/",H,
        " row_rank=",rank_rows(armRows),"/",B+H," signed_patterns=",H*8192," candidate_sum_adds=",H*8192*2,
        " target_comparisons=",H*8192," cpu_ms=",getabstime()-t0," wall_ms=",getwalltime()-w0);
};
run_all()={my(t0=getabstime(),w0=getwalltime(),h0,hw0);
  train();
  h0=getabstime();hw0=getwalltime();
  setup_curve(1019,20,1033,[38,39,41,46,49,53,54,56]);
  print("heldout_curve p=",p," b=",b," order=",r," base_x=",fbx," target_x=",targetx,
        " setup_cpu_ms=",getabstime()-h0," setup_wall_ms=",getwalltime()-hw0);
  run_arm("adaptive_ema",1);run_arm("train_best",2);run_arm("static_1",3);run_arm("seeded_random",4);
  print("all_arms_total_cpu_ms=",getabstime()-t0," all_arms_total_wall_ms=",getwalltime()-w0);
};
run_all();print("complete EC adaptive planner 29");
