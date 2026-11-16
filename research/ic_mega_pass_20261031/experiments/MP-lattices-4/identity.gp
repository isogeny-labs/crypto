setrand(1); p=nextprime(2^32); c=Mod(123456789,p); A=Mod(5,p); B=c^2;
E=ellinit([0,A,0,B,0]); T=[0,0];
Yf(P,cc)=(P[1]-cc)/(P[1]+cc);
{ok=0;bad=0;negc=0;
for(t=1,50, P=vector(4,i,random(E)); S=elladd(E,elladd(E,P[1],P[2]),elladd(E,P[3],P[4]));
 for(mask=0,15, Q=vector(4,i,if(bittest(mask,i-1),elladd(E,P[i],T),P[i]));
  sg=vector(4,i,Yf(Q[i],c)/Yf(P[i],c)); par=hammingweight(mask)%2;
  if(sg==vector(4,i,if(bittest(mask,i-1),-1,1)),ok++,bad++);
  S2=elladd(E,elladd(E,Q[1],Q[2]),elladd(E,Q[3],Q[4]));
  if(par==0, if(S2!=S,bad++), if(S2!=elladd(E,S,T),bad++)));
 c2=c+1; if(Yf(elladd(E,P[1],T),c2)!=-Yf(P[1],c2),negc++));
print("signflip_ok=",ok," bad=",bad," negctrl_wrongc_identity_fails=",negc,"/50");}
{cnt=0;for(d=0,8,for(a=0,4,for(b=0,4,for(cc=0,4,if(2*(a+b+cc)+d<=8,cnt++)))));print("monomials=",cnt);}
