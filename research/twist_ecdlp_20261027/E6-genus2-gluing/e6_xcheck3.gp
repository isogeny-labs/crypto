default(parisizemax,2000000000);
one(p,fn)={my(L=readstr(fn),c=List(),ok=1);
  foreach(L,s,my(v=eval(Str("[",strjoin(strsplit(s," "),","),"]")));
    my(A=v[1],B=v[2],nE=v[3],nEt=v[4],t=v[5]);
    \\ own point count by Legendre symbol
    my(n=1,d=2);for(u=0,p-1,n+=1+kronecker((u^3+A*u+B)%p,p));
    while(kronecker(d,p)!=-1,d++);
    my(Ap=(A*d^2)%p,Bp=(B*d^3)%p,m=1);for(u=0,p-1,m+=1+kronecker((u^3+Ap*u+Bp)%p,p));
    if(n!=nE||m!=nEt||nE+nEt!=2*p+2||p+1-n!=t,ok=0;print("MISMATCH ",v," own=",[n,m]));
    listput(c,sqrt(Pi*n/2)/p));
  c=vecsort(Vec(c));print("p=",p," curves=",#c," allmatch=",ok," ctie_full_med(own)=",c[(#c+1)\2]," max=",c[#c]," hasse bound=",sqrt(Pi/2)*(sqrt(p)+1)/p)};
one(4001,"raw_4001.txt");one(10007,"raw_10007.txt");one(20011,"raw_20011.txt");
quit;
