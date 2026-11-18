#include <array>
#include <iostream>
#include <stdexcept>
#include <vector>
using F=int;
int mod(int x){x%=7;return x<0?x+7:x;}
F add(F x,F y){return mod(x%7+y%7)+7*mod(x/7+y/7);}
F neg(F x){return mod(-x%7)+7*mod(-x/7);}
F sub(F x,F y){return add(x,neg(y));}
F mul(F x,F y){return mod(x%7*(y%7)+3*(x/7)*(y/7))+7*mod(x%7*(y/7)+(x/7)*(y%7));}
F power(F a,int n){F r=1;for(;n;n>>=1,a=mul(a,a))if(n&1)r=mul(r,a);return r;}
F inv(F x){if(!x)throw std::runtime_error("zero inverse");return power(x,47);}
F scale(F a,int n){return mul(a,mod(n));}
bool prime(int n){if(n<2)return false;for(int d=2;d*d<=n;d++)if(n%d==0)return false;return true;}
int count(F A,F B){int n=1;for(F x=0;x<49;x++){F f=add(add(mul(mul(x,x),x),mul(A,x)),B);for(F y=0;y<49;y++)n+=mul(y,y)==f;}return n;}
struct Z{F a,b;bool operator==(const Z&)const=default;};
F delta;
Z plus(Z x,Z y){return {add(x.a,y.a),add(x.b,y.b)};}
Z minus(Z x){return {neg(x.a),neg(x.b)};}
Z times(Z x,Z y){return {add(mul(x.a,y.a),mul(delta,mul(x.b,y.b))),add(mul(x.a,y.b),mul(x.b,y.a))};}
Z recip(Z x){F den=inv(sub(mul(x.a,x.a),mul(delta,mul(x.b,x.b))));return {mul(x.a,den),neg(mul(x.b,den))};}
Z integer(int n){return {mod(n),0};}
struct P{Z x,y;bool o=true;bool operator==(const P&)const=default;};
F AA,BB;
P sum(P p,P q){if(p.o)return q;if(q.o)return p;if(p.x==q.x&&plus(p.y,q.y)==Z{0,0})return {};Z l;if(p.x==q.x)l=times(plus(times(integer(3),times(p.x,p.x)),Z{AA,0}),recip(times(integer(2),p.y)));else l=times(plus(q.y,minus(p.y)),recip(plus(q.x,minus(p.x))));Z x=plus(plus(times(l,l),minus(p.x)),minus(q.x));Z y=plus(times(l,plus(p.x,minus(x))),minus(p.y));return {x,y,false};}
P scalar(P p,int n){P r;for(;n;n>>=1,p=sum(p,p))if(n&1)r=sum(r,p);return r;}
bool on(P p){return p.o||times(p.y,p.y)==plus(plus(times(times(p.x,p.x),p.x),times(Z{AA,0},p.x)),Z{BB,0});}
P frob(P p){if(p.o)return p;p.x.b=neg(p.x.b);p.y.b=neg(p.y.b);return p;}
P embed(P t){if(t.o)return {};F di=inv(delta);return {{mul(t.x.a,di),0},{0,mul(t.y.a,mul(di,di))},false};}
std::vector<P> points(F A,F B){std::vector<P> v(1);for(F x=0;x<49;x++)for(F y=0;y<49;y++)if(mul(y,y)==add(add(mul(mul(x,x),x),mul(A,x)),B))v.push_back({{x,0},{y,0},false});return v;}
int main(){try{
 for(delta=1;delta<49&&power(delta,24)!=neg(1);delta++);if(delta==49)throw std::runtime_error("no nonsquare");
 int r=0,t=0,eligible=0;F A=-1,B=-1;for(F a=0;a<49;a++)for(F b=0;b<49;b++){F disc=add(scale(mul(mul(a,a),a),4),scale(mul(b,b),27));if(!disc)continue;F j=mul(scale(scale(mul(mul(a,a),a),4),1728),inv(disc));if(j/7==0)continue;int n=count(a,b),m=100-n;if(!prime(n)||!prime(m)||n==m||n<11||m<11)continue;eligible++;if(A<0){A=a;B=b;r=n;t=m;}}
 if(A<0)throw std::runtime_error("no fixture");AA=A;BB=B;F AT=mul(mul(delta,delta),A),BT=mul(mul(mul(delta,delta),delta),B);auto e=points(A,B),tw=points(AT,BT);if((int)e.size()!=r||(int)tw.size()!=t)throw std::runtime_error("order");
 int h=(r+1)/2,checked=0;for(P p:e)for(P u:tw){P c=embed(u),lift=sum(scalar(p,h),c);if(!on(c)||!on(lift)||!sum(c,frob(c)).o||!(sum(lift,frob(lift))==p))throw std::runtime_error("trace map");checked++;}
 std::cout<<"{\"schema\":\"twist.trace-fixture/v1\",\"p\":7,\"base_modulus\":3,\"delta\":"<<delta<<",\"A\":"<<A<<",\"B\":"<<B<<",\"twist_A\":"<<AT<<",\"twist_B\":"<<BT<<",\"source_order\":"<<r<<",\"twist_order\":"<<t<<",\"half\":"<<h<<",\"eligible_curves\":"<<eligible<<",\"trace_pairs\":"<<checked<<"}\n";
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
