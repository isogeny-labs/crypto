// Independent cross-check of coverage_t3.cpp for ONE cell, by a different method:
// direct affine point enumeration + hash set, standard group law, NO discrete-log
// representation. Prints coverage(u=1) and E2/X^2 for p=1009, y^2=x^3+x+14.
// Build: g++ -std=c++20 -O3 -Wall -Wextra -Werror verify_one.cpp -o verify_one
#include <cstdint>
#include <cstdio>
#include <cmath>
#include <vector>
#include <unordered_map>
#include <unordered_set>
using i64 = long long;
static i64 P=1009, A=1, B=14;
static i64 md(i64 x){ x%=P; return x<0?x+P:x; }
static i64 pw(i64 x,i64 e){ i64 y=1;x=md(x);for(;e;e>>=1,x=md(x*x))if(e&1)y=md(y*x);return y; }
static i64 inv(i64 x){ return pw(x,P-2); }
static i64 leg(i64 t){ t=md(t); if(!t)return 0; return pw(t,(P-1)/2)==1?1:-1; }
// encode an affine point as x*P+y; O = -1
static i64 enc(i64 x,i64 y){ return x*P+y; }
static i64 addEnc(i64 e1,i64 e2){
    if(e1<0) return e2;
    if(e2<0) return e1;
    i64 x1=e1/P,y1=e1%P,x2=e2/P,y2=e2%P;
    if(x1==x2 && md(y1+y2)==0) return -1; // O
    i64 num,den;
    if(x1==x2 && y1==y2){ num=md(3*x1%P*x1+A); den=md(2*y1); }
    else { num=md(y2-y1); den=md(x2-x1); }
    i64 s=md(num*inv(den));
    i64 x3=md(s*s-x1-x2), y3=md(s*md(x1-x3)-y1);
    return enc(x3,y3);
}
int main(){
    i64 X=(i64)std::floor(std::sqrt((double)P));
    // all affine points
    std::vector<i64> pts; // encodings
    for(i64 x=0;x<P;x++){ i64 rhs=md((x*x%P)*x+A*x+B); i64 l=leg(rhs); if(l==0){ pts.push_back(enc(x,0)); } else if(l==1){ i64 y=-1; for(i64 t=0;t<P;t++) if(t*t%P==rhs){y=t;break;} pts.push_back(enc(x,y)); pts.push_back(enc(x,md(-y))); } }
    i64 n=(i64)pts.size()+1; // + O
    // u=1: factor base = affine points with x in [1,X]
    std::vector<i64> Bu; for(i64 e:pts){ i64 x=e/P; if(x>=1&&x<=X) Bu.push_back(e); }
    // coverage: distinct generic 2-sums (distinct x)
    std::unordered_set<i64> covered;
    for(size_t i=0;i<Bu.size();i++) for(size_t j=i+1;j<Bu.size();j++){ if(Bu[i]/P==Bu[j]/P) continue; covered.insert(addEnc(Bu[i],Bu[j])); }
    double cov=(double)covered.size()/(double)n;
    // energy: ordered pairs histogram, E2 = sum r^2
    std::unordered_map<i64,i64> r;
    for(size_t i=0;i<Bu.size();i++) for(size_t j=0;j<Bu.size();j++) r[addEnc(Bu[i],Bu[j])]++;
    i64 E2=0,chk=0; for(auto&kv:r){ E2+=kv.second*kv.second; chk+=kv.second; }
    printf("independent verify  p=%lld a=%lld b=%lld  n=%lld  X=%lld  f=%zu\n",(long long)P,(long long)A,(long long)B,(long long)n,(long long)X,Bu.size());
    printf("  coverage(u=1) = %.6f   E2/X^2 = %.6f   (sum r = %lld, f^2 = %zu)\n",cov,(double)E2/((double)X*X),(long long)chk,Bu.size()*Bu.size());
    return 0;
}
