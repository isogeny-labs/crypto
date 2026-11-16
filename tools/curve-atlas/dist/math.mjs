export const mod=(n,p)=>((n%p)+p)%p;
export function prime(p){if(!Number.isInteger(p)||p<2)return false;for(let d=2;d*d<=p;d++)if(p%d===0)return false;return true}
export function pow(a,n,p){let r=1;for(;n;n=Math.floor(n/2),a=mod(a*a,p))if(n%2)r=mod(r*a,p);return r}
export const inv=(a,p)=>pow(a,p-2,p);
export function gcd(a,b){while(b)[a,b]=[b,a%b];return a}
export function points(a,b,p){let count=1;for(let x=0;x<p;x++){let v=mod(x*x*x+a*x+b,p);count+=v===0?1:pow(v,(p-1)/2,p)===1?2:0}return count}
export function build({p,a,b,c,maxN,depth,quartic=true,jacobians=true}){
 if(!prime(p)||p<5||p>499)throw Error('Choose a prime field with 5 ≤ p ≤ 499.');
 if(![a,b,c,maxN,depth].every(Number.isSafeInteger)||Math.max(Math.abs(a),Math.abs(b),Math.abs(c))>1000000)throw Error('Coefficients must be integers of absolute value at most 1,000,000.');
 if(maxN<2||maxN>6||depth<0||depth>2)throw Error('Use cover exponent 2–6 and isogeny depth 0–2.');
 a=mod(a,p);b=mod(b,p);c=mod(c,p);
 if(mod(4*a*a*a+27*b*b,p)===0)throw Error('This cubic is singular. Choose coefficients with 4a³ + 27b² ≠ 0 mod p.');
 const j=mod(1728*4*a*a*a*inv(mod(4*a*a*a+27*b*b,p),p),p);
 const nodes=[{id:'E0',kind:'elliptic',a,b,genus:1,label:'E₀',count:points(a,b,p),level:0}],edges=[],seen=new Map([[`${a},${b}`,'E0']]);
 for(let i=0;i<nodes.length;i++){const e=nodes[i];if(e.kind!=='elliptic'||e.level>=depth)continue;
 for(let r=0;r<p;r++)if(mod(r*r*r+e.a*r+e.b,p)===0){const A=mod(-15*r*r-4*e.a,p),B=mod(-22*r*r*r-8*e.a*r,p),key=`${A},${B}`;let id=seen.get(key);if(!id){id='E'+seen.size;seen.set(key,id);nodes.push({id,kind:'elliptic',a:A,b:B,genus:1,label:'E'+String(seen.size-1).replace(/\d/g,d=>'₀₁₂₃₄₅₆₇₈₉'[Number(d)]),count:points(A,B,p),level:e.level+1})}edges.push({id:'i'+edges.length,source:e.id,target:id,kind:'isogeny',degree:2,kernel:r,formula:`X = x + (3r²+a)/(x−r), Y = y · (1−(3r²+a)/(x−r)²), r=${r}`,detail:'Normalized Vélu 2-isogeny, extended at the kernel to the point at infinity. Kernel {O, (r,0)}. Coefficients in this formula belong to the source model.'})}}
 const skipped=[];
 if(mod(c*c*c+a*c+b,p)===0)skipped.push('Cover shift c is a root of the cubic: the constructed affine models are not squarefree. Change c to generate smooth covers.');
 else for(const m of quartic?[2,4]:[2])for(let n=m===2?2:1;n<=maxN;n++){
 if((m*n)%p===0){skipped.push(`Skipped m=${m}, n=${n}: characteristic divides the cover exponent.`);continue}
 const g=((m-1)*(3*n-1)-gcd(m,3*n)+1)/2,id=`C${m}_${n}`,degree=n*m/2;
 nodes.push({id,kind:m===2?'hyperelliptic':'superelliptic',m,n,c,genus:g,label:m===2?`H${n}`:`S${n}`,equation:`v^${m} = (u^${n} + ${c})³ + ${a}(u^${n} + ${c}) + ${b}`,classification:m===2?'Hyperelliptic':n===1?'Nonhyperelliptic plane quartic':'Superelliptic; hyperellipticity not tested'});
 edges.push({id:'c'+id,source:id,target:'E0',kind:'cover',degree,formula:`(u, v) ↦ (u^${n} + ${c}, v^${m/2})`,detail:`Finite separable cover of degree ${degree}. Genus ${g} is for the smooth projective model. Tame total ramification: ${2*g-2}.`});
 if(jacobians){const jid='J'+id;nodes.push({id:jid,kind:'jacobian',genus:g,dimension:g,label:`J(${m===2?'H':'S'}${n})`,curve:id});edges.push({id:'p'+id,source:'E0',target:jid,kind:'pullback',degree:null,formula:'π*: E ≅ J(E) → J(C)',detail:`Pullback of degree-zero divisor classes. Together with the norm, π* satisfies π_* ∘ π* = [${degree}] on E. No isogeny onto this ${g}-dimensional Jacobian is implied.`});edges.push({id:'t'+id,source:jid,target:'E0',kind:'norm',degree:null,formula:'π_*: J(C) → E ≅ J(E)',detail:`Norm / pushforward of degree-zero divisor classes. π_* ∘ π* = [${degree}]. This is a homomorphism of abelian varieties, not a map from E to a higher-genus curve.`})}
 }
 return {parameters:{p,a,b,c,maxN,depth,quartic,jacobians},j,nodes,edges,skipped,scope:'Rational degree-2 kernels over F_p, explored to the selected depth on short-Weierstrass models (not isomorphism classes); cyclic covers v^m=f(u^n+c), m=2 or 4, within selected exponent bounds. Jacobian maps are induced structurally, not symbolic coordinate computations. Not an exhaustive list of all covers, isogenies, correspondences or transfers.'};
}
