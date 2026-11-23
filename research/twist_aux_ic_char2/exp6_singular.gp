tovec(x, n) = my(v=Vecrev(x.pol, n)); vector(n, i, Mod(lift(v[i]), x.p));
chk(q, seed) = {
  my(n=4, t=ffgen(q^4,'t), bas=vector(n,i,t^(i-1)), sing=0, tot=0, err=0, rk=0);
  setrand(seed);
  for(trial=1,60, my(e=vector(3,i,random(t))); for(i=1,3,for(j=1,3, if(i==j,next); my(k=6-i-j, A=e[i]-e[k], B=e[j]-e[k], M=matrix(n,n));
    for(b=1,n, M[,b]=tovec(B*bas[b]-A^q*bas[b]^q,n)~); tot++;
    if(matdet(M)==0, sing++; if(((B/A^q)^((q^4-1)/(q-1)))!=1, err++)))));
  printf("q=%d: pairs %d singular %d (rate %.3f, 1/(q-1)=%.3f); singular without norm condition: %d\n", q, tot, sing, sing/tot*1., 1./(q-1), err);
}
chk(11,21); chk(13,22); chk(17,23); chk(19,24);
