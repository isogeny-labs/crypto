'use strict';
// CT72 independent: parameterize all irreducible trace/determinant matrices
// directly; never enumerate GL2 conjugates or use matrix inverses.
const primes=[3,5,7,11];
const mod=(a,p)=>((a%p)+p)%p;
const inv=(n,p)=>{for(let i=1;i<p;i++)if(mod(i*n,p)===1)return i;throw Error('nonunit')};
const sq=(a,p)=>{for(let i=0;i<p;i++)if(mod(i*i,p)===mod(a,p))return true;return false};
const same=(a,b)=>a.length===b.length&&a.every((v,i)=>v===b[i]);
const mul=(a,b,p)=>[mod(a[0]*b[0]+a[1]*b[2],p),mod(a[0]*b[1]+a[1]*b[3],p),mod(a[2]*b[0]+a[3]*b[2],p),mod(a[2]*b[1]+a[3]*b[3],p)];
const matrices=(p)=>{let g=[];for(let a=0;a<p;a++)for(let b=0;b<p;b++)for(let c=0;c<p;c++)for(let d=0;d<p;d++)if(mod(a*d-b*c,p)!==0)g.push([a,b,c,d]);return g};
function run(p){
  const tr=1;
  let det=0;
  for(let d=1;d<p;d++)if(!sq(1-4*d,p)){det=d;break}
  if(!det)throw Error('nonsplit det not found');
  const data=[];
  for(let u=0;u<p;u++)for(let v=1;v<p;v++){
    // det=u*(tr-u)-v*w
    const w=mod((u*(tr-u)-det)*inv(v,p),p);
    if(!w)throw Error('irreducible requires nonzero off diagonal');
    if(mod(2*u-tr,p)!==mod(u-mod(tr-u,p),p))throw Error('mixed trace');
    let a=[u,v,w,mod(tr-u,p)];
    if(mod(a[0]+a[3],p)!==tr||mod(a[0]*a[3]-a[1]*a[2],p)!==det)throw Error('charpoly');
    data.push(a);
  }
  const g=matrices(p),T=[0,mod(-det,p),1,tr],S=[1,0,0,p-1];
  const Cs=g.filter(M=>same(mul(M,S,p),mul(S,M,p)));
  const Ct=g.filter(M=>same(mul(M,T,p),mul(T,M,p)));
  if(g.length!==p*(p-1)**2*(p+1)||Cs.length!==(p-1)**2||Ct.length!==p*p-1)throw Error('centralizer');
  const seen=new Set();
  for(let u=0;u<p;u++){
    const arr=data.filter(a=>a[0]===u);
    if(arr.length!==p-1)throw Error('orbit size');
    const base=arr[0];
    const images=new Set();
    for(let z=1;z<p;z++){
      const transported=[base[0],mod(z*base[1],p),mod(base[2]*inv(z,p),p),base[3]];
      images.add(transported.join(','));
    }
    if(images.size!==p-1||!arr.every(a=>images.has(a.join(','))))throw Error('diagonal orbit');
    if(Cs.filter(D=>same(mul(D,base,p),mul(base,D,p))).length!==p-1)throw Error('scalar intersection');
    for(const a of arr)seen.add(a.join(','));
  }
  if(seen.size!==p*(p-1))throw Error('coverage');
  return {ell:p,trace:tr,det:det,gl2_size:g.length,split_centralizer:Cs.length,
    nonsplit_centralizer:Ct.length,scalar_intersection:p-1,relative_orbits:p,
    matrices_per_orbit:p-1,conjugates:data.length,
    invariant_u:Array.from({length:p},(_,i)=>i)};
}
for(const p of primes)console.log(JSON.stringify(Object.fromEntries(Object.entries(run(p)).sort(([a],[b])=>a.localeCompare(b)))));
