'use strict';
// CT343 independent-language *destination-only* group-law cross-check.
// Does not import CT339/CT343 and cannot independently validate the Cantor map.
const fs=require('node:fs');
const source=JSON.parse(fs.readFileSync(process.argv[2] || '/mnt/data/CT343_RESULT.json','utf8'));
const p=103; const mod=x=>((x%p)+p)%p;
function pow(x,n){let z=1;x=mod(x);for(;n;n=Math.floor(n/2),x=mod(x*x))if(n%2)z=mod(z*x);return z;}
const inv=x=>{if(mod(x)===0)throw Error('zero inversion');return pow(x,p-2)};
function mulpoly(a,b){const c=new Array(a.length+b.length-1).fill(0);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)c[i+j]=mod(c[i+j]+a[i]*b[j]);return c;}
const v=[1,9,25].map(a=>mod(a*inv(4)));
function curve(mode){return v.reduce((f,x)=>mulpoly(f,mode==='plus'?[mod(-x),1]:[1,mod(-x)]),[1]);}
const curves=[curve('plus'),curve('minus')];
function peval(f,x){return f.reduceRight((v,c)=>mod(v*x+c),0);}
function parsePair(A){if(A===null)return null;for(const x of A)if(x[1]!==0)throw Error('nonbase coordinate');return A.map(x=>x[0]);}
const eq=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
function eadd(f,A,B){if(!A)return B;if(!B)return A;let d;
 if(A[0]===B[0]){if(mod(A[1]+B[1])===0)return null;if(A[1]!==B[1])throw Error('illegal same-x');d=mod((3*f[3]*A[0]*A[0]+2*f[2]*A[0]+f[1])*inv(2*A[1]));}
 else d=mod((B[1]-A[1])*inv(B[0]-A[0]));
 const x=mod((d*d-f[2])*inv(f[3])-A[0]-B[0]),y=mod(d*(A[0]-x)-A[1]);
 if(mod(y*y)!==peval(f,x))throw Error('off-curve addition');return [x,y];}
function emul(f,A,n){let R=null;while(n){if(n%2)R=eadd(f,R,A);n=Math.floor(n/2);if(n)A=eadd(f,A,A);}return R;}
let checks=0,checksCurve=0;
for(const row of source.cohorts){const V=row.target_generator.map(parsePair);for(let i=0;i<2;i++){if(V[i]){if(mod(V[i][1]**2)!==peval(curves[i],V[i][0]))throw Error('generator off curve');checksCurve++;}}
 const mo=row.mapped_order;if(!eq(V.map((x,i)=>emul(curves[i],x,mo)),[null,null]))throw Error('order upper fails');
 for(const d of (mo===23?[23]:[5,23]))if(eq(V.map((x,i)=>emul(curves[i],x,mo/d)),[null,null]))throw Error('order lower fails');
 for(let k=0;k<row.order;k++){
  const a=V.map((x,i)=>emul(curves[i],x,k)),b=V.map((x,i)=>emul(curves[i],x,k%mo));
  if(!eq(a,b))throw Error('mapped cyclic repeat fails at '+k);
  if(!eq(a,V.map((x,i)=>emul(curves[i],x,k+mo))))throw Error('mapped double lift mismatch');checks++;
 }
}
console.log(JSON.stringify({schema:'ct343-target-crosscheck/v1',status:'passed',source_schema:source.schema,independent_target_scalar_checks:checks,curve_point_checks:checksCurve,target_orders:source.cohorts.map(x=>x.mapped_order)}));
