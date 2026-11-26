// CT335 frozen source. Node.js v22 or later; exact rational Fp algebra, not solver/PCC.
// Input: CT334_RESULT.json Git blob 01be355bf299c01419ac3a43a53adc4f8f7d61ea.
const study = function study() {
  const P=2147483647n;
  const mod=n=>((n%P)+P)%P, norm=a=>{let b=a.map(mod); while(b.length && b[b.length-1]===0n)b.pop();return b;};
  const sum=(a,b,s=1n)=>norm(Array.from({length:Math.max(a.length,b.length)},(_,i)=>(a[i]||0n)+s*(b[i]||0n)));
  const mul=(a,b)=>{if(!a.length||!b.length)return [];const c=Array(a.length+b.length-1).fill(0n);for(let i=0;i<a.length;i++)for(let j=0;j<b.length;j++)c[i+j]=mod(c[i+j]+a[i]*b[j]);return norm(c)};
  const prod=arr=>arr.reduce((a,b)=>mul(a,b),[1n]);
  const deriv=a=>norm(a.slice(1).map((x,i)=>x*BigInt(i+1)));
  const pow=(x,n)=>{let a=mod(x),r=1n;while(n){if(n&1n)r=mod(r*a);a=mod(a*a);n>>=1n;}return r;};
  const inv=x=>{if(!mod(x))throw Error("zero inverse");return pow(x,P-2n)};
  const rem=(a,b)=>{a=norm(a);b=norm(b);if(!b.length)throw Error("zero polynomial");let t=inv(b[b.length-1]);while(a.length>=b.length){const d=a.length-b.length,v=mod(a[a.length-1]*t);for(let i=0;i<b.length;i++)a[d+i]=mod(a[d+i]-v*b[i]);a=norm(a)}return a};
  const gcd=(a,b)=>{while(b.length){const r=rem(a,b);a=b;b=r;}return a.length?norm(a.map(x=>x*inv(a[a.length-1]))):[]};
  const determinant=g=>{let x=g.map(a=>Array.from({length:3},(_,i)=>a[i]||0n));let [a,b,c]=x;return mod(a[0]*(b[1]*c[2]-b[2]*c[1])-a[1]*(b[0]*c[2]-b[2]*c[0])+a[2]*(b[0]*c[1]-b[1]*c[0]))};
  const bracket=(a,b)=>sum(mul(deriv(a),b),mul(a,deriv(b)),-1n);
  const map=g=>{const d=determinant(g);if(d===0n)return {d,h:null,H:null};const v=inv(d),h=[0,1,2].map(i=>norm(bracket(g[(i+1)%3],g[(i+2)%3]).map(x=>x*v)));return {d,h,H:prod(h)};};
  function matchings(xs){if(!xs.length)return [[]];let result=[];for(let i=1;i<xs.length;i++){let rest=xs.slice(1).filter(y=>y!==xs[i]);for(let tail of matchings(rest))result.push([[xs[0],xs[i]],...tail]);}return result;}
  const nums=a=>a.map(v=>Number(v)),roots=[1,2,3,4,5,6];
  const source=prod(roots.map(r=>[mod(-BigInt(r)),1n]));
  if(JSON.stringify(nums(source))!==JSON.stringify([720,2147481883,1624,2147482912,175,2147483626,1]))throw Error("CT334 source mismatch");
  let rows=[];
  for(const pairs of matchings(roots)){
    const g=pairs.map(pair=>prod(pair.map(r=>[mod(-BigInt(r)),1n])));
    const {d,h,H}=map(g);
    const row={pairs,g:g.map(nums),delta:Number(d)};
    if(d===0n){row.type="decomposable_quotient";row.h=null;row.H=null;row.smooth=null;row.dual=null;}
    else {
      if(![6,7].includes(H.length)||JSON.stringify(nums(gcd(H,deriv(H))))!=="[1]")throw Error("nonsmooth nonzero-delta");
      const dual=map(h),deltaDual=dual.d;
      const factorScales=dual.h.map((q,i)=>{if(q.length!==g[i].length)return null;const s=mod(q[q.length-1]*inv(g[i][g[i].length-1]));return JSON.stringify(nums(q))===JSON.stringify(nums(g[i].map(x=>mod(s*x))))?Number(s):null});
      const dualIdentity=JSON.stringify(dual.h.map(nums))===JSON.stringify(g.map(nums));
      if(deltaDual===0n||mod(d*deltaDual)!==P-2n||!dualIdentity||factorScales.some(t=>t===null))throw Error("Richelot dual mismatch");
      row.type="jacobian_codomain";row.h=h.map(nums);row.H=nums(H);row.smooth=true;
      row.dual={delta:Number(deltaDual),h:dual.h.map(nums),H:nums(dual.H),exact_source_factors:dualIdentity,quadratic_factor_scales:factorScales};
    }
    rows.push(row);
  }
  if(rows.length!==15)throw Error("matchings count");
  const first=rows[0],g=first.g.map(a=>a.map(BigInt)),m=map(g);
  const accepts=(x,y,z,w)=>{const d=determinant(x);if(d!==y)throw Error("bad det");if(JSON.stringify(nums(prod(x)))!==JSON.stringify(nums(source)))throw Error("wrong source");const n=map(x);if(JSON.stringify(n.h?.map(nums))!==JSON.stringify(z?.map(nums))||JSON.stringify(n.H&&nums(n.H))!==JSON.stringify(w&&nums(w)))throw Error("bad output");return true;};
  const reject=f=>{try{f();return false;}catch{return true;}};
  const mutation={
    perturb_delta:reject(()=>accepts(g,m.d+1n,m.h,m.H)),
    perturb_bracket:reject(()=>accepts(g,m.d,[sum(m.h[0],[1n]),m.h[1],m.h[2]],m.H)),
    duplicate_root:reject(()=>accepts([mul([P-1n,1n],[P-1n,1n]),g[1],g[2]],m.d,m.h,m.H))
  };
  if(Object.values(mutation).some(x=>!x))throw Error("failed negative control");
  return {
    p:Number(P),ct334_json_blob:"01be355bf299c01419ac3a43a53adc4f8f7d61ea",
    scope:"exact finite Fp Richelot codomain/dual equations only; no evaluated divisor map, native benchmark or signed PCC",
    source_f:nums(source),geometric_kernels:15,rational_kernels:15,
    smooth_jacobian_codomains:rows.filter(r=>r.smooth).length,
    decomposable_quotients:rows.filter(r=>r.delta===0).length,
    dual_algebra_identities:rows.filter(r=>r.dual?.exact_source_factors).length,
    mutation_rejected:mutation,positive_control:accepts(g,m.d,m.h,m.H),
    rows,unmet:"No native Cantor, full 695/701 replay, divisor images, complete ECDLP cost, independence or signed PCC"
  };
};
process.stdout.write(JSON.stringify(study(),null,2)+"\n");
