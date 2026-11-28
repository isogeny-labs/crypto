#include <iostream>
#include <vector>
#include <stdexcept>
int f(int x){x%=61;return x<0?x+61:x;}int pw(int a,int n){int r=1;for(;n;n>>=1,a=f(a*a))if(n&1)r=f(r*a);return r;}int inv(int x){if(!x)throw std::runtime_error("zero inverse");return pw(x,59);}
struct P{int x=0,y=0;bool o=true;bool operator==(const P&)const=default;};
P add(P p,P q,int a){if(p.o)return q;if(q.o)return p;if(p.x==q.x&&f(p.y+q.y)==0)return {};int l=p.x==q.x?f((3*p.x*p.x+a)*inv(f(2*p.y))):f((q.y-p.y)*inv(f(q.x-p.x)));int x=f(l*l-p.x-q.x);return {x,f(l*(p.x-x)-p.y),false};}
P mul(P p,int n,int a){P r;for(;n;n>>=1,p=add(p,p,a))if(n&1)r=add(r,p,a);return r;}
bool on(P p,int a,int b){return p.o||f(p.y*p.y)==f(p.x*p.x*p.x+a*p.x+b);}
std::vector<P> pts(int a,int b){std::vector<P>v(1);for(int x=0;x<61;x++)for(int y=0;y<61;y++)if(on({x,y,false},a,b))v.push_back({x,y,false});return v;}
P phi(P p){if(p.o||p.x==1)return {};int t=inv(f(p.x-1));return{f(p.x+24*t),f(p.y*(1-24*t*t)),false};}
P dual(P p){if(p.o||p.x==59)return {};int t=inv(f(p.x-59));return{f(46*(p.x+35*t)),f(23*p.y*(1-35*t*t)),false};}
void point(P p){std::cout<<(p.o?-1:p.x)<<' '<<(p.o?-1:p.y);}
int main(){try{auto e=pts(21,39),t=pts(23,54);if(e.size()!=62||t.size()!=62||pw(59,30)!=60||f(59*59*21)!=23||f(pw(59,3)*39)!=54)throw std::runtime_error("candidate field/domain");if(!f(4*21*21*21+27*39*39)||!f(4*23*23*23+27*54*54))throw std::runtime_error("singular");P g;for(P p:e)if(!p.o&&mul(p,31,21).o){g=p;break;}P u=phi(g);if(g.o||u.o||!mul(u,31,23).o)throw std::runtime_error("odd generators");
 std::cout<<"CM61 61 59 21 39 23 54 31 16 24 35 46 23\nGEN ";point(g);std::cout<<' ';point(u);std::cout<<'\n';int es=0,ts=0,ek=0,tk=0;
 for(unsigned i=0;i<e.size();i++){P p=e[i],z=phi(p),d=dual(z),b=mul(d,16,21);bool odd=mul(p,31,21).o;if(!on(z,23,54)||!(d==mul(p,2,21))||(odd&&!(b==p)))throw std::runtime_error("source inverse/composition");es+=odd;ek+=z.o;std::cout<<"E "<<i<<' ';point(p);std::cout<<' ';point(z);std::cout<<' ';point(d);std::cout<<' ';point(b);std::cout<<' '<<odd<<'\n';}
 for(unsigned i=0;i<t.size();i++){P p=t[i],d=dual(p),z=phi(d),b=phi(mul(d,16,21));bool odd=mul(p,31,23).o;if(!on(d,21,39)||!(z==mul(p,2,23))||(odd&&!(b==p)))throw std::runtime_error("target inverse/composition");ts+=odd;tk+=d.o;std::cout<<"T "<<i<<' ';point(p);std::cout<<' ';point(d);std::cout<<' ';point(z);std::cout<<' ';point(b);std::cout<<' '<<odd<<'\n';}
 int eh=0,th=0;for(P p:e)for(P q:e){if(!(phi(add(p,q,21))==add(phi(p),phi(q),23)))throw std::runtime_error("phi additivity");eh++;}for(P p:t)for(P q:t){if(!(dual(add(p,q,23))==add(dual(p),dual(q),21)))throw std::runtime_error("dual additivity");th++;}for(int n=0;n<31;n++)if(!(phi(mul(g,n,21))==mul(u,n,23)))throw std::runtime_error("public scalar map");int scales=0;for(int a=1;a<61;a++)scales+=f(21*pw(a,4))==23&&f(39*pw(a,6))==54;
 P k{1,0,false};if(k.o||!phi(k).o||mul(k,31,21).o||mul(dual(phi(k)),16,21)==k)throw std::runtime_error("kernel domain control");if(es!=31||ts!=31||ek!=2||tk!=2)throw std::runtime_error("subgroup/kernel count");
 std::cout<<"CHECK "<<e.size()<<' '<<t.size()<<' '<<eh<<' '<<th<<' '<<es<<' '<<ts<<' '<<31<<' '<<scales<<' '<<ek<<' '<<tk<<"\nINJECTION 0\n";
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}return 0;}
