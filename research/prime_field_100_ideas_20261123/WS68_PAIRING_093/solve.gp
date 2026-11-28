\\ #93 public supersingular E(F_43), order-11 subgroup. Target supplied as coordinates only.
default(parisize, 8000000);
p=43; r=11; k=2;
start=getwalltime();
t=ffgen(p^k,'t);
i=sqrt(-1+0*t);
E=ellinit([0,0,0,1+0*t,0*t]);
E0=ellinit([0,0,0,Mod(1,p),Mod(0,p)]);
G=[31+0*t,18+0*t];
H=[13+0*t,24+0*t];
Q=[-31+0*t,i*18];
setup_ms=getwalltime()-start;
if(ellcard(E0)!=44 || ellcard(E)!=1936 || (p%r)!=(-1+r)%r || (p^2%r)!=1, error("order/embedding certificate failed"));
if(ellmul(E,G,r)!=[0] || ellmul(E,H,r)!=[0] || ellmul(E,Q,r)!=[0], error("subgroup check failed"));
if(G==[0] || H==[0] || Q==[0],error("identity point"));
if(!ellisoncurve(E,G) || !ellisoncurve(E,H) || !ellisoncurve(E,Q),error("off curve"));
print("curve_p=",p," group_order=",ellcard(E0)," subgroup_r=",r," embedding_k=",k," extension_order=",ellcard(E));
print("G=",G," H=",H," Q=",Q," i=",i," i2=",i^2);
print("setup_ms=",setup_ms);
pair_start=getwalltime();
w=ellweilpairing(E,G,Q,r);
h=ellweilpairing(E,H,Q,r);
pair_ms=getwalltime()-pair_start;
if(w==1 || w^r!=1 || h^r!=1,error("pairing order failure"));
print("pair_GQ=",w," pair_HQ=",h," pairing_ms=",pair_ms);
dlp_start=getwalltime();
x=fflog(h,w,r);
dlp_ms=getwalltime()-dlp_start;
if(x<0 || x>=r || w^x!=h || ellmul(E,G,x)!=H,error("inverse transfer failure"));
print("transfer_x=",x," extension_dlp_ms=",dlp_ms," inverse_point_check=1");

rho_step(s)=
{
  my(P=s[1],a=s[2],b=s[3],z=-1);
  if(P==[0],z=0,for(n=0,42,if(P[1]==n+0*t,z=n%3;break())));
  if(z<0,error("rho x coordinate outside base field"));
  if(z==0,return([elladd(E,P,G),(a+1)%r,b]));
  if(z==1,return([elladd(E,P,H),a,(b+1)%r]));
  return([elladd(E,P,P),(2*a)%r,(2*b)%r]);
};
rho_run(x)=
{
  my(rho_start=getwalltime(),rho_ok=0,rho_transitions=0,rho_attempts=0,rho_degenerate=0,slow,fast,den,rho_x);
  for(seed=1,3,
    if(rho_ok,break());
    rho_attempts++;
    slow=[ellmul(E,G,seed),seed,0]; fast=slow;
    for(step=1,100,
      slow=rho_step(slow); fast=rho_step(rho_step(fast)); rho_transitions+=3;
      if(slow[1]==fast[1],
        den=(fast[3]-slow[3])%r;
        if(den==0,
          rho_degenerate++;
          print("rho_seed=",seed," collision_step=",step," denominator_zero=1");
          break()
        );
        rho_x=lift(Mod(slow[2]-fast[2],r)/Mod(den,r));
        if(ellmul(E,G,rho_x)!=H,error("rho inverse check failed"));
        rho_ok=1;
        print("rho_seed=",seed," collision_step=",step," rho_x=",rho_x);
        break()
      )
    )
  );
  print("rho_attempts=",rho_attempts," rho_transitions=",rho_transitions," rho_degenerate=",rho_degenerate," rho_ms=",getwalltime()-rho_start," rho_ok=",rho_ok);
  if(!rho_ok,error("rho censored at three seeds x 100 Floyd steps"));
  if(rho_x!=x,error("two solver disagreement"));
  return(rho_x);
};
rho_run(x);
print("RUN_OK=1");
