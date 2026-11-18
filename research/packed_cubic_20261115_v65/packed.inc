static Family solve(const std::array<Row,83>&,Row,int);
static bool use_packed=false,verify_packed=false,profile_enabled=false;
static uint64_t packed_calls=0,wide_calls=0,verified_packed_calls=0;
static uint64_t profile_setup_ns=0,profile_fiber_ns=0,profile_support_ns=0,profile_total_ns=0;
static std::chrono::steady_clock::time_point profile_start;
static Family packed_solve(const std::array<Row,83>& columns,Row rhs,int count,int low_width,int high_width){
 if(count<0||count>64||low_width<0||high_width<0||low_width+high_width>64)throw std::runtime_error("packed dimensions");
 auto pack=[&](const Row& r)->uint64_t{if((r.lo>>low_width)||(r.hi>>high_width))throw std::runtime_error("packed row outside declared width");return uint64_t(r.lo)|(high_width?(uint64_t(r.hi)<<low_width):0);};
 std::array<uint64_t,64> pivots{},coordinates{};Family result;
 for(int index=0;index<count;++index){uint64_t value=pack(columns[index]),coordinate=uint64_t(1)<<index;
  while(value){int p=63-__builtin_clzll(value);if(pivots[p]){value^=pivots[p];coordinate^=coordinates[p];}else{pivots[p]=value;coordinates[p]=coordinate;++result.rank;break;}}
  if(!value)result.kernel.push_back(E(coordinate));
 }
 uint64_t value=pack(rhs);while(value){int p=63-__builtin_clzll(value);if(!pivots[p]){result.consistent=false;return result;}value^=pivots[p];result.particular^=E(coordinates[p]);}
 return result;
}
static Family inner_dispatch(const std::array<Row,83>& columns,Row rhs,int count,int low_width,int high_width){
 if(use_packed&&count<=64&&low_width+high_width<=64){++packed_calls;Family value=packed_solve(columns,rhs,count,low_width,high_width);
  if(verify_packed){Family ref=solve(columns,rhs,count);if(value.consistent!=ref.consistent||value.rank!=ref.rank||value.kernel!=ref.kernel||(value.consistent&&value.particular!=ref.particular))throw std::runtime_error("packed/wide exact family mismatch");++verified_packed_calls;}
  return value;
 }
 ++wide_calls;return solve(columns,rhs,count);
}
