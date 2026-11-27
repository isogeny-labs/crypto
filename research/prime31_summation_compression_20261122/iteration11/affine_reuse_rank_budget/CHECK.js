// Deterministic integer counting only, no cryptographic arithmetic or random draws.
const r=2147446831n,N=1923n,k=31n,K=(N*N-1n)/2n;
const ceil=(a,b)=>(a+b-1n)/b;
function verify(a,b,p,s,c) {
 if(ceil(a*r*k,b*K)!==p || (s-1n)*(s-1n)/4n>=p || s*s/4n<p || s*N!==c) throw Error("threshold");
}
verify(1n,2n,18003n,269n,517287n);
verify(9n,10n,32404n,361n,694203n);
verify(19n,20n,34205n,370n,711510n);
verify(1n,1n,36005n,380n,730740n);
if(N*(N+1n)/2n-(N+1n)/2n!==K || K!==1848964n) throw Error("split count");
if(!(4096n*K<4n*r && 4096n*K*1000n<114n*r*k)) throw Error("batch rank bound");
console.log("four thresholds, split identity, batch rank bound: PASS");
