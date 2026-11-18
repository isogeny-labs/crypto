'use strict';
// CT92 independent modular matrix rank and fixed-vector enumeration.
const mod = (x, n) => (x % n + n) % n;
const gcd=(a,b)=>b===0?a:gcd(b,a%b);
function count(ell) {
 let group=0, eligible=0, total=0;
 const histogram=new Map([[0,0],[ell-1,0],[ell*ell-1,0]]);
 for(let a=0;a<ell;a++) for(let b=0;b<ell;b++)
  for(let c=0;c<ell;c++) for(let d=0;d<ell;d++) {
   if(mod(a*d-b*c,ell)===0)continue;
   group++;
   // Independent count via rank of A-I rather than vector enumeration.
   let fixed=0;
   if(a===1&&b===0&&c===0&&d===1)fixed=ell*ell-1;
   else if(mod((a-1)*(d-1)-b*c,ell)===0)fixed=ell-1;
   if(fixed)eligible++;
   total+=fixed;
   if(!histogram.has(fixed))throw Error('unexpected rank');
   histogram.set(fixed,histogram.get(fixed)+1);
  }
 if(group!==ell*(ell-1)*(ell-1)*(ell+1)||eligible!==ell*(ell*ell-2)||total!==group)throw Error('Burnside');
 if(histogram.get(ell*ell-1)!==1||histogram.get(ell-1)!==eligible-1)throw Error('fixed histogram');
 const num=eligible,den=group,g=gcd(num,den);
 return {ell,gl2_order:group,matrices_with_eigenvalue_one:eligible,
  degree_one_prime_counts:[...histogram].sort((x,y)=>x[0]-y[0]),
  chebotarev_density:`${num/g}/${den/g}`,average_degree_one_places:1,
  oriented_field_degree_if_transitive:ell*ell-1,
  x_field_degree_if_transitive:(ell*ell-1)/2,
  dense_monic_x_polynomial_entries:(ell*ell+1)/2,
  source_subgroup_full_lookup_entries:ell};
}
function curveOrder(p) {
 let n=1;
 for(let x=0;x<p;x++)for(let y=0;y<p;y++)
  if(mod(y*y+y-x*x*x+x*x+10*x+20,p)===0)n++;
 return n;
}
// Independent permutation cycles: Frobenius on F7² and on sign classes.
function orbitTypes(A, sign) {
 const norm = v => {
  if(!sign)return v;
  const w=v.map(z=>mod(-z,7));
  return (v[0]<w[0]||(v[0]===w[0]&&v[1]<=w[1]))?v:w;
 };
 const key=v=>v[0]+','+v[1];
 const step=v=>[mod(A[0][0]*v[0]+A[0][1]*v[1],7),mod(A[1][0]*v[0]+A[1][1]*v[1],7)];
 let remaining=new Set();
 for(let a=0;a<7;a++)for(let b=0;b<7;b++)if(a!==0||b!==0)remaining.add(key(norm([a,b])));
 let degrees=new Map();
 while(remaining.size){
  const first=[...remaining][0];let cur=first,len=0;
  do {
   if(!remaining.delete(cur))throw Error('Frobenius nonpermutation');
   len++;cur=key(norm(step(cur.split(',').map(Number))));
  } while(cur!==first);
  degrees.set(len,(degrees.get(len)||0)+1);
 }
 return [...degrees].sort((a,b)=>a[0]-b[0]);
}
const source=curveOrder(37),target=curveOrder(2);
if(source!==35||target!==5||(38-source)!==3)throw Error('curve count');
let roots=[];for(let a=0;a<7;a++)if(mod(a*a-3*a+2,7)===0)roots.push(a);
if(JSON.stringify(roots)!=='[1,2]')throw Error('Frobenius roots');
const sourceFrob=[[1,0],[0,2]],binaryFrob=[[0,5],[1,5]];
const so=orbitTypes(sourceFrob,false),sx=orbitTypes(sourceFrob,true);
const bo=orbitTypes(binaryFrob,false),bx=orbitTypes(binaryFrob,true);
if(JSON.stringify(so)!=='[[1,6],[3,14]]'||JSON.stringify(sx)!=='[[1,3],[3,7]]'||
   JSON.stringify(bo)!=='[[24,2]]'||JSON.stringify(bx)!=='[[12,2]]')throw Error('splitting type');
let result={source_oriented_residue_degrees:so,source_x_residue_degrees:sx,
 binary_oriented_residue_degrees:bo,binary_x_residue_degrees:bx,
 experiment:'ct92-degree-one-place-density',
  galois_image_hypothesis:'full_GL2_mod_ell',
  source_prime:37,source_curve_order:source,source_trace:3,
  source_eigenvalues_mod_7:roots,source_degree_one_primes:6,
  binary_curve_order:target,ell_cases:[3,5,7,11].map(count)};
function canonical(x) {
 if(Array.isArray(x))return x.map(canonical);
 if(x!==null&&typeof x==='object')return Object.fromEntries(Object.keys(x).sort().map(k=>[k,canonical(x[k])]));
 return x;
}
console.log(JSON.stringify(canonical(result)));
