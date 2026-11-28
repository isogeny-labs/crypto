#include <cmath>
#include <iomanip>
#include <set>
#include <stdexcept>
#include <string_view>
#include <algorithm>
#include <array>
#include <cassert>
#include <cstdint>
#include <iostream>
#include <map>
#include <numeric>
#include <random>
#include <vector>

using Point=std::array<int,2>;
const Point infinity{-1,-1};

struct Curve {
    int p,a,b,n;
    std::vector<Point> points, multiples;
    int mod(long long x) const { return (x%p+p)%p; }
    int power(int x,int exponent) const {
        int y=1;
        for (; exponent; exponent>>=1,x=mod(1LL*x*x)) if (exponent&1) y=mod(1LL*x*y);
        return y;
    }
    Point add(Point s,Point t) const {
        if (s==infinity) return t;
        if (t==infinity) return s;
        if (s[0]==t[0] && mod(s[1]+t[1])==0) return infinity;
        int numerator=s==t ? mod(3LL*s[0]*s[0]+a) : mod(t[1]-s[1]);
        int denominator=s==t ? mod(2*s[1]) : mod(t[0]-s[0]);
        int slope=mod(1LL*numerator*power(denominator,p-2));
        int x=mod(1LL*slope*slope-s[0]-t[0]);
        return {x,mod(1LL*slope*(s[0]-x)-s[1])};
    }
};

bool prime(int n) {
    if (n<2) return false;
    for (int d=2;d*d<=n;++d) if (n%d==0) return false;
    return true;
}

struct Random {
    std::uint64_t state;
    std::uint64_t next() {
        std::uint64_t z = (state += 0x9e3779b97f4a7c15ULL);
        z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ULL;
        z = (z ^ (z >> 27)) * 0x94d049bb133111ebULL;
        return z ^ (z >> 31);
    }
    int bounded(int bound) {
        auto limit = -std::uint64_t(bound) % bound;
        std::uint64_t value;
        do { value = next(); } while (value < limit);
        return value % bound;
    }
};

constexpr int curve_count=6, random_count=16, label_count=5;
constexpr std::array fields{101,211,503,1009,2003,4001,8009,16001};

int mod_r(long long val,int n) { return (val%n+n)%n; }

std::vector<Curve> choose_curves(int p) {
    assert(prime(p));
    Random rng{0x202611170101ULL ^ std::uint64_t(p)};
    std::vector<Curve> curves;
    std::set<int> used_j;
    std::vector<int> root(p,-1);
    for (int y=p/2;y>=0;--y) root[1LL*y*y%p]=y;
    for (int attempt=0;attempt<8192 && curves.size()<curve_count;++attempt) {
        int a=1+rng.bounded(p-1), b=1+rng.bounded(p-1);
        Curve c{p,a,b,0,{},{}};
        int a_cube=c.mod(4LL*a*a*a);
        int discriminant=c.mod(a_cube+27LL*b*b);
        if (!discriminant) continue;
        int j=c.mod(1728LL*a_cube*c.power(discriminant,p-2));
        if (used_j.contains(j)) continue;
        int n=1;
        for (int x=0;x<p;++x) {
            int y=root[c.mod(1LL*x*x*x+a*x+b)];
            if (y<0) continue;
            n+=y==0?1:2;
            if (y!=0) c.points.push_back({x,y});
        }
        if (!prime(n) || n==2 || n==p) continue;
        bool low_embedding=false;
        int residue=1;
        for (int k=1;k<=6;++k) {
            residue=1LL*residue*p%n;
            low_embedding|=residue==1;
        }
        if (low_embedding) continue;
        c.n=n;
        Point g=c.points.front();
        c.multiples.push_back(infinity);
        for (int k=1;k<n;++k) c.multiples.push_back(c.add(c.multiples.back(),g));
        assert(c.add(c.multiples.back(),g)==infinity);
        assert(std::set<Point>(c.multiples.begin(),c.multiples.end()).size()==c.multiples.size());
        used_j.insert(j);
        curves.push_back(std::move(c));
    }
    if (curves.size()!=curve_count) throw std::runtime_error("curve draw cap");
    return curves;
}

std::uint64_t seed(const Curve& c,int size,int id,std::uint64_t domain) {
    Random rng{domain ^ (std::uint64_t(c.p)<<48) ^ (std::uint64_t(c.a)<<32) ^
               (std::uint64_t(c.b)<<16) ^ (std::uint64_t(size)<<8) ^ std::uint64_t(id)};
    return rng.next();
}

std::vector<Point> choose(const Curve& c,int size,int arm,int id) {
    auto pool=c.points;
    if (arm==1) {
        std::vector<int> by_x(c.p,-1);
        for (const auto& point:pool) by_x[point[0]]=point[1];
        pool.clear();
        for (int z=0;z<c.p && int(pool.size())<size;++z) {
            int z2=c.mod(1LL*z*z);
            int x=c.mod(1LL*z2*z2-4*z2+2);
            if (by_x[x]<0) continue;
            pool.push_back({x,by_x[x]});
            by_x[x]=-1;
        }
    } else if (arm==2) {
        Random rng{seed(c,size,id,0x20261117b451ULL)};
        for (int k=0;k<size;++k)
            std::swap(pool[k],pool[k+rng.bounded(pool.size()-k)]);
    }
    if (int(pool.size())<size) throw std::runtime_error("short base");
    pool.resize(size);
    std::sort(pool.begin(),pool.end());
    assert(std::adjacent_find(pool.begin(),pool.end())==pool.end());
    return pool;
}

struct Moment {
    long long sum=0,energy=0,hits=0;
};

Moment moments(const std::vector<int>& bins) {
    Moment m;
    for (auto h:bins) {
        m.sum+=h;
        m.energy+=1LL*h*h;
        m.hits+=h>0;
    }
    assert(m.sum*m.sum<=m.energy*m.hits);
    return m;
}

Point times(const Curve& c,Point point,int multiple) {
    Point out=infinity;
    for (int x=multiple;x;x>>=1) {
        if (x&1) out=c.add(out,point);
        point=c.add(point,point);
    }
    return out;
}

int inverse_subgroup(int x,int n) {
    long long inv=1,power=x;
    for (int e=n-2;e;e>>=1,power=power*power%n)
        if (e&1) inv=inv*power%n;
    assert(inv*x%n==1);
    return inv;
}

long long tuple_cap(int n) {
    return 16LL*n*(n-1)*(n-2)*(n-3)/24;
}

void coverage(const Curve& c,const std::map<Point,int>& logs,const std::vector<Point>& base,
              int relative_B,int prime_id,int curve_id,int arm,int base_id) {
    int B=base.size(), n=c.n;
    int q=17;
    const Point G=c.multiples[1], public_Q=times(c,G,q);
    std::vector<int> scalar(B);
    std::array<std::vector<int>,label_count> beta;
    for (int i=0;i<B;++i) scalar[i]=logs.at(base[i]);
    for (int law=0;law<label_count;++law) {
        Random rng{seed(c,B,law,0x20261117ab10ULL)};
        beta[law].reserve(B);
        for (int i=0;i<B;++i) beta[law].push_back(law?1+rng.bounded(n-1):1);
    }
    std::vector<int> total(n);
    std::array<std::array<std::vector<int>,2>,label_count> informative;
    std::array<std::array<bool,2>,label_count> recovery{};
    for (auto& per_law:informative) for (auto& bins:per_law) bins.resize(n);
    long long tuple_index=0, point_checks=0;
    const long long samples_stride=std::max(1LL,tuple_cap(B)/32);
    for (int i=0;i<B;++i) for (int j=i+1;j<B;++j)
    for (int l=j+1;l<B;++l) for (int m=l+1;m<B;++m) {
        std::array index{i,j,l,m};
        for (int mask=0;mask<16;++mask,++tuple_index) {
            std::array<int,4> sign;
            int alpha=0;
            for (int k=0;k<4;++k) {
                sign[k]=(mask>>k&1)?-1:1;
                alpha+=sign[k]*scalar[index[k]];
            }
            int R=mod_r(alpha,n);
            if (tuple_index%samples_stride==0) {
                Point witness=infinity;
                for (int k=0;k<4;++k) {
                    auto point=base[index[k]];
                    point[1]=c.mod(sign[k]*point[1]);
                    witness=c.add(witness,point);
                }
                assert(witness==c.multiples[R]);
                ++point_checks;
            }
            ++total[R];
            for (int law=0;law<label_count;++law) {
                int sum_beta=0;
                for (int k=0;k<4;++k) sum_beta+=sign[k]*beta[law][index[k]];
                for (int v=0;v<=1;++v) {
                    int denominator=mod_r(sum_beta-v,n);
                    if (!denominator) continue;
                    ++informative[law][v][R];
                    if (recovery[law][v]) continue;
                    // Construct the label relation independently, then recover q using it.
                    int sum_u=0;
                    for (int k=0;k<4;++k) {
                        int ix=index[k];
                        sum_u+=sign[k]*mod_r(scalar[ix]-q*beta[law][ix],n);
                    }
                    int u=mod_r(R-q*v,n);
                    int d=inverse_subgroup(denominator,n);
                    int q_recovered=mod_r(1LL*(u-sum_u)*d,n);
                    assert(times(c,G,q_recovered)==public_Q);
                    assert(c.add(times(c,G,u),times(c,public_Q,v))==c.multiples[R]);
                    recovery[law][v]=true;
                }
            }
        }
    }
    assert(point_checks>=32);
    auto raw=moments(total);
    assert(raw.sum==tuple_cap(B) && raw.sum==tuple_index);
    for (int law=0;law<label_count;++law) for (int v=0;v<=1;++v) {
        auto use=moments(informative[law][v]);
        assert(use.sum<=raw.sum && use.hits<=raw.hits);
        assert(recovery[law][v]==(use.sum>0));
        for (int i=0;i<n;++i) assert(informative[law][v][i]<=total[i]);
        if (!law && v==1) assert(use.sum==raw.sum && use.energy==raw.energy);
        if (!law && v==0) assert(use.sum*16==raw.sum*10);
        std::cout<<prime_id<<','<<curve_id<<','<<c.p<<','<<c.a<<','<<c.b<<','<<n<<','
                 <<relative_B<<','<<B<<','<<arm<<','<<base_id<<','<<law<<','<<v<<','
                 <<raw.sum<<','<<raw.energy<<','<<raw.hits<<','<<use.sum<<','<<use.energy<<','
                 <<use.hits<<','<<point_checks<<','<<recovery[law][v]<<'\n';
    }
}

int main() {
    std::cout<<"field_id,curve_id,p,a,b,r,relative_B,B,arm,base_id,law,v,tuples,raw_energy,raw_hits,informative_tuples,informative_energy,informative_hits,point_checks,recovery_checked\n";
    for (int field_id=0;field_id<int(fields.size());++field_id) {
        auto curves=choose_curves(fields[field_id]);
        for (int curve_id=0;curve_id<curve_count;++curve_id) {
            const auto& c=curves[curve_id];
            std::cerr<<"field="<<c.p<<" curve="<<curve_id<<" a="<<c.a<<" b="<<c.b<<" r="<<c.n<<'\n';
            std::map<Point,int> logs;
            for (int i=0;i<c.n;++i) assert(logs.emplace(c.multiples[i],i).second);
            int B0=4;
            while (tuple_cap(B0)<c.n) ++B0;
            for (int offset=-1;offset<=1;++offset) for (int arm=0;arm<3;++arm)
                for (int id=0;id<(arm==2?random_count:1);++id)
                    coverage(c,logs,choose(c,B0+offset,arm,id),offset,field_id,curve_id,arm,id);
        }
    }
}
