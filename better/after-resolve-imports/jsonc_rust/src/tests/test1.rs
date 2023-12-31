use ::libc;
extern "C" {
    
    
    
    
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::debug::mc_set_debug;
pub use crate::src::json_object::json_object_array_add;
pub use crate::src::json_object::json_object_array_bsearch;
pub use crate::src::json_object::json_object_array_del_idx;
pub use crate::src::json_object::json_object_array_get_idx;
pub use crate::src::json_object::json_object_array_length;
pub use crate::src::json_object::json_object_array_put_idx;
pub use crate::src::json_object::json_object_array_sort;
pub use crate::src::json_object::json_object_get;
pub use crate::src::json_object::json_object_get_int;
pub use crate::src::json_object::json_object_get_object;
pub use crate::src::json_object::json_object_get_string;
pub use crate::src::json_object::json_object_new_array;
pub use crate::src::json_object::json_object_new_array_ext;
pub use crate::src::json_object::json_object_new_boolean;
pub use crate::src::json_object::json_object_new_int;
pub use crate::src::json_object::json_object_new_null;
pub use crate::src::json_object::json_object_new_object;
pub use crate::src::json_object::json_object_new_string;
pub use crate::src::json_object::json_object_object_add;
pub use crate::src::json_object::json_object_object_del;
pub use crate::src::json_object::json_object_put;
pub use crate::src::json_object::json_object_to_json_string_ext;
pub use crate::src::json_object::json_object_to_json_string_length;
pub use crate::src::tests::parse_flags::parse_flags;
pub use crate::src::json_object::json_object;
pub use crate::src::debug::_IO_wide_data;
pub use crate::src::json_object::_IO_codecvt;
pub use crate::src::json_visit::_IO_marker;
pub type size_t = crate::src::apps::json_parse::size_t;
pub type __int32_t = crate::src::json_object::__int32_t;
pub type __off_t = crate::src::apps::json_parse::__off_t;
pub type __off64_t = crate::src::apps::json_parse::__off64_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::apps::json_parse::_IO_FILE;
pub type _IO_lock_t = crate::src::apps::json_parse::_IO_lock_t;
pub type FILE = crate::src::apps::json_parse::FILE;
pub type int32_t = crate::src::json_object::int32_t;
pub type uintptr_t = crate::src::json_object::uintptr_t;
// #[derive(Copy, Clone)]

pub type lh_entry = crate::src::json_object::lh_entry;
pub type json_bool = crate::src::json_object::json_bool;
// #[derive(Copy, Clone)]

pub type lh_table = crate::src::json_object::lh_table;
pub type lh_equal_fn = crate::src::json_object::lh_equal_fn;
pub type lh_hash_fn = crate::src::json_object::lh_hash_fn;
pub type lh_entry_free_fn = crate::src::json_object::lh_entry_free_fn;
#[inline]
unsafe extern "C" fn lh_table_head(mut t: *const lh_table) -> *mut lh_entry {
    return (*t).head;
}
#[inline]
unsafe extern "C" fn lh_entry_k(mut e: *const lh_entry) -> *mut libc::c_void {
    return (*e).k as uintptr_t as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn lh_entry_v(mut e: *const lh_entry) -> *mut libc::c_void {
    return (*e).v as uintptr_t as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn lh_entry_next(mut e: *const lh_entry) -> *mut lh_entry {
    return (*e).next;
}
unsafe extern "C" fn sort_fn(
    mut j1: *const libc::c_void,
    mut j2: *const libc::c_void,
) -> libc::c_int {
    let mut jso1: *const *mut json_object = 0 as *const *mut json_object;
    let mut jso2: *const *mut json_object = 0 as *const *mut json_object;
    let mut i1: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    jso1 = j1 as *const *mut json_object;
    jso2 = j2 as *const *mut json_object;
    if (*jso1).is_null() && (*jso2).is_null() {
        return 0 as libc::c_int;
    }
    if (*jso1).is_null() {
        return -(1 as libc::c_int);
    }
    if (*jso2).is_null() {
        return 1 as libc::c_int;
    }
    i1 = json_object_get_int(*jso1);
    i2 = json_object_get_int(*jso2);
    return i1 - i2;
}
unsafe extern "C" fn to_json_string(
    mut obj: *mut json_object,
    mut flags: libc::c_int,
) -> *const libc::c_char {
    let mut length: size_t = 0;
    let mut copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: *const libc::c_char = 0 as *const libc::c_char;
    result = json_object_to_json_string_length(obj, flags, &mut length);
    copy = strdup(result);
    if copy.is_null() {
        printf(
            b"to_json_string: Allocation failed!\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        result = json_object_to_json_string_ext(obj, flags);
        if length != strlen(result) {
            printf(
                b"to_json_string: Length mismatch!\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if strcmp(copy, result) != 0 as libc::c_int {
            printf(
                b"to_json_string: Comparison Failed!\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        free(copy as *mut libc::c_void);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn make_array() -> *mut json_object {
    let mut my_array: *mut json_object = 0 as *mut json_object;
    my_array = json_object_new_array();
    json_object_array_add(my_array, json_object_new_int(1 as libc::c_int));
    json_object_array_add(my_array, json_object_new_int(2 as libc::c_int));
    json_object_array_add(my_array, json_object_new_int(3 as libc::c_int));
    json_object_array_put_idx(
        my_array,
        4 as libc::c_int as size_t,
        json_object_new_int(5 as libc::c_int),
    );
    json_object_array_put_idx(
        my_array,
        3 as libc::c_int as size_t,
        json_object_new_int(4 as libc::c_int),
    );
    json_object_array_put_idx(
        my_array,
        6 as libc::c_int as size_t,
        json_object_new_int(7 as libc::c_int),
    );
    return my_array;
}
#[no_mangle]
pub unsafe extern "C" fn test_array_del_idx() {
    let mut rc: libc::c_int = 0;
    let mut ii: size_t = 0;
    let mut orig_array_len: size_t = 0;
    let mut my_array: *mut json_object = 0 as *mut json_object;
    let mut sflags: libc::c_int = 0 as libc::c_int;
    my_array = make_array();
    orig_array_len = json_object_array_length(my_array);
    printf(b"my_array=\n\0" as *const u8 as *const libc::c_char);
    ii = 0 as libc::c_int as size_t;
    while ii < json_object_array_length(my_array) {
        let mut obj: *mut json_object = json_object_array_get_idx(my_array, ii);
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const libc::c_char,
            ii as libc::c_int,
            to_json_string(obj, sflags),
        );
        ii = ii.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_array, sflags),
    );
    ii = 0 as libc::c_int as size_t;
    while ii < orig_array_len {
        rc = json_object_array_del_idx(
            my_array,
            0 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
        );
        printf(
            b"after del_idx(0,1)=%d, my_array.to_string()=%s\n\0" as *const u8
                as *const libc::c_char,
            rc,
            to_json_string(my_array, sflags),
        );
        ii = ii.wrapping_add(1);
    }
    rc = json_object_array_del_idx(
        my_array,
        0 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
    );
    printf(
        b"after del_idx(0,1)=%d, my_array.to_string()=%s\n\0" as *const u8
            as *const libc::c_char,
        rc,
        to_json_string(my_array, sflags),
    );
    json_object_put(my_array);
    my_array = make_array();
    rc = json_object_array_del_idx(my_array, 0 as libc::c_int as size_t, orig_array_len);
    printf(
        b"after del_idx(0,%d)=%d, my_array.to_string()=%s\n\0" as *const u8
            as *const libc::c_char,
        orig_array_len as libc::c_int,
        rc,
        to_json_string(my_array, sflags),
    );
    json_object_put(my_array);
    my_array = make_array();
    rc = json_object_array_del_idx(
        my_array,
        0 as libc::c_int as size_t,
        orig_array_len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    printf(
        b"after del_idx(0,%d)=%d, my_array.to_string()=%s\n\0" as *const u8
            as *const libc::c_char,
        orig_array_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        rc,
        to_json_string(my_array, sflags),
    );
    json_object_put(my_array);
    my_array = make_array();
    rc = json_object_array_del_idx(
        my_array,
        0 as libc::c_int as size_t,
        orig_array_len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    printf(
        b"after del_idx(0,%d)=%d, my_array.to_string()=%s\n\0" as *const u8
            as *const libc::c_char,
        orig_array_len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        rc,
        to_json_string(my_array, sflags),
    );
    json_object_array_add(
        my_array,
        json_object_new_string(b"s1\0" as *const u8 as *const libc::c_char),
    );
    json_object_array_add(
        my_array,
        json_object_new_string(b"s2\0" as *const u8 as *const libc::c_char),
    );
    json_object_array_add(
        my_array,
        json_object_new_string(b"s3\0" as *const u8 as *const libc::c_char),
    );
    printf(
        b"after adding more entries, my_array.to_string()=%s\n\0" as *const u8
            as *const libc::c_char,
        to_json_string(my_array, sflags),
    );
    json_object_put(my_array);
}
#[no_mangle]
pub unsafe extern "C" fn test_array_list_expand_internal() {
    let mut rc: libc::c_int = 0;
    let mut ii: size_t = 0;
    let mut idx: size_t = 0;
    let mut my_array: *mut json_object = 0 as *mut json_object;
    let mut sflags: libc::c_int = 0 as libc::c_int;
    my_array = make_array();
    printf(b"my_array=\n\0" as *const u8 as *const libc::c_char);
    ii = 0 as libc::c_int as size_t;
    while ii < json_object_array_length(my_array) {
        let mut obj: *mut json_object = json_object_array_get_idx(my_array, ii);
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const libc::c_char,
            ii as libc::c_int,
            to_json_string(obj, sflags),
        );
        ii = ii.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_array, sflags),
    );
    rc = json_object_array_put_idx(
        my_array,
        5 as libc::c_int as size_t,
        json_object_new_int(6 as libc::c_int),
    );
    printf(b"put_idx(5,6)=%d\n\0" as *const u8 as *const libc::c_char, rc);
    idx = (32 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int) as size_t;
    rc = json_object_array_put_idx(my_array, idx, json_object_new_int(0 as libc::c_int));
    printf(
        b"put_idx(%d,0)=%d\n\0" as *const u8 as *const libc::c_char,
        idx as libc::c_int,
        rc,
    );
    idx = (32 as libc::c_int * 2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
        as size_t;
    rc = json_object_array_put_idx(my_array, idx, json_object_new_int(0 as libc::c_int));
    printf(
        b"put_idx(%d,0)=%d\n\0" as *const u8 as *const libc::c_char,
        idx as libc::c_int,
        rc,
    );
    idx = 18446744073709551615 as libc::c_ulong;
    let mut tmp: *mut json_object = json_object_new_int(10 as libc::c_int);
    rc = json_object_array_put_idx(my_array, idx, tmp);
    printf(b"put_idx(SIZE_T_MAX,0)=%d\n\0" as *const u8 as *const libc::c_char, rc);
    if rc == -(1 as libc::c_int) {
        json_object_put(tmp);
    }
    json_object_put(my_array);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut my_string: *mut json_object = 0 as *mut json_object;
    let mut my_int: *mut json_object = 0 as *mut json_object;
    let mut my_null: *mut json_object = 0 as *mut json_object;
    let mut my_object: *mut json_object = 0 as *mut json_object;
    let mut my_array: *mut json_object = 0 as *mut json_object;
    let mut i: size_t = 0;
    let mut sflags: libc::c_int = 0 as libc::c_int;
    sflags = parse_flags(argc, argv);
    my_string = json_object_new_string(b"\t\0" as *const u8 as *const libc::c_char);
    printf(
        b"my_string=%s\n\0" as *const u8 as *const libc::c_char,
        json_object_get_string(my_string),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_string, sflags),
    );
    json_object_put(my_string);
    my_string = json_object_new_string(b"\\\0" as *const u8 as *const libc::c_char);
    printf(
        b"my_string=%s\n\0" as *const u8 as *const libc::c_char,
        json_object_get_string(my_string),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_string, sflags),
    );
    json_object_put(my_string);
    my_string = json_object_new_string(b"/\0" as *const u8 as *const libc::c_char);
    printf(
        b"my_string=%s\n\0" as *const u8 as *const libc::c_char,
        json_object_get_string(my_string),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_string, sflags),
    );
    printf(
        b"my_string.to_string(NOSLASHESCAPE)=%s\n\0" as *const u8 as *const libc::c_char,
        json_object_to_json_string_ext(my_string, (1 as libc::c_int) << 4 as libc::c_int),
    );
    json_object_put(my_string);
    my_string = json_object_new_string(
        b"/foo/bar/baz\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"my_string=%s\n\0" as *const u8 as *const libc::c_char,
        json_object_get_string(my_string),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_string, sflags),
    );
    printf(
        b"my_string.to_string(NOSLASHESCAPE)=%s\n\0" as *const u8 as *const libc::c_char,
        json_object_to_json_string_ext(my_string, (1 as libc::c_int) << 4 as libc::c_int),
    );
    json_object_put(my_string);
    my_string = json_object_new_string(b"foo\0" as *const u8 as *const libc::c_char);
    printf(
        b"my_string=%s\n\0" as *const u8 as *const libc::c_char,
        json_object_get_string(my_string),
    );
    printf(
        b"my_string.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_string, sflags),
    );
    my_int = json_object_new_int(9 as libc::c_int);
    printf(
        b"my_int=%d\n\0" as *const u8 as *const libc::c_char,
        json_object_get_int(my_int),
    );
    printf(
        b"my_int.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_int, sflags),
    );
    my_null = json_object_new_null();
    printf(
        b"my_null.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_null, sflags),
    );
    my_array = json_object_new_array();
    json_object_array_add(my_array, json_object_new_int(1 as libc::c_int));
    json_object_array_add(my_array, json_object_new_int(2 as libc::c_int));
    json_object_array_add(my_array, json_object_new_int(3 as libc::c_int));
    json_object_array_put_idx(
        my_array,
        4 as libc::c_int as size_t,
        json_object_new_int(5 as libc::c_int),
    );
    printf(b"my_array=\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i < json_object_array_length(my_array) {
        let mut obj: *mut json_object = json_object_array_get_idx(my_array, i);
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const libc::c_char,
            i as libc::c_int,
            to_json_string(obj, sflags),
        );
        i = i.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_array, sflags),
    );
    json_object_put(my_array);
    test_array_del_idx();
    test_array_list_expand_internal();
    my_array = json_object_new_array_ext(5 as libc::c_int);
    json_object_array_add(my_array, json_object_new_int(3 as libc::c_int));
    json_object_array_add(my_array, json_object_new_int(1 as libc::c_int));
    json_object_array_add(my_array, json_object_new_int(2 as libc::c_int));
    json_object_array_put_idx(
        my_array,
        4 as libc::c_int as size_t,
        json_object_new_int(0 as libc::c_int),
    );
    printf(b"my_array=\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i < json_object_array_length(my_array) {
        let mut obj_0: *mut json_object = json_object_array_get_idx(my_array, i);
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const libc::c_char,
            i as libc::c_int,
            to_json_string(obj_0, sflags),
        );
        i = i.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_array, sflags),
    );
    json_object_array_sort(
        my_array,
        Some(
            sort_fn
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    printf(b"my_array=\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i < json_object_array_length(my_array) {
        let mut obj_1: *mut json_object = json_object_array_get_idx(my_array, i);
        printf(
            b"\t[%d]=%s\n\0" as *const u8 as *const libc::c_char,
            i as libc::c_int,
            to_json_string(obj_1, sflags),
        );
        i = i.wrapping_add(1);
    }
    printf(
        b"my_array.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_array, sflags),
    );
    let mut one: *mut json_object = json_object_new_int(1 as libc::c_int);
    let mut result: *mut json_object = json_object_array_bsearch(
        one,
        my_array,
        Some(
            sort_fn
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    printf(
        b"find json_object(1) in my_array successfully: %s\n\0" as *const u8
            as *const libc::c_char,
        to_json_string(result, sflags),
    );
    json_object_put(one);
    my_object = json_object_new_object();
    let mut rc: libc::c_int = json_object_object_add(
        my_object,
        b"abc\0" as *const u8 as *const libc::c_char,
        my_object,
    );
    if rc != -(1 as libc::c_int) {
        printf(
            b"ERROR: able to successfully add object to itself!\n\0" as *const u8
                as *const libc::c_char,
        );
        fflush(stdout);
    }
    json_object_object_add(
        my_object,
        b"abc\0" as *const u8 as *const libc::c_char,
        json_object_new_int(12 as libc::c_int),
    );
    json_object_object_add(
        my_object,
        b"foo\0" as *const u8 as *const libc::c_char,
        json_object_new_string(b"bar\0" as *const u8 as *const libc::c_char),
    );
    json_object_object_add(
        my_object,
        b"bool0\0" as *const u8 as *const libc::c_char,
        json_object_new_boolean(0 as libc::c_int),
    );
    json_object_object_add(
        my_object,
        b"bool1\0" as *const u8 as *const libc::c_char,
        json_object_new_boolean(1 as libc::c_int),
    );
    json_object_object_add(
        my_object,
        b"baz\0" as *const u8 as *const libc::c_char,
        json_object_new_string(b"bang\0" as *const u8 as *const libc::c_char),
    );
    let mut baz_obj: *mut json_object = json_object_new_string(
        b"fark\0" as *const u8 as *const libc::c_char,
    );
    json_object_get(baz_obj);
    json_object_object_add(
        my_object,
        b"baz\0" as *const u8 as *const libc::c_char,
        baz_obj,
    );
    json_object_object_del(my_object, b"baz\0" as *const u8 as *const libc::c_char);
    printf(
        b"baz_obj.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(baz_obj, sflags),
    );
    json_object_put(baz_obj);
    printf(b"my_object=\n\0" as *const u8 as *const libc::c_char);
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut json_object = 0 as *mut json_object;
    let mut entrykey: *mut lh_entry = lh_table_head(json_object_get_object(my_object));
    let mut entry_nextkey: *mut lh_entry = 0 as *mut lh_entry;
    while !({
        if !entrykey.is_null() {
            key = lh_entry_k(entrykey) as *mut libc::c_char;
            val = lh_entry_v(entrykey) as *mut json_object;
            entry_nextkey = lh_entry_next(entrykey);
        }
        entrykey
    })
        .is_null()
    {
        printf(
            b"\t%s: %s\n\0" as *const u8 as *const libc::c_char,
            key,
            to_json_string(val, sflags),
        );
        entrykey = entry_nextkey;
    }
    let mut empty_array: *mut json_object = json_object_new_array();
    let mut empty_obj: *mut json_object = json_object_new_object();
    json_object_object_add(
        my_object,
        b"empty_array\0" as *const u8 as *const libc::c_char,
        empty_array,
    );
    json_object_object_add(
        my_object,
        b"empty_obj\0" as *const u8 as *const libc::c_char,
        empty_obj,
    );
    printf(
        b"my_object.to_string()=%s\n\0" as *const u8 as *const libc::c_char,
        to_json_string(my_object, sflags),
    );
    json_object_put(my_array);
    my_array = json_object_new_array_ext(
        -(2147483647 as libc::c_int) - 1 as libc::c_int + 1 as libc::c_int,
    );
    if !my_array.is_null() {
        printf(
            b"ERROR: able to allocate an array of negative size!\n\0" as *const u8
                as *const libc::c_char,
        );
        fflush(stdout);
        json_object_put(my_array);
        my_array = 0 as *mut json_object;
    }
    json_object_put(my_string);
    json_object_put(my_int);
    json_object_put(my_null);
    json_object_put(my_object);
    json_object_put(my_array);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
