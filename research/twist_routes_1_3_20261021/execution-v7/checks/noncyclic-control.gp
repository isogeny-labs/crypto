default(parisizemax,4000000000);
seed(A,B,p,n,want)={my(E=ellinit([Mod(A,p),Mod(B,p)]));for(x=0,p-1,for(y=1,(p-1)/2,if((y*y-x^3-A*x-B)%p==0,if(!want||ellorder(E,[Mod(x,p),Mod(y,p)])==n,return([x,y]));break)));error("no seed");};
base(A,B,a,b,p,d,H,n,c,k)={my(E=ellinit([Mod(A,p),Mod(B,p)]),R=[Mod(H[1],p),Mod(H[2],p)],T=R,S=[]);for(i=1,n,if(T==[0],break);my(x=lift(T[1]/Mod(d,p)+c));if(kronecker((x^3+a*x+b)%p,p)==1,S=setunion(S,[x]));if(#S==k,break);T=elladd(E,T,R));S;};
E=ellinit([Mod(0,7),Mod(1,7)]);if(vecsort(ellgroup(E))!=[2,6]||seed(0,1,7,6,1)!=[1,3],error("noncyclic control failed"));print("NONCYCLIC_CONTROL_OK");
