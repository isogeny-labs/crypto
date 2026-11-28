\\ Bounded simulated annealing on a public four-spin EC relation factor.
\\ Fixed 16 candidate masks, 128 deterministic trials, at most 16 EC sums/trial.
p=1009; ntrials=128; cap=16; seed0=202611240000;
E=ellinit([Mod(1,p),Mod(14,p)]);
Q=[Mod(0,p),Mod(425,p)];
F3=[Mod(4,p),Mod(456,p)]; F5=[Mod(10,p),Mod(32,p)]; F12=[Mod(29,p),Mod(35,p)];
F4=[Mod(5,p),Mod(12,p)]; F11=[Mod(25,p),Mod(23,p)];
T1=[F3,F5,F12]; T2=[F4,F5,F11];

cycdist(x,y)={my(d=abs(x-y));min(d,p-d)};
ec_energy(mask)={
  my(T=if(mask<8,T1,T2),s=mask%8,u,v,w,R);
  u=ellmul(E,T[1],if(s%2,-1,1));
  v=ellmul(E,T[2],if((s\2)%2,-1,1));
  w=ellmul(E,T[3],if((s\4)%2,-1,1));
  R=elladd(E,elladd(E,u,v),w);
  if(R==[0],return(2*p));
  cycdist(lift(R[1]),0)+cycdist(lift(R[2]),425);
};

toggle(mask,bit)={if((mask\2^bit)%2,mask-2^bit,mask+2^bit)};

anneal_trial(seed)={
  my(mask,e,candidate,ec,T,threshold,calls=1,up=0,accepted=0);
  setrand(seed);
  mask=random(16);e=ec_energy(mask);
  if(e==0,return([1,calls,up,accepted]));
  for(step=1,cap-1,
    candidate=toggle(mask,random(4));ec=ec_energy(candidate);calls++;
    if(ec==0,return([1,calls,up,accepted]));
    if(ec<=e,mask=candidate;e=ec,
      up++;
      T=500/2^((step-1)\4);
      threshold=floor(1000000*exp(-(ec-e)/T));
      if(random(1000000)<threshold,mask=candidate;e=ec;accepted++);
    );
  );
  [0,calls,up,accepted];
};

random_trial(seed)={
  my(mask,calls=0);
  setrand(seed);
  for(step=1,cap,
    mask=random(16);calls++;
    if(ec_energy(mask)==0,return([1,calls]));
  );
  [0,calls];
};

run()={
  my(marked=List(),a,b,as=0,rs=0,acalls=0,rcalls=0,up=0,accepted=0,exact_calls=0);
  if(!ellisoncurve(E,Q),error("target off curve"));
  for(j=1,3,if(!ellisoncurve(E,T1[j]) || !ellisoncurve(E,T2[j]),error("base off curve")));
  for(mask=0,15,
    if(ec_energy(mask)==0,listput(marked,mask));
  );
  if(Vec(marked)!=[5,14],error("positive-control mark set changed"));
  exact_calls=6; \\ fixed ascending scan first reaches mask 5 on call 6.
  for(trial=1,ntrials,
    a=anneal_trial(seed0+trial);b=random_trial(seed0+trial);
    as+=a[1];rs+=b[1];acalls+=a[2];rcalls+=b[2];up+=a[3];accepted+=a[4];
  );
  if(up==0,error("annealing uphill branch unexercised"));
  print("instance p=",p," masks=16 marked=",Vec(marked)," trials=",ntrials," cap=",cap," seed_first=",seed0+1," seed_last=",seed0+ntrials);
  print("energy=cyclic_x_distance_plus_cyclic_y_distance infinity=",2*p," schedule_T=500,250,125,125/2 per_four_proposals");
  print("anneal successes=",as," total_ec_evals=",acalls," uphill_proposals=",up," uphill_accepted=",accepted," point_adds=",2*acalls);
  print("uniform_with_replacement successes=",rs," total_ec_evals=",rcalls," point_adds=",2*rcalls);
  print("ascending_exhaustive_first_hit_ec_evals=",exact_calls," full_scan_ec_evals=16);
  print("complete_anneal48_control");
};
run();
