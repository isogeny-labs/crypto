// Deterministic non-elliptic M31 projection-certificate check. Node.js, no PRNG.
const p=2147483647n, mod=x=>(x%p+p)%p;
function pow(a,e){let r=1n;for(;e;e>>=1n,a=mod(a*a))if(e&1n)r=mod(r*a);return r}
const pts=Array.from({length:9},(_,i)=>[BigInt(i%3),BigInt(Math.floor(i/3))]);
let checks=0,subset=0,exact=0,scalarFalse=0,shapeFalse=0,colliding=0,sepCounts=[0,0,0,0];
for(let S=0;S<512;S++){
  const s=pts.filter((_,i)=>S>>i&1);let c=0n,labels;
  for(;c<=3n;c++){labels=s.map(([x,y])=>x+c*y);if(new Set(labels).size===s.length)break}
  if(c>3n)throw Error("separation");sepCounts[Number(c)]++;
  const d=labels.map((a,i)=>pow(mod(labels.reduce((v,b,j)=>j===i?v:v*(a-b),1n)),p-2n));
  let shapeMask=0,scalarMask=0;
  for(let j=0;j<9;j++){
    const [x,y]=pts[j],t=x+c*y;
    const q=mod(labels.reduce((v,a)=>v*(t-a),1n));
    const g=mod(labels.reduce((v,a,i)=>v+s[i][1]*d[i]*mod(labels.reduce((w,b,k)=>k===i?w:w*(t-b),1n)),0n));
    if(q===0n)scalarMask|=1<<j;
    if(q===0n&&mod(y-g)===0n)shapeMask|=1<<j;
    checks++;
  }
  if(shapeMask!==S)throw Error("point equality");
  if(scalarMask!==S)colliding++;
  for(let T=0;T<512;T++){
    const ok=(T&~shapeMask)===0,truth=(T&~S)===0;
    if(ok!==truth)shapeFalse++;
    if((T&~scalarMask)===0&&!truth)scalarFalse++;
    subset++;
    if((ok&&(S&~T)===0)!==(T===S))throw Error("support equality");
    exact++;
  }
}
const result={checks,subset,exact,scalarFalse,shapeFalse,colliding,sepCounts};
if(JSON.stringify(result)!==JSON.stringify({checks:4608,subset:262144,exact:262144,scalarFalse:42598,shapeFalse:0,colliding:315,sepCounts:[64,98,154,196]}))throw Error("counter mismatch");
console.log(JSON.stringify(result));
