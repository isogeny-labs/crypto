// Deterministic count identities only; no curve operations or random draws.
const r=2147446831n,N=1923n,J=1922n,h=64n,g=64n,M=h*g;
function require(x){if(!x)throw Error("count identity failed")}
let checks=0;
for(const m of [1n,64n,4096n,r])for(const j of [1n,J,618884n,r]){
 const lhs=m*j*r*(r-1n)+m*(m-1n)*j*(j-1n)*r-m*m*j*j*(r-1n);
 require(lhs===m*j*(r-j)*(r-m));checks++;
}
const seen=new Set();for(let i=0;i<64;i++)for(let j=0;j<64;j++)seen.add(i+64*j);
require(seen.size===4096 && Math.min(...seen)===0 && Math.max(...seen)===4095);
const scan=N*M,join=N*(h+g),stored=N*h;
require(scan===7876608n && join===246144n && stored===123072n && scan===32n*join);
const one=(r+J-1n)/J;require(one===1117299n && (one-1n)*J<r && one*J>=r);
const C=65536n;require((C-1n)**2n<2n*r && C*C>=2n*r);
console.log(JSON.stringify({variance_identities:checks,mixed_radix_indices:seen.size,direct_candidates:String(scan),join_entries:String(join),stored_labels:String(stored),rank_floor_mu_one:String(one),half_success_necessary_entries:String(C),expected_ordered_matches:Number(M*N*N)/Number(r)}));
