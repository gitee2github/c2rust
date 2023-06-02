use :: libc;
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    static mut Curl_cmalloc: curl_malloc_callback;
    static mut Curl_cfree: curl_free_callback;
    static mut Curl_cstrdup: curl_strdup_callback;
}
pub type size_t = u64;
pub type curl_malloc_callback = Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type curl_free_callback = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type curl_strdup_callback = Option<unsafe extern "C" fn(*const i8) -> *mut i8>;
#[no_mangle]
pub extern "C" fn Curl_dedotdotify(mut input: *const i8) -> *mut i8 {
    let mut inlen: size_t = unsafe { strlen(input) };
    let mut clone: *mut i8 = 0 as *mut i8;
    let mut clen: size_t = inlen;
    let mut out: *mut i8 =
        (unsafe { Curl_cmalloc.expect("non-null function pointer")(inlen.wrapping_add(1 as i32 as u64)) })
            as *mut i8;
    let mut outptr: *mut i8 = 0 as *mut i8;
    let mut orgclone: *mut i8 = 0 as *mut i8;
    let mut queryp: *mut i8 = 0 as *mut i8;
    if out.is_null() {
        return 0 as *mut i8;
    }
    (unsafe { *out = 0 as i32 as i8 });
    clone = unsafe { Curl_cstrdup.expect("non-null function pointer")(input) };
    if clone.is_null() {
        (unsafe { Curl_cfree.expect("non-null function pointer")(out as *mut libc::c_void) });
        return 0 as *mut i8;
    }
    orgclone = clone;
    outptr = out;
    if (unsafe { *clone }) == 0 {
        (unsafe { Curl_cfree.expect("non-null function pointer")(out as *mut libc::c_void) });
        return clone;
    }
    queryp = unsafe { strchr(clone, '?' as i32) };
    if !queryp.is_null() {
        (unsafe { *queryp = 0 as i32 as i8 });
    }
    loop {
        if (unsafe { strncmp(b"./\0" as *const u8 as *const i8, clone, 2 as i32 as u64) }) == 0 {
            clone = unsafe { clone.offset(2 as i32 as isize) };
            clen = (clen as u64).wrapping_sub(2 as i32 as u64) as size_t as size_t;
        } else if (unsafe { strncmp(b"../\0" as *const u8 as *const i8, clone, 3 as i32 as u64) }) == 0 {
            clone = unsafe { clone.offset(3 as i32 as isize) };
            clen = (clen as u64).wrapping_sub(3 as i32 as u64) as size_t as size_t;
        } else if (unsafe { strncmp(b"/./\0" as *const u8 as *const i8, clone, 3 as i32 as u64) }) == 0 {
            clone = unsafe { clone.offset(2 as i32 as isize) };
            clen = (clen as u64).wrapping_sub(2 as i32 as u64) as size_t as size_t;
        } else if (unsafe { strcmp(b"/.\0" as *const u8 as *const i8, clone) }) == 0 {
            (unsafe { *clone.offset(1 as i32 as isize) = '/' as i32 as i8 });
            clone = unsafe { clone.offset(1) };
            clen = (clen as u64).wrapping_sub(1 as i32 as u64) as size_t as size_t;
        } else if (unsafe { strncmp(b"/../\0" as *const u8 as *const i8, clone, 4 as i32 as u64) }) == 0 {
            clone = unsafe { clone.offset(3 as i32 as isize) };
            clen = (clen as u64).wrapping_sub(3 as i32 as u64) as size_t as size_t;
            while outptr > out {
                outptr = unsafe { outptr.offset(-1) };
                if (unsafe { *outptr }) as i32 == '/' as i32 {
                    break;
                }
            }
            (unsafe { *outptr = 0 as i32 as i8 });
        } else if (unsafe { strcmp(b"/..\0" as *const u8 as *const i8, clone) }) == 0 {
            (unsafe { *clone.offset(2 as i32 as isize) = '/' as i32 as i8 });
            clone = unsafe { clone.offset(2 as i32 as isize) };
            clen = (clen as u64).wrapping_sub(2 as i32 as u64) as size_t as size_t;
            while outptr > out {
                outptr = unsafe { outptr.offset(-1) };
                if (unsafe { *outptr }) as i32 == '/' as i32 {
                    break;
                }
            }
            (unsafe { *outptr = 0 as i32 as i8 });
        } else if (unsafe { strcmp(b".\0" as *const u8 as *const i8, clone) }) == 0
            || (unsafe { strcmp(b"..\0" as *const u8 as *const i8, clone) }) == 0
        {
            (unsafe { *clone = 0 as i32 as i8 });
            (unsafe { *out = 0 as i32 as i8 });
        } else {
            loop {
                let fresh0 = clone;
                clone = unsafe { clone.offset(1) };
                let fresh1 = outptr;
                outptr = unsafe { outptr.offset(1) };
                (unsafe { *fresh1 = *fresh0 });
                clen = clen.wrapping_sub(1);
                if !((unsafe { *clone }) as i32 != 0 && (unsafe { *clone }) as i32 != '/' as i32) {
                    break;
                }
            }
            (unsafe { *outptr = 0 as i32 as i8 });
        }
        if !((unsafe { *clone }) != 0) {
            break;
        }
    }
    if !queryp.is_null() {
        let mut qlen: size_t = 0;
        let mut oindex: size_t = (unsafe { queryp.offset_from(orgclone) }) as i64 as size_t;
        qlen = unsafe { strlen(&*input.offset(oindex as isize)) };
        (unsafe { memcpy(
            outptr as *mut libc::c_void,
            &*input.offset(oindex as isize) as *const i8 as *const libc::c_void,
            qlen.wrapping_add(1 as i32 as u64),
        ) });
    }
    (unsafe { Curl_cfree.expect("non-null function pointer")(orgclone as *mut libc::c_void) });
    return out;
}
