\\ Frozen #48 replication: two disjoint public prime fields, fixed point rule.
\\ Exact EC group predicate and coordinate-distance Metropolis energy.
ntrials=128; cap=16; seed0=202611250000;
toggle(mask,bit)={if((mask\2^bit)%2,mask-2^bit,mask+2^bit)};
ecenergy(mask)={
  my(T=if(mask<8,T1,T2),s=mask%8,u,v,w,R,dx,dy);
  u=ellmul(E,T[1],if(s%2,-1,1));
  v=ellmul(E,T[2],if((s\2)%2,-1,1));
  w=ellmul(E,T[3],if((s\4)%2,-1,1));
  R=elladd(E,elladd(E,u,v),w);
  if(R==Q,return(0));
  if(R==[0],return(2*pp));
  dx=abs(lift(R[1])-lift(Q[1]));dy=abs(lift(R[2])-lift(Q[2]));
  min(dx,pp-dx)+min(dy,pp-dy);
};

run_curve(p,a,b)={
  my(xx=1,n=0,q,
     setup_scan=0,marked=List(),as=0,rs=0,acalls=0,rcalls=0,up=0,accepted=0,
     sa,sb,aa,bb,cur,ec,m,prop,temp,threshold,hit,calls,uptrial,accepttrial,
     distance);
  pp=p;E=ellinit([Mod(a,p),Mod(b,p)]);F=vector(6);T1=0;T2=0;Q=0;
  if((4*a^3+27*b^2)%p==0,error("singular curve"));
  while(n<6 && xx<p,
    q=Mod(xx^3+a*xx+b,p);setup_scan++;
    if(issquare(q),n++;F[n]=[Mod(xx,p),sqrt(q)];if(!ellisoncurve(E,F[n]),error("base off curve")));
    xx++;
  );
  if(n!=6,error("insufficient base"));
  T1=[F[1],F[2],F[3]];T2=[F[4],F[5],F[6]];
  \\ Planted target: first triple and signs [-1,+1,-1], mask 5.
  Q=elladd(E,elladd(E,ellmul(E,F[1],-1),F[2]),ellmul(E,F[3],-1));
  if(Q==[0] || !ellisoncurve(E,Q),error("target guard"));

  for(mask=0,15,if(ecenergy(mask)==0,listput(marked,mask)));
  if(#marked<1 || marked[1]!=5,error("planted relation absent"));

  for(trial=1,ntrials,
    setrand(seed0+trial);
    m=random(16);cur=ecenergy(m);calls=1;hit=(cur==0);uptrial=0;accepttrial=0;
    for(step=1,cap-1,
      if(!hit,
        prop=toggle(m,random(4));ec=ecenergy(prop);calls++;
        if(ec==0,hit=1,
          if(ec<=cur,m=prop;cur=ec,
            uptrial++;
            temp=500/2^((step-1)\4);
            threshold=floor(1000000*exp(-(ec-cur)/temp));
            if(random(1000000)<threshold,m=prop;cur=ec;accepttrial++);
          );
        );
      );
    );
    as+=hit;acalls+=calls;up+=uptrial;accepted+=accepttrial;

    setrand(seed0+trial);hit=0;calls=0;
    for(step=1,cap,
      if(!hit,m=random(16);calls++;if(ecenergy(m)==0,hit=1));
    );
    rs+=hit;rcalls+=calls;
  );
  if(up==0,error("uphill branch not exercised"));
  print("curve p=",p," a=",a," b=",b," card=",ellcard(E)," base_x=",vector(6,i,lift(F[i][1]))," Q=",Q," setup_x_scans=",setup_scan," positive_control_ec_evals=16");
  print("marked_masks=",Vec(marked)," marked_fraction=",#marked,"/16");
  print("anneal successes=",as,"/",ntrials," ec_evals=",acalls," ec_adds=",2*acalls," uphill_proposals=",up," uphill_accepted=",accepted);
  print("uniform successes=",rs,"/",ntrials," ec_evals=",rcalls," ec_adds=",2*rcalls);
};

run_curve(1009,1,14);
run_curve(1019,2,3);
print("complete_two_curve_replication");
