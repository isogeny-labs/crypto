#include <array>
#include <chrono>
#include <cstdlib>
#include <iostream>
#include <vector>
using namespace std;

static int mod3(int x) { x %= 3; return x < 0 ? x + 3 : x; }
static int ipow(int b, int e) { int y=1; while(e--) y*=b; return y; }
struct Field {
    int m, q, modulus_mask;
    vector<int> modc, add, mul, chi;
    vector<array<int,12>> powers;
    int at(const vector<int>& tab,int a,int b) const { return tab[a*q+b]; }
    Field(int degree,int mask):m(degree),q(ipow(3,degree)),modulus_mask(mask),modc(degree),add(q*q),mul(q*q),chi(q,-1),powers(q) {
        int u=mask;for(int i=0;i<m;i++){modc[i]=u%3;u/=3;}
        for(int a=0;a<q;a++)for(int b=0;b<q;b++){
            int aa=a,bb=b,c=0,p=1;for(int i=0;i<m;i++){c+=mod3(aa%3+bb%3)*p;aa/=3;bb/=3;p*=3;}add[a*q+b]=c;
            array<int,11> v{};aa=a;for(int i=0;i<m;i++){bb=b;for(int j=0;j<m;j++){v[i+j]=mod3(v[i+j]+(aa%3)*(bb%3));bb/=3;}aa/=3;}
            for(int i=2*m-2;i>=m;i--){int h=v[i];for(int j=0;j<m;j++)v[i-m+j]=mod3(v[i-m+j]-h*modc[j]);}
            c=0;p=1;for(int i=0;i<m;i++){c+=v[i]*p;p*=3;}mul[a*q+b]=c;
        }
        chi[0]=0;for(int a=1;a<q;a++)chi[at(mul,a,a)]=1;
        for(int a=0;a<q;a++){powers[a][0]=1;for(int j=1;j<=11;j++)powers[a][j]=at(mul,powers[a][j-1],a);}
        for(int a=1;a<q;a++)if(chi[a]==-1){} // nonsquares remain -1
    }
    int count(const array<int,11>& c) const {
        int sum=0;
        for(int x=0;x<q;x++){
            int y=powers[x][11];
            for(int j=0;j<11;j++)if(c[j]==1)y=at(add,y,powers[x][j]);else if(c[j]==2)y=at(add,y,at(add,powers[x][j],powers[x][j]));
            sum+=chi[y];
        }
        return q+1+sum;
    }
};
static void trim(vector<int>& a){while(!a.empty()&&a.back()==0)a.pop_back();}
static vector<int> rem(vector<int> a,const vector<int>& b){trim(a);int inv=b.back()==1?1:2;while(a.size()>=b.size()){int shift=int(a.size()-b.size()),factor=mod3(a.back()*inv);for(size_t i=0;i<b.size();i++)a[i+shift]=mod3(a[i+shift]-factor*b[i]);trim(a);}return a;}
static bool squarefree(const array<int,11>& c){vector<int> a(c.begin(),c.end());a.push_back(1);vector<int> b(11);for(int i=1;i<=11;i++){int z=(i==11?1:c[i]);b[i-1]=mod3(i*z);}trim(b);while(!b.empty()){auto r=rem(a,b);a=b;b=r;}return a.size()==1;}
int main(){
    vector<Field> F;for(auto [m,mask]:vector<pair<int,int>>{{1,0},{2,1},{3,7},{4,5},{5,7}})F.emplace_back(m,mask);
    array<long long,8> stage{};vector<array<int,7>> rows;auto start=chrono::steady_clock::now();
    for(int mask=0;mask<ipow(3,11);mask++){
        stage[0]++;array<int,11> c{};int u=mask;for(int j=0;j<11;j++){c[j]=u%3;u/=3;}
        bool ok=true;for(int j=0;j<4;j++){if(F[j].count(c)!=F[j].q+1){ok=false;break;}stage[j+1]++;}if(!ok)continue;
        if(!squarefree(c))continue;stage[5]++;
        int n5=F[4].count(c);int diff=244-n5;if(diff%5)continue;stage[6]++;
        int t=diff/5;if(abs(t)>31||t%3==0)continue;stage[7]++;
        rows.push_back({mask,4,10,28,82,n5,t});
    }
    double sec=chrono::duration<double>(chrono::steady_clock::now()-start).count();
    cout<<"{\"schema\":\"ghs-odd5-carrier-screen/v1\",\"modulus_masks\":[0,1,7,5,7],\"stage_counts\":[";
    for(size_t i=0;i<stage.size();i++){if(i)cout<<',';cout<<stage[i];}cout<<"],\"elapsed_sec\":"<<sec<<",\"rows\":[";
    for(size_t i=0;i<rows.size();i++){if(i)cout<<',';cout<<'[';for(size_t j=0;j<rows[i].size();j++){if(j)cout<<',';cout<<rows[i][j];}cout<<']';}cout<<"]}\n";
}
