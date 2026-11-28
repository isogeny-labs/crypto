#!/usr/bin/env node
'use strict';
// Independent CT114: modular polynomial / oriented double-cover algebra, full 10-place CRT.
const P=59;
function verify(v,msg){if(!v)throw new Error(msg)}
function tidy(a){while(a.length>1&&!a[a.length-1])a.pop();return a}
function norm(v,m){return (v%m+m)%m}
function sum(a,b,m,sign=1){let c=new Array(Math.max(a.length,b.length)).fill(0);for(let i=0;i<c.length;i++)c[i]=norm((a[i]||0)+sign*(b[i]||0),m);return tidy(c)}
function times(a,b,m){let c=new Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)c[i+j]=(c[i+j]+a[i]*b[j])%m;return tidy(c)}
function power(a,n,m){let out=[1];while(n){if(n%2)out=times(out,a,m);a=times(a,a,m);n=Math.floor(n/2)}return out}
function inv(a,m){a=norm(a,m);verify(a!==0,'inverse zero');let [r,s]=[m,a],[u,v]=[0,1];while(s){let q=Math.floor(r/s);[r,s]=[s,r-q*s];[u,v]=[v,u-q*v]}verify(r===1,'not invertible');return norm(u,m)}
function psiel(m){
 let d=[-79,-40,-4,4],a=[-21,-237,-60,-4,3],b=[-5821,-1496,-210,-790,-100,-4,2];
 let d2=power(d,2,m),a3=power(a,3,m),b2=power(b,2,m),b3=power(b,3,m);
 let c=sum(times(d2,b,m),a3,m,-1);
 let d6=times(a,sum(c,b2,m,-1),m);
 let d7=sum(times(c,a3,m),times(d2,b3,m),m,-1);
 return sum(times(d7,power(c,3,m),m),times(times(d2,b,m),power(d6,3,m),m),m,-1)
}
const polyEval=(f,x,m)=>f.reduceRight((v,c)=>(v*x+c)%m,0);
function remainder(a,F,m){
 a=a.map(z=>norm(z,m));const d=F.length-1,fInv=inv(F[d],m);
 for(let i=a.length-1;i>=d;i--){const q=a[i]*fInv%m;if(!q)continue;
   for(let j=0;j<=d;j++)a[i-d+j]=norm(a[i-d+j]-q*F[j],m)}
 return tidy(a.slice(0,d).length?a.slice(0,d):[0])
}
function gcdMonic(a,b,p){
 while(!(b.length===1&&b[0]===0)){
   let [old,newB]=[b,remainder(a,b,p)];a=old;b=newB
 }
 const z=inv(a[a.length-1],p);return tidy(a.map(v=>v*z%p));
}
class Algebra {
 constructor(modulus){this.m=modulus;this.f=psiel(modulus);verify(this.f.length===61&&this.f[60]===11,'psi order')}
 plus(a,b){return [sum(a[0],b[0],this.m),sum(a[1],b[1],this.m)]}
 minus(a,b){return [sum(a[0],b[0],this.m,-1),sum(a[1],b[1],this.m,-1)]}
 product(a,b){
   const m=this.m, ab=times(a[1],b[1],m);
   // Y^2 = X^3 -X^2 -10X -20 -Y
   const c=sum(times(a[0],b[0],m),times(ab,[-20,-10,-1,1],m),m);
   const y=sum(sum(times(a[0],b[1],m),times(a[1],b[0],m),m),ab,m,-1);
   return [remainder(c,this.f,m),remainder(y,this.f,m)]
 }
 zero(){return [[0],[0]]}
 one(){return [[1],[0]]}
 equal(a,b){return this.minus(a,b).every(c=>remainder(c,this.f,this.m).length===1&&remainder(c,this.f,this.m)[0]===0)}
 projector(x,y){
   const p=P;
   verify(Number.isInteger(x)&&x>=0&&x<p&&Number.isInteger(y)&&y>=0&&y<p,'bad encoding');
   verify(polyEval(this.f,x,p)===0,'not torsion root');
   verify(norm(y*y+y-x*x*x+x*x+10*x+20,p)===0,'not curve point');
   let quot=new Array(60).fill(0);quot[59]=this.f[60]%p;
   for(let i=58;i>=0;i--)quot[i]=norm(this.f[i+1]+x*quot[i+1],p);
   verify(norm(this.f[0]+x*quot[0],p)===0,'root quotient');
   let der=0;
   for(let i=60;i>=1;i--)der=norm(der*x+i*this.f[i],p);
   verify(der!==0&&norm(2*y+1,p)!==0,'ramified');
   const scale=inv(norm((2*y+1)*der,p),p);
   return [tidy(quot.map(a=>a*(y+1)*scale%p)),tidy(quot.map(a=>a*scale%p))]
 }
 newton(e){const s=this.product(e,e),t=this.product(s,e);
   return this.minus(this.plus(this.plus(s,s),s),this.plus(t,t))}
}
const A=new Algebra(59),B=new Algebra(3481);
verify(JSON.stringify(gcdMonic(A.f,A.f.slice(1).map((x,i)=>x*(i+1)%P),P))==='[1]',
 'division polynomial repeated mod59');
verify(JSON.stringify(gcdMonic(A.f,[-79,-40,-4,4].map(x=>norm(x,P)),P))==='[1]',
 'ordinate branch at eleven torsion');
const roots=Array.from({length:59},(_,x)=>x).filter(x=>polyEval(A.f,x,P)===0);
verify(JSON.stringify(roots)==='[19,20,22,43,46]','bad linear roots');
const pts=[];
for(const x of roots)for(let y=0;y<P;y++)if(norm(y*y+y-x*x*x+x*x+10*x+20,P)===0)pts.push([x,y]);
verify(pts.length===10,'bad oriented rational places');
let ids=[],lifts=[];
for(const [x,y] of pts){
 const e=A.projector(x,y);verify(A.equal(A.product(e,e),e),'idempotent mod59');
 const h=B.newton(e);verify(B.equal(B.product(h,h),h),'idempotent mod59^2');
 verify(h.every((v,i)=>JSON.stringify(v.map(x=>x%P))===JSON.stringify(e[i].map(x=>x%P))),'wrong lift residue');
 for(const [a,b] of pts){let got=(polyEval(e[0],a,P)+polyEval(e[1],a,P)*b)%P;
   verify(got===(a===x&&b===y?1:0),'wrong oriented point selector')}
 ids.push(e);lifts.push(h)
}
for(let i=0;i<10;i++)for(let j=0;j<i;j++){
 verify(A.equal(A.product(ids[i],ids[j]),A.zero()),'bad orthogonality mod59');
 verify(B.equal(B.product(lifts[i],lifts[j]),B.zero()),'bad orthogonality mod59^2')
}
let part=A.zero();for(const e of ids)part=A.plus(part,e);
verify(A.equal(A.product(part,part),part)&&!A.equal(part,A.one()),'bad degree-one summand');
for(const [x,y] of [[19,23],[0,0],[-1,0],[19,59]]){
 let refused=false;try{A.projector(x,y)}catch(e){refused=true}verify(refused,'invalid point admitted')}
verify((3481%2)===1&&!A.equal(ids[0],A.one())&&!A.equal(ids[0],A.zero()),'prime-switch control');
const out={case:'ct114-source-etale-crt-idempotents',source_modulus:59,source_order:11,
 source_linear_x_roots:roots,source_oriented_points:10,primitive_idempotents_mod59:10,
 lifted_idempotents_mod3481:10,pairwise_orthogonality:true,source_degree_one_summand_proper:true,
 two_to_fiftynine_squared_ambiguity:true,global_idempotent_in_K:false,
 fixed_binary_target_value_selected:false,online_scalar_used:false,full_cold_cost_measured:false};
process.stdout.write(JSON.stringify(Object.fromEntries(Object.entries(out).sort((a,b)=>a[0].localeCompare(b[0]))))+'\n');
