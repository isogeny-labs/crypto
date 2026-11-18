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

Curve first_curve(int p) {
    for (int a=1;a<p;++a) for (int b=1;b<p;++b) {
        Curve c{p,a,b,0,{},{}};
        if (c.mod(4LL*a*a*a+27LL*b*b)==0) continue;
        std::vector<int> root(p,-1);
        for (int y=p/2;y>=0;--y) root[c.mod(1LL*y*y)]=y;
        int n=1;
        for (int x=0;x<p;++x) {
            int y=root[c.mod(1LL*x*x*x+a*x+b)];
            if (y<0) continue;
            n+=y==0?1:2;
            if (y!=0) c.points.push_back({x,y});
        }
        if (!prime(n) || n==p) continue;
        bool low_embedding=false;
        int residue=1;
        for (int k=1;k<=6;++k) {
            residue=residue*p%n;
            low_embedding|=residue==1;
        }
        if (low_embedding) continue;
        c.n=n;
        Point g=c.points.front();
        c.multiples.push_back(infinity);
        for (int k=1;k<n;++k) c.multiples.push_back(c.add(c.multiples.back(),g));
        assert(c.add(c.multiples.back(),g)==infinity);
        return c;
    }
    __builtin_abort();
}

std::vector<Point> choose(const Curve& c,int size,int kind) {
    auto pool=c.points;
    if (kind==1) {
        std::map<int,Point> by_x;
        for (auto point:pool) by_x.emplace(point[0],point);
        pool.clear();
        for (int z=0;z<c.p;++z) {
            int z2=c.mod(1LL*z*z);
            int x=c.mod(1LL*z2*z2-4*z2+2);
            if (auto it=by_x.find(x);it!=by_x.end()) {
                pool.push_back(it->second);
                by_x.erase(it);
            }
        }
    } else if (kind==2) {
        std::mt19937 random(20261117+c.p+size);
        std::shuffle(pool.begin(),pool.end(),random);
    }
    assert(int(pool.size())>=size);
    pool.resize(size);
    return pool;
}

void coverage(const Curve& c,const std::vector<Point>& base,const char* name) {
    std::map<Point,int> logs;
    for (int k=0;k<c.n;++k) assert(logs.emplace(c.multiples[k],k).second);
    int B=base.size();
    std::vector<int> k;
    for (auto point:base) k.push_back(logs.at(point));
    std::mt19937 random(17+c.p+B);
    std::vector<int> beta(B);
    for (int& b:beta) b=1+random()%(c.n-1);
    for (int law=0;law<2;++law) {
        std::vector<long long> h(c.n), total(c.n);
        for (int i=0;i<B;++i) for (int j=i+1;j<B;++j)
        for (int l=j+1;l<B;++l) for (int m=l+1;m<B;++m) {
            std::array<int,4> index{i,j,l,m};
            for (int mask=0;mask<16;++mask) {
                int scalar=0,denominator=0;
                Point witness=infinity;
                for (int j=0;j<4;++j) {
                    int sign=(mask>>j&1)?-1:1;
                    int at=index[j];
                    scalar+=sign*k[at];
                    denominator+=sign*(law==0?1:beta[at]);
                    Point point=base[at];
                    point[1]=c.mod(sign*point[1]);
                    witness=c.add(witness,point);
                }
                scalar=(scalar%c.n+c.n)%c.n;
                assert(witness==c.multiples[scalar]);
                ++total[scalar];
                if (denominator%c.n) ++h[scalar];
            }
        }
        long long tuples=std::accumulate(total.begin(),total.end(),0LL);
        assert(tuples==16LL*B*(B-1)*(B-2)*(B-3)/24);
        long long count=0, energy=0,covered=0, raw_covered=0;
        for (int i=0;i<c.n;++i) {
            count+=h[i]; energy+=h[i]*h[i];
            covered+=h[i]>0; raw_covered+=total[i]>0;
        }
        assert(count*count<=energy*covered);
        std::cout<<"coverage,"<<c.p<<','<<c.n<<','<<c.a<<','<<c.b<<','<<B<<','<<name<<','<<law
                 <<','<<tuples<<','<<count<<','<<energy<<','<<raw_covered<<','<<covered<<','
                 <<(energy?double(count*count)/(c.n*energy):0)<<'\n';
    }
}

using Terms=std::map<std::array<int,3>,int>;
Terms semaev(const Curve& c) {
    return {
       {{2,0,2},1},{{1,1,2},c.mod(-2)},{{0,2,2},1},
       {{2,1,1},c.mod(-2)},{{1,2,1},c.mod(-2)},{{1,0,1},c.mod(-2*c.a)},
       {{0,1,1},c.mod(-2*c.a)},{{0,0,1},c.mod(-4*c.b)},
       {{2,2,0},1},{{1,1,0},c.mod(-2*c.a)},{{0,0,0},c.mod(1LL*c.a*c.a)},
       {{1,0,0},c.mod(-4*c.b)},{{0,1,0},c.mod(-4*c.b)}
    };
}
int eval(const Curve& c,const Terms& terms,int x,int y,int t) {
    int z=0;
    for (const auto& [e,v]:terms) z=c.mod(z+1LL*v*c.power(x,e[0])*c.power(y,e[1])*c.power(t,e[2]));
    return z;
}

void quotient(const Curve& c,const std::vector<Point>& base,const char* name) {
    int B=base.size(),N=B*B;
    std::vector<int> h{1};
    for (Point point:base) {
        std::vector<int> next(h.size()+1);
        for (int i=0;i<int(h.size());++i) {
            next[i]=c.mod(next[i]-1LL*point[0]*h[i]);
            next[i+1]=c.mod(next[i+1]+h[i]);
        }
        h=std::move(next);
    }
    std::vector<std::vector<int>> power(B+2,std::vector<int>(B));
    power[0][0]=1;
    for (int i=1;i<B+2;++i) for (int j=0;j<B;++j)
        power[i][j]=c.mod((j?power[i-1][j-1]:0)-1LL*power[i-1].back()*h[j]);
    using Sparse=std::vector<std::pair<int,int>>;
    std::vector<Sparse> reduced(B+2);
    for (int i=0;i<B+2;++i) for (int j=0;j<B;++j)
        if (power[i][j]) reduced[i].emplace_back(j,power[i][j]);

    auto exact=semaev(c);
    auto randomized=exact;
    std::mt19937 random(23+c.p+B);
    for (auto& [e,value]:randomized) if (value) value=1+random()%(c.p-1);
    for (int variant=0;variant<2;++variant) {
        const auto& terms=variant?randomized:exact;
        std::array<std::vector<std::map<int,int>>,3> matrix;
        for (auto& part:matrix) part.resize(N);
        long long construction=0, nonzeros=0;
        for (int i=0;i<B;++i) for (int j=0;j<B;++j)
        for (const auto& [e,value]:terms) if (value)
        for (auto [u,cu]:reduced[i+e[0]]) for (auto [v,cv]:reduced[j+e[1]]) {
            auto& entry=matrix[e[2]][i*B+j][u*B+v];
            entry=c.mod(entry+1LL*value*cu*cv);
            construction+=2;
        }
        std::array<std::vector<int>,3> applied;
        for (int d=0;d<3;++d) {
            applied[d].resize(N);
            for (int col=0;col<N;++col) for (auto [row,value]:matrix[d][col]) {
                if (!value) continue;
                ++nonzeros;
                applied[d][row]=c.mod(applied[d][row]+1LL*value*(col+1));
            }
        }
        for (auto xp:base) for (auto yp:base) for (int t:{0,1,7}) {
            int x=xp[0],y=yp[0],before=0,after=0;
            for (int i=0;i<B;++i) for (int j=0;j<B;++j) {
                int offset=i*B+j;
                int xy=c.mod(1LL*c.power(x,i)*c.power(y,j));
                before=c.mod(before+1LL*(offset+1)*xy);
                for (int d=0;d<3;++d)
                    after=c.mod(after+1LL*xy*c.power(t,d)*applied[d][offset]);
            }
            assert(after==c.mod(1LL*before*eval(c,terms,x,y,t)));
        }
        std::cout<<"operator,"<<c.p<<','<<B<<','<<name<<','<<variant<<','<<N<<','
                 <<construction<<','<<nonzeros<<'\n';
    }

    if (B!=4) return;
    auto left1=base[0],left2=base[1],right1=base[2],right2=base[3];
    auto signed_pairs=[&](Point x,Point y) {
        std::vector<Point> sums;
        for (int sx:{1,-1}) for (int sy:{1,-1})
            sums.push_back(c.add({x[0],c.mod(sx*x[1])},{y[0],c.mod(sy*y[1])}));
        return sums;
    };
    auto left=signed_pairs(left1,left2);
    assert(std::none_of(left.begin(),left.end(),[](Point x){return x==infinity;}));
    int points_checked=0, matches=0, exceptional=0;
    for (Point residual:c.multiples) {
        std::vector<Point> complements;
        for (auto point:signed_pairs(right1,right2)) {
            assert(point!=infinity);
            complements.push_back(c.add(residual,{point[0],c.mod(-point[1])}));
        }
        if (std::find(complements.begin(),complements.end(),infinity)!=complements.end()) {
            ++exceptional;
            continue;
        }
        auto in_x=[](const auto& set,int t) {
            return std::any_of(set.begin(),set.end(),[&](Point point){return point[0]==t;});
        };
        for (int t=0;t<c.p;++t) {
            bool l=eval(c,exact,left1[0],left2[0],t)==0;
            bool right_raw=eval(c,exact,right1[0],right2[0],t)==0;
            assert(l==in_x(left,t));
            assert(right_raw==in_x(signed_pairs(right1,right2),t));
            if (!l) continue;
            for (auto point:left) if (point[0]==t) {
                Point neg{point[0],c.mod(-point[1])};
                auto remainder=c.add(residual,neg);
                bool algebra=remainder!=infinity &&
                    eval(c,exact,right1[0],right2[0],remainder[0])==0;
                bool direct=std::find(complements.begin(),complements.end(),point)!=complements.end();
                assert(algebra==direct);
                matches+=direct;
                ++points_checked;
            }
        }
    }
    std::cout<<"join,"<<c.p<<','<<name<<','<<points_checked<<','<<matches<<','<<exceptional<<'\n';
}
int main() {
    for (int p:{101,211,503,1009}) {
        auto curve=first_curve(p);
        for (int size:{4,8,12,16}) for (int kind=0;kind<3;++kind) {
            auto base=choose(curve,size,kind);
            const char* name=std::array{"interval","dickson_d4","random"}[kind];
            coverage(curve,base,name);
            quotient(curve,base,name);
        }
    }
}
