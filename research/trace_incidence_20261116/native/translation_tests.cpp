// Bounded native development checks. No timings or benchmark admission.
#include "api.hpp"
#include <algorithm>
#include <iostream>
#include <stdexcept>
#include <vector>

using namespace paper_ec;
namespace {
Word mul(Word a, Word b, Word p) { return static_cast<Word>(__uint128_t(a) * b % p); }
Word power(Word a, Word n, Word p) { Word r=1; while(n) { if(n&1)r=mul(r,a,p); a=mul(a,a,p); n>>=1; } return r; }
Word evaluate(const Coefficients& f, Word x, Word p) { Word r=0; for(auto i=f.rbegin();i!=f.rend();++i)r=(mul(r,x,p)+*i)%p; return r; }
Coefficients product(const Coefficients& a,const Coefficients& b,Word p) {
    Coefficients r(a.size()+b.size()-1,0);
    for(std::size_t i=0;i<a.size();++i)for(std::size_t j=0;j<b.size();++j)r[i+j]=(r[i+j]+mul(a[i],b[j],p))%p;
    return r;
}
Component encode(const Curve& c,const std::vector<Point>& points) {
    Coefficients u{1},v(points.size(),0);
    for(const Point& P:points)u=product(u,{(c.p-P.x)%c.p,1},c.p);
    for(std::size_t i=0;i<points.size();++i) {
        Coefficients basis{1};Word denominator=1;
        for(std::size_t j=0;j<points.size();++j)if(i!=j) {
            basis=product(basis,{(c.p-points[j].x)%c.p,1},c.p);
            denominator=mul(denominator,(points[i].x+c.p-points[j].x)%c.p,c.p);
        }
        if(!denominator)throw std::runtime_error("fixture repeated abscissa");
        Word factor=mul(points[i].y,power(denominator,c.p-2,c.p),c.p);
        for(std::size_t j=0;j<basis.size();++j)v[j]=(v[j]+mul(factor,basis[j],c.p))%c.p;
    }
    while(!v.empty()&&!v.back())v.pop_back();
    return {u,v};
}
std::vector<Point> decode(const Curve& c,const Divisor& d) {
    std::vector<Point> result;
    if(d.infinity)result.push_back({});
    for(const auto& component:d.components) {
        if(component.u.empty()||component.u.back()!=1)throw std::runtime_error("nonmonic output");
        std::size_t roots=0;
        for(Word x=0;x<c.p;++x)if(!evaluate(component.u,x,c.p)) {
            Point point{x,evaluate(component.v,x,c.p),false};
            if(!on_curve(c,point))throw std::runtime_error("off-curve output");
            result.push_back(point);++roots;
        }
        if(roots+1!=component.u.size())throw std::runtime_error("output not split squarefree");
    }
    std::sort(result.begin(),result.end());return result;
}
Point reference_add(const Curve& c,Point A,Point B) {
    if(A.infinity)return B;
    if(B.infinity)return A;
    if(A.x==B.x&&(A.y+B.y)%c.p==0)return {};
    Word numerator,denominator;
    if(A==B) { numerator=(mul(3,mul(A.x,A.x,c.p),c.p)+c.a)%c.p;denominator=2*A.y%c.p; }
    else { numerator=(B.y+c.p-A.y)%c.p;denominator=(B.x+c.p-A.x)%c.p; }
    Word slope=mul(numerator,power(denominator,c.p-2,c.p),c.p);
    Word x=(mul(slope,slope,c.p)+2*c.p-A.x-B.x)%c.p;
    return {x,(mul(slope,(A.x+c.p-x)%c.p,c.p)+c.p-A.y)%c.p,false};
}
void require(bool condition,const char* message) { if(!condition)throw std::runtime_error(message); }
}
int main() {
    try {
        std::size_t translations=0,point_checks=0,invalid_checks=0;
        Stats stats;
        for(const Curve c:std::vector<Curve>{{5,3,0},{7,2,3},{17,2,0},{83,31,21},{97,2,3}}) {
            std::vector<Point> points{{}};
            for(Word x=0;x<c.p;++x)for(Word y=0;y<c.p;++y)
                if(on_curve(c,{x,y,false}))points.push_back({x,y,false});
            for(const Point& P:points) {
                Point repeated;
                for(Word n=0;n<=9;++n) {
                    require(point_mul(c,n,P)==repeated,"scalar multiplication mismatch");++point_checks;
                    repeated=reference_add(c,repeated,P);
                }
                require(point_add(c,P,point_neg(c,P)).infinity,"inverse point mismatch");++point_checks;
            }
            for(std::size_t seed=0;seed<9;++seed) {
                std::vector<Point> selected;std::vector<Word> xs;
                for(std::size_t j=1;j<points.size();++j) {
                    Point P=points[1+(j+seed*7)%(points.size()-1)];
                    if(std::find(xs.begin(),xs.end(),P.x)!=xs.end())continue;
                    selected.push_back(P);xs.push_back(P.x);
                    if(selected.size()==std::min<std::size_t>(seed+1,7))break;
                }
                Divisor input{{encode(c,selected)},bool(seed&1)};
                // All rational x lifts on F5 give the characteristic-p degree case.
                if(c.p==5&&seed==8)require(selected.size()==5,"missing d=p fixture");
                for(const Point& Q:points) {
                    std::vector<Point> expected;
                    for(const Point& P:selected)expected.push_back(reference_add(c,P,Q));
                    if(input.infinity)expected.push_back(Q);
                    std::sort(expected.begin(),expected.end());
                    for(TranslationMethod method:{TranslationMethod::Trace,TranslationMethod::Dense}) {
                        Divisor result=translate(c,input,Q,method,0,&stats);
                        require(decode(c,result)==expected,"translated point mismatch");++translations;
                    }
                }
            }
            // Separate simple components sharing x with opposite ordinates.
            auto found=std::find_if(points.begin()+1,points.end(),[](Point P){return P.y!=0;});
            require(found!=points.end(),"missing non-two-torsion point");
            Point P=*found;Divisor input{{encode(c,{P}),encode(c,{point_neg(c,P)})},true};
            for(const Point& Q:points) {
                std::vector<Point> expected{reference_add(c,P,Q),reference_add(c,point_neg(c,P),Q),Q};
                std::sort(expected.begin(),expected.end());
                for(TranslationMethod method:{TranslationMethod::Trace,TranslationMethod::Dense}) {
                    require(decode(c,translate(c,input,Q,method,0,&stats))==expected,"multicomponent mismatch");++translations;
                }
            }
        }
        for(const auto& bad:std::vector<std::pair<Curve,Divisor>>{
                {{3,1,1},{}},{{5,0,0},{}},{{83,31,21},{{{{0,0,1},{}}},false}},
                {{83,31,21},{{{{1,2},{}}},false}},{{83,31,21},{{{{1,1},{0}}},false}}}) {
            bool rejected=false;
            try { (void)translate(bad.first,bad.second,{},TranslationMethod::Trace,0); }
            catch(const std::invalid_argument&) { rejected=true; }
            require(rejected,"invalid fixture accepted");++invalid_checks;
        }
        // Word-width arithmetic without any full-field enumeration.
        Curve large{2305843009213693951ULL,0,7};Point G{1,4294967296ULL,false};
        // Find a small known point with p=3 mod4 using the separate scalar power.
        bool found=false;
        for(Word x=0;x<100;++x) {
            Word h=(mul(x,mul(x,x,large.p),large.p)+7)%large.p;
            Word y=power(h,(large.p+1)/4,large.p);
            if(mul(y,y,large.p)==h) { G={x,y,false};found=true;break; }
        }
        require(found,"large-prime point fixture missing");
        for(Word n=0;n<32;++n)require(on_curve(large,point_mul(large,n,G)),"word-width scalar mismatch");
        std::cout<<"{\"status\":\"development_checks_passed\",\"translations\":"<<translations
                 <<",\"point_checks\":"<<point_checks<<",\"invalid_inputs\":"<<invalid_checks
                 <<",\"collision_lifts\":"<<stats.counts["collision_lift_calls"]
                 <<",\"exceptional_points\":"<<stats.counts["translation_exceptional_points"]
                 <<",\"nonsquare_trials\":"<<stats.counts["nonsquare_search_trials"]<<"}\n";
        return 0;
    } catch(const std::exception& error) { std::cerr<<error.what()<<'\n';return 1; }
}
