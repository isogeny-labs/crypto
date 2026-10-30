a=-3; b=56;
S3(u,v,w)=(u-v)^2*w^2-2*((u+v)*(u*v+a)+2*b)*w+(u*v-a)^2-4*b*(u+v);
S4s=polresultant(S3(x1,x2,X),S3(x3,z,X),X);
v(xs,zz)=subst(subst(subst(subst(S4s,x1,xs[1]),x2,xs[2]),x3,xs[3]),z,zz);
{foreach([[[1,1,1],1],[[1,1,1],25],[[2,2,3],3],[[2,4,4],2],[[1,4,4],1],[[1,1,3],3],[[1,2,2],1],[[1,1,2],2],[[3,6,6],3],[[1,2,2],260423849117]],c,
 printf("x=%s z=%d  S4(symbolic) over Z = %d\n",c[1],c[2],v(c[1],c[2])))}
