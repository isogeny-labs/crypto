'use strict';
// CT73 independent oracle. Field operations use carryless convolution and
// explicit polynomial reduction, rather than a shift-register multiplier.
const check=(ok,msg)=>{if(!ok)throw new Error(msg)};
function gf(a,b){
 let raw=0;
 for(let j=0;j<8;j++)for(let i=0;i<8;i++)if((a&(1<<i))&&(b&(1<<j)))raw^=1<<(i+j);
 for(let k=14;k>=8;k--)if(raw&(1<<k))raw^=0x17b<<(k-8);
 return raw;
}
const square=(a)=>gf(a,a);
const p0=[19,2];
const orbit=[p0];
for(let i=1;i<8;i++)orbit.push([square(orbit[i-1][0]),square(orbit[i-1][1])]);
check(JSON.stringify([square(orbit[7][0]),square(orbit[7][1])])===JSON.stringify(p0),'orbit end');
check(new Set(orbit.map(x=>x.join(','))).size===8,'orbit distinct');
for(let i=0;i<8;i++){
 const [x,y]=orbit[i];
 check((gf(y,y)^y)===(gf(gf(x,x),x)^gf(x,x)),'curve');
 check(orbit[(i+4)%8][0]===x && orbit[(i+4)%8][1]===(y^1),'frob inverse');
}
let ok=0,bad=0,off=0;
for(const [i,[x,y]] of orbit.entries())for(const [j,[xx,yy]] of orbit.entries()){
 const Q=[xx,yy^1];
 if(Q[0]===x && Q[1]===(y^1))ok++;
 else if(Q[0]===x && Q[1]===y)bad++;
 else off++;
}
check(ok===8&&bad===8&&off===48,'place mismatch counts');
const rows=[{case:'ct69_fixed_order3',target_field:256,base_point:p0,orientations:8,
 paired_choices:64,same_place_valid:8,frobenius_halfturn_equals_negation:true,
 independent_correct:ok,independent_wrong_scalar:bad,independent_outside_subgroup:off}];
const image=(M,v,q)=>[(M[0]*v[0]+M[1]*v[1])%q,(M[2]*v[0]+M[3]*v[1])%q];
for(const q of [3,5,7,11]){
 let count=0;
 const im=new Set();
 for(let a=0;a<q;a++)for(let b=0;b<q;b++)for(let c=0;c<q;c++)for(let d=0;d<q;d++){
  if(((a*d-b*c)%q+q)%q===0)continue;
  count++;
  const g=[a,b,c,d], gp=image(g,[1,0],q);
  im.add(gp.join(','));
  // Randomness is not used: test every possible scalar x for every g.
  for(let x=0;x<q;x++){
    const lhs=image(g,[x,0],q),rhs=[x*gp[0]%q,x*gp[1]%q];
    check(lhs[0]===rhs[0]&&lhs[1]===rhs[1],'shared map');
  }
 }
 const n=q*q-1;
 check(count===n*(q*q-q)&&im.size===n,'GL2 orbit');
 const vectors=[...im].map(t=>t.split(',').map(Number));
 let correct=0;
 for(const u of vectors)for(const v of vectors)if(v[0]===2*u[0]%q&&v[1]===2*u[1]%q)correct++;
 check(correct===n,'independent places');
 rows.push({case:'abstract_full_image',ell:q,group_size:count,nonzero_vectors:n,
  shared_maps_checked:count,independent_pairs:n*n,independent_correct:correct,probability_denominator:n});
}
for(const r of rows){
 const sort=v=>Array.isArray(v)?v.map(sort):v&&typeof v==='object'?Object.fromEntries(Object.keys(v).sort().map(k=>[k,sort(v[k])])):v;
 process.stdout.write(JSON.stringify(sort(r))+'\n');
}
