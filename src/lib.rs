#![allow(non_camel_case_types)]

extern crate libc;

pub mod acc;
pub mod pow;
pub mod accpow;

use libc::{uint64_t, c_double, c_int};

/// Typedef for the window completion callback function.
#[repr(C)]
pub type heartbeat_window_complete = extern fn(*const heartbeat_context);

/// Unsigned global and window data.
#[repr(C)]
pub struct heartbeat_udata {
    pub global: uint64_t,
    pub window: uint64_t,
}

/// Rate data.
#[repr(C)]
pub struct heartbeat_rates {
    pub global: c_double,
    pub window: c_double,
    pub instant: c_double,
}

/// State for window buffer
#[repr(C)]
pub struct heartbeat_window_state {
    pub buffer_index: uint64_t,
    pub read_index: uint64_t,
    pub window_size: uint64_t,
}

/// A heartbeat record with current rates.
#[repr(C)]
pub struct heartbeat_record {
    pub id: uint64_t,
    pub user_tag: uint64_t,

    pub work: uint64_t,
    pub start_time: uint64_t,
    pub end_time: uint64_t,
    pub perf: heartbeat_rates,
}

/// A `heartbeat_context` is used for tracking performance/power of recurring jobs.
#[repr(C)]
pub struct heartbeat_context {
    pub ws: heartbeat_window_state,
    pub window_buffer: *mut heartbeat_record,
    pub counter: uint64_t,
    pub lock: c_int,
    pub hwc_callback: heartbeat_window_complete,

    pub td: heartbeat_udata,
    pub wd: heartbeat_udata,
    pub ed: heartbeat_udata,
}

extern "C" {
    // Core functions

    pub fn heartbeat_init(hb: *mut heartbeat_context,
                          window_size: uint64_t,
                          window_buffer: *mut heartbeat_record,
                          hwc_callback: Option<heartbeat_window_complete>) -> c_int;

    pub fn heartbeat(hb: *mut heartbeat_context,
                     user_tag: uint64_t,
                     work: uint64_t,
                     start_time: uint64_t,
                     end_time: uint64_t,
                     start_energy: uint64_t,
                     end_energy: uint64_t);

    pub fn heartbeat_log_window_buffer(hb: *const heartbeat_context,
                                       fd: c_int,
                                       print_header: c_int) -> c_int;

    // Utility functions

    pub fn hb_get_window_size(hb: *const heartbeat_context) -> uint64_t;

    pub fn hb_get_user_tag(hb: *const heartbeat_context) -> uint64_t;

    pub fn hb_get_global_time(hb: *const heartbeat_context) -> uint64_t;
    pub fn hb_get_window_time(hb: *const heartbeat_context) -> uint64_t;
    pub fn hb_get_global_work(hb: *const heartbeat_context) -> uint64_t;
    pub fn hb_get_window_work(hb: *const heartbeat_context) -> uint64_t;

    pub fn hb_get_global_perf(hb: *const heartbeat_context) -> c_double;
    pub fn hb_get_window_perf(hb: *const heartbeat_context) -> c_double;
    pub fn hb_get_instant_perf(hb: *const heartbeat_context) -> c_double;
}
