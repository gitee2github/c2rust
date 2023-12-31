use :: libc;
extern "C" {
    fn json_object_get_object(obj: *const json_object) -> *mut lh_table;
}
pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type uintptr_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct printbuf {
    pub buf: *mut i8,
    pub bpos: i32,
    pub size: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_entry {
    pub k: *const libc::c_void,
    pub k_is_constant: i32,
    pub v: *const libc::c_void,
    pub next: *mut lh_entry,
    pub prev: *mut lh_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object {
    pub o_type: json_type,
    pub _ref_count: uint32_t,
    pub _to_json_string: Option<json_object_to_json_string_fn>,
    pub _pb: *mut printbuf,
    pub _user_delete: Option<json_object_delete_fn>,
    pub _userdata: *mut libc::c_void,
}
pub type json_object_delete_fn = unsafe extern "C" fn(*mut json_object, *mut libc::c_void) -> ();
pub type json_object_to_json_string_fn =
    unsafe extern "C" fn(*mut json_object, *mut printbuf, i32, i32) -> i32;
pub type json_type = u32;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
pub type json_bool = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lh_table {
    pub size: i32,
    pub count: i32,
    pub head: *mut lh_entry,
    pub tail: *mut lh_entry,
    pub table: *mut lh_entry,
    pub free_fn: Option<lh_entry_free_fn>,
    pub hash_fn: Option<lh_hash_fn>,
    pub equal_fn: Option<lh_equal_fn>,
}
pub type lh_equal_fn = unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32;
pub type lh_hash_fn = unsafe extern "C" fn(*const libc::c_void) -> u64;
pub type lh_entry_free_fn = unsafe extern "C" fn(*mut lh_entry) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct json_object_iterator {
    pub opaque_: *const libc::c_void,
}
#[inline]
extern "C" fn lh_table_head(mut t: *const lh_table) -> *mut lh_entry {
    return unsafe { (*t).head };
}
#[inline]
extern "C" fn lh_entry_next(mut e: *const lh_entry) -> *mut lh_entry {
    return unsafe { (*e).next };
}
#[inline]
extern "C" fn lh_entry_v(mut e: *const lh_entry) -> *mut libc::c_void {
    return (unsafe { (*e).v }) as uintptr_t as *mut libc::c_void;
}
static mut kObjectEndIterValue: *const libc::c_void = 0 as *const libc::c_void;
#[no_mangle]
pub extern "C" fn json_object_iter_begin(mut obj: *mut json_object) -> json_object_iterator {
    let mut iter: json_object_iterator = json_object_iterator {
        opaque_: 0 as *const libc::c_void,
    };
    let mut pTable: *mut lh_table = 0 as *mut lh_table;
    pTable = unsafe { json_object_get_object(obj) };
    iter.opaque_ = lh_table_head(pTable) as *const libc::c_void;
    return iter;
}
#[no_mangle]
pub extern "C" fn json_object_iter_end(mut _obj: *const json_object) -> json_object_iterator {
    let mut iter: json_object_iterator = json_object_iterator {
        opaque_: 0 as *const libc::c_void,
    };
    iter.opaque_ = unsafe { kObjectEndIterValue };
    return iter;
}
#[no_mangle]
pub extern "C" fn json_object_iter_next(mut iter: *mut json_object_iterator) {
    let fresh0 = unsafe { &mut ((*iter).opaque_) };
    *fresh0 = lh_entry_next((unsafe { (*iter).opaque_ }) as *const lh_entry) as *const libc::c_void;
}
#[no_mangle]
pub extern "C" fn json_object_iter_peek_name(mut iter: *const json_object_iterator) -> *const i8 {
    return (unsafe { (*((*iter).opaque_ as *const lh_entry)).k }) as *const i8;
}
#[no_mangle]
pub extern "C" fn json_object_iter_peek_value(
    mut iter: *const json_object_iterator,
) -> *mut json_object {
    return lh_entry_v((unsafe { (*iter).opaque_ }) as *const lh_entry) as *mut json_object;
}
#[no_mangle]
pub extern "C" fn json_object_iter_equal(
    mut iter1: *const json_object_iterator,
    mut iter2: *const json_object_iterator,
) -> json_bool {
    return ((unsafe { (*iter1).opaque_ }) == (unsafe { (*iter2).opaque_ })) as i32;
}
#[no_mangle]
pub extern "C" fn json_object_iter_init_default() -> json_object_iterator {
    let mut iter: json_object_iterator = json_object_iterator {
        opaque_: 0 as *const libc::c_void,
    };
    iter.opaque_ = 0 as *const libc::c_void;
    return iter;
}
