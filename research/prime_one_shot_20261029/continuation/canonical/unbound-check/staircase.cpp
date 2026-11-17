// Independent printed-GB leading-monomial/staircase checker.
// It does not certify Buchberger's criterion or equality with the source ideal.
#include <algorithm>
#include <cctype>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <limits>
#include <map>
#include <queue>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

using Mono=std::vector<unsigned>;
using Poly=std::map<Mono,std::uint64_t>;
std::string trim(std::string s){auto a=s.find_first_not_of(" \r\n\t");return a==std::string::npos?"":s.substr(a,s.find_last_not_of(" \r\n\t")-a+1);}
std::vector<std::string> split(const std::string&s,char c){std::vector<std::string>v;std::stringstream in(s);std::string x;while(std::getline(in,x,c)){x=trim(x);if(!x.empty())v.push_back(x);}return v;}
unsigned degree(const Mono&m){unsigned d=0;for(unsigned e:m){if(e>100000||d>100000-e)throw std::runtime_error("degree cap exceeded");d+=e;}return d;}
bool greater_grevlex(const Mono&a,const Mono&b){unsigned da=degree(a),db=degree(b);if(da!=db)return da>db;for(std::size_t i=a.size();i-->0;)if(a[i]!=b[i])return a[i]<b[i];return false;}
bool divides(const Mono&a,const Mono&b){for(std::size_t i=0;i<a.size();++i)if(a[i]>b[i])return false;return true;}
struct TermParser {
    const std::string&s;const std::map<std::string,std::size_t>&indices;std::uint64_t p;std::size_t n,pos=0;
    void white(){while(pos<s.size()&&std::isspace(static_cast<unsigned char>(s[pos])))++pos;}
    bool take(char c){white();if(pos<s.size()&&s[pos]==c){++pos;return true;}return false;}
    std::uint64_t number(){white();if(pos==s.size()||!std::isdigit(static_cast<unsigned char>(s[pos])))throw std::runtime_error("expected decimal number");std::uint64_t v=0;while(pos<s.size()&&std::isdigit(static_cast<unsigned char>(s[pos]))){if(v>1000000000)throw std::runtime_error("numeric token cap");v=10*v+static_cast<unsigned>(s[pos++]-'0');}return v;}
    Poly parse(){
        Poly out;bool first=true;
        while(true){
            white();if(pos==s.size())break;
            int sign=1;if(take('+')){}else if(take('-'))sign=-1;else if(!first)throw std::runtime_error("missing term separator");
            first=false;Mono monomial(n,0);std::uint64_t coefficient=1;bool factor=false;
            do{
                white();if(pos==s.size())throw std::runtime_error("missing factor");
                if(std::isdigit(static_cast<unsigned char>(s[pos]))){coefficient=(coefficient*(number()%p))%p;}
                else{
                    std::size_t start=pos;while(pos<s.size()&&(std::isalnum(static_cast<unsigned char>(s[pos]))||s[pos]=='_'))++pos;
                    if(start==pos)throw std::runtime_error("unsupported polynomial token");
                    auto it=indices.find(s.substr(start,pos-start));if(it==indices.end())throw std::runtime_error("unknown printed variable");
                    std::uint64_t exponent=1;if(take('^'))exponent=number();
                    if(exponent>100000||monomial[it->second]>100000-exponent)throw std::runtime_error("exponent cap exceeded");
                    monomial[it->second]+=static_cast<unsigned>(exponent);
                }
                factor=true;
            }while(take('*'));
            if(!factor)throw std::runtime_error("empty term");
            if(sign<0&&coefficient)coefficient=p-coefficient;
            auto&v=out[monomial];v=(v+coefficient)%p;if(v==0)out.erase(monomial);
            white();if(pos==s.size())break;if(s[pos]!='+'&&s[pos]!='-')throw std::runtime_error("trailing unsupported syntax");
        }
        return out;
    }
};
std::string name(const Mono&m,const std::vector<std::string>&variables){std::ostringstream out;bool first=true;for(std::size_t i=0;i<m.size();++i)if(m[i]){if(!first)out<<'*';out<<variables[i];if(m[i]!=1)out<<'^'<<m[i];first=false;}if(first)out<<'1';return out.str();}
int main(int argc,char**argv){
    try{
        if(argc!=2)throw std::runtime_error("usage: gb_staircase printed-grevlex.gb");
        std::ifstream in(argv[1]);if(!in)throw std::runtime_error("cannot open input");
        std::string line,body;std::uint64_t p=0;std::vector<std::string>variables;std::size_t declared=0;bool order_ok=false;
        while(std::getline(in,line)){
            if(line.rfind("#field characteristic:",0)==0)p=std::stoull(trim(line.substr(line.find(':')+1)));
            else if(line.rfind("#variable order:",0)==0)variables=split(line.substr(line.find(':')+1),',');
            else if(line.rfind("#monomial order:",0)==0)order_ok=trim(line.substr(line.find(':')+1))=="graded reverse lexicographical";
            else if(line.rfind("#length of basis:",0)==0){std::stringstream header(trim(line.substr(line.find(':')+1)));header>>declared;}
            else if(!line.empty()&&line[0]!='#')body+=line+'\n';
        }
        if(!order_ok||p<5||p>2147483647||variables.empty()||variables.size()>1024)throw std::runtime_error("unsupported/missing basis header");
        if(body.size()>16U*1024U*1024U)throw std::runtime_error("input cap exceeded");
        std::map<std::string,std::size_t>indices;for(std::size_t i=0;i<variables.size();++i)if(!indices.emplace(variables[i],i).second)throw std::runtime_error("duplicate variable");
        body=trim(body);if(body.empty()||body.front()!='[')throw std::runtime_error("missing basis bracket");
        auto close=body.rfind(']');if(close==std::string::npos)throw std::runtime_error("missing close bracket");
        std::string suffix=trim(body.substr(close+1));if(suffix!=""&&suffix!=":"&&suffix!=";")throw std::runtime_error("unexpected basis suffix");
        auto polynomials=split(body.substr(1,close-1),',');if(polynomials.size()!=declared)throw std::runtime_error("printed basis count mismatch");
        std::vector<Mono>leaders;unsigned max_basis_degree=0;bool monic=true;
        for(const auto&text:polynomials){
            Poly poly=TermParser{text,indices,p,variables.size(),0}.parse();
            if(poly.empty())continue;
            auto best=poly.begin();for(auto it=poly.begin();it!=poly.end();++it){max_basis_degree=std::max(max_basis_degree,degree(it->first));if(greater_grevlex(it->first,best->first))best=it;}
            monic=monic&&best->second==1;leaders.push_back(best->first);
        }
        std::vector<Mono>minimal;
        for(std::size_t i=0;i<leaders.size();++i){bool redundant=false;for(std::size_t j=0;j<leaders.size();++j)if(i!=j&&divides(leaders[j],leaders[i])&&(leaders[j]!=leaders[i]||j<i)){redundant=true;break;}if(!redundant)minimal.push_back(leaders[i]);}
        std::map<unsigned,std::size_t>histogram;std::set<Mono>standard;
        bool unit=std::any_of(minimal.begin(),minimal.end(),[](const Mono&m){return degree(m)==0;});
        if(!unit){
            std::vector<unsigned>pure(variables.size(),0);
            for(const auto&m:minimal){std::size_t nonzero=0,index=0;for(std::size_t i=0;i<m.size();++i)if(m[i]){++nonzero;index=i;}if(nonzero==1&&(pure[index]==0||m[index]<pure[index]))pure[index]=m[index];}
            for(std::size_t i=0;i<pure.size();++i)if(pure[i]==0)throw std::runtime_error("initial ideal not certified zero-dimensional: no pure power for "+variables[i]);
            std::queue<Mono>frontier;Mono one(variables.size(),0);standard.insert(one);frontier.push(one);
            while(!frontier.empty()){
                Mono current=frontier.front();frontier.pop();
                for(std::size_t i=0;i<variables.size();++i){Mono next=current;++next[i];if(next[i]>=pure[i])continue;bool killed=false;for(const auto&m:minimal)if(divides(m,next)){killed=true;break;}if(!killed&&standard.insert(next).second){if(standard.size()>100000)throw std::runtime_error("staircase cap exceeded");frontier.push(next);}}
            }
            for(const auto&m:standard)++histogram[degree(m)];
        }
        std::cout<<"{\"variables\":"<<variables.size()<<",\"printed_polynomials\":"<<polynomials.size()<<",\"minimal_leading_monomials\":"<<minimal.size()<<",\"all_monic\":"<<(monic?"true":"false")<<",\"max_printed_basis_degree\":"<<max_basis_degree<<",\"standard_monomials\":"<<standard.size()<<",\"degree_histogram\":{";
        bool first=true;for(auto [d,count]:histogram){if(!first)std::cout<<',';first=false;std::cout<<'"'<<d<<"\":"<<count;}std::cout<<"},\"qualification\":\"Staircase of printed leading ideal; not an independent Groebner/source-ideal certificate\"}\n";
        for(const auto&m:standard)std::cout<<"standard\t"<<degree(m)<<'\t'<<name(m,variables)<<'\n';
        return 0;
    }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}
}
