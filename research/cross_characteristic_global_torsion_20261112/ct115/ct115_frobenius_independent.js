#!/usr/bin/env node
'use strict';
// CT115 independent target point law (line intercept form) and norm inversion.
function check(cond,msg){if(!cond)throw Error(msg);}
const poly=0x130027, lim=(1<<20)-1;
function fmul(x,y){let z=0;for(let k=0;k<20;k++){if(y&1)z^=x;y>>>=1;x<<=1;if(x&(1<<20))x^=poly;}check(y===0,'field bound');return z&lim;}
function fpow(x,k){let z=1;while(k>0){if(k%2)z=fmul(z,x);x=fmul(x,x);k=Math.floor(k/2);}return z;}
const t=2, r=fmul(fmul(t,t),t)^fmul(t,t);
const zero=[0,0],one=[1,0];
const sum=(x,y)=>[x[0]^y[0],x[1]^y[1]];
const prod=(x,y)=>{const ab=fmul(x[1],y[1]);return [fmul(x[0],y[0])^fmul(ab,r),fmul(x[0],y[1])^fmul(x[1],y[0])^ab];};
const eq=(x,y)=>JSON.stringify(x)===JSON.stringify(y);
function invert(x){check(!eq(x,zero),'inverse nonzero');let a=x[0],b=x[1],n=fmul(a,a)^fmul(a,b)^fmul(fmul(b,b),r);check(n!==0,'quadratic norm');let ni=fpow(n,(1<<20)-2),out=[fmul(a^b,ni),fmul(b,ni)];check(eq(prod(x,out),one),'inverse identity');return out;}
function negative(p){return p===null?null:[p[0],sum(p[1],one)];}
function lineadd(p,q){if(p===null)return q;if(q===null)return p;let [x,y]=p,[z,w]=q;
  if(eq(x,z)&&!eq(y,w))return null;
  // For doubling the tangent slope is x^2 because a3=1.
  let lam=eq(x,z)?prod(x,x):prod(sum(y,w),invert(sum(x,z)));
  let nu=sum(y,prod(lam,x));
  let x3=sum(sum(prod(lam,lam),one),sum(x,z));
  let y3=sum(sum(prod(lam,x3),nu),one);
  return [x3,y3];
}
function times(n,p){let z=null;while(n>0){if(n%2)z=lineadd(z,p);p=lineadd(p,p);n=Math.floor(n/2);}return z;}
function squarek(v,k){for(let i=0;i<k;i++)v=prod(v,v);return v;}
function frob(p,k){return p===null?null:[squarek(p[0],k),squarek(p[1],k)];}
function valid(p){if(p===null)return true;let [x,y]=p;return eq(sum(prod(y,y),y),sum(prod(prod(x,x),x),prod(x,x)));}
const baseCount=1+[0,1].flatMap(x=>[0,1].map(y=>(y*y+y-x*x*x-x*x)%2===0?1:0)).reduce((a,b)=>a+b,0);
check(baseCount===5,'F2 point count');
// Confirm the base polynomial and quadratic really define the advertised fields.
function rem(a,b){while(a!==0&&Math.floor(Math.log2(a))>=Math.floor(Math.log2(b)))a^=b<<Math.floor(Math.log2(a/b));return a;}
function gcd(a,b){while(b!==0)[a,b]=[b,rem(a,b)];return a;}
check(fpow(t,2**20)===t,'degree20 closure');for(const d of [4,10])check(gcd(fpow(t,2**d)^t,poly)===1,'base irreducibility');
let tr=0,v=r;for(let k=0;k<20;k++){tr^=v;v=fmul(v,v);}check(tr===1&&v===r,'quadratic trace');
const P=[[2,0],[0,1]];check(valid(P),'point on curve');check(times(11,P)===null,'exact order');
let pts=Array.from({length:10},(_,i)=>times(i+1,P));check(new Set(pts.map(JSON.stringify)).size===10,'subgroup distinct');
check(new Set(pts.map(v=>JSON.stringify(v[0]))).size===5,'five Kummer x classes');
for(let k=0;k<10;k++){
 const v=pts[k];check(valid(v),'valid multiple');
 check(eq(frob(v,4),times(7,v)),'fourth Frobenius');
 check(eq(frob(v,20),negative(v)),'twentieth Frobenius');
 check(eq(frob(v,40),v),'fortieth Frobenius');
 check(eq(frob(v,20)[0],v[0])&&!eq(frob(v,20)[1],v[1]),'x field and y obstruction');
 for(let d=1;d<20;d++)check(!eq(frob(v,d)[0],v[0]),'x exact degree20');
 check(new Set(Array.from({length:5},(_,j)=>JSON.stringify(frob(v,4*j)[0]))).size===5,'five-step Frobenius orbit');
 for(let d=1;d<40;d++)check(!eq(frob(v,d),v),'no proper subfield orientation');
}
for(let i=1;i<=10;i++)for(let j=1;j<=10;j++)check(eq(lineadd(pts[i-1],pts[j-1]),times((i+j)%11,P)),'all addition pairs');
// Derive Frobenius companion from #E(F2)=5, giving t=2+1-5=-2.
const ttrace=2+1-baseCount;
let A=[[0,(-2+11)%11],[1,(ttrace+11)%11]];
const mm=(a,b)=>a.map((row,i)=>row.map((_,j)=>((a[i][0]*b[0][j]+a[i][1]*b[1][j])%11)));
let ident=[[1,0],[0,1]],m=ident,counts=[];
for(let d=1;d<=40;d++){
 m=mm(m,A);let n=0;
 for(let x=0;x<11;x++)for(let y=0;y<11;y++)if(((m[0][0]-1)*x+m[0][1]*y)%11===0&&(m[1][0]*x+(m[1][1]-1)*y)%11===0)n++;
 counts.push(n);
}
check(counts.every((c,k)=>c===(k===39?121:1)),'torsion fixed-vector counts');
let a4=mm(mm(mm(A,A),A),A);check(eq(a4,[[7,0],[0,7]]),'scalar fourth Frobenius');
const o={base_curve_f2_points:baseCount,binary_oriented_field_degree:40,binary_x_field_degree:20,case:'ct115-frobenius-descent-obstruction',complete_dlp_cost_compared:false,fixed_vector_count_at_40:121,fixed_vector_counts_first_39:Array(39).fill(1),frobenius_20_scalar_mod_11:10,frobenius_4_scalar_mod_11:7,global_point_lift_computed:false,group_law_pairs_checked:100,hom_constant_z11_into_11_torsion_over_f2_20:1,hom_constant_z11_into_11_torsion_over_f2_40:121,hom_constant_z11_into_fixed_cyclic_target_over_f2_40:11,nonzero_hom_into_11_torsion_over_f2_40:120,nonzero_target_subgroup_checked:10,kummer_x_classes:5,kummer_x_orbit_length_under_frob4:5,kummer_x_residue_degree:20};
process.stdout.write(JSON.stringify(Object.fromEntries(Object.entries(o).sort(([a],[b])=>a.localeCompare(b))))+'\n');
