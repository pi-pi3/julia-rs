
use std::intrinsics::*;

use super::*;

pub unsafe fn jl_cpu_pause() -> i32 {
    0
}

pub unsafe fn jl_cpu_wake() -> i32 {
    0
}

pub const JL_CPU_WAKE_NOOP: i64 = 1;

#[cfg(windows)]
pub unsafe fn jl_thread_self() -> u64 {
    GetCurrentThreadId()
}

#[cfg(not(windows))]
pub unsafe fn jl_thread_self() -> u64 {
    pthread_self()
}

pub unsafe fn jl_signal_fence() {
    atomic_fence();
}

pub unsafe fn jl_atomic_fetch_add_relaxed<T>(obj: *mut T, arg: T) -> T {
    atomic_xadd_relaxed(obj, arg)
}

pub unsafe fn jl_atomic_fetch_add<T>(obj: *mut T, arg: T) -> T {
    atomic_xadd(obj, arg)
}

pub unsafe fn jl_atomic_fetch_and_relaxed<T>(obj: *mut T, arg: T) -> T {
    atomic_and_relaxed(obj, arg)
}

pub unsafe fn jl_atomic_fetch_and<T>(obj: *mut T, arg: T) -> T {
    atomic_and(obj, arg)
}

pub unsafe fn jl_atomic_fetch_or_relaxed<T>(obj: *mut T, arg: T) -> T {
    atomic_or_relaxed(obj, arg)
}

pub unsafe fn jl_atomic_fetch_or<T>(obj: *mut T, arg: T) -> T {
    atomic_or(obj, arg)
}

pub unsafe fn jl_atomic_compare_exchange<T>(obj: *mut T, expected: T, desired: T) -> T {
    let (ret, _) = atomic_cxchg(obj, expected, desired);
    ret
}

pub unsafe fn jl_atomic_store<T>(obj: *mut T, val: T) {
    atomic_store(obj, val);
}

pub unsafe fn jl_atomic_store_release<T>(obj: *mut T, val: T) {
    atomic_store_rel(obj, val);
}

pub unsafe fn jl_atomic_load<T>(obj: *mut T) -> T {
    atomic_load(obj)
}

pub unsafe fn jl_atomic_load_acquire<T>(obj: *mut T) -> T {
    atomic_load_acq(obj)
}

#[allow(path_statements)]
pub unsafe fn jl_gc_safepoint_(ptls: jl_ptls_t) {
    jl_signal_fence();
    let _safepoint_load = *(*ptls).safepoint;
    jl_signal_fence();
    _safepoint_load;
}

#[allow(path_statements)]
pub unsafe fn jl_sigint_safepoint(ptls: jl_ptls_t) {
    jl_signal_fence();
    let _safepoint_load = *(*ptls).safepoint.offset(-1);
    jl_signal_fence();
    _safepoint_load;
}

/* #ifndef JULIA_ENABLE_THREADING

pub unsafe fn jl_get_ptls_states() -> jl_ptls_t {
    *mut jl_tls_states
}

pub unsafe fn jl_gc_state(ptls: jl_ptls_t) -> i8 {
    0
}

pub unsafe fn jl_gc_state_set(_ptls: jl_ptls_t, _state: i8, old_state: i8) -> i8 {
    return old_state;
}

#else // ifndef JULIA_ENABLE_THREADING */

pub unsafe fn jl_gc_state(ptls: jl_ptls_t) -> i8 {
    (*ptls).gc_state
}

pub unsafe fn jl_gc_state_set(ptls: jl_ptls_t, state: i8, old_state: i8) -> i8 {
    (*ptls).gc_state = state;
    if old_state != 0 && state == 0 {
        jl_gc_safepoint_(ptls);
    }
    return old_state;
}

// #endif // ifndef JULIA_ENABLE_THREADING

pub unsafe fn jl_gc_state_save_and_set(ptls: jl_ptls_t, state: i8) -> i8 {
    jl_gc_state_set(ptls, state, jl_gc_state(ptls))
}

pub unsafe fn jl_gc_unsafe_enter(ptls: jl_ptls_t) -> i8 {
    jl_gc_state_save_and_set(ptls, 0)
}

pub unsafe fn jl_gc_unsafe_leave(ptls: jl_ptls_t, state: i8) {
    jl_gc_state_set(ptls, state, 0);
}

pub unsafe fn jl_gc_safe_enter(ptls: jl_ptls_t) -> i8 {
    jl_gc_state_save_and_set(ptls, JL_GC_STATE_SAFE as i8)
}

pub unsafe fn jl_gc_safe_leave(ptls: jl_ptls_t, state: i8) {
    jl_gc_state_set(ptls, state, JL_GC_STATE_SAFE as i8);
}

pub unsafe fn JL_SIGATOMIC_BEGIN() {
    (*jl_get_ptls_states()).defer_signal += 1;
    jl_signal_fence();
}

pub unsafe fn JL_SIGATOMIC_END() {
    jl_signal_fence();
    (*jl_get_ptls_states()).defer_signal -= 1;
    if (*jl_get_ptls_states()).defer_signal == 0 {
        jl_sigint_safepoint(jl_get_ptls_states());
    }
}


pub unsafe fn jl_mutex_wait(lock: *mut jl_mutex_t, safepoint: i32) {
    let _self = jl_thread_self();
    let mut owner = jl_atomic_load_acquire(&mut (*lock).owner as *mut _);
    if owner == _self {
        (*lock).count += 1;
        return;
    }

    loop {
        if owner == 0 &&
            jl_atomic_compare_exchange(&mut (*lock).owner as *mut _, 0, _self) == 0 {
            (*lock).count += 1;
            return;
        }
        if safepoint != 0 {
            let ptls = jl_get_ptls_states();
            jl_gc_safepoint_(ptls);
        }
        jl_cpu_pause();
        owner = (*lock).owner;
    }
}

pub unsafe fn jl_mutex_lock_nogc(lock: *mut jl_mutex_t) {
    jl_mutex_wait(lock, 0);
}

pub unsafe fn jl_mutex_lock(lock: *mut jl_mutex_t) {
    let ptls = jl_get_ptls_states();
    JL_SIGATOMIC_BEGIN();
    jl_mutex_wait(lock, 1);
    jl_lock_frame_push(lock);
    jl_gc_enable_finalizers(ptls, 0);
}

pub unsafe fn jl_mutex_lock_maybe_nogc(lock: *mut jl_mutex_t) {
    let ptls = jl_get_ptls_states();
    if !(*ptls).safepoint.is_null() {
        jl_mutex_lock(lock);
    } else {
        jl_mutex_lock_nogc(lock);
    }
}

pub unsafe fn jl_mutex_unlock_nogc(lock: *mut jl_mutex_t) {
    assert_eq!((*lock).owner, jl_thread_self(),
           "Unlocking a lock in a different thread.");
    (*lock).owner -= 1;
    if (*lock).owner == 0 {
        jl_atomic_store_release(&mut (*lock).owner as *mut u64, 0);
        jl_cpu_wake();
    }
}

pub unsafe fn jl_mutex_unlock(lock: *mut jl_mutex_t) {
    let ptls = jl_get_ptls_states();
    jl_mutex_unlock_nogc(lock);
    jl_gc_enable_finalizers(ptls, 1);
    jl_lock_frame_pop();
    JL_SIGATOMIC_END();
}

pub unsafe fn jl_mutex_unlock_maybe_nogc(lock: *mut jl_mutex_t) {
    let ptls = jl_get_ptls_states();
    if !(*ptls).safepoint.is_null() {
        jl_mutex_unlock(lock);
    } else {
        jl_mutex_unlock_nogc(lock);
    }
}

pub unsafe fn jl_mutex_init(lock: *mut jl_mutex_t) {
    (*lock).owner = 0;
    (*lock).count = 0;
}


pub unsafe fn JL_MUTEX_INIT(m: *mut jl_mutex_t) {
    jl_mutex_init(m)
}

pub unsafe fn JL_LOCK(m: *mut jl_mutex_t) {
    jl_mutex_lock(m)
}

pub unsafe fn JL_UNLOCK(m: *mut jl_mutex_t) {
    jl_mutex_unlock(m)
}

pub unsafe fn JL_LOCK_NOGC(m: *mut jl_mutex_t) {
    jl_mutex_lock_nogc(m)
}

pub unsafe fn JL_UNLOCK_NOGC(m: *mut jl_mutex_t) {
    jl_mutex_unlock_nogc(m)
}
