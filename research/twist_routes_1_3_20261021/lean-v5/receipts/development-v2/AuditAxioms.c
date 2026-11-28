// Lean compiler output
// Module: tools.lean.AuditAxioms
// Imports: public import Init public meta import Init public import Lean
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
lean_object* l_Lean_Environment_getModuleIdxFor_x3f(lean_object*, lean_object*);
lean_object* l_Lean_Environment_header(lean_object*);
lean_object* l_Lean_EnvironmentHeader_moduleNames(lean_object*);
lean_object* lean_array_get(lean_object*, lean_object*, lean_object*);
uint8_t lean_name_eq(lean_object*, lean_object*);
uint8_t l_Lean_Name_isPrefixOf(lean_object*, lean_object*);
uint8_t lean_nat_dec_lt(lean_object*, lean_object*);
lean_object* lean_array_fswap(lean_object*, lean_object*, lean_object*);
lean_object* lean_array_fget_borrowed(lean_object*, lean_object*);
lean_object* l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(lean_object*, uint8_t);
uint8_t lean_string_dec_lt(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
lean_object* lean_array_push(lean_object*, lean_object*);
uint8_t l_Lean_Name_isInternal(lean_object*);
lean_object* lean_array_fget(lean_object*, lean_object*);
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
lean_object* lean_nat_shiftr(lean_object*, lean_object*);
lean_object* l_instMonadEIO(lean_object*);
lean_object* l_instMonadStateOfStateTOfMonad___redArg(lean_object*);
lean_object* l_List_reverse___redArg(lean_object*);
lean_object* l_Lean_Name_mkStr2(lean_object*, lean_object*);
lean_object* l_String_splitOnAux(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_List_lengthTR___redArg(lean_object*);
lean_object* l_Lean_Name_mkStr1(lean_object*);
uint8_t l_List_elem___at___00__private_Lean_Server_Completion_CompletionCollectors_0__Lean_Server_Completion_bestLabelForDecl_x3f_spec__0(lean_object*, lean_object*);
size_t lean_array_size(lean_object*);
size_t lean_usize_add(size_t, size_t);
uint8_t lean_usize_dec_lt(size_t, size_t);
lean_object* lean_array_uget_borrowed(lean_object*, size_t);
extern lean_object* l___private_Lean_Util_CollectAxioms_0__Lean_instInhabitedExportedAxiomsState;
lean_object* l_Lean_Environment_setExporting(lean_object*, uint8_t);
extern lean_object* l___private_Lean_Util_CollectAxioms_0__Lean_exportedAxiomsExt;
lean_object* l_Lean_PersistentEnvExtension_getState___redArg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l___private_Lean_Util_CollectAxioms_0__Lean_ExportedAxiomsState_find_x3f___boxed(lean_object*, lean_object*, lean_object*);
lean_object* l___private_Lean_Util_CollectAxioms_0__Lean_CollectAxioms_collectAndGet___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l___private_Lean_Util_CollectAxioms_0__Lean_CollectAxioms_runM___redArg(lean_object*, lean_object*);
lean_object* lean_array_to_list(lean_object*);
uint8_t l_List_isEmpty___redArg(lean_object*);
lean_object* lean_string_append(lean_object*, lean_object*);
lean_object* l_List_toString___at___00Lean_resolveUniqueNamespace___at___00Lean_Elab_OpenDecl_elabOpenDecl___at___00Lean_Elab_Command_elabOpen_spec__0_spec__4_spec__5(lean_object*);
lean_object* l_IO_println___at___00__private_Lean_Shell_0__Lean_ShellOptions_process_spec__3(lean_object*);
lean_object* l_String_intercalate(lean_object*, lean_object*);
lean_object* lean_array_get_size(lean_object*);
lean_object* l_Nat_reprFast(lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
lean_object* l_IO_eprintln___at___00__private_Lean_Shell_0__Lean_shellMain_spec__1(lean_object*);
lean_object* l_Lean_findSysroot(lean_object*);
lean_object* l_Lean_initSearchPath(lean_object*, lean_object*);
lean_object* l_String_toName(lean_object*);
lean_object* lean_array_mk(lean_object*);
extern lean_object* l_Lean_Options_empty;
lean_object* lean_mk_empty_array_with_capacity(lean_object*);
lean_object* l_Lean_importModules(lean_object*, lean_object*, uint32_t, lean_object*, uint8_t, uint8_t, uint8_t, lean_object*);
lean_object* l_Lean_Environment_constants(lean_object*);
lean_object* l_Lean_SMap_fold___at___00Lean_SMap_toList___at___00__private_Lean_Elab_DocString_Builtin_0__Lean_Doc_getQualified_spec__1_spec__2___redArg(lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
uint8_t lean_string_dec_eq(lean_object*, lean_object*);
lean_object* l_IO_FS_lines(lean_object*);
lean_object* l_List_head_x21___redArg(lean_object*, lean_object*);
lean_object* lean_string_data(lean_object*);
uint8_t lean_uint32_dec_eq(uint32_t, uint32_t);
lean_object* lean_string_mk(lean_object*);
lean_object* lean_string_utf8_byte_size(lean_object*);
lean_object* l_instMonadStateOfMonadStateOf___redArg(lean_object*);
lean_object* l_modify(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* l_StateT_get(lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_string_object l_standardAxioms___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 8, .m_capacity = 8, .m_length = 7, .m_data = "propext"};
static const lean_object* l_standardAxioms___closed__0 = (const lean_object*)&l_standardAxioms___closed__0_value;
static const lean_ctor_object l_standardAxioms___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&l_standardAxioms___closed__0_value),LEAN_SCALAR_PTR_LITERAL(53, 150, 49, 30, 125, 3, 39, 172)}};
static const lean_object* l_standardAxioms___closed__1 = (const lean_object*)&l_standardAxioms___closed__1_value;
static const lean_string_object l_standardAxioms___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 10, .m_capacity = 10, .m_length = 9, .m_data = "Classical"};
static const lean_object* l_standardAxioms___closed__2 = (const lean_object*)&l_standardAxioms___closed__2_value;
static const lean_string_object l_standardAxioms___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 7, .m_capacity = 7, .m_length = 6, .m_data = "choice"};
static const lean_object* l_standardAxioms___closed__3 = (const lean_object*)&l_standardAxioms___closed__3_value;
static const lean_ctor_object l_standardAxioms___closed__4_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&l_standardAxioms___closed__2_value),LEAN_SCALAR_PTR_LITERAL(40, 236, 220, 79, 38, 141, 161, 150)}};
static const lean_ctor_object l_standardAxioms___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&l_standardAxioms___closed__4_value_aux_0),((lean_object*)&l_standardAxioms___closed__3_value),LEAN_SCALAR_PTR_LITERAL(76, 246, 154, 249, 193, 98, 251, 55)}};
static const lean_object* l_standardAxioms___closed__4 = (const lean_object*)&l_standardAxioms___closed__4_value;
static const lean_string_object l_standardAxioms___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "Quot"};
static const lean_object* l_standardAxioms___closed__5 = (const lean_object*)&l_standardAxioms___closed__5_value;
static const lean_string_object l_standardAxioms___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "sound"};
static const lean_object* l_standardAxioms___closed__6 = (const lean_object*)&l_standardAxioms___closed__6_value;
static const lean_ctor_object l_standardAxioms___closed__7_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&l_standardAxioms___closed__5_value),LEAN_SCALAR_PTR_LITERAL(91, 127, 250, 116, 111, 99, 160, 200)}};
static const lean_ctor_object l_standardAxioms___closed__7_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&l_standardAxioms___closed__7_value_aux_0),((lean_object*)&l_standardAxioms___closed__6_value),LEAN_SCALAR_PTR_LITERAL(255, 255, 230, 69, 40, 79, 199, 28)}};
static const lean_object* l_standardAxioms___closed__7 = (const lean_object*)&l_standardAxioms___closed__7_value;
static const lean_ctor_object l_standardAxioms___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&l_standardAxioms___closed__7_value),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* l_standardAxioms___closed__8 = (const lean_object*)&l_standardAxioms___closed__8_value;
static const lean_ctor_object l_standardAxioms___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&l_standardAxioms___closed__4_value),((lean_object*)&l_standardAxioms___closed__8_value)}};
static const lean_object* l_standardAxioms___closed__9 = (const lean_object*)&l_standardAxioms___closed__9_value;
static const lean_ctor_object l_standardAxioms___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&l_standardAxioms___closed__1_value),((lean_object*)&l_standardAxioms___closed__9_value)}};
static const lean_object* l_standardAxioms___closed__10 = (const lean_object*)&l_standardAxioms___closed__10_value;
LEAN_EXPORT const lean_object* l_standardAxioms = (const lean_object*)&l_standardAxioms___closed__10_value;
LEAN_EXPORT uint8_t l_List_any___at___00inScope_spec__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_any___at___00inScope_spec__0___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t l_inScope(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_inScope___boxed(lean_object*, lean_object*, lean_object*);
static lean_once_cell_t l_instMonadEnvAuditM___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* l_instMonadEnvAuditM___closed__0;
static lean_once_cell_t l_instMonadEnvAuditM___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* l_instMonadEnvAuditM___closed__1;
static lean_once_cell_t l_instMonadEnvAuditM___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* l_instMonadEnvAuditM___closed__2;
static lean_once_cell_t l_instMonadEnvAuditM___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* l_instMonadEnvAuditM___closed__3;
static lean_once_cell_t l_instMonadEnvAuditM___closed__4_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* l_instMonadEnvAuditM___closed__4;
static lean_once_cell_t l_instMonadEnvAuditM___closed__5_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* l_instMonadEnvAuditM___closed__5;
LEAN_EXPORT lean_object* l_instMonadEnvAuditM;
static const lean_string_object l_isNativeAxiom___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 24, .m_capacity = 24, .m_length = 23, .m_data = "._native.native_decide."};
static const lean_object* l_isNativeAxiom___closed__0 = (const lean_object*)&l_isNativeAxiom___closed__0_value;
static const lean_string_object l_isNativeAxiom___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "Lean"};
static const lean_object* l_isNativeAxiom___closed__1 = (const lean_object*)&l_isNativeAxiom___closed__1_value;
static const lean_string_object l_isNativeAxiom___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 13, .m_capacity = 13, .m_length = 12, .m_data = "ofReduceBool"};
static const lean_object* l_isNativeAxiom___closed__2 = (const lean_object*)&l_isNativeAxiom___closed__2_value;
static const lean_ctor_object l_isNativeAxiom___closed__3_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&l_isNativeAxiom___closed__1_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object l_isNativeAxiom___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&l_isNativeAxiom___closed__3_value_aux_0),((lean_object*)&l_isNativeAxiom___closed__2_value),LEAN_SCALAR_PTR_LITERAL(33, 77, 217, 63, 156, 187, 147, 213)}};
static const lean_object* l_isNativeAxiom___closed__3 = (const lean_object*)&l_isNativeAxiom___closed__3_value;
static const lean_string_object l_isNativeAxiom___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 14, .m_capacity = 14, .m_length = 13, .m_data = "trustCompiler"};
static const lean_object* l_isNativeAxiom___closed__4 = (const lean_object*)&l_isNativeAxiom___closed__4_value;
static const lean_ctor_object l_isNativeAxiom___closed__5_value_aux_0 = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&l_isNativeAxiom___closed__1_value),LEAN_SCALAR_PTR_LITERAL(70, 193, 83, 126, 233, 67, 208, 165)}};
static const lean_ctor_object l_isNativeAxiom___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 8, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)&l_isNativeAxiom___closed__5_value_aux_0),((lean_object*)&l_isNativeAxiom___closed__4_value),LEAN_SCALAR_PTR_LITERAL(121, 250, 167, 102, 228, 12, 150, 57)}};
static const lean_object* l_isNativeAxiom___closed__5 = (const lean_object*)&l_isNativeAxiom___closed__5_value;
LEAN_EXPORT uint8_t l_isNativeAxiom(lean_object*);
LEAN_EXPORT lean_object* l_isNativeAxiom___boxed(lean_object*);
LEAN_EXPORT lean_object* l_List_filterTR_loop___at___00readTrusted_spec__0(lean_object*, lean_object*);
static const lean_string_object l_List_filterMapTR_go___at___00readTrusted_spec__1___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 1, .m_capacity = 1, .m_length = 0, .m_data = ""};
static const lean_object* l_List_filterMapTR_go___at___00readTrusted_spec__1___closed__0 = (const lean_object*)&l_List_filterMapTR_go___at___00readTrusted_spec__1___closed__0_value;
static const lean_string_object l_List_filterMapTR_go___at___00readTrusted_spec__1___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = "#"};
static const lean_object* l_List_filterMapTR_go___at___00readTrusted_spec__1___closed__1 = (const lean_object*)&l_List_filterMapTR_go___at___00readTrusted_spec__1___closed__1_value;
LEAN_EXPORT lean_object* l_List_filterMapTR_go___at___00readTrusted_spec__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_filterMapTR_go___at___00readTrusted_spec__1___boxed(lean_object*, lean_object*);
static const lean_array_object l_readTrusted___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_array_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 246}, .m_size = 0, .m_capacity = 0, .m_data = {}};
static const lean_object* l_readTrusted___closed__0 = (const lean_object*)&l_readTrusted___closed__0_value;
LEAN_EXPORT lean_object* l_readTrusted(lean_object*);
LEAN_EXPORT lean_object* l_readTrusted___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Lean_collectAxioms___at___00main_spec__2(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_Lean_collectAxioms___at___00main_spec__2___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_main___lam__0(lean_object*, lean_object*, uint8_t, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_main___lam__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__1(uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__1___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___lam__0(uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___lam__0___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7___redArg(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_filterTR_loop___at___00main_spec__3(uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_filterTR_loop___at___00main_spec__3___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t l_List_all___at___00main_spec__4(lean_object*);
LEAN_EXPORT lean_object* l_List_all___at___00main_spec__4___boxed(lean_object*);
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__5(uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__5___boxed(lean_object*, lean_object*, lean_object*);
static const lean_string_object l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 7, .m_capacity = 7, .m_length = 6, .m_data = "FAIL  "};
static const lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__0 = (const lean_object*)&l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__0_value;
static const lean_string_object l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 24, .m_capacity = 24, .m_length = 23, .m_data = "  non-standard axioms: "};
static const lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__1 = (const lean_object*)&l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__1_value;
static const lean_string_object l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 7, .m_capacity = 7, .m_length = 6, .m_data = "TRUST "};
static const lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__2 = (const lean_object*)&l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__2_value;
static const lean_string_object l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 48, .m_capacity = 48, .m_length = 47, .m_data = "  native_decide, recorded in the trust manifest"};
static const lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__3 = (const lean_object*)&l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__3_value;
static const lean_string_object l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 7, .m_capacity = 7, .m_length = 6, .m_data = "ok    "};
static const lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__4 = (const lean_object*)&l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__4_value;
static const lean_string_object l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 4, .m_capacity = 4, .m_length = 3, .m_data = "  ["};
static const lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__5 = (const lean_object*)&l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__5_value;
static const lean_string_object l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = ", "};
static const lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__6 = (const lean_object*)&l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__6_value;
static const lean_string_object l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__7_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = "]"};
static const lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__7 = (const lean_object*)&l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__7_value;
LEAN_EXPORT lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6(lean_object*, uint8_t, lean_object*, lean_object*, size_t, size_t, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_string_object l_main___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 9, .m_capacity = 9, .m_length = 8, .m_data = "audited "};
static const lean_object* l_main___closed__0 = (const lean_object*)&l_main___closed__0_value;
static const lean_string_object l_main___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 14, .m_capacity = 14, .m_length = 13, .m_data = " theorems in "};
static const lean_object* l_main___closed__1 = (const lean_object*)&l_main___closed__1_value;
static const lean_string_object l_main___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "; "};
static const lean_object* l_main___closed__2 = (const lean_object*)&l_main___closed__2_value;
static const lean_string_object l_main___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 11, .m_capacity = 11, .m_length = 10, .m_data = " failing, "};
static const lean_object* l_main___closed__3 = (const lean_object*)&l_main___closed__3_value;
static const lean_string_object l_main___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 25, .m_capacity = 25, .m_length = 24, .m_data = " trusted (native_decide)"};
static const lean_object* l_main___closed__4 = (const lean_object*)&l_main___closed__4_value;
static const lean_string_object l_main___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 56, .m_capacity = 56, .m_length = 55, .m_data = "no theorems found: refusing to pass an audit of nothing"};
static const lean_object* l_main___closed__5 = (const lean_object*)&l_main___closed__5_value;
static const lean_string_object l_main___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "lean"};
static const lean_object* l_main___closed__6 = (const lean_object*)&l_main___closed__6_value;
static const lean_array_object l_main___closed__7_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_array_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 246}, .m_size = 0, .m_capacity = 0, .m_data = {}};
static const lean_object* l_main___closed__7 = (const lean_object*)&l_main___closed__7_value;
static const lean_string_object l_main___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 60, .m_capacity = 60, .m_length = 59, .m_data = "usage: AuditAxioms [--trusted FILE] <Module> [<Module> ...]"};
static const lean_object* l_main___closed__8 = (const lean_object*)&l_main___closed__8_value;
static const lean_string_object l_main___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 10, .m_capacity = 10, .m_length = 9, .m_data = "--trusted"};
static const lean_object* l_main___closed__9 = (const lean_object*)&l_main___closed__9_value;
static const lean_string_object l_main___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "--"};
static const lean_object* l_main___closed__10 = (const lean_object*)&l_main___closed__10_value;
LEAN_EXPORT lean_object* l_main___boxed__const__1;
LEAN_EXPORT lean_object* l_main___boxed__const__2;
LEAN_EXPORT lean_object* _lean_main(lean_object*);
LEAN_EXPORT lean_object* l_main___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT uint8_t l_List_any___at___00inScope_spec__0(lean_object* v_m_24_, lean_object* v_x_25_){
_start:
{
if (lean_obj_tag(v_x_25_) == 0)
{
uint8_t v___x_26_; 
v___x_26_ = 0;
return v___x_26_;
}
else
{
lean_object* v_head_27_; lean_object* v_tail_28_; uint8_t v___y_30_; uint8_t v___x_32_; 
v_head_27_ = lean_ctor_get(v_x_25_, 0);
v_tail_28_ = lean_ctor_get(v_x_25_, 1);
v___x_32_ = lean_name_eq(v_head_27_, v_m_24_);
if (v___x_32_ == 0)
{
uint8_t v___x_33_; 
v___x_33_ = l_Lean_Name_isPrefixOf(v_head_27_, v_m_24_);
v___y_30_ = v___x_33_;
goto v___jp_29_;
}
else
{
v___y_30_ = v___x_32_;
goto v___jp_29_;
}
v___jp_29_:
{
if (v___y_30_ == 0)
{
v_x_25_ = v_tail_28_;
goto _start;
}
else
{
return v___y_30_;
}
}
}
}
}
LEAN_EXPORT lean_object* l_List_any___at___00inScope_spec__0___boxed(lean_object* v_m_34_, lean_object* v_x_35_){
_start:
{
uint8_t v_res_36_; lean_object* v_r_37_; 
v_res_36_ = l_List_any___at___00inScope_spec__0(v_m_34_, v_x_35_);
lean_dec(v_x_35_);
lean_dec(v_m_34_);
v_r_37_ = lean_box(v_res_36_);
return v_r_37_;
}
}
LEAN_EXPORT uint8_t l_inScope(lean_object* v_env_38_, lean_object* v_roots_39_, lean_object* v_c_40_){
_start:
{
lean_object* v___x_41_; 
v___x_41_ = l_Lean_Environment_getModuleIdxFor_x3f(v_env_38_, v_c_40_);
if (lean_obj_tag(v___x_41_) == 0)
{
uint8_t v___x_42_; 
v___x_42_ = 0;
return v___x_42_;
}
else
{
lean_object* v_val_43_; lean_object* v___x_44_; lean_object* v___x_45_; lean_object* v___x_46_; lean_object* v_m_47_; uint8_t v___x_48_; 
v_val_43_ = lean_ctor_get(v___x_41_, 0);
lean_inc(v_val_43_);
lean_dec_ref_known(v___x_41_, 1);
v___x_44_ = lean_box(0);
v___x_45_ = l_Lean_Environment_header(v_env_38_);
v___x_46_ = l_Lean_EnvironmentHeader_moduleNames(v___x_45_);
v_m_47_ = lean_array_get(v___x_44_, v___x_46_, v_val_43_);
lean_dec(v_val_43_);
lean_dec_ref(v___x_46_);
v___x_48_ = l_List_any___at___00inScope_spec__0(v_m_47_, v_roots_39_);
lean_dec(v_m_47_);
return v___x_48_;
}
}
}
LEAN_EXPORT lean_object* l_inScope___boxed(lean_object* v_env_49_, lean_object* v_roots_50_, lean_object* v_c_51_){
_start:
{
uint8_t v_res_52_; lean_object* v_r_53_; 
v_res_52_ = l_inScope(v_env_49_, v_roots_50_, v_c_51_);
lean_dec(v_c_51_);
lean_dec(v_roots_50_);
lean_dec_ref(v_env_49_);
v_r_53_ = lean_box(v_res_52_);
return v_r_53_;
}
}
static lean_object* _init_l_instMonadEnvAuditM___closed__0(void){
_start:
{
lean_object* v___x_54_; 
v___x_54_ = l_instMonadEIO(lean_box(0));
return v___x_54_;
}
}
static lean_object* _init_l_instMonadEnvAuditM___closed__1(void){
_start:
{
lean_object* v___x_55_; lean_object* v___x_56_; 
v___x_55_ = lean_obj_once(&l_instMonadEnvAuditM___closed__0, &l_instMonadEnvAuditM___closed__0_once, _init_l_instMonadEnvAuditM___closed__0);
v___x_56_ = l_instMonadStateOfStateTOfMonad___redArg(v___x_55_);
return v___x_56_;
}
}
static lean_object* _init_l_instMonadEnvAuditM___closed__2(void){
_start:
{
lean_object* v___x_57_; lean_object* v___x_58_; 
v___x_57_ = lean_obj_once(&l_instMonadEnvAuditM___closed__1, &l_instMonadEnvAuditM___closed__1_once, _init_l_instMonadEnvAuditM___closed__1);
v___x_58_ = l_instMonadStateOfMonadStateOf___redArg(v___x_57_);
return v___x_58_;
}
}
static lean_object* _init_l_instMonadEnvAuditM___closed__3(void){
_start:
{
lean_object* v___x_59_; lean_object* v___x_60_; 
v___x_59_ = lean_obj_once(&l_instMonadEnvAuditM___closed__0, &l_instMonadEnvAuditM___closed__0_once, _init_l_instMonadEnvAuditM___closed__0);
v___x_60_ = lean_alloc_closure((void*)(l_StateT_get), 4, 3);
lean_closure_set(v___x_60_, 0, lean_box(0));
lean_closure_set(v___x_60_, 1, lean_box(0));
lean_closure_set(v___x_60_, 2, v___x_59_);
return v___x_60_;
}
}
static lean_object* _init_l_instMonadEnvAuditM___closed__4(void){
_start:
{
lean_object* v___x_61_; lean_object* v___x_62_; 
v___x_61_ = lean_obj_once(&l_instMonadEnvAuditM___closed__2, &l_instMonadEnvAuditM___closed__2_once, _init_l_instMonadEnvAuditM___closed__2);
v___x_62_ = lean_alloc_closure((void*)(l_modify), 4, 3);
lean_closure_set(v___x_62_, 0, lean_box(0));
lean_closure_set(v___x_62_, 1, lean_box(0));
lean_closure_set(v___x_62_, 2, v___x_61_);
return v___x_62_;
}
}
static lean_object* _init_l_instMonadEnvAuditM___closed__5(void){
_start:
{
lean_object* v___x_63_; lean_object* v___x_64_; lean_object* v___x_65_; 
v___x_63_ = lean_obj_once(&l_instMonadEnvAuditM___closed__4, &l_instMonadEnvAuditM___closed__4_once, _init_l_instMonadEnvAuditM___closed__4);
v___x_64_ = lean_obj_once(&l_instMonadEnvAuditM___closed__3, &l_instMonadEnvAuditM___closed__3_once, _init_l_instMonadEnvAuditM___closed__3);
v___x_65_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_65_, 0, v___x_64_);
lean_ctor_set(v___x_65_, 1, v___x_63_);
return v___x_65_;
}
}
static lean_object* _init_l_instMonadEnvAuditM(void){
_start:
{
lean_object* v___x_66_; 
v___x_66_ = lean_obj_once(&l_instMonadEnvAuditM___closed__5, &l_instMonadEnvAuditM___closed__5_once, _init_l_instMonadEnvAuditM___closed__5);
return v___x_66_;
}
}
LEAN_EXPORT uint8_t l_isNativeAxiom(lean_object* v_a_77_){
_start:
{
uint8_t v___y_79_; lean_object* v___x_89_; uint8_t v___x_90_; 
v___x_89_ = ((lean_object*)(l_isNativeAxiom___closed__3));
v___x_90_ = lean_name_eq(v_a_77_, v___x_89_);
if (v___x_90_ == 0)
{
lean_object* v___x_91_; uint8_t v___x_92_; 
v___x_91_ = ((lean_object*)(l_isNativeAxiom___closed__5));
v___x_92_ = lean_name_eq(v_a_77_, v___x_91_);
v___y_79_ = v___x_92_;
goto v___jp_78_;
}
else
{
v___y_79_ = v___x_90_;
goto v___jp_78_;
}
v___jp_78_:
{
if (v___y_79_ == 0)
{
lean_object* v___x_80_; uint8_t v___x_81_; lean_object* v___x_82_; lean_object* v___x_83_; lean_object* v___x_84_; lean_object* v___x_85_; lean_object* v___x_86_; lean_object* v___x_87_; uint8_t v___x_88_; 
v___x_80_ = lean_unsigned_to_nat(1u);
v___x_81_ = 1;
v___x_82_ = l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(v_a_77_, v___x_81_);
v___x_83_ = ((lean_object*)(l_isNativeAxiom___closed__0));
v___x_84_ = lean_unsigned_to_nat(0u);
v___x_85_ = lean_box(0);
v___x_86_ = l_String_splitOnAux(v___x_82_, v___x_83_, v___x_84_, v___x_84_, v___x_84_, v___x_85_);
lean_dec_ref(v___x_82_);
v___x_87_ = l_List_lengthTR___redArg(v___x_86_);
lean_dec(v___x_86_);
v___x_88_ = lean_nat_dec_lt(v___x_80_, v___x_87_);
lean_dec(v___x_87_);
return v___x_88_;
}
else
{
lean_dec(v_a_77_);
return v___y_79_;
}
}
}
}
LEAN_EXPORT lean_object* l_isNativeAxiom___boxed(lean_object* v_a_93_){
_start:
{
uint8_t v_res_94_; lean_object* v_r_95_; 
v_res_94_ = l_isNativeAxiom(v_a_93_);
v_r_95_ = lean_box(v_res_94_);
return v_r_95_;
}
}
LEAN_EXPORT lean_object* l_List_filterTR_loop___at___00readTrusted_spec__0(lean_object* v_a_96_, lean_object* v_a_97_){
_start:
{
if (lean_obj_tag(v_a_96_) == 0)
{
lean_object* v___x_98_; 
v___x_98_ = l_List_reverse___redArg(v_a_97_);
return v___x_98_;
}
else
{
lean_object* v_head_99_; lean_object* v_tail_100_; lean_object* v___x_102_; uint8_t v_isShared_103_; uint8_t v_isSharedCheck_126_; 
v_head_99_ = lean_ctor_get(v_a_96_, 0);
v_tail_100_ = lean_ctor_get(v_a_96_, 1);
v_isSharedCheck_126_ = !lean_is_exclusive(v_a_96_);
if (v_isSharedCheck_126_ == 0)
{
v___x_102_ = v_a_96_;
v_isShared_103_ = v_isSharedCheck_126_;
goto v_resetjp_101_;
}
else
{
lean_inc(v_tail_100_);
lean_inc(v_head_99_);
lean_dec(v_a_96_);
v___x_102_ = lean_box(0);
v_isShared_103_ = v_isSharedCheck_126_;
goto v_resetjp_101_;
}
v_resetjp_101_:
{
uint8_t v___y_105_; uint8_t v___y_112_; uint32_t v___x_120_; uint32_t v___x_121_; uint8_t v___x_122_; 
v___x_120_ = 32;
v___x_121_ = lean_unbox_uint32(v_head_99_);
v___x_122_ = lean_uint32_dec_eq(v___x_121_, v___x_120_);
if (v___x_122_ == 0)
{
uint32_t v___x_123_; uint32_t v___x_124_; uint8_t v___x_125_; 
v___x_123_ = 9;
v___x_124_ = lean_unbox_uint32(v_head_99_);
v___x_125_ = lean_uint32_dec_eq(v___x_124_, v___x_123_);
v___y_112_ = v___x_125_;
goto v___jp_111_;
}
else
{
v___y_112_ = v___x_122_;
goto v___jp_111_;
}
v___jp_104_:
{
if (v___y_105_ == 0)
{
lean_object* v___x_107_; 
if (v_isShared_103_ == 0)
{
lean_ctor_set(v___x_102_, 1, v_a_97_);
v___x_107_ = v___x_102_;
goto v_reusejp_106_;
}
else
{
lean_object* v_reuseFailAlloc_109_; 
v_reuseFailAlloc_109_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_109_, 0, v_head_99_);
lean_ctor_set(v_reuseFailAlloc_109_, 1, v_a_97_);
v___x_107_ = v_reuseFailAlloc_109_;
goto v_reusejp_106_;
}
v_reusejp_106_:
{
v_a_96_ = v_tail_100_;
v_a_97_ = v___x_107_;
goto _start;
}
}
else
{
lean_del_object(v___x_102_);
lean_dec(v_head_99_);
v_a_96_ = v_tail_100_;
goto _start;
}
}
v___jp_111_:
{
if (v___y_112_ == 0)
{
uint32_t v___x_113_; uint32_t v___x_114_; uint8_t v___x_115_; 
v___x_113_ = 13;
v___x_114_ = lean_unbox_uint32(v_head_99_);
v___x_115_ = lean_uint32_dec_eq(v___x_114_, v___x_113_);
if (v___x_115_ == 0)
{
uint32_t v___x_116_; uint32_t v___x_117_; uint8_t v___x_118_; 
v___x_116_ = 10;
v___x_117_ = lean_unbox_uint32(v_head_99_);
v___x_118_ = lean_uint32_dec_eq(v___x_117_, v___x_116_);
v___y_105_ = v___x_118_;
goto v___jp_104_;
}
else
{
v___y_105_ = v___x_115_;
goto v___jp_104_;
}
}
else
{
lean_del_object(v___x_102_);
lean_dec(v_head_99_);
v_a_96_ = v_tail_100_;
goto _start;
}
}
}
}
}
}
LEAN_EXPORT lean_object* l_List_filterMapTR_go___at___00readTrusted_spec__1(lean_object* v_a_129_, lean_object* v_a_130_){
_start:
{
if (lean_obj_tag(v_a_129_) == 0)
{
lean_object* v___x_131_; 
v___x_131_ = lean_array_to_list(v_a_130_);
return v___x_131_;
}
else
{
lean_object* v_head_132_; lean_object* v_tail_133_; lean_object* v___x_134_; lean_object* v___x_135_; lean_object* v___x_136_; lean_object* v___x_137_; lean_object* v___x_138_; lean_object* v___x_139_; lean_object* v___x_140_; lean_object* v___x_141_; lean_object* v___x_142_; lean_object* v___x_143_; uint8_t v___x_144_; 
v_head_132_ = lean_ctor_get(v_a_129_, 0);
v_tail_133_ = lean_ctor_get(v_a_129_, 1);
v___x_134_ = ((lean_object*)(l_List_filterMapTR_go___at___00readTrusted_spec__1___closed__0));
v___x_135_ = ((lean_object*)(l_List_filterMapTR_go___at___00readTrusted_spec__1___closed__1));
v___x_136_ = lean_unsigned_to_nat(0u);
v___x_137_ = lean_box(0);
v___x_138_ = l_String_splitOnAux(v_head_132_, v___x_135_, v___x_136_, v___x_136_, v___x_136_, v___x_137_);
v___x_139_ = l_List_head_x21___redArg(v___x_134_, v___x_138_);
lean_dec(v___x_138_);
v___x_140_ = lean_string_data(v___x_139_);
v___x_141_ = l_List_filterTR_loop___at___00readTrusted_spec__0(v___x_140_, v___x_137_);
v___x_142_ = lean_string_mk(v___x_141_);
v___x_143_ = lean_string_utf8_byte_size(v___x_142_);
v___x_144_ = lean_nat_dec_eq(v___x_143_, v___x_136_);
if (v___x_144_ == 0)
{
lean_object* v___x_145_; lean_object* v___x_146_; 
v___x_145_ = l_String_toName(v___x_142_);
v___x_146_ = lean_array_push(v_a_130_, v___x_145_);
v_a_129_ = v_tail_133_;
v_a_130_ = v___x_146_;
goto _start;
}
else
{
lean_dec_ref(v___x_142_);
v_a_129_ = v_tail_133_;
goto _start;
}
}
}
}
LEAN_EXPORT lean_object* l_List_filterMapTR_go___at___00readTrusted_spec__1___boxed(lean_object* v_a_149_, lean_object* v_a_150_){
_start:
{
lean_object* v_res_151_; 
v_res_151_ = l_List_filterMapTR_go___at___00readTrusted_spec__1(v_a_149_, v_a_150_);
lean_dec(v_a_149_);
return v_res_151_;
}
}
LEAN_EXPORT lean_object* l_readTrusted(lean_object* v_path_154_){
_start:
{
lean_object* v___x_156_; 
v___x_156_ = l_IO_FS_lines(v_path_154_);
if (lean_obj_tag(v___x_156_) == 0)
{
lean_object* v_a_157_; lean_object* v___x_159_; uint8_t v_isShared_160_; uint8_t v_isSharedCheck_167_; 
v_a_157_ = lean_ctor_get(v___x_156_, 0);
v_isSharedCheck_167_ = !lean_is_exclusive(v___x_156_);
if (v_isSharedCheck_167_ == 0)
{
v___x_159_ = v___x_156_;
v_isShared_160_ = v_isSharedCheck_167_;
goto v_resetjp_158_;
}
else
{
lean_inc(v_a_157_);
lean_dec(v___x_156_);
v___x_159_ = lean_box(0);
v_isShared_160_ = v_isSharedCheck_167_;
goto v_resetjp_158_;
}
v_resetjp_158_:
{
lean_object* v___x_161_; lean_object* v___x_162_; lean_object* v___x_163_; lean_object* v___x_165_; 
v___x_161_ = lean_array_to_list(v_a_157_);
v___x_162_ = ((lean_object*)(l_readTrusted___closed__0));
v___x_163_ = l_List_filterMapTR_go___at___00readTrusted_spec__1(v___x_161_, v___x_162_);
lean_dec(v___x_161_);
if (v_isShared_160_ == 0)
{
lean_ctor_set(v___x_159_, 0, v___x_163_);
v___x_165_ = v___x_159_;
goto v_reusejp_164_;
}
else
{
lean_object* v_reuseFailAlloc_166_; 
v_reuseFailAlloc_166_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_166_, 0, v___x_163_);
v___x_165_ = v_reuseFailAlloc_166_;
goto v_reusejp_164_;
}
v_reusejp_164_:
{
return v___x_165_;
}
}
}
else
{
lean_object* v_a_168_; lean_object* v___x_170_; uint8_t v_isShared_171_; uint8_t v_isSharedCheck_175_; 
v_a_168_ = lean_ctor_get(v___x_156_, 0);
v_isSharedCheck_175_ = !lean_is_exclusive(v___x_156_);
if (v_isSharedCheck_175_ == 0)
{
v___x_170_ = v___x_156_;
v_isShared_171_ = v_isSharedCheck_175_;
goto v_resetjp_169_;
}
else
{
lean_inc(v_a_168_);
lean_dec(v___x_156_);
v___x_170_ = lean_box(0);
v_isShared_171_ = v_isSharedCheck_175_;
goto v_resetjp_169_;
}
v_resetjp_169_:
{
lean_object* v___x_173_; 
if (v_isShared_171_ == 0)
{
v___x_173_ = v___x_170_;
goto v_reusejp_172_;
}
else
{
lean_object* v_reuseFailAlloc_174_; 
v_reuseFailAlloc_174_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_174_, 0, v_a_168_);
v___x_173_ = v_reuseFailAlloc_174_;
goto v_reusejp_172_;
}
v_reusejp_172_:
{
return v___x_173_;
}
}
}
}
}
LEAN_EXPORT lean_object* l_readTrusted___boxed(lean_object* v_path_176_, lean_object* v_a_177_){
_start:
{
lean_object* v_res_178_; 
v_res_178_ = l_readTrusted(v_path_176_);
lean_dec_ref(v_path_176_);
return v_res_178_;
}
}
LEAN_EXPORT lean_object* l_Lean_collectAxioms___at___00main_spec__2(lean_object* v_constName_179_, lean_object* v___y_180_){
_start:
{
lean_object* v___x_182_; uint8_t v___x_183_; lean_object* v_privateEnv_184_; lean_object* v___x_185_; lean_object* v___x_186_; lean_object* v___x_187_; lean_object* v_s_188_; lean_object* v___x_189_; lean_object* v___x_190_; lean_object* v___x_191_; lean_object* v___x_192_; lean_object* v___x_193_; 
v___x_182_ = l___private_Lean_Util_CollectAxioms_0__Lean_instInhabitedExportedAxiomsState;
v___x_183_ = 0;
lean_inc_ref_n(v___y_180_, 2);
v_privateEnv_184_ = l_Lean_Environment_setExporting(v___y_180_, v___x_183_);
v___x_185_ = l___private_Lean_Util_CollectAxioms_0__Lean_exportedAxiomsExt;
v___x_186_ = lean_box(2);
v___x_187_ = lean_box(0);
v_s_188_ = l_Lean_PersistentEnvExtension_getState___redArg(v___x_182_, v___x_185_, v___y_180_, v___x_186_, v___x_187_);
v___x_189_ = lean_alloc_closure((void*)(l___private_Lean_Util_CollectAxioms_0__Lean_ExportedAxiomsState_find_x3f___boxed), 3, 1);
lean_closure_set(v___x_189_, 0, v_s_188_);
v___x_190_ = lean_alloc_closure((void*)(l___private_Lean_Util_CollectAxioms_0__Lean_CollectAxioms_collectAndGet___boxed), 4, 2);
lean_closure_set(v___x_190_, 0, v___x_189_);
lean_closure_set(v___x_190_, 1, v_constName_179_);
v___x_191_ = l___private_Lean_Util_CollectAxioms_0__Lean_CollectAxioms_runM___redArg(v_privateEnv_184_, v___x_190_);
v___x_192_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_192_, 0, v___x_191_);
lean_ctor_set(v___x_192_, 1, v___y_180_);
v___x_193_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_193_, 0, v___x_192_);
return v___x_193_;
}
}
LEAN_EXPORT lean_object* l_Lean_collectAxioms___at___00main_spec__2___boxed(lean_object* v_constName_194_, lean_object* v___y_195_, lean_object* v___y_196_){
_start:
{
lean_object* v_res_197_; 
v_res_197_ = l_Lean_collectAxioms___at___00main_spec__2(v_constName_194_, v___y_195_);
return v_res_197_;
}
}
LEAN_EXPORT lean_object* l_main___lam__0(lean_object* v_a_198_, lean_object* v_roots_199_, uint8_t v___x_200_, lean_object* v_acc_201_, lean_object* v_n_202_, lean_object* v_ci_203_){
_start:
{
if (lean_obj_tag(v_ci_203_) == 2)
{
uint8_t v___x_207_; 
v___x_207_ = l_Lean_Name_isInternal(v_n_202_);
if (v___x_207_ == 0)
{
goto v___jp_204_;
}
else
{
if (v___x_200_ == 0)
{
lean_dec(v_n_202_);
return v_acc_201_;
}
else
{
goto v___jp_204_;
}
}
}
else
{
lean_dec(v_n_202_);
return v_acc_201_;
}
v___jp_204_:
{
uint8_t v___x_205_; 
v___x_205_ = l_inScope(v_a_198_, v_roots_199_, v_n_202_);
if (v___x_205_ == 0)
{
lean_dec(v_n_202_);
return v_acc_201_;
}
else
{
lean_object* v___x_206_; 
v___x_206_ = lean_array_push(v_acc_201_, v_n_202_);
return v___x_206_;
}
}
}
}
LEAN_EXPORT lean_object* l_main___lam__0___boxed(lean_object* v_a_208_, lean_object* v_roots_209_, lean_object* v___x_210_, lean_object* v_acc_211_, lean_object* v_n_212_, lean_object* v_ci_213_){
_start:
{
uint8_t v___x_5004__boxed_214_; lean_object* v_res_215_; 
v___x_5004__boxed_214_ = lean_unbox(v___x_210_);
v_res_215_ = l_main___lam__0(v_a_208_, v_roots_209_, v___x_5004__boxed_214_, v_acc_211_, v_n_212_, v_ci_213_);
lean_dec_ref(v_ci_213_);
lean_dec(v_roots_209_);
lean_dec_ref(v_a_208_);
return v_res_215_;
}
}
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__0(lean_object* v_a_216_, lean_object* v_a_217_){
_start:
{
if (lean_obj_tag(v_a_216_) == 0)
{
lean_object* v___x_218_; 
v___x_218_ = l_List_reverse___redArg(v_a_217_);
return v___x_218_;
}
else
{
lean_object* v_head_219_; lean_object* v_tail_220_; lean_object* v___x_222_; uint8_t v_isShared_223_; uint8_t v_isSharedCheck_229_; 
v_head_219_ = lean_ctor_get(v_a_216_, 0);
v_tail_220_ = lean_ctor_get(v_a_216_, 1);
v_isSharedCheck_229_ = !lean_is_exclusive(v_a_216_);
if (v_isSharedCheck_229_ == 0)
{
v___x_222_ = v_a_216_;
v_isShared_223_ = v_isSharedCheck_229_;
goto v_resetjp_221_;
}
else
{
lean_inc(v_tail_220_);
lean_inc(v_head_219_);
lean_dec(v_a_216_);
v___x_222_ = lean_box(0);
v_isShared_223_ = v_isSharedCheck_229_;
goto v_resetjp_221_;
}
v_resetjp_221_:
{
lean_object* v___x_224_; lean_object* v___x_226_; 
v___x_224_ = l_String_toName(v_head_219_);
if (v_isShared_223_ == 0)
{
lean_ctor_set(v___x_222_, 1, v_a_217_);
lean_ctor_set(v___x_222_, 0, v___x_224_);
v___x_226_ = v___x_222_;
goto v_reusejp_225_;
}
else
{
lean_object* v_reuseFailAlloc_228_; 
v_reuseFailAlloc_228_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_228_, 0, v___x_224_);
lean_ctor_set(v_reuseFailAlloc_228_, 1, v_a_217_);
v___x_226_ = v_reuseFailAlloc_228_;
goto v_reusejp_225_;
}
v_reusejp_225_:
{
v_a_216_ = v_tail_220_;
v_a_217_ = v___x_226_;
goto _start;
}
}
}
}
}
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__1(uint8_t v___x_230_, lean_object* v_a_231_, lean_object* v_a_232_){
_start:
{
if (lean_obj_tag(v_a_231_) == 0)
{
lean_object* v___x_233_; 
v___x_233_ = l_List_reverse___redArg(v_a_232_);
return v___x_233_;
}
else
{
lean_object* v_head_234_; lean_object* v_tail_235_; lean_object* v___x_237_; uint8_t v_isShared_238_; uint8_t v_isSharedCheck_245_; 
v_head_234_ = lean_ctor_get(v_a_231_, 0);
v_tail_235_ = lean_ctor_get(v_a_231_, 1);
v_isSharedCheck_245_ = !lean_is_exclusive(v_a_231_);
if (v_isSharedCheck_245_ == 0)
{
v___x_237_ = v_a_231_;
v_isShared_238_ = v_isSharedCheck_245_;
goto v_resetjp_236_;
}
else
{
lean_inc(v_tail_235_);
lean_inc(v_head_234_);
lean_dec(v_a_231_);
v___x_237_ = lean_box(0);
v_isShared_238_ = v_isSharedCheck_245_;
goto v_resetjp_236_;
}
v_resetjp_236_:
{
uint8_t v___x_239_; lean_object* v___x_240_; lean_object* v___x_242_; 
v___x_239_ = 1;
v___x_240_ = lean_alloc_ctor(0, 1, 3);
lean_ctor_set(v___x_240_, 0, v_head_234_);
lean_ctor_set_uint8(v___x_240_, sizeof(void*)*1, v___x_230_);
lean_ctor_set_uint8(v___x_240_, sizeof(void*)*1 + 1, v___x_239_);
lean_ctor_set_uint8(v___x_240_, sizeof(void*)*1 + 2, v___x_230_);
if (v_isShared_238_ == 0)
{
lean_ctor_set(v___x_237_, 1, v_a_232_);
lean_ctor_set(v___x_237_, 0, v___x_240_);
v___x_242_ = v___x_237_;
goto v_reusejp_241_;
}
else
{
lean_object* v_reuseFailAlloc_244_; 
v_reuseFailAlloc_244_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_244_, 0, v___x_240_);
lean_ctor_set(v_reuseFailAlloc_244_, 1, v_a_232_);
v___x_242_ = v_reuseFailAlloc_244_;
goto v_reusejp_241_;
}
v_reusejp_241_:
{
v_a_231_ = v_tail_235_;
v_a_232_ = v___x_242_;
goto _start;
}
}
}
}
}
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__1___boxed(lean_object* v___x_246_, lean_object* v_a_247_, lean_object* v_a_248_){
_start:
{
uint8_t v___x_5046__boxed_249_; lean_object* v_res_250_; 
v___x_5046__boxed_249_ = lean_unbox(v___x_246_);
v_res_250_ = l_List_mapTR_loop___at___00main_spec__1(v___x_5046__boxed_249_, v_a_247_, v_a_248_);
return v_res_250_;
}
}
LEAN_EXPORT uint8_t l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___lam__0(uint8_t v___x_251_, lean_object* v_x1_252_, lean_object* v_x2_253_){
_start:
{
lean_object* v___x_254_; lean_object* v___x_255_; uint8_t v___x_256_; 
v___x_254_ = l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(v_x1_252_, v___x_251_);
v___x_255_ = l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(v_x2_253_, v___x_251_);
v___x_256_ = lean_string_dec_lt(v___x_254_, v___x_255_);
lean_dec_ref(v___x_255_);
lean_dec_ref(v___x_254_);
return v___x_256_;
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___lam__0___boxed(lean_object* v___x_257_, lean_object* v_x1_258_, lean_object* v_x2_259_){
_start:
{
uint8_t v___x_5081__boxed_260_; uint8_t v_res_261_; lean_object* v_r_262_; 
v___x_5081__boxed_260_ = lean_unbox(v___x_257_);
v_res_261_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___lam__0(v___x_5081__boxed_260_, v_x1_258_, v_x2_259_);
v_r_262_ = lean_box(v_res_261_);
return v_r_262_;
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7___redArg(lean_object* v_hi_263_, lean_object* v_pivot_264_, lean_object* v_as_265_, lean_object* v_i_266_, lean_object* v_k_267_){
_start:
{
uint8_t v___x_268_; 
v___x_268_ = lean_nat_dec_lt(v_k_267_, v_hi_263_);
if (v___x_268_ == 0)
{
lean_object* v___x_269_; lean_object* v___x_270_; 
lean_dec(v_k_267_);
lean_dec(v_pivot_264_);
v___x_269_ = lean_array_fswap(v_as_265_, v_i_266_, v_hi_263_);
v___x_270_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_270_, 0, v_i_266_);
lean_ctor_set(v___x_270_, 1, v___x_269_);
return v___x_270_;
}
else
{
lean_object* v___x_271_; lean_object* v___x_272_; lean_object* v___x_273_; uint8_t v___x_274_; 
v___x_271_ = lean_array_fget_borrowed(v_as_265_, v_k_267_);
lean_inc(v___x_271_);
v___x_272_ = l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(v___x_271_, v___x_268_);
lean_inc(v_pivot_264_);
v___x_273_ = l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(v_pivot_264_, v___x_268_);
v___x_274_ = lean_string_dec_lt(v___x_272_, v___x_273_);
lean_dec_ref(v___x_273_);
lean_dec_ref(v___x_272_);
if (v___x_274_ == 0)
{
lean_object* v___x_275_; lean_object* v___x_276_; 
v___x_275_ = lean_unsigned_to_nat(1u);
v___x_276_ = lean_nat_add(v_k_267_, v___x_275_);
lean_dec(v_k_267_);
v_k_267_ = v___x_276_;
goto _start;
}
else
{
lean_object* v___x_278_; lean_object* v___x_279_; lean_object* v___x_280_; lean_object* v___x_281_; 
v___x_278_ = lean_array_fswap(v_as_265_, v_i_266_, v_k_267_);
v___x_279_ = lean_unsigned_to_nat(1u);
v___x_280_ = lean_nat_add(v_i_266_, v___x_279_);
lean_dec(v_i_266_);
v___x_281_ = lean_nat_add(v_k_267_, v___x_279_);
lean_dec(v_k_267_);
v_as_265_ = v___x_278_;
v_i_266_ = v___x_280_;
v_k_267_ = v___x_281_;
goto _start;
}
}
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7___redArg___boxed(lean_object* v_hi_283_, lean_object* v_pivot_284_, lean_object* v_as_285_, lean_object* v_i_286_, lean_object* v_k_287_){
_start:
{
lean_object* v_res_288_; 
v_res_288_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7___redArg(v_hi_283_, v_pivot_284_, v_as_285_, v_i_286_, v_k_287_);
lean_dec(v_hi_283_);
return v_res_288_;
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg(lean_object* v_n_289_, lean_object* v_as_290_, lean_object* v_lo_291_, lean_object* v_hi_292_){
_start:
{
lean_object* v___y_294_; uint8_t v___x_304_; 
v___x_304_ = lean_nat_dec_lt(v_lo_291_, v_hi_292_);
if (v___x_304_ == 0)
{
lean_dec(v_lo_291_);
return v_as_290_;
}
else
{
lean_object* v___x_305_; lean_object* v___x_306_; lean_object* v_mid_307_; lean_object* v___y_309_; lean_object* v___y_315_; lean_object* v___x_320_; lean_object* v___x_321_; uint8_t v___x_322_; 
v___x_305_ = lean_nat_add(v_lo_291_, v_hi_292_);
v___x_306_ = lean_unsigned_to_nat(1u);
v_mid_307_ = lean_nat_shiftr(v___x_305_, v___x_306_);
lean_dec(v___x_305_);
v___x_320_ = lean_array_fget_borrowed(v_as_290_, v_mid_307_);
v___x_321_ = lean_array_fget_borrowed(v_as_290_, v_lo_291_);
lean_inc(v___x_321_);
lean_inc(v___x_320_);
v___x_322_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___lam__0(v___x_304_, v___x_320_, v___x_321_);
if (v___x_322_ == 0)
{
v___y_315_ = v_as_290_;
goto v___jp_314_;
}
else
{
lean_object* v___x_323_; 
v___x_323_ = lean_array_fswap(v_as_290_, v_lo_291_, v_mid_307_);
v___y_315_ = v___x_323_;
goto v___jp_314_;
}
v___jp_308_:
{
lean_object* v___x_310_; lean_object* v___x_311_; uint8_t v___x_312_; 
v___x_310_ = lean_array_fget_borrowed(v___y_309_, v_mid_307_);
v___x_311_ = lean_array_fget_borrowed(v___y_309_, v_hi_292_);
lean_inc(v___x_311_);
lean_inc(v___x_310_);
v___x_312_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___lam__0(v___x_304_, v___x_310_, v___x_311_);
if (v___x_312_ == 0)
{
lean_dec(v_mid_307_);
v___y_294_ = v___y_309_;
goto v___jp_293_;
}
else
{
lean_object* v___x_313_; 
v___x_313_ = lean_array_fswap(v___y_309_, v_mid_307_, v_hi_292_);
lean_dec(v_mid_307_);
v___y_294_ = v___x_313_;
goto v___jp_293_;
}
}
v___jp_314_:
{
lean_object* v___x_316_; lean_object* v___x_317_; uint8_t v___x_318_; 
v___x_316_ = lean_array_fget_borrowed(v___y_315_, v_hi_292_);
v___x_317_ = lean_array_fget_borrowed(v___y_315_, v_lo_291_);
lean_inc(v___x_317_);
lean_inc(v___x_316_);
v___x_318_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___lam__0(v___x_304_, v___x_316_, v___x_317_);
if (v___x_318_ == 0)
{
v___y_309_ = v___y_315_;
goto v___jp_308_;
}
else
{
lean_object* v___x_319_; 
v___x_319_ = lean_array_fswap(v___y_315_, v_lo_291_, v_hi_292_);
v___y_309_ = v___x_319_;
goto v___jp_308_;
}
}
}
v___jp_293_:
{
lean_object* v_pivot_295_; lean_object* v___x_296_; lean_object* v_fst_297_; lean_object* v_snd_298_; uint8_t v___x_299_; 
v_pivot_295_ = lean_array_fget(v___y_294_, v_hi_292_);
lean_inc_n(v_lo_291_, 2);
v___x_296_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7___redArg(v_hi_292_, v_pivot_295_, v___y_294_, v_lo_291_, v_lo_291_);
v_fst_297_ = lean_ctor_get(v___x_296_, 0);
lean_inc(v_fst_297_);
v_snd_298_ = lean_ctor_get(v___x_296_, 1);
lean_inc(v_snd_298_);
lean_dec_ref(v___x_296_);
v___x_299_ = lean_nat_dec_le(v_hi_292_, v_fst_297_);
if (v___x_299_ == 0)
{
lean_object* v___x_300_; lean_object* v___x_301_; lean_object* v___x_302_; 
v___x_300_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg(v_n_289_, v_snd_298_, v_lo_291_, v_fst_297_);
v___x_301_ = lean_unsigned_to_nat(1u);
v___x_302_ = lean_nat_add(v_fst_297_, v___x_301_);
lean_dec(v_fst_297_);
v_as_290_ = v___x_300_;
v_lo_291_ = v___x_302_;
goto _start;
}
else
{
lean_dec(v_fst_297_);
lean_dec(v_lo_291_);
return v_snd_298_;
}
}
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg___boxed(lean_object* v_n_324_, lean_object* v_as_325_, lean_object* v_lo_326_, lean_object* v_hi_327_){
_start:
{
lean_object* v_res_328_; 
v_res_328_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg(v_n_324_, v_as_325_, v_lo_326_, v_hi_327_);
lean_dec(v_hi_327_);
lean_dec(v_n_324_);
return v_res_328_;
}
}
LEAN_EXPORT lean_object* l_List_filterTR_loop___at___00main_spec__3(uint8_t v___x_329_, lean_object* v_a_330_, lean_object* v_a_331_){
_start:
{
if (lean_obj_tag(v_a_330_) == 0)
{
lean_object* v___x_332_; 
v___x_332_ = l_List_reverse___redArg(v_a_331_);
return v___x_332_;
}
else
{
lean_object* v_head_333_; lean_object* v_tail_334_; lean_object* v___x_336_; uint8_t v_isShared_337_; uint8_t v_isSharedCheck_346_; 
v_head_333_ = lean_ctor_get(v_a_330_, 0);
v_tail_334_ = lean_ctor_get(v_a_330_, 1);
v_isSharedCheck_346_ = !lean_is_exclusive(v_a_330_);
if (v_isSharedCheck_346_ == 0)
{
v___x_336_ = v_a_330_;
v_isShared_337_ = v_isSharedCheck_346_;
goto v_resetjp_335_;
}
else
{
lean_inc(v_tail_334_);
lean_inc(v_head_333_);
lean_dec(v_a_330_);
v___x_336_ = lean_box(0);
v_isShared_337_ = v_isSharedCheck_346_;
goto v_resetjp_335_;
}
v_resetjp_335_:
{
lean_object* v___x_343_; uint8_t v___x_344_; 
v___x_343_ = ((lean_object*)(l_standardAxioms));
v___x_344_ = l_List_elem___at___00__private_Lean_Server_Completion_CompletionCollectors_0__Lean_Server_Completion_bestLabelForDecl_x3f_spec__0(v_head_333_, v___x_343_);
if (v___x_344_ == 0)
{
goto v___jp_338_;
}
else
{
if (v___x_329_ == 0)
{
lean_del_object(v___x_336_);
lean_dec(v_head_333_);
v_a_330_ = v_tail_334_;
goto _start;
}
else
{
goto v___jp_338_;
}
}
v___jp_338_:
{
lean_object* v___x_340_; 
if (v_isShared_337_ == 0)
{
lean_ctor_set(v___x_336_, 1, v_a_331_);
v___x_340_ = v___x_336_;
goto v_reusejp_339_;
}
else
{
lean_object* v_reuseFailAlloc_342_; 
v_reuseFailAlloc_342_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_342_, 0, v_head_333_);
lean_ctor_set(v_reuseFailAlloc_342_, 1, v_a_331_);
v___x_340_ = v_reuseFailAlloc_342_;
goto v_reusejp_339_;
}
v_reusejp_339_:
{
v_a_330_ = v_tail_334_;
v_a_331_ = v___x_340_;
goto _start;
}
}
}
}
}
}
LEAN_EXPORT lean_object* l_List_filterTR_loop___at___00main_spec__3___boxed(lean_object* v___x_347_, lean_object* v_a_348_, lean_object* v_a_349_){
_start:
{
uint8_t v___x_5184__boxed_350_; lean_object* v_res_351_; 
v___x_5184__boxed_350_ = lean_unbox(v___x_347_);
v_res_351_ = l_List_filterTR_loop___at___00main_spec__3(v___x_5184__boxed_350_, v_a_348_, v_a_349_);
return v_res_351_;
}
}
LEAN_EXPORT uint8_t l_List_all___at___00main_spec__4(lean_object* v_x_352_){
_start:
{
if (lean_obj_tag(v_x_352_) == 0)
{
uint8_t v___x_353_; 
v___x_353_ = 1;
return v___x_353_;
}
else
{
lean_object* v_head_354_; lean_object* v_tail_355_; uint8_t v___x_356_; 
v_head_354_ = lean_ctor_get(v_x_352_, 0);
lean_inc(v_head_354_);
v_tail_355_ = lean_ctor_get(v_x_352_, 1);
lean_inc(v_tail_355_);
lean_dec_ref_known(v_x_352_, 2);
v___x_356_ = l_isNativeAxiom(v_head_354_);
if (v___x_356_ == 0)
{
lean_dec(v_tail_355_);
return v___x_356_;
}
else
{
v_x_352_ = v_tail_355_;
goto _start;
}
}
}
}
LEAN_EXPORT lean_object* l_List_all___at___00main_spec__4___boxed(lean_object* v_x_358_){
_start:
{
uint8_t v_res_359_; lean_object* v_r_360_; 
v_res_359_ = l_List_all___at___00main_spec__4(v_x_358_);
v_r_360_ = lean_box(v_res_359_);
return v_r_360_;
}
}
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__5(uint8_t v___x_361_, lean_object* v_a_362_, lean_object* v_a_363_){
_start:
{
if (lean_obj_tag(v_a_362_) == 0)
{
lean_object* v___x_364_; 
v___x_364_ = l_List_reverse___redArg(v_a_363_);
return v___x_364_;
}
else
{
lean_object* v_head_365_; lean_object* v_tail_366_; lean_object* v___x_368_; uint8_t v_isShared_369_; uint8_t v_isSharedCheck_375_; 
v_head_365_ = lean_ctor_get(v_a_362_, 0);
v_tail_366_ = lean_ctor_get(v_a_362_, 1);
v_isSharedCheck_375_ = !lean_is_exclusive(v_a_362_);
if (v_isSharedCheck_375_ == 0)
{
v___x_368_ = v_a_362_;
v_isShared_369_ = v_isSharedCheck_375_;
goto v_resetjp_367_;
}
else
{
lean_inc(v_tail_366_);
lean_inc(v_head_365_);
lean_dec(v_a_362_);
v___x_368_ = lean_box(0);
v_isShared_369_ = v_isSharedCheck_375_;
goto v_resetjp_367_;
}
v_resetjp_367_:
{
lean_object* v___x_370_; lean_object* v___x_372_; 
v___x_370_ = l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(v_head_365_, v___x_361_);
if (v_isShared_369_ == 0)
{
lean_ctor_set(v___x_368_, 1, v_a_363_);
lean_ctor_set(v___x_368_, 0, v___x_370_);
v___x_372_ = v___x_368_;
goto v_reusejp_371_;
}
else
{
lean_object* v_reuseFailAlloc_374_; 
v_reuseFailAlloc_374_ = lean_alloc_ctor(1, 2, 0);
lean_ctor_set(v_reuseFailAlloc_374_, 0, v___x_370_);
lean_ctor_set(v_reuseFailAlloc_374_, 1, v_a_363_);
v___x_372_ = v_reuseFailAlloc_374_;
goto v_reusejp_371_;
}
v_reusejp_371_:
{
v_a_362_ = v_tail_366_;
v_a_363_ = v___x_372_;
goto _start;
}
}
}
}
}
LEAN_EXPORT lean_object* l_List_mapTR_loop___at___00main_spec__5___boxed(lean_object* v___x_376_, lean_object* v_a_377_, lean_object* v_a_378_){
_start:
{
uint8_t v___x_5236__boxed_379_; lean_object* v_res_380_; 
v___x_5236__boxed_379_ = lean_unbox(v___x_376_);
v_res_380_ = l_List_mapTR_loop___at___00main_spec__5(v___x_5236__boxed_379_, v_a_377_, v_a_378_);
return v_res_380_;
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6(lean_object* v_a_389_, uint8_t v___x_390_, lean_object* v_fst_391_, lean_object* v_as_392_, size_t v_sz_393_, size_t v_i_394_, lean_object* v_b_395_){
_start:
{
lean_object* v_a_398_; uint8_t v___x_402_; 
v___x_402_ = lean_usize_dec_lt(v_i_394_, v_sz_393_);
if (v___x_402_ == 0)
{
lean_object* v___x_403_; 
lean_dec_ref(v_a_389_);
v___x_403_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v___x_403_, 0, v_b_395_);
return v___x_403_;
}
else
{
lean_object* v_a_404_; lean_object* v___x_405_; 
v_a_404_ = lean_array_uget_borrowed(v_as_392_, v_i_394_);
lean_inc_ref(v_a_389_);
lean_inc(v_a_404_);
v___x_405_ = l_Lean_collectAxioms___at___00main_spec__2(v_a_404_, v_a_389_);
if (lean_obj_tag(v___x_405_) == 0)
{
lean_object* v_a_406_; lean_object* v_fst_407_; lean_object* v_snd_408_; lean_object* v_fst_409_; lean_object* v___x_411_; uint8_t v_isShared_412_; uint8_t v_isSharedCheck_483_; 
v_a_406_ = lean_ctor_get(v___x_405_, 0);
lean_inc(v_a_406_);
lean_dec_ref_known(v___x_405_, 1);
v_fst_407_ = lean_ctor_get(v_b_395_, 0);
lean_inc(v_fst_407_);
v_snd_408_ = lean_ctor_get(v_b_395_, 1);
lean_inc(v_snd_408_);
lean_dec_ref(v_b_395_);
v_fst_409_ = lean_ctor_get(v_a_406_, 0);
v_isSharedCheck_483_ = !lean_is_exclusive(v_a_406_);
if (v_isSharedCheck_483_ == 0)
{
lean_object* v_unused_484_; 
v_unused_484_ = lean_ctor_get(v_a_406_, 1);
lean_dec(v_unused_484_);
v___x_411_ = v_a_406_;
v_isShared_412_ = v_isSharedCheck_483_;
goto v_resetjp_410_;
}
else
{
lean_inc(v_fst_409_);
lean_dec(v_a_406_);
v___x_411_ = lean_box(0);
v_isShared_412_ = v_isSharedCheck_483_;
goto v_resetjp_410_;
}
v_resetjp_410_:
{
lean_object* v___x_413_; lean_object* v___x_414_; lean_object* v___x_415_; uint8_t v___x_416_; 
v___x_413_ = lean_array_to_list(v_fst_409_);
v___x_414_ = lean_box(0);
lean_inc(v___x_413_);
v___x_415_ = l_List_filterTR_loop___at___00main_spec__3(v___x_390_, v___x_413_, v___x_414_);
v___x_416_ = l_List_isEmpty___redArg(v___x_415_);
if (v___x_416_ == 0)
{
lean_object* v___x_417_; uint8_t v___y_419_; uint8_t v___x_458_; 
lean_dec(v___x_413_);
v___x_417_ = lean_unsigned_to_nat(1u);
v___x_458_ = l_List_elem___at___00__private_Lean_Server_Completion_CompletionCollectors_0__Lean_Server_Completion_bestLabelForDecl_x3f_spec__0(v_a_404_, v_fst_391_);
if (v___x_458_ == 0)
{
v___y_419_ = v___x_390_;
goto v___jp_418_;
}
else
{
uint8_t v___x_459_; 
lean_inc(v___x_415_);
v___x_459_ = l_List_all___at___00main_spec__4(v___x_415_);
v___y_419_ = v___x_459_;
goto v___jp_418_;
}
v___jp_418_:
{
if (v___y_419_ == 0)
{
lean_object* v___x_420_; lean_object* v___x_421_; lean_object* v___x_422_; lean_object* v___x_423_; lean_object* v___x_424_; lean_object* v___x_425_; lean_object* v___x_426_; lean_object* v___x_427_; 
v___x_420_ = ((lean_object*)(l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__0));
lean_inc(v_a_404_);
v___x_421_ = l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(v_a_404_, v___x_402_);
v___x_422_ = lean_string_append(v___x_420_, v___x_421_);
lean_dec_ref(v___x_421_);
v___x_423_ = ((lean_object*)(l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__1));
v___x_424_ = lean_string_append(v___x_422_, v___x_423_);
v___x_425_ = l_List_toString___at___00Lean_resolveUniqueNamespace___at___00Lean_Elab_OpenDecl_elabOpenDecl___at___00Lean_Elab_Command_elabOpen_spec__0_spec__4_spec__5(v___x_415_);
v___x_426_ = lean_string_append(v___x_424_, v___x_425_);
lean_dec_ref(v___x_425_);
v___x_427_ = l_IO_println___at___00__private_Lean_Shell_0__Lean_ShellOptions_process_spec__3(v___x_426_);
if (lean_obj_tag(v___x_427_) == 0)
{
lean_object* v___x_428_; lean_object* v___x_430_; 
lean_dec_ref_known(v___x_427_, 1);
v___x_428_ = lean_nat_add(v_fst_407_, v___x_417_);
lean_dec(v_fst_407_);
if (v_isShared_412_ == 0)
{
lean_ctor_set(v___x_411_, 1, v_snd_408_);
lean_ctor_set(v___x_411_, 0, v___x_428_);
v___x_430_ = v___x_411_;
goto v_reusejp_429_;
}
else
{
lean_object* v_reuseFailAlloc_431_; 
v_reuseFailAlloc_431_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_431_, 0, v___x_428_);
lean_ctor_set(v_reuseFailAlloc_431_, 1, v_snd_408_);
v___x_430_ = v_reuseFailAlloc_431_;
goto v_reusejp_429_;
}
v_reusejp_429_:
{
v_a_398_ = v___x_430_;
goto v___jp_397_;
}
}
else
{
lean_object* v_a_432_; lean_object* v___x_434_; uint8_t v_isShared_435_; uint8_t v_isSharedCheck_439_; 
lean_del_object(v___x_411_);
lean_dec(v_snd_408_);
lean_dec(v_fst_407_);
lean_dec_ref(v_a_389_);
v_a_432_ = lean_ctor_get(v___x_427_, 0);
v_isSharedCheck_439_ = !lean_is_exclusive(v___x_427_);
if (v_isSharedCheck_439_ == 0)
{
v___x_434_ = v___x_427_;
v_isShared_435_ = v_isSharedCheck_439_;
goto v_resetjp_433_;
}
else
{
lean_inc(v_a_432_);
lean_dec(v___x_427_);
v___x_434_ = lean_box(0);
v_isShared_435_ = v_isSharedCheck_439_;
goto v_resetjp_433_;
}
v_resetjp_433_:
{
lean_object* v___x_437_; 
if (v_isShared_435_ == 0)
{
v___x_437_ = v___x_434_;
goto v_reusejp_436_;
}
else
{
lean_object* v_reuseFailAlloc_438_; 
v_reuseFailAlloc_438_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_438_, 0, v_a_432_);
v___x_437_ = v_reuseFailAlloc_438_;
goto v_reusejp_436_;
}
v_reusejp_436_:
{
return v___x_437_;
}
}
}
}
else
{
lean_object* v___x_440_; lean_object* v___x_441_; lean_object* v___x_442_; lean_object* v___x_443_; lean_object* v___x_444_; lean_object* v___x_445_; 
lean_dec(v___x_415_);
v___x_440_ = ((lean_object*)(l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__2));
lean_inc(v_a_404_);
v___x_441_ = l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(v_a_404_, v___y_419_);
v___x_442_ = lean_string_append(v___x_440_, v___x_441_);
lean_dec_ref(v___x_441_);
v___x_443_ = ((lean_object*)(l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__3));
v___x_444_ = lean_string_append(v___x_442_, v___x_443_);
v___x_445_ = l_IO_println___at___00__private_Lean_Shell_0__Lean_ShellOptions_process_spec__3(v___x_444_);
if (lean_obj_tag(v___x_445_) == 0)
{
lean_object* v___x_446_; lean_object* v___x_448_; 
lean_dec_ref_known(v___x_445_, 1);
v___x_446_ = lean_nat_add(v_snd_408_, v___x_417_);
lean_dec(v_snd_408_);
if (v_isShared_412_ == 0)
{
lean_ctor_set(v___x_411_, 1, v___x_446_);
lean_ctor_set(v___x_411_, 0, v_fst_407_);
v___x_448_ = v___x_411_;
goto v_reusejp_447_;
}
else
{
lean_object* v_reuseFailAlloc_449_; 
v_reuseFailAlloc_449_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_449_, 0, v_fst_407_);
lean_ctor_set(v_reuseFailAlloc_449_, 1, v___x_446_);
v___x_448_ = v_reuseFailAlloc_449_;
goto v_reusejp_447_;
}
v_reusejp_447_:
{
v_a_398_ = v___x_448_;
goto v___jp_397_;
}
}
else
{
lean_object* v_a_450_; lean_object* v___x_452_; uint8_t v_isShared_453_; uint8_t v_isSharedCheck_457_; 
lean_del_object(v___x_411_);
lean_dec(v_snd_408_);
lean_dec(v_fst_407_);
lean_dec_ref(v_a_389_);
v_a_450_ = lean_ctor_get(v___x_445_, 0);
v_isSharedCheck_457_ = !lean_is_exclusive(v___x_445_);
if (v_isSharedCheck_457_ == 0)
{
v___x_452_ = v___x_445_;
v_isShared_453_ = v_isSharedCheck_457_;
goto v_resetjp_451_;
}
else
{
lean_inc(v_a_450_);
lean_dec(v___x_445_);
v___x_452_ = lean_box(0);
v_isShared_453_ = v_isSharedCheck_457_;
goto v_resetjp_451_;
}
v_resetjp_451_:
{
lean_object* v___x_455_; 
if (v_isShared_453_ == 0)
{
v___x_455_ = v___x_452_;
goto v_reusejp_454_;
}
else
{
lean_object* v_reuseFailAlloc_456_; 
v_reuseFailAlloc_456_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_456_, 0, v_a_450_);
v___x_455_ = v_reuseFailAlloc_456_;
goto v_reusejp_454_;
}
v_reusejp_454_:
{
return v___x_455_;
}
}
}
}
}
}
else
{
lean_object* v___x_460_; lean_object* v___x_461_; lean_object* v___x_462_; lean_object* v___x_463_; lean_object* v___x_464_; lean_object* v___x_465_; lean_object* v___x_466_; lean_object* v___x_467_; lean_object* v___x_468_; lean_object* v___x_469_; lean_object* v___x_470_; lean_object* v___x_471_; 
lean_dec(v___x_415_);
v___x_460_ = ((lean_object*)(l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__4));
lean_inc(v_a_404_);
v___x_461_ = l_Lean_Name_toStringWithToken___at___00Lean_Name_toString_spec__0(v_a_404_, v___x_416_);
v___x_462_ = lean_string_append(v___x_460_, v___x_461_);
lean_dec_ref(v___x_461_);
v___x_463_ = ((lean_object*)(l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__5));
v___x_464_ = lean_string_append(v___x_462_, v___x_463_);
v___x_465_ = ((lean_object*)(l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__6));
v___x_466_ = l_List_mapTR_loop___at___00main_spec__5(v___x_416_, v___x_413_, v___x_414_);
v___x_467_ = l_String_intercalate(v___x_465_, v___x_466_);
v___x_468_ = lean_string_append(v___x_464_, v___x_467_);
lean_dec_ref(v___x_467_);
v___x_469_ = ((lean_object*)(l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___closed__7));
v___x_470_ = lean_string_append(v___x_468_, v___x_469_);
v___x_471_ = l_IO_println___at___00__private_Lean_Shell_0__Lean_ShellOptions_process_spec__3(v___x_470_);
if (lean_obj_tag(v___x_471_) == 0)
{
lean_object* v___x_473_; 
lean_dec_ref_known(v___x_471_, 1);
if (v_isShared_412_ == 0)
{
lean_ctor_set(v___x_411_, 1, v_snd_408_);
lean_ctor_set(v___x_411_, 0, v_fst_407_);
v___x_473_ = v___x_411_;
goto v_reusejp_472_;
}
else
{
lean_object* v_reuseFailAlloc_474_; 
v_reuseFailAlloc_474_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v_reuseFailAlloc_474_, 0, v_fst_407_);
lean_ctor_set(v_reuseFailAlloc_474_, 1, v_snd_408_);
v___x_473_ = v_reuseFailAlloc_474_;
goto v_reusejp_472_;
}
v_reusejp_472_:
{
v_a_398_ = v___x_473_;
goto v___jp_397_;
}
}
else
{
lean_object* v_a_475_; lean_object* v___x_477_; uint8_t v_isShared_478_; uint8_t v_isSharedCheck_482_; 
lean_del_object(v___x_411_);
lean_dec(v_snd_408_);
lean_dec(v_fst_407_);
lean_dec_ref(v_a_389_);
v_a_475_ = lean_ctor_get(v___x_471_, 0);
v_isSharedCheck_482_ = !lean_is_exclusive(v___x_471_);
if (v_isSharedCheck_482_ == 0)
{
v___x_477_ = v___x_471_;
v_isShared_478_ = v_isSharedCheck_482_;
goto v_resetjp_476_;
}
else
{
lean_inc(v_a_475_);
lean_dec(v___x_471_);
v___x_477_ = lean_box(0);
v_isShared_478_ = v_isSharedCheck_482_;
goto v_resetjp_476_;
}
v_resetjp_476_:
{
lean_object* v___x_480_; 
if (v_isShared_478_ == 0)
{
v___x_480_ = v___x_477_;
goto v_reusejp_479_;
}
else
{
lean_object* v_reuseFailAlloc_481_; 
v_reuseFailAlloc_481_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_481_, 0, v_a_475_);
v___x_480_ = v_reuseFailAlloc_481_;
goto v_reusejp_479_;
}
v_reusejp_479_:
{
return v___x_480_;
}
}
}
}
}
}
else
{
lean_object* v_a_485_; lean_object* v___x_487_; uint8_t v_isShared_488_; uint8_t v_isSharedCheck_492_; 
lean_dec_ref(v_b_395_);
lean_dec_ref(v_a_389_);
v_a_485_ = lean_ctor_get(v___x_405_, 0);
v_isSharedCheck_492_ = !lean_is_exclusive(v___x_405_);
if (v_isSharedCheck_492_ == 0)
{
v___x_487_ = v___x_405_;
v_isShared_488_ = v_isSharedCheck_492_;
goto v_resetjp_486_;
}
else
{
lean_inc(v_a_485_);
lean_dec(v___x_405_);
v___x_487_ = lean_box(0);
v_isShared_488_ = v_isSharedCheck_492_;
goto v_resetjp_486_;
}
v_resetjp_486_:
{
lean_object* v___x_490_; 
if (v_isShared_488_ == 0)
{
v___x_490_ = v___x_487_;
goto v_reusejp_489_;
}
else
{
lean_object* v_reuseFailAlloc_491_; 
v_reuseFailAlloc_491_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_491_, 0, v_a_485_);
v___x_490_ = v_reuseFailAlloc_491_;
goto v_reusejp_489_;
}
v_reusejp_489_:
{
return v___x_490_;
}
}
}
}
v___jp_397_:
{
size_t v___x_399_; size_t v___x_400_; 
v___x_399_ = ((size_t)1ULL);
v___x_400_ = lean_usize_add(v_i_394_, v___x_399_);
v_i_394_ = v___x_400_;
v_b_395_ = v_a_398_;
goto _start;
}
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6___boxed(lean_object* v_a_493_, lean_object* v___x_494_, lean_object* v_fst_495_, lean_object* v_as_496_, lean_object* v_sz_497_, lean_object* v_i_498_, lean_object* v_b_499_, lean_object* v___y_500_){
_start:
{
uint8_t v___x_5286__boxed_501_; size_t v_sz_boxed_502_; size_t v_i_boxed_503_; lean_object* v_res_504_; 
v___x_5286__boxed_501_ = lean_unbox(v___x_494_);
v_sz_boxed_502_ = lean_unbox_usize(v_sz_497_);
lean_dec(v_sz_497_);
v_i_boxed_503_ = lean_unbox_usize(v_i_498_);
lean_dec(v_i_498_);
v_res_504_ = l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6(v_a_493_, v___x_5286__boxed_501_, v_fst_495_, v_as_496_, v_sz_boxed_502_, v_i_boxed_503_, v_b_499_);
lean_dec_ref(v_as_496_);
lean_dec(v_fst_495_);
return v_res_504_;
}
}
static lean_object* _init_l_main___boxed__const__1(void){
_start:
{
uint32_t v___x_517_; lean_object* v___x_518_; 
v___x_517_ = 1;
v___x_518_ = lean_box_uint32(v___x_517_);
return v___x_518_;
}
}
static lean_object* _init_l_main___boxed__const__2(void){
_start:
{
uint32_t v___x_519_; lean_object* v___x_520_; 
v___x_519_ = 2;
v___x_520_ = lean_box_uint32(v___x_519_);
return v___x_520_;
}
}
LEAN_EXPORT lean_object* _lean_main(lean_object* v_args_521_){
_start:
{
lean_object* v___y_524_; lean_object* v___y_525_; uint32_t v___y_526_; lean_object* v___y_527_; uint8_t v___y_528_; lean_object* v___y_529_; lean_object* v___y_530_; lean_object* v___y_608_; lean_object* v___y_609_; lean_object* v___y_610_; lean_object* v___y_611_; uint8_t v___y_612_; lean_object* v___y_613_; uint32_t v___y_614_; lean_object* v___y_615_; lean_object* v___y_616_; lean_object* v___y_617_; lean_object* v___y_620_; lean_object* v___y_621_; lean_object* v___y_622_; lean_object* v___y_623_; uint32_t v___y_624_; lean_object* v___y_625_; uint8_t v___y_626_; lean_object* v___y_627_; lean_object* v___y_628_; lean_object* v___y_629_; lean_object* v_fst_632_; lean_object* v_snd_633_; lean_object* v___y_704_; lean_object* v___y_707_; lean_object* v_head_708_; lean_object* v_tail_709_; lean_object* v___y_725_; 
if (lean_obj_tag(v_args_521_) == 1)
{
lean_object* v_head_728_; lean_object* v_tail_729_; lean_object* v___x_730_; uint8_t v___x_731_; 
v_head_728_ = lean_ctor_get(v_args_521_, 0);
lean_inc(v_head_728_);
v_tail_729_ = lean_ctor_get(v_args_521_, 1);
lean_inc(v_tail_729_);
v___x_730_ = ((lean_object*)(l_main___closed__10));
v___x_731_ = lean_string_dec_eq(v_head_728_, v___x_730_);
if (v___x_731_ == 0)
{
v___y_707_ = v_args_521_;
v_head_708_ = v_head_728_;
v_tail_709_ = v_tail_729_;
goto v___jp_706_;
}
else
{
lean_dec_ref_known(v_args_521_, 2);
lean_dec(v_head_728_);
v___y_725_ = v_tail_729_;
goto v___jp_724_;
}
}
else
{
v___y_725_ = v_args_521_;
goto v___jp_724_;
}
v___jp_523_:
{
lean_object* v___x_531_; size_t v_sz_532_; size_t v___x_533_; lean_object* v___x_534_; 
lean_inc_n(v___y_525_, 2);
v___x_531_ = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(v___x_531_, 0, v___y_525_);
lean_ctor_set(v___x_531_, 1, v___y_525_);
v_sz_532_ = lean_array_size(v___y_530_);
v___x_533_ = ((size_t)0ULL);
v___x_534_ = l___private_Init_Data_Array_Basic_0__Array_forIn_x27Unsafe_loop___at___00main_spec__6(v___y_524_, v___y_528_, v___y_527_, v___y_530_, v_sz_532_, v___x_533_, v___x_531_);
lean_dec(v___y_527_);
if (lean_obj_tag(v___x_534_) == 0)
{
lean_object* v_a_535_; lean_object* v_fst_536_; lean_object* v_snd_537_; lean_object* v___x_538_; lean_object* v___x_539_; lean_object* v___x_540_; lean_object* v___x_541_; lean_object* v___x_542_; lean_object* v___x_543_; lean_object* v___x_544_; lean_object* v___x_545_; lean_object* v___x_546_; lean_object* v___x_547_; lean_object* v___x_548_; lean_object* v___x_549_; lean_object* v___x_550_; lean_object* v___x_551_; lean_object* v___x_552_; lean_object* v___x_553_; lean_object* v___x_554_; lean_object* v___x_555_; lean_object* v___x_556_; 
v_a_535_ = lean_ctor_get(v___x_534_, 0);
lean_inc(v_a_535_);
lean_dec_ref_known(v___x_534_, 1);
v_fst_536_ = lean_ctor_get(v_a_535_, 0);
lean_inc_n(v_fst_536_, 2);
v_snd_537_ = lean_ctor_get(v_a_535_, 1);
lean_inc(v_snd_537_);
lean_dec(v_a_535_);
v___x_538_ = ((lean_object*)(l_main___closed__0));
v___x_539_ = lean_array_get_size(v___y_530_);
lean_dec_ref(v___y_530_);
v___x_540_ = l_Nat_reprFast(v___x_539_);
v___x_541_ = lean_string_append(v___x_538_, v___x_540_);
lean_dec_ref(v___x_540_);
v___x_542_ = ((lean_object*)(l_main___closed__1));
v___x_543_ = lean_string_append(v___x_541_, v___x_542_);
v___x_544_ = l_List_toString___at___00Lean_resolveUniqueNamespace___at___00Lean_Elab_OpenDecl_elabOpenDecl___at___00Lean_Elab_Command_elabOpen_spec__0_spec__4_spec__5(v___y_529_);
v___x_545_ = lean_string_append(v___x_543_, v___x_544_);
lean_dec_ref(v___x_544_);
v___x_546_ = ((lean_object*)(l_main___closed__2));
v___x_547_ = lean_string_append(v___x_545_, v___x_546_);
v___x_548_ = l_Nat_reprFast(v_fst_536_);
v___x_549_ = lean_string_append(v___x_547_, v___x_548_);
lean_dec_ref(v___x_548_);
v___x_550_ = ((lean_object*)(l_main___closed__3));
v___x_551_ = lean_string_append(v___x_549_, v___x_550_);
v___x_552_ = l_Nat_reprFast(v_snd_537_);
v___x_553_ = lean_string_append(v___x_551_, v___x_552_);
lean_dec_ref(v___x_552_);
v___x_554_ = ((lean_object*)(l_main___closed__4));
v___x_555_ = lean_string_append(v___x_553_, v___x_554_);
v___x_556_ = l_IO_println___at___00__private_Lean_Shell_0__Lean_ShellOptions_process_spec__3(v___x_555_);
if (lean_obj_tag(v___x_556_) == 0)
{
lean_object* v___x_558_; uint8_t v_isShared_559_; uint8_t v_isSharedCheck_589_; 
v_isSharedCheck_589_ = !lean_is_exclusive(v___x_556_);
if (v_isSharedCheck_589_ == 0)
{
lean_object* v_unused_590_; 
v_unused_590_ = lean_ctor_get(v___x_556_, 0);
lean_dec(v_unused_590_);
v___x_558_ = v___x_556_;
v_isShared_559_ = v_isSharedCheck_589_;
goto v_resetjp_557_;
}
else
{
lean_dec(v___x_556_);
v___x_558_ = lean_box(0);
v_isShared_559_ = v_isSharedCheck_589_;
goto v_resetjp_557_;
}
v_resetjp_557_:
{
uint8_t v___x_560_; 
v___x_560_ = lean_nat_dec_eq(v___x_539_, v___y_525_);
if (v___x_560_ == 0)
{
uint8_t v___x_561_; 
v___x_561_ = lean_nat_dec_eq(v_fst_536_, v___y_525_);
lean_dec(v___y_525_);
lean_dec(v_fst_536_);
if (v___x_561_ == 0)
{
lean_object* v___x_562_; lean_object* v___x_564_; 
v___x_562_ = l_main___boxed__const__1;
if (v_isShared_559_ == 0)
{
lean_ctor_set(v___x_558_, 0, v___x_562_);
v___x_564_ = v___x_558_;
goto v_reusejp_563_;
}
else
{
lean_object* v_reuseFailAlloc_565_; 
v_reuseFailAlloc_565_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_565_, 0, v___x_562_);
v___x_564_ = v_reuseFailAlloc_565_;
goto v_reusejp_563_;
}
v_reusejp_563_:
{
return v___x_564_;
}
}
else
{
lean_object* v___x_566_; lean_object* v___x_568_; 
v___x_566_ = lean_box_uint32(v___y_526_);
if (v_isShared_559_ == 0)
{
lean_ctor_set(v___x_558_, 0, v___x_566_);
v___x_568_ = v___x_558_;
goto v_reusejp_567_;
}
else
{
lean_object* v_reuseFailAlloc_569_; 
v_reuseFailAlloc_569_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_569_, 0, v___x_566_);
v___x_568_ = v_reuseFailAlloc_569_;
goto v_reusejp_567_;
}
v_reusejp_567_:
{
return v___x_568_;
}
}
}
else
{
lean_object* v___x_570_; lean_object* v___x_571_; 
lean_del_object(v___x_558_);
lean_dec(v_fst_536_);
lean_dec(v___y_525_);
v___x_570_ = ((lean_object*)(l_main___closed__5));
v___x_571_ = l_IO_eprintln___at___00__private_Lean_Shell_0__Lean_shellMain_spec__1(v___x_570_);
if (lean_obj_tag(v___x_571_) == 0)
{
lean_object* v___x_573_; uint8_t v_isShared_574_; uint8_t v_isSharedCheck_579_; 
v_isSharedCheck_579_ = !lean_is_exclusive(v___x_571_);
if (v_isSharedCheck_579_ == 0)
{
lean_object* v_unused_580_; 
v_unused_580_ = lean_ctor_get(v___x_571_, 0);
lean_dec(v_unused_580_);
v___x_573_ = v___x_571_;
v_isShared_574_ = v_isSharedCheck_579_;
goto v_resetjp_572_;
}
else
{
lean_dec(v___x_571_);
v___x_573_ = lean_box(0);
v_isShared_574_ = v_isSharedCheck_579_;
goto v_resetjp_572_;
}
v_resetjp_572_:
{
lean_object* v___x_575_; lean_object* v___x_577_; 
v___x_575_ = l_main___boxed__const__1;
if (v_isShared_574_ == 0)
{
lean_ctor_set(v___x_573_, 0, v___x_575_);
v___x_577_ = v___x_573_;
goto v_reusejp_576_;
}
else
{
lean_object* v_reuseFailAlloc_578_; 
v_reuseFailAlloc_578_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_578_, 0, v___x_575_);
v___x_577_ = v_reuseFailAlloc_578_;
goto v_reusejp_576_;
}
v_reusejp_576_:
{
return v___x_577_;
}
}
}
else
{
lean_object* v_a_581_; lean_object* v___x_583_; uint8_t v_isShared_584_; uint8_t v_isSharedCheck_588_; 
v_a_581_ = lean_ctor_get(v___x_571_, 0);
v_isSharedCheck_588_ = !lean_is_exclusive(v___x_571_);
if (v_isSharedCheck_588_ == 0)
{
v___x_583_ = v___x_571_;
v_isShared_584_ = v_isSharedCheck_588_;
goto v_resetjp_582_;
}
else
{
lean_inc(v_a_581_);
lean_dec(v___x_571_);
v___x_583_ = lean_box(0);
v_isShared_584_ = v_isSharedCheck_588_;
goto v_resetjp_582_;
}
v_resetjp_582_:
{
lean_object* v___x_586_; 
if (v_isShared_584_ == 0)
{
v___x_586_ = v___x_583_;
goto v_reusejp_585_;
}
else
{
lean_object* v_reuseFailAlloc_587_; 
v_reuseFailAlloc_587_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_587_, 0, v_a_581_);
v___x_586_ = v_reuseFailAlloc_587_;
goto v_reusejp_585_;
}
v_reusejp_585_:
{
return v___x_586_;
}
}
}
}
}
}
else
{
lean_object* v_a_591_; lean_object* v___x_593_; uint8_t v_isShared_594_; uint8_t v_isSharedCheck_598_; 
lean_dec(v_fst_536_);
lean_dec(v___y_525_);
v_a_591_ = lean_ctor_get(v___x_556_, 0);
v_isSharedCheck_598_ = !lean_is_exclusive(v___x_556_);
if (v_isSharedCheck_598_ == 0)
{
v___x_593_ = v___x_556_;
v_isShared_594_ = v_isSharedCheck_598_;
goto v_resetjp_592_;
}
else
{
lean_inc(v_a_591_);
lean_dec(v___x_556_);
v___x_593_ = lean_box(0);
v_isShared_594_ = v_isSharedCheck_598_;
goto v_resetjp_592_;
}
v_resetjp_592_:
{
lean_object* v___x_596_; 
if (v_isShared_594_ == 0)
{
v___x_596_ = v___x_593_;
goto v_reusejp_595_;
}
else
{
lean_object* v_reuseFailAlloc_597_; 
v_reuseFailAlloc_597_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_597_, 0, v_a_591_);
v___x_596_ = v_reuseFailAlloc_597_;
goto v_reusejp_595_;
}
v_reusejp_595_:
{
return v___x_596_;
}
}
}
}
else
{
lean_object* v_a_599_; lean_object* v___x_601_; uint8_t v_isShared_602_; uint8_t v_isSharedCheck_606_; 
lean_dec_ref(v___y_530_);
lean_dec(v___y_529_);
lean_dec(v___y_525_);
v_a_599_ = lean_ctor_get(v___x_534_, 0);
v_isSharedCheck_606_ = !lean_is_exclusive(v___x_534_);
if (v_isSharedCheck_606_ == 0)
{
v___x_601_ = v___x_534_;
v_isShared_602_ = v_isSharedCheck_606_;
goto v_resetjp_600_;
}
else
{
lean_inc(v_a_599_);
lean_dec(v___x_534_);
v___x_601_ = lean_box(0);
v_isShared_602_ = v_isSharedCheck_606_;
goto v_resetjp_600_;
}
v_resetjp_600_:
{
lean_object* v___x_604_; 
if (v_isShared_602_ == 0)
{
v___x_604_ = v___x_601_;
goto v_reusejp_603_;
}
else
{
lean_object* v_reuseFailAlloc_605_; 
v_reuseFailAlloc_605_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_605_, 0, v_a_599_);
v___x_604_ = v_reuseFailAlloc_605_;
goto v_reusejp_603_;
}
v_reusejp_603_:
{
return v___x_604_;
}
}
}
}
v___jp_607_:
{
lean_object* v___x_618_; 
v___x_618_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg(v___y_615_, v___y_608_, v___y_611_, v___y_617_);
lean_dec(v___y_617_);
lean_dec(v___y_615_);
v___y_524_ = v___y_609_;
v___y_525_ = v___y_610_;
v___y_526_ = v___y_614_;
v___y_527_ = v___y_613_;
v___y_528_ = v___y_612_;
v___y_529_ = v___y_616_;
v___y_530_ = v___x_618_;
goto v___jp_523_;
}
v___jp_619_:
{
uint8_t v___x_630_; 
v___x_630_ = lean_nat_dec_le(v___y_629_, v___y_628_);
if (v___x_630_ == 0)
{
lean_dec(v___y_628_);
lean_inc(v___y_629_);
v___y_608_ = v___y_620_;
v___y_609_ = v___y_621_;
v___y_610_ = v___y_622_;
v___y_611_ = v___y_629_;
v___y_612_ = v___y_626_;
v___y_613_ = v___y_625_;
v___y_614_ = v___y_624_;
v___y_615_ = v___y_623_;
v___y_616_ = v___y_627_;
v___y_617_ = v___y_629_;
goto v___jp_607_;
}
else
{
v___y_608_ = v___y_620_;
v___y_609_ = v___y_621_;
v___y_610_ = v___y_622_;
v___y_611_ = v___y_629_;
v___y_612_ = v___y_626_;
v___y_613_ = v___y_625_;
v___y_614_ = v___y_624_;
v___y_615_ = v___y_623_;
v___y_616_ = v___y_627_;
v___y_617_ = v___y_628_;
goto v___jp_607_;
}
}
v___jp_631_:
{
uint8_t v___x_634_; 
v___x_634_ = l_List_isEmpty___redArg(v_snd_633_);
if (v___x_634_ == 0)
{
lean_object* v___x_635_; lean_object* v___x_636_; 
v___x_635_ = ((lean_object*)(l_main___closed__6));
v___x_636_ = l_Lean_findSysroot(v___x_635_);
if (lean_obj_tag(v___x_636_) == 0)
{
lean_object* v_a_637_; lean_object* v___x_638_; lean_object* v___x_639_; 
v_a_637_ = lean_ctor_get(v___x_636_, 0);
lean_inc(v_a_637_);
lean_dec_ref_known(v___x_636_, 1);
v___x_638_ = lean_box(0);
v___x_639_ = l_Lean_initSearchPath(v_a_637_, v___x_638_);
if (lean_obj_tag(v___x_639_) == 0)
{
lean_object* v_roots_640_; lean_object* v___x_641_; lean_object* v___x_642_; lean_object* v___x_643_; uint32_t v___x_644_; lean_object* v___x_645_; lean_object* v___x_646_; uint8_t v___x_647_; lean_object* v___x_648_; lean_object* v___x_649_; 
lean_dec_ref_known(v___x_639_, 1);
v_roots_640_ = l_List_mapTR_loop___at___00main_spec__0(v_snd_633_, v___x_638_);
lean_inc(v_roots_640_);
v___x_641_ = l_List_mapTR_loop___at___00main_spec__1(v___x_634_, v_roots_640_, v___x_638_);
v___x_642_ = lean_array_mk(v___x_641_);
v___x_643_ = l_Lean_Options_empty;
v___x_644_ = 0;
v___x_645_ = lean_unsigned_to_nat(0u);
v___x_646_ = ((lean_object*)(l_main___closed__7));
v___x_647_ = 2;
v___x_648_ = lean_box(1);
v___x_649_ = l_Lean_importModules(v___x_642_, v___x_643_, v___x_644_, v___x_646_, v___x_634_, v___x_634_, v___x_647_, v___x_648_);
if (lean_obj_tag(v___x_649_) == 0)
{
lean_object* v_a_650_; lean_object* v___x_651_; lean_object* v___f_652_; lean_object* v___x_653_; lean_object* v___x_654_; lean_object* v___x_655_; uint8_t v___x_656_; 
v_a_650_ = lean_ctor_get(v___x_649_, 0);
lean_inc_n(v_a_650_, 3);
lean_dec_ref_known(v___x_649_, 1);
v___x_651_ = lean_box(v___x_634_);
lean_inc(v_roots_640_);
v___f_652_ = lean_alloc_closure((void*)(l_main___lam__0___boxed), 6, 3);
lean_closure_set(v___f_652_, 0, v_a_650_);
lean_closure_set(v___f_652_, 1, v_roots_640_);
lean_closure_set(v___f_652_, 2, v___x_651_);
v___x_653_ = l_Lean_Environment_constants(v_a_650_);
v___x_654_ = l_Lean_SMap_fold___at___00Lean_SMap_toList___at___00__private_Lean_Elab_DocString_Builtin_0__Lean_Doc_getQualified_spec__1_spec__2___redArg(v___f_652_, v___x_646_, v___x_653_);
lean_dec_ref(v___x_653_);
v___x_655_ = lean_array_get_size(v___x_654_);
v___x_656_ = lean_nat_dec_eq(v___x_655_, v___x_645_);
if (v___x_656_ == 0)
{
lean_object* v___x_657_; lean_object* v___x_658_; uint8_t v___x_659_; 
v___x_657_ = lean_unsigned_to_nat(1u);
v___x_658_ = lean_nat_sub(v___x_655_, v___x_657_);
v___x_659_ = lean_nat_dec_le(v___x_645_, v___x_658_);
if (v___x_659_ == 0)
{
lean_inc(v___x_658_);
v___y_620_ = v___x_654_;
v___y_621_ = v_a_650_;
v___y_622_ = v___x_645_;
v___y_623_ = v___x_655_;
v___y_624_ = v___x_644_;
v___y_625_ = v_fst_632_;
v___y_626_ = v___x_634_;
v___y_627_ = v_roots_640_;
v___y_628_ = v___x_658_;
v___y_629_ = v___x_658_;
goto v___jp_619_;
}
else
{
v___y_620_ = v___x_654_;
v___y_621_ = v_a_650_;
v___y_622_ = v___x_645_;
v___y_623_ = v___x_655_;
v___y_624_ = v___x_644_;
v___y_625_ = v_fst_632_;
v___y_626_ = v___x_634_;
v___y_627_ = v_roots_640_;
v___y_628_ = v___x_658_;
v___y_629_ = v___x_645_;
goto v___jp_619_;
}
}
else
{
v___y_524_ = v_a_650_;
v___y_525_ = v___x_645_;
v___y_526_ = v___x_644_;
v___y_527_ = v_fst_632_;
v___y_528_ = v___x_634_;
v___y_529_ = v_roots_640_;
v___y_530_ = v___x_654_;
goto v___jp_523_;
}
}
else
{
lean_object* v_a_660_; lean_object* v___x_662_; uint8_t v_isShared_663_; uint8_t v_isSharedCheck_667_; 
lean_dec(v_roots_640_);
lean_dec(v_fst_632_);
v_a_660_ = lean_ctor_get(v___x_649_, 0);
v_isSharedCheck_667_ = !lean_is_exclusive(v___x_649_);
if (v_isSharedCheck_667_ == 0)
{
v___x_662_ = v___x_649_;
v_isShared_663_ = v_isSharedCheck_667_;
goto v_resetjp_661_;
}
else
{
lean_inc(v_a_660_);
lean_dec(v___x_649_);
v___x_662_ = lean_box(0);
v_isShared_663_ = v_isSharedCheck_667_;
goto v_resetjp_661_;
}
v_resetjp_661_:
{
lean_object* v___x_665_; 
if (v_isShared_663_ == 0)
{
v___x_665_ = v___x_662_;
goto v_reusejp_664_;
}
else
{
lean_object* v_reuseFailAlloc_666_; 
v_reuseFailAlloc_666_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_666_, 0, v_a_660_);
v___x_665_ = v_reuseFailAlloc_666_;
goto v_reusejp_664_;
}
v_reusejp_664_:
{
return v___x_665_;
}
}
}
}
else
{
lean_object* v_a_668_; lean_object* v___x_670_; uint8_t v_isShared_671_; uint8_t v_isSharedCheck_675_; 
lean_dec(v_snd_633_);
lean_dec(v_fst_632_);
v_a_668_ = lean_ctor_get(v___x_639_, 0);
v_isSharedCheck_675_ = !lean_is_exclusive(v___x_639_);
if (v_isSharedCheck_675_ == 0)
{
v___x_670_ = v___x_639_;
v_isShared_671_ = v_isSharedCheck_675_;
goto v_resetjp_669_;
}
else
{
lean_inc(v_a_668_);
lean_dec(v___x_639_);
v___x_670_ = lean_box(0);
v_isShared_671_ = v_isSharedCheck_675_;
goto v_resetjp_669_;
}
v_resetjp_669_:
{
lean_object* v___x_673_; 
if (v_isShared_671_ == 0)
{
v___x_673_ = v___x_670_;
goto v_reusejp_672_;
}
else
{
lean_object* v_reuseFailAlloc_674_; 
v_reuseFailAlloc_674_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_674_, 0, v_a_668_);
v___x_673_ = v_reuseFailAlloc_674_;
goto v_reusejp_672_;
}
v_reusejp_672_:
{
return v___x_673_;
}
}
}
}
else
{
lean_object* v_a_676_; lean_object* v___x_678_; uint8_t v_isShared_679_; uint8_t v_isSharedCheck_683_; 
lean_dec(v_snd_633_);
lean_dec(v_fst_632_);
v_a_676_ = lean_ctor_get(v___x_636_, 0);
v_isSharedCheck_683_ = !lean_is_exclusive(v___x_636_);
if (v_isSharedCheck_683_ == 0)
{
v___x_678_ = v___x_636_;
v_isShared_679_ = v_isSharedCheck_683_;
goto v_resetjp_677_;
}
else
{
lean_inc(v_a_676_);
lean_dec(v___x_636_);
v___x_678_ = lean_box(0);
v_isShared_679_ = v_isSharedCheck_683_;
goto v_resetjp_677_;
}
v_resetjp_677_:
{
lean_object* v___x_681_; 
if (v_isShared_679_ == 0)
{
v___x_681_ = v___x_678_;
goto v_reusejp_680_;
}
else
{
lean_object* v_reuseFailAlloc_682_; 
v_reuseFailAlloc_682_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_682_, 0, v_a_676_);
v___x_681_ = v_reuseFailAlloc_682_;
goto v_reusejp_680_;
}
v_reusejp_680_:
{
return v___x_681_;
}
}
}
}
else
{
lean_object* v___x_684_; lean_object* v___x_685_; 
lean_dec(v_snd_633_);
lean_dec(v_fst_632_);
v___x_684_ = ((lean_object*)(l_main___closed__8));
v___x_685_ = l_IO_eprintln___at___00__private_Lean_Shell_0__Lean_shellMain_spec__1(v___x_684_);
if (lean_obj_tag(v___x_685_) == 0)
{
lean_object* v___x_687_; uint8_t v_isShared_688_; uint8_t v_isSharedCheck_693_; 
v_isSharedCheck_693_ = !lean_is_exclusive(v___x_685_);
if (v_isSharedCheck_693_ == 0)
{
lean_object* v_unused_694_; 
v_unused_694_ = lean_ctor_get(v___x_685_, 0);
lean_dec(v_unused_694_);
v___x_687_ = v___x_685_;
v_isShared_688_ = v_isSharedCheck_693_;
goto v_resetjp_686_;
}
else
{
lean_dec(v___x_685_);
v___x_687_ = lean_box(0);
v_isShared_688_ = v_isSharedCheck_693_;
goto v_resetjp_686_;
}
v_resetjp_686_:
{
lean_object* v___x_689_; lean_object* v___x_691_; 
v___x_689_ = l_main___boxed__const__2;
if (v_isShared_688_ == 0)
{
lean_ctor_set(v___x_687_, 0, v___x_689_);
v___x_691_ = v___x_687_;
goto v_reusejp_690_;
}
else
{
lean_object* v_reuseFailAlloc_692_; 
v_reuseFailAlloc_692_ = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(v_reuseFailAlloc_692_, 0, v___x_689_);
v___x_691_ = v_reuseFailAlloc_692_;
goto v_reusejp_690_;
}
v_reusejp_690_:
{
return v___x_691_;
}
}
}
else
{
lean_object* v_a_695_; lean_object* v___x_697_; uint8_t v_isShared_698_; uint8_t v_isSharedCheck_702_; 
v_a_695_ = lean_ctor_get(v___x_685_, 0);
v_isSharedCheck_702_ = !lean_is_exclusive(v___x_685_);
if (v_isSharedCheck_702_ == 0)
{
v___x_697_ = v___x_685_;
v_isShared_698_ = v_isSharedCheck_702_;
goto v_resetjp_696_;
}
else
{
lean_inc(v_a_695_);
lean_dec(v___x_685_);
v___x_697_ = lean_box(0);
v_isShared_698_ = v_isSharedCheck_702_;
goto v_resetjp_696_;
}
v_resetjp_696_:
{
lean_object* v___x_700_; 
if (v_isShared_698_ == 0)
{
v___x_700_ = v___x_697_;
goto v_reusejp_699_;
}
else
{
lean_object* v_reuseFailAlloc_701_; 
v_reuseFailAlloc_701_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_701_, 0, v_a_695_);
v___x_700_ = v_reuseFailAlloc_701_;
goto v_reusejp_699_;
}
v_reusejp_699_:
{
return v___x_700_;
}
}
}
}
}
v___jp_703_:
{
lean_object* v___x_705_; 
v___x_705_ = lean_box(0);
v_fst_632_ = v___x_705_;
v_snd_633_ = v___y_704_;
goto v___jp_631_;
}
v___jp_706_:
{
lean_object* v___x_710_; uint8_t v___x_711_; 
v___x_710_ = ((lean_object*)(l_main___closed__9));
v___x_711_ = lean_string_dec_eq(v_head_708_, v___x_710_);
lean_dec_ref(v_head_708_);
if (v___x_711_ == 0)
{
lean_dec(v_tail_709_);
v___y_704_ = v___y_707_;
goto v___jp_703_;
}
else
{
if (lean_obj_tag(v_tail_709_) == 1)
{
lean_object* v_head_712_; lean_object* v_tail_713_; lean_object* v___x_714_; 
lean_dec(v___y_707_);
v_head_712_ = lean_ctor_get(v_tail_709_, 0);
lean_inc(v_head_712_);
v_tail_713_ = lean_ctor_get(v_tail_709_, 1);
lean_inc(v_tail_713_);
lean_dec_ref_known(v_tail_709_, 2);
v___x_714_ = l_readTrusted(v_head_712_);
lean_dec(v_head_712_);
if (lean_obj_tag(v___x_714_) == 0)
{
lean_object* v_a_715_; 
v_a_715_ = lean_ctor_get(v___x_714_, 0);
lean_inc(v_a_715_);
lean_dec_ref_known(v___x_714_, 1);
v_fst_632_ = v_a_715_;
v_snd_633_ = v_tail_713_;
goto v___jp_631_;
}
else
{
lean_object* v_a_716_; lean_object* v___x_718_; uint8_t v_isShared_719_; uint8_t v_isSharedCheck_723_; 
lean_dec(v_tail_713_);
v_a_716_ = lean_ctor_get(v___x_714_, 0);
v_isSharedCheck_723_ = !lean_is_exclusive(v___x_714_);
if (v_isSharedCheck_723_ == 0)
{
v___x_718_ = v___x_714_;
v_isShared_719_ = v_isSharedCheck_723_;
goto v_resetjp_717_;
}
else
{
lean_inc(v_a_716_);
lean_dec(v___x_714_);
v___x_718_ = lean_box(0);
v_isShared_719_ = v_isSharedCheck_723_;
goto v_resetjp_717_;
}
v_resetjp_717_:
{
lean_object* v___x_721_; 
if (v_isShared_719_ == 0)
{
v___x_721_ = v___x_718_;
goto v_reusejp_720_;
}
else
{
lean_object* v_reuseFailAlloc_722_; 
v_reuseFailAlloc_722_ = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(v_reuseFailAlloc_722_, 0, v_a_716_);
v___x_721_ = v_reuseFailAlloc_722_;
goto v_reusejp_720_;
}
v_reusejp_720_:
{
return v___x_721_;
}
}
}
}
else
{
lean_dec(v_tail_709_);
v___y_704_ = v___y_707_;
goto v___jp_703_;
}
}
}
v___jp_724_:
{
if (lean_obj_tag(v___y_725_) == 1)
{
lean_object* v_head_726_; lean_object* v_tail_727_; 
v_head_726_ = lean_ctor_get(v___y_725_, 0);
lean_inc(v_head_726_);
v_tail_727_ = lean_ctor_get(v___y_725_, 1);
lean_inc(v_tail_727_);
v___y_707_ = v___y_725_;
v_head_708_ = v_head_726_;
v_tail_709_ = v_tail_727_;
goto v___jp_706_;
}
else
{
v___y_704_ = v___y_725_;
goto v___jp_703_;
}
}
}
}
LEAN_EXPORT lean_object* l_main___boxed(lean_object* v_args_732_, lean_object* v_a_733_){
_start:
{
lean_object* v_res_734_; 
v_res_734_ = _lean_main(v_args_732_);
return v_res_734_;
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7(lean_object* v_n_735_, lean_object* v_as_736_, lean_object* v_lo_737_, lean_object* v_hi_738_, lean_object* v_w_739_, lean_object* v_hlo_740_, lean_object* v_hhi_741_){
_start:
{
lean_object* v___x_742_; 
v___x_742_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___redArg(v_n_735_, v_as_736_, v_lo_737_, v_hi_738_);
return v___x_742_;
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7___boxed(lean_object* v_n_743_, lean_object* v_as_744_, lean_object* v_lo_745_, lean_object* v_hi_746_, lean_object* v_w_747_, lean_object* v_hlo_748_, lean_object* v_hhi_749_){
_start:
{
lean_object* v_res_750_; 
v_res_750_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7(v_n_743_, v_as_744_, v_lo_745_, v_hi_746_, v_w_747_, v_hlo_748_, v_hhi_749_);
lean_dec(v_hi_746_);
lean_dec(v_n_743_);
return v_res_750_;
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7(lean_object* v_n_751_, lean_object* v_lo_752_, lean_object* v_hi_753_, lean_object* v_hhi_754_, lean_object* v_pivot_755_, lean_object* v_as_756_, lean_object* v_i_757_, lean_object* v_k_758_, lean_object* v_ilo_759_, lean_object* v_ik_760_, lean_object* v_w_761_){
_start:
{
lean_object* v___x_762_; 
v___x_762_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7___redArg(v_hi_753_, v_pivot_755_, v_as_756_, v_i_757_, v_k_758_);
return v___x_762_;
}
}
LEAN_EXPORT lean_object* l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7___boxed(lean_object* v_n_763_, lean_object* v_lo_764_, lean_object* v_hi_765_, lean_object* v_hhi_766_, lean_object* v_pivot_767_, lean_object* v_as_768_, lean_object* v_i_769_, lean_object* v_k_770_, lean_object* v_ilo_771_, lean_object* v_ik_772_, lean_object* v_w_773_){
_start:
{
lean_object* v_res_774_; 
v_res_774_ = l___private_Init_Data_Array_QSort_Basic_0__Array_qpartition_loop___at___00__private_Init_Data_Array_QSort_Basic_0__Array_qsort_sort___at___00main_spec__7_spec__7(v_n_763_, v_lo_764_, v_hi_765_, v_hhi_766_, v_pivot_767_, v_as_768_, v_i_769_, v_k_770_, v_ilo_771_, v_ik_772_, v_w_773_);
lean_dec(v_hi_765_);
lean_dec(v_lo_764_);
lean_dec(v_n_763_);
return v_res_774_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Lean(uint8_t builtin);
void lean_initialize();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_tools_lean_AuditAxioms(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
lean_initialize();
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Lean(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
l_instMonadEnvAuditM = _init_l_instMonadEnvAuditM();
lean_mark_persistent(l_instMonadEnvAuditM);
l_main___boxed__const__1 = _init_l_main___boxed__const__1();
lean_mark_persistent(l_main___boxed__const__1);
l_main___boxed__const__2 = _init_l_main___boxed__const__2();
lean_mark_persistent(l_main___boxed__const__2);
return lean_io_result_mk_ok(lean_box(0));
}
char ** lean_setup_args(int argc, char ** argv);
#if defined(WIN32) || defined(_WIN32)
#include <windows.h>
#endif
lean_object* run_main(int argc, char ** argv) {
    lean_object* in = lean_box(0);
    int i = argc;
    while (i > 1) {
      lean_object* n;
      i--;
      n = lean_alloc_ctor(1,2,0); lean_ctor_set(n, 0, lean_mk_string(argv[i])); lean_ctor_set(n, 1, in);
      in = n;
    }
    return _lean_main(in);
}
int main(int argc, char ** argv) {
#if defined(WIN32) || defined(_WIN32)
  SetErrorMode(SEM_FAILCRITICALERRORS);
  SetConsoleOutputCP(CP_UTF8);
#endif
  lean_object* res;
  argv = lean_setup_args(argc, argv);
  res = initialize_tools_lean_AuditAxioms(1 /* builtin */);
  lean_io_mark_end_initialization();
  if (lean_io_result_is_ok(res)) {
    lean_dec_ref(res);
    lean_init_task_manager();
    res = lean_run_main(&run_main, argc, argv);
  }
  lean_finalize_task_manager();
  if (lean_io_result_is_ok(res)) {
    int ret = lean_unbox_uint32(lean_io_result_get_value(res));
    lean_dec_ref(res);
    return ret;
  } else {
    lean_io_result_show_error(res);
    lean_dec_ref(res);
    return 1;
  }
}
#ifdef __cplusplus
}
#endif
