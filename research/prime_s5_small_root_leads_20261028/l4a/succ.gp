ok=[[[3,3,1],1],[[5,8,4],1],[[4,5,2],2],[[3,3,1],25],[[7,16,12],3],[[10,32,32],2],[[9,24,16],1],[[5,7,3],3]];
a=-3; b=56;
S3(u,v,w)=(u-v)^2*w^2-2*((u+v)*(u*v+a)+2*b)*w+(u*v-a)^2-4*b*(u+v);
S4s=polresultant(S3(x1,x2,X),S3(x3,z,X),X);
{my(n=0,g=0);foreach(ok,c,my(e=c[1],r=polroots(x^3-e[1]*x^2+e[2]*x-e[3]),xs=vector(3,i,round(real(r[i]))));
 my(v=subst(subst(subst(subst(S4s,x1,xs[1]),x2,xs[2]),x3,xs[3]),z,c[2]));n++;if(v==0,g++,printf("NOT global: e=%s z=%d v=%d\n",e,c[2],v)));
 printf("distinct successful planted (e*, z) across all v1 cells: %d, global (S4 = 0 over Z): %d\n",n,g)}
