\\ Independent PARI/GP check of the fixed V5 accepted rows.
\\ Input is exact Perl extraction of a,b,h1,h2 from frozen V5 stdout.
rows=readvec("/tmp/ec13_coset_rows_20261128.gpdat");
n=#rows;
if(n!=72,error("accepted row count mismatch"));
e=ellinit([0,210,0,186,126],211);
G=[165,19]; Q=[178,46];
base=Set(concat(vector(n,i,[rows[i][3],rows[i][4]]),vector(n,i,[rows[i][5],rows[i][6]])));
k=#base;
A=matrix(n,k,i,j,Mod(0,109));
t=vector(n,i,Mod(-4*rows[i][2],109));
c=vector(n,i,Mod(4*rows[i][1],109));
for(i=1,n,
  h1=[rows[i][3],rows[i][4]]; h2=[rows[i][5],rows[i][6]];
  j1=setsearch(base,h1); j2=setsearch(base,h2);
  if(!j1||!j2,error("missing base coordinate"));
  A[i,j1]+=Mod(1,109); A[i,j2]+=Mod(1,109);
  source=elladd(e,ellmul(e,G,rows[i][1]),ellmul(e,Q,rows[i][2]));
  if(elladd(e,h1,h2)!=ellmul(e,source,4),error("point identity mismatch at row ",i));
);
BA=matrank(A); BT=matrank(matconcat([A,Mat(t~)]));
if(BA!=42||BT!=43,error("rank mismatch"));
N=matker(A~);
chosen=0;
for(j=1,matsize(N)[2],
  y=N[,j]; yt=sum(i=1,n,y[i]*t[i]);
  if(yt!=Mod(0,109),chosen=j;break)
);
if(!chosen,error("target coefficient is in base column span"));
y=N[,chosen]; yt=sum(i=1,n,y[i]*t[i]); yc=sum(i=1,n,y[i]*c[i]);
d=yc/yt;
if(ellmul(e,G,lift(d))!=Q,error("independent scalar verification failed"));
print("verify rows=",n," observed_base=",k," base_rank=",BA," augmented_rank=",BT," kernel_dimension=",matsize(N)[2]);
print("certificate_kernel_column=",chosen," y_nonzero=",sum(i=1,n,y[i]!=Mod(0,109))," y_dot_target=",lift(yt)," y_dot_constant=",lift(yc)," d=",lift(d));
print("certificate_y=",vector(n,i,lift(y[i])));
print("all_72_group_identities=true scalar_group_check=true");
