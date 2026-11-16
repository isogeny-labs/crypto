#pragma once
#include "../photo_volcano_restart_20261017/source/json.hpp"
#include <algorithm>
#include <array>
#include <chrono>
#include <cstdint>
#include <functional>
#include <limits>
#include <stdexcept>
#include <vector>

// Development of a generic PDP fallback for a relation/linear-algebra pipeline.
// No factor-base logarithm or target witness is an input to this component.
namespace vic {
using U = std::uint64_t;
using W = __uint128_t;
using json = nlohmann::json;
inline void require(bool ok, const char* why) { if (!ok) throw std::runtime_error(why); }
struct Counts {
    U add=0, sub=0, mul=0, square=0, inverse=0, group_add=0, group_double=0,
      scalar=0, normalizations=0, table_insert_probes=0, table_query_probes=0;
    json wire() const { return {{"field_add",add},{"field_sub",sub},{"field_mul_excluding_squares",mul},
        {"field_square",square},{"field_inverse",inverse},{"group_add",group_add},
        {"group_double",group_double},{"scalar_calls",scalar},{"normalizations",normalizations},
        {"table_insert_probes",table_insert_probes},{"table_query_probes",table_query_probes}}; }
};
struct Field {
    U p; Counts count;
    explicit Field(U modulus):p(modulus) { require(p>3 && p<(U(1)<<62),"modulus range"); }
    U add(U a,U b) { ++count.add; return a>=p-b?a-(p-b):a+b; }
    U sub(U a,U b) { ++count.sub; return a>=b?a-b:p-(b-a); }
    U mul(U a,U b) { ++count.mul; return U(W(a)*b%p); }
    U sq(U a) { ++count.square; return U(W(a)*a%p); }
    U pow(U a,U n) { U r=1; while(n) { if(n&1)r=mul(r,a); n>>=1; if(n)a=sq(a); } return r; }
    U inv(U a) { require(a!=0,"inverse of zero"); ++count.inverse; return pow(a,p-2); }
};
inline bool is_prime(U n) {
    if(n<2)return false;
    for(U a:{2,3,5,7,11,13,17,19,23,29,31,37})if(n%a==0)return n==a;
    U d=n-1;unsigned s=0;while(!(d&1)){d>>=1;++s;}
    auto product=[n](U a,U b){return U(W(a)*b%n);};
    auto power=[&](U a,U k){U r=1;while(k){if(k&1)r=product(r,a);a=product(a,a);k>>=1;}return r;};
    for(U a:{2ULL,325ULL,9375ULL,28178ULL,450775ULL,9780504ULL,1795265022ULL}) {
        if(a%n==0)continue;
        U x=power(a%n,d);if(x==1||x==n-1)continue;
        bool ok=false;for(unsigned j=1;j<s;++j){x=product(x,x);if(x==n-1){ok=true;break;}}if(!ok)return false;
    }return true;
}
struct Point {
    U x=0,y=0;bool infinity=true;
    bool operator==(const Point& q)const { return infinity==q.infinity&&(infinity||(x==q.x&&y==q.y)); }
};
struct Jacobian { U x=0,y=1,z=0; };
struct Curve {
    Field f;U a,b;
    Curve(U p,U aa,U bb):f(p),a(aa),b(bb) {
        require(is_prime(p),"field modulus is not prime");require(a<p&&b<p,"noncanonical curve");
        require(f.add(f.mul(4,f.mul(f.sq(a),a)),f.mul(27%p,f.sq(b)))!=0,"singular curve");
    }
    bool on(Point p) { return p.infinity||(p.x<f.p&&p.y<f.p&&f.sq(p.y)==f.add(f.add(f.mul(f.sq(p.x),p.x),f.mul(a,p.x)),b)); }
    Point neg(Point p)const {if(!p.infinity&&p.y)p.y=f.p-p.y;return p;}
    Jacobian lift(Point p)const {return p.infinity?Jacobian{}:Jacobian{p.x,p.y,1};}
    Jacobian dbl(Jacobian p) {
        ++f.count.group_double;if(!p.z||!p.y)return {};
        U xx=f.sq(p.x),yy=f.sq(p.y),yyyy=f.sq(yy),zz=f.sq(p.z);
        U s=f.mul(2,f.sub(f.sub(f.sq(f.add(p.x,yy)),xx),yyyy));
        U m=f.add(f.mul(3,xx),f.mul(a,f.sq(zz)));
        U x=f.sub(f.sq(m),f.mul(2,s));
        U y=f.sub(f.mul(m,f.sub(s,x)),f.mul(8,yyyy));
        return {x,y,f.sub(f.sub(f.sq(f.add(p.y,p.z)),yy),zz)};
    }
    Jacobian mixed(Jacobian p,Point q) {
        ++f.count.group_add;if(q.infinity)return p;if(!p.z)return lift(q);
        U zz=f.sq(p.z),u=f.mul(q.x,zz),s=f.mul(q.y,f.mul(p.z,zz));
        U h=f.sub(u,p.x),r=f.sub(s,p.y);
        if(!h)return r?Jacobian{}:dbl(p);
        U hh=f.sq(h),hhh=f.mul(h,hh),v=f.mul(p.x,hh);
        U x=f.sub(f.sub(f.sq(r),hhh),f.mul(2,v));
        return {x,f.sub(f.mul(r,f.sub(v,x)),f.mul(p.y,hhh)),f.mul(p.z,h)};
    }
    std::vector<Point> normalize(const std::vector<Jacobian>& points) {
        std::vector<U> prefix(points.size());U acc=1;bool any=false;
        for(std::size_t i=0;i<points.size();++i){prefix[i]=acc;if(points[i].z){acc=f.mul(acc,points[i].z);any=true;}}
        U inverse=any?f.inv(acc):1;std::vector<Point> result(points.size());
        for(std::size_t i=points.size();i-->0;){const auto& p=points[i];if(!p.z)continue;
            U iz=f.mul(inverse,prefix[i]);inverse=f.mul(inverse,p.z);U iz2=f.sq(iz);
            result[i]={f.mul(p.x,iz2),f.mul(p.y,f.mul(iz2,iz)),false};++f.count.normalizations;
        }return result;
    }
    Point affine(Jacobian p) {return normalize({p})[0];}
    Point add(Point p,Point q) {return affine(mixed(lift(p),q));}
    Point scalar(U k,Point p) {
        ++f.count.scalar;Jacobian r;bool started=false;
        for(int bit=63;bit>=0;--bit){if(started)r=dbl(r);if((k>>bit)&1){r=mixed(r,p);started=true;}}
        return affine(r);
    }
    Point signed_scalar(int k,Point p) {return k<0?neg(scalar(U(-k),p)):scalar(U(k),p);}
};
using Coeff=std::array<int,12>;
inline U encoding(Point p) {return p.infinity?std::numeric_limits<U>::max():((p.x<<1)|(p.y&1))+1;}
inline U hash(U x) {x^=x>>30;x*=0xbf58476d1ce4e5b9ULL;x^=x>>27;x*=0x94d049bb133111ebULL;return x^(x>>31);}
struct Layout {U side_entries,slots,table_bytes,next_bytes,total_bytes,radix;};
inline Layout layout(unsigned radius) {
    require(radius>=1&&radius<=31,"radius range 1..31");U q=2*U(radius)+1,n=1;
    for(int i=0;i<6;++i){require(n<=std::numeric_limits<std::uint32_t>::max()/q,"side code exceeds uint32");n*=q;}
    U slots=1;while(slots<2*n)slots<<=1;
    return {n,slots,slots*16,n*4,slots*16+n*4,q};
}
// Mixed-radix streaming; all 6 coefficients are in [-radius,+radius].
// Each full point is normalized once per batch, including rolled-over states.
struct Walker {
    Curve& e;std::array<Point,6> base,roll;std::array<U,6> digit{};Jacobian current;U q;
    Walker(Curve& curve,const std::array<Point,6>& b,unsigned radius,Point offset={},U start_code=0):e(curve),base(b),q(2*U(radius)+1) {
        current=e.lift(offset);for(int i=0;i<6;++i){digit[i]=start_code%q;start_code/=q;current=e.mixed(current,e.signed_scalar(int(digit[i])-int(radius),base[i]));roll[i]=e.neg(e.scalar(q,base[i]));}
        require(start_code==0,"walker start code out of range");
    }
    Jacobian take() {auto result=current;for(int i=0;i<6;++i){current=e.mixed(current,base[i]);if(++digit[i]<q)break;digit[i]=0;current=e.mixed(current,roll[i]);}return result;}
};
struct Slot {U key=0;std::uint32_t head=0,count=0;};
static_assert(sizeof(Slot)==16,"memory estimate requires 16-byte slots");
struct Match {bool found=false;Coeff coefficient{};U right_examined=0;};
class Decomposer {
    Curve& e;std::array<Point,12> base;unsigned radius;Layout sizes;
    std::vector<Slot> table;std::vector<std::uint32_t> next;
    std::function<void(U)> progress;static constexpr std::uint32_t end=std::numeric_limits<std::uint32_t>::max();
    U locate(U key,bool insert) {
        U at=hash(key)&(sizes.slots-1);for(U probes=0;probes<sizes.slots;++probes){
            if(insert)++e.f.count.table_insert_probes;else ++e.f.count.table_query_probes;
            if(!table[at].key||table[at].key==key)return at;
            at=(at+1)&(sizes.slots-1);
        }throw std::runtime_error("table unexpectedly full");
    }
    std::array<Point,6> half(bool right)const {std::array<Point,6> b{};for(int i=0;i<6;++i)b[i]=base[i+(right?6:0)];return b;}
public:
    U entries_built=0,distinct_left_points=0;
    Decomposer(Curve& curve,const std::array<Point,12>& b,unsigned rad,U memory_limit,std::function<void(U)> tick={}):
        e(curve),base(b),radius(rad),sizes(layout(rad)),progress(std::move(tick)) {
        require(sizes.total_bytes<=memory_limit,"table exceeds explicit memory limit");
    }
    Layout memory()const{return sizes;}
    void build() {
        require(table.empty(),"table build called twice");table.resize(sizes.slots);next.resize(sizes.side_entries,end);Walker w(e,half(false),radius);
        constexpr U block=4096;
        for(U start=0;start<sizes.side_entries;start+=block){if(progress)progress(start);
            U n=std::min(block,sizes.side_entries-start);std::vector<Jacobian> p;p.reserve(n);for(U i=0;i<n;++i)p.push_back(w.take());
            auto points=e.normalize(p);for(U i=0;i<n;++i){U key=encoding(points[i]),at=locate(key,true);auto& slot=table[at];
                std::uint32_t code=std::uint32_t(start+i);if(!slot.key){slot.key=key;slot.head=code;slot.count=1;++distinct_left_points;}
                else{next[code]=slot.head;slot.head=code;++slot.count;}++entries_built;
            }
        }if(progress)progress(sizes.side_entries);
    }
    // Pick selects a retained left representation, not a logarithm. Keeping
    // all duplicate representations prevents silent loss of relation rank.
    Match find(Point target,const std::function<U(U)>& pick) {
        require(entries_built==sizes.side_entries,"table incomplete");auto b=half(true);for(auto& p:b)p=e.neg(p);
        U origin=pick(sizes.side_entries);require(origin<sizes.side_entries,"right start out of range");
        Walker w(e,b,radius,target,origin);constexpr U block=4096;
        for(U start=0;start<sizes.side_entries;start+=block){if(progress)progress(start);U n=std::min(block,sizes.side_entries-start);
            std::vector<Jacobian> p;p.reserve(n);for(U i=0;i<n;++i)p.push_back(w.take());auto points=e.normalize(p);
            for(U i=0;i<n;++i){U at=locate(encoding(points[i]),false);const auto& slot=table[at];if(!slot.key)continue;
                U selected=pick(slot.count);require(selected<slot.count,"representation selection out of range");std::uint32_t code=slot.head;
                while(selected--){code=next[code];require(code!=end,"broken representation chain");}
                Coeff c{};U left=code,right=(origin+start+i)%sizes.side_entries;for(int j=0;j<6;++j){c[j]=int(left%sizes.radix)-int(radius);left/=sizes.radix;c[j+6]=int(right%sizes.radix)-int(radius);right/=sizes.radix;}
                return {true,c,start+i+1};
            }
        }return {false,{},sizes.side_entries};
    }
};
inline Point evaluate(Curve& e,const std::array<Point,12>& base,const Coeff& c) {
    Jacobian sum;for(int i=0;i<12;++i)sum=e.mixed(sum,e.signed_scalar(c[i],base[i]));return e.affine(sum);
}
class RowSpace {
    Field f;std::vector<std::array<U,13>> rows;std::vector<int> pivots;
public:
    explicit RowSpace(U r):f(r) {require(is_prime(r),"scalar modulus is not prime");}
    std::size_t rank()const{return rows.size();}Counts counters()const{return f.count;}
    Counts take_counts(){auto c=f.count;f.count={};return c;}
    bool insert(const Coeff& c,U rhs) {
        require(rhs<f.p,"noncanonical RHS");std::array<U,13> v{};for(int i=0;i<12;++i){U a=U(c[i]<0?-c[i]:c[i])%f.p;v[i]=c[i]<0&&a?f.p-a:a;}v[12]=rhs;
        for(std::size_t i=0;i<rows.size();++i){U a=v[pivots[i]];if(a)for(int j=0;j<13;++j)v[j]=f.sub(v[j],f.mul(a,rows[i][j]));}
        int pivot=0;while(pivot<12&&!v[pivot])++pivot;if(pivot==12){require(!v[12],"inconsistent relation");return false;}
        U z=f.inv(v[pivot]);for(auto& x:v)x=f.mul(x,z);
        for(auto& row:rows){U a=row[pivot];if(a)for(int j=0;j<13;++j)row[j]=f.sub(row[j],f.mul(a,v[j]));}
        auto it=std::lower_bound(pivots.begin(),pivots.end(),pivot);auto at=it-pivots.begin();pivots.insert(it,pivot);rows.insert(rows.begin()+at,v);return true;
    }
    std::array<U,12> logs()const {require(rank()==12,"insufficient rank");std::array<U,12> result{};for(int i=0;i<12;++i){require(pivots[i]==i,"noncanonical row space");result[i]=rows[i][12];}return result;}
    U recover(const Coeff& c) {auto d=logs();U k=0;for(int i=0;i<12;++i){U a=U(c[i]<0?-c[i]:c[i])%f.p;if(c[i]<0&&a)a=f.p-a;k=f.add(k,f.mul(a,d[i]));}return k;}
};
} // namespace vic
