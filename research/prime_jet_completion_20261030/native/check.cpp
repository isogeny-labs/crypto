#include <gmpxx.h>
#include <bit>
#include <iostream>
#include <vector>
using Q=mpq_class;
Q ratio(int n,int d){Q q(n,d);q.canonicalize();return q;}
Q cp(int l,int t){Q r=1;for(int s=-l;s<l;++s)if(s!=t)r*=t-s;return r;}
Q slope(int l,int t){Q r=0;for(int s=-l;s<l;++s)if(s!=t)r+=ratio(1,t-s);return r;}
int val(Q q){if(q==0)return 100000;mpz_class n=q.get_num();if(n<0)n=-n;return int(mpz_scan1(n.get_mpz_t(),0))-int(mpz_scan1(q.get_den().get_mpz_t(),0));}
unsigned rank(std::vector<unsigned long long> a,int l){unsigned r=0;for(int c=0;c<l;++c){unsigned k=r;while(k<a.size()&&!((a[k]>>c)&1))++k;if(k==a.size())continue;std::swap(a[k],a[r]);for(unsigned j=0;j<a.size();++j)if(j!=r&&((a[j]>>c)&1))a[j]^=a[r];++r;}return r;}
int main(){for(int l:{4,8,16,32}){unsigned checks=0;Q c0=cp(l,0),ct=cp(l,1);int alpha=2*l;std::vector<std::vector<Q>> a(2,std::vector<Q>(l*l));for(int d=-l;d<l;++d){Q cd=cp(l,d),sd=slope(l,d);for(int t=-l;t<l;++t){Q e=(t==d)?Q(1):Q(0);Q ep=(t==d)?sd:Q(cp(l,t)/(cd*(t-d)));Q f=e*e*(1-2*sd*(t-d));Q fp=2*e*ep*(1-2*sd*(t-d))-2*sd*e*e;if(f!=e||fp!=0)return 1;++checks;}}for(int x=0;x<l;++x)for(int y=0;y<l;++y)if((x%2)==(y%2)){int d=y-x;Q cd=cp(l,d),sd=slope(l,d),ep=ct/(cd*(1-d));a[0][x*l+y]=Q(alpha)*ct/(c0*(1-d));a[1][x*l+y]=(cd/c0)*ep*ep*alpha*alpha*(1-2*sd*(1-d));}for(int k=0;k<2;++k){int v=100000;for(Q q:a[k])v=std::min(v,val(q));std::vector<unsigned long long> rows(l);for(int x=0;x<l;++x)for(int y=0;y<l;++y){Q q=a[k][x*l+y];if(q==0)continue;if(v>=0)q/=mpz_class(1)<<v;else q*=mpz_class(1)<<(-v);q.canonicalize();if(mpz_even_p(q.get_den().get_mpz_t()))return 2;if(mpz_odd_p(q.get_num().get_mpz_t()))rows[x]|=1ULL<<y;}std::cout<<"L="<<l<<" nodal_checks="<<checks<<" variant="<<k<<" valuation="<<v<<" residue_rank="<<rank(rows,l)<<'\n';}}}
