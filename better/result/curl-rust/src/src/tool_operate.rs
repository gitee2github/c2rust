use :: libc;
extern "C" {
    static mut stdout: *mut crate::src::lib::http2::_IO_FILE;
    static mut stderr: *mut crate::src::lib::http2::_IO_FILE;
    fn fclose(__stream: *mut crate::src::lib::http2::_IO_FILE) -> i32;
    fn fflush(__stream: *mut crate::src::lib::http2::_IO_FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut crate::src::lib::http2::_IO_FILE;
    fn fputs(__s: *const i8, __stream: *mut crate::src::lib::http2::_IO_FILE) -> i32;
    fn fseek(__stream: *mut crate::src::lib::http2::_IO_FILE, __off: i64, __whence: i32) -> i32;
    fn fileno(__stream: *mut crate::src::lib::http2::_IO_FILE) -> i32;
    fn time(__timer: *mut i64) -> i64;
    fn calloc(_: u64, _: u64) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn __xstat(
        __ver: i32,
        __filename: *const i8,
        __stat_buf: *mut crate::src::lib::file::stat,
    ) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut crate::src::lib::file::stat) -> i32;
    fn close(__fd: i32) -> i32;
    fn isatty(__fd: i32) -> i32;
    fn ftruncate(__fd: i32, __length: i64) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn curlx_dyn_init(s: *mut crate::src::lib::http2::dynbuf, toobig: u64);
    fn curlx_dyn_free(s: *mut crate::src::lib::http2::dynbuf);
    fn curlx_dyn_addf(s: *mut crate::src::lib::http2::dynbuf, fmt: *const i8, _: ...) -> u32;
    fn curlx_dyn_ptr(s: *const crate::src::lib::http2::dynbuf) -> *mut i8;
}
pub use crate::src::lib::asyn_thread::Curl_share;
pub use crate::src::lib::easy::curl_easy_cleanup;
pub use crate::src::lib::easy::curl_easy_getinfo;
pub use crate::src::lib::easy::curl_easy_init;
pub use crate::src::lib::easy::curl_easy_perform;
pub use crate::src::lib::escape::curl_free;
pub use crate::src::lib::getenv::curl_getenv;
pub use crate::src::lib::http2::curl_mime;
pub use crate::src::lib::http2::Curl_easy;
pub use crate::src::lib::http2::Curl_multi;
pub use crate::src::lib::mime::curl_mime_free;
pub use crate::src::lib::mprintf::curl_maprintf;
pub use crate::src::lib::mprintf::curl_mfprintf;
pub use crate::src::lib::mprintf::curl_mprintf;
pub use crate::src::lib::mprintf::curl_msnprintf;
pub use crate::src::lib::mqtt::_IO_codecvt;
pub use crate::src::lib::multi::curl_multi_add_handle;
pub use crate::src::lib::multi::curl_multi_cleanup;
pub use crate::src::lib::multi::curl_multi_info_read;
pub use crate::src::lib::multi::curl_multi_init;
pub use crate::src::lib::multi::curl_multi_perform;
pub use crate::src::lib::multi::curl_multi_poll;
pub use crate::src::lib::multi::curl_multi_remove_handle;
pub use crate::src::lib::nonblock::curlx_nonblock;
pub use crate::src::lib::setopt::curl_easy_setopt;
pub use crate::src::lib::share::curl_share_cleanup;
pub use crate::src::lib::share::curl_share_init;
pub use crate::src::lib::share::curl_share_setopt;
pub use crate::src::lib::strcase::curl_strequal;
pub use crate::src::lib::strcase::curl_strnequal;
pub use crate::src::lib::strerror::curl_easy_strerror;
pub use crate::src::src::tool_cb_dbg::tool_debug_cb;
pub use crate::src::src::tool_cb_hdr::tool_header_cb;
pub use crate::src::src::tool_cb_prg::progressbarinit;
pub use crate::src::src::tool_cb_prg::tool_progress_cb;
pub use crate::src::src::tool_cb_rea::_IO_wide_data;
pub use crate::src::src::tool_cb_rea::tool_read_cb;
pub use crate::src::src::tool_cb_rea::tool_readbusy_cb;
pub use crate::src::src::tool_cb_see::tool_seek_cb;
pub use crate::src::src::tool_cb_wrt::tool_create_output_file;
pub use crate::src::src::tool_cb_wrt::tool_write_cb;
pub use crate::src::src::tool_dirhie::create_dir_hierarchy;
pub use crate::src::src::tool_easysrc::dumpeasysrc;
pub use crate::src::src::tool_easysrc::easysrc_cleanup;
pub use crate::src::src::tool_easysrc::easysrc_init;
pub use crate::src::src::tool_easysrc::easysrc_perform;
pub use crate::src::src::tool_filetime::setfiletime;
pub use crate::src::src::tool_formparse::tool2curlmime;
pub use crate::src::src::tool_getparam::parse_args;
pub use crate::src::src::tool_help::tool_help;
pub use crate::src::src::tool_help::tool_list_engines;
pub use crate::src::src::tool_help::tool_version_info;
pub use crate::src::src::tool_helpers::customrequest_helper;
pub use crate::src::src::tool_helpers::SetHTTPrequest;
pub use crate::src::src::tool_homedir::homedir;
pub use crate::src::src::tool_hugehelp::hugehelp;
pub use crate::src::src::tool_libinfo::built_in_protos;
pub use crate::src::src::tool_libinfo::curlinfo;
pub use crate::src::src::tool_msgs::_IO_marker;
pub use crate::src::src::tool_msgs::errorf;
pub use crate::src::src::tool_msgs::helpf;
pub use crate::src::src::tool_msgs::warnf;
pub use crate::src::src::tool_operhlp::add_file_name_to_url;
pub use crate::src::src::tool_operhlp::clean_getout;
pub use crate::src::src::tool_operhlp::get_url_file_name;
pub use crate::src::src::tool_operhlp::output_expected;
pub use crate::src::src::tool_operhlp::stdin_upload;
pub use crate::src::src::tool_paramhlp::add2list;
pub use crate::src::src::tool_paramhlp::file2string;
pub use crate::src::src::tool_paramhlp::get_args;
pub use crate::src::src::tool_parsecfg::parseconfig;
pub use crate::src::src::tool_progress::all_xfers;
pub use crate::src::src::tool_progress::progress_finalize;
pub use crate::src::src::tool_progress::progress_meter;
pub use crate::src::src::tool_progress::xferinfo_cb;
pub use crate::src::src::tool_setopt::setopt_nv_CURLAUTH;
pub use crate::src::src::tool_setopt::setopt_nv_CURLFTPSSL_CCC;
pub use crate::src::src::tool_setopt::setopt_nv_CURLPROTO;
pub use crate::src::src::tool_setopt::setopt_nv_CURLPROXY;
pub use crate::src::src::tool_setopt::setopt_nv_CURLSSLOPT;
pub use crate::src::src::tool_setopt::setopt_nv_CURLUSESSL;
pub use crate::src::src::tool_setopt::setopt_nv_CURL_HTTP_VERSION;
pub use crate::src::src::tool_setopt::setopt_nv_CURL_NETRC;
pub use crate::src::src::tool_setopt::setopt_nv_CURL_SSLVERSION;
pub use crate::src::src::tool_setopt::setopt_nv_CURL_TIMECOND;
pub use crate::src::src::tool_setopt::tool_setopt;
pub use crate::src::src::tool_setopt::tool_setopt_bitmask;
pub use crate::src::src::tool_setopt::tool_setopt_enum;
pub use crate::src::src::tool_setopt::tool_setopt_flags;
pub use crate::src::src::tool_setopt::tool_setopt_mimepost;
pub use crate::src::src::tool_setopt::tool_setopt_skip;
pub use crate::src::src::tool_setopt::tool_setopt_slist;
pub use crate::src::src::tool_sleep::tool_go_sleep;
pub use crate::src::src::tool_urlglob::glob_cleanup;
pub use crate::src::src::tool_urlglob::glob_match_url;
pub use crate::src::src::tool_urlglob::glob_next_url;
pub use crate::src::src::tool_urlglob::glob_url;
pub use crate::src::src::tool_util::tvdiff;
pub use crate::src::src::tool_util::tvnow;
pub use crate::src::src::tool_writeout::ourWriteOut;
pub use crate::src::src::tool_xattr::fwrite_xattr;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type time_t = i64;
pub type size_t = u64;
pub type timeval = crate::src::lib::openldap::timeval;
pub type timespec = crate::src::lib::file::timespec;
pub type curl_off_t = i64;
pub type _IO_FILE = crate::src::lib::http2::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::lib::http2::_IO_FILE;
pub type CURL = crate::src::lib::http2::Curl_easy;
pub type CURLSH = crate::src::lib::asyn_thread::Curl_share;
pub type curl_socket_t = i32;
pub type curl_sslbackend = u32;
pub const CURLSSLBACKEND_RUSTLS: curl_sslbackend = 14;
pub const CURLSSLBACKEND_BEARSSL: curl_sslbackend = 13;
pub const CURLSSLBACKEND_MESALINK: curl_sslbackend = 12;
pub const CURLSSLBACKEND_MBEDTLS: curl_sslbackend = 11;
pub const CURLSSLBACKEND_AXTLS: curl_sslbackend = 10;
pub const CURLSSLBACKEND_SECURETRANSPORT: curl_sslbackend = 9;
pub const CURLSSLBACKEND_SCHANNEL: curl_sslbackend = 8;
pub const CURLSSLBACKEND_WOLFSSL: curl_sslbackend = 7;
pub const CURLSSLBACKEND_POLARSSL: curl_sslbackend = 6;
pub const CURLSSLBACKEND_GSKIT: curl_sslbackend = 5;
pub const CURLSSLBACKEND_OBSOLETE4: curl_sslbackend = 4;
pub const CURLSSLBACKEND_NSS: curl_sslbackend = 3;
pub const CURLSSLBACKEND_GNUTLS: curl_sslbackend = 2;
pub const CURLSSLBACKEND_OPENSSL: curl_sslbackend = 1;
pub const CURLSSLBACKEND_NONE: curl_sslbackend = 0;
pub type curl_slist = crate::src::lib::http2::curl_slist;
pub type curl_infotype = u32;
pub const CURLINFO_END: curl_infotype = 7;
pub const CURLINFO_SSL_DATA_OUT: curl_infotype = 6;
pub const CURLINFO_SSL_DATA_IN: curl_infotype = 5;
pub const CURLINFO_DATA_OUT: curl_infotype = 4;
pub const CURLINFO_DATA_IN: curl_infotype = 3;
pub const CURLINFO_HEADER_OUT: curl_infotype = 2;
pub const CURLINFO_HEADER_IN: curl_infotype = 1;
pub const CURLINFO_TEXT: curl_infotype = 0;
pub type CURLcode = u32;
pub const CURL_LAST: CURLcode = 99;
pub const CURLE_SSL_CLIENTCERT: CURLcode = 98;
pub const CURLE_PROXY: CURLcode = 97;
pub const CURLE_QUIC_CONNECT_ERROR: CURLcode = 96;
pub const CURLE_HTTP3: CURLcode = 95;
pub const CURLE_AUTH_ERROR: CURLcode = 94;
pub const CURLE_RECURSIVE_API_CALL: CURLcode = 93;
pub const CURLE_HTTP2_STREAM: CURLcode = 92;
pub const CURLE_SSL_INVALIDCERTSTATUS: CURLcode = 91;
pub const CURLE_SSL_PINNEDPUBKEYNOTMATCH: CURLcode = 90;
pub const CURLE_NO_CONNECTION_AVAILABLE: CURLcode = 89;
pub const CURLE_CHUNK_FAILED: CURLcode = 88;
pub const CURLE_FTP_BAD_FILE_LIST: CURLcode = 87;
pub const CURLE_RTSP_SESSION_ERROR: CURLcode = 86;
pub const CURLE_RTSP_CSEQ_ERROR: CURLcode = 85;
pub const CURLE_FTP_PRET_FAILED: CURLcode = 84;
pub const CURLE_SSL_ISSUER_ERROR: CURLcode = 83;
pub const CURLE_SSL_CRL_BADFILE: CURLcode = 82;
pub const CURLE_AGAIN: CURLcode = 81;
pub const CURLE_SSL_SHUTDOWN_FAILED: CURLcode = 80;
pub const CURLE_SSH: CURLcode = 79;
pub const CURLE_REMOTE_FILE_NOT_FOUND: CURLcode = 78;
pub const CURLE_SSL_CACERT_BADFILE: CURLcode = 77;
pub const CURLE_CONV_REQD: CURLcode = 76;
pub const CURLE_CONV_FAILED: CURLcode = 75;
pub const CURLE_TFTP_NOSUCHUSER: CURLcode = 74;
pub const CURLE_REMOTE_FILE_EXISTS: CURLcode = 73;
pub const CURLE_TFTP_UNKNOWNID: CURLcode = 72;
pub const CURLE_TFTP_ILLEGAL: CURLcode = 71;
pub const CURLE_REMOTE_DISK_FULL: CURLcode = 70;
pub const CURLE_TFTP_PERM: CURLcode = 69;
pub const CURLE_TFTP_NOTFOUND: CURLcode = 68;
pub const CURLE_LOGIN_DENIED: CURLcode = 67;
pub const CURLE_SSL_ENGINE_INITFAILED: CURLcode = 66;
pub const CURLE_SEND_FAIL_REWIND: CURLcode = 65;
pub const CURLE_USE_SSL_FAILED: CURLcode = 64;
pub const CURLE_FILESIZE_EXCEEDED: CURLcode = 63;
pub const CURLE_LDAP_INVALID_URL: CURLcode = 62;
pub const CURLE_BAD_CONTENT_ENCODING: CURLcode = 61;
pub const CURLE_PEER_FAILED_VERIFICATION: CURLcode = 60;
pub const CURLE_SSL_CIPHER: CURLcode = 59;
pub const CURLE_SSL_CERTPROBLEM: CURLcode = 58;
pub const CURLE_OBSOLETE57: CURLcode = 57;
pub const CURLE_RECV_ERROR: CURLcode = 56;
pub const CURLE_SEND_ERROR: CURLcode = 55;
pub const CURLE_SSL_ENGINE_SETFAILED: CURLcode = 54;
pub const CURLE_SSL_ENGINE_NOTFOUND: CURLcode = 53;
pub const CURLE_GOT_NOTHING: CURLcode = 52;
pub const CURLE_OBSOLETE51: CURLcode = 51;
pub const CURLE_OBSOLETE50: CURLcode = 50;
pub const CURLE_SETOPT_OPTION_SYNTAX: CURLcode = 49;
pub const CURLE_UNKNOWN_OPTION: CURLcode = 48;
pub const CURLE_TOO_MANY_REDIRECTS: CURLcode = 47;
pub const CURLE_OBSOLETE46: CURLcode = 46;
pub const CURLE_INTERFACE_FAILED: CURLcode = 45;
pub const CURLE_OBSOLETE44: CURLcode = 44;
pub const CURLE_BAD_FUNCTION_ARGUMENT: CURLcode = 43;
pub const CURLE_ABORTED_BY_CALLBACK: CURLcode = 42;
pub const CURLE_FUNCTION_NOT_FOUND: CURLcode = 41;
pub const CURLE_OBSOLETE40: CURLcode = 40;
pub const CURLE_LDAP_SEARCH_FAILED: CURLcode = 39;
pub const CURLE_LDAP_CANNOT_BIND: CURLcode = 38;
pub const CURLE_FILE_COULDNT_READ_FILE: CURLcode = 37;
pub const CURLE_BAD_DOWNLOAD_RESUME: CURLcode = 36;
pub const CURLE_SSL_CONNECT_ERROR: CURLcode = 35;
pub const CURLE_HTTP_POST_ERROR: CURLcode = 34;
pub const CURLE_RANGE_ERROR: CURLcode = 33;
pub const CURLE_OBSOLETE32: CURLcode = 32;
pub const CURLE_FTP_COULDNT_USE_REST: CURLcode = 31;
pub const CURLE_FTP_PORT_FAILED: CURLcode = 30;
pub const CURLE_OBSOLETE29: CURLcode = 29;
pub const CURLE_OPERATION_TIMEDOUT: CURLcode = 28;
pub const CURLE_OUT_OF_MEMORY: CURLcode = 27;
pub const CURLE_READ_ERROR: CURLcode = 26;
pub const CURLE_UPLOAD_FAILED: CURLcode = 25;
pub const CURLE_OBSOLETE24: CURLcode = 24;
pub const CURLE_WRITE_ERROR: CURLcode = 23;
pub const CURLE_HTTP_RETURNED_ERROR: CURLcode = 22;
pub const CURLE_QUOTE_ERROR: CURLcode = 21;
pub const CURLE_OBSOLETE20: CURLcode = 20;
pub const CURLE_FTP_COULDNT_RETR_FILE: CURLcode = 19;
pub const CURLE_PARTIAL_FILE: CURLcode = 18;
pub const CURLE_FTP_COULDNT_SET_TYPE: CURLcode = 17;
pub const CURLE_HTTP2: CURLcode = 16;
pub const CURLE_FTP_CANT_GET_HOST: CURLcode = 15;
pub const CURLE_FTP_WEIRD_227_FORMAT: CURLcode = 14;
pub const CURLE_FTP_WEIRD_PASV_REPLY: CURLcode = 13;
pub const CURLE_FTP_ACCEPT_TIMEOUT: CURLcode = 12;
pub const CURLE_FTP_WEIRD_PASS_REPLY: CURLcode = 11;
pub const CURLE_FTP_ACCEPT_FAILED: CURLcode = 10;
pub const CURLE_REMOTE_ACCESS_DENIED: CURLcode = 9;
pub const CURLE_WEIRD_SERVER_REPLY: CURLcode = 8;
pub const CURLE_COULDNT_CONNECT: CURLcode = 7;
pub const CURLE_COULDNT_RESOLVE_HOST: CURLcode = 6;
pub const CURLE_COULDNT_RESOLVE_PROXY: CURLcode = 5;
pub const CURLE_NOT_BUILT_IN: CURLcode = 4;
pub const CURLE_URL_MALFORMAT: CURLcode = 3;
pub const CURLE_FAILED_INIT: CURLcode = 2;
pub const CURLE_UNSUPPORTED_PROTOCOL: CURLcode = 1;
pub const CURLE_OK: CURLcode = 0;
pub type C2RustUnnamed = u32;
pub const CURLUSESSL_LAST: C2RustUnnamed = 4;
pub const CURLUSESSL_ALL: C2RustUnnamed = 3;
pub const CURLUSESSL_CONTROL: C2RustUnnamed = 2;
pub const CURLUSESSL_TRY: C2RustUnnamed = 1;
pub const CURLUSESSL_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = u32;
pub const CURLFTP_CREATE_DIR_LAST: C2RustUnnamed_0 = 3;
pub const CURLFTP_CREATE_DIR_RETRY: C2RustUnnamed_0 = 2;
pub const CURLFTP_CREATE_DIR: C2RustUnnamed_0 = 1;
pub const CURLFTP_CREATE_DIR_NONE: C2RustUnnamed_0 = 0;
pub type CURLoption = u32;
pub const CURLOPT_LASTENTRY: CURLoption = 40311;
pub const CURLOPT_PROXY_CAINFO_BLOB: CURLoption = 40310;
pub const CURLOPT_CAINFO_BLOB: CURLoption = 40309;
pub const CURLOPT_DOH_SSL_VERIFYSTATUS: CURLoption = 308;
pub const CURLOPT_DOH_SSL_VERIFYHOST: CURLoption = 307;
pub const CURLOPT_DOH_SSL_VERIFYPEER: CURLoption = 306;
pub const CURLOPT_AWS_SIGV4: CURLoption = 10305;
pub const CURLOPT_HSTSWRITEDATA: CURLoption = 10304;
pub const CURLOPT_HSTSWRITEFUNCTION: CURLoption = 20303;
pub const CURLOPT_HSTSREADDATA: CURLoption = 10302;
pub const CURLOPT_HSTSREADFUNCTION: CURLoption = 20301;
pub const CURLOPT_HSTS: CURLoption = 10300;
pub const CURLOPT_HSTS_CTRL: CURLoption = 299;
pub const CURLOPT_SSL_EC_CURVES: CURLoption = 10298;
pub const CURLOPT_PROXY_ISSUERCERT_BLOB: CURLoption = 40297;
pub const CURLOPT_PROXY_ISSUERCERT: CURLoption = 10296;
pub const CURLOPT_ISSUERCERT_BLOB: CURLoption = 40295;
pub const CURLOPT_PROXY_SSLKEY_BLOB: CURLoption = 40294;
pub const CURLOPT_PROXY_SSLCERT_BLOB: CURLoption = 40293;
pub const CURLOPT_SSLKEY_BLOB: CURLoption = 40292;
pub const CURLOPT_SSLCERT_BLOB: CURLoption = 40291;
pub const CURLOPT_MAIL_RCPT_ALLLOWFAILS: CURLoption = 290;
pub const CURLOPT_SASL_AUTHZID: CURLoption = 10289;
pub const CURLOPT_MAXAGE_CONN: CURLoption = 288;
pub const CURLOPT_ALTSVC: CURLoption = 10287;
pub const CURLOPT_ALTSVC_CTRL: CURLoption = 286;
pub const CURLOPT_HTTP09_ALLOWED: CURLoption = 285;
pub const CURLOPT_TRAILERDATA: CURLoption = 10284;
pub const CURLOPT_TRAILERFUNCTION: CURLoption = 20283;
pub const CURLOPT_CURLU: CURLoption = 10282;
pub const CURLOPT_UPKEEP_INTERVAL_MS: CURLoption = 281;
pub const CURLOPT_UPLOAD_BUFFERSIZE: CURLoption = 280;
pub const CURLOPT_DOH_URL: CURLoption = 10279;
pub const CURLOPT_DISALLOW_USERNAME_IN_URL: CURLoption = 278;
pub const CURLOPT_PROXY_TLS13_CIPHERS: CURLoption = 10277;
pub const CURLOPT_TLS13_CIPHERS: CURLoption = 10276;
pub const CURLOPT_DNS_SHUFFLE_ADDRESSES: CURLoption = 275;
pub const CURLOPT_HAPROXYPROTOCOL: CURLoption = 274;
pub const CURLOPT_RESOLVER_START_DATA: CURLoption = 10273;
pub const CURLOPT_RESOLVER_START_FUNCTION: CURLoption = 20272;
pub const CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS: CURLoption = 271;
pub const CURLOPT_TIMEVALUE_LARGE: CURLoption = 30270;
pub const CURLOPT_MIMEPOST: CURLoption = 10269;
pub const CURLOPT_SSH_COMPRESSION: CURLoption = 268;
pub const CURLOPT_SOCKS5_AUTH: CURLoption = 267;
pub const CURLOPT_REQUEST_TARGET: CURLoption = 10266;
pub const CURLOPT_SUPPRESS_CONNECT_HEADERS: CURLoption = 265;
pub const CURLOPT_ABSTRACT_UNIX_SOCKET: CURLoption = 10264;
pub const CURLOPT_PROXY_PINNEDPUBLICKEY: CURLoption = 10263;
pub const CURLOPT_PRE_PROXY: CURLoption = 10262;
pub const CURLOPT_PROXY_SSL_OPTIONS: CURLoption = 261;
pub const CURLOPT_PROXY_CRLFILE: CURLoption = 10260;
pub const CURLOPT_PROXY_SSL_CIPHER_LIST: CURLoption = 10259;
pub const CURLOPT_PROXY_KEYPASSWD: CURLoption = 10258;
pub const CURLOPT_PROXY_SSLKEYTYPE: CURLoption = 10257;
pub const CURLOPT_PROXY_SSLKEY: CURLoption = 10256;
pub const CURLOPT_PROXY_SSLCERTTYPE: CURLoption = 10255;
pub const CURLOPT_PROXY_SSLCERT: CURLoption = 10254;
pub const CURLOPT_PROXY_TLSAUTH_TYPE: CURLoption = 10253;
pub const CURLOPT_PROXY_TLSAUTH_PASSWORD: CURLoption = 10252;
pub const CURLOPT_PROXY_TLSAUTH_USERNAME: CURLoption = 10251;
pub const CURLOPT_PROXY_SSLVERSION: CURLoption = 250;
pub const CURLOPT_PROXY_SSL_VERIFYHOST: CURLoption = 249;
pub const CURLOPT_PROXY_SSL_VERIFYPEER: CURLoption = 248;
pub const CURLOPT_PROXY_CAPATH: CURLoption = 10247;
pub const CURLOPT_PROXY_CAINFO: CURLoption = 10246;
pub const CURLOPT_KEEP_SENDING_ON_ERROR: CURLoption = 245;
pub const CURLOPT_TCP_FASTOPEN: CURLoption = 244;
pub const CURLOPT_CONNECT_TO: CURLoption = 10243;
pub const CURLOPT_TFTP_NO_OPTIONS: CURLoption = 242;
pub const CURLOPT_STREAM_DEPENDS_E: CURLoption = 10241;
pub const CURLOPT_STREAM_DEPENDS: CURLoption = 10240;
pub const CURLOPT_STREAM_WEIGHT: CURLoption = 239;
pub const CURLOPT_DEFAULT_PROTOCOL: CURLoption = 10238;
pub const CURLOPT_PIPEWAIT: CURLoption = 237;
pub const CURLOPT_SERVICE_NAME: CURLoption = 10236;
pub const CURLOPT_PROXY_SERVICE_NAME: CURLoption = 10235;
pub const CURLOPT_PATH_AS_IS: CURLoption = 234;
pub const CURLOPT_SSL_FALSESTART: CURLoption = 233;
pub const CURLOPT_SSL_VERIFYSTATUS: CURLoption = 232;
pub const CURLOPT_UNIX_SOCKET_PATH: CURLoption = 10231;
pub const CURLOPT_PINNEDPUBLICKEY: CURLoption = 10230;
pub const CURLOPT_HEADEROPT: CURLoption = 229;
pub const CURLOPT_PROXYHEADER: CURLoption = 10228;
pub const CURLOPT_EXPECT_100_TIMEOUT_MS: CURLoption = 227;
pub const CURLOPT_SSL_ENABLE_ALPN: CURLoption = 226;
pub const CURLOPT_SSL_ENABLE_NPN: CURLoption = 225;
pub const CURLOPT_LOGIN_OPTIONS: CURLoption = 10224;
pub const CURLOPT_DNS_LOCAL_IP6: CURLoption = 10223;
pub const CURLOPT_DNS_LOCAL_IP4: CURLoption = 10222;
pub const CURLOPT_DNS_INTERFACE: CURLoption = 10221;
pub const CURLOPT_XOAUTH2_BEARER: CURLoption = 10220;
pub const CURLOPT_XFERINFOFUNCTION: CURLoption = 20219;
pub const CURLOPT_SASL_IR: CURLoption = 218;
pub const CURLOPT_MAIL_AUTH: CURLoption = 10217;
pub const CURLOPT_SSL_OPTIONS: CURLoption = 216;
pub const CURLOPT_TCP_KEEPINTVL: CURLoption = 215;
pub const CURLOPT_TCP_KEEPIDLE: CURLoption = 214;
pub const CURLOPT_TCP_KEEPALIVE: CURLoption = 213;
pub const CURLOPT_ACCEPTTIMEOUT_MS: CURLoption = 212;
pub const CURLOPT_DNS_SERVERS: CURLoption = 10211;
pub const CURLOPT_GSSAPI_DELEGATION: CURLoption = 210;
pub const CURLOPT_CLOSESOCKETDATA: CURLoption = 10209;
pub const CURLOPT_CLOSESOCKETFUNCTION: CURLoption = 20208;
pub const CURLOPT_TRANSFER_ENCODING: CURLoption = 207;
pub const CURLOPT_TLSAUTH_TYPE: CURLoption = 10206;
pub const CURLOPT_TLSAUTH_PASSWORD: CURLoption = 10205;
pub const CURLOPT_TLSAUTH_USERNAME: CURLoption = 10204;
pub const CURLOPT_RESOLVE: CURLoption = 10203;
pub const CURLOPT_FNMATCH_DATA: CURLoption = 10202;
pub const CURLOPT_CHUNK_DATA: CURLoption = 10201;
pub const CURLOPT_FNMATCH_FUNCTION: CURLoption = 20200;
pub const CURLOPT_CHUNK_END_FUNCTION: CURLoption = 20199;
pub const CURLOPT_CHUNK_BGN_FUNCTION: CURLoption = 20198;
pub const CURLOPT_WILDCARDMATCH: CURLoption = 197;
pub const CURLOPT_INTERLEAVEFUNCTION: CURLoption = 20196;
pub const CURLOPT_INTERLEAVEDATA: CURLoption = 10195;
pub const CURLOPT_RTSP_SERVER_CSEQ: CURLoption = 194;
pub const CURLOPT_RTSP_CLIENT_CSEQ: CURLoption = 193;
pub const CURLOPT_RTSP_TRANSPORT: CURLoption = 10192;
pub const CURLOPT_RTSP_STREAM_URI: CURLoption = 10191;
pub const CURLOPT_RTSP_SESSION_ID: CURLoption = 10190;
pub const CURLOPT_RTSP_REQUEST: CURLoption = 189;
pub const CURLOPT_FTP_USE_PRET: CURLoption = 188;
pub const CURLOPT_MAIL_RCPT: CURLoption = 10187;
pub const CURLOPT_MAIL_FROM: CURLoption = 10186;
pub const CURLOPT_SSH_KEYDATA: CURLoption = 10185;
pub const CURLOPT_SSH_KEYFUNCTION: CURLoption = 20184;
pub const CURLOPT_SSH_KNOWNHOSTS: CURLoption = 10183;
pub const CURLOPT_REDIR_PROTOCOLS: CURLoption = 182;
pub const CURLOPT_PROTOCOLS: CURLoption = 181;
pub const CURLOPT_SOCKS5_GSSAPI_NEC: CURLoption = 180;
pub const CURLOPT_SOCKS5_GSSAPI_SERVICE: CURLoption = 10179;
pub const CURLOPT_TFTP_BLKSIZE: CURLoption = 178;
pub const CURLOPT_NOPROXY: CURLoption = 10177;
pub const CURLOPT_PROXYPASSWORD: CURLoption = 10176;
pub const CURLOPT_PROXYUSERNAME: CURLoption = 10175;
pub const CURLOPT_PASSWORD: CURLoption = 10174;
pub const CURLOPT_USERNAME: CURLoption = 10173;
pub const CURLOPT_CERTINFO: CURLoption = 172;
pub const CURLOPT_ADDRESS_SCOPE: CURLoption = 171;
pub const CURLOPT_ISSUERCERT: CURLoption = 10170;
pub const CURLOPT_CRLFILE: CURLoption = 10169;
pub const CURLOPT_SEEKDATA: CURLoption = 10168;
pub const CURLOPT_SEEKFUNCTION: CURLoption = 20167;
pub const CURLOPT_PROXY_TRANSFER_MODE: CURLoption = 166;
pub const CURLOPT_COPYPOSTFIELDS: CURLoption = 10165;
pub const CURLOPT_OPENSOCKETDATA: CURLoption = 10164;
pub const CURLOPT_OPENSOCKETFUNCTION: CURLoption = 20163;
pub const CURLOPT_SSH_HOST_PUBLIC_KEY_MD5: CURLoption = 10162;
pub const CURLOPT_POSTREDIR: CURLoption = 161;
pub const CURLOPT_NEW_DIRECTORY_PERMS: CURLoption = 160;
pub const CURLOPT_NEW_FILE_PERMS: CURLoption = 159;
pub const CURLOPT_HTTP_CONTENT_DECODING: CURLoption = 158;
pub const CURLOPT_HTTP_TRANSFER_DECODING: CURLoption = 157;
pub const CURLOPT_CONNECTTIMEOUT_MS: CURLoption = 156;
pub const CURLOPT_TIMEOUT_MS: CURLoption = 155;
pub const CURLOPT_FTP_SSL_CCC: CURLoption = 154;
pub const CURLOPT_SSH_PRIVATE_KEYFILE: CURLoption = 10153;
pub const CURLOPT_SSH_PUBLIC_KEYFILE: CURLoption = 10152;
pub const CURLOPT_SSH_AUTH_TYPES: CURLoption = 151;
pub const CURLOPT_SSL_SESSIONID_CACHE: CURLoption = 150;
pub const CURLOPT_SOCKOPTDATA: CURLoption = 10149;
pub const CURLOPT_SOCKOPTFUNCTION: CURLoption = 20148;
pub const CURLOPT_FTP_ALTERNATIVE_TO_USER: CURLoption = 10147;
pub const CURLOPT_MAX_RECV_SPEED_LARGE: CURLoption = 30146;
pub const CURLOPT_MAX_SEND_SPEED_LARGE: CURLoption = 30145;
pub const CURLOPT_CONV_FROM_UTF8_FUNCTION: CURLoption = 20144;
pub const CURLOPT_CONV_TO_NETWORK_FUNCTION: CURLoption = 20143;
pub const CURLOPT_CONV_FROM_NETWORK_FUNCTION: CURLoption = 20142;
pub const CURLOPT_CONNECT_ONLY: CURLoption = 141;
pub const CURLOPT_LOCALPORTRANGE: CURLoption = 140;
pub const CURLOPT_LOCALPORT: CURLoption = 139;
pub const CURLOPT_FTP_FILEMETHOD: CURLoption = 138;
pub const CURLOPT_FTP_SKIP_PASV_IP: CURLoption = 137;
pub const CURLOPT_IGNORE_CONTENT_LENGTH: CURLoption = 136;
pub const CURLOPT_COOKIELIST: CURLoption = 10135;
pub const CURLOPT_FTP_ACCOUNT: CURLoption = 10134;
pub const CURLOPT_IOCTLDATA: CURLoption = 10131;
pub const CURLOPT_IOCTLFUNCTION: CURLoption = 20130;
pub const CURLOPT_FTPSSLAUTH: CURLoption = 129;
pub const CURLOPT_TCP_NODELAY: CURLoption = 121;
pub const CURLOPT_POSTFIELDSIZE_LARGE: CURLoption = 30120;
pub const CURLOPT_USE_SSL: CURLoption = 119;
pub const CURLOPT_NETRC_FILE: CURLoption = 10118;
pub const CURLOPT_MAXFILESIZE_LARGE: CURLoption = 30117;
pub const CURLOPT_RESUME_FROM_LARGE: CURLoption = 30116;
pub const CURLOPT_INFILESIZE_LARGE: CURLoption = 30115;
pub const CURLOPT_MAXFILESIZE: CURLoption = 114;
pub const CURLOPT_IPRESOLVE: CURLoption = 113;
pub const CURLOPT_FTP_RESPONSE_TIMEOUT: CURLoption = 112;
pub const CURLOPT_PROXYAUTH: CURLoption = 111;
pub const CURLOPT_FTP_CREATE_MISSING_DIRS: CURLoption = 110;
pub const CURLOPT_SSL_CTX_DATA: CURLoption = 10109;
pub const CURLOPT_SSL_CTX_FUNCTION: CURLoption = 20108;
pub const CURLOPT_HTTPAUTH: CURLoption = 107;
pub const CURLOPT_FTP_USE_EPRT: CURLoption = 106;
pub const CURLOPT_UNRESTRICTED_AUTH: CURLoption = 105;
pub const CURLOPT_HTTP200ALIASES: CURLoption = 10104;
pub const CURLOPT_PRIVATE: CURLoption = 10103;
pub const CURLOPT_ACCEPT_ENCODING: CURLoption = 10102;
pub const CURLOPT_PROXYTYPE: CURLoption = 101;
pub const CURLOPT_SHARE: CURLoption = 10100;
pub const CURLOPT_NOSIGNAL: CURLoption = 99;
pub const CURLOPT_BUFFERSIZE: CURLoption = 98;
pub const CURLOPT_CAPATH: CURLoption = 10097;
pub const CURLOPT_COOKIESESSION: CURLoption = 96;
pub const CURLOPT_DEBUGDATA: CURLoption = 10095;
pub const CURLOPT_DEBUGFUNCTION: CURLoption = 20094;
pub const CURLOPT_PREQUOTE: CURLoption = 10093;
pub const CURLOPT_DNS_CACHE_TIMEOUT: CURLoption = 92;
pub const CURLOPT_DNS_USE_GLOBAL_CACHE: CURLoption = 91;
pub const CURLOPT_SSLENGINE_DEFAULT: CURLoption = 90;
pub const CURLOPT_SSLENGINE: CURLoption = 10089;
pub const CURLOPT_SSLKEYTYPE: CURLoption = 10088;
pub const CURLOPT_SSLKEY: CURLoption = 10087;
pub const CURLOPT_SSLCERTTYPE: CURLoption = 10086;
pub const CURLOPT_FTP_USE_EPSV: CURLoption = 85;
pub const CURLOPT_HTTP_VERSION: CURLoption = 84;
pub const CURLOPT_SSL_CIPHER_LIST: CURLoption = 10083;
pub const CURLOPT_COOKIEJAR: CURLoption = 10082;
pub const CURLOPT_SSL_VERIFYHOST: CURLoption = 81;
pub const CURLOPT_HTTPGET: CURLoption = 80;
pub const CURLOPT_HEADERFUNCTION: CURLoption = 20079;
pub const CURLOPT_CONNECTTIMEOUT: CURLoption = 78;
pub const CURLOPT_EGDSOCKET: CURLoption = 10077;
pub const CURLOPT_RANDOM_FILE: CURLoption = 10076;
pub const CURLOPT_FORBID_REUSE: CURLoption = 75;
pub const CURLOPT_FRESH_CONNECT: CURLoption = 74;
pub const CURLOPT_OBSOLETE72: CURLoption = 72;
pub const CURLOPT_MAXCONNECTS: CURLoption = 71;
pub const CURLOPT_TELNETOPTIONS: CURLoption = 10070;
pub const CURLOPT_FILETIME: CURLoption = 69;
pub const CURLOPT_MAXREDIRS: CURLoption = 68;
pub const CURLOPT_CAINFO: CURLoption = 10065;
pub const CURLOPT_SSL_VERIFYPEER: CURLoption = 64;
pub const CURLOPT_KRBLEVEL: CURLoption = 10063;
pub const CURLOPT_INTERFACE: CURLoption = 10062;
pub const CURLOPT_HTTPPROXYTUNNEL: CURLoption = 61;
pub const CURLOPT_POSTFIELDSIZE: CURLoption = 60;
pub const CURLOPT_PROXYPORT: CURLoption = 59;
pub const CURLOPT_AUTOREFERER: CURLoption = 58;
pub const CURLOPT_XFERINFODATA: CURLoption = 10057;
pub const CURLOPT_PROGRESSFUNCTION: CURLoption = 20056;
pub const CURLOPT_PUT: CURLoption = 54;
pub const CURLOPT_TRANSFERTEXT: CURLoption = 53;
pub const CURLOPT_FOLLOWLOCATION: CURLoption = 52;
pub const CURLOPT_NETRC: CURLoption = 51;
pub const CURLOPT_APPEND: CURLoption = 50;
pub const CURLOPT_DIRLISTONLY: CURLoption = 48;
pub const CURLOPT_POST: CURLoption = 47;
pub const CURLOPT_UPLOAD: CURLoption = 46;
pub const CURLOPT_FAILONERROR: CURLoption = 45;
pub const CURLOPT_NOBODY: CURLoption = 44;
pub const CURLOPT_NOPROGRESS: CURLoption = 43;
pub const CURLOPT_HEADER: CURLoption = 42;
pub const CURLOPT_VERBOSE: CURLoption = 41;
pub const CURLOPT_OBSOLETE40: CURLoption = 10040;
pub const CURLOPT_POSTQUOTE: CURLoption = 10039;
pub const CURLOPT_STDERR: CURLoption = 10037;
pub const CURLOPT_CUSTOMREQUEST: CURLoption = 10036;
pub const CURLOPT_TIMEVALUE: CURLoption = 34;
pub const CURLOPT_TIMECONDITION: CURLoption = 33;
pub const CURLOPT_SSLVERSION: CURLoption = 32;
pub const CURLOPT_COOKIEFILE: CURLoption = 10031;
pub const CURLOPT_HEADERDATA: CURLoption = 10029;
pub const CURLOPT_QUOTE: CURLoption = 10028;
pub const CURLOPT_CRLF: CURLoption = 27;
pub const CURLOPT_KEYPASSWD: CURLoption = 10026;
pub const CURLOPT_SSLCERT: CURLoption = 10025;
pub const CURLOPT_HTTPPOST: CURLoption = 10024;
pub const CURLOPT_HTTPHEADER: CURLoption = 10023;
pub const CURLOPT_COOKIE: CURLoption = 10022;
pub const CURLOPT_RESUME_FROM: CURLoption = 21;
pub const CURLOPT_LOW_SPEED_TIME: CURLoption = 20;
pub const CURLOPT_LOW_SPEED_LIMIT: CURLoption = 19;
pub const CURLOPT_USERAGENT: CURLoption = 10018;
pub const CURLOPT_FTPPORT: CURLoption = 10017;
pub const CURLOPT_REFERER: CURLoption = 10016;
pub const CURLOPT_POSTFIELDS: CURLoption = 10015;
pub const CURLOPT_INFILESIZE: CURLoption = 14;
pub const CURLOPT_TIMEOUT: CURLoption = 13;
pub const CURLOPT_READFUNCTION: CURLoption = 20012;
pub const CURLOPT_WRITEFUNCTION: CURLoption = 20011;
pub const CURLOPT_ERRORBUFFER: CURLoption = 10010;
pub const CURLOPT_READDATA: CURLoption = 10009;
pub const CURLOPT_RANGE: CURLoption = 10007;
pub const CURLOPT_PROXYUSERPWD: CURLoption = 10006;
pub const CURLOPT_USERPWD: CURLoption = 10005;
pub const CURLOPT_PROXY: CURLoption = 10004;
pub const CURLOPT_PORT: CURLoption = 3;
pub const CURLOPT_URL: CURLoption = 10002;
pub const CURLOPT_WRITEDATA: CURLoption = 10001;
pub type C2RustUnnamed_1 = u32;
pub const CURL_HTTP_VERSION_LAST: C2RustUnnamed_1 = 31;
pub const CURL_HTTP_VERSION_3: C2RustUnnamed_1 = 30;
pub const CURL_HTTP_VERSION_2_PRIOR_KNOWLEDGE: C2RustUnnamed_1 = 5;
pub const CURL_HTTP_VERSION_2TLS: C2RustUnnamed_1 = 4;
pub const CURL_HTTP_VERSION_2_0: C2RustUnnamed_1 = 3;
pub const CURL_HTTP_VERSION_1_1: C2RustUnnamed_1 = 2;
pub const CURL_HTTP_VERSION_1_0: C2RustUnnamed_1 = 1;
pub const CURL_HTTP_VERSION_NONE: C2RustUnnamed_1 = 0;
pub type CURL_NETRC_OPTION = u32;
pub const CURL_NETRC_LAST: CURL_NETRC_OPTION = 3;
pub const CURL_NETRC_REQUIRED: CURL_NETRC_OPTION = 2;
pub const CURL_NETRC_OPTIONAL: CURL_NETRC_OPTION = 1;
pub const CURL_NETRC_IGNORED: CURL_NETRC_OPTION = 0;
pub type curl_TimeCond = u32;
pub const CURL_TIMECOND_LAST: curl_TimeCond = 4;
pub const CURL_TIMECOND_LASTMOD: curl_TimeCond = 3;
pub const CURL_TIMECOND_IFUNMODSINCE: curl_TimeCond = 2;
pub const CURL_TIMECOND_IFMODSINCE: curl_TimeCond = 1;
pub const CURL_TIMECOND_NONE: curl_TimeCond = 0;
pub type curl_tlssessioninfo = crate::src::lib::http2::curl_tlssessioninfo;
pub type CURLINFO = u32;
pub const CURLINFO_LASTONE: CURLINFO = 60;
pub const CURLINFO_REFERER: CURLINFO = 1048636;
pub const CURLINFO_PROXY_ERROR: CURLINFO = 2097211;
pub const CURLINFO_EFFECTIVE_METHOD: CURLINFO = 1048634;
pub const CURLINFO_RETRY_AFTER: CURLINFO = 6291513;
pub const CURLINFO_APPCONNECT_TIME_T: CURLINFO = 6291512;
pub const CURLINFO_REDIRECT_TIME_T: CURLINFO = 6291511;
pub const CURLINFO_STARTTRANSFER_TIME_T: CURLINFO = 6291510;
pub const CURLINFO_PRETRANSFER_TIME_T: CURLINFO = 6291509;
pub const CURLINFO_CONNECT_TIME_T: CURLINFO = 6291508;
pub const CURLINFO_NAMELOOKUP_TIME_T: CURLINFO = 6291507;
pub const CURLINFO_TOTAL_TIME_T: CURLINFO = 6291506;
pub const CURLINFO_SCHEME: CURLINFO = 1048625;
pub const CURLINFO_PROTOCOL: CURLINFO = 2097200;
pub const CURLINFO_PROXY_SSL_VERIFYRESULT: CURLINFO = 2097199;
pub const CURLINFO_HTTP_VERSION: CURLINFO = 2097198;
pub const CURLINFO_TLS_SSL_PTR: CURLINFO = 4194349;
pub const CURLINFO_ACTIVESOCKET: CURLINFO = 5242924;
pub const CURLINFO_TLS_SESSION: CURLINFO = 4194347;
pub const CURLINFO_LOCAL_PORT: CURLINFO = 2097194;
pub const CURLINFO_LOCAL_IP: CURLINFO = 1048617;
pub const CURLINFO_PRIMARY_PORT: CURLINFO = 2097192;
pub const CURLINFO_RTSP_CSEQ_RECV: CURLINFO = 2097191;
pub const CURLINFO_RTSP_SERVER_CSEQ: CURLINFO = 2097190;
pub const CURLINFO_RTSP_CLIENT_CSEQ: CURLINFO = 2097189;
pub const CURLINFO_RTSP_SESSION_ID: CURLINFO = 1048612;
pub const CURLINFO_CONDITION_UNMET: CURLINFO = 2097187;
pub const CURLINFO_CERTINFO: CURLINFO = 4194338;
pub const CURLINFO_APPCONNECT_TIME: CURLINFO = 3145761;
pub const CURLINFO_PRIMARY_IP: CURLINFO = 1048608;
pub const CURLINFO_REDIRECT_URL: CURLINFO = 1048607;
pub const CURLINFO_FTP_ENTRY_PATH: CURLINFO = 1048606;
pub const CURLINFO_LASTSOCKET: CURLINFO = 2097181;
pub const CURLINFO_COOKIELIST: CURLINFO = 4194332;
pub const CURLINFO_SSL_ENGINES: CURLINFO = 4194331;
pub const CURLINFO_NUM_CONNECTS: CURLINFO = 2097178;
pub const CURLINFO_OS_ERRNO: CURLINFO = 2097177;
pub const CURLINFO_PROXYAUTH_AVAIL: CURLINFO = 2097176;
pub const CURLINFO_HTTPAUTH_AVAIL: CURLINFO = 2097175;
pub const CURLINFO_HTTP_CONNECTCODE: CURLINFO = 2097174;
pub const CURLINFO_PRIVATE: CURLINFO = 1048597;
pub const CURLINFO_REDIRECT_COUNT: CURLINFO = 2097172;
pub const CURLINFO_REDIRECT_TIME: CURLINFO = 3145747;
pub const CURLINFO_CONTENT_TYPE: CURLINFO = 1048594;
pub const CURLINFO_STARTTRANSFER_TIME: CURLINFO = 3145745;
pub const CURLINFO_CONTENT_LENGTH_UPLOAD_T: CURLINFO = 6291472;
pub const CURLINFO_CONTENT_LENGTH_UPLOAD: CURLINFO = 3145744;
pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD_T: CURLINFO = 6291471;
pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD: CURLINFO = 3145743;
pub const CURLINFO_FILETIME_T: CURLINFO = 6291470;
pub const CURLINFO_FILETIME: CURLINFO = 2097166;
pub const CURLINFO_SSL_VERIFYRESULT: CURLINFO = 2097165;
pub const CURLINFO_REQUEST_SIZE: CURLINFO = 2097164;
pub const CURLINFO_HEADER_SIZE: CURLINFO = 2097163;
pub const CURLINFO_SPEED_UPLOAD_T: CURLINFO = 6291466;
pub const CURLINFO_SPEED_UPLOAD: CURLINFO = 3145738;
pub const CURLINFO_SPEED_DOWNLOAD_T: CURLINFO = 6291465;
pub const CURLINFO_SPEED_DOWNLOAD: CURLINFO = 3145737;
pub const CURLINFO_SIZE_DOWNLOAD_T: CURLINFO = 6291464;
pub const CURLINFO_SIZE_DOWNLOAD: CURLINFO = 3145736;
pub const CURLINFO_SIZE_UPLOAD_T: CURLINFO = 6291463;
pub const CURLINFO_SIZE_UPLOAD: CURLINFO = 3145735;
pub const CURLINFO_PRETRANSFER_TIME: CURLINFO = 3145734;
pub const CURLINFO_CONNECT_TIME: CURLINFO = 3145733;
pub const CURLINFO_NAMELOOKUP_TIME: CURLINFO = 3145732;
pub const CURLINFO_TOTAL_TIME: CURLINFO = 3145731;
pub const CURLINFO_RESPONSE_CODE: CURLINFO = 2097154;
pub const CURLINFO_EFFECTIVE_URL: CURLINFO = 1048577;
pub const CURLINFO_NONE: CURLINFO = 0;
pub type C2RustUnnamed_2 = u32;
pub const CURL_LOCK_DATA_LAST: C2RustUnnamed_2 = 7;
pub const CURL_LOCK_DATA_PSL: C2RustUnnamed_2 = 6;
pub const CURL_LOCK_DATA_CONNECT: C2RustUnnamed_2 = 5;
pub const CURL_LOCK_DATA_SSL_SESSION: C2RustUnnamed_2 = 4;
pub const CURL_LOCK_DATA_DNS: C2RustUnnamed_2 = 3;
pub const CURL_LOCK_DATA_COOKIE: C2RustUnnamed_2 = 2;
pub const CURL_LOCK_DATA_SHARE: C2RustUnnamed_2 = 1;
pub const CURL_LOCK_DATA_NONE: C2RustUnnamed_2 = 0;
pub type CURLSHcode = u32;
pub const CURLSHE_LAST: CURLSHcode = 6;
pub const CURLSHE_NOT_BUILT_IN: CURLSHcode = 5;
pub const CURLSHE_NOMEM: CURLSHcode = 4;
pub const CURLSHE_INVALID: CURLSHcode = 3;
pub const CURLSHE_IN_USE: CURLSHcode = 2;
pub const CURLSHE_BAD_OPTION: CURLSHcode = 1;
pub const CURLSHE_OK: CURLSHcode = 0;
pub type CURLSHoption = u32;
pub const CURLSHOPT_LAST: CURLSHoption = 6;
pub const CURLSHOPT_USERDATA: CURLSHoption = 5;
pub const CURLSHOPT_UNLOCKFUNC: CURLSHoption = 4;
pub const CURLSHOPT_LOCKFUNC: CURLSHoption = 3;
pub const CURLSHOPT_UNSHARE: CURLSHoption = 2;
pub const CURLSHOPT_SHARE: CURLSHoption = 1;
pub const CURLSHOPT_NONE: CURLSHoption = 0;
pub type CURLversion = u32;
pub const CURLVERSION_LAST: CURLversion = 10;
pub const CURLVERSION_TENTH: CURLversion = 9;
pub const CURLVERSION_NINTH: CURLversion = 8;
pub const CURLVERSION_EIGHTH: CURLversion = 7;
pub const CURLVERSION_SEVENTH: CURLversion = 6;
pub const CURLVERSION_SIXTH: CURLversion = 5;
pub const CURLVERSION_FIFTH: CURLversion = 4;
pub const CURLVERSION_FOURTH: CURLversion = 3;
pub const CURLVERSION_THIRD: CURLversion = 2;
pub const CURLVERSION_SECOND: CURLversion = 1;
pub const CURLVERSION_FIRST: CURLversion = 0;
pub type curl_version_info_data<'a> = crate::src::lib::version::curl_version_info_data<'a>;
pub type CURLM = crate::src::lib::http2::Curl_multi;
pub type CURLMcode = i32;
pub const CURLM_LAST: CURLMcode = 11;
pub const CURLM_BAD_FUNCTION_ARGUMENT: CURLMcode = 10;
pub const CURLM_WAKEUP_FAILURE: CURLMcode = 9;
pub const CURLM_RECURSIVE_API_CALL: CURLMcode = 8;
pub const CURLM_ADDED_ALREADY: CURLMcode = 7;
pub const CURLM_UNKNOWN_OPTION: CURLMcode = 6;
pub const CURLM_BAD_SOCKET: CURLMcode = 5;
pub const CURLM_INTERNAL_ERROR: CURLMcode = 4;
pub const CURLM_OUT_OF_MEMORY: CURLMcode = 3;
pub const CURLM_BAD_EASY_HANDLE: CURLMcode = 2;
pub const CURLM_BAD_HANDLE: CURLMcode = 1;
pub const CURLM_OK: CURLMcode = 0;
pub const CURLM_CALL_MULTI_PERFORM: CURLMcode = -1;
pub type CURLMSG = u32;
pub const CURLMSG_LAST: CURLMSG = 2;
pub const CURLMSG_DONE: CURLMSG = 1;
pub const CURLMSG_NONE: CURLMSG = 0;
pub type CURLMsg = crate::src::lib::http2::CURLMsg;
pub type C2RustUnnamed_3 = crate::src::lib::http2::C2RustUnnamed_3;
pub type curl_waitfd = crate::src::lib::easy::curl_waitfd;
pub type stat = crate::src::lib::file::stat;
pub type OutStruct = crate::src::src::tool_cb_hdr::OutStruct;
pub type InStruct = crate::src::src::tool_cb_hdr::InStruct;
pub type OperationConfig = crate::src::src::tool_cb_dbg::OperationConfig;
pub type State = crate::src::src::tool_cb_dbg::State;
pub type URLGlob = crate::src::src::tool_cb_dbg::URLGlob;
pub type URLPattern = crate::src::src::tool_cb_dbg::URLPattern;
pub type C2RustUnnamed_4 = crate::src::src::tool_cb_dbg::C2RustUnnamed;
pub type C2RustUnnamed_5 = crate::src::src::tool_cb_dbg::C2RustUnnamed_0;
pub type C2RustUnnamed_6 = crate::src::src::tool_cb_dbg::C2RustUnnamed_1;
pub type C2RustUnnamed_7 = crate::src::src::tool_cb_dbg::C2RustUnnamed_2;
pub type URLPatternType = u32;
pub const UPTNumRange: URLPatternType = 3;
pub const UPTCharRange: URLPatternType = 2;
pub const UPTSet: URLPatternType = 1;
pub type getout = crate::src::src::tool_cb_dbg::getout;
pub type GlobalConfig = crate::src::src::tool_cb_dbg::GlobalConfig;
pub type trace = u32;
pub const TRACE_PLAIN: trace = 3;
pub const TRACE_ASCII: trace = 2;
pub const TRACE_BIN: trace = 1;
pub const TRACE_NONE: trace = 0;
pub type curl_error = u32;
pub const ERR_LAST: curl_error = 2;
pub const ERR_BINARY_TERMINAL: curl_error = 1;
pub const ERR_NONE: curl_error = 0;
pub type HttpReq = u32;
pub const HTTPREQ_SIMPLEPOST: HttpReq = 4;
pub const HTTPREQ_MIMEPOST: HttpReq = 3;
pub const HTTPREQ_HEAD: HttpReq = 2;
pub const HTTPREQ_GET: HttpReq = 1;
pub const HTTPREQ_UNSPEC: HttpReq = 0;
pub type tool_mime = crate::src::src::tool_cb_dbg::tool_mime;
pub type toolmimekind = u32;
pub const TOOLMIME_STDINDATA: toolmimekind = 6;
pub const TOOLMIME_STDIN: toolmimekind = 5;
pub const TOOLMIME_FILEDATA: toolmimekind = 4;
pub const TOOLMIME_FILE: toolmimekind = 3;
pub const TOOLMIME_DATA: toolmimekind = 2;
pub const TOOLMIME_PARTS: toolmimekind = 1;
pub const TOOLMIME_NONE: toolmimekind = 0;
pub type HdrCbData<'a> = crate::src::src::tool_cb_hdr::HdrCbData<'a>;
pub type ProgressData = crate::src::src::tool_cb_hdr::ProgressData;
pub type ParameterError = u32;
pub const PARAM_LAST: ParameterError = 21;
pub const PARAM_CONTDISP_RESUME_FROM: ParameterError = 20;
pub const PARAM_CONTDISP_SHOW_HEADER: ParameterError = 19;
pub const PARAM_NO_NOT_BOOLEAN: ParameterError = 18;
pub const PARAM_NUMBER_TOO_LARGE: ParameterError = 17;
pub const PARAM_NO_PREFIX: ParameterError = 16;
pub const PARAM_NEXT_OPERATION: ParameterError = 15;
pub const PARAM_NO_MEM: ParameterError = 14;
pub const PARAM_LIBCURL_UNSUPPORTED_PROTOCOL: ParameterError = 13;
pub const PARAM_LIBCURL_DOESNT_SUPPORT: ParameterError = 12;
pub const PARAM_NEGATIVE_NUMERIC: ParameterError = 11;
pub const PARAM_BAD_NUMERIC: ParameterError = 10;
pub const PARAM_GOT_EXTRA_PARAMETER: ParameterError = 9;
pub const PARAM_ENGINES_REQUESTED: ParameterError = 8;
pub const PARAM_VERSION_INFO_REQUESTED: ParameterError = 7;
pub const PARAM_MANUAL_REQUESTED: ParameterError = 6;
pub const PARAM_HELP_REQUESTED: ParameterError = 5;
pub const PARAM_BAD_USE: ParameterError = 4;
pub const PARAM_REQUIRES_PARAMETER: ParameterError = 3;
pub const PARAM_OPTION_UNKNOWN: ParameterError = 2;
pub const PARAM_OPTION_AMBIGUOUS: ParameterError = 1;
pub const PARAM_OK: ParameterError = 0;
pub type per_transfer<'a> = crate::src::src::tool_cb_hdr::per_transfer<'a>;
pub type C2RustUnnamed_8 = u32;
pub const RETRY_LAST: C2RustUnnamed_8 = 6;
pub const RETRY_FTP: C2RustUnnamed_8 = 5;
pub const RETRY_HTTP: C2RustUnnamed_8 = 4;
pub const RETRY_CONNREFUSED: C2RustUnnamed_8 = 3;
pub const RETRY_TIMEOUT: C2RustUnnamed_8 = 2;
pub const RETRY_ALL_ERRORS: C2RustUnnamed_8 = 1;
pub const RETRY_NO: C2RustUnnamed_8 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameValue {
    pub name: *const i8,
    pub value: i64,
}
impl NameValue {
    pub const fn new() -> Self {
        NameValue {
            name: (0 as *const i8),
            value: 0,
        }
    }
}
impl std::default::Default for NameValue {
    fn default() -> Self {
        NameValue::new()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NameValueUnsigned {
    pub name: *const i8,
    pub value: u64,
}
impl NameValueUnsigned {
    pub const fn new() -> Self {
        NameValueUnsigned {
            name: (0 as *const i8),
            value: 0,
        }
    }
}
impl std::default::Default for NameValueUnsigned {
    fn default() -> Self {
        NameValueUnsigned::new()
    }
}
pub type dynbuf = crate::src::lib::http2::dynbuf;
#[inline]
extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut crate::src::lib::file::stat) -> i32 {
    return unsafe { __xstat(1 as i32, __path, __statbuf) };
}
#[inline]
extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut crate::src::lib::file::stat) -> i32 {
    return unsafe { __fxstat(1 as i32, __fd, __statbuf) };
}
extern "C" fn is_fatal_error(mut code: u32) -> bool {
    match code as u32 {
        2 | 27 | 48 | 41 | 43 => return 1 as i32 != 0,
        _ => {}
    }
    return 0 as i32 != 0;
}
extern "C" fn is_pkcs11_uri(mut string: *const i8) -> bool {
    if curl_strnequal(
        string,
        b"pkcs11:\0" as *const u8 as *const i8,
        7 as i32 as size_t,
    ) != 0
    {
        return 1 as i32 != 0;
    } else {
        return 0 as i32 != 0;
    };
}
#[no_mangle]
pub static mut transfers: *mut crate::src::src::tool_cb_hdr::per_transfer<'static> =
    0 as *const per_transfer as *mut per_transfer;
static mut transfersl: *mut crate::src::src::tool_cb_hdr::per_transfer<'static> =
    0 as *const per_transfer as *mut per_transfer;
extern "C" fn add_per_transfer<'a1>(
    mut per: *mut *mut crate::src::src::tool_cb_hdr::per_transfer<'a1>,
) -> u32
where
    'a1: 'static,
{
    let mut p: *mut crate::src::src::tool_cb_hdr::per_transfer<'_> = 0 as *mut per_transfer;
    p = (unsafe { calloc(
        ::std::mem::size_of::<per_transfer>() as u64,
        1 as i32 as u64,
    ) }) as *mut per_transfer;
    if p.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    if (unsafe { transfers }).is_null() {
        (unsafe { transfers = p });
        (unsafe { transfersl = transfers });
    } else {
        let mut fresh0 = unsafe { &mut ((*transfersl).next) };
        *fresh0 = p;
        let mut fresh1 = unsafe { &mut ((*p).prev) };
        *fresh1 = unsafe { transfersl };
        (unsafe { transfersl = p });
    }
    (unsafe { *per = p });
    (unsafe { all_xfers += 1 });
    return CURLE_OK;
}
extern "C" fn del_per_transfer<'a1, 'a2>(
    mut per: *mut crate::src::src::tool_cb_hdr::per_transfer<'a1>,
) -> *mut crate::src::src::tool_cb_hdr::per_transfer<'a2>
where
    'a2: 'static,
    'a2: 'a1,
    'a1: 'a2,
{
    let mut n: *mut crate::src::src::tool_cb_hdr::per_transfer<'_> = 0 as *mut per_transfer;
    let mut p: *mut crate::src::src::tool_cb_hdr::per_transfer<'_> = 0 as *mut per_transfer;
    n = unsafe { (*per).next };
    p = unsafe { (*per).prev };
    if !p.is_null() {
        let mut fresh2 = unsafe { &mut ((*p).next) };
        *fresh2 = n;
    } else {
        (unsafe { transfers = n });
    }
    if !n.is_null() {
        let mut fresh3 = unsafe { &mut ((*n).prev) };
        *fresh3 = p;
    } else {
        (unsafe { transfersl = p });
    }
    (unsafe { free(per as *mut libc::c_void) });
    return n;
}
extern "C" fn pre_transfer<'a1>(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut per: *mut crate::src::src::tool_cb_hdr::per_transfer<'a1>,
) -> u32 {
    let mut uploadfilesize: i64 = -(1 as i32) as curl_off_t;
    let mut fileinfo: crate::src::lib::file::stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut result: u32 = CURLE_OK;
    if !(unsafe { (*per).separator_err }).is_null() {
        (unsafe { curl_mfprintf(
            (*global).errors,
            b"%s\n\0" as *const u8 as *const i8,
            (*per).separator_err,
        ) });
    }
    if !(unsafe { (*per).separator }).is_null() {
        (unsafe { curl_mprintf(b"%s\n\0" as *const u8 as *const i8, (*per).separator) });
    }
    if !(unsafe { (*per).uploadfile }).is_null() && !stdin_upload(unsafe { (*per).uploadfile }) {
        (unsafe { (*per).infd = open((*per).uploadfile, 0 as i32 | 0 as i32) });
        if (unsafe { (*per).infd }) == -(1 as i32) || fstat(unsafe { (*per).infd }, &mut fileinfo) != 0 {
            (unsafe { helpf(
                (*global).errors,
                b"Can't open '%s'!\n\0" as *const u8 as *const i8,
                (*per).uploadfile,
            ) });
            if (unsafe { (*per).infd }) != -(1 as i32) {
                (unsafe { close((*per).infd) });
                (unsafe { (*per).infd = 0 as i32 });
            }
            return CURLE_READ_ERROR;
        }
        (unsafe { (*per).infdopen = 1 as i32 != 0 });
        if fileinfo.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32 {
            uploadfilesize = fileinfo.st_size;
        }
        if uploadfilesize != -(1 as i32) as i64 {
            let mut config: *mut crate::src::src::tool_cb_dbg::OperationConfig = unsafe { (*per).config };
            if !tool_setopt_skip(CURLOPT_INFILESIZE_LARGE) {
                result = unsafe { tool_setopt(
                    (*per).curl,
                    0 as i32 != 0,
                    global,
                    config,
                    b"CURLOPT_INFILESIZE_LARGE\0" as *const u8 as *const i8,
                    CURLOPT_INFILESIZE_LARGE,
                    uploadfilesize,
                ) };
                let _ = result as u64 != 0;
            }
        }
        (unsafe { (*per).input.fd = (*per).infd });
    }
    return result;
}
extern "C" fn post_per_transfer<'a1, 'a2, 'a3>(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut per: *mut crate::src::src::tool_cb_hdr::per_transfer<'a1>,
    mut result: u32,
    mut retryp: Option<&'a2 mut bool>,
    mut delay: Option<&'a3 mut i64>,
) -> u32
where
    'a1: 'static,
{
    let mut current_block: u64;
    let mut outs: Option<&'_ mut crate::src::src::tool_cb_hdr::OutStruct> = Some(unsafe { &mut (*per).outs });
    let mut curl: *mut crate::src::lib::http2::Curl_easy = unsafe { (*per).curl };
    let mut config: *mut crate::src::src::tool_cb_dbg::OperationConfig = unsafe { (*per).config };
    if curl.is_null() || config.is_null() {
        return result;
    }
    *(borrow_mut(&mut retryp)).unwrap() = 0 as i32 != 0;
    *(borrow_mut(&mut delay)).unwrap() = 0 as i32 as i64;
    if unsafe { (*per).infdopen } {
        (unsafe { close((*per).infd) });
    }
    if (unsafe { (*config).synthetic_error }) as u64 == 0 && result as u32 != 0 && (unsafe { (*global).showerror }) != 0 {
        (unsafe { curl_mfprintf(
            (*global).errors,
            b"curl: (%d) %s\n\0" as *const u8 as *const i8,
            result as u32,
            if (*per).errorbuffer[0 as i32 as usize] as i32 != 0 {
                ((*per).errorbuffer).as_mut_ptr() as *const i8
            } else {
                curl_easy_strerror(result)
            },
        ) });
        if result as u32 == CURLE_PEER_FAILED_VERIFICATION as i32 as u32 {
            (unsafe { fputs (b"More details here: https://curl.se/docs/sslcerts.html\n\ncurl failed to verify the legitimacy of the server and therefore could not\nestablish a secure connection to it. To learn more about this situation and\nhow to fix it, please visit the web page mentioned above.\n\0" as * const u8 as * const i8 , (* global) . errors ,) }) ;
        }
    } else if unsafe { (*config).failwithbody } {
        let mut code: i64 = 0 as i32 as i64;
        (unsafe { curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &mut code as *mut i64) });
        if code >= 400 as i32 as i64 {
            if (unsafe { (*global).showerror }) != 0 {
                (unsafe { curl_mfprintf(
                    (*global).errors,
                    b"curl: (%d) The requested URL returned error: %ld\n\0" as *const u8
                        as *const i8,
                    CURLE_HTTP_RETURNED_ERROR as i32,
                    code,
                ) });
            }
            result = CURLE_HTTP_RETURNED_ERROR;
        }
    }
    if result as u64 == 0
        && (unsafe { (*config).xattr }) as i32 != 0
        && (*(borrow(&outs)).unwrap()).fopened as i32 != 0
        && !((*(borrow(&outs)).unwrap()).stream).is_null()
    {
        let mut rc: i32 = fwrite_xattr(curl, unsafe { fileno((*(borrow_mut(&mut outs)).unwrap()).stream) });
        if rc != 0 {
            (unsafe { warnf(
                (*config).global,
                b"Error setting extended attributes on '%s': %s\n\0" as *const u8 as *const i8,
                (*(borrow(&outs)).unwrap()).filename,
                strerror(*__errno_location()),
            ) });
        }
    }
    if result as u64 == 0
        && ((*(borrow(&outs)).unwrap()).stream).is_null()
        && (*(borrow(&outs)).unwrap()).bytes == 0
    {
        let mut cond_unmet: i64 = 0 as i64;
        (unsafe { curl_easy_getinfo(curl, CURLINFO_CONDITION_UNMET, &mut cond_unmet as *mut i64) });
        if cond_unmet == 0 && !tool_create_output_file(borrow_mut(&mut outs), config) {
            result = CURLE_WRITE_ERROR;
        }
    }
    if !(*(borrow(&outs)).unwrap()).s_isreg && !((*(borrow(&outs)).unwrap()).stream).is_null() {
        let mut rc_0: i32 = unsafe { fflush((*(borrow_mut(&mut outs)).unwrap()).stream) };
        if result as u64 == 0 && rc_0 != 0 {
            result = CURLE_WRITE_ERROR;
            if (unsafe { (*global).showerror }) != 0 {
                (unsafe { curl_mfprintf(
                    (*global).errors,
                    b"curl: (%d) Failed writing body\n\0" as *const u8 as *const i8,
                    result as u32,
                ) });
            }
        }
    }
    if (unsafe { (*per).retry_numretries }) != 0
        && ((unsafe { (*config).retry_maxtime }) == 0
            || tvdiff(tvnow(), unsafe { (*per).retrystart }) < (unsafe { (*config).retry_maxtime }) * 1000 as i64)
    {
        let mut retry: u32 = RETRY_NO;
        let mut response: i64 = 0 as i32 as i64;
        if CURLE_OPERATION_TIMEDOUT as i32 as u32 == result as u32
            || CURLE_COULDNT_RESOLVE_HOST as i32 as u32 == result as u32
            || CURLE_COULDNT_RESOLVE_PROXY as i32 as u32 == result as u32
            || CURLE_FTP_ACCEPT_TIMEOUT as i32 as u32 == result as u32
        {
            retry = RETRY_TIMEOUT;
        } else if (unsafe { (*config).retry_connrefused }) as i32 != 0
            && CURLE_COULDNT_CONNECT as i32 as u32 == result as u32
        {
            let mut oserrno: i64 = 0 as i32 as i64;
            (unsafe { curl_easy_getinfo(curl, CURLINFO_OS_ERRNO, &mut oserrno as *mut i64) });
            if 111 as i32 as i64 == oserrno {
                retry = RETRY_CONNREFUSED;
            }
        } else if CURLE_OK as i32 as u32 == result as u32
            || (unsafe { (*config).failonerror }) as i32 != 0
                && CURLE_HTTP_RETURNED_ERROR as i32 as u32 == result as u32
        {
            let mut protocol: i64 = 0 as i32 as i64;
            (unsafe { curl_easy_getinfo(curl, CURLINFO_PROTOCOL, &mut protocol as *mut i64) });
            if protocol == ((1 as i32) << 0 as i32) as i64
                || protocol == ((1 as i32) << 1 as i32) as i64
            {
                (unsafe { curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &mut response as *mut i64) });
                let mut current_block_42: u64;
                match response {
                    429 => {
                        current_block_42 = 6372292276806709886;
                    }
                    500 => {
                        current_block_42 = 6372292276806709886;
                    }
                    502 => {
                        current_block_42 = 9982339796614704152;
                    }
                    503 => {
                        current_block_42 = 1919708120026664077;
                    }
                    408 | 504 => {
                        current_block_42 = 2218155957277790543;
                    }
                    _ => {
                        current_block_42 = 5892776923941496671;
                    }
                }
                match current_block_42 {
                    6372292276806709886 => {
                        current_block_42 = 9982339796614704152;
                    }
                    _ => {}
                }
                match current_block_42 {
                    9982339796614704152 => {
                        current_block_42 = 1919708120026664077;
                    }
                    _ => {}
                }
                match current_block_42 {
                    1919708120026664077 => {
                        current_block_42 = 2218155957277790543;
                    }
                    _ => {}
                }
                match current_block_42 {
                    2218155957277790543 => {
                        retry = RETRY_HTTP;
                    }
                    _ => {}
                }
            }
        } else if result as u64 != 0 {
            let mut protocol_0: i64 = 0 as i32 as i64;
            (unsafe { curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &mut response as *mut i64) });
            (unsafe { curl_easy_getinfo(curl, CURLINFO_PROTOCOL, &mut protocol_0 as *mut i64) });
            if (protocol_0 == ((1 as i32) << 2 as i32) as i64
                || protocol_0 == ((1 as i32) << 3 as i32) as i64)
                && response / 100 as i32 as i64 == 4 as i32 as i64
            {
                retry = RETRY_FTP;
            }
        }
        if result as u32 != 0 && retry as u64 == 0 && (unsafe { (*config).retry_all_errors }) as i32 != 0 {
            retry = RETRY_ALL_ERRORS;
        }
        if retry as u64 != 0 {
            let mut sleeptime: i64 = 0 as i32 as i64;
            let mut retry_after: i64 = 0 as i32 as curl_off_t;
            static mut m: [*const i8; 6] = [
                0 as *const i8,
                b"(retrying all errors)\0" as *const u8 as *const i8,
                b": timeout\0" as *const u8 as *const i8,
                b": connection refused\0" as *const u8 as *const i8,
                b": HTTP error\0" as *const u8 as *const i8,
                b": FTP error\0" as *const u8 as *const i8,
            ];
            sleeptime = unsafe { (*per).retry_sleep };
            if RETRY_HTTP as i32 as u32 == retry as u32 {
                (unsafe { curl_easy_getinfo(
                    curl,
                    CURLINFO_RETRY_AFTER,
                    &mut retry_after as *mut curl_off_t,
                ) });
                if retry_after != 0 {
                    if retry_after > 9223372036854775807 as i64 / 1000 as i32 as i64 {
                        sleeptime = 9223372036854775807 as i64;
                    } else {
                        sleeptime = retry_after * 1000 as i32 as i64;
                    }
                    if (unsafe { (*config).retry_maxtime }) != 0 {
                        let mut seconds: i64 =
                            tvdiff(tvnow(), unsafe { (*per).retrystart }) / 1000 as i32 as i64;
                        if 0x7fffffffffffffff as i64 - retry_after < seconds
                            || seconds + retry_after > (unsafe { (*config).retry_maxtime })
                        {
                            (unsafe { warnf ((* config) . global , b"The Retry-After: time would make this command line exceed the maximum allowed time for retries.\0" as * const u8 as * const i8 ,) }) ;
                            current_block = 8481821627268246989;
                        } else {
                            current_block = 6033931424626438518;
                        }
                    } else {
                        current_block = 6033931424626438518;
                    }
                } else {
                    current_block = 6033931424626438518;
                }
            } else {
                current_block = 6033931424626438518;
            }
            match current_block {
                8481821627268246989 => {}
                _ => {
                    (unsafe { warnf(
                        (*config).global,
                        b"Problem %s. Will retry in %ld seconds. %ld retries left.\n\0" as *const u8
                            as *const i8,
                        m[retry as usize],
                        sleeptime / 1000 as i64,
                        (*per).retry_numretries,
                    ) });
                    let mut fresh4 = unsafe { &mut ((*per).retry_numretries) };
                    *fresh4 -= 1;
                    if (unsafe { (*config).retry_delay }) == 0 {
                        (unsafe { (*per).retry_sleep *= 2 as i32 as i64 });
                        if (unsafe { (*per).retry_sleep }) > 600000 as i64 {
                            (unsafe { (*per).retry_sleep = 600000 as i64 });
                        }
                    }
                    if (*(borrow(&outs)).unwrap()).bytes != 0
                        && !((*(borrow(&outs)).unwrap()).filename).is_null()
                        && !((*(borrow(&outs)).unwrap()).stream).is_null()
                    {
                        let mut rc_1: i32 = 0;
                        if !(unsafe { (*global).mute }) {
                            (unsafe { curl_mfprintf(
                                (*global).errors,
                                b"Throwing away %ld bytes\n\0" as *const u8 as *const i8,
                                (*(borrow(&outs)).unwrap()).bytes,
                            ) });
                        }
                        (unsafe { fflush((*(borrow_mut(&mut outs)).unwrap()).stream) });
                        if (unsafe { ftruncate(
                            fileno((*(borrow_mut(&mut outs)).unwrap()).stream),
                            (*(borrow_mut(&mut outs)).unwrap()).init,
                        ) }) != 0
                        {
                            if (unsafe { (*global).showerror }) != 0 {
                                (unsafe { curl_mfprintf(
                                    (*global).errors,
                                    b"curl: (23) Failed to truncate file\n\0" as *const u8
                                        as *const i8,
                                ) });
                            }
                            return CURLE_WRITE_ERROR;
                        }
                        rc_1 = unsafe { fseek(
                            (*(borrow_mut(&mut outs)).unwrap()).stream,
                            0 as i32 as i64,
                            2 as i32,
                        ) };
                        if rc_1 != 0 {
                            if (unsafe { (*global).showerror }) != 0 {
                                (unsafe { curl_mfprintf(
                                    (*global).errors,
                                    b"curl: (23) Failed seeking to end of file\n\0" as *const u8
                                        as *const i8,
                                ) });
                            }
                            return CURLE_WRITE_ERROR;
                        }
                        (*(borrow_mut(&mut outs)).unwrap()).bytes = 0 as i32 as curl_off_t;
                    }
                    *(borrow_mut(&mut retryp)).unwrap() = 1 as i32 != 0;
                    *(borrow_mut(&mut delay)).unwrap() = sleeptime;
                    return CURLE_OK;
                }
            }
        }
    }
    if (unsafe { (*global).progressmode }) == 1 as i32 && (unsafe { (*per).progressbar.calls }) != 0 {
        (unsafe { fputs(b"\n\0" as *const u8 as *const i8, (*per).progressbar.out) });
    }
    if (*(borrow(&outs)).unwrap()).fopened as i32 != 0
        && !((*(borrow(&outs)).unwrap()).stream).is_null()
    {
        let mut rc_2: i32 = unsafe { fclose((*(borrow_mut(&mut outs)).unwrap()).stream) };
        if result as u64 == 0 && rc_2 != 0 {
            result = CURLE_WRITE_ERROR;
            if (unsafe { (*global).showerror }) != 0 {
                (unsafe { curl_mfprintf(
                    (*global).errors,
                    b"curl: (%d) Failed writing body\n\0" as *const u8 as *const i8,
                    result as u32,
                ) });
            }
        }
    }
    if result as u64 == 0
        && (unsafe { (*config).remote_time }) as i32 != 0
        && (*(borrow(&outs)).unwrap()).s_isreg as i32 != 0
        && !((*(borrow(&outs)).unwrap()).filename).is_null()
    {
        let mut filetime: i64 = -(1 as i32) as curl_off_t;
        (unsafe { curl_easy_getinfo(curl, CURLINFO_FILETIME_T, &mut filetime as *mut curl_off_t) });
        setfiletime(filetime, (*(borrow(&outs)).unwrap()).filename, global);
    }
    if !(unsafe { (*config).writeout }).is_null() {
        ourWriteOut(unsafe { (*config).writeout }, per, result);
    }
    if (unsafe { (*per).heads.fopened }) as i32 != 0 && !(unsafe { (*per).heads.stream }).is_null() {
        (unsafe { fclose((*per).heads.stream) });
    }
    if unsafe { (*per).heads.alloc_filename } {
        (unsafe { free((*per).heads.filename as *mut libc::c_void) });
        let mut fresh5 = unsafe { &mut ((*per).heads.filename) };
        *fresh5 = 0 as *mut i8;
    }
    if (unsafe { (*per).etag_save.fopened }) as i32 != 0 && !(unsafe { (*per).etag_save.stream }).is_null() {
        (unsafe { fclose((*per).etag_save.stream) });
    }
    if unsafe { (*per).etag_save.alloc_filename } {
        (unsafe { free((*per).etag_save.filename as *mut libc::c_void) });
        let mut fresh6 = unsafe { &mut ((*per).etag_save.filename) };
        *fresh6 = 0 as *mut i8;
    }
    curl_easy_cleanup(unsafe { (*per).curl });
    if (*(borrow(&outs)).unwrap()).alloc_filename {
        (unsafe { free((*(borrow_mut(&mut outs)).unwrap()).filename as *mut libc::c_void) });
    }
    (unsafe { free((*per).this_url as *mut libc::c_void) });
    (unsafe { free((*per).separator_err as *mut libc::c_void) });
    (unsafe { free((*per).separator as *mut libc::c_void) });
    (unsafe { free((*per).outfile as *mut libc::c_void) });
    (unsafe { free((*per).uploadfile as *mut libc::c_void) });
    return result;
}
extern "C" fn single_transfer_cleanup(
    mut config: *mut crate::src::src::tool_cb_dbg::OperationConfig,
) {
    if !config.is_null() {
        let mut state: Option<&'_ mut crate::src::src::tool_cb_dbg::State> =
            Some(unsafe { &mut (*config).state });
        if !((*(borrow_mut(&mut state)).unwrap()).urls).is_null() {
            glob_cleanup((*(borrow_mut(&mut state)).unwrap()).urls);
            let mut fresh7 = &mut ((*(borrow_mut(&mut state)).unwrap()).urls);
            *fresh7 = 0 as *mut URLGlob;
        }
        (unsafe { free((*(borrow_mut(&mut state)).unwrap()).outfiles as *mut libc::c_void) });
        let mut fresh8 = &mut ((*(borrow_mut(&mut state)).unwrap()).outfiles);
        *fresh8 = 0 as *mut i8;
        (unsafe { free((*(borrow_mut(&mut state)).unwrap()).httpgetfields as *mut libc::c_void) });
        let mut fresh9 = &mut ((*(borrow_mut(&mut state)).unwrap()).httpgetfields);
        *fresh9 = 0 as *mut i8;
        (unsafe { free((*(borrow_mut(&mut state)).unwrap()).uploadfile as *mut libc::c_void) });
        let mut fresh10 = &mut ((*(borrow_mut(&mut state)).unwrap()).uploadfile);
        *fresh10 = 0 as *mut i8;
        if !((*(borrow_mut(&mut state)).unwrap()).inglob).is_null() {
            glob_cleanup((*(borrow_mut(&mut state)).unwrap()).inglob);
            let mut fresh11 = &mut ((*(borrow_mut(&mut state)).unwrap()).inglob);
            *fresh11 = 0 as *mut URLGlob;
        }
    }
}
extern "C" fn single_transfer<'a1>(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut config: *mut crate::src::src::tool_cb_dbg::OperationConfig,
    mut share: *mut crate::src::lib::asyn_thread::Curl_share,
    mut capath_from_env: bool,
    mut added: Option<&'a1 mut bool>,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut urlnode: *mut crate::src::src::tool_cb_dbg::getout = 0 as *mut getout;
    let mut orig_noprogress: bool = unsafe { (*global).noprogress };
    let mut orig_isatty: bool = unsafe { (*global).isatty };
    let mut state: *mut crate::src::src::tool_cb_dbg::State = unsafe { &mut (*config).state };
    let mut httpgetfields: *mut i8 = unsafe { (*state).httpgetfields };
    *(borrow_mut(&mut added)).unwrap() = 0 as i32 != 0;
    if !(unsafe { (*config).postfields }).is_null() {
        if unsafe { (*config).use_httpget } {
            if httpgetfields.is_null() {
                let mut fresh12 = unsafe { &mut ((*state).httpgetfields) };
                *fresh12 = unsafe { strdup((*config).postfields) };
                httpgetfields = *fresh12;
                (unsafe { free((*config).postfields as *mut libc::c_void) });
                let mut fresh13 = unsafe { &mut ((*config).postfields) };
                *fresh13 = 0 as *mut i8;
                if httpgetfields.is_null() {
                    (unsafe { errorf(global, b"out of memory\n\0" as *const u8 as *const i8) });
                    result = CURLE_OUT_OF_MEMORY;
                } else if SetHTTPrequest(
                    config,
                    (if (unsafe { (*config).no_body }) as i32 != 0 {
                        HTTPREQ_HEAD as i32
                    } else {
                        HTTPREQ_GET as i32
                    }) as HttpReq,
                    Some(unsafe { &mut (*config).httpreq }),
                ) != 0
                {
                    result = CURLE_FAILED_INIT;
                }
            }
        } else if SetHTTPrequest(config, HTTPREQ_SIMPLEPOST, Some(unsafe { &mut (*config).httpreq })) != 0 {
            result = CURLE_FAILED_INIT;
        }
        if result as u64 != 0 {
            single_transfer_cleanup(config);
            return result;
        }
    }
    if (unsafe { (*state).urlnode }).is_null() {
        let mut fresh14 = unsafe { &mut ((*state).urlnode) };
        *fresh14 = unsafe { (*config).url_list };
        (unsafe { (*state).infilenum = 1 as i32 as u64 });
    }
    while !(unsafe { (*config).state.urlnode }).is_null() {
        let mut infiles: *mut i8 = 0 as *mut i8;
        let mut inglob: *mut crate::src::src::tool_cb_dbg::URLGlob = unsafe { (*state).inglob };
        urlnode = unsafe { (*config).state.urlnode };
        if (unsafe { (*urlnode).url }).is_null() {
            (unsafe { free((*urlnode).outfile as *mut libc::c_void) });
            let mut fresh15 = unsafe { &mut ((*urlnode).outfile) };
            *fresh15 = 0 as *mut i8;
            (unsafe { free((*urlnode).infile as *mut libc::c_void) });
            let mut fresh16 = unsafe { &mut ((*urlnode).infile) };
            *fresh16 = 0 as *mut i8;
            (unsafe { (*urlnode).flags = 0 as i32 });
            let mut fresh17 = unsafe { &mut ((*config).state.urlnode) };
            *fresh17 = unsafe { (*urlnode).next };
            (unsafe { (*state).up = 0 as i32 as u64 });
        } else {
            if !(unsafe { (*urlnode).outfile }).is_null() && (unsafe { (*state).outfiles }).is_null() {
                let mut fresh18 = unsafe { &mut ((*state).outfiles) };
                *fresh18 = unsafe { strdup((*urlnode).outfile) };
                if (unsafe { (*state).outfiles }).is_null() {
                    (unsafe { errorf(global, b"out of memory\n\0" as *const u8 as *const i8) });
                    result = CURLE_OUT_OF_MEMORY;
                    break;
                }
            }
            infiles = unsafe { (*urlnode).infile };
            if !(unsafe { (*config).globoff }) && !infiles.is_null() && inglob.is_null() {
                result = glob_url(
                    Some(&mut inglob),
                    infiles,
                    Some(unsafe { &mut (*state).infilenum }),
                    if (unsafe { (*global).showerror }) != 0 {
                        unsafe { (*global).errors }
                    } else {
                        0 as *mut FILE
                    },
                );
                if result as u64 != 0 {
                    break;
                }
                let mut fresh19 = unsafe { &mut ((*config).state.inglob) };
                *fresh19 = inglob;
            }
            let mut separator: i32 = 0;
            let mut urlnum: u64 = 0;
            if !((unsafe { (*state).up }) == 0 && infiles.is_null()) {
                if (unsafe { (*state).uploadfile }).is_null() {
                    if !inglob.is_null() {
                        result = glob_next_url(Some(unsafe { &mut (*state).uploadfile }), inglob);
                        if result as u32 == CURLE_OUT_OF_MEMORY as i32 as u32 {
                            (unsafe { errorf(global, b"out of memory\n\0" as *const u8 as *const i8) });
                        }
                    } else if (unsafe { (*state).up }) == 0 {
                        let mut fresh20 = unsafe { &mut ((*state).uploadfile) };
                        *fresh20 = unsafe { strdup(infiles) };
                        if (unsafe { (*state).uploadfile }).is_null() {
                            (unsafe { errorf(global, b"out of memory\n\0" as *const u8 as *const i8) });
                            result = CURLE_OUT_OF_MEMORY;
                        }
                    }
                }
                if result as u64 != 0 {
                    break;
                }
            }
            if (unsafe { (*state).urlnum }) == 0 {
                if !(unsafe { (*config).globoff }) {
                    result = glob_url(
                        Some(unsafe { &mut (*state).urls }),
                        unsafe { (*urlnode).url },
                        Some(unsafe { &mut (*state).urlnum }),
                        if (unsafe { (*global).showerror }) != 0 {
                            unsafe { (*global).errors }
                        } else {
                            0 as *mut FILE
                        },
                    );
                    if result as u64 != 0 {
                        break;
                    }
                    urlnum = unsafe { (*state).urlnum };
                } else {
                    urlnum = 1 as i32 as u64;
                }
            } else {
                urlnum = unsafe { (*state).urlnum };
            }
            separator = (((unsafe { (*state).outfiles }).is_null()
                || (unsafe { strcmp((*state).outfiles, b"-\0" as *const u8 as *const i8) }) == 0)
                && urlnum > 1 as i32 as u64) as i32;
            if (unsafe { (*state).up }) < (unsafe { (*state).infilenum }) {
                let mut per: *mut crate::src::src::tool_cb_hdr::per_transfer<'_> =
                    0 as *mut per_transfer;
                let mut outs: Option<&'_ mut crate::src::src::tool_cb_hdr::OutStruct> =
                    Option::<&'_ mut crate::src::src::tool_cb_hdr::OutStruct>::None;
                let mut input: *mut crate::src::src::tool_cb_hdr::InStruct = 0 as *mut InStruct;
                let mut heads: Option<&'_ mut crate::src::src::tool_cb_hdr::OutStruct> =
                    Option::<&'_ mut crate::src::src::tool_cb_hdr::OutStruct>::None;
                let mut etag_save: Option<&'_ mut crate::src::src::tool_cb_hdr::OutStruct> =
                    Option::<&'_ mut crate::src::src::tool_cb_hdr::OutStruct>::None;
                let mut hdrcbdata: Option<&'_ mut crate::src::src::tool_cb_hdr::HdrCbData<'_>> =
                    Option::<&'_ mut crate::src::src::tool_cb_hdr::HdrCbData<'_>>::None;
                let mut curl: *mut crate::src::lib::http2::Curl_easy = curl_easy_init();
                result = add_per_transfer(&mut per);
                if result as u32 != 0 || curl.is_null() {
                    curl_easy_cleanup(curl);
                    result = CURLE_OUT_OF_MEMORY;
                    break;
                } else {
                    if !(unsafe { (*state).uploadfile }).is_null() {
                        let mut fresh21 = unsafe { &mut ((*per).uploadfile) };
                        *fresh21 = unsafe { strdup((*state).uploadfile) };
                        if (unsafe { (*per).uploadfile }).is_null() {
                            curl_easy_cleanup(curl);
                            result = CURLE_OUT_OF_MEMORY;
                            break;
                        }
                    }
                    *(borrow_mut(&mut added)).unwrap() = 1 as i32 != 0;
                    let mut fresh22 = unsafe { &mut ((*per).config) };
                    *fresh22 = config;
                    let mut fresh23 = unsafe { &mut ((*per).curl) };
                    *fresh23 = curl;
                    (unsafe { (*per).urlnum = (*urlnode).num as u32 });
                    heads = Some(unsafe { &mut (*per).heads });
                    let mut fresh24 = &mut ((*(borrow_mut(&mut heads)).unwrap()).stream);
                    *fresh24 = unsafe { stdout };
                    if !(unsafe { (*config).headerfile }).is_null() {
                        if (unsafe { strcmp((*config).headerfile, b"-\0" as *const u8 as *const i8) }) != 0 {
                            let mut newfile: *mut crate::src::lib::http2::_IO_FILE = 0 as *mut FILE;
                            newfile = unsafe { fopen(
                                (*config).headerfile,
                                if ((*per).prev).is_null() {
                                    b"wb\0" as *const u8 as *const i8
                                } else {
                                    b"ab\0" as *const u8 as *const i8
                                },
                            ) };
                            if newfile.is_null() {
                                (unsafe { warnf(
                                    global,
                                    b"Failed to open %s\n\0" as *const u8 as *const i8,
                                    (*config).headerfile,
                                ) });
                                result = CURLE_WRITE_ERROR;
                                break;
                            } else {
                                let mut fresh25 =
                                    &mut ((*(borrow_mut(&mut heads)).unwrap()).filename);
                                *fresh25 = unsafe { (*config).headerfile };
                                (*(borrow_mut(&mut heads)).unwrap()).s_isreg = 1 as i32 != 0;
                                (*(borrow_mut(&mut heads)).unwrap()).fopened = 1 as i32 != 0;
                                let mut fresh26 =
                                    &mut ((*(borrow_mut(&mut heads)).unwrap()).stream);
                                *fresh26 = newfile;
                            }
                        }
                    }
                    hdrcbdata = Some(unsafe { &mut (*per).hdrcbdata });
                    outs = Some(unsafe { &mut (*per).outs });
                    input = unsafe { &mut (*per).input };
                    let mut fresh27 = unsafe { &mut ((*per).outfile) };
                    *fresh27 = 0 as *mut i8;
                    (unsafe { (*per).infdopen = 0 as i32 != 0 });
                    (unsafe { (*per).infd = 0 as i32 });
                    let mut fresh28 = &mut ((*(borrow_mut(&mut outs)).unwrap()).stream);
                    *fresh28 = unsafe { stdout };
                    if !(unsafe { (*config).etag_compare_file }).is_null() {
                        let mut etag_from_file: *mut i8 = 0 as *mut i8;
                        let mut header: *mut i8 = 0 as *mut i8;
                        let mut file: *mut crate::src::lib::http2::_IO_FILE = unsafe { fopen(
                            (*config).etag_compare_file,
                            b"r\0" as *const u8 as *const i8,
                        ) };
                        if file.is_null() && (unsafe { (*config).etag_save_file }).is_null() {
                            (unsafe { errorf(
                                global,
                                b"Failed to open %s\n\0" as *const u8 as *const i8,
                                (*config).etag_compare_file,
                            ) });
                            result = CURLE_READ_ERROR;
                            break;
                        } else {
                            if PARAM_OK as i32 as u32
                                == file2string(Some(&mut etag_from_file), file) as u32
                                && !etag_from_file.is_null()
                            {
                                header = unsafe { curl_maprintf(
                                    b"If-None-Match: %s\0" as *const u8 as *const i8,
                                    etag_from_file,
                                ) };
                                (unsafe { free(etag_from_file as *mut libc::c_void) });
                                etag_from_file = 0 as *mut i8;
                            } else {
                                header = unsafe { curl_maprintf(
                                    b"If-None-Match: \"\"\0" as *const u8 as *const i8,
                                ) };
                            }
                            if header.is_null() {
                                if !file.is_null() {
                                    (unsafe { fclose(file) });
                                }
                                (unsafe { errorf(
                                    global,
                                    b"Failed to allocate memory for custom etag header\n\0"
                                        as *const u8
                                        as *const i8,
                                ) });
                                result = CURLE_OUT_OF_MEMORY;
                                break;
                            } else {
                                add2list(Some(unsafe { &mut (*config).headers }), header);
                                (unsafe { free(header as *mut libc::c_void) });
                                header = 0 as *mut i8;
                                if !file.is_null() {
                                    (unsafe { fclose(file) });
                                }
                            }
                        }
                    }
                    etag_save = Some(unsafe { &mut (*per).etag_save });
                    let mut fresh29 = &mut ((*(borrow_mut(&mut etag_save)).unwrap()).stream);
                    *fresh29 = unsafe { stdout };
                    if !(unsafe { (*config).etag_save_file }).is_null() {
                        if (unsafe { strcmp((*config).etag_save_file, b"-\0" as *const u8 as *const i8) }) != 0 {
                            let mut newfile_0: *mut crate::src::lib::http2::_IO_FILE =
                                unsafe { fopen((*config).etag_save_file, b"wb\0" as *const u8 as *const i8) };
                            if newfile_0.is_null() {
                                (unsafe { warnf(
                                    global,
                                    b"Failed to open %s\n\0" as *const u8 as *const i8,
                                    (*config).etag_save_file,
                                ) });
                                result = CURLE_WRITE_ERROR;
                                break;
                            } else {
                                let mut fresh30 =
                                    &mut ((*(borrow_mut(&mut etag_save)).unwrap()).filename);
                                *fresh30 = unsafe { (*config).etag_save_file };
                                (*(borrow_mut(&mut etag_save)).unwrap()).s_isreg = 1 as i32 != 0;
                                (*(borrow_mut(&mut etag_save)).unwrap()).fopened = 1 as i32 != 0;
                                let mut fresh31 =
                                    &mut ((*(borrow_mut(&mut etag_save)).unwrap()).stream);
                                *fresh31 = newfile_0;
                            }
                        }
                    }
                    if !(unsafe { (*state).urls }).is_null() {
                        result = glob_next_url(Some(unsafe { &mut (*per).this_url }), unsafe { (*state).urls });
                        if result as u64 != 0 {
                            break;
                        }
                    } else if (unsafe { (*state).li }) == 0 {
                        let mut fresh32 = unsafe { &mut ((*per).this_url) };
                        *fresh32 = unsafe { strdup((*urlnode).url) };
                        if (unsafe { (*per).this_url }).is_null() {
                            result = CURLE_OUT_OF_MEMORY;
                            break;
                        }
                    } else {
                        let mut fresh33 = unsafe { &mut ((*per).this_url) };
                        *fresh33 = 0 as *mut i8;
                    }
                    if (unsafe { (*per).this_url }).is_null() {
                        break;
                    }
                    if !(unsafe { (*state).outfiles }).is_null() {
                        let mut fresh34 = unsafe { &mut ((*per).outfile) };
                        *fresh34 = unsafe { strdup((*state).outfiles) };
                        if (unsafe { (*per).outfile }).is_null() {
                            result = CURLE_OUT_OF_MEMORY;
                            break;
                        }
                    }
                    if (unsafe { (*urlnode).flags }) & (1 as i32) << 2 as i32 != 0
                        || !(unsafe { (*per).outfile }).is_null()
                            && (unsafe { strcmp(b"-\0" as *const u8 as *const i8, (*per).outfile) }) != 0
                    {
                        if (unsafe { (*per).outfile }).is_null() {
                            result = get_url_file_name(Some(unsafe { &mut (*per).outfile }), unsafe { (*per).this_url });
                            if result as u64 != 0 {
                                (unsafe { errorf (global , b"Failed to extract a sensible file name from the URL to use for storage!\n\0" as * const u8 as * const i8 ,) }) ;
                                break;
                            } else if (unsafe { *(*per).outfile }) == 0 && !(unsafe { (*config).content_disposition }) {
                                (unsafe { errorf(
                                    global,
                                    b"Remote file name has no length!\n\0" as *const u8
                                        as *const i8,
                                ) });
                                result = CURLE_WRITE_ERROR;
                                break;
                            }
                        } else if !(unsafe { (*state).urls }).is_null() {
                            let mut storefile: *mut i8 = unsafe { (*per).outfile };
                            result =
                                glob_match_url(Some(unsafe { &mut (*per).outfile }), storefile, unsafe { (*state).urls });
                            (unsafe { free(storefile as *mut libc::c_void) });
                            storefile = 0 as *mut i8;
                            if result as u64 != 0 {
                                (unsafe { warnf(global, b"bad output glob!\n\0" as *const u8 as *const i8) });
                                break;
                            }
                        }
                        if !(unsafe { (*config).output_dir }).is_null() && (unsafe { *(*config).output_dir }) as i32 != 0 {
                            let mut d: *mut i8 = unsafe { curl_maprintf(
                                b"%s/%s\0" as *const u8 as *const i8,
                                (*config).output_dir,
                                (*per).outfile,
                            ) };
                            if d.is_null() {
                                result = CURLE_WRITE_ERROR;
                                break;
                            } else {
                                (unsafe { free((*per).outfile as *mut libc::c_void) });
                                let mut fresh35 = unsafe { &mut ((*per).outfile) };
                                *fresh35 = d;
                            }
                        }
                        if unsafe { (*config).create_dirs } {
                            result = create_dir_hierarchy(unsafe { (*per).outfile }, unsafe { (*global).errors });
                            if result as u64 != 0 {
                                break;
                            }
                        }
                        let _ = (unsafe { (*urlnode).flags }) & (1 as i32) << 2 as i32 != 0
                            && (unsafe { (*config).content_disposition }) as i32 != 0;
                        if unsafe { (*config).resume_from_current } {
                            let mut fileinfo: crate::src::lib::file::stat = stat {
                                st_dev: 0,
                                st_ino: 0,
                                st_nlink: 0,
                                st_mode: 0,
                                st_uid: 0,
                                st_gid: 0,
                                __pad0: 0,
                                st_rdev: 0,
                                st_size: 0,
                                st_blksize: 0,
                                st_blocks: 0,
                                st_atim: timespec {
                                    tv_sec: 0,
                                    tv_nsec: 0,
                                },
                                st_mtim: timespec {
                                    tv_sec: 0,
                                    tv_nsec: 0,
                                },
                                st_ctim: timespec {
                                    tv_sec: 0,
                                    tv_nsec: 0,
                                },
                                __glibc_reserved: [0; 3],
                            };
                            if 0 as i32 == stat(unsafe { (*per).outfile }, &mut fileinfo) {
                                (unsafe { (*config).resume_from = fileinfo.st_size });
                            } else {
                                (unsafe { (*config).resume_from = 0 as i32 as curl_off_t });
                            }
                        }
                        if (unsafe { (*config).resume_from }) != 0 {
                            let mut file_0: *mut crate::src::lib::http2::_IO_FILE =
                                unsafe { fopen((*per).outfile, b"ab\0" as *const u8 as *const i8) };
                            if file_0.is_null() {
                                (unsafe { errorf(
                                    global,
                                    b"Can't open '%s'!\n\0" as *const u8 as *const i8,
                                    (*per).outfile,
                                ) });
                                result = CURLE_WRITE_ERROR;
                                break;
                            } else {
                                (*(borrow_mut(&mut outs)).unwrap()).fopened = 1 as i32 != 0;
                                let mut fresh36 = &mut ((*(borrow_mut(&mut outs)).unwrap()).stream);
                                *fresh36 = file_0;
                                (*(borrow_mut(&mut outs)).unwrap()).init = unsafe { (*config).resume_from };
                            }
                        } else {
                            let mut fresh37 = &mut ((*(borrow_mut(&mut outs)).unwrap()).stream);
                            *fresh37 = 0 as *mut FILE;
                        }
                        let mut fresh38 = &mut ((*(borrow_mut(&mut outs)).unwrap()).filename);
                        *fresh38 = unsafe { (*per).outfile };
                        (*(borrow_mut(&mut outs)).unwrap()).s_isreg = 1 as i32 != 0;
                    }
                    if !(unsafe { (*per).uploadfile }).is_null() && !stdin_upload(unsafe { (*per).uploadfile }) {
                        let mut nurl: *mut i8 =
                            add_file_name_to_url(unsafe { (*per).this_url }, unsafe { (*per).uploadfile });
                        if nurl.is_null() {
                            result = CURLE_OUT_OF_MEMORY;
                            break;
                        } else {
                            let mut fresh39 = unsafe { &mut ((*per).this_url) };
                            *fresh39 = nurl;
                        }
                    } else if !(unsafe { (*per).uploadfile }).is_null()
                        && stdin_upload(unsafe { (*per).uploadfile }) as i32 != 0
                    {
                        let mut authbits: i32 = 0 as i32;
                        let mut bitcheck: i32 = 0 as i32;
                        while bitcheck < 32 as i32 {
                            let mut fresh40 = bitcheck;
                            bitcheck = bitcheck + 1;
                            if !((unsafe { (*config).authtype }) & (1 as u64) << fresh40 != 0) {
                                continue;
                            }
                            authbits += 1;
                            if authbits > 1 as i32 {
                                break;
                            }
                        }
                        if (unsafe { (*config).proxyanyauth }) as i32 != 0 || authbits > 1 as i32 {
                            (unsafe { warnf (global , b"Using --anyauth or --proxy-anyauth with upload from stdin involves a big risk of it not working. Use a temporary file or a fixed auth type instead!\n\0" as * const u8 as * const i8 ,) }) ;
                        }
                        if (unsafe { strcmp((*per).uploadfile, b".\0" as *const u8 as *const i8) }) == 0 {
                            if curlx_nonblock(unsafe { (*per).infd }, 1 as i32) < 0 as i32 {
                                (unsafe { warnf(
                                    global,
                                    b"fcntl failed on fd=%d: %s\n\0" as *const u8 as *const i8,
                                    (*per).infd,
                                    strerror(*__errno_location()),
                                ) });
                            }
                        }
                    }
                    if !(unsafe { (*per).uploadfile }).is_null() && (unsafe { (*config).resume_from_current }) as i32 != 0 {
                        (unsafe { (*config).resume_from = -(1 as i32) as curl_off_t });
                    }
                    if output_expected(unsafe { (*per).this_url }, unsafe { (*per).uploadfile }) as i32 != 0
                        && !((*(borrow(&outs)).unwrap()).stream).is_null()
                        && (unsafe { isatty(fileno((*(borrow_mut(&mut outs)).unwrap()).stream)) }) != 0
                    {
                        let mut fresh41 = unsafe { &mut ((*global).isatty) };
                        *fresh41 = 1 as i32 != 0;
                        let mut fresh42 = unsafe { &mut ((*global).noprogress) };
                        *fresh42 = *fresh41;
                        (unsafe { (*per).noprogress = *fresh42 });
                    } else {
                        let mut fresh43 = unsafe { &mut ((*global).noprogress) };
                        *fresh43 = orig_noprogress;
                        (unsafe { (*per).noprogress = *fresh43 });
                        (unsafe { (*global).isatty = orig_isatty });
                    }
                    if urlnum > 1 as i32 as u64 && !(unsafe { (*global).mute }) {
                        let mut fresh44 = unsafe { &mut ((*per).separator_err) };
                        *fresh44 = unsafe { curl_maprintf(
                            b"\n[%lu/%lu]: %s --> %s\0" as *const u8 as *const i8,
                            ((*state).li).wrapping_add(1 as i32 as u64),
                            urlnum,
                            (*per).this_url,
                            if !((*per).outfile).is_null() {
                                (*per).outfile as *const i8
                            } else {
                                b"<stdout>\0" as *const u8 as *const i8
                            },
                        ) };
                        if separator != 0 {
                            let mut fresh45 = unsafe { &mut ((*per).separator) };
                            *fresh45 = unsafe { curl_maprintf(
                                b"%s%s\0" as *const u8 as *const i8,
                                b"--_curl_--\0" as *const u8 as *const i8,
                                (*per).this_url,
                            ) };
                        }
                    }
                    if !httpgetfields.is_null() {
                        let mut urlbuffer: *mut i8 = 0 as *mut i8;
                        let mut pc: *const i8 =
                            unsafe { strstr((*per).this_url, b"://\0" as *const u8 as *const i8) };
                        let mut sep: i8 = '?' as i32 as i8;
                        if !pc.is_null() {
                            pc = unsafe { pc.offset(3 as i32 as isize) };
                        } else {
                            pc = unsafe { (*per).this_url };
                        }
                        pc = unsafe { strrchr(pc, '/' as i32) };
                        if !pc.is_null() {
                            if !(unsafe { strchr(pc, '?' as i32) }).is_null() {
                                sep = '&' as i32 as i8;
                            }
                        }
                        if !pc.is_null() {
                            urlbuffer = unsafe { curl_maprintf(
                                b"%s%c%s\0" as *const u8 as *const i8,
                                (*per).this_url,
                                sep as i32,
                                httpgetfields,
                            ) };
                        } else {
                            urlbuffer = unsafe { curl_maprintf(
                                b"%s/?%s\0" as *const u8 as *const i8,
                                (*per).this_url,
                                httpgetfields,
                            ) };
                        }
                        if urlbuffer.is_null() {
                            result = CURLE_OUT_OF_MEMORY;
                            break;
                        } else {
                            (unsafe { free((*per).this_url as *mut libc::c_void) });
                            let mut fresh46 = unsafe { &mut ((*per).this_url) };
                            *fresh46 = 0 as *mut i8;
                            let mut fresh47 = unsafe { &mut ((*per).this_url) };
                            *fresh47 = urlbuffer;
                        }
                    }
                    if (unsafe { (*global).errors }).is_null() {
                        let mut fresh48 = unsafe { &mut ((*global).errors) };
                        *fresh48 = unsafe { stderr };
                    }
                    let _ = ((unsafe { (*per).outfile }).is_null()
                        || (unsafe { strcmp((*per).outfile, b"-\0" as *const u8 as *const i8) }) == 0)
                        && !(unsafe { (*config).use_ascii });
                    (unsafe { (*config).terminal_binary_ok = !((*per).outfile).is_null()
                        && strcmp((*per).outfile, b"-\0" as *const u8 as *const i8) == 0 });
                    result = unsafe { curl_easy_setopt(curl, CURLOPT_SHARE, share) };
                    if result as u64 != 0 {
                        break;
                    }
                    if !(unsafe { (*config).tcp_nodelay }) {
                        if !tool_setopt_skip(CURLOPT_TCP_NODELAY) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_TCP_NODELAY\0" as *const u8 as *const i8,
                                CURLOPT_TCP_NODELAY,
                                0 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).tcp_fastopen } {
                        if !tool_setopt_skip(CURLOPT_TCP_FASTOPEN) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_TCP_FASTOPEN\0" as *const u8 as *const i8,
                                CURLOPT_TCP_FASTOPEN,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_WRITEDATA) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_WRITEDATA\0" as *const u8 as *const i8,
                            CURLOPT_WRITEDATA,
                            per,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_INTERLEAVEDATA) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_INTERLEAVEDATA\0" as *const u8 as *const i8,
                            CURLOPT_INTERLEAVEDATA,
                            per,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_WRITEFUNCTION) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_WRITEFUNCTION\0" as *const u8 as *const i8,
                            CURLOPT_WRITEFUNCTION,
                            Some(tool_write_cb),
                        ) };
                        let _ = result as u64 != 0;
                    }
                    let mut fresh49 = unsafe { &mut ((*input).config) };
                    *fresh49 = config;
                    if !tool_setopt_skip(CURLOPT_READDATA) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_READDATA\0" as *const u8 as *const i8,
                            CURLOPT_READDATA,
                            input,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_READFUNCTION) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_READFUNCTION\0" as *const u8 as *const i8,
                            CURLOPT_READFUNCTION,
                            Some(tool_read_cb),
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_SEEKDATA) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_SEEKDATA\0" as *const u8 as *const i8,
                            CURLOPT_SEEKDATA,
                            input,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_SEEKFUNCTION) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_SEEKFUNCTION\0" as *const u8 as *const i8,
                            CURLOPT_SEEKFUNCTION,
                            Some(tool_seek_cb),
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if (unsafe { (*config).recvpersecond }) != 0
                        && (unsafe { (*config).recvpersecond }) < (100 as i32 * 1024 as i32) as i64
                    {
                        if !tool_setopt_skip(CURLOPT_BUFFERSIZE) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_BUFFERSIZE\0" as *const u8 as *const i8,
                                CURLOPT_BUFFERSIZE,
                                (*config).recvpersecond,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    } else if !tool_setopt_skip(CURLOPT_BUFFERSIZE) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_BUFFERSIZE\0" as *const u8 as *const i8,
                            CURLOPT_BUFFERSIZE,
                            (100 as i32 * 1024 as i32) as i64,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_URL) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_URL\0" as *const u8 as *const i8,
                            CURLOPT_URL,
                            (*per).this_url,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_NOPROGRESS) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_NOPROGRESS\0" as *const u8 as *const i8,
                            CURLOPT_NOPROGRESS,
                            if (*global).noprogress as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if unsafe { (*config).no_body } {
                        if !tool_setopt_skip(CURLOPT_NOBODY) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_NOBODY\0" as *const u8 as *const i8,
                                CURLOPT_NOBODY,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).oauth_bearer }).is_null() {
                        if !tool_setopt_skip(CURLOPT_XOAUTH2_BEARER) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_XOAUTH2_BEARER\0" as *const u8 as *const i8,
                                CURLOPT_XOAUTH2_BEARER,
                                (*config).oauth_bearer,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_PROXY) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_PROXY\0" as *const u8 as *const i8,
                            CURLOPT_PROXY,
                            (*config).proxy,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !(unsafe { (*config).proxy }).is_null() {
                        if !tool_setopt_skip(CURLOPT_PROXYTYPE) {
                            result = tool_setopt_enum(
                                curl,
                                global,
                                b"CURLOPT_PROXYTYPE\0" as *const u8 as *const i8,
                                CURLOPT_PROXYTYPE,
                                unsafe { setopt_nv_CURLPROXY.as_ptr() },
                                (unsafe { (*config).proxyver }) as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_PROXYUSERPWD) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_PROXYUSERPWD\0" as *const u8 as *const i8,
                            CURLOPT_PROXYUSERPWD,
                            (*config).proxyuserpwd,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_HTTPPROXYTUNNEL) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_HTTPPROXYTUNNEL\0" as *const u8 as *const i8,
                            CURLOPT_HTTPPROXYTUNNEL,
                            if (*config).proxytunnel as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !(unsafe { (*config).preproxy }).is_null() {
                        if !tool_setopt_skip(CURLOPT_PRE_PROXY) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PRE_PROXY\0" as *const u8 as *const i8,
                                CURLOPT_PRE_PROXY,
                                (*config).preproxy,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).proxyanyauth } {
                        if !tool_setopt_skip(CURLOPT_PROXYAUTH) {
                            result = tool_setopt_bitmask(
                                curl,
                                global,
                                b"CURLOPT_PROXYAUTH\0" as *const u8 as *const i8,
                                CURLOPT_PROXYAUTH,
                                unsafe { setopt_nv_CURLAUTH.as_ptr() },
                                !((1 as i32 as u64) << 4 as i32) as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    } else if unsafe { (*config).proxynegotiate } {
                        if !tool_setopt_skip(CURLOPT_PROXYAUTH) {
                            result = tool_setopt_bitmask(
                                curl,
                                global,
                                b"CURLOPT_PROXYAUTH\0" as *const u8 as *const i8,
                                CURLOPT_PROXYAUTH,
                                unsafe { setopt_nv_CURLAUTH.as_ptr() },
                                ((1 as i32 as u64) << 2 as i32) as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    } else if unsafe { (*config).proxyntlm } {
                        if !tool_setopt_skip(CURLOPT_PROXYAUTH) {
                            result = tool_setopt_bitmask(
                                curl,
                                global,
                                b"CURLOPT_PROXYAUTH\0" as *const u8 as *const i8,
                                CURLOPT_PROXYAUTH,
                                unsafe { setopt_nv_CURLAUTH.as_ptr() },
                                ((1 as i32 as u64) << 3 as i32) as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    } else if unsafe { (*config).proxydigest } {
                        if !tool_setopt_skip(CURLOPT_PROXYAUTH) {
                            result = tool_setopt_bitmask(
                                curl,
                                global,
                                b"CURLOPT_PROXYAUTH\0" as *const u8 as *const i8,
                                CURLOPT_PROXYAUTH,
                                unsafe { setopt_nv_CURLAUTH.as_ptr() },
                                ((1 as i32 as u64) << 1 as i32) as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    } else if unsafe { (*config).proxybasic } {
                        if !tool_setopt_skip(CURLOPT_PROXYAUTH) {
                            result = tool_setopt_bitmask(
                                curl,
                                global,
                                b"CURLOPT_PROXYAUTH\0" as *const u8 as *const i8,
                                CURLOPT_PROXYAUTH,
                                unsafe { setopt_nv_CURLAUTH.as_ptr() },
                                ((1 as i32 as u64) << 0 as i32) as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_NOPROXY) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_NOPROXY\0" as *const u8 as *const i8,
                            CURLOPT_NOPROXY,
                            (*config).noproxy,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_SUPPRESS_CONNECT_HEADERS) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_SUPPRESS_CONNECT_HEADERS\0" as *const u8 as *const i8,
                            CURLOPT_SUPPRESS_CONNECT_HEADERS,
                            if (*config).suppress_connect_headers as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_FAILONERROR) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_FAILONERROR\0" as *const u8 as *const i8,
                            CURLOPT_FAILONERROR,
                            if (*config).failonerror as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_REQUEST_TARGET) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_REQUEST_TARGET\0" as *const u8 as *const i8,
                            CURLOPT_REQUEST_TARGET,
                            (*config).request_target,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_UPLOAD) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_UPLOAD\0" as *const u8 as *const i8,
                            CURLOPT_UPLOAD,
                            if !((*per).uploadfile).is_null() {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_DIRLISTONLY) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_DIRLISTONLY\0" as *const u8 as *const i8,
                            CURLOPT_DIRLISTONLY,
                            if (*config).dirlistonly as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_APPEND) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_APPEND\0" as *const u8 as *const i8,
                            CURLOPT_APPEND,
                            if (*config).ftp_append as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if unsafe { (*config).netrc_opt } {
                        if !tool_setopt_skip(CURLOPT_NETRC) {
                            result = tool_setopt_enum(
                                curl,
                                global,
                                b"CURLOPT_NETRC\0" as *const u8 as *const i8,
                                CURLOPT_NETRC,
                                unsafe { setopt_nv_CURL_NETRC.as_ptr() },
                                CURL_NETRC_OPTIONAL as i32 as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    } else if (unsafe { (*config).netrc }) as i32 != 0 || !(unsafe { (*config).netrc_file }).is_null() {
                        if !tool_setopt_skip(CURLOPT_NETRC) {
                            result = tool_setopt_enum(
                                curl,
                                global,
                                b"CURLOPT_NETRC\0" as *const u8 as *const i8,
                                CURLOPT_NETRC,
                                unsafe { setopt_nv_CURL_NETRC.as_ptr() },
                                CURL_NETRC_REQUIRED as i32 as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    } else if !tool_setopt_skip(CURLOPT_NETRC) {
                        result = tool_setopt_enum(
                            curl,
                            global,
                            b"CURLOPT_NETRC\0" as *const u8 as *const i8,
                            CURLOPT_NETRC,
                            unsafe { setopt_nv_CURL_NETRC.as_ptr() },
                            CURL_NETRC_IGNORED as i32 as i64,
                        );
                        let _ = result as u64 != 0;
                    }
                    if !(unsafe { (*config).netrc_file }).is_null() {
                        if !tool_setopt_skip(CURLOPT_NETRC_FILE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_NETRC_FILE\0" as *const u8 as *const i8,
                                CURLOPT_NETRC_FILE,
                                (*config).netrc_file,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_TRANSFERTEXT) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_TRANSFERTEXT\0" as *const u8 as *const i8,
                            CURLOPT_TRANSFERTEXT,
                            if (*config).use_ascii as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !(unsafe { (*config).login_options }).is_null() {
                        if !tool_setopt_skip(CURLOPT_LOGIN_OPTIONS) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_LOGIN_OPTIONS\0" as *const u8 as *const i8,
                                CURLOPT_LOGIN_OPTIONS,
                                (*config).login_options,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_USERPWD) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_USERPWD\0" as *const u8 as *const i8,
                            CURLOPT_USERPWD,
                            (*config).userpwd,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_RANGE) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_RANGE\0" as *const u8 as *const i8,
                            CURLOPT_RANGE,
                            (*config).range,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_ERRORBUFFER) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_ERRORBUFFER\0" as *const u8 as *const i8,
                            CURLOPT_ERRORBUFFER,
                            ((*per).errorbuffer).as_mut_ptr(),
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_TIMEOUT_MS) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_TIMEOUT_MS\0" as *const u8 as *const i8,
                            CURLOPT_TIMEOUT_MS,
                            ((*config).timeout * 1000 as i32 as f64) as i64,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    match (unsafe { (*config).httpreq }) as u32 {
                        4 => {
                            if !tool_setopt_skip(CURLOPT_POSTFIELDS) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_POSTFIELDS\0" as *const u8 as *const i8,
                                    CURLOPT_POSTFIELDS,
                                    (*config).postfields,
                                ) };
                                let _ = result as u64 != 0;
                            }
                            if !tool_setopt_skip(CURLOPT_POSTFIELDSIZE_LARGE) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_POSTFIELDSIZE_LARGE\0" as *const u8 as *const i8,
                                    CURLOPT_POSTFIELDSIZE_LARGE,
                                    (*config).postfieldsize,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        3 => {
                            curl_mime_free(unsafe { (*config).mimepost });
                            let mut fresh50 = unsafe { &mut ((*config).mimepost) };
                            *fresh50 = 0 as *mut curl_mime;
                            result = tool2curlmime(
                                curl,
                                unsafe { (*config).mimeroot },
                                Some(unsafe { &mut (*config).mimepost }),
                            );
                            if !(result as u64 != 0) {
                                if !tool_setopt_skip(CURLOPT_MIMEPOST) {
                                    result = tool_setopt_mimepost(
                                        curl,
                                        global,
                                        b"CURLOPT_MIMEPOST\0" as *const u8 as *const i8,
                                        CURLOPT_MIMEPOST,
                                        unsafe { (*config).mimepost },
                                    );
                                    let _ = result as u64 != 0;
                                }
                            }
                        }
                        _ => {}
                    }
                    if result as u64 != 0 {
                        break;
                    }
                    if (unsafe { (*config).authtype }) != 0 {
                        if !tool_setopt_skip(CURLOPT_HTTPAUTH) {
                            result = tool_setopt_bitmask(
                                curl,
                                global,
                                b"CURLOPT_HTTPAUTH\0" as *const u8 as *const i8,
                                CURLOPT_HTTPAUTH,
                                unsafe { setopt_nv_CURLAUTH.as_ptr() },
                                (unsafe { (*config).authtype }) as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_HTTPHEADER) {
                        result = tool_setopt_slist(
                            curl,
                            global,
                            b"CURLOPT_HTTPHEADER\0" as *const u8 as *const i8,
                            CURLOPT_HTTPHEADER,
                            unsafe { (*config).headers },
                        );
                        let _ = result as u64 != 0;
                    }
                    if (unsafe { built_in_protos }) & ((1 as i32) << 0 as i32 | (1 as i32) << 18 as i32) as i64
                        != 0
                    {
                        if !tool_setopt_skip(CURLOPT_REFERER) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_REFERER\0" as *const u8 as *const i8,
                                CURLOPT_REFERER,
                                (*config).referer,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_USERAGENT) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_USERAGENT\0" as *const u8 as *const i8,
                                CURLOPT_USERAGENT,
                                (*config).useragent,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { built_in_protos }) & ((1 as i32) << 0 as i32) as i64 != 0 {
                        let mut postRedir: i64 = 0 as i32 as i64;
                        if !tool_setopt_skip(CURLOPT_FOLLOWLOCATION) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_FOLLOWLOCATION\0" as *const u8 as *const i8,
                                CURLOPT_FOLLOWLOCATION,
                                if (*config).followlocation as i32 != 0 {
                                    1 as i64
                                } else {
                                    0 as i64
                                },
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_UNRESTRICTED_AUTH) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_UNRESTRICTED_AUTH\0" as *const u8 as *const i8,
                                CURLOPT_UNRESTRICTED_AUTH,
                                if (*config).unrestricted_auth as i32 != 0 {
                                    1 as i64
                                } else {
                                    0 as i64
                                },
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_AUTOREFERER) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_AUTOREFERER\0" as *const u8 as *const i8,
                                CURLOPT_AUTOREFERER,
                                if (*config).autoreferer as i32 != 0 {
                                    1 as i64
                                } else {
                                    0 as i64
                                },
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !(unsafe { (*config).proxyheaders }).is_null() {
                            if !tool_setopt_skip(CURLOPT_PROXYHEADER) {
                                result = tool_setopt_slist(
                                    curl,
                                    global,
                                    b"CURLOPT_PROXYHEADER\0" as *const u8 as *const i8,
                                    CURLOPT_PROXYHEADER,
                                    unsafe { (*config).proxyheaders },
                                );
                                let _ = result as u64 != 0;
                            }
                            if !tool_setopt_skip(CURLOPT_HEADEROPT) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_HEADEROPT\0" as *const u8 as *const i8,
                                    CURLOPT_HEADEROPT,
                                    (1 as i32) << 0 as i32,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if !tool_setopt_skip(CURLOPT_MAXREDIRS) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_MAXREDIRS\0" as *const u8 as *const i8,
                                CURLOPT_MAXREDIRS,
                                (*config).maxredirs,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if (unsafe { (*config).httpversion }) != 0 {
                            if !tool_setopt_skip(CURLOPT_HTTP_VERSION) {
                                result = tool_setopt_enum(
                                    curl,
                                    global,
                                    b"CURLOPT_HTTP_VERSION\0" as *const u8 as *const i8,
                                    CURLOPT_HTTP_VERSION,
                                    unsafe { setopt_nv_CURL_HTTP_VERSION.as_ptr() },
                                    unsafe { (*config).httpversion },
                                );
                                let _ = result as u64 != 0;
                            }
                        } else if (unsafe { (*curlinfo).features }) & (1 as i32) << 16 as i32 != 0 {
                            if !tool_setopt_skip(CURLOPT_HTTP_VERSION) {
                                result = tool_setopt_enum(
                                    curl,
                                    global,
                                    b"CURLOPT_HTTP_VERSION\0" as *const u8 as *const i8,
                                    CURLOPT_HTTP_VERSION,
                                    unsafe { setopt_nv_CURL_HTTP_VERSION.as_ptr() },
                                    CURL_HTTP_VERSION_2TLS as i32 as i64,
                                );
                                let _ = result as u64 != 0;
                            }
                        }
                        if unsafe { (*config).post301 } {
                            postRedir |= 1 as i32 as i64;
                        }
                        if unsafe { (*config).post302 } {
                            postRedir |= 2 as i32 as i64;
                        }
                        if unsafe { (*config).post303 } {
                            postRedir |= 4 as i32 as i64;
                        }
                        if !tool_setopt_skip(CURLOPT_POSTREDIR) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_POSTREDIR\0" as *const u8 as *const i8,
                                CURLOPT_POSTREDIR,
                                postRedir,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if unsafe { (*config).encoding } {
                            if !tool_setopt_skip(CURLOPT_ACCEPT_ENCODING) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_ACCEPT_ENCODING\0" as *const u8 as *const i8,
                                    CURLOPT_ACCEPT_ENCODING,
                                    b"\0" as *const u8 as *const i8,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if unsafe { (*config).tr_encoding } {
                            if !tool_setopt_skip(CURLOPT_TRANSFER_ENCODING) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_TRANSFER_ENCODING\0" as *const u8 as *const i8,
                                    CURLOPT_TRANSFER_ENCODING,
                                    1 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if !tool_setopt_skip(CURLOPT_HTTP09_ALLOWED) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_HTTP09_ALLOWED\0" as *const u8 as *const i8,
                                CURLOPT_HTTP09_ALLOWED,
                                if (*config).http09_allowed as i32 != 0 {
                                    1 as i64
                                } else {
                                    0 as i64
                                },
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if result as u64 != 0 {
                            (unsafe { errorf(
                                global,
                                b"HTTP/0.9 is not supported in this build!\n\0" as *const u8
                                    as *const i8,
                            ) });
                            return result;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_FTPPORT) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_FTPPORT\0" as *const u8 as *const i8,
                            CURLOPT_FTPPORT,
                            (*config).ftpport,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_LOW_SPEED_LIMIT) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_LOW_SPEED_LIMIT\0" as *const u8 as *const i8,
                            CURLOPT_LOW_SPEED_LIMIT,
                            (*config).low_speed_limit,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_LOW_SPEED_TIME) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_LOW_SPEED_TIME\0" as *const u8 as *const i8,
                            CURLOPT_LOW_SPEED_TIME,
                            (*config).low_speed_time,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_MAX_SEND_SPEED_LARGE) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_MAX_SEND_SPEED_LARGE\0" as *const u8 as *const i8,
                            CURLOPT_MAX_SEND_SPEED_LARGE,
                            (*config).sendpersecond,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_MAX_RECV_SPEED_LARGE) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_MAX_RECV_SPEED_LARGE\0" as *const u8 as *const i8,
                            CURLOPT_MAX_RECV_SPEED_LARGE,
                            (*config).recvpersecond,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if unsafe { (*config).use_resume } {
                        if !tool_setopt_skip(CURLOPT_RESUME_FROM_LARGE) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_RESUME_FROM_LARGE\0" as *const u8 as *const i8,
                                CURLOPT_RESUME_FROM_LARGE,
                                (*config).resume_from,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    } else if !tool_setopt_skip(CURLOPT_RESUME_FROM_LARGE) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_RESUME_FROM_LARGE\0" as *const u8 as *const i8,
                            CURLOPT_RESUME_FROM_LARGE,
                            0 as i64,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_KEYPASSWD) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_KEYPASSWD\0" as *const u8 as *const i8,
                            CURLOPT_KEYPASSWD,
                            (*config).key_passwd,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_PROXY_KEYPASSWD) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_PROXY_KEYPASSWD\0" as *const u8 as *const i8,
                            CURLOPT_PROXY_KEYPASSWD,
                            (*config).proxy_key_passwd,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if (unsafe { built_in_protos }) & ((1 as i32) << 4 as i32 | (1 as i32) << 5 as i32) as i64
                        != 0
                    {
                        if !tool_setopt_skip(CURLOPT_SSH_PRIVATE_KEYFILE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSH_PRIVATE_KEYFILE\0" as *const u8 as *const i8,
                                CURLOPT_SSH_PRIVATE_KEYFILE,
                                (*config).key,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_SSH_PUBLIC_KEYFILE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSH_PUBLIC_KEYFILE\0" as *const u8 as *const i8,
                                CURLOPT_SSH_PUBLIC_KEYFILE,
                                (*config).pubkey,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_SSH_HOST_PUBLIC_KEY_MD5) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSH_HOST_PUBLIC_KEY_MD5\0" as *const u8 as *const i8,
                                CURLOPT_SSH_HOST_PUBLIC_KEY_MD5,
                                (*config).hostpubmd5,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if unsafe { (*config).ssh_compression } {
                            if !tool_setopt_skip(CURLOPT_SSH_COMPRESSION) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_SSH_COMPRESSION\0" as *const u8 as *const i8,
                                    CURLOPT_SSH_COMPRESSION,
                                    1 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                    }
                    if !(unsafe { (*config).cacert }).is_null() {
                        if !tool_setopt_skip(CURLOPT_CAINFO) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_CAINFO\0" as *const u8 as *const i8,
                                CURLOPT_CAINFO,
                                (*config).cacert,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).proxy_cacert }).is_null() {
                        if !tool_setopt_skip(CURLOPT_PROXY_CAINFO) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_CAINFO\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_CAINFO,
                                (*config).proxy_cacert,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).capath }).is_null() {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_CAPATH\0" as *const u8 as *const i8,
                            CURLOPT_CAPATH,
                            (*config).capath,
                        ) };
                        if result as u32 == CURLE_NOT_BUILT_IN as i32 as u32 {
                            (unsafe { warnf(
                                global,
                                b"ignoring %s, not supported by libcurl\n\0" as *const u8
                                    as *const i8,
                                if capath_from_env as i32 != 0 {
                                    b"SSL_CERT_DIR environment variable\0" as *const u8 as *const i8
                                } else {
                                    b"--capath\0" as *const u8 as *const i8
                                },
                            ) });
                        } else if result as u64 != 0 {
                            break;
                        }
                    }
                    if (!(unsafe { (*config).proxy_capath }).is_null() || !(unsafe { (*config).capath }).is_null())
                        && !tool_setopt_skip(CURLOPT_PROXY_CAPATH)
                    {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_PROXY_CAPATH\0" as *const u8 as *const i8,
                            CURLOPT_PROXY_CAPATH,
                            if !((*config).proxy_capath).is_null() {
                                (*config).proxy_capath
                            } else {
                                (*config).capath
                            },
                        ) };
                        if result as u32 == CURLE_NOT_BUILT_IN as i32 as u32 {
                            if !(unsafe { (*config).proxy_capath }).is_null() {
                                (unsafe { warnf(
                                    global,
                                    b"ignoring --proxy-capath, not supported by libcurl\n\0"
                                        as *const u8
                                        as *const i8,
                                ) });
                            }
                        } else if result as u64 != 0 {
                            break;
                        }
                    }
                    if !(unsafe { (*config).crlfile }).is_null() {
                        if !tool_setopt_skip(CURLOPT_CRLFILE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_CRLFILE\0" as *const u8 as *const i8,
                                CURLOPT_CRLFILE,
                                (*config).crlfile,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).proxy_crlfile }).is_null() {
                        if !tool_setopt_skip(CURLOPT_PROXY_CRLFILE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_CRLFILE\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_CRLFILE,
                                (*config).proxy_crlfile,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    } else if !(unsafe { (*config).crlfile }).is_null() {
                        if !tool_setopt_skip(CURLOPT_PROXY_CRLFILE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_CRLFILE\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_CRLFILE,
                                (*config).crlfile,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).pinnedpubkey }).is_null() {
                        if !tool_setopt_skip(CURLOPT_PINNEDPUBLICKEY) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PINNEDPUBLICKEY\0" as *const u8 as *const i8,
                                CURLOPT_PINNEDPUBLICKEY,
                                (*config).pinnedpubkey,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).ssl_ec_curves }).is_null() {
                        if !tool_setopt_skip(CURLOPT_SSL_EC_CURVES) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSL_EC_CURVES\0" as *const u8 as *const i8,
                                CURLOPT_SSL_EC_CURVES,
                                (*config).ssl_ec_curves,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { (*curlinfo).features }) & (1 as i32) << 2 as i32 != 0 {
                        if !(unsafe { (*config).cert }).is_null() {
                            if (unsafe { (*config).cert_type }).is_null() {
                                if is_pkcs11_uri(unsafe { (*config).cert }) {
                                    let mut fresh51 = unsafe { &mut ((*config).cert_type) };
                                    *fresh51 = unsafe { strdup(b"ENG\0" as *const u8 as *const i8) };
                                }
                            }
                        }
                        if !(unsafe { (*config).key }).is_null() {
                            if (unsafe { (*config).key_type }).is_null() {
                                if is_pkcs11_uri(unsafe { (*config).key }) {
                                    let mut fresh52 = unsafe { &mut ((*config).key_type) };
                                    *fresh52 = unsafe { strdup(b"ENG\0" as *const u8 as *const i8) };
                                }
                            }
                        }
                        if !(unsafe { (*config).proxy_cert }).is_null() {
                            if (unsafe { (*config).proxy_cert_type }).is_null() {
                                if is_pkcs11_uri(unsafe { (*config).proxy_cert }) {
                                    let mut fresh53 = unsafe { &mut ((*config).proxy_cert_type) };
                                    *fresh53 = unsafe { strdup(b"ENG\0" as *const u8 as *const i8) };
                                }
                            }
                        }
                        if !(unsafe { (*config).proxy_key }).is_null() {
                            if (unsafe { (*config).proxy_key_type }).is_null() {
                                if is_pkcs11_uri(unsafe { (*config).proxy_key }) {
                                    let mut fresh54 = unsafe { &mut ((*config).proxy_key_type) };
                                    *fresh54 = unsafe { strdup(b"ENG\0" as *const u8 as *const i8) };
                                }
                            }
                        }
                        if !tool_setopt_skip(CURLOPT_SSLCERT) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSLCERT\0" as *const u8 as *const i8,
                                CURLOPT_SSLCERT,
                                (*config).cert,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_PROXY_SSLCERT) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_SSLCERT\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_SSLCERT,
                                (*config).proxy_cert,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_SSLCERTTYPE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSLCERTTYPE\0" as *const u8 as *const i8,
                                CURLOPT_SSLCERTTYPE,
                                (*config).cert_type,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_PROXY_SSLCERTTYPE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_SSLCERTTYPE\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_SSLCERTTYPE,
                                (*config).proxy_cert_type,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_SSLKEY) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSLKEY\0" as *const u8 as *const i8,
                                CURLOPT_SSLKEY,
                                (*config).key,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_PROXY_SSLKEY) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_SSLKEY\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_SSLKEY,
                                (*config).proxy_key,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_SSLKEYTYPE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSLKEYTYPE\0" as *const u8 as *const i8,
                                CURLOPT_SSLKEYTYPE,
                                (*config).key_type,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_PROXY_SSLKEYTYPE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_SSLKEYTYPE\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_SSLKEYTYPE,
                                (*config).proxy_key_type,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_AWS_SIGV4) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_AWS_SIGV4\0" as *const u8 as *const i8,
                                CURLOPT_AWS_SIGV4,
                                (*config).aws_sigv4,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if unsafe { (*config).insecure_ok } {
                            if !tool_setopt_skip(CURLOPT_SSL_VERIFYPEER) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_SSL_VERIFYPEER\0" as *const u8 as *const i8,
                                    CURLOPT_SSL_VERIFYPEER,
                                    0 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                            if !tool_setopt_skip(CURLOPT_SSL_VERIFYHOST) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_SSL_VERIFYHOST\0" as *const u8 as *const i8,
                                    CURLOPT_SSL_VERIFYHOST,
                                    0 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        } else if !tool_setopt_skip(CURLOPT_SSL_VERIFYPEER) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSL_VERIFYPEER\0" as *const u8 as *const i8,
                                CURLOPT_SSL_VERIFYPEER,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if unsafe { (*config).doh_insecure_ok } {
                            if !tool_setopt_skip(CURLOPT_DOH_SSL_VERIFYPEER) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_DOH_SSL_VERIFYPEER\0" as *const u8 as *const i8,
                                    CURLOPT_DOH_SSL_VERIFYPEER,
                                    0 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                            if !tool_setopt_skip(CURLOPT_DOH_SSL_VERIFYHOST) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_DOH_SSL_VERIFYHOST\0" as *const u8 as *const i8,
                                    CURLOPT_DOH_SSL_VERIFYHOST,
                                    0 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if unsafe { (*config).proxy_insecure_ok } {
                            if !tool_setopt_skip(CURLOPT_PROXY_SSL_VERIFYPEER) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_PROXY_SSL_VERIFYPEER\0" as *const u8 as *const i8,
                                    CURLOPT_PROXY_SSL_VERIFYPEER,
                                    0 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                            if !tool_setopt_skip(CURLOPT_PROXY_SSL_VERIFYHOST) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_PROXY_SSL_VERIFYHOST\0" as *const u8 as *const i8,
                                    CURLOPT_PROXY_SSL_VERIFYHOST,
                                    0 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        } else if !tool_setopt_skip(CURLOPT_PROXY_SSL_VERIFYPEER) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_SSL_VERIFYPEER\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_SSL_VERIFYPEER,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if unsafe { (*config).verifystatus } {
                            if !tool_setopt_skip(CURLOPT_SSL_VERIFYSTATUS) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_SSL_VERIFYSTATUS\0" as *const u8 as *const i8,
                                    CURLOPT_SSL_VERIFYSTATUS,
                                    1 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if unsafe { (*config).doh_verifystatus } {
                            if !tool_setopt_skip(CURLOPT_DOH_SSL_VERIFYSTATUS) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_DOH_SSL_VERIFYSTATUS\0" as *const u8 as *const i8,
                                    CURLOPT_DOH_SSL_VERIFYSTATUS,
                                    1 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if unsafe { (*config).falsestart } {
                            if !tool_setopt_skip(CURLOPT_SSL_FALSESTART) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_SSL_FALSESTART\0" as *const u8 as *const i8,
                                    CURLOPT_SSL_FALSESTART,
                                    1 as i64,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if !tool_setopt_skip(CURLOPT_SSLVERSION) {
                            result = tool_setopt_enum(
                                curl,
                                global,
                                b"CURLOPT_SSLVERSION\0" as *const u8 as *const i8,
                                CURLOPT_SSLVERSION,
                                unsafe { setopt_nv_CURL_SSLVERSION.as_ptr() },
                                (unsafe { (*config).ssl_version }) | (unsafe { (*config).ssl_version_max }),
                            );
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_PROXY_SSLVERSION) {
                            result = tool_setopt_enum(
                                curl,
                                global,
                                b"CURLOPT_PROXY_SSLVERSION\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_SSLVERSION,
                                unsafe { setopt_nv_CURL_SSLVERSION.as_ptr() },
                                unsafe { (*config).proxy_ssl_version },
                            );
                            let _ = result as u64 != 0;
                        }
                        let mut mask: i64 = ((if (unsafe { (*config).ssl_allow_beast }) as i32 != 0 {
                            (1 as i32) << 0 as i32
                        } else {
                            0 as i32
                        }) | (if (unsafe { (*config).ssl_no_revoke }) as i32 != 0 {
                            (1 as i32) << 1 as i32
                        } else {
                            0 as i32
                        }) | (if (unsafe { (*config).ssl_revoke_best_effort }) as i32 != 0 {
                            (1 as i32) << 3 as i32
                        } else {
                            0 as i32
                        }) | (if (unsafe { (*config).native_ca_store }) as i32 != 0 {
                            (1 as i32) << 4 as i32
                        } else {
                            0 as i32
                        }) | (if (unsafe { (*config).ssl_auto_client_cert }) as i32 != 0 {
                            (1 as i32) << 5 as i32
                        } else {
                            0 as i32
                        })) as i64;
                        if mask != 0 {
                            if !tool_setopt_skip(CURLOPT_SSL_OPTIONS) {
                                result = tool_setopt_bitmask(
                                    curl,
                                    global,
                                    b"CURLOPT_SSL_OPTIONS\0" as *const u8 as *const i8,
                                    CURLOPT_SSL_OPTIONS,
                                    unsafe { setopt_nv_CURLSSLOPT.as_ptr() },
                                    mask,
                                );
                                let _ = result as u64 != 0;
                            }
                        }
                        let mut mask_0: i64 =
                            ((if (unsafe { (*config).proxy_ssl_allow_beast }) as i32 != 0 {
                                (1 as i32) << 0 as i32
                            } else {
                                0 as i32
                            }) | (if (unsafe { (*config).proxy_ssl_auto_client_cert }) as i32 != 0 {
                                (1 as i32) << 5 as i32
                            } else {
                                0 as i32
                            })) as i64;
                        if mask_0 != 0 {
                            if !tool_setopt_skip(CURLOPT_PROXY_SSL_OPTIONS) {
                                result = tool_setopt_bitmask(
                                    curl,
                                    global,
                                    b"CURLOPT_PROXY_SSL_OPTIONS\0" as *const u8 as *const i8,
                                    CURLOPT_PROXY_SSL_OPTIONS,
                                    unsafe { setopt_nv_CURLSSLOPT.as_ptr() },
                                    mask_0,
                                );
                                let _ = result as u64 != 0;
                            }
                        }
                    }
                    if unsafe { (*config).path_as_is } {
                        if !tool_setopt_skip(CURLOPT_PATH_AS_IS) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PATH_AS_IS\0" as *const u8 as *const i8,
                                CURLOPT_PATH_AS_IS,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { built_in_protos }) & ((1 as i32) << 4 as i32 | (1 as i32) << 5 as i32) as i64
                        != 0
                        && !(unsafe { (*config).insecure_ok })
                    {
                        let mut home: *mut i8 = homedir(0 as *const i8);
                        if !home.is_null() {
                            let mut file_1: *mut i8 = unsafe { curl_maprintf(
                                b"%s/.ssh/known_hosts\0" as *const u8 as *const i8,
                                home,
                            ) };
                            if !file_1.is_null() {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_SSH_KNOWNHOSTS\0" as *const u8 as *const i8,
                                    CURLOPT_SSH_KNOWNHOSTS,
                                    file_1,
                                ) };
                                curl_free(file_1 as *mut libc::c_void);
                                if result as u32 == CURLE_UNKNOWN_OPTION as i32 as u32 {
                                    result = CURLE_OK;
                                }
                            }
                            (unsafe { free(home as *mut libc::c_void) });
                            home = 0 as *mut i8;
                            if result as u64 != 0 {
                                break;
                            }
                        } else {
                            (unsafe { warnf(
                                global,
                                b"No home dir, couldn't find known_hosts file!\0" as *const u8
                                    as *const i8,
                            ) });
                        }
                    }
                    if (unsafe { (*config).no_body }) as i32 != 0 || (unsafe { (*config).remote_time }) as i32 != 0 {
                        if !tool_setopt_skip(CURLOPT_FILETIME) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_FILETIME\0" as *const u8 as *const i8,
                                CURLOPT_FILETIME,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_CRLF) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_CRLF\0" as *const u8 as *const i8,
                            CURLOPT_CRLF,
                            if (*config).crlf as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_QUOTE) {
                        result = tool_setopt_slist(
                            curl,
                            global,
                            b"CURLOPT_QUOTE\0" as *const u8 as *const i8,
                            CURLOPT_QUOTE,
                            unsafe { (*config).quote },
                        );
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_POSTQUOTE) {
                        result = tool_setopt_slist(
                            curl,
                            global,
                            b"CURLOPT_POSTQUOTE\0" as *const u8 as *const i8,
                            CURLOPT_POSTQUOTE,
                            unsafe { (*config).postquote },
                        );
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_PREQUOTE) {
                        result = tool_setopt_slist(
                            curl,
                            global,
                            b"CURLOPT_PREQUOTE\0" as *const u8 as *const i8,
                            CURLOPT_PREQUOTE,
                            unsafe { (*config).prequote },
                        );
                        let _ = result as u64 != 0;
                    }
                    if !(unsafe { (*config).cookies }).is_null() {
                        let mut cookies: crate::src::lib::http2::dynbuf = dynbuf {
                            bufr: 0 as *mut i8,
                            leng: 0,
                            allc: 0,
                            toobig: 0,
                        };
                        let mut cl: *mut crate::src::lib::http2::curl_slist = 0 as *mut curl_slist;
                        let mut ret: u32 = CURLE_OK;
                        (unsafe { curlx_dyn_init(&mut cookies, 4096 as i32 as size_t) });
                        cl = unsafe { (*config).cookies };
                        while !cl.is_null() {
                            if cl == (unsafe { (*config).cookies }) {
                                ret = unsafe { curlx_dyn_addf(
                                    &mut cookies as *mut dynbuf,
                                    b"%s\0" as *const u8 as *const i8,
                                    (*cl).data,
                                ) };
                            } else {
                                ret = unsafe { curlx_dyn_addf(
                                    &mut cookies as *mut dynbuf,
                                    b";%s\0" as *const u8 as *const i8,
                                    (*cl).data,
                                ) };
                            }
                            if ret as u64 != 0 {
                                result = CURLE_OUT_OF_MEMORY;
                                break;
                            } else {
                                cl = unsafe { (*cl).next };
                            }
                        }
                        if !tool_setopt_skip(CURLOPT_COOKIE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_COOKIE\0" as *const u8 as *const i8,
                                CURLOPT_COOKIE,
                                curlx_dyn_ptr(&mut cookies),
                            ) };
                            let _ = result as u64 != 0;
                        }
                        (unsafe { curlx_dyn_free(&mut cookies) });
                    }
                    if !(unsafe { (*config).cookiefiles }).is_null() {
                        let mut cfl: *mut crate::src::lib::http2::curl_slist = 0 as *mut curl_slist;
                        cfl = unsafe { (*config).cookiefiles };
                        while !cfl.is_null() {
                            if !tool_setopt_skip(CURLOPT_COOKIEFILE) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_COOKIEFILE\0" as *const u8 as *const i8,
                                    CURLOPT_COOKIEFILE,
                                    (*cfl).data,
                                ) };
                                let _ = result as u64 != 0;
                            }
                            cfl = unsafe { (*cfl).next };
                        }
                    }
                    if !(unsafe { (*config).cookiejar }).is_null() {
                        if !tool_setopt_skip(CURLOPT_COOKIEJAR) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_COOKIEJAR\0" as *const u8 as *const i8,
                                CURLOPT_COOKIEJAR,
                                (*config).cookiejar,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_COOKIESESSION) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_COOKIESESSION\0" as *const u8 as *const i8,
                            CURLOPT_COOKIESESSION,
                            if (*config).cookiesession as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_TIMECONDITION) {
                        result = tool_setopt_enum(
                            curl,
                            global,
                            b"CURLOPT_TIMECONDITION\0" as *const u8 as *const i8,
                            CURLOPT_TIMECONDITION,
                            unsafe { setopt_nv_CURL_TIMECOND.as_ptr() },
                            (unsafe { (*config).timecond }) as i64,
                        );
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_TIMEVALUE_LARGE) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_TIMEVALUE_LARGE\0" as *const u8 as *const i8,
                            CURLOPT_TIMEVALUE_LARGE,
                            (*config).condtime,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_CUSTOMREQUEST) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_CUSTOMREQUEST\0" as *const u8 as *const i8,
                            CURLOPT_CUSTOMREQUEST,
                            (*config).customrequest,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    customrequest_helper(config, unsafe { (*config).httpreq }, unsafe { (*config).customrequest });
                    if !tool_setopt_skip(CURLOPT_STDERR) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_STDERR\0" as *const u8 as *const i8,
                            CURLOPT_STDERR,
                            (*global).errors,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_INTERFACE) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_INTERFACE\0" as *const u8 as *const i8,
                            CURLOPT_INTERFACE,
                            (*config).iface,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_KRBLEVEL) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_KRBLEVEL\0" as *const u8 as *const i8,
                            CURLOPT_KRBLEVEL,
                            (*config).krblevel,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    progressbarinit(unsafe { &mut (*per).progressbar }, config);
                    if (unsafe { (*global).progressmode }) == 1 as i32
                        && !(unsafe { (*global).noprogress })
                        && !(unsafe { (*global).mute })
                    {
                        if !tool_setopt_skip(CURLOPT_XFERINFOFUNCTION) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_XFERINFOFUNCTION\0" as *const u8 as *const i8,
                                CURLOPT_XFERINFOFUNCTION,
                                Some(tool_progress_cb),
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_XFERINFODATA) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_XFERINFODATA\0" as *const u8 as *const i8,
                                CURLOPT_XFERINFODATA,
                                per,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    } else if !(unsafe { (*per).uploadfile }).is_null()
                        && (unsafe { strcmp((*per).uploadfile, b".\0" as *const u8 as *const i8) }) == 0
                    {
                        if !tool_setopt_skip(CURLOPT_NOPROGRESS) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_NOPROGRESS\0" as *const u8 as *const i8,
                                CURLOPT_NOPROGRESS,
                                0 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_XFERINFOFUNCTION) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_XFERINFOFUNCTION\0" as *const u8 as *const i8,
                                CURLOPT_XFERINFOFUNCTION,
                                Some(tool_readbusy_cb),
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_XFERINFODATA) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_XFERINFODATA\0" as *const u8 as *const i8,
                                CURLOPT_XFERINFODATA,
                                per,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).dns_servers }).is_null() {
                        if !tool_setopt_skip(CURLOPT_DNS_SERVERS) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_DNS_SERVERS\0" as *const u8 as *const i8,
                                CURLOPT_DNS_SERVERS,
                                (*config).dns_servers,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).dns_interface }).is_null() {
                        if !tool_setopt_skip(CURLOPT_DNS_INTERFACE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_DNS_INTERFACE\0" as *const u8 as *const i8,
                                CURLOPT_DNS_INTERFACE,
                                (*config).dns_interface,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).dns_ipv4_addr }).is_null() {
                        if !tool_setopt_skip(CURLOPT_DNS_LOCAL_IP4) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_DNS_LOCAL_IP4\0" as *const u8 as *const i8,
                                CURLOPT_DNS_LOCAL_IP4,
                                (*config).dns_ipv4_addr,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).dns_ipv6_addr }).is_null() {
                        if !tool_setopt_skip(CURLOPT_DNS_LOCAL_IP6) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_DNS_LOCAL_IP6\0" as *const u8 as *const i8,
                                CURLOPT_DNS_LOCAL_IP6,
                                (*config).dns_ipv6_addr,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_TELNETOPTIONS) {
                        result = tool_setopt_slist(
                            curl,
                            global,
                            b"CURLOPT_TELNETOPTIONS\0" as *const u8 as *const i8,
                            CURLOPT_TELNETOPTIONS,
                            unsafe { (*config).telnet_options },
                        );
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_RANDOM_FILE) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_RANDOM_FILE\0" as *const u8 as *const i8,
                            CURLOPT_RANDOM_FILE,
                            (*config).random_file,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_EGDSOCKET) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_EGDSOCKET\0" as *const u8 as *const i8,
                            CURLOPT_EGDSOCKET,
                            (*config).egd_file,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_CONNECTTIMEOUT_MS) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_CONNECTTIMEOUT_MS\0" as *const u8 as *const i8,
                            CURLOPT_CONNECTTIMEOUT_MS,
                            ((*config).connecttimeout * 1000 as i32 as f64) as i64,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !(unsafe { (*config).doh_url }).is_null() {
                        if !tool_setopt_skip(CURLOPT_DOH_URL) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_DOH_URL\0" as *const u8 as *const i8,
                                CURLOPT_DOH_URL,
                                (*config).doh_url,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).cipher_list }).is_null() {
                        if !tool_setopt_skip(CURLOPT_SSL_CIPHER_LIST) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSL_CIPHER_LIST\0" as *const u8 as *const i8,
                                CURLOPT_SSL_CIPHER_LIST,
                                (*config).cipher_list,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).proxy_cipher_list }).is_null() {
                        if !tool_setopt_skip(CURLOPT_PROXY_SSL_CIPHER_LIST) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_SSL_CIPHER_LIST\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_SSL_CIPHER_LIST,
                                (*config).proxy_cipher_list,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).cipher13_list }).is_null() {
                        if !tool_setopt_skip(CURLOPT_TLS13_CIPHERS) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_TLS13_CIPHERS\0" as *const u8 as *const i8,
                                CURLOPT_TLS13_CIPHERS,
                                (*config).cipher13_list,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).proxy_cipher13_list }).is_null() {
                        if !tool_setopt_skip(CURLOPT_PROXY_TLS13_CIPHERS) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_TLS13_CIPHERS\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_TLS13_CIPHERS,
                                (*config).proxy_cipher13_list,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).disable_epsv } {
                        if !tool_setopt_skip(CURLOPT_FTP_USE_EPSV) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_FTP_USE_EPSV\0" as *const u8 as *const i8,
                                CURLOPT_FTP_USE_EPSV,
                                0 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).disable_eprt } {
                        if !tool_setopt_skip(CURLOPT_FTP_USE_EPRT) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_FTP_USE_EPRT\0" as *const u8 as *const i8,
                                CURLOPT_FTP_USE_EPRT,
                                0 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { (*global).tracetype }) as u32 != TRACE_NONE as i32 as u32 {
                        if !tool_setopt_skip(CURLOPT_DEBUGFUNCTION) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_DEBUGFUNCTION\0" as *const u8 as *const i8,
                                CURLOPT_DEBUGFUNCTION,
                                Some(tool_debug_cb),
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_DEBUGDATA) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_DEBUGDATA\0" as *const u8 as *const i8,
                                CURLOPT_DEBUGDATA,
                                config,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_VERBOSE) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_VERBOSE\0" as *const u8 as *const i8,
                                CURLOPT_VERBOSE,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).engine }).is_null() {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_SSLENGINE\0" as *const u8 as *const i8,
                            CURLOPT_SSLENGINE,
                            (*config).engine,
                        ) };
                        if result as u64 != 0 {
                            break;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_FTP_CREATE_MISSING_DIRS) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_FTP_CREATE_MISSING_DIRS\0" as *const u8 as *const i8,
                            CURLOPT_FTP_CREATE_MISSING_DIRS,
                            (if (*config).ftp_create_dirs as i32 != 0 {
                                CURLFTP_CREATE_DIR_RETRY as i32
                            } else {
                                CURLFTP_CREATE_DIR_NONE as i32
                            }) as i64,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if (unsafe { (*config).max_filesize }) != 0 {
                        if !tool_setopt_skip(CURLOPT_MAXFILESIZE_LARGE) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_MAXFILESIZE_LARGE\0" as *const u8 as *const i8,
                                CURLOPT_MAXFILESIZE_LARGE,
                                (*config).max_filesize,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_IPRESOLVE) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_IPRESOLVE\0" as *const u8 as *const i8,
                            CURLOPT_IPRESOLVE,
                            (*config).ip_version,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if unsafe { (*config).ftp_ssl_reqd } {
                        if !tool_setopt_skip(CURLOPT_USE_SSL) {
                            result = tool_setopt_enum(
                                curl,
                                global,
                                b"CURLOPT_USE_SSL\0" as *const u8 as *const i8,
                                CURLOPT_USE_SSL,
                                unsafe { setopt_nv_CURLUSESSL.as_ptr() },
                                CURLUSESSL_ALL as i32 as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    } else if unsafe { (*config).ftp_ssl } {
                        if !tool_setopt_skip(CURLOPT_USE_SSL) {
                            result = tool_setopt_enum(
                                curl,
                                global,
                                b"CURLOPT_USE_SSL\0" as *const u8 as *const i8,
                                CURLOPT_USE_SSL,
                                unsafe { setopt_nv_CURLUSESSL.as_ptr() },
                                CURLUSESSL_TRY as i32 as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    } else if unsafe { (*config).ftp_ssl_control } {
                        if !tool_setopt_skip(CURLOPT_USE_SSL) {
                            result = tool_setopt_enum(
                                curl,
                                global,
                                b"CURLOPT_USE_SSL\0" as *const u8 as *const i8,
                                CURLOPT_USE_SSL,
                                unsafe { setopt_nv_CURLUSESSL.as_ptr() },
                                CURLUSESSL_CONTROL as i32 as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).ftp_ssl_ccc } {
                        if !tool_setopt_skip(CURLOPT_FTP_SSL_CCC) {
                            result = tool_setopt_enum(
                                curl,
                                global,
                                b"CURLOPT_FTP_SSL_CCC\0" as *const u8 as *const i8,
                                CURLOPT_FTP_SSL_CCC,
                                unsafe { setopt_nv_CURLFTPSSL_CCC.as_ptr() },
                                (unsafe { (*config).ftp_ssl_ccc_mode }) as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { (*config).socks5_gssapi_nec }) != 0 {
                        if !tool_setopt_skip(CURLOPT_SOCKS5_GSSAPI_NEC) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SOCKS5_GSSAPI_NEC\0" as *const u8 as *const i8,
                                CURLOPT_SOCKS5_GSSAPI_NEC,
                                (*config).socks5_gssapi_nec,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { (*config).socks5_auth }) != 0 {
                        if !tool_setopt_skip(CURLOPT_SOCKS5_AUTH) {
                            result = tool_setopt_bitmask(
                                curl,
                                global,
                                b"CURLOPT_SOCKS5_AUTH\0" as *const u8 as *const i8,
                                CURLOPT_SOCKS5_AUTH,
                                unsafe { setopt_nv_CURLAUTH.as_ptr() },
                                (unsafe { (*config).socks5_auth }) as i64,
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).proxy_service_name }).is_null() {
                        if !tool_setopt_skip(CURLOPT_PROXY_SERVICE_NAME) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_PROXY_SERVICE_NAME\0" as *const u8 as *const i8,
                                CURLOPT_PROXY_SERVICE_NAME,
                                (*config).proxy_service_name,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).service_name }).is_null() {
                        if !tool_setopt_skip(CURLOPT_SERVICE_NAME) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SERVICE_NAME\0" as *const u8 as *const i8,
                                CURLOPT_SERVICE_NAME,
                                (*config).service_name,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_FTP_ACCOUNT) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_FTP_ACCOUNT\0" as *const u8 as *const i8,
                            CURLOPT_FTP_ACCOUNT,
                            (*config).ftp_account,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_IGNORE_CONTENT_LENGTH) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_IGNORE_CONTENT_LENGTH\0" as *const u8 as *const i8,
                            CURLOPT_IGNORE_CONTENT_LENGTH,
                            if (*config).ignorecl as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_FTP_SKIP_PASV_IP) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_FTP_SKIP_PASV_IP\0" as *const u8 as *const i8,
                            CURLOPT_FTP_SKIP_PASV_IP,
                            if (*config).ftp_skip_ip as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_FTP_FILEMETHOD) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_FTP_FILEMETHOD\0" as *const u8 as *const i8,
                            CURLOPT_FTP_FILEMETHOD,
                            (*config).ftp_filemethod as i64,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if (unsafe { (*config).localport }) != 0 {
                        if !tool_setopt_skip(CURLOPT_LOCALPORT) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_LOCALPORT\0" as *const u8 as *const i8,
                                CURLOPT_LOCALPORT,
                                (*config).localport,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_LOCALPORTRANGE) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_LOCALPORTRANGE\0" as *const u8 as *const i8,
                                CURLOPT_LOCALPORTRANGE,
                                (*config).localportrange,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_FTP_ALTERNATIVE_TO_USER) {
                        result = unsafe { tool_setopt(
                            curl,
                            1 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_FTP_ALTERNATIVE_TO_USER\0" as *const u8 as *const i8,
                            CURLOPT_FTP_ALTERNATIVE_TO_USER,
                            (*config).ftp_alternative_to_user,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if unsafe { (*config).disable_sessionid } {
                        if !tool_setopt_skip(CURLOPT_SSL_SESSIONID_CACHE) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSL_SESSIONID_CACHE\0" as *const u8 as *const i8,
                                CURLOPT_SSL_SESSIONID_CACHE,
                                0 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).raw } {
                        if !tool_setopt_skip(CURLOPT_HTTP_CONTENT_DECODING) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_HTTP_CONTENT_DECODING\0" as *const u8 as *const i8,
                                CURLOPT_HTTP_CONTENT_DECODING,
                                0 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if !tool_setopt_skip(CURLOPT_HTTP_TRANSFER_DECODING) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_HTTP_TRANSFER_DECODING\0" as *const u8 as *const i8,
                                CURLOPT_HTTP_TRANSFER_DECODING,
                                0 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).nokeepalive }) {
                        if !tool_setopt_skip(CURLOPT_TCP_KEEPALIVE) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_TCP_KEEPALIVE\0" as *const u8 as *const i8,
                                CURLOPT_TCP_KEEPALIVE,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                        if (unsafe { (*config).alivetime }) != 0 {
                            if !tool_setopt_skip(CURLOPT_TCP_KEEPIDLE) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_TCP_KEEPIDLE\0" as *const u8 as *const i8,
                                    CURLOPT_TCP_KEEPIDLE,
                                    (*config).alivetime,
                                ) };
                                let _ = result as u64 != 0;
                            }
                            if !tool_setopt_skip(CURLOPT_TCP_KEEPINTVL) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    0 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_TCP_KEEPINTVL\0" as *const u8 as *const i8,
                                    CURLOPT_TCP_KEEPINTVL,
                                    (*config).alivetime,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                    } else if !tool_setopt_skip(CURLOPT_TCP_KEEPALIVE) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_TCP_KEEPALIVE\0" as *const u8 as *const i8,
                            CURLOPT_TCP_KEEPALIVE,
                            0 as i64,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if (unsafe { (*config).tftp_blksize }) != 0 {
                        if !tool_setopt_skip(CURLOPT_TFTP_BLKSIZE) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_TFTP_BLKSIZE\0" as *const u8 as *const i8,
                                CURLOPT_TFTP_BLKSIZE,
                                (*config).tftp_blksize,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).mail_from }).is_null() {
                        if !tool_setopt_skip(CURLOPT_MAIL_FROM) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_MAIL_FROM\0" as *const u8 as *const i8,
                                CURLOPT_MAIL_FROM,
                                (*config).mail_from,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).mail_rcpt }).is_null() {
                        if !tool_setopt_skip(CURLOPT_MAIL_RCPT) {
                            result = tool_setopt_slist(
                                curl,
                                global,
                                b"CURLOPT_MAIL_RCPT\0" as *const u8 as *const i8,
                                CURLOPT_MAIL_RCPT,
                                unsafe { (*config).mail_rcpt },
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if !tool_setopt_skip(CURLOPT_MAIL_RCPT_ALLLOWFAILS) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_MAIL_RCPT_ALLLOWFAILS\0" as *const u8 as *const i8,
                            CURLOPT_MAIL_RCPT_ALLLOWFAILS,
                            if (*config).mail_rcpt_allowfails as i32 != 0 {
                                1 as i64
                            } else {
                                0 as i64
                            },
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if unsafe { (*config).ftp_pret } {
                        if !tool_setopt_skip(CURLOPT_FTP_USE_PRET) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_FTP_USE_PRET\0" as *const u8 as *const i8,
                                CURLOPT_FTP_USE_PRET,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { (*config).create_file_mode }) != 0 {
                        if !tool_setopt_skip(CURLOPT_NEW_FILE_PERMS) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_NEW_FILE_PERMS\0" as *const u8 as *const i8,
                                CURLOPT_NEW_FILE_PERMS,
                                (*config).create_file_mode,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).proto_present } {
                        if !tool_setopt_skip(CURLOPT_PROTOCOLS) {
                            result = tool_setopt_flags(
                                curl,
                                global,
                                b"CURLOPT_PROTOCOLS\0" as *const u8 as *const i8,
                                CURLOPT_PROTOCOLS,
                                unsafe { setopt_nv_CURLPROTO.as_ptr() },
                                unsafe { (*config).proto },
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).proto_redir_present } {
                        if !tool_setopt_skip(CURLOPT_REDIR_PROTOCOLS) {
                            result = tool_setopt_flags(
                                curl,
                                global,
                                b"CURLOPT_REDIR_PROTOCOLS\0" as *const u8 as *const i8,
                                CURLOPT_REDIR_PROTOCOLS,
                                unsafe { setopt_nv_CURLPROTO.as_ptr() },
                                unsafe { (*config).proto_redir },
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { (*config).content_disposition }) as i32 != 0
                        && (unsafe { (*urlnode).flags }) & (1 as i32) << 2 as i32 != 0
                    {
                        (*(borrow_mut(&mut hdrcbdata)).unwrap()).honor_cd_filename = 1 as i32 != 0;
                    } else {
                        (*(borrow_mut(&mut hdrcbdata)).unwrap()).honor_cd_filename = 0 as i32 != 0;
                    }
                    let mut fresh55 = &mut ((*(borrow_mut(&mut hdrcbdata)).unwrap()).outs);
                    *fresh55 = outs;
                    let mut fresh56 = &mut ((*(borrow_mut(&mut hdrcbdata)).unwrap()).heads);
                    *fresh56 = heads;
                    let mut fresh57 = &mut ((*(borrow_mut(&mut hdrcbdata)).unwrap()).etag_save);
                    *fresh57 = etag_save;
                    let mut fresh58 = &mut ((*(borrow_mut(&mut hdrcbdata)).unwrap()).global);
                    *fresh58 = global;
                    let mut fresh59 = &mut ((*(borrow_mut(&mut hdrcbdata)).unwrap()).config);
                    *fresh59 = config;
                    if !tool_setopt_skip(CURLOPT_HEADERFUNCTION) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_HEADERFUNCTION\0" as *const u8 as *const i8,
                            CURLOPT_HEADERFUNCTION,
                            Some(tool_header_cb),
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !tool_setopt_skip(CURLOPT_HEADERDATA) {
                        result = unsafe { tool_setopt(
                            curl,
                            0 as i32 != 0,
                            global,
                            config,
                            b"CURLOPT_HEADERDATA\0" as *const u8 as *const i8,
                            CURLOPT_HEADERDATA,
                            per,
                        ) };
                        let _ = result as u64 != 0;
                    }
                    if !(unsafe { (*config).resolve }).is_null() {
                        if !tool_setopt_skip(CURLOPT_RESOLVE) {
                            result = tool_setopt_slist(
                                curl,
                                global,
                                b"CURLOPT_RESOLVE\0" as *const u8 as *const i8,
                                CURLOPT_RESOLVE,
                                unsafe { (*config).resolve },
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).connect_to }).is_null() {
                        if !tool_setopt_skip(CURLOPT_CONNECT_TO) {
                            result = tool_setopt_slist(
                                curl,
                                global,
                                b"CURLOPT_CONNECT_TO\0" as *const u8 as *const i8,
                                CURLOPT_CONNECT_TO,
                                unsafe { (*config).connect_to },
                            );
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { (*curlinfo).features }) & (1 as i32) << 14 as i32 != 0 {
                        if !(unsafe { (*config).tls_username }).is_null() {
                            if !tool_setopt_skip(CURLOPT_TLSAUTH_USERNAME) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_TLSAUTH_USERNAME\0" as *const u8 as *const i8,
                                    CURLOPT_TLSAUTH_USERNAME,
                                    (*config).tls_username,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if !(unsafe { (*config).tls_password }).is_null() {
                            if !tool_setopt_skip(CURLOPT_TLSAUTH_PASSWORD) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_TLSAUTH_PASSWORD\0" as *const u8 as *const i8,
                                    CURLOPT_TLSAUTH_PASSWORD,
                                    (*config).tls_password,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if !(unsafe { (*config).tls_authtype }).is_null() {
                            if !tool_setopt_skip(CURLOPT_TLSAUTH_TYPE) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_TLSAUTH_TYPE\0" as *const u8 as *const i8,
                                    CURLOPT_TLSAUTH_TYPE,
                                    (*config).tls_authtype,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if !(unsafe { (*config).proxy_tls_username }).is_null() {
                            if !tool_setopt_skip(CURLOPT_PROXY_TLSAUTH_USERNAME) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_PROXY_TLSAUTH_USERNAME\0" as *const u8 as *const i8,
                                    CURLOPT_PROXY_TLSAUTH_USERNAME,
                                    (*config).proxy_tls_username,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if !(unsafe { (*config).proxy_tls_password }).is_null() {
                            if !tool_setopt_skip(CURLOPT_PROXY_TLSAUTH_PASSWORD) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_PROXY_TLSAUTH_PASSWORD\0" as *const u8 as *const i8,
                                    CURLOPT_PROXY_TLSAUTH_PASSWORD,
                                    (*config).proxy_tls_password,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                        if !(unsafe { (*config).proxy_tls_authtype }).is_null() {
                            if !tool_setopt_skip(CURLOPT_PROXY_TLSAUTH_TYPE) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_PROXY_TLSAUTH_TYPE\0" as *const u8 as *const i8,
                                    CURLOPT_PROXY_TLSAUTH_TYPE,
                                    (*config).proxy_tls_authtype,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        }
                    }
                    if (unsafe { (*config).gssapi_delegation }) != 0 {
                        if !tool_setopt_skip(CURLOPT_GSSAPI_DELEGATION) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_GSSAPI_DELEGATION\0" as *const u8 as *const i8,
                                CURLOPT_GSSAPI_DELEGATION,
                                (*config).gssapi_delegation,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).mail_auth }).is_null() {
                        if !tool_setopt_skip(CURLOPT_MAIL_AUTH) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_MAIL_AUTH\0" as *const u8 as *const i8,
                                CURLOPT_MAIL_AUTH,
                                (*config).mail_auth,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).sasl_authzid }).is_null() {
                        if !tool_setopt_skip(CURLOPT_SASL_AUTHZID) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SASL_AUTHZID\0" as *const u8 as *const i8,
                                CURLOPT_SASL_AUTHZID,
                                (*config).sasl_authzid,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).sasl_ir } {
                        if !tool_setopt_skip(CURLOPT_SASL_IR) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SASL_IR\0" as *const u8 as *const i8,
                                CURLOPT_SASL_IR,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).nonpn } {
                        if !tool_setopt_skip(CURLOPT_SSL_ENABLE_NPN) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSL_ENABLE_NPN\0" as *const u8 as *const i8,
                                CURLOPT_SSL_ENABLE_NPN,
                                0 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).noalpn } {
                        if !tool_setopt_skip(CURLOPT_SSL_ENABLE_ALPN) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_SSL_ENABLE_ALPN\0" as *const u8 as *const i8,
                                CURLOPT_SSL_ENABLE_ALPN,
                                0 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).unix_socket_path }).is_null() {
                        if unsafe { (*config).abstract_unix_socket } {
                            if !tool_setopt_skip(CURLOPT_ABSTRACT_UNIX_SOCKET) {
                                result = unsafe { tool_setopt(
                                    curl,
                                    1 as i32 != 0,
                                    global,
                                    config,
                                    b"CURLOPT_ABSTRACT_UNIX_SOCKET\0" as *const u8 as *const i8,
                                    CURLOPT_ABSTRACT_UNIX_SOCKET,
                                    (*config).unix_socket_path,
                                ) };
                                let _ = result as u64 != 0;
                            }
                        } else if !tool_setopt_skip(CURLOPT_UNIX_SOCKET_PATH) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_UNIX_SOCKET_PATH\0" as *const u8 as *const i8,
                                CURLOPT_UNIX_SOCKET_PATH,
                                (*config).unix_socket_path,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).proto_default }).is_null() {
                        if !tool_setopt_skip(CURLOPT_DEFAULT_PROTOCOL) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_DEFAULT_PROTOCOL\0" as *const u8 as *const i8,
                                CURLOPT_DEFAULT_PROTOCOL,
                                (*config).proto_default,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { (*config).expect100timeout }) > 0 as i32 as f64 {
                        if !tool_setopt_skip(CURLOPT_EXPECT_100_TIMEOUT_MS) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_EXPECT_100_TIMEOUT_MS\0" as *const u8 as *const i8,
                                CURLOPT_EXPECT_100_TIMEOUT_MS,
                                ((*config).expect100timeout * 1000 as i32 as f64) as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).tftp_no_options } {
                        if !tool_setopt_skip(CURLOPT_TFTP_NO_OPTIONS) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_TFTP_NO_OPTIONS\0" as *const u8 as *const i8,
                                CURLOPT_TFTP_NO_OPTIONS,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if (unsafe { (*config).happy_eyeballs_timeout_ms }) != 200 as i64 {
                        if !tool_setopt_skip(CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS\0" as *const u8 as *const i8,
                                CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS,
                                (*config).happy_eyeballs_timeout_ms,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).haproxy_protocol } {
                        if !tool_setopt_skip(CURLOPT_HAPROXYPROTOCOL) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_HAPROXYPROTOCOL\0" as *const u8 as *const i8,
                                CURLOPT_HAPROXYPROTOCOL,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if unsafe { (*config).disallow_username_in_url } {
                        if !tool_setopt_skip(CURLOPT_DISALLOW_USERNAME_IN_URL) {
                            result = unsafe { tool_setopt(
                                curl,
                                0 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_DISALLOW_USERNAME_IN_URL\0" as *const u8 as *const i8,
                                CURLOPT_DISALLOW_USERNAME_IN_URL,
                                1 as i64,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).altsvc }).is_null() {
                        if !tool_setopt_skip(CURLOPT_ALTSVC) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_ALTSVC\0" as *const u8 as *const i8,
                                CURLOPT_ALTSVC,
                                (*config).altsvc,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    if !(unsafe { (*config).hsts }).is_null() {
                        if !tool_setopt_skip(CURLOPT_HSTS) {
                            result = unsafe { tool_setopt(
                                curl,
                                1 as i32 != 0,
                                global,
                                config,
                                b"CURLOPT_HSTS\0" as *const u8 as *const i8,
                                CURLOPT_HSTS,
                                (*config).hsts,
                            ) };
                            let _ = result as u64 != 0;
                        }
                    }
                    (unsafe { (*per).retry_sleep_default = if (*config).retry_delay != 0 {
                        (*config).retry_delay * 1000 as i64
                    } else {
                        1000 as i64
                    } });
                    (unsafe { (*per).retry_numretries = (*config).req_retry });
                    (unsafe { (*per).retry_sleep = (*per).retry_sleep_default });
                    (unsafe { (*per).retrystart = tvnow() });
                    let mut fresh60 = unsafe { &mut ((*state).li) };
                    *fresh60 = (*fresh60).wrapping_add(1);
                    if (unsafe { (*state).li }) >= urlnum {
                        (unsafe { (*state).li = 0 as i32 as u64 });
                        (unsafe { (*state).urlnum = 0 as i32 as u64 });
                        glob_cleanup(unsafe { (*state).urls });
                        let mut fresh61 = unsafe { &mut ((*state).urls) };
                        *fresh61 = 0 as *mut URLGlob;
                        let mut fresh62 = unsafe { &mut ((*state).up) };
                        *fresh62 = (*fresh62).wrapping_add(1);
                        (unsafe { free((*state).uploadfile as *mut libc::c_void) });
                        let mut fresh63 = unsafe { &mut ((*state).uploadfile) };
                        *fresh63 = 0 as *mut i8;
                    }
                    break;
                }
            } else {
                (unsafe { free((*urlnode).outfile as *mut libc::c_void) });
                let mut fresh64 = unsafe { &mut ((*urlnode).outfile) };
                *fresh64 = 0 as *mut i8;
                (unsafe { free((*urlnode).infile as *mut libc::c_void) });
                let mut fresh65 = unsafe { &mut ((*urlnode).infile) };
                *fresh65 = 0 as *mut i8;
                (unsafe { (*urlnode).flags = 0 as i32 });
                glob_cleanup(unsafe { (*state).urls });
                let mut fresh66 = unsafe { &mut ((*state).urls) };
                *fresh66 = 0 as *mut URLGlob;
                (unsafe { (*state).urlnum = 0 as i32 as u64 });
                (unsafe { free((*state).outfiles as *mut libc::c_void) });
                let mut fresh67 = unsafe { &mut ((*state).outfiles) };
                *fresh67 = 0 as *mut i8;
                (unsafe { free((*state).uploadfile as *mut libc::c_void) });
                let mut fresh68 = unsafe { &mut ((*state).uploadfile) };
                *fresh68 = 0 as *mut i8;
                if !(unsafe { (*state).inglob }).is_null() {
                    glob_cleanup(unsafe { (*state).inglob });
                    let mut fresh69 = unsafe { &mut ((*state).inglob) };
                    *fresh69 = 0 as *mut URLGlob;
                }
                let mut fresh70 = unsafe { &mut ((*config).state.urlnode) };
                *fresh70 = unsafe { (*urlnode).next };
                (unsafe { (*state).up = 0 as i32 as u64 });
            }
        }
    }
    if !*(borrow(&added)).unwrap() || result as u32 != 0 {
        *(borrow_mut(&mut added)).unwrap() = 0 as i32 != 0;
        single_transfer_cleanup(config);
    }
    return result;
}
static mut all_added: i64 = 0;
extern "C" fn add_parallel_transfers<'a1, 'a2>(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut multi: *mut crate::src::lib::http2::Curl_multi,
    mut share: *mut crate::src::lib::asyn_thread::Curl_share,
    mut morep: Option<&'a1 mut bool>,
    mut addedp: Option<&'a2 mut bool>,
) -> u32 {
    let mut per: *mut crate::src::src::tool_cb_hdr::per_transfer<'_> = 0 as *mut per_transfer;
    let mut result: u32 = CURLE_OK;
    let mut mcode: i32 = CURLM_OK;
    let mut sleeping: bool = 0 as i32 != 0;
    *(borrow_mut(&mut addedp)).unwrap() = 0 as i32 != 0;
    *(borrow_mut(&mut morep)).unwrap() = 0 as i32 != 0;
    result = create_transfer(global, share, borrow_mut(&mut addedp));
    if result as u64 != 0 {
        return result;
    }
    per = unsafe { transfers };
    while !per.is_null() && (unsafe { all_added }) < (unsafe { (*global).parallel_max }) {
        let mut getadded: bool = 0 as i32 != 0;
        if !(unsafe { (*per).added }) {
            if (unsafe { (*per).startat }) != 0 && (unsafe { time(0 as *mut time_t) }) < (unsafe { (*per).startat }) {
                sleeping = 1 as i32 != 0;
            } else {
                result = pre_transfer(global, per);
                if result as u64 != 0 {
                    return result;
                }
                (unsafe { curl_easy_setopt(
                    (*per).curl,
                    CURLOPT_PIPEWAIT,
                    if (*global).parallel_connect as i32 != 0 {
                        0 as i64
                    } else {
                        1 as i64
                    },
                ) });
                (unsafe { curl_easy_setopt((*per).curl, CURLOPT_PRIVATE, per) });
                (unsafe { curl_easy_setopt((*per).curl, CURLOPT_XFERINFOFUNCTION, Some(xferinfo_cb)) });
                (unsafe { curl_easy_setopt((*per).curl, CURLOPT_XFERINFODATA, per) });
                (unsafe { curl_easy_setopt((*per).curl, CURLOPT_NOPROGRESS, 0 as i64) });
                mcode = curl_multi_add_handle(multi, unsafe { (*per).curl });
                if mcode as u64 != 0 {
                    return CURLE_OUT_OF_MEMORY;
                }
                result = create_transfer(global, share, Some(&mut getadded));
                if result as u64 != 0 {
                    return result;
                }
                (unsafe { (*per).added = 1 as i32 != 0 });
                (unsafe { all_added += 1 });
                *(borrow_mut(&mut addedp)).unwrap() = 1 as i32 != 0;
            }
        }
        per = unsafe { (*per).next };
    }
    *(borrow_mut(&mut morep)).unwrap() = if !per.is_null() || sleeping as i32 != 0 {
        1 as i32
    } else {
        0 as i32
    } != 0;
    return CURLE_OK;
}
extern "C" fn parallel_transfers(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut share: *mut crate::src::lib::asyn_thread::Curl_share,
) -> u32 {
    let mut multi: *mut crate::src::lib::http2::Curl_multi = 0 as *mut CURLM;
    let mut mcode: i32 = CURLM_OK;
    let mut result: u32 = CURLE_OK;
    let mut still_running: i32 = 1 as i32;
    let mut start: crate::src::lib::openldap::timeval = tvnow();
    let mut more_transfers: bool = false;
    let mut added_transfers: bool = false;
    let mut wrapitup: bool = 0 as i32 != 0;
    let mut wrapitup_processed: bool = 0 as i32 != 0;
    let mut tick: i64 = unsafe { time(0 as *mut time_t) };
    multi = curl_multi_init();
    if multi.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    result = add_parallel_transfers(
        global,
        multi,
        share,
        Some(&mut more_transfers),
        Some(&mut added_transfers),
    );
    if result as u64 != 0 {
        curl_multi_cleanup(multi);
        return result;
    }
    while mcode as u64 == 0 && (still_running != 0 || more_transfers as i32 != 0) {
        if wrapitup {
            if still_running == 0 {
                break;
            }
            if !wrapitup_processed {
                let mut per: *mut crate::src::src::tool_cb_hdr::per_transfer<'_> =
                    0 as *mut per_transfer;
                per = unsafe { transfers };
                while !per.is_null() {
                    if unsafe { (*per).added } {
                        (unsafe { (*per).abort = 1 as i32 != 0 });
                    }
                    per = unsafe { (*per).next };
                }
                wrapitup_processed = 1 as i32 != 0;
            }
        }
        mcode = curl_multi_poll(
            multi,
            0 as *mut crate::src::lib::easy::curl_waitfd,
            0 as i32 as u32,
            1000 as i32,
            Option::<&'_ mut i32>::None,
        );
        if mcode as u64 == 0 {
            mcode = curl_multi_perform(multi, Some(&mut still_running));
        }
        progress_meter(global, Some(&mut start), 0 as i32 != 0);
        if !(mcode as u64 == 0) {
            continue;
        }
        let mut rc: i32 = 0;
        let mut msg: Option<&'_ mut crate::src::lib::http2::CURLMsg> =
            Option::<&'_ mut crate::src::lib::http2::CURLMsg>::None;
        let mut checkmore: bool = 0 as i32 != 0;
        loop {
            msg = curl_multi_info_read(multi, Some(&mut rc));
            if !borrow(&msg).is_none() {
                let mut retry: bool = false;
                let mut delay: i64 = 0;
                let mut ended: *mut crate::src::src::tool_cb_hdr::per_transfer<'_> =
                    0 as *mut per_transfer;
                let mut easy: *mut crate::src::lib::http2::Curl_easy =
                    (*(borrow_mut(&mut msg)).unwrap()).easy_handle;
                let mut tres: u32 = unsafe { (*(borrow_mut(&mut msg)).unwrap()).data.result };
                (unsafe { curl_easy_getinfo(
                    easy,
                    CURLINFO_PRIVATE,
                    &mut ended as *mut *mut per_transfer as *mut libc::c_void,
                ) });
                curl_multi_remove_handle(multi, easy);
                if (unsafe { (*ended).abort }) as i32 != 0
                    && tres as u32 == CURLE_ABORTED_BY_CALLBACK as i32 as u32
                {
                    (unsafe { curl_msnprintf(
                        ((*ended).errorbuffer).as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 256]>() as u64,
                        b"Transfer aborted due to critical error in another transfer\0" as *const u8
                            as *const i8,
                    ) });
                }
                tres = post_per_transfer(global, ended, tres, Some(&mut retry), Some(&mut delay));
                progress_finalize(ended);
                (unsafe { all_added -= 1 });
                checkmore = 1 as i32 != 0;
                if retry {
                    (unsafe { (*ended).added = 0 as i32 != 0 });
                    (unsafe { (*ended).startat = if delay != 0 {
                        time(0 as *mut time_t) + delay / 1000 as i32 as i64
                    } else {
                        0 as i32 as i64
                    } });
                } else {
                    if tres as u32 != 0 && (!(unsafe { (*ended).abort }) || result as u64 == 0) {
                        result = tres;
                    }
                    if is_fatal_error(result) as i32 != 0
                        || result as u32 != 0 && (unsafe { (*global).fail_early }) as i32 != 0
                    {
                        wrapitup = 1 as i32 != 0;
                    }
                    del_per_transfer(ended);
                }
            }
            if borrow(&msg).is_none() {
                break;
            }
        }
        if wrapitup {
            if !(still_running != 0) {
                break;
            }
        } else {
            if !checkmore {
                let mut tock: i64 = unsafe { time(0 as *mut time_t) };
                if tick != tock {
                    checkmore = 1 as i32 != 0;
                    tick = tock;
                }
            }
            if checkmore {
                let mut tres_0: u32 = add_parallel_transfers(
                    global,
                    multi,
                    share,
                    Some(&mut more_transfers),
                    Some(&mut added_transfers),
                );
                if tres_0 as u64 != 0 {
                    result = tres_0;
                }
                if added_transfers {
                    still_running = 1 as i32;
                }
            }
            if is_fatal_error(result) as i32 != 0
                || result as u32 != 0 && (unsafe { (*global).fail_early }) as i32 != 0
            {
                wrapitup = 1 as i32 != 0;
            }
        }
    }
    progress_meter(global, Some(&mut start), 1 as i32 != 0);
    if mcode as u64 != 0 {
        result = (if mcode as i32 == CURLM_OUT_OF_MEMORY as i32 {
            CURLE_OUT_OF_MEMORY as i32
        } else {
            CURLE_BAD_FUNCTION_ARGUMENT as i32
        }) as CURLcode;
    }
    curl_multi_cleanup(multi);
    return result;
}
extern "C" fn serial_transfers(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut share: *mut crate::src::lib::asyn_thread::Curl_share,
) -> u32 {
    let mut returncode: u32 = CURLE_OK;
    let mut result: u32 = CURLE_OK;
    let mut per: *mut crate::src::src::tool_cb_hdr::per_transfer<'_> = 0 as *mut per_transfer;
    let mut added: bool = 0 as i32 != 0;
    result = create_transfer(global, share, Some(&mut added));
    if result as u32 != 0 || !added {
        return result;
    }
    per = unsafe { transfers };
    while !per.is_null() {
        let mut retry: bool = false;
        let mut delay: i64 = 0;
        let mut bailout: bool = 0 as i32 != 0;
        result = pre_transfer(global, per);
        if result as u64 != 0 {
            break;
        }
        if !(unsafe { (*global).libcurl }).is_null() {
            result = easysrc_perform();
            if result as u64 != 0 {
                break;
            }
        }
        result = curl_easy_perform(unsafe { (*per).curl });
        returncode = post_per_transfer(global, per, result, Some(&mut retry), Some(&mut delay));
        if retry {
            tool_go_sleep(delay);
        } else {
            if is_fatal_error(returncode) as i32 != 0
                || returncode as u32 != 0 && (unsafe { (*global).fail_early }) as i32 != 0
            {
                bailout = 1 as i32 != 0;
            } else {
                result = create_transfer(global, share, Some(&mut added));
                if result as u64 != 0 {
                    bailout = 1 as i32 != 0;
                }
            }
            per = del_per_transfer(per);
            if bailout {
                break;
            }
        }
    }
    if returncode as u64 != 0 {
        result = returncode;
    }
    if result as u64 != 0 {
        single_transfer_cleanup(unsafe { (*global).current });
    }
    return result;
}
extern "C" fn transfer_per_config<'a1>(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut config: *mut crate::src::src::tool_cb_dbg::OperationConfig,
    mut share: *mut crate::src::lib::asyn_thread::Curl_share,
    mut added: Option<&'a1 mut bool>,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut capath_from_env: bool = false;
    *(borrow_mut(&mut added)).unwrap() = 0 as i32 != 0;
    if (unsafe { (*config).url_list }).is_null() || (unsafe { (*(*config).url_list).url }).is_null() {
        (unsafe { helpf(
            (*global).errors,
            b"no URL specified!\n\0" as *const u8 as *const i8,
        ) });
        return CURLE_FAILED_INIT;
    }
    capath_from_env = 0 as i32 != 0;
    if (unsafe { (*config).cacert }).is_null()
        && (unsafe { (*config).capath }).is_null()
        && (!(unsafe { (*config).insecure_ok }) || !(unsafe { (*config).doh_url }).is_null() && !(unsafe { (*config).doh_insecure_ok }))
    {
        let mut curltls: *mut crate::src::lib::http2::Curl_easy = curl_easy_init();
        let mut tls_backend_info: *mut crate::src::lib::http2::curl_tlssessioninfo =
            0 as *mut curl_tlssessioninfo;
        result = unsafe { curl_easy_getinfo(
            curltls,
            CURLINFO_TLS_SSL_PTR,
            &mut tls_backend_info as *mut *mut curl_tlssessioninfo,
        ) };
        if result as u64 != 0 {
            return result;
        }
        if (unsafe { (*tls_backend_info).backend }) as u32 != CURLSSLBACKEND_SCHANNEL as i32 as u32 {
            let mut env: *mut i8 = 0 as *mut i8;
            env = curl_getenv(b"CURL_CA_BUNDLE\0" as *const u8 as *const i8);
            if !env.is_null() {
                let mut fresh71 = unsafe { &mut ((*config).cacert) };
                *fresh71 = unsafe { strdup(env) };
                if (unsafe { (*config).cacert }).is_null() {
                    curl_free(env as *mut libc::c_void);
                    (unsafe { errorf(global, b"out of memory\n\0" as *const u8 as *const i8) });
                    return CURLE_OUT_OF_MEMORY;
                }
            } else {
                env = curl_getenv(b"SSL_CERT_DIR\0" as *const u8 as *const i8);
                if !env.is_null() {
                    let mut fresh72 = unsafe { &mut ((*config).capath) };
                    *fresh72 = unsafe { strdup(env) };
                    if (unsafe { (*config).capath }).is_null() {
                        curl_free(env as *mut libc::c_void);
                        (unsafe { helpf(
                            (*global).errors,
                            b"out of memory\n\0" as *const u8 as *const i8,
                        ) });
                        return CURLE_OUT_OF_MEMORY;
                    }
                    capath_from_env = 1 as i32 != 0;
                } else {
                    env = curl_getenv(b"SSL_CERT_FILE\0" as *const u8 as *const i8);
                    if !env.is_null() {
                        let mut fresh73 = unsafe { &mut ((*config).cacert) };
                        *fresh73 = unsafe { strdup(env) };
                        if (unsafe { (*config).cacert }).is_null() {
                            curl_free(env as *mut libc::c_void);
                            (unsafe { errorf(global, b"out of memory\n\0" as *const u8 as *const i8) });
                            return CURLE_OUT_OF_MEMORY;
                        }
                    }
                }
            }
            if !env.is_null() {
                curl_free(env as *mut libc::c_void);
            }
        }
        curl_easy_cleanup(curltls);
    }
    if result as u64 == 0 {
        result = single_transfer(
            global,
            config,
            share,
            capath_from_env,
            borrow_mut(&mut added),
        );
    }
    return result;
}
extern "C" fn create_transfer<'a1>(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut share: *mut crate::src::lib::asyn_thread::Curl_share,
    mut added: Option<&'a1 mut bool>,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    *(borrow_mut(&mut added)).unwrap() = 0 as i32 != 0;
    while !(unsafe { (*global).current }).is_null() {
        result = transfer_per_config(global, unsafe { (*global).current }, share, borrow_mut(&mut added));
        if !(result as u64 == 0 && !*(borrow(&added)).unwrap()) {
            break;
        }
        let mut fresh74 = unsafe { &mut ((*global).current) };
        *fresh74 = unsafe { (*(*global).current).next };
    }
    return result;
}
extern "C" fn run_all_transfers(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut share: *mut crate::src::lib::asyn_thread::Curl_share,
    mut result: u32,
) -> u32 {
    let mut orig_noprogress: bool = unsafe { (*global).noprogress };
    let mut orig_isatty: bool = unsafe { (*global).isatty };
    let mut per: *mut crate::src::src::tool_cb_hdr::per_transfer<'_> = 0 as *mut per_transfer;
    if result as u64 == 0 {
        if unsafe { (*global).parallel } {
            result = parallel_transfers(global, share);
        } else {
            result = serial_transfers(global, share);
        }
    }
    per = unsafe { transfers };
    while !per.is_null() {
        let mut retry: bool = false;
        let mut delay: i64 = 0;
        let mut result2: u32 =
            post_per_transfer(global, per, result, Some(&mut retry), Some(&mut delay));
        if result as u64 == 0 {
            result = result2;
        }
        clean_getout(unsafe { (*per).config });
        per = del_per_transfer(per);
    }
    (unsafe { (*global).noprogress = orig_noprogress });
    (unsafe { (*global).isatty = orig_isatty });
    return result;
}
#[no_mangle]
pub extern "C" fn operate(
    mut global: *mut crate::src::src::tool_cb_dbg::GlobalConfig,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut first_arg: *mut i8 = if argc > 1 as i32 {
        unsafe { strdup(*argv.offset(1 as i32 as isize)) }
    } else {
        0 as *mut i8
    };
    (unsafe { setlocale(6 as i32, b"\0" as *const u8 as *const i8) });
    if argc == 1 as i32
        || !first_arg.is_null()
            && (unsafe { strncmp(
                first_arg,
                b"-q\0" as *const u8 as *const i8,
                2 as i32 as u64,
            ) }) != 0
            && curl_strequal(first_arg, b"--disable\0" as *const u8 as *const i8) == 0
    {
        parseconfig(0 as *const i8, global);
        if argc < 2 as i32 && (unsafe { (*(*global).first).url_list }).is_null() {
            (unsafe { helpf((*global).errors, 0 as *const i8) });
            result = CURLE_FAILED_INIT;
        }
    }
    if !first_arg.is_null() {
        (unsafe { free(first_arg as *mut libc::c_void) });
        first_arg = 0 as *mut i8;
    }
    if result as u64 == 0 {
        let mut res: u32 = parse_args(global, argc, argv);
        if res as u64 != 0 {
            result = CURLE_OK;
            if res as u32 == PARAM_HELP_REQUESTED as i32 as u32 {
                tool_help(unsafe { (*global).help_category });
            } else if res as u32 == PARAM_MANUAL_REQUESTED as i32 as u32 {
                hugehelp();
            } else if res as u32 == PARAM_VERSION_INFO_REQUESTED as i32 as u32 {
                tool_version_info();
            } else if res as u32 == PARAM_ENGINES_REQUESTED as i32 as u32 {
                tool_list_engines();
            } else if res as u32 == PARAM_LIBCURL_UNSUPPORTED_PROTOCOL as i32 as u32 {
                result = CURLE_UNSUPPORTED_PROTOCOL;
            } else {
                result = CURLE_FAILED_INIT;
            }
        } else {
            if !(unsafe { (*global).libcurl }).is_null() {
                result = easysrc_init();
            }
            if result as u64 == 0 {
                let mut count: u64 = 0 as i32 as size_t;
                let mut operation: *mut crate::src::src::tool_cb_dbg::OperationConfig =
                    unsafe { (*global).first };
                let mut share: *mut crate::src::lib::asyn_thread::Curl_share = curl_share_init();
                if share.is_null() {
                    if !(unsafe { (*global).libcurl }).is_null() {
                        easysrc_cleanup();
                    }
                    return CURLE_OUT_OF_MEMORY;
                }
                (unsafe { curl_share_setopt(share, CURLSHOPT_SHARE, CURL_LOCK_DATA_COOKIE as i32) });
                (unsafe { curl_share_setopt(share, CURLSHOPT_SHARE, CURL_LOCK_DATA_DNS as i32) });
                (unsafe { curl_share_setopt(share, CURLSHOPT_SHARE, CURL_LOCK_DATA_SSL_SESSION as i32) });
                (unsafe { curl_share_setopt(share, CURLSHOPT_SHARE, CURL_LOCK_DATA_CONNECT as i32) });
                (unsafe { curl_share_setopt(share, CURLSHOPT_SHARE, CURL_LOCK_DATA_PSL as i32) });
                loop {
                    let mut fresh75 = count;
                    count = count.wrapping_add(1);
                    result = get_args(operation, fresh75);
                    operation = unsafe { (*operation).next };
                    if !(result as u64 == 0 && !operation.is_null()) {
                        break;
                    }
                }
                let mut fresh76 = unsafe { &mut ((*global).current) };
                *fresh76 = unsafe { (*global).first };
                result = run_all_transfers(global, share, result);
                curl_share_cleanup(share);
                if !(unsafe { (*global).libcurl }).is_null() {
                    easysrc_cleanup();
                    dumpeasysrc(global);
                }
            } else {
                (unsafe { errorf(global, b"out of memory\n\0" as *const u8 as *const i8) });
            }
        }
    }
    return result;
}
use crate::laertes_rt::*;
