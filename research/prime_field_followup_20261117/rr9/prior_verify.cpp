#include <algorithm>
#include <array>
#include <cassert>
#include <chrono>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <stdexcept>
#include <string>
#include <vector>

using Int=std::int64_t;
using Exp=std::uint32_t;
using Poly=std::map<Exp,Int>;
using Point=std::array<Int,2>;
constexpr Int p=2147483647, curve_a=847514846, curve_b=2087444724;
constexpr Point infinity{-1,-1};
constexpr std::array<Int,8> xs{0,1,2,3,5,6,8,9};
constexpr std::array<const char*,7> variables{"a0","a1","a2","a3","v0","v1","v2"};

Int mod(Int x) {
    auto y=x%p;
    return y<0?y+p:y;
}

Int power(Int x, Int exponent) {
    Int y=1;
    for (;exponent;exponent>>=1,x=mod(x*x)) if (exponent&1) y=mod(y*x);
    return y;
}

void accumulate(Poly& dst, Exp e, Int c) {
    Int& v=dst[e];
    v=mod(v+c);
    if (!v) dst.erase(e);
}
Poly constant(Int c) { Poly out; accumulate(out,0,c); return out; }
Poly variable(int i) { return {{1U<<(4*i),1}}; }

void scaled_add(Poly& dst,const Poly& src,Int c) {
    c=mod(c);
    for (auto [e,v]:src) accumulate(dst,e,c*v);
}

Poly multiply(const Poly& a,const Poly& b) {
    Poly c;
    for (auto [e,s]:a) for (auto [f,t]:b) {
        for (int i=0;i<7;++i) assert(((e>>(4*i))&15)+((f>>(4*i))&15)<16);
        accumulate(c,e+f,s*t);
    }
    return c;
}

Poly read_poly(const std::string& text) {
    Poly result;
    for (size_t start=0;start<text.size();) {
        Int sign=1;
        if (text[start]=='-') { sign=-1; ++start; }
        else if (text[start]=='+') ++start;
        auto end=text.find_first_of("+-",start);
        if (end==std::string::npos) end=text.size();
        Exp exponent=0;
        Int coef=sign;
        for (size_t pos=start;pos<end;) {
            auto stop=std::min(end,text.find('*',pos));
            auto factor=text.substr(pos,stop-pos);
            if (factor.front()>='0' && factor.front()<='9') {
                size_t consumed=0;
                coef=mod(coef*std::stoll(factor,&consumed));
                assert(consumed==factor.size());
            } else {
                auto id=std::find(variables.begin(),variables.end(),factor.substr(0,2));
                assert(id!=variables.end());
                int power=1;
                if (factor.size()!=2) {
                    assert(factor[2]=='^');
                    power=std::stoi(factor.substr(3));
                }
                assert(power>0 && power<16);
                exponent+=static_cast<Exp>(power)<<(4*(id-variables.begin()));
            }
            pos=stop+1;
        }
        accumulate(result,exponent,coef);
        start=end;
    }
    return result;
}

std::array<Poly,8> equations(Point R,bool evaluation_presentation) {
    auto [r,s]=R;
    std::array<Poly,9> norm;
    std::array<Poly,4> a{variable(0),variable(1),variable(2),variable(3)};
    std::array<Poly,4> v{variable(4),variable(5),variable(6),constant(1)};
    std::array<Int,3> field{mod(r*r+curve_a),r,1};
    for (int i=0;i<4;++i) for (int j=0;j<4;++j) {
        auto vv=multiply(v[i],v[j]);
        for (int k=0;k<3;++k) scaled_add(norm[i+j+k],vv,field[k]);
        auto aa=multiply(a[i],a[j]);
        scaled_add(norm[i+j],aa,r);
        scaled_add(norm[i+j+1],aa,-1);
        scaled_add(norm[i+j],multiply(a[i],v[j]),-2*s);
    }
    std::vector<Int> h{1};
    for (Int x:xs) {
        std::vector<Int> next(h.size()+1);
        for (size_t i=0;i<h.size();++i) {
            next[i]=mod(next[i]-x*h[i]);
            next[i+1]=mod(next[i+1]+h[i]);
        }
        h=std::move(next);
    }
    for (int i=0;i<9;++i) accumulate(norm[i],0,-h[i]);
    assert(norm[8].empty());
    std::array<Poly,8> out;
    for (int i=0;i<8;++i) {
        if (!evaluation_presentation) out[i]=norm[i];
        else for (int j=0;j<8;++j) scaled_add(out[i],norm[j],power(xs[i],j));
    }
    return out;
}

Point add(Point x,Point y) {
    if (x==infinity) return y;
    if (y==infinity) return x;
    if (x[0]==y[0] && !mod(x[1]+y[1])) return infinity;
    Int num=x==y?mod(3*mod(x[0]*x[0])+curve_a):mod(y[1]-x[1]);
    Int den=x==y?mod(2*x[1]):mod(y[0]-x[0]);
    Int m=mod(num*power(den,p-2));
    Int out=mod(m*m-x[0]-y[0]);
    return {out,mod(m*(x[0]-out)-x[1])};
}

Point negate(Point x) {
    return x==infinity?x:Point{x[0],mod(-x[1])};
}

std::array<Point,8> base_points() {
    std::array<Point,8> base;
    for (size_t i=0;i<xs.size();++i) {
        Int x=xs[i],rhs=mod(mod(x*x)*x+mod(curve_a*x)+curve_b);
        Int y=power(rhs,(p+1)/4);
        assert(y && mod(y*y)==rhs);
        base[i]={x,std::min(y,p-y)};
    }
    return base;
}

bool mitm(Point target,const std::array<Point,8>& base) {
    auto sum=[](const auto& b,int start,int mask) {
        Point v=infinity;
        for (int i=0;i<4;++i) v=add(v,mask>>i&1?negate(b[start+i]):b[start+i]);
        return v;
    };
    std::array<Point,16> left;
    for (int mask=0;mask<16;++mask) left[mask]=sum(base,0,mask);
    for (int mask=0;mask<16;++mask) {
        auto missing=add(target,negate(sum(base,4,mask)));
        if (std::find(left.begin(),left.end(),missing)!=left.end()) return true;
    }
    return false;
}

int main(int argc,char** argv) {
    assert(argc==5);
    std::ifstream input(argv[1]);
    assert(input);
    Point R{std::stoll(argv[2]),std::stoll(argv[3])};
    assert(mod(R[1]*R[1])==mod(mod(R[0]*R[0])*R[0]+mod(curve_a*R[0])+curve_b));
    auto eq=equations(R,std::stoi(argv[4])!=0);
    std::array<Poly,8> w;
    std::array<bool,8> inputs{}, multipliers{};
    std::array<bool,7> values{};
    std::array<Int,7> solution{};
    bool unit=false, positive=false;
    for (std::string line; std::getline(input,line);) {
        assert(line.find("? error")==std::string::npos);
        if (line=="UNIT_REPLAY_OK") unit=true;
        if (line=="POSITIVE_EXTRACTED") positive=true;
        if (line.starts_with("VALUE")) {
            size_t i=line[5]-'1';
            assert(i<7 && !values[i]);
            values[i]=true;
            solution[i]=mod(std::stoll(line.substr(7)));
        }
        if (line.size()<4 || line[2]!=' ' || (line[0]!='Q' && line[0]!='W')) continue;
        size_t i=line[1]-'1';
        assert(i<8);
        Poly poly=read_poly(line.substr(3));
        if (line[0]=='Q') {
            assert(!inputs[i] && poly==eq[i]);
            inputs[i]=true;
        } else {
            assert(!multipliers[i]);
            multipliers[i]=true;
            w[i]=std::move(poly);
        }
    }
    assert(unit!=positive);
    auto base=base_points();
    auto start=std::chrono::steady_clock::now();
    bool generic=mitm(R,base);
    auto end=std::chrono::steady_clock::now();
    assert(generic==positive);
    auto generic_us=std::chrono::duration<double,std::micro>(end-start).count();
    if (unit) {
        Poly sum;
        size_t terms=0;
        int q_degree=0;
        for (int i=0;i<8;++i) {
            assert(inputs[i] && multipliers[i]);
            scaled_add(sum,multiply(w[i],eq[i]),1);
            terms+=w[i].size();
            for (auto [e,c]:w[i]) {
                assert(c);
                int degree=0;
                for (int slot=0;slot<7;++slot) degree+=(e>>(4*slot))&15;
                q_degree=std::max(q_degree,degree);
            }
        }
        assert(sum==constant(1));
        std::cout<<"unit replay: "<<terms<<" multiplier terms, degree "<<q_degree
                 <<", generic control "<<generic_us<<" us\n";
    } else {
        assert(std::all_of(values.begin(),values.end(),[](bool x){return x;}));
        for (const auto& f:eq) {
            Int sum=0;
            for (auto [e,c]:f) {
                for (int i=0;i<7;++i) c=mod(c*power(solution[i],(e>>(4*i))&15));
                sum=mod(sum+c);
            }
            assert(sum==0);
        }
        Point sum=infinity;
        for (Point pt:base) {
            Int x=pt[0], aa=0,vv=1;
            for (int i=3;i>=0;--i) aa=mod(aa*x+solution[i]);
            for (int i=6;i>=4;--i) vv=mod(vv*x+solution[i]);
            auto test=[&](Int y) { return mod(mod(x-R[0])*aa+mod(y+R[1])*vv)==0; };
            bool plus=test(pt[1]), minus=test(mod(-pt[1]));
            assert(plus!=minus);
            sum=add(sum,plus?pt:negate(pt));
        }
        assert(sum==R);
        std::cout<<"positive coefficients and signed group sum replay; generic control "<<generic_us<<" us\n";
    }
}
