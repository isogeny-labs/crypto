// Fixed source/kernel proof controls; not the degree-ell map or a timer.
#define main complete_pipeline_fixture_entry
#include "../native/complete_pipeline.cpp"
#undef main

static bool same(const Point& x,const Point& y) {
  return x.inf==y.inf && (x.inf || (x.x==y.x && x.y==y.y));
}
static Point negative(Point x) {if(!x.inf)x.y=mod(-x.y);return x;}
static Z pow_mod(Z x,const Z& e,const Z& modulus) {
  Z result;mpz_powm(result.get_mpz_t(),x.get_mpz_t(),e.get_mpz_t(),modulus.get_mpz_t());return result;
}
static Point cm(Point P,const Z& s) {
  Z r=mod(7+s),u=mod((1-s)*inverse(2)),t=mod(3*r*r+a);
  if(P.inf || P.x==r)return {};
  Z v=inverse(mod(P.x-r)),u2=mod(u*u),u3=mod(u2*u);
  return {mod((P.x+t*v)*inverse(u2)),mod(P.y*(1-t*v*v)*inverse(u3)),false};
}
using Ring=std::array<Z,2>; // alpha^2=alpha-2
static Ring times(const Ring& x,const Ring& y,const Z& modulus) {
  Ring r={x[0]*y[0]-2*x[1]*y[1],x[0]*y[1]+x[1]*y[0]+x[1]*y[1]};
  for(auto& c:r)mpz_mod(c.get_mpz_t(),c.get_mpz_t(),modulus.get_mpz_t());
  return r;
}
static bool identity(const Ring& x) {return x[0]==1 && x[1]==0;}
static bool matrix_identity(GEN x) {
  return gequal1(gcoeff(x,1,1))&&gequal1(gcoeff(x,2,2))&&
         gequal0(gcoeff(x,1,2))&&gequal0(gcoeff(x,2,1));
}
static void encode(std::ostream& out,const Point& x) {
  if(x.inf)out<<"null";
  else out<<"[\""<<x.x<<"\",\""<<x.y<<"\"]";
}
int main(int argc,char**argv) {try {
  bool dependent=argc==2&&std::string(argv[1])=="--dependent-pair";
  require(argc==1||dependent,"only --dependent-pair control is accepted");
  Z ell,q,N;Point K,G;K.inf=G.inf=false;
  require(bool(std::cin>>p>>a>>b>>ell>>K.x>>K.y>>q>>G.x>>G.y>>N),"frozen source input");
  std::string extra;require(!(std::cin>>extra),"unexpected extra input");
  require(p==Z("4794168233664648458988367110514316682522236917173341621535751")&&a==-140&&b==784&&
          ell==Z("729845429276318409687041")&&q==Z("1125023250121"),"exact frozen target required");
  pari_init(256000000,2000000);
  require(isprime(gen(p))&&isprime(gen(ell))&&isprime(gen(q)),"prime inputs");
  require(N==8*q*ell*ell && on(K,a,b)&&on(G,a,b)&&mul(K,ell).inf&&mul(G,q).inf,"source/order witnesses");
  require(mpz_kronecker_si(ell.get_mpz_t(),-7)!=0,"nonramified prime");
  Z d=-7;require(mpz_kronecker(d.get_mpz_t(),ell.get_mpz_t())==-1,"frozen prime must be inert");
  require(p%4==3,"fixed square-root extraction case");
  Z s=pow_mod(mod(-7),Z((p+1)/4),p);if(s>p-s)s=p-s;
  require(mod(s*s)==mod(-7),"square root of -7");
  GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC);
  Point L=dependent?mul(K,2):cm(K,s);
  require(on(L,a,b)&&!L.inf&&mul(L,ell).inf,"second torsion point");
  Z w=integer(ellweilpairing(E,gp(K),gp(L),gen(ell)));
  require(w!=1&&pow_mod(w,ell,p)==1,"dependent torsion pair rejected");
  require(integer(ellweilpairing(E,gp(K),gp(mul(K,2)),gen(ell)))==1,"dependent pairing control");
  require(integer(ellweilpairing(E,gp(L),gp(K),gen(ell)))==inverse(w),"alternating pairing control");
  require(integer(ellweilpairing(E,gp(mul(K,2)),gp(L),gen(ell)))==mod(w*w),"bilinear pairing control");
  std::ostringstream out;
  out<<"{\"scope\":\"source CM/kernel/HD prerequisite checks only; no degree-ell map\",\"ell\":\""<<ell
     <<"\",\"p\":\""<<p<<"\",\"weil_pairing\":\""<<w
     <<"\",\"weil_pairing_exact_prime_order\":true,\"kernel_basis\":[";encode(out,K);out<<',';encode(out,L);
  out<<"],\"kernel_cm_stable\":false,\"cm_branches\":[";
  for(unsigned branch=0;branch<2;++branch) {
    Z root=branch?mod(-s):s,r=mod(7+root),u=mod((1-root)*inverse(2));
    Z t=mod(3*r*r+a),ca=mod(a-5*t),cb=mod(b-7*(5*r*r*r+3*a*r+2*b));
    Z u2=mod(u*u),u3=mod(u2*u),u4=mod(u2*u2),u6=mod(u3*u3);
    require(mod(ca-u4*a)==0&&mod(cb-u6*b)==0&&mod(u*u-u+2)==0,"quotient isomorphism and differential");
    Point T{r,0,false};require(on(T,a,b)&&mul(T,2).inf,"rational 2-kernel");
    GEN iso=ellisogeny(E,gp(T),0,0,1),coeff=gel(iso,1),map=gel(iso,2);
    require(integer(gel(coeff,4))==ca&&integer(gel(coeff,5))==cb,"PARI quotient coefficients");
    require(equal(cm(T,root),ellisogenyapply(map,gp(T)))&&equal(cm(Point{},root),ellisogenyapply(map,gp(Point{}))),"kernel/infinity exceptions");
    if(branch)out<<',';
    out<<"{\"s\":\""<<root<<"\",\"r\":\""<<r<<"\",\"u\":\""<<u
       <<"\",\"normalized_quotient\":[\""<<ca<<"\",\""<<cb<<"\"],\"samples\":[";
    for(unsigned i=1;i<=32;++i) {
      Point P=mul(K,Z(i)),Q=mul(G,Z(i)),AP=cm(P,root),AQ=cm(Q,root);
      for(Point X:{P,Q}) {
        Point A=cm(X,root),C=cm(X,mod(-root));GEN ref=ellisogenyapply(map,gp(X));
        require(lg(ref)==3,"finite PARI quotient image");
        Point normalized{mod(integer(gel(ref,1))*inverse(u2)),mod(integer(gel(ref,2))*inverse(u3)),false};
        require(same(A,normalized)&&on(A,a,b),"native/PARI CM image");
        require(same(add(A,C),X)&&same(cm(A,mod(-root)),mul(X,2)),"conjugate sum and product");
        require(add(add(cm(A,root),negative(A)),mul(X,2)).inf,"CM characteristic identity");
      }
      require(mul(AP,ell).inf&&mul(AQ,q).inf,"CM subgroup preservation");
      if(i>1)out<<',';out<<"{\"scalar\":"<<i<<",\"kernel_point\":";encode(out,P);out<<",\"kernel_cm_image\":";encode(out,AP);
      out<<",\"q_point\":";encode(out,Q);out<<",\"q_cm_image\":";encode(out,AQ);out<<'}';
    }
    out<<"]}";
  }
  Z trace=p+1-N,disc=4*p-trace*trace,conductor;
  require(disc>0&&disc%7==0,"CM discriminant metadata");
  disc/=7;mpz_sqrt(conductor.get_mpz_t(),disc.get_mpz_t());
  require(conductor*conductor==disc&&(trace-conductor)%2==0,"CM Frobenius conductor");
  Ring pi={Z((trace-conductor)/2),conductor};
  require(pi[0]*pi[0]+pi[0]*pi[1]+2*pi[1]*pi[1]==p&&2*pi[0]+pi[1]==trace,"exact Frobenius norm/trace");
  unsigned cardinality_m=1,embedding_m=1;
  while((Z(1)<<(2*cardinality_m))<4*ell+1)++cardinality_m;
  while((Z(1)<<embedding_m)<=ell)++embedding_m;
  out<<"],\"frobenius_input_trace\":\""<<trace<<"\",\"frobenius_coefficients\":[\""<<pi[0]<<"\",\""<<pi[1]
     <<"\"],\"trace_status\":\"frozen CM trace supplied; not recounted here\",\"interpolation_cardinality_m\":"<<cardinality_m
     <<",\"single_power_embedding_m\":"<<embedding_m<<",\"full_two_power_torsion_orders\":[";
  for(unsigned m=1;m<=80;++m) {
    pari_sp av=avma;Z modulus=Z(1)<<m;Ring current=times(pi,{1,0},modulus),previous=current;Z order=1;
    unsigned squares=0;
    while(!identity(current)) {require(squares++<=m,"two-primary order cap");previous=current;current=times(current,current,modulus);order*=2;}
    require(order==1||!identity(previous),"minimal two-power order");
    GEN matrix=mkmat2(mkcol2(gen(pi[0]),gen(pi[1])),mkcol2(gen(Z(-2*pi[1])),gen(Z(pi[0]+pi[1]))));
    matrix=gmul(gmodulo(gen_1,gen(modulus)),matrix);
    require(matrix_identity(gpow(matrix,gen(order),DEFAULTPREC)),"PARI modular matrix order");
    require(order==1||!matrix_identity(gpow(matrix,gen(Z(order/2)),DEFAULTPREC)),"PARI previous power rejection");
    if(m>1)out<<',';out<<"{\"m\":"<<m<<",\"extension_degree\":\""<<order<<"\",\"matrix_check\":true,\"half_power_reject\":"<<(order>1?"true":"null")<<'}';
    avma=av;
  }
  out<<"],\"target_quotient\":null,\"target_non_kernel_images\":null,\"endpoint_timings\":null}\n";
  std::cout<<out.str();pari_close();flint_cleanup();return 0;
}catch(const std::exception& e){std::cerr<<e.what()<<'\n';return 2;}}
