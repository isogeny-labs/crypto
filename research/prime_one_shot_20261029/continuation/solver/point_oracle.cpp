// Independent signed-point triple oracle. No norm/coefficient algebra is used.
#include <algorithm>
#include <iostream>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace reference {
struct Point {int x=0,y=0;bool infinity=true;};
struct Curve {
    int p,a,b;
    int residue(long long n)const{n%=p;return static_cast<int>(n<0?n+p:n);}
    int powmod(int n,unsigned e)const{int out=1;while(e){if(e&1U)out=residue(out*n);n=residue(n*n);e>>=1;}return out;}
    int inverse(int n)const{if(!residue(n))throw std::invalid_argument("zero inverse");return powmod(n,static_cast<unsigned>(p-2));}
    bool contains(Point q)const{return q.infinity||residue(q.y*q.y)==residue(q.x*q.x*q.x+a*q.x+b);}
    Point sum(Point q,Point r)const{
        if(!contains(q)||!contains(r))throw std::runtime_error("off-curve group input");
        if(q.infinity)return r;
        if(r.infinity)return q;
        if(q.x==r.x&&residue(q.y+r.y)==0)return {};
        int top=0,bottom=0;
        if(q.x==r.x){top=3*q.x*q.x+a;bottom=2*q.y;}
        else{top=r.y-q.y;bottom=r.x-q.x;}
        int slope=residue(top*inverse(bottom));
        int x=residue(slope*slope-q.x-r.x);
        return {x,residue(slope*(q.x-x)-q.y),false};
    }
    std::vector<Point> points()const{
        std::vector<Point> v;
        for(int x=0;x<p;++x)for(int y=0;y<p;++y)if(contains({x,y,false}))v.push_back({x,y,false});
        return v;
    }
};
bool isprime(int n){if(n<2)return false;for(int i=2;i*i<=n;++i)if(n%i==0)return false;return true;}
int read_int(const char*s){std::string v(s);if(v.empty()||v.find_first_not_of("0123456789")!=std::string::npos)throw std::invalid_argument("nonnegative decimal expected");return std::stoi(v);}
bool equal(Point q,Point r){return q.infinity==r.infinity&&(q.infinity||(q.x==r.x&&q.y==r.y));}
std::string encode(Point q,Point r,Point s){std::ostringstream out;out<<q.x<<':'<<q.y<<';'<<r.x<<':'<<r.y<<';'<<s.x<<':'<<s.y;return out.str();}
} // namespace reference
#ifndef POINT_ORACLE_NO_MAIN
int main(int argc,char**argv){
    try{
        using namespace reference;
        if(argc==2&&std::string(argv[1])=="--fixture-points"){
            Curve E{17,2,2};for(Point q:E.points())std::cout<<q.x<<' '<<q.y<<'\n';return 0;
        }
        if(argc!=8)throw std::invalid_argument("usage: point_oracle p a b t c target_x target_y");
        Curve E{read_int(argv[1]),read_int(argv[2]),read_int(argv[3])};
        int t=read_int(argv[4]),c=read_int(argv[5]);Point R{read_int(argv[6]),read_int(argv[7]),false};
        if(E.p<5||E.p>257||!isprime(E.p))throw std::invalid_argument("bounded prime field required");
        for(int v:{E.a,E.b,c,R.x,R.y})if(v<0||v>=E.p)throw std::invalid_argument("parameter outside field");
        if(!E.residue(4LL*E.a*E.a*E.a+27LL*E.b*E.b))throw std::invalid_argument("singular curve");
        auto all=E.points();int order=static_cast<int>(all.size()+1);
        if(!isprime(order)||order==E.p)throw std::invalid_argument("nonanomalous prime curve order required");
        if(t<1||t>8||(E.p-1)%(1U<<t)||c==0||E.powmod(c,static_cast<unsigned>((E.p-1)/(1U<<t)))!=1)throw std::invalid_argument("split nonzero coset required");
        if(!E.contains(R))throw std::invalid_argument("off-curve target");
        std::vector<Point> base;
        for(Point q:all)if(q.x!=R.x&&E.powmod(q.x,1U<<t)==c)base.push_back(q);
        std::set<int> usable_abscissas;for(Point q:base)usable_abscissas.insert(q.x);
        std::set<std::string> models;
        for(std::size_t i=0;i<base.size();++i)for(std::size_t j=i+1;j<base.size();++j)for(std::size_t k=j+1;k<base.size();++k){
            Point q=base[i],r=base[j],s=base[k];
            if(q.x==r.x||q.x==s.x||r.x==s.x)continue;
            if(equal(E.sum(E.sum(q,r),s),R))models.insert(encode(q,r,s));
        }
        for(const auto&m:models)std::cout<<"model\t"<<m<<'\n';
        std::cout<<"summary\t"<<models.size()<<'\t'<<usable_abscissas.size()<<'\n';return 0;
    }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 2;}
}
#endif
