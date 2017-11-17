
#![feature(core_intrinsics)]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate libc;

use libc::{c_void, c_char};
use std::mem;
use std::ptr;

mod threads;
pub use threads::*;

pub unsafe fn jl_astaggedvalue(v: *mut c_void) -> *mut jl_taggedvalue_t {
    (v as *const c_char).offset(-(mem::size_of::<jl_taggedvalue_t>() as isize)) as *mut jl_taggedvalue_t 
}

pub unsafe fn jl_valueof(v: *mut c_void) -> *mut jl_value_t {
    (v as *const c_char).offset(mem::size_of::<jl_taggedvalue_t>() as isize) as *mut jl_value_t 
}

pub unsafe fn jl_typeof(v: *mut c_void) -> *mut jl_value_t {
    ((*jl_astaggedvalue(v)).__bindgen_anon_1.header & (!(15 as usize))) as *mut jl_value_t
}

pub unsafe fn jl_typeis(v: *mut c_void, t: *mut c_void) -> bool {
    jl_typeof(v) == t as *mut jl_value_t
}

pub unsafe fn jl_pgcstack() -> *mut jl_gcframe_t {
    (*jl_get_ptls_states()).pgcstack
}

pub unsafe fn jl_pgcstack_usize() -> usize {
    (*jl_get_ptls_states()).pgcstack as usize
}

pub unsafe fn JL_GC_PUSH1(arg1: *const c_void) {
    let __gc_stkf = [3_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH2(arg1: *const c_void, arg2: *const c_void) {
    let __gc_stkf = [5_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void, arg2 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH3(arg1: *const c_void, arg2: *const c_void, arg3: *const c_void) {
    let __gc_stkf = [7_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void, arg2 as *const c_void,
                     arg3 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH4(arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void) {
    let __gc_stkf = [9_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void, arg2 as *const c_void,
                     arg3 as *const c_void, arg4 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH5(arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void, arg5: *const c_void) {
    let __gc_stkf = [11_usize as *const c_void,
                     jl_pgcstack_usize() as *const c_void,
                     arg1 as *const c_void, arg2 as *const c_void,
                     arg3 as *const c_void, arg4 as *const c_void,
                     arg5 as *const c_void].as_mut_ptr();
    
    (*jl_get_ptls_states()).pgcstack = __gc_stkf as *mut jl_gcframe_t;
}

pub unsafe fn JL_GC_PUSH6(arg1: *const c_void, arg2: *const c_void, arg3: *const c_void, arg4: *const c_void, arg5: *const c_void, arg6: *const c_void) {
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

pub unsafe fn jl_gc_wb(parent: *mut c_void, ptr: *mut c_void) {
    // parent and ptr isa jl_value_t*
    if (*jl_astaggedvalue(parent)).__bindgen_anon_1.bits.gc() == 3 &&
        ((*jl_astaggedvalue(ptr)).__bindgen_anon_1.bits.gc() & 1) == 0 {
        jl_gc_queue_root(parent as *mut _);
    }
}

pub unsafe fn jl_gc_wb_back(ptr: *mut c_void) {
    // if ptr is old
    if (*jl_astaggedvalue(ptr)).__bindgen_anon_1.bits.gc() == 3 {
        jl_gc_queue_root(ptr as *mut _);
    }
}

macro_rules! getter_fun {
    { $name:ident :: $ft:ty  => $t:ty > $f:ident } => (
        pub unsafe fn $name(x: *mut c_void) -> $ft {
            (*(x as *mut $t)).$f as $ft
        }
    )
}


macro_rules! setter_fun {
    { $name:ident => $t:ty > $f:ident = $ft:ty   } => (
        pub unsafe fn $name(x: *mut c_void, v: $ft) -> $ft {
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

pub unsafe fn jl_svecref(t: *mut c_void, i: usize) -> *mut jl_value_t {
    assert!(jl_typeis(t, jl_simplevector_type as *mut _));
    assert!(i < jl_svec_len(t));
    *(jl_svec_data(t as *mut _).offset(i as isize))
}

pub unsafe fn jl_svecset(t: *mut c_void, i: usize, x: *mut c_void) -> *mut jl_value_t {
    assert!(jl_typeis(t, jl_simplevector_type as *mut _));
    assert!(i < jl_svec_len(t));
    *(jl_svec_data(t as *mut _).offset(i as isize)) = x as *mut jl_value_t;
    if !x.is_null() {
        jl_gc_wb(t, x);
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

pub unsafe fn jl_array_ndims(a: *mut c_void) -> usize {
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

pub unsafe fn jl_array_data_owner(a: *mut c_void) -> *mut jl_value_t {
    *((a as *mut c_char).offset(jl_array_data_owner_offset(jl_array_ndims(a)) as isize) as *mut *mut jl_value_t)
}

pub unsafe fn jl_array_ptr_ref(a: *mut c_void, i: usize) -> *mut jl_value_t {
    assert!(i < jl_array_len(a));
    *((jl_array_data(a) as *mut *mut jl_value_t).offset(i as isize))
}

pub unsafe fn jl_array_ptr_set(mut a: *mut c_void, i: usize, x: *mut c_void) -> *mut jl_value_t {
    assert!(i < jl_array_len(a));
    *((jl_array_data(a) as *mut *mut jl_value_t).offset(i as isize)) = x as *mut jl_value_t;
    if !x.is_null() {
        if (*(a as *mut jl_array_t)).flags.how() == 3 {
            a = jl_array_data_owner(a) as *mut c_void;
        }
        jl_gc_wb(a, x);
    }
    x as *mut jl_value_t
}

pub unsafe fn jl_array_uint8_ref(a: *mut c_void, i: usize) -> u8 {
    assert!(i < jl_array_len(a));
    assert!(jl_typeis(a, jl_array_uint8_type as *mut _));
    *((jl_array_data(a) as *mut u8).offset(i as isize))
}

pub unsafe fn jl_array_uint8_set(a: *mut c_void, i: usize, x: u8) -> u8 {
    assert!(i < jl_array_len(a));
    assert!(jl_typeis(a, jl_array_uint8_type as *mut _));
    *((jl_array_data(a) as *mut u8).offset(i as isize)) = x;
    x
}

pub unsafe fn jl_exprarg(e: *mut c_void , n: usize) -> *mut jl_value_t {
    *((jl_array_data((*(e as *mut jl_expr_t)).args as *mut _) as *mut *mut jl_value_t).offset(n as isize))
}

pub unsafe fn jl_exprargset(e: *mut c_void, n: usize, v: *mut c_void) -> *mut jl_value_t {
    jl_array_ptr_set((*(e as *mut jl_expr_t)).args as *mut _, n, v)
}

pub unsafe fn jl_expr_nargs(e: *mut c_void) -> usize {
    jl_array_len((*(e as *mut jl_expr_t)).args as *mut _)
}


pub unsafe fn jl_fieldref(s: *mut c_void, i: usize) -> *mut jl_value_t {
    jl_get_nth_field(s as *mut _, i)
}

pub unsafe fn jl_nfields(v: *mut c_void) -> usize {
    jl_datatype_nfields(jl_typeof(v) as *mut _) as usize
}

// Not using jl_fieldref to avoid allocations
pub unsafe fn jl_linenode_line(x: *mut c_void) -> isize {
    *(x as *mut isize)
}

pub unsafe fn jl_labelnode_label(x: *mut c_void) -> isize {
    *(x as *mut isize)
}

pub unsafe fn jl_slot_number(x: *mut c_void) -> isize {
    *(x as *mut isize)
}

pub unsafe fn jl_typedslot_get_type(x: *mut c_void) -> *mut jl_value_t {
    *((x as *mut *mut jl_value_t).offset(1))
}

pub unsafe fn jl_gotonode_label(x: *mut c_void) -> isize {
    *(x as *mut isize)
}

pub unsafe fn jl_globalref_mod(s: *mut c_void) -> *mut jl_module_t {
    *(s as *mut *mut jl_module_t)
}

pub unsafe fn jl_globalref_name(s: *mut c_void) -> *mut jl_sym_t {
    *((s as *mut *mut jl_sym_t).offset(1))
}


pub unsafe fn jl_nparams(t: *mut c_void) -> usize {
    jl_svec_len((*(t as *mut jl_datatype_t)).parameters as *mut _)
}

pub unsafe fn jl_tparam0(t: *mut c_void) -> *mut jl_value_t {
    jl_tparam(t, 0)
}

pub unsafe fn jl_tparam1(t: *mut c_void) -> *mut jl_value_t {
    jl_tparam(t, 1)
}

pub unsafe fn jl_tparam(t: *mut c_void, i: usize) -> *mut jl_value_t {
    jl_svecref((*(t as *mut jl_datatype_t)).parameters as *mut _, i)
}


// get a pointer to the data in a datatype
pub unsafe fn jl_data_ptr(v: *mut c_void) -> *mut *mut jl_value_t {
    v as *mut *mut jl_value_t
}


pub unsafe fn jl_array_ptr_data(a: *mut c_void) -> *mut *mut jl_value_t {
    (*(a as *mut jl_array_t)).data as *mut *mut jl_value_t
}

pub unsafe fn jl_string_data(s: *mut c_void) -> *mut c_char {
    (s as *mut c_char).offset(mem::size_of::<*const c_void>() as isize)
}

pub unsafe fn jl_string_len(s: *mut c_void) -> usize {
    *(s as *mut usize)
}

pub unsafe fn jl_gf_mtable(f: *mut c_void) -> *mut jl_methtable_t {
    (*((*(jl_typeof(f) as *mut jl_datatype_t)).name)).mt
}

pub unsafe fn jl_gf_name(f: *mut c_void) -> *mut jl_sym_t {
    (*jl_gf_mtable(f)).name
}


// struct type info
pub unsafe fn jl_field_name(st: *mut c_void, i: usize) -> *mut jl_sym_t {
    jl_svecref((*((*(st as *mut jl_datatype_t)).name)).names as *mut _, i) as *mut jl_sym_t
}

pub unsafe fn jl_field_type(st: *mut c_void, i: usize) -> *mut jl_value_t {
    jl_svecref((*(st as *mut jl_datatype_t)).types as *mut _, i)
}

pub unsafe fn jl_field_count(st: *mut c_void) -> usize {
    jl_svec_len((*(st as *mut jl_datatype_t)).types as *mut _)
}

pub unsafe fn jl_datatype_size(t: *mut c_void) -> usize {
    (*(t as *mut jl_datatype_t)).size as usize
}

pub unsafe fn jl_datatype_align(t: *mut c_void) -> usize {
    (*((*(t as *mut jl_datatype_t)).layout)).alignment() as usize
}

pub unsafe fn jl_datatype_nbits(t: *mut c_void) -> usize {
    jl_datatype_size(t) * 8
}

pub unsafe fn jl_datatype_nfields(t: *mut c_void) -> usize {
    (*((*(t as *mut jl_datatype_t)).layout)).nfields as usize
}


// from julia/dtypes.h: 
// #define LLT_ALIGN(x, sz) (((x) + (sz)-1) & -(sz))
fn LLT_ALIGN(x: usize, sz: usize) -> usize {
    (x + sz - 1) & (-(sz as isize) as usize)
}

pub unsafe fn jl_symbol_name(s: *const c_void) -> *mut c_char {
    (s as *mut c_char).offset(LLT_ALIGN(mem::size_of::<jl_sym_t>(), mem::size_of::<*const c_void>()) as isize)
}

pub unsafe fn jl_dt_layout_fields(d: *const c_void) -> *mut c_char {
    (d as *mut c_char).offset(mem::size_of::<jl_datatype_layout_t>() as isize)
}

pub unsafe fn jl_field_offset(st: *mut jl_datatype_t, i: usize) -> usize {
    let ly = (*st).layout;
    assert!(i < (*ly).nfields as usize);

    match (*ly).fielddesc_type() {
        0 => (*((jl_dt_layout_fields(ly as *const _) as *mut jl_fielddesc8_t).offset(i as isize))).offset as usize,
        1 => (*((jl_dt_layout_fields(ly as *const _) as *mut jl_fielddesc16_t).offset(i as isize))).offset as usize,
        _ => (*((jl_dt_layout_fields(ly as *const _) as *mut jl_fielddesc32_t).offset(i as isize))).offset as usize,
    }
}

pub unsafe fn jl_field_size(st: *mut jl_datatype_t, i: usize) -> usize {
    let ly = (*st).layout;
    assert!(i < (*ly).nfields as usize);

    match (*ly).fielddesc_type() {
        0 => (*((jl_dt_layout_fields(ly as *const _) as *mut jl_fielddesc8_t).offset(i as isize))).size() as usize,
        1 => (*((jl_dt_layout_fields(ly as *const _) as *mut jl_fielddesc16_t).offset(i as isize))).size() as usize,
        _ => (*((jl_dt_layout_fields(ly as *const _) as *mut jl_fielddesc32_t).offset(i as isize))).size() as usize,
    }
}

pub unsafe fn jl_is_nothing(v: *mut c_void) -> bool { (v as *const jl_value_t) == jl_nothing }
pub unsafe fn jl_is_tuple(v: *mut c_void) -> bool { (*(jl_typeof(v) as *mut jl_datatype_t)).name == jl_tuple_typename }
pub unsafe fn jl_is_svec(v: *mut c_void) -> bool { jl_typeis(v, jl_simplevector_type as *mut _) }
pub unsafe fn jl_is_simplevector(v: *mut c_void) -> bool { jl_is_svec(v) }
pub unsafe fn jl_is_datatype(v: *mut c_void) -> bool { jl_typeis(v, jl_datatype_type as *mut _) }
pub unsafe fn jl_is_mutable(t: *mut c_void) -> bool { (*(t as *mut jl_datatype_t)).mutabl != 0 }
pub unsafe fn jl_is_mutable_datatype(t: *mut c_void) -> bool { jl_is_datatype(t) && jl_is_mutable(t) }
pub unsafe fn jl_is_immutable(t: *mut c_void) -> bool { !jl_is_mutable(t) }
pub unsafe fn jl_is_immutable_datatype(t: *mut c_void) -> bool { jl_is_datatype(t) && !jl_is_mutable(t) }
pub unsafe fn jl_is_uniontype(v: *mut c_void) -> bool { jl_typeis(v, jl_uniontype_type as *mut _) }
pub unsafe fn jl_is_typevar(v: *mut c_void) -> bool { jl_typeis(v, jl_tvar_type as *mut _) }
pub unsafe fn jl_is_unionall(v: *mut c_void) -> bool { jl_typeis(v, jl_unionall_type as *mut _) }
pub unsafe fn jl_is_typename(v: *mut c_void) -> bool { jl_typeis(v, jl_typename_type as *mut _) }
pub unsafe fn jl_is_int8(v: *mut c_void) -> bool { jl_typeis(v, jl_int8_type as *mut _) }
pub unsafe fn jl_is_int16(v: *mut c_void) -> bool { jl_typeis(v, jl_int16_type as *mut _) }
pub unsafe fn jl_is_int32(v: *mut c_void) -> bool { jl_typeis(v, jl_int32_type as *mut _) }
pub unsafe fn jl_is_int64(v: *mut c_void) -> bool { jl_typeis(v, jl_int64_type as *mut _) }
pub unsafe fn jl_is_uint8(v: *mut c_void) -> bool { jl_typeis(v, jl_uint8_type as *mut _) }
pub unsafe fn jl_is_uint16(v: *mut c_void) -> bool { jl_typeis(v, jl_uint16_type as *mut _) }
pub unsafe fn jl_is_uint32(v: *mut c_void) -> bool { jl_typeis(v, jl_uint32_type as *mut _) }
pub unsafe fn jl_is_uint64(v: *mut c_void) -> bool { jl_typeis(v, jl_uint64_type as *mut _) }
pub unsafe fn jl_is_float16(v: *mut c_void) -> bool { jl_typeis(v, jl_float16_type as *mut _) }
pub unsafe fn jl_is_float32(v: *mut c_void) -> bool { jl_typeis(v, jl_float32_type as *mut _) }
pub unsafe fn jl_is_float64(v: *mut c_void) -> bool { jl_typeis(v, jl_float64_type as *mut _) }
pub unsafe fn jl_is_bool(v: *mut c_void) -> bool { jl_typeis(v, jl_bool_type as *mut _) }
pub unsafe fn jl_is_symbol(v: *mut c_void) -> bool { jl_typeis(v, jl_sym_type as *mut _) }
pub unsafe fn jl_is_ssavalue(v: *mut c_void) -> bool { jl_typeis(v, jl_ssavalue_type as *mut _) }
pub unsafe fn jl_is_slot(v: *mut c_void) -> bool { jl_typeis(v, jl_slotnumber_type as *mut _) || jl_typeis(v, jl_typedslot_type as *mut _) }
pub unsafe fn jl_is_expr(v: *mut c_void) -> bool { jl_typeis(v, jl_expr_type as *mut _) }
pub unsafe fn jl_is_globalref(v: *mut c_void) -> bool { jl_typeis(v, jl_globalref_type as *mut _) }
pub unsafe fn jl_is_labelnode(v: *mut c_void) -> bool { jl_typeis(v, jl_labelnode_type as *mut _) }
pub unsafe fn jl_is_gotonode(v: *mut c_void) -> bool { jl_typeis(v, jl_gotonode_type as *mut _) }
pub unsafe fn jl_is_quotenode(v: *mut c_void) -> bool { jl_typeis(v, jl_quotenode_type as *mut _) }
pub unsafe fn jl_is_newvarnode(v: *mut c_void) -> bool { jl_typeis(v, jl_newvarnode_type as *mut _) }
pub unsafe fn jl_is_linenode(v: *mut c_void) -> bool { jl_typeis(v, jl_linenumbernode_type as *mut _) }
pub unsafe fn jl_is_method_instance(v: *mut c_void) -> bool { jl_typeis(v, jl_method_instance_type as *mut _) }
pub unsafe fn jl_is_code_info(v: *mut c_void) -> bool { jl_typeis(v, jl_code_info_type as *mut _) }
pub unsafe fn jl_is_method(v: *mut c_void) -> bool { jl_typeis(v, jl_method_type as *mut _) }
pub unsafe fn jl_is_module(v: *mut c_void) -> bool { jl_typeis(v, jl_module_type as *mut _) }
pub unsafe fn jl_is_mtable(v: *mut c_void) -> bool { jl_typeis(v, jl_methtable_type as *mut _) }
pub unsafe fn jl_is_task(v: *mut c_void) -> bool { jl_typeis(v, jl_task_type as *mut _) }
pub unsafe fn jl_is_string(v: *mut c_void) -> bool { jl_typeis(v, jl_string_type as *mut _) }
pub unsafe fn jl_is_cpointer(v: *mut c_void) -> bool { jl_is_cpointer_type(jl_typeof(v) as *mut _) }
pub unsafe fn jl_is_pointer(v: *mut c_void) -> bool { jl_is_cpointer_type(jl_typeof(v) as *mut _) }
pub unsafe fn jl_is_intrinsic(v: *mut c_void) -> bool { jl_typeis(v, jl_intrinsic_type as *mut _) }

pub unsafe fn jl_is_kind(v: *mut jl_value_t) -> bool {
    v == jl_uniontype_type as *mut jl_value_t || v == jl_datatype_type as *mut jl_value_t ||
            v == jl_unionall_type as *mut jl_value_t || v == jl_typeofbottom_type as *mut jl_value_t
}

pub unsafe fn jl_is_type(v: *mut c_void) -> bool {
    jl_is_kind(jl_typeof(v))
}

pub unsafe fn jl_is_primitivetype(v: *mut c_void) -> bool {
    jl_is_datatype(v) && jl_is_immutable(v) &&
            !(*(v as *mut jl_datatype_t)).layout.is_null() &&
            jl_datatype_nfields(v) == 0 &&
            jl_datatype_size(v) > 0
}

pub unsafe fn jl_is_structtype(v: *mut c_void) -> bool {
    jl_is_datatype(v) &&
            (jl_field_count(v) > 0 ||
             jl_datatype_size(v) == 0) &&
            (*(v as *mut jl_datatype_t)).abstract_ == 0
}

pub unsafe fn jl_isbits(t: *mut c_void) -> bool {
    jl_is_datatype(t) && !(*(t as *mut jl_datatype_t)).layout.is_null() &&
            (*(t as *mut jl_datatype_t)).mutabl == 0 &&
            (*((*(t as *mut jl_datatype_t)).layout)).npointers() == 0
}

pub unsafe fn jl_is_datatype_singleton(d: *mut jl_datatype_t) -> bool {
    !(*d).instance.is_null()
}

pub unsafe fn jl_is_datatype_make_singleton(d: *mut jl_datatype_t) -> bool {
    (*d).abstract_ == 0 && jl_datatype_size(d as *mut _) == 0 && d != jl_sym_type && (*d).name != jl_array_typename &&
            (*d).uid != 0 && ((*((*d).name)).names == jl_emptysvec || (*d).mutabl == 0)
}

pub unsafe fn jl_is_abstracttype(v: *mut c_void) -> bool {
    jl_is_datatype(v) && (*(v as *mut jl_datatype_t)).abstract_ != 0
}

pub unsafe fn jl_is_array_type(t: *mut c_void) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == jl_array_typename
}

pub unsafe fn jl_is_array(v: *mut c_void) -> bool {
    let t = jl_typeof(v);
    jl_is_array_type(t as *mut _)
}

pub unsafe fn jl_is_cpointer_type(t: *mut c_void) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == (*((*jl_pointer_type).body as *mut jl_datatype_t)).name
}

pub unsafe fn jl_is_abstract_ref_type(t: *mut c_void) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == (*((*jl_ref_type).body as *mut jl_datatype_t)).name
}

pub unsafe fn jl_is_tuple_type(t: *mut c_void) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == jl_tuple_typename
}

pub unsafe fn jl_is_vecelement_type(t: *mut c_void) -> bool {
    jl_is_datatype(t) &&
            (*(t as *mut jl_datatype_t)).name == jl_vecelement_typename
}

pub unsafe fn jl_is_type_type(v: *mut c_void) -> bool {
    jl_is_datatype(v) &&
            (*(v as *mut jl_datatype_t)).name == (*((*jl_type_type).body as *mut jl_datatype_t)).name
}

pub unsafe fn jl_is_vararg_type(mut v: *mut c_void) -> bool {
    v = jl_unwrap_unionall(v as *mut _) as *mut c_void;
    jl_is_datatype(v) &&
            (*(v as *mut jl_datatype_t)).name == jl_vararg_typename
}

pub unsafe fn jl_unwrap_vararg(v: *mut c_void) -> *mut jl_value_t {
    jl_tparam0(jl_unwrap_unionall(v as *mut _) as *mut _)
}

pub unsafe fn jl_vararg_kind(mut v: *mut c_void) -> jl_vararg_kind_t {
    if !jl_is_vararg_type(v) {
        return jl_vararg_kind_t_JL_VARARG_NONE;
    }

    let mut v1 = ptr::null_mut();
    let mut v2 = ptr::null_mut();

    if jl_is_unionall(v) {
        v1 = (*(v as *mut jl_unionall_t)).var as *mut jl_tvar_t;
        v = (*(v as *mut jl_unionall_t)).body as *mut c_void;
        if jl_is_unionall(v) {
            v2 = (*(v as *mut jl_unionall_t)).var as *mut jl_tvar_t;
            v = (*(v as *mut jl_unionall_t)).body as *mut c_void;
        }
    }

    assert!(jl_is_datatype(v));
    let lenv = jl_tparam1(v);

    if jl_is_long(lenv) {
        jl_vararg_kind_t_JL_VARARG_INT
    } else if jl_is_typevar(lenv as *mut _) && lenv != v1 as *mut jl_value_t && lenv != v2 as *mut jl_value_t {
        jl_vararg_kind_t_JL_VARARG_BOUND
    } else {
        jl_vararg_kind_t_JL_VARARG_UNBOUND
    }
}

pub unsafe fn jl_is_va_tuple(t: *mut jl_datatype_t) -> bool {
    assert!(jl_is_tuple_type(t as *mut _));
    let l = jl_svec_len((*t).parameters as *mut _);
    l > 0 && jl_is_vararg_type(jl_tparam(t as *mut _, l - 1) as *mut _)
}

pub unsafe fn jl_va_tuple_kind(mut t: *mut jl_datatype_t) -> jl_vararg_kind_t {
    t = jl_unwrap_unionall(t as *mut _) as *mut jl_datatype_t;
    assert!(jl_is_tuple_type(t as *mut _));

    let l = jl_svec_len((*t).parameters as *mut _);
    if l == 0 {
        jl_vararg_kind_t_JL_VARARG_NONE
    } else {
        jl_vararg_kind(jl_tparam(t as *mut _, l - 1) as *mut _)
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

pub unsafe fn jl_apply(args: *mut *mut c_void, nargs: usize) -> *mut jl_value_t {
    jl_apply_generic(args as *mut *mut jl_value_t, nargs as u32)
}

pub unsafe fn jl_lock_frame_push(lock: *mut jl_mutex_t) {
    let ptls = jl_get_ptls_states();
    if (*ptls).current_task.is_null() {
        return;
    }

    let locks = &mut (*(*ptls).current_task).locks as *mut arraylist_t;
    let len = (*locks).len;

    if len >= (*locks).max {
        arraylist_grow(locks, 1);
    } else {
        (*locks).len = len + 1;
    }
    *(*locks).items.offset(len as isize) = lock as *mut _;
}

pub unsafe fn jl_lock_frame_pop() {
    let ptls = jl_get_ptls_states();
    if !(*ptls).current_task.is_null() {
        (*(*ptls).current_task).locks.len -= 1;
    }
}

pub unsafe fn jl_eh_restore_state(eh: *mut jl_handler_t) {
    let ptls = jl_get_ptls_states();
    let current_task = (*ptls).current_task;
    
    let old_defer_signal = (*ptls).defer_signal;
    let old_gc_state = (*ptls).gc_state;
    (*current_task).eh = (*eh).prev;
    (*ptls).pgcstack = (*eh).gcstack;

    let locks = &mut (*current_task).locks as *mut arraylist_t;
    if (*locks).len > (*eh).locks_len {
        let mut i = (*locks).len;
        while i < (*eh).locks_len {
            jl_mutex_unlock_nogc(*((*locks).items.offset((i - 1) as isize)) as *mut jl_mutex_t);
            i += 1;
        }
        (*locks).len = (*eh).locks_len;
    }

    (*ptls).world_age = (*eh).world_age;
    (*ptls).defer_signal = (*eh).defer_signal;
    (*ptls).gc_state = (*eh).gc_state;
    (*ptls).finalizers_inhibited = (*eh).finalizers_inhibited;
    if old_gc_state != 0 && (*eh).gc_state == 0 {
        jl_gc_safepoint_(ptls);
    }
    if old_defer_signal != 0 && (*eh).defer_signal == 0 {
        jl_sigint_safepoint(ptls);
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
        jl_is_int64(x as *mut _)
    }

    pub unsafe fn jl_is_ulong(x: *mut jl_value_t) -> bool {
        jl_is_uint64(x as *mut _)
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
