#include "../dual_lattice_20261109_v2/frozen_prefix.hpp"
using Q=mpq_class;using Vec=std::array<I,4>;using Mat=std::array<Vec,4>;using QVec=std::array<Q,4>;
I imod(I a,const I&p){a%=p;if(a<0)a+=p;return a;}
I nearest(const Q&q){I num=2*q.get_num()+q.get_den(),den=2*q.get_den();return fd(num,den);}
struct GS{std::array<QVec,4>s{};std::array<std::array<Q,4>,4>mu{};std::array<Q,4>norm{};};
GS gram(const Mat&B){GS g;for(int i=0;i<4;i++){for(int t=0;t<4;t++)g.s[i][t]=Q(B[i][t]);for(int j=0;j<i;j++){need(g.norm[j]>0,"nonzero GS pivot");Q dot=0;for(int t=0;t<4;t++)dot+=Q(B[i][t])*g.s[j][t];g.mu[i][j]=dot/g.norm[j];for(int t=0;t<4;t++)g.s[i][t]-=g.mu[i][j]*g.s[j][t];}for(int t=0;t<4;t++)g.norm[i]+=g.s[i][t]*g.s[i][t];}return g;}
bool lll(Mat&B,unsigned&steps){int k=1;GS g=gram(B);while(k<4&&steps<1024){steps++;for(int j=k-1;j>=0;j--){I z=nearest(g.mu[k][j]);if(z!=0){for(int t=0;t<4;t++)B[k][t]-=z*B[j][t];g=gram(B);}}if(g.norm[k]>=(Q(3,4)-g.mu[k][k-1]*g.mu[k][k-1])*g.norm[k-1])k++;else{std::swap(B[k],B[k-1]);g=gram(B);k=std::max(1,k-1);}}return k==4;}
