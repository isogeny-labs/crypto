// Separate arithmetic backend: OpenSSL EC, not the producer's group law.
// Finite small-prime witness/rejection replay; no execution-proof standing.
#include <openssl/bn.h>
#include <openssl/ec.h>
#include <algorithm>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <memory>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using I=std::int64_t;
void need(bool b,const char* msg){if(!b)throw std::runtime_error(msg);}
I residue(I x,I p){x%=p;return x<0?x+p:x;}
bool is_prime(I n){if(n<2)return false;for(I d=2;d*d<=n;++d)if(n%d==0)return false;return true;}
using Number=std::unique_ptr<BIGNUM,decltype(&BN_free)>;
Number number(I n){need(n>=0,"negative BN input");Number x(BN_new(),BN_free);need(x && BN_set_word(x.get(),static_cast<BN_ULONG>(n)),"BN allocation");return x;}
using Point=std::unique_ptr<EC_POINT,decltype(&EC_POINT_free)>;
using Group=std::unique_ptr<EC_GROUP,decltype(&EC_GROUP_free)>;
using Context=std::unique_ptr<BN_CTX,decltype(&BN_CTX_free)>;
std::map<std::string,I> coordinates(const std::string& text){
    std::vector<std::string> t;std::string token;
    auto flush=[&](){if(!token.empty()){t.push_back(token);token.clear();}};
    for(char c:text){if(c=='('||c==')'){flush();t.emplace_back(1,c);}else if(c==' '||c=='\t'||c=='\r'||c=='\n')flush();else token+=c;}flush();
    std::set<std::string> wanted{"ax","ay","bx","by","cx","cy"};std::map<std::string,I> result;
    for(std::size_t i=0;i+6<t.size();++i)if(t[i]=="define-fun"&&wanted.contains(t[i+1])){
        need(t[i+2]=="("&&t[i+3]==")","coordinate arity");std::size_t j=i+4;
        if(t[j]=="Int")++j;
        else {need(j+4<t.size()&&t[j]=="("&&t[j+1]=="_"&&t[j+2]=="BitVec"&&t[j+4]==")","coordinate sort");I width=std::stoll(t[j+3]);need(width>0&&width<=63,"coordinate width");j+=5;}
        need(j<t.size(),"coordinate value absent");I value=0;
        if(t[j].starts_with("#x"))value=std::stoll(t[j].substr(2),nullptr,16);
        else if(t[j].starts_with("#b"))value=std::stoll(t[j].substr(2),nullptr,2);
        else if(t[j]=="("){need(j+4<t.size()&&t[j+1]=="_"&&t[j+2].starts_with("bv")&&t[j+4]==")","coordinate numeral");value=std::stoll(t[j+2].substr(2));j+=4;}
        else value=std::stoll(t[j]);
        need(j+1<t.size()&&t[j+1]==")","nonliteral coordinate model");need(result.emplace(t[i+1],value).second,"duplicate coordinate");
    }
    need(result.size()==6,"missing coordinates");return result;
}
int main(int argc,char** argv){
    try{
        need(argc==10,"usage: openssl-replay P A B ELL BASE TARGET_X TARGET_Y TARGET_INF SOLVER_OUTPUT");
        I p=std::stoll(argv[1]),a=std::stoll(argv[2]),b=std::stoll(argv[3]),ell=std::stoll(argv[4]);
        need(p>=101&&p<=65521&&is_prime(p)&&ell>3&&ell<2*p&&is_prime(ell),"prime/subgroup domain");
        need(a>=0&&a<p&&b>0&&b<p&&residue(b*(a*a-4*b),p)!=0,"singular or noncanonical model");
        Context ctx(BN_CTX_new(),BN_CTX_free);need(static_cast<bool>(ctx),"BN context");auto bp=number(p),three=number(3);
        Number inv3(BN_mod_inverse(nullptr,three.get(),bp.get(),ctx.get()),BN_free);need(static_cast<bool>(inv3),"inverse 3");
        I v=static_cast<I>(BN_get_word(inv3.get())),shift=residue(a*v,p),inv27=residue(residue(v*v,p)*v,p);
        auto short_a=number(residue(b-a*a*v,p));
        auto short_b=number(residue(2*residue(residue(a*a,p)*a,p)*inv27-a*b*v,p));
        Group group(EC_GROUP_new_curve_GFp(bp.get(),short_a.get(),short_b.get(),ctx.get()),EC_GROUP_free);need(static_cast<bool>(group),"OpenSSL curve construction");
        auto fresh=[&](){Point q(EC_POINT_new(group.get()),EC_POINT_free);need(static_cast<bool>(q),"point allocation");return q;};
        auto affine=[&](I x,I y){need(x>=0&&x<p&&y>=0&&y<p,"noncanonical point");auto q=fresh();auto bx=number(residue(x+shift,p)),by=number(y);need(EC_POINT_set_affine_coordinates(group.get(),q.get(),bx.get(),by.get(),ctx.get())==1,"point not on OpenSSL curve");return q;};
        auto bnell=number(ell);
        auto subgroup=[&](const Point& q){auto z=fresh();need(EC_POINT_mul(group.get(),z.get(),nullptr,q.get(),bnell.get(),ctx.get())==1,"OpenSSL scalar multiplication");need(EC_POINT_is_at_infinity(group.get(),z.get())==1,"point outside subgroup");};
        std::ifstream base_file(argv[5]);need(base_file.good(),"base file unavailable");std::string line;std::getline(base_file,line);need(line=="column,source_x,source_y,model_x,model_y","base schema");
        std::vector<std::pair<I,I>> base;std::vector<Point> points;std::set<I> xs;
        while(std::getline(base_file,line)){std::replace(line.begin(),line.end(),',',' ');std::istringstream row(line);I col,sx,sy,x,y;std::string extra;need(static_cast<bool>(row>>col>>sx>>sy>>x>>y)&&!(row>>extra),"base row");need(col==static_cast<I>(base.size())&&y>0&&xs.insert(x).second,"duplicate/invalid base");auto q=affine(x,y);subgroup(q);base.emplace_back(x,y);points.push_back(std::move(q));}
        need(base.size()>=3,"insufficient base");
        I tx=std::stoll(argv[6]),ty=std::stoll(argv[7]),inf=std::stoll(argv[8]);need(inf==0||inf==1,"target infinity flag");
        auto target=fresh();if(inf){need(tx==0&&ty==0,"noncanonical infinity");need(EC_POINT_set_to_infinity(group.get(),target.get())==1,"infinity construction");}else target=affine(tx,ty);subgroup(target);
        std::ifstream model(argv[9]);need(model.good(),"solver output missing");std::string text((std::istreambuf_iterator<char>(model)),{});
        if(text.starts_with("sat\n")){
            auto values=coordinates(text);std::set<I> used;auto sum=fresh();need(EC_POINT_set_to_infinity(group.get(),sum.get())==1,"sum initialization");
            for(std::string prefix:{"a","b","c"}){I x=values.at(prefix+"x"),y=values.at(prefix+"y");need(used.insert(x).second,"repeated witness sign orbit");need(std::any_of(base.begin(),base.end(),[&](auto q){return q.first==x&&(q.second==y||p-q.second==y);}),"witness outside base");auto q=affine(x,y);subgroup(q);need(EC_POINT_add(group.get(),sum.get(),sum.get(),q.get(),ctx.get())==1,"OpenSSL witness addition");}
            need(EC_POINT_cmp(group.get(),sum.get(),target.get(),ctx.get())==0,"OpenSSL witness sum differs from target");
        }else if(text.starts_with("unsat\n")){
            for(std::size_t i=0;i<points.size();++i)for(std::size_t j=i+1;j<points.size();++j)for(std::size_t k=j+1;k<points.size();++k)for(unsigned signs=0;signs<8;++signs){
                auto sum=fresh();need(EC_POINT_set_to_infinity(group.get(),sum.get())==1,"enumeration initialization");unsigned bit=0;
                for(auto index:{i,j,k}){auto q=fresh();need(EC_POINT_copy(q.get(),points[index].get())==1,"point copy");if((signs>>bit++)&1U)need(EC_POINT_invert(group.get(),q.get(),ctx.get())==1,"point negation");need(EC_POINT_add(group.get(),sum.get(),sum.get(),q.get(),ctx.get())==1,"enumerated addition");}
                need(EC_POINT_cmp(group.get(),sum.get(),target.get(),ctx.get())!=0,"UNSAT refuted by independent OpenSSL enumeration");
            }
        }else throw std::runtime_error("unknown/timeout/error is not a replay certificate");
        std::cout<<"OpenSSL arithmetic replay accepted (producer invocation; independent execution standing unverified)\n";return 0;
    }catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 1;}
}
