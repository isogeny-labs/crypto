#include <gmpxx.h>
#include <algorithm>
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

using Q=mpq_class;
using Z=mpz_class;
using U=uint64_t;
using Matrix=std::vector<std::vector<U>>;
void need(bool b) { if (!b) throw std::runtime_error("exact certificate failed"); }
int val(Q q) {
    if(q==0) return 100000;
    Z n=q.get_num(); if(n<0)n=-n;
    return int(mpz_scan1(n.get_mpz_t(),0))-int(mpz_scan1(q.get_den().get_mpz_t(),0));
}
Q derivative(int l,int d) {
    Q r=1; for(int s=-l;s<l;++s) if(s!=d) r*=d-s; return r;
}
Q rho(int l,int d) { return derivative(l,d)/derivative(l,0); }
Q original(int l,int d,int t) {
    Q r=1; for(int s=-l;s<l;++s) if(s!=d) r*=t-s;
    return r/derivative(l,0);
}
U modular(Q a,U modulus) {
    a.canonicalize(); Z inv,m=modulus;
    need(mpz_invert(inv.get_mpz_t(),a.get_den().get_mpz_t(),m.get_mpz_t())!=0);
    Z v=a.get_num()*inv; return mpz_fdiv_ui(v.get_mpz_t(),modulus);
}
U inverse(U a,U modulus) { return modular(Q(1)/Q(a),modulus); }
U submul(U a,U b,U c,U modulus) { return (a+modulus-(b*c)%modulus)%modulus; }
int uv(U a,int q) { return a?__builtin_ctzll(a):q; }
Matrix identity(size_t n) {
    Matrix r(n,std::vector<U>(n)); for(size_t i=0;i<n;++i)r[i][i]=1; return r;
}
Matrix multiply(const Matrix&a,const Matrix&b,U m) {
    need(!a.empty()&&!b.empty()&&a[0].size()==b.size());
    Matrix c(a.size(),std::vector<U>(b[0].size()));
    for(size_t i=0;i<a.size();++i) for(size_t k=0;k<b.size();++k)
        for(size_t j=0;j<b[0].size();++j)c[i][j]=(c[i][j]+a[i][k]*b[k][j])%m;
    return c;
}
std::vector<int> smith(Matrix a,int q) {
    const Matrix input=a; U modulus=U(1)<<q;
    size_t rows=a.size(),cols=a[0].size();
    Matrix left=identity(rows),right=identity(cols);
    std::vector<int> weights;
    for(size_t k=0;k<std::min(rows,cols);++k) {
        int best=q;size_t br=k,bc=k;
        for(size_t r=k;r<rows;++r) for(size_t c=k;c<cols;++c)
            if(uv(a[r][c],q)<best){best=uv(a[r][c],q);br=r;bc=c;}
        if(best==q)break;
        std::swap(a[k],a[br]);
        for(auto&r:left)std::swap(r[k],r[br]);
        for(auto&r:a)std::swap(r[k],r[bc]);
        std::swap(right[k],right[bc]);
        U reduced=modulus>>best,inv=inverse(a[k][k]>>best,reduced);
        for(size_t r=k+1;r<rows;++r) {
            need(a[r][k]%(U(1)<<best)==0);
            U c=((a[r][k]>>best)*inv)%reduced;
            for(size_t j=0;j<cols;++j)a[r][j]=submul(a[r][j],c,a[k][j],modulus);
            for(size_t j=0;j<rows;++j)left[j][k]=(left[j][k]+c*left[j][r])%modulus;
        }
        for(size_t c=k+1;c<cols;++c) {
            need(a[k][c]%(U(1)<<best)==0);
            U r=((a[k][c]>>best)*inv)%reduced;
            for(size_t j=0;j<rows;++j)a[j][c]=submul(a[j][c],r,a[j][k],modulus);
            for(size_t j=0;j<cols;++j)right[k][j]=(right[k][j]+r*right[c][j])%modulus;
        }
        weights.push_back(best);
    }
    for(size_t r=0;r<rows;++r) for(size_t c=0;c<cols;++c)
        if(r!=c)need(a[r][c]==0);
    need(multiply(multiply(left,a,modulus),right,modulus)==input);
    need(std::is_sorted(weights.begin(),weights.end()));
    return weights;
}
std::vector<Q> differences(std::vector<Q> v) {
    std::vector<Q> c;
    while(!v.empty()) {
        c.push_back(v[0]);
        for(size_t j=0;j+1<v.size();++j)v[j]=v[j+1]-v[j];
        v.pop_back();
    }
    return c;
}
Z choose(unsigned n,unsigned k) { Z v;mpz_bin_uiui(v.get_mpz_t(),n,k);return v; }
void reconstruct(const std::vector<Q>&v,const std::vector<Q>&c) {
    for(unsigned j=0;j<v.size();++j) {
        Q r=0;for(unsigned b=0;b<=j;++b)r+=c[b]*choose(j,b);need(r==v[j]);
    }
}
Z tuples(const std::vector<int>&weights,int q) {
    std::vector<Z> dp(q);dp[0]=1;
    for(int w:weights) {
        need(w>0); if(w>=q)continue;
        for(int s=w;s<q;++s)dp[s]+=dp[s-w];
    }
    Z total=0;for(const Z&v:dp)total+=v;return total;
}
std::string histogram(const std::vector<int>&w,int q) {
    std::vector<int> count(q);
    for(int v:w)if(v<q){need(v>0);++count[v];}
    std::string r;
    for(int i=1;i<q;++i)if(count[i]) {
        if(!r.empty())r+=';';
        r+=std::to_string(i)+":"+std::to_string(count[i]);
    }
    return r;
}
void unit() {
    Q b=original(4,2,2),r=rho(4,2);
    need(b==5&&b*b!=b&&b*b/r==b);
    std::vector<Q> invrho{Q(1)/rho(8,-2),Q(1)/rho(8,0),Q(1)/rho(8,2)};
    auto c=differences(invrho); Q expected(-4862,6435);expected.canonicalize();
    need(c[2]==expected&&val(c[2])==1);reconstruct(invrho,c);
    Matrix a{{2,4},{6,0}};auto w=smith(a,8);
    need(w==std::vector<int>({1,2}));need(tuples({2,3},6)==5);
    std::cout<<"unit_node_positive_negative=PASS\nunit_normalizer_second_difference=-4862/6435 valuation=1\nunit_smith=1;2\nunit_tuple_count=5\nPASS\n";
}
void panel() {
    std::cout<<"L,t,level,zeta,q,method,original_tuples,candidate_tuples,original_weights,candidate_weights\n";
    for(int l:{4,8,16,32}) {
        std::vector<Q> normal(2*l-1);
        for(int d=-l+1;d<l;++d){normal[d+l-1]=rho(l,d);need(val(normal[d+l-1])==0);}
        for(int t=-l+1;t<l;++t)if(t!=0)for(int z:{-1,1,3}) {
            int i=val(Q(t)),stride=1<<(i+1),n=l/stride,T=t+2*l*z;
            std::vector<int> nw[2];std::vector<Matrix> matrices[2];
            for(int y0=0;y0<stride;++y0) {
                std::vector<int> rowweights[2];rowweights[0].assign(n,100000);rowweights[1].assign(n,100000);
                std::vector<std::vector<Q>> rational[2];
                for(int x=0;x<l;++x)if(((x>>i)&1)==((y0>>i)&1)) {
                    std::vector<Q> values[2];
                    for(int j=0;j<n;++j) {
                        int d=y0+stride*j-x;Q b=original(l,d,T),r=normal[d+l-1];
                        values[0].push_back(b);values[1].push_back(b*b/r);
                        need(val(b)>=0&&val(values[1].back())==2*val(b));
                    }
                    for(int k=0;k<2;++k) {
                        auto c=differences(values[k]);reconstruct(values[k],c);
                        for(int b=0;b<n;++b)rowweights[k][b]=std::min(rowweights[k][b],val(c[b]));
                        rational[k].push_back(values[k]);
                    }
                }
                for(int k=0;k<2;++k) {
                    nw[k].insert(nw[k].end(),rowweights[k].begin(),rowweights[k].end());
                    // Store exact integral residues at maximum precision; lower q reduces them.
                    Matrix a;for(const auto&row:rational[k]) {
                        std::vector<U> v;for(const Q&c:row)v.push_back(modular(c,U(1)<<24));a.push_back(v);
                    }
                    matrices[k].push_back(a);
                }
            }
            for(int q:{8,12,16,24}) {
                std::vector<int> sw[2];U m=U(1)<<q;
                for(int k=0;k<2;++k)for(Matrix a:matrices[k]) {
                    for(auto&r:a)for(U&v:r)v%=m;
                    auto w=smith(a,q);sw[k].insert(sw[k].end(),w.begin(),w.end());
                    if(k==0) {
                        for(auto&r:a)for(U&v:r)v=(2*v)%m;
                        auto doubled=smith(a,q);std::vector<int> expected;
                        for(int v:w)if(v+1<q)expected.push_back(v+1);
                        need(doubled==expected);
                    }
                }
                for(int method=0;method<2;++method) {
                    const auto&a=method?sw[0]:nw[0];const auto&b=method?sw[1]:nw[1];
                    std::cout<<l<<','<<t<<','<<i<<','<<z<<','<<q<<','<<(method?"smith":"newton")
                             <<','<<tuples(a,q)<<','<<tuples(b,q)<<','<<histogram(a,q)<<','<<histogram(b,q)<<'\n';
                }
            }
        }
    }
}
int main(int argc,char**argv) {
    try {
        need(argc==2);std::string mode=argv[1];
        if(mode=="--unit")unit();else if(mode=="--panel")panel();else need(false);
    }catch(const std::exception&e){std::cerr<<"FAIL "<<e.what()<<'\n';return 1;}
}
