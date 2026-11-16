#!/usr/bin/env node
'use strict';
// CT116 independent verifier: BigInt carryless field reduction and
// separately implemented affine group law, x-class dictionary and BSGS.
const p=59, polynomial=0x130027, order=11;
const good=(x,msg)=>{if(!x)throw new Error(msg)};
const mod=n=>(n%p+p)%p;
const inv=n=>{n=mod(n);good(n!==0,'division by zero');for(let k=1;k<p;k++)if(mod(k*n)===1)return k;throw new Error('no inverse')};
const same=(P,Q)=>JSON.stringify(P)===JSON.stringify(Q);
const inverse=P=>P===null?null:[P[0],mod(-P[1]-1)];
const plus=(P,Q)=>{
  if(P===null)return Q;if(Q===null)return P;
  const [x,y]=P,[X,Y]=Q;
  if(x===X && mod(y+Y+1)===0)return null;
  let lambda;
  if(x===X){good(y===Y,'ramification');lambda=mod((3*x*x-2*x-10)*inv(2*y+1))}
  else lambda=mod((Y-y)*inv(X-x));
  const z=mod(lambda*lambda+1-x-X);
  return [z,mod(-1-y+lambda*(x-z))];
};
const multiply=(n,P)=>{let R=null,B=P;while(n){if(n%2)R=plus(R,B);B=plus(B,B);n=Math.floor(n/2)}return R};
const on=P=>P===null || mod(P[1]*P[1]+P[1]-P[0]**3+P[0]**2+10*P[0]+20)===0;
const P=[19,22];good(on(P)&&multiply(11,P)===null,'order check');

const polyMul=(a,b)=>{
  let v=0n;
  for(let i=0n;i<20n;i++)if((BigInt(b)>>i)&1n)v^=BigInt(a)<<i;
  for(let i=38n;i>=20n;i--)if((v & (1n<<i))!==0n)v ^= (BigInt(polynomial) << (i-20n));
  good(v>=0n && v<(1n<<20n),'binary reduction');
  return Number(v);
};
const pow16=x=>{for(let i=0;i<4;i++)x=polyMul(x,x);return x};
const target=[2];for(let j=1;j<5;j++)target.push(pow16(target[j-1]));
good(new Set(target).size===5 && pow16(target[4])===target[0],'target orbit');
const scalarByOrbit=[];let m=1;
for(let j=0;j<5;j++){scalarByOrbit.push(m);m=m*7%order}
good(scalarByOrbit.join(',')==='1,7,5,2,3','frobenius index');

const table=new Map();let accum=null;
for(let k=1;k<=5;k++){
  accum=plus(accum,P);good(accum!==null && on(accum),'source walk');
  good(!table.has(accum[0]),'duplicate source x');table.set(accum[0],k);
}
const record=[];
for(let secret=1;secret<=10;secret++){
  const Q=multiply(secret,P);good(on(Q),'public input');
  const small=table.get(Q[0]);good(small!==undefined,'no x class');
  let j=-1;
  for(let k=0;k<5;k++)if(Math.min(scalarByOrbit[k],11-scalarByOrbit[k])===small)j=k;
  good(j>=0,'no orbit index');
  const binary=target[j], recoverIndex=target.indexOf(binary);
  good(recoverIndex===j,'ambiguous target log');
  let recovered=scalarByOrbit[recoverIndex];
  if(!same(multiply(recovered,P),Q))recovered=(11-recovered)%11;
  good(same(multiply(recovered,P),Q)&&recovered===secret,'source sign failure');
  record.push({binary_x:binary,orbit_index:j,recovered,source_x:Q[0],source_y:Q[1]});
}
const bsgs=Q=>{
  const root=Math.floor(Math.sqrt(order))+1,babies=new Map();let R=null,ops=0;
  for(let j=0;j<root;j++){babies.set(JSON.stringify(R),j);R=plus(R,P);ops++}
  const minusStep=inverse(R);let test=Q;
  for(let i=0;i<=root;i++){
    const v=babies.get(JSON.stringify(test));
    if(v!==undefined){
      const k=(i*root+v)%order;
      good(same(multiply(k,P),Q),'BSGS');
      return [k,ops];
    }
    test=plus(test,minusStep);ops++;
  }
  throw new Error('BSGS miss');
};
const b=[];for(let k=1;k<11;k++)b.push(bsgs(multiply(k,P)));
good(b.every(([k],i)=>k===i+1),'BSGS incorrect');
const result={
 all_public_targets_recovered:record.length,
 binary_frobenius_orbit:target,
 binary_frobenius_orbit_entries:target.length,
 case:'ct116-frobenius-orbit-index-barrier',
 full_cold_cost_ratio_available:false,
 online_global_root_recognizer:false,
 orbit_scalar_representatives:scalarByOrbit,
 pcc:'unverified',
 public_cases:record,
 shortcut_inherent:false,
 source_bsgs_additions_max:Math.max(...b.map(x=>x[1])),
 source_bsgs_additions_min:Math.min(...b.map(x=>x[1])),
 source_bsgs_additions_sum:b.reduce((acc,[_,n])=>acc+n,0),
 source_x_precompute_additions:4,
 source_x_precomputed_entries:table.size,
 subgroup_order:order
};
const sorted=Object.fromEntries(Object.keys(result).sort().map(k=>[k,result[k]]));
process.stdout.write(JSON.stringify(sorted)+'\n');
