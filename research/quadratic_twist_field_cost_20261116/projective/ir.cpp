// Fixed engineering preflight for an external instruction counter, not a speed result.
#define main projective_census_main
#include "projective.cpp"
#undef main
#include <valgrind/callgrind.h>
#include <valgrind/valgrind.h>
volatile unsigned long long counter_probe=1;
void probe(){for(int i=0;i<1000000;i++)counter_probe=counter_probe*3+1;}
int main(int argc,char**argv){try{
 need(argc==3,"ir NEW_JSON COUNTER_BASE");need(RUNNING_ON_VALGRIND>0,"real external counter required");
 RECORD=false;P g{F(0),F(10),false},s=chi(P{F(37),F(14),false});auto source=group(g,21,39);std::vector<P>reps;
 for(P q:source)if(!q.o&&q<neg(q))reps.push_back(q);need(reps.size()==15,"full source inverse-pair domain");
 std::vector<P>base{reps[0],reps[1],reps[2]};P unknown=source.at(1);J records=J::array();int part=0;
 auto one=[&](int route,int cap,std::string label,bool inside,bool outside){
  ROUTE=route;terminal_stop=true;if(outside)probe();
  CALLGRIND_ZERO_STATS;CALLGRIND_TOGGLE_COLLECT;
  auto recovered=recover_cold(unknown,g,s,base,cap);if(inside)probe();
  CALLGRIND_TOGGLE_COLLECT;CALLGRIND_DUMP_STATS_AT(label.c_str());
  records.push_back(J{{"route",route},{"policy",0},{"cap",cap},{"target",1},{"label",label},{"inside_probe",inside},{"outside_probe",outside},{"counter_path",std::string(argv[2])+"."+std::to_string(++part)},{"result",result(recovered)}});
 };
 for(int route:{3,4,5})for(int cap:{3,4})one(route,cap,"plain-"+std::to_string(route)+"-"+std::to_string(cap),false,false);
 one(3,3,"repeat-aa",false,false);one(3,3,"positive-inside",true,false);one(3,3,"negative-outside",false,true);
 J doc={{"schema","twist.ir-counter-preflight/v1"},{"field",61},{"relation_field",31},{"target_point",J::array({encode(unknown.x),encode(unknown.y)})},{"records",records}};
 write(argv[1],doc.dump()+"\n");return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 101;}}
