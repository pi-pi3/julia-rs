
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;

use libc::{c_void, c_char};
use std::mem;
use std::ptr;

pub unsafe fn jl_astaggedvalue<T>(v: *mut T) -> *mut jl_taggedvalue_t {
    (v as *const c_char).offset(-(mem::size_of::<jl_taggedvalue_t>() as isize)) as *mut jl_taggedvalue_t 
}

pub unsafe fn jl_valueof<T>(v: *mut T) -> *mut jl_value_t {
    (v as *const c_char).offset(mem::size_of::<jl_taggedvalue_t>() as isize) as *mut jl_value_t 
}

pub unsafe fn jl_typeof<T>(v: *mut T) -> *mut jl_value_t {
    ((*jl_astaggedvalue(v)).__bindgen_anon_1.header & (!(15 as usize))) as *mut jl_value_t
}

pub unsafe fn jl_typeis<T, U>(v: *mut T, t: *mut U) -> bool {
    jl_typeof(v) == (t as *mut jl_value_t)
}

pub unsafe fn jl_pgcstack() -> *mut jl_gcframe_t {
    (*jl_get_ptls_states()).pgcstack
}

pub unsafe fn jl_pgcstack_usize() -> usize {
    (*jl_get_ptls_states()).pgcstack as usize
}

pub unsafe fn JL_GC_PUSH1<T>(arg1: *const T) {
    let __gc_stkf = [3_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH2<T>(arg1: *const T, arg2: *const T) {
    let __gc_stkf = [5_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void, arg2 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH3<T>(arg1: *const T, arg2: *const T, arg3: *const T) {
    let __gc_stkf = [7_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void, arg2 as *const c_void,
                     arg3 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH4<T>(arg1: *const T, arg2: *const T, arg3: *const T, arg4: *const T) {
    let __gc_stkf = [9_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void, arg2 as *const c_void,
                     arg3 as *const c_void, arg4 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH5<T>(arg1: *const T, arg2: *const T, arg3: *const T, arg4: *const T, arg5: *const T) {
    let __gc_stkf = [11_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void, arg2 as *const c_void,
                     arg3 as *const c_void, arg4 as *const c_void,
                     arg5 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH6<T>(arg1: *const T, arg2: *const T, arg3: *const T, arg4: *const T, arg5: *const T, arg6: *const T) {
    let __gc_stkf = [13_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void, arg2 as *const c_void,
                     arg3 as *const c_void, arg4 as *const c_void,
                     arg5 as *const c_void, arg6 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_POP() -> *mut jl_gcframe_t {
    (*jl_get_ptls_states()).pgcstack = (*(*jl_get_ptls_states()).pgcstack).prev;
    (*jl_get_ptls_states()).pgcstack 
}

pub unsafe fn jl_gc_wb<T, U>(parent: *mut T, ptr: *mut U) {
    // parent and ptr isa jl_value_t*
    if (*jl_astaggedvalue(parent)).__bindgen_anon_1.bits.gc() == 3 &&
        ((*jl_astaggedvalue(ptr)).__bindgen_anon_1.bits.gc() & 1) == 0 {
        jl_gc_queue_root(parent as *mut jl_value_t);
    }
}

pub unsafe fn jl_gc_wb_back<T>(ptr: *mut T) {
    // if ptr is old
    if (*jl_astaggedvalue(ptr)).__bindgen_anon_1.bits.gc() == 3 {
        jl_gc_queue_root(ptr as *mut jl_value_t);
    }
}

macro_rules! getter_fun {
    { $name:ident :: $ft:ty  => $t:ty > $f:ident } => (
        pub unsafe fn $name<T>(x: *mut T) -> $ft {
            (*(x as *mut $t)).$f as $ft
        }
    )
}


macro_rules! setter_fun {
    { $name:ident => $t:ty > $f:ident = $ft:ty   } => (
        pub unsafe fn $name<T>(x: *mut T, v: $ft) -> $ft {
            (*(x as *mut $t)).$f = v as $ft;
            (*(x as *mut $t)).$f
        }
    )
}

getter_fun! { jl_svec_len :: usize => jl_svec_t>length }
setter_fun! { jl_svec_set_len_unsafe => jl_svec_t>length = usize }

pub unsafe fn jl_svec_data(t: *mut jl_svec_t) -> *mut *mut jl_value_t {
    (t as *mut c_char).offset(mem::size_of::<jl_svec_t>() as isize) as *mut *mut jl_value_t
}

pub unsafe fn jl_svecref<T>(t: *mut T, i: usize) -> *mut jl_value_t {
    assert!(jl_typeis(t as *mut jl_svec_t, jl_simplevector_type));
    assert!(i < jl_svec_len(t as *mut jl_svec_t));
    *(jl_svec_data(t as *mut jl_svec_t).offset(i as isize))
}

pub unsafe fn jl_svecset<T, U>(t: *mut T, i: usize, x: *mut U) -> *mut jl_value_t {
    assert!(jl_typeis(t as *mut jl_svec_t, jl_simplevector_type));
    assert!(i < jl_svec_len(t as *mut jl_svec_t));
    *(jl_svec_data(t as *mut jl_svec_t).offset(i as isize)) = x as *mut jl_value_t;
    if !x.is_null() {
        jl_gc_wb(t as *mut jl_svec_t, x);
    }
    x as *mut jl_value_t
}

getter_fun! { jl_array_len :: usize => jl_array_t>length }
getter_fun! { jl_array_data :: *mut c_void => jl_array_t>data }

pub unsafe fn jl_array_dim(a: *const jl_array_t, i: usize) -> usize {
    *((&(*a).nrows) as *const usize).offset(i as isize)
}

getter_fun! { jl_array_dim0 :: usize => jl_array_t>nrows }
getter_fun! { jl_array_nrows :: usize => jl_array_t>nrows }

pub unsafe fn jl_array_ndims<T>(a: *mut T) -> usize {
    (*(a as *mut jl_array_t)).flags.ndims() as usize
}

pub unsafe fn jl_array_ndimwords(ndims: usize) -> usize {
    if ndims < 3 { 0 }  else { ndims-2 }
}

pub unsafe fn jl_array_data_owner_offset(ndims: usize) -> usize {
    let a = mem::uninitialized::<jl_array_t>(); 
    let offset = ((&a.__bindgen_anon_1.ncols) as *const _ as usize) - ((&a) as *const _ as usize);

    offset + mem::size_of::<usize>() * (1 + jl_array_ndimwords(ndims))
}

pub unsafe fn jl_array_data_owner(a: *mut jl_array_t) -> *mut jl_value_t {
    *((a as *mut c_char).offset(jl_array_data_owner_offset(jl_array_ndims(a)) as isize) as *mut *mut jl_value_t)
}

pub unsafe fn jl_array_ptr_ref<T>(a: *mut T, i: usize) -> *mut jl_value_t {
    assert!(i < jl_array_len(a as *mut jl_array_t));
    *((jl_array_data(a as *mut jl_array_t) as *mut *mut jl_value_t).offset(i as isize))
}

pub unsafe fn jl_array_ptr_set<T, U>(mut a: *mut T, i: usize, x: *mut U) -> *mut jl_value_t {
    assert!(i < jl_array_len(a as *mut jl_array_t));
    *((jl_array_data(a as *mut jl_array_t) as *mut *mut jl_value_t).offset(i as isize)) = x as *mut jl_value_t;
    if !x.is_null() {
        if (*(a as *mut jl_array_t)).flags.how() == 3 {
            a = jl_array_data_owner(a as *mut jl_array_t) as *mut T;
        }
        jl_gc_wb(a, x);
    }
    x as *mut jl_value_t
}

pub unsafe fn jl_array_uint8_ref<T>(a: *mut T, i: usize) -> u8 {
    assert!(i < jl_array_len(a as *mut jl_array_t));
    assert!(jl_typeis(a, jl_array_uint8_type));
    *((jl_array_data(a as *mut jl_array_t) as *mut u8).offset(i as isize))
}

pub unsafe fn jl_array_uint8_set<T>(a: *mut T, i: usize, x: u8) -> u8 {
    assert!(i < jl_array_len(a as *mut jl_array_t));
    assert!(jl_typeis(a, jl_array_uint8_type));
    *((jl_array_data(a as *mut jl_array_t) as *mut u8).offset(i as isize)) = x;
    x
}

pub unsafe fn jl_exprarg<T>(e: *mut T , n: usize) -> *mut jl_value_t {
    *((jl_array_data((*(e as *mut jl_expr_t)).args) as *mut *mut jl_value_t).offset(n as isize))
}

pub unsafe fn jl_exprargset<T, U>(e: *mut T, n: usize, v: *mut U) -> *mut jl_value_t {
    jl_array_ptr_set((*(e as *mut jl_expr_t)).args, n, v)
}

pub unsafe fn jl_expr_nargs<T>(e: *mut T) -> usize {
    jl_array_len((*(e as *mut jl_expr_t)).args)
}


pub unsafe fn jl_fieldref<T>(s: *mut T, i: usize) -> *mut jl_value_t {
    jl_get_nth_field(s as *mut jl_value_t, i)
}

pub unsafe fn jl_nfields<T>(v: *mut T) -> usize {
    jl_datatype_nfields(jl_typeof(v)) as usize
}

// Not using jl_fieldref to avoid allocations
pub unsafe fn jl_linenode_line<T>(x: *mut T) -> isize {
    *(x as *mut isize)
}

pub unsafe fn jl_labelnode_label<T>(x: *mut T) -> isize {
    *(x as *mut isize)
}

pub unsafe fn jl_slot_number<T>(x: *mut T) -> isize {
    *(x as *mut isize)
}

pub unsafe fn jl_typedslot_get_type<T>(x: *mut T) -> *mut jl_value_t {
    *((x as *mut *mut jl_value_t).offset(1))
}

pub unsafe fn jl_gotonode_label<T>(x: *mut T) -> isize {
    *(x as *mut isize)
}

pub unsafe fn jl_globalref_mod<T>(s: *mut T) -> *mut jl_module_t {
    *(s as *mut *mut jl_module_t)
}

pub unsafe fn jl_globalref_name<T>(s: *mut T) -> *mut jl_sym_t {
    *((s as *mut *mut jl_sym_t).offset(1))
}


pub unsafe fn jl_nparams<T>(t: *mut T) -> usize {
    jl_svec_len((*(t as *mut jl_datatype_t)).parameters)
}

pub unsafe fn jl_tparam0<T>(t: *mut T) -> *mut jl_value_t {
    jl_tparam(t, 0)
}

pub unsafe fn jl_tparam1<T>(t: *mut T) -> *mut jl_value_t {
    jl_tparam(t, 1)
}

pub unsafe fn jl_tparam<T>(t: *mut T, i: usize) -> *mut jl_value_t {
    jl_svecref((*(t as *mut jl_datatype_t)).parameters, i)
}


// get a pointer to the data in a datatype
pub unsafe fn jl_data_ptr<T>(v: *mut T) -> *mut *mut jl_value_t {
    v as *mut *mut jl_value_t
}


pub unsafe fn jl_array_ptr_data<T>(a: *mut T) -> *mut *mut jl_value_t {
    (*(a as *mut jl_array_t)).data as *mut *mut jl_value_t
}

pub unsafe fn jl_string_data<T>(s: *mut T) -> *mut c_char {
    (s as *mut c_char).offset(mem::size_of::<*const c_void>() as isize)
}

pub unsafe fn jl_string_len<T>(s: *mut T) -> usize {
    *(s as *mut usize)
}

pub unsafe fn jl_gf_mtable<T>(f: *mut T) -> *mut jl_methtable_t {
    (*((*(jl_typeof(f) as *mut jl_datatype_t)).name)).mt
}

pub unsafe fn jl_gf_name<T>(f: *mut T) -> *mut jl_sym_t {
    (*jl_gf_mtable(f)).name
}


// struct type info
pub unsafe fn jl_field_name<T>(st: *mut T, i: usize) -> *mut jl_sym_t {
    jl_svecref((*((*(st as *mut jl_datatype_t)).name)).names, i) as *mut jl_sym_t
}

pub unsafe fn jl_field_type<T>(st: *mut T, i: usize) -> *mut jl_value_t {
    jl_svecref((*(st as *mut jl_datatype_t)).types, i)
}

pub unsafe fn jl_field_count<T>(st: *mut T) -> usize {
    jl_svec_len((*(st as *mut jl_datatype_t)).types)
}

pub unsafe fn jl_datatype_size<T>(t: *mut T) -> usize {
    (*(t as *mut jl_datatype_t)).size as usize
}

pub unsafe fn jl_datatype_align<T>(t: *mut T) -> usize {
    (*((*(t as *mut jl_datatype_t)).layout)).alignment() as usize
}

pub unsafe fn jl_datatype_nbits<T>(t: *mut T) -> usize {
    jl_datatype_size(t) * 8
}

pub unsafe fn jl_datatype_nfields<T>(t: *mut T) -> usize {
    (*((*(t as *mut jl_datatype_t)).layout)).nfields as usize
}


// from julia/dtypes.h: 
// #define LLT_ALIGN(x, sz) (((x) + (sz)-1) & -(sz))
fn LLT_ALIGN(x: usize, sz: usize) -> usize {
    (x + sz - 1) & (-(sz as isize) as usize)
}

pub unsafe fn jl_symbol_name<T>(s: *const T) -> *mut c_char {
    (s as *mut c_char).offset(LLT_ALIGN(mem::size_of::<jl_sym_t>(), mem::size_of::<*const c_void>()) as isize)
}

pub unsafe fn jl_dt_layout_fields<T>(d: *const T) -> *mut c_char {
    (d as *mut c_char).offset(mem::size_of::<jl_datatype_layout_t>() as isize)
}

pub unsafe fn jl_field_offset(st: *mut jl_datatype_t, i: usize) -> usize {
    let ly = (*st).layout;
    assert!(i < (*ly).nfields as usize);

    match (*ly).fielddesc_type() {
        0 => (*((jl_dt_layout_fields(ly) as *mut jl_fielddesc8_t).offset(i as isize))).offset as usize,
        1 => (*((jl_dt_layout_fields(ly) as *mut jl_fielddesc16_t).offset(i as isize))).offset as usize,
        _ => (*((jl_dt_layout_fields(ly) as *mut jl_fielddesc32_t).offset(i as isize))).offset as usize,
    }
}

pub unsafe fn jl_field_size(st: *mut jl_datatype_t, i: usize) -> usize {
    let ly = (*st).layout;
    assert!(i < (*ly).nfields as usize);

    match (*ly).fielddesc_type() {
        0 => (*((jl_dt_layout_fields(ly) as *mut jl_fielddesc8_t).offset(i as isize))).size() as usize,
        1 => (*((jl_dt_layout_fields(ly) as *mut jl_fielddesc16_t).offset(i as isize))).size() as usize,
        _ => (*((jl_dt_layout_fields(ly) as *mut jl_fielddesc32_t).offset(i as isize))).size() as usize,
    }
}

pub unsafe fn jl_is_nothing<T>(v: *mut T) -> bool { (v as *const jl_value_t) == jl_nothing }
pub unsafe fn jl_is_tuple<T>(v: *mut T) -> bool { (*(jl_typeof(v) as *mut jl_datatype_t)).name == jl_tuple_typename }
pub unsafe fn jl_is_svec<T>(v: *mut T) -> bool { jl_typeis(v, jl_simplevector_type) }
pub unsafe fn jl_is_simplevector<T>(v: *mut T) -> bool { jl_is_svec(v) }
pub unsafe fn jl_is_datatype<T>(v: *mut T) -> bool { jl_typeis(v, jl_datatype_type) }
pub unsafe fn jl_is_mutable<T>(t: *mut T) -> bool { (*(t as *mut jl_datatype_t)).mutabl != 0 }
pub unsafe fn jl_is_mutable_datatype<T>(t: *mut T) -> bool { jl_is_datatype(t) && jl_is_mutable(t) }
pub unsafe fn jl_is_immutable<T>(t: *mut T) -> bool { !jl_is_mutable(t) }
pub unsafe fn jl_is_immutable_datatype<T>(t: *mut T) -> bool { jl_is_datatype(t) && !jl_is_mutable(t) }
pub unsafe fn jl_is_uniontype<T>(v: *mut T) -> bool { jl_typeis(v, jl_uniontype_type) }
pub unsafe fn jl_is_typevar<T>(v: *mut T) -> bool { jl_typeis(v, jl_tvar_type) }
pub unsafe fn jl_is_unionall<T>(v: *mut T) -> bool { jl_typeis(v, jl_unionall_type) }
pub unsafe fn jl_is_typename<T>(v: *mut T) -> bool { jl_typeis(v, jl_typename_type) }
pub unsafe fn jl_is_int8<T>(v: *mut T) -> bool { jl_typeis(v, jl_int8_type) }
pub unsafe fn jl_is_int16<T>(v: *mut T) -> bool { jl_typeis(v, jl_int16_type) }
pub unsafe fn jl_is_int32<T>(v: *mut T) -> bool { jl_typeis(v, jl_int32_type) }
pub unsafe fn jl_is_int64<T>(v: *mut T) -> bool { jl_typeis(v, jl_int64_type) }
pub unsafe fn jl_is_uint8<T>(v: *mut T) -> bool { jl_typeis(v, jl_uint8_type) }
pub unsafe fn jl_is_uint16<T>(v: *mut T) -> bool { jl_typeis(v, jl_uint16_type) }
pub unsafe fn jl_is_uint32<T>(v: *mut T) -> bool { jl_typeis(v, jl_uint32_type) }
pub unsafe fn jl_is_uint64<T>(v: *mut T) -> bool { jl_typeis(v, jl_uint64_type) }
pub unsafe fn jl_is_float16<T>(v: *mut T) -> bool { jl_typeis(v, jl_float16_type) }
pub unsafe fn jl_is_float32<T>(v: *mut T) -> bool { jl_typeis(v, jl_float32_type) }
pub unsafe fn jl_is_float64<T>(v: *mut T) -> bool { jl_typeis(v, jl_float64_type) }
pub unsafe fn jl_is_bool<T>(v: *mut T) -> bool { jl_typeis(v, jl_bool_type) }
pub unsafe fn jl_is_symbol<T>(v: *mut T) -> bool { jl_typeis(v, jl_sym_type) }
pub unsafe fn jl_is_ssavalue<T>(v: *mut T) -> bool { jl_typeis(v, jl_ssavalue_type) }
pub unsafe fn jl_is_slot<T>(v: *mut T) -> bool { (jl_typeis(v, jl_slotnumber_type) || jl_typeis(v,jl_typedslot_type)) }
pub unsafe fn jl_is_expr<T>(v: *mut T) -> bool { jl_typeis(v, jl_expr_type) }
pub unsafe fn jl_is_globalref<T>(v: *mut T) -> bool { jl_typeis(v, jl_globalref_type) }
pub unsafe fn jl_is_labelnode<T>(v: *mut T) -> bool { jl_typeis(v, jl_labelnode_type) }
pub unsafe fn jl_is_gotonode<T>(v: *mut T) -> bool { jl_typeis(v, jl_gotonode_type) }
pub unsafe fn jl_is_quotenode<T>(v: *mut T) -> bool { jl_typeis(v, jl_quotenode_type) }
pub unsafe fn jl_is_newvarnode<T>(v: *mut T) -> bool { jl_typeis(v, jl_newvarnode_type) }
pub unsafe fn jl_is_linenode<T>(v: *mut T) -> bool { jl_typeis(v, jl_linenumbernode_type) }
pub unsafe fn jl_is_method_instance<T>(v: *mut T) -> bool { jl_typeis(v, jl_method_instance_type) }
pub unsafe fn jl_is_code_info<T>(v: *mut T) -> bool { jl_typeis(v, jl_code_info_type) }
pub unsafe fn jl_is_method<T>(v: *mut T) -> bool { jl_typeis(v, jl_method_type) }
pub unsafe fn jl_is_module<T>(v: *mut T) -> bool { jl_typeis(v, jl_module_type) }
pub unsafe fn jl_is_mtable<T>(v: *mut T) -> bool { jl_typeis(v, jl_methtable_type) }
pub unsafe fn jl_is_task<T>(v: *mut T) -> bool { jl_typeis(v, jl_task_type) }
pub unsafe fn jl_is_string<T>(v: *mut T) -> bool { jl_typeis(v, jl_string_type) }
pub unsafe fn jl_is_cpointer<T>(v: *mut T) -> bool { jl_is_cpointer_type(jl_typeof(v)) }
pub unsafe fn jl_is_pointer<T>(v: *mut T) -> bool { jl_is_cpointer_type(jl_typeof(v)) }
pub unsafe fn jl_is_intrinsic<T>(v: *mut T) -> bool { jl_typeis(v, jl_intrinsic_type) }

pub unsafe fn jl_is_kind(v: *mut jl_value_t) -> bool {
    v == jl_uniontype_type as *mut jl_value_t || v == jl_datatype_type as *mut jl_value_t ||
            v == jl_unionall_type as *mut jl_value_t || v == jl_typeofbottom_type as *mut jl_value_t
}

pub unsafe fn jl_is_type<T>(v: *mut T) -> bool {
    jl_is_kind(jl_typeof(v))
}

pub unsafe fn jl_is_primitivetype<T>(v: *mut T) -> bool {
    jl_is_datatype(v) && jl_is_immutable(v) &&
            !(*(v as *mut jl_datatype_t)).layout.is_null() &&
            jl_datatype_nfields(v) == 0 &&
            jl_datatype_size(v) > 0
}

pub unsafe fn jl_is_structtype<T>(v: *mut T) -> bool {
    jl_is_datatype(v) &&
            (jl_field_count(v) > 0 ||
             jl_datatype_size(v) == 0) &&
            (*(v as *mut jl_datatype_t)).abstract_ == 0
}

pub unsafe fn jl_isbits<T>(t: *mut T) -> bool {
    jl_is_datatype(t) && !(*(t as *mut jl_datatype_t)).layout.is_null() &&
            (*(t as *mut jl_datatype_t)).mutabl == 0 &&
            (*((*(t as *mut jl_datatype_t)).layout)).npointers() == 0
}

pub unsafe fn jl_is_datatype_singleton(d: *mut jl_datatype_t) -> bool {
    !(*d).instance.is_null()
}

pub unsafe fn jl_is_datatype_make_singleton(d: *mut jl_datatype_t) -> bool {
    (*d).abstract_ == 0 && jl_datatype_size(d) == 0 && d != jl_sym_type && (*d).name != jl_array_typename &&
            (*d).uid != 0 && ((*((*d).name)).names == jl_emptysvec || (*d).mutabl == 0)
}

pub unsafe fn jl_is_abstracttype<T>(v: *mut T) -> bool {
    jl_is_datatype(v) && (*(v as *mut jl_datatype_t)).abstract_ != 0
}

pub unsafe fn jl_is_array_type<T>(t: *mut T) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == jl_array_typename
}

pub unsafe fn jl_is_array<T>(v: *mut T) -> bool {
    let t = jl_typeof(v);
    jl_is_array_type(t)
}

pub unsafe fn jl_is_cpointer_type<T>(t: *mut T) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == (*((*jl_pointer_type).body as *mut jl_datatype_t)).name
}

pub unsafe fn jl_is_abstract_ref_type<T>(t: *mut T) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == (*((*jl_ref_type).body as *mut jl_datatype_t)).name
}

pub unsafe fn jl_is_tuple_type<T>(t: *mut T) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == jl_tuple_typename
}

pub unsafe fn jl_is_vecelement_type<T>(t: *mut T) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == jl_vecelement_typename
}

pub unsafe fn jl_is_type_type<T>(v: *mut T) -> bool {
    jl_is_datatype(v) &&
            (*(v as *mut jl_datatype_t)).name == (*((*jl_type_type).body as *mut jl_datatype_t)).name
}

pub unsafe fn jl_is_vararg_type<T>(mut v: *mut T) -> bool {
    v = jl_unwrap_unionall(v as *mut jl_value_t) as *mut T;
    jl_is_datatype(v) &&
            (*(v as *mut jl_datatype_t)).name == jl_vararg_typename
}

pub unsafe fn jl_unwrap_vararg<T>(v: *mut T) -> *mut jl_value_t {
    jl_tparam0(jl_unwrap_unionall(v as *mut jl_value_t))
}

pub unsafe fn jl_vararg_kind<T>(mut v: *mut T) -> jl_vararg_kind_t {
    if !jl_is_vararg_type(v) {
        return jl_vararg_kind_t_JL_VARARG_NONE;
    }

    let mut v1 = ptr::null_mut();
    let mut v2 = ptr::null_mut();

    if jl_is_unionall(v as *mut jl_value_t) {
        v1 = (*(v as *mut jl_unionall_t)).var as *mut jl_tvar_t;
        v = (*(v as *mut jl_unionall_t)).body as *mut T;
        if jl_is_unionall(v) {
            v2 = (*(v as *mut jl_unionall_t)).var as *mut jl_tvar_t;
            v = (*(v as *mut jl_unionall_t)).body as *mut T;
        }
    }

    assert!(jl_is_datatype(v));
    let lenv = jl_tparam1(v);

    if jl_is_long(lenv) {
        jl_vararg_kind_t_JL_VARARG_INT
    } else if jl_is_typevar(lenv) && lenv != v1 as *mut jl_value_t && lenv != v2 as *mut jl_value_t {
        jl_vararg_kind_t_JL_VARARG_BOUND
    } else {
        jl_vararg_kind_t_JL_VARARG_UNBOUND
    }
}

pub unsafe fn jl_is_va_tuple(t: *mut jl_datatype_t) -> bool {
    assert!(jl_is_tuple_type(t));
    let l = jl_svec_len((*t).parameters);
    l > 0 && jl_is_vararg_type(jl_tparam(t, l - 1))
}

pub unsafe fn jl_va_tuple_kind(mut t: *mut jl_datatype_t) -> jl_vararg_kind_t {
    t = jl_unwrap_unionall(t as *mut jl_value_t) as *mut jl_datatype_t;
    assert!(jl_is_tuple_type(t));

    let l = jl_svec_len((*t).parameters);
    if l == 0 {
        jl_vararg_kind_t_JL_VARARG_NONE
    } else {
        jl_vararg_kind(jl_tparam(t, l - 1))
    }
}

pub unsafe fn jl_get_function(m: *mut jl_module_t, name: *const c_char) -> *mut jl_function_t {
    jl_get_global(m, jl_symbol(name)) as *mut jl_function_t 
}

pub unsafe fn jl_vinfo_sa(vi: u8) -> bool {
    (vi & 16) != 0
}

pub unsafe fn jl_vinfo_usedundef(vi: u8) -> bool {
    (vi & 32) != 0
}

pub unsafe fn jl_apply<T>(args: *mut *mut T, nargs: usize) -> *mut jl_value_t {
    jl_apply_generic(args as *mut *mut jl_value_t, nargs as u32)
}

pub unsafe fn jl_eh_restore_state(eh: *mut jl_handler_t) {
    let ptls = jl_get_ptls_states();
    let current_task = (*ptls).current_task;
    // `eh` may not be `(*((*ptls).current_task)).eh`. See `jl_pop_handler`
    // This function should **NOT** have any safepoint before the ones at the
    // end.
    let old_defer_signal = (*ptls).defer_signal;
    let old_gc_state = (*ptls).gc_state;
    (*current_task).eh = (*eh).prev;
    (*ptls).pgcstack = (*eh).gcstack;
// TODO
/*
#ifdef JULIA_ENABLE_THREADING
    let locks = &(*current_task).locks;
    if (*locks).len > (*eh).locks_len {
        for i in ((*eh).locks_len .. (*locks).len).rev() {
            jl_mutex_unlock_nogc(*((*locks).items.offset((i - 1) as isize)) as *mut jl_mutex_t);
        }
        (*locks).len = (*eh).locks_len;
    }
#endif
*/
    (*ptls).world_age = (*eh).world_age;
    (*ptls).defer_signal = (*eh).defer_signal;
    (*ptls).gc_state = (*eh).gc_state;
    (*ptls).finalizers_inhibited = (*eh).finalizers_inhibited;
    if old_gc_state != 0 && (*eh).gc_state == 0 {
        // jl_gc_safepoint_(ptls); TODO: julia_threads.h
    }
    if old_defer_signal != 0 && (*eh).defer_signal == 0 {
        // jl_sigint_safepoint(ptls); TODO: julia_threads.h
    }
}

#[cfg(target_pointer_width = "64")]
mod box_long {
    use super::*;

    pub unsafe fn jl_box_long(x: isize) -> *mut jl_value_t {
        jl_box_int64(x as i64)
    }

    pub unsafe fn jl_box_ulong(x: usize) -> *mut jl_value_t {
        jl_box_uint64(x as u64)
    }

    pub unsafe fn jl_unbox_long(x: *mut jl_value_t) -> isize {
        jl_unbox_int64(x) as isize
    }

    pub unsafe fn jl_unbox_ulong(x: *mut jl_value_t) -> usize {
        jl_unbox_uint64(x) as usize
    }

    pub unsafe fn jl_is_long(x: *mut jl_value_t) -> bool {
        jl_is_int64(x)
    }

    pub unsafe fn jl_is_ulong(x: *mut jl_value_t) -> bool {
        jl_is_uint64(x)
    }

    extern "C" {
        #[link_name = "jl_int64_type"]
        pub static mut jl_long_type: *mut jl_datatype_t;
        #[link_name = "jl_uint64_type"]
        pub static mut jl_ulong_type: *mut jl_datatype_t;
    }
}

#[cfg(target_pointer_width = "32")]
mod box_long {
    use super::*;

    pub unsafe fn jl_box_long(x: isize) -> *mut jl_value_t {
        jl_box_int32(x as i32)
    }

    pub unsafe fn jl_box_ulong(x: usize) -> *mut jl_value_t {
        jl_box_uint64(x as u32)
    }

    pub unsafe fn jl_unbox_long(x: *mut jl_value_t) -> isize {
        jl_unbox_int32(x) as isize
    }

    pub unsafe fn jl_unbox_ulong(x: *mut jl_value_t) -> usize {
        jl_unbox_uint32(x) as usize
    }

    pub unsafe fn jl_is_long(x: *mut jl_value_t) -> bool {
        jl_is_int32(x)
    }

    pub unsafe fn jl_is_ulong(x: *mut jl_value_t) -> bool {
        jl_is_uint32(x)
    }

    extern "C" {
        #[link_name = "jl_int32_type"]
        pub static mut jl_long_type: *mut jl_datatype_t;
        #[link_name = "jl_uint32_type"]
        pub static mut jl_ulong_type: *mut jl_datatype_t;
    }
}

pub use box_long::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        unsafe {
            jl_init();
            assert!(jl_is_initialized() != 0);

            assert!(jl_exception_occurred().is_null());

            jl_atexit_hook(0);
        }
    }
}
