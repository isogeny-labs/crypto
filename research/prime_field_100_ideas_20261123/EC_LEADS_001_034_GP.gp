\\ Frozen exact EC(Fp) screens for proposals 6, 16, 21, and 29.
p=1009; a=Mod(1,p); b=Mod(14,p); E=ellinit([a,b]);
if(ellcard(E)!=1013 || !isprime(1013),error("main curve order guard"));
G=[Mod(0,p),Mod(425,p)];
if(!ellisoncurve(E,G) || ellorder(E,G)!=1013,error("main generator guard"));
liftable(x)=issquare(Mod(x^3+x+14,p));

shift_screen()={
  my(n=0,ok=0,bad=0,first=-1,y0);
  for(xx=0,p-1,
    if(liftable(xx),
      n++;ok++;
      if(!liftable((xx+1)%p),bad++;if(first<0,first=xx));
    );
  );
  if(n!=506 || ok!=n || bad<=0,error("PGL2 point-set screen guard"));
  y0=lift(sqrt(Mod(first^3+first+14,p)));
  print("lead6 shift_x_plus_1 liftable_input_x=",n," identity_valid=",ok,
        " shifted_nonliftable=",bad," first_bad_point=",[first,y0]);
};

cmphi(P0,zeta)=if(P0==[0],[0],[zeta*P0[1],P0[2]]);
cm_control()={
  my(E0=ellinit([Mod(0,p),Mod(3,p)]),zeta=0,found=0,G0=[0],P0,H,Q0,R0,T0,xx0=0,q0);
  if(ellcard(E0)!=948 || !isprime(79),error("CM curve order guard"));
  for(k=2,p-1,if(zeta==0 && Mod(k,p)^3==1,zeta=Mod(k,p)));
  if(zeta==0,error("cube root guard"));
  for(xx=1,p-1,
    if(found==0,
      q0=Mod(xx^3+3,p);
      if(issquare(q0),
        P0=[Mod(xx,p),sqrt(q0)];
        H=ellmul(E0,P0,12);
        if(H!=[0] && ellorder(E0,H)==79,G0=H;found=1;xx0=xx);
      );
    );
  );
  if(found!=1 || !ellisoncurve(E0,G0),error("CM subgroup generator guard"));
  Q0=ellmul(E0,G0,2);R0=elladd(E0,G0,Q0);
  T0=elladd(E0,cmphi(G0,zeta),cmphi(Q0,zeta));
  if(T0!=cmphi(R0,zeta),error("CM automorphism additivity guard"));
  if(cmphi(cmphi(cmphi(G0,zeta),zeta),zeta)!=G0,error("CM orbit order guard"));
  if(cmphi(G0,zeta)==G0,error("CM nontrivial orbit guard"));
  print("lead6_cm_control curve_card=948 subgroup_order=79 cube_root=",lift(zeta),
        " seed_x=",xx0," orbit_x=",[lift(G0[1]),lift(cmphi(G0,zeta)[1]),lift(cmphi(cmphi(G0,zeta),zeta)[1])],
        " additive_identity=true");
};

coord_screen()={
  my(Q=ellmul(E,G,2),R=ellmul(E,G,3),xP,xQ,xR);
  if(elladd(E,G,Q)!=R,error("point-addition positive control"));
  xP=lift(G[1]);xQ=lift(Q[1]);xR=lift(R[1]);
  if(Mod(xR,p)==Mod(xP*xQ,p) || Mod(xR,p)==Mod(xP+xQ,p),error("coordinate counterexample guard"));
  print("lead16_21 identity_isogeny P_x=",xP," Q_x=",xQ," sum_x=",xR,
        " x_product=",(xP*xQ)%p," x_add=",(xP+xQ)%p," point_additive=true");
};

build_inputs()={
  my(n=0,xx=1,q);
  fbx=vector(16);fb=vector(16);targetx=vector(8);targets=vector(8);
  while(n<24 && xx<p,
    q=Mod(xx^3+xx+14,p);
    if(issquare(q),
      n++;
      if(n<=16,
        fbx[n]=xx;fb[n]=[Mod(xx,p),sqrt(q)],
        targetx[n-16]=xx;targets[n-16]=[Mod(xx,p),sqrt(q)]
      );
    );
    xx++;
  );
  if(n!=24,error("factor base and target size guard"));
  for(i=1,16,if(!ellisoncurve(E,fb[i]) || ellorder(E,fb[i])!=1013,error("factor base point guard")));
  for(i=1,8,if(!ellisoncurve(E,targets[i]) || ellorder(E,targets[i])!=1013,error("target point guard")));
  print("lead29 base_x=",fbx," target_x=",targetx);
};

sched_screen()={
  my(negfb=vector(16,i,ellsub(E,[0],fb[i])),hits=matrix(8,4,ii,jj,0),
     block,e1,e2,e3,P1,P2,P3,Qsum,train,hold,selected=1,allhold=0);
  for(i=1,16,
    block=1+(i-1)\4;
    for(j=1,16,for(k=1,16,for(mask=0,7,
      e1=if(mask%2,-1,1);e2=if((mask\2)%2,-1,1);e3=if((mask\4)%2,-1,1);
      P1=if(e1<0,negfb[i],fb[i]);
      P2=if(e2<0,negfb[j],fb[j]);
      P3=if(e3<0,negfb[k],fb[k]);
      Qsum=elladd(E,elladd(E,P1,P2),P3);
      for(u=1,8,if(Qsum==targets[u],hits[u,block]++));
    )));
  );
  train=vector(4,bb,sum(u=1,4,hits[u,bb]));
  hold=vector(4,bb,sum(u=5,8,hits[u,bb]));
  for(bb=2,4,if(train[bb]>train[selected],selected=bb));
  for(bb=1,4,allhold+=hold[bb]);
  for(u=1,8,print("lead29_target index=",u," x=",targetx[u]," block_hits=",[hits[u,1],hits[u,2],hits[u,3],hits[u,4]]));
  print("lead29_summary candidate_sign_patterns=32768 per_block=8192 train_hits=",train,
        " selected_block=",selected," holdout_hits=",hold,
        " selected_holdout=",hold[selected]," static_block1_holdout=",hold[1],
        " all_block_holdout_mean=",allhold,"/4");
};

shift_screen();
cm_control();
coord_screen();
build_inputs();
sched_screen();
print("complete EC leads 6 16 21 29");
