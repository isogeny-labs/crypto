// Independent small-prime polynomial/Cantor implementation and exhaustive
// checker. The primary Rust arithmetic is unmodified upstream code.
// This is a same-host check, not a signed verdict or a second-machine run.
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>
using Poly=std::vector<int>;
using Divisor=std::pair<Poly,Poly>;
static int modulus=0;
static void require(bool b,const std::string& s) {if(!b)throw std::runtime_error(s);}
static int mod(int a) {a%=modulus;return a<0?a+modulus:a;}
static void trim(Poly& a) {while(!a.empty()&&a.back()==0)a.pop_back();}
static int degree(const Poly& a) {return static_cast<int>(a.size())-1;}
static int inv(int a) {for(int b=1;b<modulus;++b)if(mod(a*b)==1)return b;throw std::runtime_error("inverse of zero");}
static Poly scale(Poly a,int b) {for(int&x:a)x=mod(x*b);trim(a);return a;}
static Poly plus(const Poly&a,const Poly&b) {Poly c(std::max(a.size(),b.size()));for(size_t i=0;i<c.size();++i)c[i]=mod((i<a.size()?a[i]:0)+(i<b.size()?b[i]:0));trim(c);return c;}
static Poly minus(const Poly&a,const Poly&b) {return plus(a,scale(b,-1));}
static Poly times(const Poly&a,const Poly&b) {
    if(a.empty()||b.empty())return {};
    Poly c(a.size()+b.size()-1);
    for(size_t i=0;i<a.size();++i)for(size_t j=0;j<b.size();++j)c[i+j]=mod(c[i+j]+a[i]*b[j]);
    trim(c);return c;
}
static std::pair<Poly,Poly> division(Poly a,const Poly& b) {
    require(!b.empty(),"zero divisor polynomial");
    Poly q(degree(a)>=degree(b)?degree(a)-degree(b)+1:0);
    while(!a.empty()&&degree(a)>=degree(b)) {
        int shift=degree(a)-degree(b),c=mod(a.back()*inv(b.back()));q[shift]=c;
        for(size_t j=0;j<b.size();++j)a[j+shift]=mod(a[j+shift]-c*b[j]);
        trim(a);
    }trim(q);return {q,a};
}
static Poly quotient(const Poly&a,const Poly&b) {auto qr=division(a,b);require(qr.second.empty(),"nonexact Cantor quotient");return qr.first;}
static Poly monic(Poly a) {require(!a.empty(),"normalizing zero polynomial");return scale(a,inv(a.back()));}
struct Bezout {Poly d,s,t;};
static Bezout bezout(Poly a,Poly b) {
    Poly s{1},t{},u{},v{1};
    while(!b.empty()) {
        auto qr=division(a,b);Poly sn=minus(s,times(qr.first,u)),tn=minus(t,times(qr.first,v));
        a=b;b=qr.second;s=u;u=sn;t=v;v=tn;
    }
    if(a.empty())return {{},{},{}};
    int k=inv(a.back());return {scale(a,k),scale(s,k),scale(t,k)};
}
static int eval(const Poly&a,int x) {int y=0;for(auto i=a.rbegin();i!=a.rend();++i)y=mod(y*x+*i);return y;}
static bool smooth(const Poly&u) {if(degree(u)<=1)return true;for(int x=0;x<modulus;++x)if(eval(u,x)==0)return true;return false;}
static bool valid_pair(const Divisor&d,const Poly&f) {
    return !d.first.empty()&&d.first.back()==1&&degree(d.first)<=2&&degree(d.second)<degree(d.first)
        &&division(minus(times(d.second,d.second),f),d.first).second.empty();
}
static bool identity(const Divisor&d) {return d.first==Poly{1}&&d.second.empty();}
static Divisor cantor(const Divisor&a,const Divisor&b,const Poly&f) {
    if(identity(a))return b;
    if(identity(b))return a;
    auto e=bezout(a.first,b.first),c=bezout(e.d,plus(a.second,b.second));
    Poly s1=times(c.s,e.s),s2=times(c.s,e.t),s3=c.t;
    Poly u=quotient(times(a.first,b.first),times(c.d,c.d));
    Poly num=plus(plus(times(times(s1,a.first),b.second),times(times(s2,b.first),a.second)),
        times(s3,plus(times(a.second,b.second),f)));
    Poly v=division(quotient(num,c.d),u).second;
    unsigned reductions=0;
    while(degree(u)>2) {
        const int old_degree=degree(u);u=monic(quotient(minus(f,times(v,v)),u));
        require(degree(u)<old_degree,"Cantor reduction did not decrease degree");
        v=division(scale(v,-1),u).second;require(++reductions<=8,"Cantor reduction bound");
    }
    u=monic(u);Divisor out{u,v};require(valid_pair(out,f),"invalid C++ Cantor output");return out;
}
static bool squarefree(const Poly&f) {
    Poly df(f.size()-1);for(size_t i=1;i<f.size();++i)df[i-1]=mod(static_cast<int>(i)*f[i]);trim(df);
    return degree(bezout(f,df).d)==0;
}
static std::vector<Divisor> all_pairs(const Poly&f) {
    std::vector<Divisor> out{{{1},{}}};
    auto add=[&](Poly u,Poly v){
        trim(v);
        Divisor d{std::move(u),std::move(v)};
        if(valid_pair(d,f))out.push_back(std::move(d));
    };
    for(int a=0;a<modulus;++a)for(int b=0;b<modulus;++b)add({a,1},{b});
    for(int a=0;a<modulus;++a)for(int b=0;b<modulus;++b)
        for(int c=0;c<modulus;++c)for(int d=0;d<modulus;++d)add({a,b,1},{c,d});
    return out;
}
using Fp2=std::pair<int,int>;
static Fp2 ext_add(Fp2 a,Fp2 b) {return {mod(a.first+b.first),mod(a.second+b.second)};}
static Fp2 ext_mul(Fp2 a,Fp2 b) {return {mod(a.first*b.first+2*a.second*b.second),mod(a.first*b.second+a.second*b.first)};}
static Fp2 ext_eval(const Poly&f,Fp2 x) {Fp2 y{0,0};for(auto i=f.rbegin();i!=f.rend();++i)y=ext_add(ext_mul(y,x),{*i,0});return y;}
struct Counts {int n1,n2,ramification,jacobian,smooth;};
static Counts point_counts(const Poly&f) {
    std::vector<int> roots(modulus),ext_roots(modulus*modulus);
    for(int y=0;y<modulus;++y)++roots[mod(y*y)];
    require(roots[2]==0,"quadratic extension parameter2 is square");
    for(int a=0;a<modulus;++a)for(int b=0;b<modulus;++b) {auto z=ext_mul({a,b},{a,b});++ext_roots[z.first+modulus*z.second];}
    int n1=1,n2=1,ramification=0;
    for(int x=0;x<modulus;++x){int fx=eval(f,x);n1+=roots[fx];if(fx==0)++ramification;}
    for(int a=0;a<modulus;++a)for(int b=0;b<modulus;++b){auto z=ext_eval(f,{a,b});n2+=ext_roots[z.first+modulus*z.second];}
    require((n1*n1+n2)%2==0,"nonintegral L-polynomial count");
    int m=n1-1;require((m*m-ramification)%2==0,"nonintegral smooth combinatorial count");
    return {n1,n2,ramification,(n1*n1+n2)/2-modulus,1+m+(m*m-ramification)/2};
}
static bool prime(size_t n) {if(n<2)return false;for(size_t d=2;d*d<=n;++d)if(n%d==0)return false;return true;}
static void token(std::istream&in,const char* expected) {std::string s;require(bool(in>>s)&&s==expected,std::string("expected token ")+expected);}
static Poly read_poly(std::istream&in,size_t maximum) {
    size_t n;require(bool(in>>n)&&n<=maximum,"bad polynomial length");Poly p(n);
    for(int&c:p)require(bool(in>>c)&&c>=0&&c<modulus,"coefficient outside field");
    require(p.empty()||p.back()!=0,"noncanonical polynomial");return p;
}
static void check(const std::string&input,const std::string&output) {
    std::ifstream in(input);require(bool(in),"missing audit input");
    token(in,"RUN");std::string mode;int field;require(bool(in>>field>>mode)&&(field==3||field==5),"bad run field");
    require(mode=="all"||mode=="fixture","bad run mode");modulus=field;
    std::ifstream existing(output);require(!existing.good(),"refuse to overwrite verifier result");
    std::ifstream existing_counts(output+".counts.tsv");
    require(!existing_counts.good(),"refuse to overwrite independent point-count records");
    std::ofstream count_rows(output+".counts.tsv");require(bool(count_rows),"cannot create count records");
    count_rows<<"model p valid n1 n2 rational_ramification point_jacobian_count point_smooth_count\n";
    uint64_t attempted=0,valid=0,addition_checks=0,association_checks=0,subgroup_checks=0;
    uint64_t bound=1;for(unsigned i=0;i<5;++i)bound*=field;if(mode=="fixture")bound=1;
    for(uint64_t index=0;index<bound;++index) {
        token(in,"C");uint64_t model;int p;require(bool(in>>model>>p)&&p==field,"curve field mismatch");
        Poly f(6);uint64_t code=0,multiplier=1;
        for(size_t j=0;j<6;++j){require(bool(in>>f[j])&&f[j]>=0&&f[j]<field,"bad f coefficient");if(j<5){code+=multiplier*f[j];multiplier*=field;}}
        require(f.back()==1&&model==code,"bad monic curve encoding");
        require(mode=="fixture"?model==uint64_t(field*(field-1)):model==index,"missing/out-of-order model");
        if(mode=="fixture")require(f==Poly({0,field-1,0,0,0,1}),"wrong fixture polynomial");
        token(in,"V");int reported_valid;require(bool(in>>reported_valid)&&(reported_valid==0||reported_valid==1),"bad validity flag");
        bool is_valid=squarefree(f);require(is_valid==bool(reported_valid),"squarefreeness disagreement");++attempted;
        if(is_valid) {
            ++valid;auto expected=all_pairs(f);const size_t n=expected.size();size_t reported_n,reported_s;
            uint64_t adds,assocs;token(in,"N");require(bool(in>>reported_n>>reported_s>>adds>>assocs),"bad N record");
            require(n==reported_n&&adds==n*n&&assocs==n*n*n,"wrong count/table counters");
            std::vector<bool> splits(n);size_t smooth_count=0;std::map<Divisor,size_t> ids;
            for(size_t i=0;i<n;++i) {
                token(in,"D");size_t id;require(bool(in>>id)&&id==i,"pair index mismatch");
                Poly u=read_poly(in,3),v=read_poly(in,2);int s;require(bool(in>>s)&&(s==0||s==1),"bad smooth flag");
                require(Divisor(u,v)==expected[i],"complete pair enumeration disagreement");
                splits[i]=smooth(u);require(splits[i]==bool(s),"splitting disagreement");if(splits[i])++smooth_count;
                require(ids.emplace(expected[i],i).second,"duplicate pair");
            }
            require(smooth_count==reported_s,"smooth numerator disagreement");
            Counts counts=point_counts(f);require(counts.jacobian==static_cast<int>(n),"point/L-polynomial count disagrees with full pairs");
            require(counts.smooth==static_cast<int>(smooth_count),"point-combinatorial smooth count disagrees with full pairs");
            count_rows<<model<<' '<<field<<" 1 "<<counts.n1<<' '<<counts.n2<<' '<<counts.ramification<<' '
                <<counts.jacobian<<' '<<counts.smooth<<'\n';
            token(in,"A");std::vector<size_t> table(n*n);
            for(size_t a=0;a<n;++a)for(size_t b=0;b<n;++b) {
                size_t value;require(bool(in>>value)&&value<n,"bad addition index");table[a*n+b]=value;
                Divisor sum=cantor(expected[a],expected[b],f);auto hit=ids.find(sum);
                require(hit!=ids.end()&&hit->second==value,"independent Cantor addition disagreement");++addition_checks;
            }
            for(size_t a=0;a<n;++a) {
                require(table[a]==a&&table[a*n]==a,"identity disagreement");bool has_inverse=false;
                for(size_t b=0;b<n;++b) {
                    require(table[a*n+b]==table[b*n+a],"commutativity disagreement");if(table[a*n+b]==0)has_inverse=true;
                    for(size_t c=0;c<n;++c) {require(table[table[a*n+b]*n+c]==table[a*n+table[b*n+c]],"associativity disagreement");++association_checks;}
                }require(has_inverse,"missing inverse");
            }
            std::map<std::vector<size_t>,std::vector<size_t>> groups;
            for(size_t g=0;g<n;++g) {
                std::set<size_t> members;size_t x=0;
                do {require(members.insert(x).second,"premature cyclic repetition");x=table[x*n+g];require(members.size()<=n,"cyclic walk exceeds model");}while(x!=0);
                groups[{members.begin(),members.end()}].push_back(g);
            }
            token(in,"G");size_t group_count;require(bool(in>>group_count)&&group_count==groups.size(),"subgroup count mismatch");
            for(auto const&[members,generators]:groups) {
                token(in,"H");size_t order,sm,ng;int proper,prime_flag,char_flag;
                require(bool(in>>order>>sm>>ng>>proper>>prime_flag>>char_flag),"bad subgroup record");
                require(order==members.size()&&ng==generators.size(),"subgroup size mismatch");
                size_t actual_smooth=0;for(size_t i:members)if(splits[i])++actual_smooth;
                require(sm==actual_smooth&&proper==int(order<n)&&prime_flag==int(prime(order))&&char_flag==int(order==size_t(field)),"subgroup stratum/smooth disagreement");
                for(size_t g:generators){size_t observed;require(bool(in>>observed)&&observed==g,"generator list mismatch");}
                for(size_t m:members){size_t observed;require(bool(in>>observed)&&observed==m,"membership list mismatch");}
                ++subgroup_checks;
            }
            if(model==uint64_t(field*(field-1)))require(n==size_t(field==3?8:16)&&smooth_count==size_t(field==3?7:16),"Lean-backed positive count control failed");
            if(field==3&&model==1) {
                require(n==10&&smooth_count==8,"source-known Stoll count control failed");
                auto generator=ids.find(Divisor{{0,1},{1}});require(generator!=ids.end(),"Stoll generator missing");
                std::set<size_t> cycle;size_t x=0;
                do {require(cycle.insert(x).second,"Stoll cycle repeated early");x=table[x*n+generator->second];}while(x!=0);
                require(cycle.size()==5,"source-known Stoll order is not5");
                for(size_t member:cycle)require(splits[member],"Stoll cyclic element is nonsmooth");
            }
        } else count_rows<<model<<' '<<field<<" 0 0 0 0 0 0\n";
        token(in,"E");
    }
    token(in,"DONE");uint64_t declared;require(bool(in>>declared)&&declared==attempted,"incomplete/miscounted footer");
    if(mode=="all")require(valid==uint64_t(field==3?162:2500),"declared squarefree-quintic count control failed");
    std::string extra;require(!(in>>extra),"extra records after footer");
    count_rows.flush();require(bool(count_rows),"point-count record write failed");
    std::ofstream out(output);require(bool(out),"cannot create verifier result");
    out<<"{\"schema\":\"mumford.cpp.check/v1\",\"status\":\"match\",\"p\":"<<field
       <<",\"models_attempted\":"<<attempted<<",\"valid_models\":"<<valid
       <<",\"cantor_additions_checked\":"<<addition_checks<<",\"associativity_checks\":"<<association_checks
       <<",\"distinct_subgroups_checked\":"<<subgroup_checks
       <<",\"scope\":\"Same-host different-language full polynomial enumeration, independent Cantor implementation, point-count controls and complete finite group/subgroup checks. No signed or independent-host standing.\"}\n";
    require(bool(out),"verifier result write failed");
}
int main(int argc,char**argv) {
    try {require(argc==3,"usage: cpp_counts AUDIT_TSV RESULT_JSON");check(argv[1],argv[2]);return 0;}
    catch(const std::exception&e){std::cerr<<"C++ Mumford verification failed: "<<e.what()<<'\n';return 1;}
}
