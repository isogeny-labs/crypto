#include <algorithm>
#include <array>
#include <cassert>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <span>
#include <string_view>
#include <unordered_map>
#include <vector>

using Word=std::uint64_t;
constexpr Word p=2147483647, curve_a=847514846, curve_b=2087444724;
struct Point { Word x,y; bool operator==(const Point&) const=default; };
constexpr Point infinity{p,0}, generator{625037715,2100627251};
struct Work {Word mults=0,inversions=0,root_visits=0,complement_visits=0;};
Work work;

Word plus(Word a,Word b) { Word x=a+b; return x>=p?x-p:x; }
Word minus(Word a,Word b) { return plus(a,p-b); }
Word times(Word a,Word b) { ++work.mults; return a*b%p; }
Word power(Word a,Word e) {
    Word out=1;
    for (; e; e>>=1,a=times(a,a)) if (e&1) out=times(out,a);
    return out;
}
Word inv(Word a) { assert(a); ++work.inversions; return power(a,p-2); }
Point neg(Point a) { return a==infinity?a:Point{a.x,(p-a.y)%p}; }
Point add(Point a,Point b) {
    if (a==infinity) return b;
    if (b==infinity) return a;
    if (a.x==b.x && plus(a.y,b.y)==0) return infinity;
    Word slope=a==b?
        times(plus(times(3,times(a.x,a.x)),curve_a),inv(plus(a.y,a.y))):
        times(minus(b.y,a.y),inv(minus(b.x,a.x)));
    Word x=minus(minus(times(slope,slope),a.x),b.x);
    return {x,minus(times(slope,minus(a.x,x)),a.y)};
}
Point scalar(Word k) {
    Point out=infinity, q=generator;
    for (; k; k>>=1,q=add(q,q)) if (k&1) out=add(out,q);
    return out;
}

void inverse_batch(std::span<const Word> in,std::vector<Word>& out,std::vector<Word>& prefix) {
    out.resize(in.size());
    prefix.resize(in.size());
    Word product=1;
    for (std::size_t i=0;i<in.size();i++) {
        prefix[i]=product;
        if (in[i]) product=times(product,in[i]);
    }
    Word reverse=inv(product);
    for (std::size_t i=in.size();i-->0;) {
        if (in[i]) {
            out[i]=times(prefix[i],reverse);
            reverse=times(reverse,in[i]);
        } else out[i]=0;
    }
}

class Row {
    std::vector<Word> denominator,inverse,prefix;
public:
    template <typename Emit>
    void roots(Point a,std::span<const Point> list,bool lift,Emit emit) {
        denominator.resize(list.size());
        for (std::size_t j=0;j<list.size();j++) {
            assert(a.x!=list[j].x);
            denominator[j]=minus(list[j].x,a.x);
        }
        inverse_batch(denominator,inverse,prefix);
        for (std::size_t j=0;j<list.size();j++) {
            Point b=list[j];
            for (bool negative:{false,true}) {
                Word num=minus(negative?neg(b).y:b.y,a.y);
                Word slope=times(num,inverse[j]);
                Word x=minus(minus(times(slope,slope),a.x),b.x);
                Word y=lift?minus(times(slope,minus(a.x,x)),a.y):0;
                ++work.root_visits;
                emit(Point{x,y});
            }
        }
    }
    template <typename Emit>
    void shift(Point target,std::span<const Point> list,bool lift,Emit emit) {
        denominator.resize(list.size());
        for (std::size_t j=0;j<list.size();j++) denominator[j]=minus(list[j].x,target.x);
        inverse_batch(denominator,inverse,prefix);
        for (std::size_t j=0;j<list.size();j++) for (bool negative:{false,true}) {
            Point b=negative?neg(list[j]):list[j];
            ++work.complement_visits;
            if (!inverse[j]) {
                auto point=add(target,b);
                emit(point);
                continue;
            }
            Word slope=times(minus(b.y,target.y),inverse[j]);
            Word x=minus(minus(times(slope,slope),target.x),b.x);
            Word y=lift?minus(times(slope,minus(target.x,x)),target.y):0;
            emit(Point{x,y});
        }
    }
};

Word s3(Word x, Word y, Word t) {
    Word d=minus(x,y),s=plus(x,y),z=times(x,y);
    Word v=minus(z,curve_a);
    Word q2=times(d,d),q1=minus(0,times(2,plus(times(s,plus(z,curve_a)),times(2,curve_b))));
    Word q0=minus(times(v,v),times(times(4,curve_b),s));
    return plus(times(t,plus(times(t,q2),q1)),q0);
}
using Bases=std::array<std::vector<Point>,4>;
Bases make_bases(std::size_t b,bool random) {
    Bases out;
    Word x=0, state=0x5a1e393a058715edULL;
    std::unordered_map<Word,bool> used;
    for (auto& points:out) {
        while(points.size()<b) {
            if (random) {
                state^=state>>12;
                state^=state<<25;
                state^=state>>27;
                x=state*2685821657736338717ULL%p;
            } else ++x;
            if (used.contains(x)) continue;
            Word rhs=plus(plus(times(times(x,x),x),times(curve_a,x)),curve_b);
            Word y=power(rhs,(p+1)/4);
            if (!y || times(y,y)!=rhs) continue;
            used.emplace(x,true);
            points.push_back({x,std::min(y,p-y)});
        }
    }
    return out;
}

struct Summary {Word hits=0,peak_keys=0,blocks=0; Work counters; std::int64_t ns=0;};
Summary query(const Bases& base,Point target,int mode) {
    assert(target!=infinity);
    const std::size_t b=base[0].size(), row_block=mode==2?4:b;
    const bool points=mode==0;
    std::unordered_map<Word,Word> left;
    std::vector<Point> right;
    Row roots,shifts;
    work={};
    auto start=std::chrono::steady_clock::now();
    left.reserve((points?4:2)*std::min(b,row_block)*b);
    right.reserve(2*b);
    Summary summary;
    for (std::size_t k=0;k<b;k+=row_block) {
        left.clear();
        ++summary.blocks;
        for (std::size_t i=k;i<std::min(k+row_block,b);i++) {
            roots.roots(base[0][i],base[1],points,[&](Point v) {
                if (points) {
                    ++left[v.x*p+v.y];
                    ++left[v.x*p+neg(v).y];
                } else ++left[v.x];
            });
        }
        summary.peak_keys=std::max(summary.peak_keys,Word(left.size()));
        for (const auto right_a:base[2]) {
            right.clear();
            roots.roots(right_a,base[3],true,[&](Point v){right.push_back(v);});
            shifts.shift(target,right,points,[&](Point v) {
                if (v==infinity) return;
                auto it=left.find(points?v.x*p+v.y:v.x);
                if (it!=left.end()) summary.hits+=it->second;
            });
        }
    }
    summary.ns=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-start).count();
    summary.counters=work;
    return summary;
}

void validate_root_grid(const Bases& base) {
    Row kernel;
    for (std::size_t arm:{0,2}) {
        for (Point a:base[arm]) {
            std::size_t index=0;
            kernel.roots(a,base[arm+1],true,[&](Point got) {
                auto b=base[arm+1][index/2];
                auto expected=add(a,index%2?neg(b):b);
                assert(expected==got && expected!=infinity && got.y);
                assert(s3(a.x,b.x,got.x)==0);
                ++index;
            });
            assert(index==2*base[arm+1].size());
        }
    }
}
Word brute_small(const Bases& base,Point target) {
    std::array<std::vector<Point>,2> halves;
    for (std::size_t arm:{0,2})
        for (Point a:base[arm]) for (Point b:base[arm+1])
            for (Point sa:{a,neg(a)}) for (Point sb:{b,neg(b)})
                halves[arm/2].push_back(add(sa,sb));
    Word count=0;
    for (Point right:halves[1]) {
        auto missing=add(target,neg(right));
        count+=std::count(halves[0].begin(),halves[0].end(),missing);
    }
    return count;
}

int main() {
    std::cout<<"B,base,target,rx,ry,mode,signed_hits,peak_keys,blocks,field_mults,field_inversions,root_visits,complement_visits,wall_ns\n";
    for (std::size_t b:{8,16,32,64,128}) for (bool random:{false,true}) {
        auto base=make_bases(b,random);
        validate_root_grid(base);
        auto planted=add(add(base[0][0],base[1][0]),add(base[2][0],base[3][0]));
        std::array<Point,6> targets{planted,add(base[2][0],base[3][0]),scalar(2),scalar(17),scalar(41),scalar(20261117)};
        for (std::size_t target=0;target<targets.size();target++) {
            std::array<Summary,3> result;
            for (int turn=0;turn<3;turn++) {
                int mode=(turn+target)%3;
                result[mode]=query(base,targets[target],mode);
            }
            assert(result[0].hits==result[1].hits && result[0].hits==result[2].hits);
            assert(result[1].peak_keys*2==result[0].peak_keys);
            assert(result[2].peak_keys<=8*b);
            if (!target) assert(result[0].hits>0);
            if (b==8) assert(brute_small(base,targets[target])==result[0].hits);
            for (std::size_t mode=0;mode<result.size();mode++) {
                auto [hits,peak,blocks,w,ns]=result[mode];
                std::cout<<b<<','<<(random?"random":"interval")<<','<<target<<','
                         <<targets[target].x<<','<<targets[target].y<<','<<mode<<','
                         <<hits<<','<<peak<<','<<blocks<<','<<w.mults<<','<<w.inversions<<','
                         <<w.root_visits<<','<<w.complement_visits<<','<<ns<<'\n';
            }
        }
    }
}
