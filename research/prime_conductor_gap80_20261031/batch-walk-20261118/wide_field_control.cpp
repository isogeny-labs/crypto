// Bounded coordinate correctness check only. No isogeny construction or timing.
// Reuse the implementation without copying its arithmetic into a test oracle.
#define main conductor_pipeline_fixture_entry
#include "../native/complete_pipeline.cpp"
#undef main
int main(){try{
 pari_init(256000000,2000000);
 p=Z("4794168233664648458988367110514316682522236917173341621535751");a=mod(-140);b=784;
 Z ell("729845429276318409687041"),B("414464344064");
 Point K{Z("4515130868018265073699728324389842779619221778507237352874816"),Z("1852242499917145718370025456583037063631410696703406330482828"),false};
 require(isprime(gen(p))&&isprime(gen(ell))&&on(K,a,b),"frozen input check");
 GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC);
 require(lg(ellmul(E,gp(K),gen(ell)))==2,"kernel order check");
 Z compared=0;bool first=true;
 std::cout<<"{\"scope\":\"bounded target-field coordinate control only\",\"p\":\""<<p<<"\",\"ell\":\""<<ell<<"\",\"kernel\":";print_point(K);
 std::cout<<",\"points_per_walk\":4096,\"controls\":[";
 for(unsigned mode=0;mode<2;mode++){
   Z start_scalar=mode?Z(2*B):Z(1),step_scalar=mode?Z(4*B):Z(2);
   Point start=mul(K,start_scalar),step=mul(K,step_scalar);
   GEN R=ellmul(E,gp(K),gen(start_scalar)),S=ellmul(E,gp(K),gen(step_scalar));
   require(equal(start,R)&&equal(step,S),"PARI scalar-start mismatch");
   std::vector<Z> expected;expected.reserve(4096);
   for(unsigned k=0;k<4096;k++){
     require(lg(R)==3,"reference walk infinity");expected.push_back(integer(gel(R,1)));R=elladd(E,R,S);
   }
   for(unsigned long batch:{1UL,7UL,64UL,4096UL}){
     WalkDiagnostics diagnostic;unsigned long seen=0;
     for_each_walk_x(start,step,4096,batch,diagnostic,[&](unsigned long i,const Z&x){require(i==seen&&x==expected[i],"PARI wide-field x mismatch");seen++;});
     require(seen==expected.size(),"wide-field prefix incomplete");compared+=seen;
     if(!first)std::cout<<",";first=false;
     std::cout<<"{\"walk\":\""<<(mode?"giant":"baby")<<"\",\"batch_cap\":"<<batch<<",\"x_comparisons\":"<<seen<<",\"batch_inverse_calls\":\""<<diagnostic.batch_inverse_calls<<"\",\"projective_additions\":\""<<diagnostic.projective_additions<<"\",\"peak_batch_points\":"<<diagnostic.peak_batch_points<<",\"first_x\":\""<<expected.front()<<"\",\"last_x\":\""<<expected.back()<<"\"}";
   }
 }
 std::cout<<"],\"total_x_comparisons\":\""<<compared<<"\",\"full_target_quotient\":null,\"isogeny_images\":null,\"runtime_measurements\":null}"<<std::endl;
 pari_close();flint_cleanup();return 0;
}catch(const std::exception&e){std::cerr<<"wide-field-control: "<<e.what()<<'\n';return 2;}}
