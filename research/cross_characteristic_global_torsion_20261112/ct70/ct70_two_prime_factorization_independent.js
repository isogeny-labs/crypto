#!/usr/bin/env node
/* CT70 independently factor small residue polynomials by explicit trial division;
   compare with direct nonzero-vector Frobenius permutation orbits. */
'use strict';
const H=[-101306819,89655,267096,349907,162543,-14817,-4813,108,27];
const mod=(x,p)=>((x%p)+p)%p;
function trim(a){a=a.slice();while(a.length&&a[a.length-1]===0)a.pop();return a;}
function norm(a,p){return trim(a.map(v=>mod(v,p)));}
function inverse(a,p){for(let b=1;b<p;b++)if(mod(a*b,p)===1)return b;throw Error('nonunit');}
function divide(a,b,p){a=norm(a,p);b=norm(b,p);if(!b.length)throw Error('division zero');
  let q=Array(Math.max(0,a.length-b.length+1)).fill(0),z=inverse(b[b.length-1],p);
  while(a.length>=b.length){const sh=a.length-b.length, t=mod(a[a.length-1]*z,p);q[sh]=t;
    for(let i=0;i<b.length;i++)a[sh+i]=mod(a[sh+i]-t*b[i],p);
    a=trim(a);
  }
  return [trim(q),a];
}
function factorAt29(){const p=29;let remaining=norm(H,p),degrees=[], factors=[];
  for(let a=0;a<p;a++){
    const f=[a,1];
    while(remaining.length>1){const [q,r]=divide(remaining,f,p);if(r.length)break;
      remaining=q;degrees.push(1);factors.push(f);
    }
  }
  for(let b=0;b<p;b++)for(let a=0;a<p;a++){
    const f=[a,b,1];
    while(remaining.length>2){const [q,r]=divide(remaining,f,p);if(r.length)break;
      remaining=q;degrees.push(2);factors.push(f);
    }
  }
  if(remaining.length!==1 || degrees.length!==5)throw Error('unexpected residue factor degrees');
  // The full polynomial factors into five distinct irreducibles, so is squarefree.
  if(new Set(factors.map(f=>f.join(','))).size!==factors.length)throw Error('repeated factors');
  return degrees.sort((a,b)=>a-b);
}
function factorAt2(){const p=2,f=norm(H,p);
  for(let d=1;d<=4;d++)for(let m=0;m<(1<<d);m++){
    const candidate=Array.from({length:d},(_,i)=>(m>>i)&1).concat([1]);
    if(divide(f,candidate,p)[1].length===0)throw Error('mod 2 reducible');
  }
  // No factor of degree at most four -> irreducible degree eight.
  if(f.length!==9)throw Error('degree mismatch');
  return [8];
}
function orbitLengths(m,p){let seen=new Set(),out=[];
  const key=v=>v.join(',');
  const act=v=>[mod(m[0][0]*v[0]+m[0][1]*v[1],p),mod(m[1][0]*v[0]+m[1][1]*v[1],p)];
  for(let a=0;a<p;a++)for(let b=0;b<p;b++){
    if(a===0&&b===0)continue;const v=[a,b];if(seen.has(key(v)))continue;
    let w=v,len=0;
    while(!seen.has(key(w))){seen.add(key(w));len++;w=act(w);}
    if(key(w)!==key(v))throw Error('broken cycle');out.push(len);
  }
  return out.sort((a,b)=>a-b);
}
function gcdDegrees(degrees){return Object.fromEntries([1,2,4,8].map(n=>[n,degrees.reduce((a,d)=>a+(n%d===0?d:0),0)]));}
function main(){let a=factorAt29(),b=factorAt2();const ca=orbitLengths([[1,0],[0,2]],3),cb=orbitLengths([[0,1],[1,1]],3);
  if(JSON.stringify(a)!==JSON.stringify(ca)||JSON.stringify(b)!==JSON.stringify(cb))throw Error('factorization vs Frobenius');
  const data={ell:3,field_degree:8,source_prime:29,target_prime:2,source_trace:0,target_trace:-2,
    source_factor_degrees:a,target_factor_degrees:b,source_frobenius_cycles:ca,target_frobenius_cycles:cb,
    source_gcd_degrees:gcdDegrees(a),target_gcd_degrees:gcdDegrees(b),both_squarefree:true,
    global_lift_algorithm:false,scalable_ecdlp:false};
  // Python uses sort_keys=True, separator without spaces.
  const sorted=Object.fromEntries(Object.keys(data).sort().map(k=>[k,data[k]]));
  process.stdout.write(JSON.stringify(sorted)+'\n');
}
main();
