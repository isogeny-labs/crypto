'use strict';
// Independent Newton-trace and rank/Frobenius enumerator; no Python imports.
function assert(x, why) { if (!x) throw Error(why); }
const mod = (x, n) => ((x % n) + n) % n;
function multiply(A, B, n) {
  return [mod(A[0]*B[0]+A[1]*B[2], n),mod(A[0]*B[1]+A[1]*B[3],n),
          mod(A[2]*B[0]+A[3]*B[2],n),mod(A[2]*B[1]+A[3]*B[3],n)];
}
function matrixPower(A, d, n) {
  let R=[1,0,0,1];
  for (; d>0;d=Math.floor(d/2)) {if(d%2)R=multiply(R,A,n);A=multiply(A,A,n);}
  return R;
}
function fixed(A, n) {
  // A-I rank, giving n^nullity fixed vectors (including identity).
  const B = [mod(A[0]-1,n), A[1], A[2], mod(A[3]-1,n)];
  let rank;
  if (mod(B[0]*B[3]-B[1]*B[2],n)!==0) rank=2;
  else rank = B.some(x=>x!==0) ? 1 : 0;
  return n**(2-rank)-1;
}
function minimumExtension(ell) {
  const A=[0,mod(-2,ell),1,mod(-2,ell)];
  // Independent computation from the exact trace recurrence and #E(F2^d).
  let sPrev=2,sCur=-2;
  for(let d=1;d<=ell*ell;d++) {
    if(d>1){const next=mod(-2*sCur-2*sPrev,ell);sPrev=sCur;sCur=next;}
    // Exact modular exponent, no floating point beyond the exponent counter.
    let qPower=1;for(let j=0;j<d;j++)qPower=mod(qPower*2,ell);
    const points=mod(qPower+1-sCur,ell);
    assert(points === mod((A=>{const B=matrixPower(A,d,ell);
      return mod((B[0]-1)*(B[3]-1)-B[1]*B[2],ell);})(A),ell), 'trace vs matrix');
    if(points===0)return d;
  }
  throw Error('missing extension');
}
const points=1+[0,1].flatMap(x=>[0,1].map(y=>mod(y*y+y-x*x*x+x*x+10*x+20,2)===0?1:0))
  .reduce((a,b)=>a+b,0);
assert(points===5,'count F2');
const ellList=[3,5,7,11,13,17,19,23];
const extension=ellList.map(ell=>[ell,minimumExtension(ell)]);
assert(JSON.stringify(extension)===JSON.stringify([[3,8],[5,1],[7,24],[11,40],[13,6],[17,16],[19,72],[23,88]]),'extension table');
const ell=7,A=[0,5,1,5];
const powers={};const fixedCounts=[];
for(let d=1;d<=24;d++){
 const M=matrixPower(A,d,ell);
 if([4,12,24].includes(d))powers[String(d)]=[[M[0],M[1]],[M[2],M[3]]];
 fixedCounts.push(fixed(M,ell));
}
assert(fixedCounts.slice(0,23).every(x=>x===0)&&fixedCounts[23]===48,'fixed counts');
const seen=new Set(), lengths=[];
for(let x=0;x<ell;x++)for(let y=0;y<ell;y++){
 if(!x&&!y)continue;
 let id=x+','+y;if(seen.has(id))continue;
 let v=[x,y],len=0;
 while(!seen.has(v[0]+','+v[1])) {
  seen.add(v[0]+','+v[1]);len++;
  v=[mod(A[0]*v[0]+A[1]*v[1],ell),mod(A[2]*v[0]+A[3]*v[1],ell)];
 }
 assert(v[0]===x&&v[1]===y,'not a cycle');lengths.push(len);
}
lengths.sort((a,b)=>a-b);
assert(JSON.stringify(lengths)==='[24,24]','orbits');
const result={
 curve_F2_points:points,frobenius_trace:3-points,
 ell7:{extension_degree:24,fixed_nonzero_d1_to_d24:fixedCounts,
        frobenius_powers:powers,nonzero_orbit_lengths:lengths,
        equivariant_nonzero_maps_F2:0,equivariant_nonzero_maps_F2_24:48},
 minimum_extension_by_ell:extension};
function stable(x){if(Array.isArray(x))return '['+x.map(stable).join(',')+']';
 if(x!==null&&typeof x==='object')return '{'+Object.keys(x).sort().map(k=>JSON.stringify(k)+':'+stable(x[k])).join(',')+'}';
 return JSON.stringify(x);}
console.log(stable(result));
