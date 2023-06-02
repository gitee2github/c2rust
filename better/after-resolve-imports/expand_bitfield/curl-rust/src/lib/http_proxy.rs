use ::libc;
use ::c2rust_bitfields;
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn sscanf(_: *const i8, _: *const i8, _: ...) -> i32;
    
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::lib::connect::Curl_closesocket;
pub use crate::src::lib::connect::Curl_conn_data_pending;
pub use crate::src::lib::connect::Curl_conncontrol;
pub use crate::src::lib::connect::Curl_timeleft;
pub use crate::src::lib::dynbuf::Curl_dyn_add;
pub use crate::src::lib::dynbuf::Curl_dyn_addf;
pub use crate::src::lib::dynbuf::Curl_dyn_addn;
pub use crate::src::lib::dynbuf::Curl_dyn_free;
pub use crate::src::lib::dynbuf::Curl_dyn_init;
pub use crate::src::lib::dynbuf::Curl_dyn_len;
pub use crate::src::lib::dynbuf::Curl_dyn_ptr;
pub use crate::src::lib::dynbuf::Curl_dyn_reset;
pub use crate::src::lib::http::Curl_add_custom_headers;
pub use crate::src::lib::http::Curl_buffer_send;
pub use crate::src::lib::http::Curl_checkProxyheaders;
pub use crate::src::lib::http::Curl_compareheader;
pub use crate::src::lib::http::Curl_copy_header_value;
pub use crate::src::lib::http::Curl_http_auth_act;
pub use crate::src::lib::http::Curl_http_input_auth;
pub use crate::src::lib::http::Curl_http_output_auth;
pub use crate::src::lib::http_chunks::Curl_httpchunk_init;
pub use crate::src::lib::http_chunks::Curl_httpchunk_read;
pub use crate::src::lib::mprintf::curl_maprintf;
pub use crate::src::lib::progress::Curl_pgrsUpdate;
pub use crate::src::lib::sendf::Curl_client_write;
pub use crate::src::lib::sendf::Curl_debug;
pub use crate::src::lib::sendf::Curl_failf;
pub use crate::src::lib::sendf::Curl_infof;
pub use crate::src::lib::sendf::Curl_read;
pub use crate::src::lib::sendf::Curl_write;
pub use crate::src::lib::strcase::curl_strnequal;
pub use crate::src::lib::strtoofft::curlx_strtoofft;
pub use crate::src::lib::transfer::Curl_fillreadbuffer;
pub use crate::src::lib::transfer::Curl_get_upload_buffer;
pub use crate::src::lib::vtls::vtls::Curl_ssl_connect_nonblocking;
pub use crate::src::lib::altsvc::altsvcinfo;
pub use crate::src::lib::asyn_thread::Curl_share;
pub use crate::src::lib::asyn_thread::thread_data;
pub use crate::src::lib::content_encoding::contenc_writer;
pub use crate::src::lib::easy::hsts;
pub use crate::src::lib::http2::curl_pushheaders;
pub use crate::src::lib::conncache::Gsasl_session;
pub use crate::src::lib::easy::Curl_ccalloc;
pub use crate::src::lib::easy::Curl_cfree;
pub use crate::src::lib::escape::Gsasl;
pub use crate::src::lib::mqtt::_IO_codecvt;
pub use crate::src::lib::speedcheck::nghttp2_session;
pub use crate::src::lib::urlapi::psl_ctx_st;
pub use crate::src::src::tool_cb_rea::_IO_wide_data;
pub use crate::src::src::tool_msgs::_IO_marker;
pub use crate::src::lib::openldap::ldapconninfo;
pub use crate::src::lib::openldap::ldapreqinfo;
pub use crate::src::lib::smb::smb_request;
pub use crate::src::lib::telnet::TELNET;
pub use crate::src::lib::tftp::tftp_state_data;
pub use crate::src::lib::urlapi::Curl_URL;
pub use crate::src::lib::vtls::openssl::ssl_backend_data;
pub type __uint8_t = crate::src::lib::http2::__uint8_t;
pub type __int32_t = crate::src::lib::http2::__int32_t;
pub type __uint32_t = crate::src::lib::http2::__uint32_t;
pub type __off_t = crate::src::lib::http2::__off_t;
pub type __off64_t = crate::src::lib::http2::__off64_t;
pub type __pid_t = crate::src::lib::http2::__pid_t;
pub type __time_t = crate::src::lib::http2::__time_t;
pub type __ssize_t = crate::src::lib::http2::__ssize_t;
pub type __socklen_t = crate::src::lib::http2::__socklen_t;
pub type pid_t = crate::src::lib::http2::pid_t;
pub type ssize_t = crate::src::lib::http2::ssize_t;
pub type time_t = crate::src::lib::http2::time_t;
pub type size_t = crate::src::lib::http2::size_t;
pub type int32_t = crate::src::lib::http2::int32_t;
pub type socklen_t = crate::src::lib::http2::socklen_t;
pub type sa_family_t = crate::src::lib::http2::sa_family_t;
// #[derive(Copy, Clone)]

pub type sockaddr = crate::src::lib::http2::sockaddr;
pub type curl_socklen_t = crate::src::lib::http2::curl_socklen_t;
pub type curl_off_t = crate::src::lib::http2::curl_off_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::lib::http2::_IO_FILE;
pub type _IO_lock_t = crate::src::lib::http2::_IO_lock_t;
pub type FILE = crate::src::lib::http2::FILE;
// #[derive(Copy, Clone)]

pub type Curl_easy = crate::src::lib::http2::Curl_easy;
// #[derive(Copy, Clone)]

pub type curl_tlssessioninfo = crate::src::lib::http2::curl_tlssessioninfo;
pub type curl_sslbackend = crate::src::lib::http2::curl_sslbackend;
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
// #[derive(Copy, Clone, BitfieldStruct)]

pub type PureInfo = crate::src::lib::http2::PureInfo;
pub type bit = crate::src::lib::http2::bit;
pub type CURLproxycode = crate::src::lib::http2::CURLproxycode;
pub const CURLPX_LAST: CURLproxycode = 34;
pub const CURLPX_USER_REJECTED: CURLproxycode = 33;
pub const CURLPX_UNKNOWN_MODE: CURLproxycode = 32;
pub const CURLPX_UNKNOWN_FAIL: CURLproxycode = 31;
pub const CURLPX_SEND_REQUEST: CURLproxycode = 30;
pub const CURLPX_SEND_CONNECT: CURLproxycode = 29;
pub const CURLPX_SEND_AUTH: CURLproxycode = 28;
pub const CURLPX_RESOLVE_HOST: CURLproxycode = 27;
pub const CURLPX_REQUEST_FAILED: CURLproxycode = 26;
pub const CURLPX_REPLY_UNASSIGNED: CURLproxycode = 25;
pub const CURLPX_REPLY_TTL_EXPIRED: CURLproxycode = 24;
pub const CURLPX_REPLY_NOT_ALLOWED: CURLproxycode = 23;
pub const CURLPX_REPLY_NETWORK_UNREACHABLE: CURLproxycode = 22;
pub const CURLPX_REPLY_HOST_UNREACHABLE: CURLproxycode = 21;
pub const CURLPX_REPLY_GENERAL_SERVER_FAILURE: CURLproxycode = 20;
pub const CURLPX_REPLY_CONNECTION_REFUSED: CURLproxycode = 19;
pub const CURLPX_REPLY_COMMAND_NOT_SUPPORTED: CURLproxycode = 18;
pub const CURLPX_REPLY_ADDRESS_TYPE_NOT_SUPPORTED: CURLproxycode = 17;
pub const CURLPX_RECV_REQACK: CURLproxycode = 16;
pub const CURLPX_RECV_CONNECT: CURLproxycode = 15;
pub const CURLPX_RECV_AUTH: CURLproxycode = 14;
pub const CURLPX_RECV_ADDRESS: CURLproxycode = 13;
pub const CURLPX_NO_AUTH: CURLproxycode = 12;
pub const CURLPX_LONG_USER: CURLproxycode = 11;
pub const CURLPX_LONG_PASSWD: CURLproxycode = 10;
pub const CURLPX_LONG_HOSTNAME: CURLproxycode = 9;
pub const CURLPX_IDENTD_DIFFER: CURLproxycode = 8;
pub const CURLPX_IDENTD: CURLproxycode = 7;
pub const CURLPX_GSSAPI_PROTECTION: CURLproxycode = 6;
pub const CURLPX_GSSAPI_PERMSG: CURLproxycode = 5;
pub const CURLPX_GSSAPI: CURLproxycode = 4;
pub const CURLPX_CLOSED: CURLproxycode = 3;
pub const CURLPX_BAD_VERSION: CURLproxycode = 2;
pub const CURLPX_BAD_ADDRESS_TYPE: CURLproxycode = 1;
pub const CURLPX_OK: CURLproxycode = 0;
// #[derive(Copy, Clone)]

pub type curl_certinfo = crate::src::lib::http2::curl_certinfo;
// #[derive(Copy, Clone)]

pub type curl_slist = crate::src::lib::http2::curl_slist;
// #[derive(Copy, Clone)]

pub type WildcardData = crate::src::lib::http2::WildcardData;
pub type wildcard_dtor = crate::src::lib::http2::wildcard_dtor;
// #[derive(Copy, Clone)]

pub type Curl_llist = crate::src::lib::http2::Curl_llist;
pub type Curl_llist_dtor = crate::src::lib::http2::Curl_llist_dtor;
// #[derive(Copy, Clone)]

pub type Curl_llist_element = crate::src::lib::http2::Curl_llist_element;
pub type wildcard_states = crate::src::lib::http2::wildcard_states;
pub const CURLWC_DONE: wildcard_states = 7;
pub const CURLWC_ERROR: wildcard_states = 6;
pub const CURLWC_SKIP: wildcard_states = 5;
pub const CURLWC_CLEAN: wildcard_states = 4;
pub const CURLWC_DOWNLOADING: wildcard_states = 3;
pub const CURLWC_MATCHING: wildcard_states = 2;
pub const CURLWC_INIT: wildcard_states = 1;
pub const CURLWC_CLEAR: wildcard_states = 0;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type UrlState = crate::src::lib::http2::UrlState;
// #[derive(Copy, Clone)]

pub type dynamically_allocated_data = crate::src::lib::http2::dynamically_allocated_data;
pub type trailers_state = crate::src::lib::http2::trailers_state;
pub const TRAILERS_DONE: trailers_state = 3;
pub const TRAILERS_SENDING: trailers_state = 2;
pub const TRAILERS_INITIALIZED: trailers_state = 1;
pub const TRAILERS_NONE: trailers_state = 0;
// #[derive(Copy, Clone)]

pub type dynbuf = crate::src::lib::http2::dynbuf;
pub type Curl_HttpReq = crate::src::lib::http2::Curl_HttpReq;
pub const HTTPREQ_HEAD: Curl_HttpReq = 5;
pub const HTTPREQ_PUT: Curl_HttpReq = 4;
pub const HTTPREQ_POST_MIME: Curl_HttpReq = 3;
pub const HTTPREQ_POST_FORM: Curl_HttpReq = 2;
pub const HTTPREQ_POST: Curl_HttpReq = 1;
pub const HTTPREQ_GET: Curl_HttpReq = 0;
// #[derive(Copy, Clone)]

pub type urlpieces = crate::src::lib::http2::urlpieces;
pub type CURLU = crate::src::lib::http2::CURLU;
pub type curl_read_callback = crate::src::lib::http2::curl_read_callback;
// #[derive(Copy, Clone)]

pub type time_node = crate::src::lib::http2::time_node;
pub type expire_id = crate::src::lib::http2::expire_id;
pub const EXPIRE_LAST: expire_id = 13;
pub const EXPIRE_QUIC: expire_id = 12;
pub const EXPIRE_TOOFAST: expire_id = 11;
pub const EXPIRE_TIMEOUT: expire_id = 10;
pub const EXPIRE_SPEEDCHECK: expire_id = 9;
pub const EXPIRE_RUN_NOW: expire_id = 8;
pub const EXPIRE_MULTI_PENDING: expire_id = 7;
pub const EXPIRE_HAPPY_EYEBALLS: expire_id = 6;
pub const EXPIRE_HAPPY_EYEBALLS_DNS: expire_id = 5;
pub const EXPIRE_DNS_PER_NAME2: expire_id = 4;
pub const EXPIRE_DNS_PER_NAME: expire_id = 3;
pub const EXPIRE_CONNECTTIMEOUT: expire_id = 2;
pub const EXPIRE_ASYNC_NAME: expire_id = 1;
pub const EXPIRE_100_TIMEOUT: expire_id = 0;
// #[derive(Copy, Clone)]

pub type curltime = crate::src::lib::http2::curltime;
// #[derive(Copy, Clone)]

pub type Curl_tree = crate::src::lib::http2::Curl_tree;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type Curl_async = crate::src::lib::http2::Curl_async;
// #[derive(Copy, Clone)]

pub type Curl_dns_entry = crate::src::lib::http2::Curl_dns_entry;
// #[derive(Copy, Clone)]

pub type Curl_addrinfo = crate::src::lib::http2::Curl_addrinfo;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type auth = crate::src::lib::http2::auth;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type digestdata = crate::src::lib::http2::digestdata;
// #[derive(Copy, Clone)]

pub type tempbuf = crate::src::lib::http2::tempbuf;
// #[derive(Copy, Clone)]

pub type Curl_ssl_session = crate::src::lib::http2::Curl_ssl_session;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_primary_config = crate::src::lib::http2::ssl_primary_config;
// #[derive(Copy, Clone)]

pub type curl_blob = crate::src::lib::http2::curl_blob;
// #[derive(Copy, Clone)]

pub type conncache = crate::src::lib::http2::conncache;
// #[derive(Copy, Clone)]

pub type Curl_hash = crate::src::lib::http2::Curl_hash;
pub type Curl_hash_dtor = crate::src::lib::http2::Curl_hash_dtor;
pub type comp_function = crate::src::lib::http2::comp_function;
pub type hash_function = crate::src::lib::http2::hash_function;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type Progress = crate::src::lib::http2::Progress;
pub type timediff_t = crate::src::lib::http2::timediff_t;
// #[derive(Copy, Clone)]

pub type CookieInfo = crate::src::lib::http2::CookieInfo;
// #[derive(Copy, Clone)]

pub type Cookie = crate::src::lib::http2::Cookie;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type UserDefined = crate::src::lib::http2::UserDefined;
pub type curl_trailer_callback = crate::src::lib::http2::curl_trailer_callback;
pub type multidone_func = crate::src::lib::http2::multidone_func;
pub type CURLcode = crate::src::lib::http2::CURLcode;
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
pub type curl_resolver_start_callback = crate::src::lib::http2::curl_resolver_start_callback;
// #[derive(Copy, Clone)]

pub type Curl_http2_dep = crate::src::lib::http2::Curl_http2_dep;
pub type curl_fnmatch_callback = crate::src::lib::http2::curl_fnmatch_callback;
pub type curl_chunk_end_callback = crate::src::lib::http2::curl_chunk_end_callback;
pub type curl_chunk_bgn_callback = crate::src::lib::http2::curl_chunk_bgn_callback;
pub type Curl_RtspReq = crate::src::lib::http2::Curl_RtspReq;
pub const RTSPREQ_LAST: Curl_RtspReq = 12;
pub const RTSPREQ_RECEIVE: Curl_RtspReq = 11;
pub const RTSPREQ_RECORD: Curl_RtspReq = 10;
pub const RTSPREQ_SET_PARAMETER: Curl_RtspReq = 9;
pub const RTSPREQ_GET_PARAMETER: Curl_RtspReq = 8;
pub const RTSPREQ_TEARDOWN: Curl_RtspReq = 7;
pub const RTSPREQ_PAUSE: Curl_RtspReq = 6;
pub const RTSPREQ_PLAY: Curl_RtspReq = 5;
pub const RTSPREQ_SETUP: Curl_RtspReq = 4;
pub const RTSPREQ_ANNOUNCE: Curl_RtspReq = 3;
pub const RTSPREQ_DESCRIBE: Curl_RtspReq = 2;
pub const RTSPREQ_OPTIONS: Curl_RtspReq = 1;
pub const RTSPREQ_NONE: Curl_RtspReq = 0;
pub type curl_usessl = crate::src::lib::http2::curl_usessl;
pub const CURLUSESSL_LAST: curl_usessl = 4;
pub const CURLUSESSL_ALL: curl_usessl = 3;
pub const CURLUSESSL_CONTROL: curl_usessl = 2;
pub const CURLUSESSL_TRY: curl_usessl = 1;
pub const CURLUSESSL_NONE: curl_usessl = 0;
pub type CURL_NETRC_OPTION = crate::src::lib::http2::CURL_NETRC_OPTION;
pub const CURL_NETRC_LAST: CURL_NETRC_OPTION = 3;
pub const CURL_NETRC_REQUIRED: CURL_NETRC_OPTION = 2;
pub const CURL_NETRC_OPTIONAL: CURL_NETRC_OPTION = 1;
pub const CURL_NETRC_IGNORED: CURL_NETRC_OPTION = 0;
pub type curl_sshkeycallback = crate::src::lib::http2::curl_sshkeycallback;
pub type curl_khmatch = crate::src::lib::http2::curl_khmatch;
pub const CURLKHMATCH_LAST: curl_khmatch = 3;
pub const CURLKHMATCH_MISSING: curl_khmatch = 2;
pub const CURLKHMATCH_MISMATCH: curl_khmatch = 1;
pub const CURLKHMATCH_OK: curl_khmatch = 0;
// #[derive(Copy, Clone)]

pub type curl_khkey = crate::src::lib::http2::curl_khkey;
pub type curl_khtype = crate::src::lib::http2::curl_khtype;
pub const CURLKHTYPE_ED25519: curl_khtype = 5;
pub const CURLKHTYPE_ECDSA: curl_khtype = 4;
pub const CURLKHTYPE_DSS: curl_khtype = 3;
pub const CURLKHTYPE_RSA: curl_khtype = 2;
pub const CURLKHTYPE_RSA1: curl_khtype = 1;
pub const CURLKHTYPE_UNKNOWN: curl_khtype = 0;
pub type CURL = crate::src::lib::http2::CURL;
pub type curl_ftpccc = crate::src::lib::http2::curl_ftpccc;
pub const CURLFTPSSL_CCC_LAST: curl_ftpccc = 3;
pub const CURLFTPSSL_CCC_ACTIVE: curl_ftpccc = 2;
pub const CURLFTPSSL_CCC_PASSIVE: curl_ftpccc = 1;
pub const CURLFTPSSL_CCC_NONE: curl_ftpccc = 0;
pub type curl_ftpauth = crate::src::lib::http2::curl_ftpauth;
pub const CURLFTPAUTH_LAST: curl_ftpauth = 3;
pub const CURLFTPAUTH_TLS: curl_ftpauth = 2;
pub const CURLFTPAUTH_SSL: curl_ftpauth = 1;
pub const CURLFTPAUTH_DEFAULT: curl_ftpauth = 0;
pub type curl_ftpfile = crate::src::lib::http2::curl_ftpfile;
pub const FTPFILE_SINGLECWD: curl_ftpfile = 3;
pub const FTPFILE_NOCWD: curl_ftpfile = 2;
pub const FTPFILE_MULTICWD: curl_ftpfile = 1;
// #[derive(Copy, Clone)]

pub type ssl_general_config = crate::src::lib::http2::ssl_general_config;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_config_data = crate::src::lib::http2::ssl_config_data;
pub type CURL_TLSAUTH = crate::src::lib::http2::CURL_TLSAUTH;
pub const CURL_TLSAUTH_LAST: CURL_TLSAUTH = 2;
pub const CURL_TLSAUTH_SRP: CURL_TLSAUTH = 1;
pub const CURL_TLSAUTH_NONE: CURL_TLSAUTH = 0;
pub type curl_ssl_ctx_callback = crate::src::lib::http2::curl_ssl_ctx_callback;
pub type curl_proxytype = crate::src::lib::http2::curl_proxytype;
pub const CURLPROXY_SOCKS5_HOSTNAME: curl_proxytype = 7;
pub const CURLPROXY_SOCKS4A: curl_proxytype = 6;
pub const CURLPROXY_SOCKS5: curl_proxytype = 5;
pub const CURLPROXY_SOCKS4: curl_proxytype = 4;
pub const CURLPROXY_HTTPS: curl_proxytype = 2;
pub const CURLPROXY_HTTP_1_0: curl_proxytype = 1;
pub const CURLPROXY_HTTP: curl_proxytype = 0;
pub type curl_TimeCond = crate::src::lib::http2::curl_TimeCond;
pub const CURL_TIMECOND_LAST: curl_TimeCond = 4;
pub const CURL_TIMECOND_LASTMOD: curl_TimeCond = 3;
pub const CURL_TIMECOND_IFUNMODSINCE: curl_TimeCond = 2;
pub const CURL_TIMECOND_IFMODSINCE: curl_TimeCond = 1;
pub const CURL_TIMECOND_NONE: curl_TimeCond = 0;
// #[derive(Copy, Clone)]

pub type curl_mimepart = crate::src::lib::http2::curl_mimepart;
// #[derive(Copy, Clone)]

pub type mime_encoder_state = crate::src::lib::http2::mime_encoder_state;
// #[derive(Copy, Clone)]

pub type mime_encoder = crate::src::lib::http2::mime_encoder;
// #[derive(Copy, Clone)]

pub type mime_state = crate::src::lib::http2::mime_state;
pub type mimestate = crate::src::lib::http2::mimestate;
pub const MIMESTATE_LAST: mimestate = 9;
pub const MIMESTATE_END: mimestate = 8;
pub const MIMESTATE_CONTENT: mimestate = 7;
pub const MIMESTATE_BOUNDARY2: mimestate = 6;
pub const MIMESTATE_BOUNDARY1: mimestate = 5;
pub const MIMESTATE_BODY: mimestate = 4;
pub const MIMESTATE_EOH: mimestate = 3;
pub const MIMESTATE_USERHEADERS: mimestate = 2;
pub const MIMESTATE_CURLHEADERS: mimestate = 1;
pub const MIMESTATE_BEGIN: mimestate = 0;
pub type curl_free_callback = crate::src::lib::http2::curl_free_callback;
pub type curl_seek_callback = crate::src::lib::http2::curl_seek_callback;
pub type mimekind = crate::src::lib::http2::mimekind;
pub const MIMEKIND_LAST: mimekind = 5;
pub const MIMEKIND_MULTIPART: mimekind = 4;
pub const MIMEKIND_CALLBACK: mimekind = 3;
pub const MIMEKIND_FILE: mimekind = 2;
pub const MIMEKIND_DATA: mimekind = 1;
pub const MIMEKIND_NONE: mimekind = 0;
// #[derive(Copy, Clone)]

pub type curl_mime = crate::src::lib::http2::curl_mime;
// #[derive(Copy, Clone)]

pub type curl_httppost = crate::src::lib::http2::curl_httppost;
pub type curl_hstswrite_callback = crate::src::lib::http2::curl_hstswrite_callback;
// #[derive(Copy, Clone)]

pub type curl_index = crate::src::lib::http2::curl_index;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type curl_hstsentry = crate::src::lib::http2::curl_hstsentry;
pub type CURLSTScode = crate::src::lib::http2::CURLSTScode;
pub const CURLSTS_FAIL: CURLSTScode = 2;
pub const CURLSTS_DONE: CURLSTScode = 1;
pub const CURLSTS_OK: CURLSTScode = 0;
pub type curl_hstsread_callback = crate::src::lib::http2::curl_hstsread_callback;
pub type curl_conv_callback = crate::src::lib::http2::curl_conv_callback;
pub type curl_closesocket_callback = crate::src::lib::http2::curl_closesocket_callback;
pub type curl_socket_t = crate::src::lib::http2::curl_socket_t;
pub type curl_opensocket_callback = crate::src::lib::http2::curl_opensocket_callback;
// #[derive(Copy, Clone)]

pub type curl_sockaddr = crate::src::lib::http2::curl_sockaddr;
pub type curlsocktype = crate::src::lib::http2::curlsocktype;
pub const CURLSOCKTYPE_LAST: curlsocktype = 2;
pub const CURLSOCKTYPE_ACCEPT: curlsocktype = 1;
pub const CURLSOCKTYPE_IPCXN: curlsocktype = 0;
pub type curl_sockopt_callback = crate::src::lib::http2::curl_sockopt_callback;
pub type curl_ioctl_callback = crate::src::lib::http2::curl_ioctl_callback;
pub type curlioerr = crate::src::lib::http2::curlioerr;
pub const CURLIOE_LAST: curlioerr = 3;
pub const CURLIOE_FAILRESTART: curlioerr = 2;
pub const CURLIOE_UNKNOWNCMD: curlioerr = 1;
pub const CURLIOE_OK: curlioerr = 0;
pub type curl_debug_callback = crate::src::lib::http2::curl_debug_callback;
pub type curl_infotype = crate::src::lib::http2::curl_infotype;
pub const CURLINFO_END: curl_infotype = 7;
pub const CURLINFO_SSL_DATA_OUT: curl_infotype = 6;
pub const CURLINFO_SSL_DATA_IN: curl_infotype = 5;
pub const CURLINFO_DATA_OUT: curl_infotype = 4;
pub const CURLINFO_DATA_IN: curl_infotype = 3;
pub const CURLINFO_HEADER_OUT: curl_infotype = 2;
pub const CURLINFO_HEADER_IN: curl_infotype = 1;
pub const CURLINFO_TEXT: curl_infotype = 0;
pub type curl_xferinfo_callback = crate::src::lib::http2::curl_xferinfo_callback;
pub type curl_progress_callback = crate::src::lib::http2::curl_progress_callback;
pub type curl_write_callback = crate::src::lib::http2::curl_write_callback;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type SingleRequest = crate::src::lib::http2::SingleRequest;
// #[derive(Copy, Clone)]

pub type dohdata = crate::src::lib::http2::dohdata;
// #[derive(Copy, Clone)]

pub type dnsprobe = crate::src::lib::http2::dnsprobe;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed = crate::src::lib::http2::C2RustUnnamed;
// #[derive(Copy, Clone)]

pub type SSHPROTO = crate::src::lib::http2::SSHPROTO;
// #[derive(Copy, Clone)]

pub type SMTP = crate::src::lib::http2::SMTP;
pub type curl_pp_transfer = crate::src::lib::http2::curl_pp_transfer;
pub const PPTRANSFER_NONE: curl_pp_transfer = 2;
pub const PPTRANSFER_INFO: curl_pp_transfer = 1;
pub const PPTRANSFER_BODY: curl_pp_transfer = 0;
// #[derive(Copy, Clone)]

pub type RTSP = crate::src::lib::http2::RTSP;
// #[derive(Copy, Clone)]

pub type HTTP = crate::src::lib::http2::HTTP;
pub type uint8_t = crate::src::lib::http2::uint8_t;
pub type uint32_t = crate::src::lib::http2::uint32_t;
pub type C2RustUnnamed_0 = u32;
pub const HTTPSEND_BODY: C2RustUnnamed_0 = 2;
pub const HTTPSEND_REQUEST: C2RustUnnamed_0 = 1;
pub const HTTPSEND_NADA: C2RustUnnamed_0 = 0;
// #[derive(Copy, Clone)]

pub type back = crate::src::lib::http2::back;
// #[derive(Copy, Clone)]

pub type POP3 = crate::src::lib::http2::POP3;
// #[derive(Copy, Clone)]

pub type MQTT = crate::src::lib::http2::MQTT;
// #[derive(Copy, Clone)]

pub type IMAP = crate::src::lib::http2::IMAP;
// #[derive(Copy, Clone)]

pub type FTP = crate::src::lib::http2::FTP;
// #[derive(Copy, Clone)]

pub type FILEPROTO = crate::src::lib::http2::FILEPROTO;
pub type upgrade101 = crate::src::lib::http2::upgrade101;
pub const UPGR101_WORKING: upgrade101 = 3;
pub const UPGR101_RECEIVED: upgrade101 = 2;
pub const UPGR101_REQUESTED: upgrade101 = 1;
pub const UPGR101_INIT: upgrade101 = 0;
pub type expect100 = crate::src::lib::http2::expect100;
pub const EXP100_FAILED: expect100 = 3;
pub const EXP100_SENDING_REQUEST: expect100 = 2;
pub const EXP100_AWAITING_CONTINUE: expect100 = 1;
pub const EXP100_SEND_DATA: expect100 = 0;
pub type C2RustUnnamed_1 = u32;
pub const HEADER_ALLBAD: C2RustUnnamed_1 = 2;
pub const HEADER_PARTHEADER: C2RustUnnamed_1 = 1;
pub const HEADER_NORMAL: C2RustUnnamed_1 = 0;
// #[derive(Copy, Clone)]

pub type PslCache = crate::src::lib::http2::PslCache;
pub type psl_ctx_t = crate::src::lib::http2::psl_ctx_t;
// #[derive(Copy, Clone)]

pub type Curl_multi = crate::src::lib::http2::Curl_multi;
pub type curl_multi_timer_callback = crate::src::lib::http2::curl_multi_timer_callback;
pub type CURLM = crate::src::lib::http2::CURLM;
pub type curl_push_callback = crate::src::lib::http2::curl_push_callback;
pub type curl_socket_callback = crate::src::lib::http2::curl_socket_callback;
// #[derive(Copy, Clone)]

pub type Names = crate::src::lib::http2::Names;
pub type C2RustUnnamed_2 = u32;
pub const HCACHE_SHARED: C2RustUnnamed_2 = 2;
pub const HCACHE_MULTI: C2RustUnnamed_2 = 1;
pub const HCACHE_NONE: C2RustUnnamed_2 = 0;
// #[derive(Copy, Clone)]

pub type Curl_message = crate::src::lib::http2::Curl_message;
// #[derive(Copy, Clone)]

pub type CURLMsg = crate::src::lib::http2::CURLMsg;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed_3 = crate::src::lib::http2::C2RustUnnamed_3;
pub type CURLMSG = crate::src::lib::http2::CURLMSG;
pub const CURLMSG_LAST: CURLMSG = 2;
pub const CURLMSG_DONE: CURLMSG = 1;
pub const CURLMSG_NONE: CURLMSG = 0;
pub type CURLMstate = crate::src::lib::http2::CURLMstate;
pub const MSTATE_LAST: CURLMstate = 17;
pub const MSTATE_MSGSENT: CURLMstate = 16;
pub const MSTATE_COMPLETED: CURLMstate = 15;
pub const MSTATE_DONE: CURLMstate = 14;
pub const MSTATE_RATELIMITING: CURLMstate = 13;
pub const MSTATE_PERFORMING: CURLMstate = 12;
pub const MSTATE_DID: CURLMstate = 11;
pub const MSTATE_DOING_MORE: CURLMstate = 10;
pub const MSTATE_DOING: CURLMstate = 9;
pub const MSTATE_DO: CURLMstate = 8;
pub const MSTATE_PROTOCONNECTING: CURLMstate = 7;
pub const MSTATE_PROTOCONNECT: CURLMstate = 6;
pub const MSTATE_TUNNELING: CURLMstate = 5;
pub const MSTATE_CONNECTING: CURLMstate = 4;
pub const MSTATE_RESOLVING: CURLMstate = 3;
pub const MSTATE_CONNECT: CURLMstate = 2;
pub const MSTATE_PENDING: CURLMstate = 1;
pub const MSTATE_INIT: CURLMstate = 0;
// #[derive(Copy, Clone)]

pub type connectdata = crate::src::lib::http2::connectdata;
// #[derive(Copy, Clone)]

pub type connectbundle = crate::src::lib::http2::connectbundle;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type http_connect_state = crate::src::lib::ftp::http_connect_state;
pub type C2RustUnnamed_4 = u32;
pub const TUNNEL_EXIT: C2RustUnnamed_4 = 3;
pub const TUNNEL_COMPLETE: C2RustUnnamed_4 = 2;
pub const TUNNEL_CONNECT: C2RustUnnamed_4 = 1;
pub const TUNNEL_INIT: C2RustUnnamed_4 = 0;
pub type keeponval = crate::src::lib::ftp::keeponval;
pub const KEEPON_IGNORE: keeponval = 2;
pub const KEEPON_CONNECT: keeponval = 1;
pub const KEEPON_DONE: keeponval = 0;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed_5 = crate::src::lib::http2::C2RustUnnamed_4;
// #[derive(Copy, Clone)]

pub type mqtt_conn = crate::src::lib::http2::mqtt_conn;
pub type mqttstate = crate::src::lib::http2::mqttstate;
pub const MQTT_NOSTATE: mqttstate = 7;
pub const MQTT_PUB_REMAIN: mqttstate = 6;
pub const MQTT_PUBWAIT: mqttstate = 5;
pub const MQTT_SUBACK_COMING: mqttstate = 4;
pub const MQTT_SUBACK: mqttstate = 3;
pub const MQTT_CONNACK: mqttstate = 2;
pub const MQTT_REMAINING_LENGTH: mqttstate = 1;
pub const MQTT_FIRST: mqttstate = 0;
// #[derive(Copy, Clone)]

pub type smb_conn = crate::src::lib::http2::smb_conn;
pub type smb_conn_state = crate::src::lib::http2::smb_conn_state;
pub const SMB_CONNECTED: smb_conn_state = 4;
pub const SMB_SETUP: smb_conn_state = 3;
pub const SMB_NEGOTIATE: smb_conn_state = 2;
pub const SMB_CONNECTING: smb_conn_state = 1;
pub const SMB_NOT_CONNECTED: smb_conn_state = 0;
// #[derive(Copy, Clone)]

pub type rtsp_conn = crate::src::lib::http2::rtsp_conn;
// #[derive(Copy, Clone)]

pub type smtp_conn = crate::src::lib::http2::smtp_conn;
// #[derive(Copy, Clone)]

pub type SASL = crate::src::lib::http2::SASL;
pub type saslstate = crate::src::lib::http2::saslstate;
pub const SASL_FINAL: saslstate = 17;
pub const SASL_CANCEL: saslstate = 16;
pub const SASL_GSASL: saslstate = 15;
pub const SASL_OAUTH2_RESP: saslstate = 14;
pub const SASL_OAUTH2: saslstate = 13;
pub const SASL_GSSAPI_NO_DATA: saslstate = 12;
pub const SASL_GSSAPI_TOKEN: saslstate = 11;
pub const SASL_GSSAPI: saslstate = 10;
pub const SASL_NTLM_TYPE2MSG: saslstate = 9;
pub const SASL_NTLM: saslstate = 8;
pub const SASL_DIGESTMD5_RESP: saslstate = 7;
pub const SASL_DIGESTMD5: saslstate = 6;
pub const SASL_CRAMMD5: saslstate = 5;
pub const SASL_EXTERNAL: saslstate = 4;
pub const SASL_LOGIN_PASSWD: saslstate = 3;
pub const SASL_LOGIN: saslstate = 2;
pub const SASL_PLAIN: saslstate = 1;
pub const SASL_STOP: saslstate = 0;
// #[derive(Copy, Clone)]

pub type SASLproto = crate::src::lib::http2::SASLproto;
pub type smtpstate = crate::src::lib::http2::smtpstate;
pub const SMTP_LAST: smtpstate = 13;
pub const SMTP_QUIT: smtpstate = 12;
pub const SMTP_POSTDATA: smtpstate = 11;
pub const SMTP_DATA: smtpstate = 10;
pub const SMTP_RCPT: smtpstate = 9;
pub const SMTP_MAIL: smtpstate = 8;
pub const SMTP_COMMAND: smtpstate = 7;
pub const SMTP_AUTH: smtpstate = 6;
pub const SMTP_UPGRADETLS: smtpstate = 5;
pub const SMTP_STARTTLS: smtpstate = 4;
pub const SMTP_HELO: smtpstate = 3;
pub const SMTP_EHLO: smtpstate = 2;
pub const SMTP_SERVERGREET: smtpstate = 1;
pub const SMTP_STOP: smtpstate = 0;
// #[derive(Copy, Clone)]

pub type pingpong = crate::src::lib::http2::pingpong;
// #[derive(Copy, Clone)]

pub type pop3_conn = crate::src::lib::http2::pop3_conn;
pub type pop3state = crate::src::lib::http2::pop3state;
pub const POP3_LAST: pop3state = 11;
pub const POP3_QUIT: pop3state = 10;
pub const POP3_COMMAND: pop3state = 9;
pub const POP3_PASS: pop3state = 8;
pub const POP3_USER: pop3state = 7;
pub const POP3_APOP: pop3state = 6;
pub const POP3_AUTH: pop3state = 5;
pub const POP3_UPGRADETLS: pop3state = 4;
pub const POP3_STARTTLS: pop3state = 3;
pub const POP3_CAPA: pop3state = 2;
pub const POP3_SERVERGREET: pop3state = 1;
pub const POP3_STOP: pop3state = 0;
// #[derive(Copy, Clone)]

pub type imap_conn = crate::src::lib::http2::imap_conn;
pub type imapstate = crate::src::lib::http2::imapstate;
pub const IMAP_LAST: imapstate = 15;
pub const IMAP_LOGOUT: imapstate = 14;
pub const IMAP_SEARCH: imapstate = 13;
pub const IMAP_APPEND_FINAL: imapstate = 12;
pub const IMAP_APPEND: imapstate = 11;
pub const IMAP_FETCH_FINAL: imapstate = 10;
pub const IMAP_FETCH: imapstate = 9;
pub const IMAP_SELECT: imapstate = 8;
pub const IMAP_LIST: imapstate = 7;
pub const IMAP_LOGIN: imapstate = 6;
pub const IMAP_AUTHENTICATE: imapstate = 5;
pub const IMAP_UPGRADETLS: imapstate = 4;
pub const IMAP_STARTTLS: imapstate = 3;
pub const IMAP_CAPABILITY: imapstate = 2;
pub const IMAP_SERVERGREET: imapstate = 1;
pub const IMAP_STOP: imapstate = 0;
// #[derive(Copy, Clone)]

pub type ssh_conn = crate::src::lib::http2::ssh_conn;
pub type sshstate = crate::src::lib::http2::sshstate;
pub const SSH_LAST: sshstate = 60;
pub const SSH_QUIT: sshstate = 59;
pub const SSH_SESSION_FREE: sshstate = 58;
pub const SSH_SESSION_DISCONNECT: sshstate = 57;
pub const SSH_SCP_CHANNEL_FREE: sshstate = 56;
pub const SSH_SCP_WAIT_CLOSE: sshstate = 55;
pub const SSH_SCP_WAIT_EOF: sshstate = 54;
pub const SSH_SCP_SEND_EOF: sshstate = 53;
pub const SSH_SCP_DONE: sshstate = 52;
pub const SSH_SCP_DOWNLOAD: sshstate = 51;
pub const SSH_SCP_DOWNLOAD_INIT: sshstate = 50;
pub const SSH_SCP_UPLOAD_INIT: sshstate = 49;
pub const SSH_SCP_TRANS_INIT: sshstate = 48;
pub const SSH_SFTP_SHUTDOWN: sshstate = 47;
pub const SSH_SFTP_CLOSE: sshstate = 46;
pub const SSH_SFTP_DOWNLOAD_STAT: sshstate = 45;
pub const SSH_SFTP_DOWNLOAD_INIT: sshstate = 44;
pub const SSH_SFTP_READDIR_DONE: sshstate = 43;
pub const SSH_SFTP_READDIR_BOTTOM: sshstate = 42;
pub const SSH_SFTP_READDIR_LINK: sshstate = 41;
pub const SSH_SFTP_READDIR: sshstate = 40;
pub const SSH_SFTP_READDIR_INIT: sshstate = 39;
pub const SSH_SFTP_CREATE_DIRS_MKDIR: sshstate = 38;
pub const SSH_SFTP_CREATE_DIRS: sshstate = 37;
pub const SSH_SFTP_CREATE_DIRS_INIT: sshstate = 36;
pub const SSH_SFTP_UPLOAD_INIT: sshstate = 35;
pub const SSH_SFTP_TRANS_INIT: sshstate = 34;
pub const SSH_SFTP_FILETIME: sshstate = 33;
pub const SSH_SFTP_GETINFO: sshstate = 32;
pub const SSH_SFTP_QUOTE_STATVFS: sshstate = 31;
pub const SSH_SFTP_QUOTE_UNLINK: sshstate = 30;
pub const SSH_SFTP_QUOTE_RMDIR: sshstate = 29;
pub const SSH_SFTP_QUOTE_RENAME: sshstate = 28;
pub const SSH_SFTP_QUOTE_MKDIR: sshstate = 27;
pub const SSH_SFTP_QUOTE_SYMLINK: sshstate = 26;
pub const SSH_SFTP_QUOTE_SETSTAT: sshstate = 25;
pub const SSH_SFTP_QUOTE_STAT: sshstate = 24;
pub const SSH_SFTP_NEXT_QUOTE: sshstate = 23;
pub const SSH_SFTP_QUOTE: sshstate = 22;
pub const SSH_SFTP_POSTQUOTE_INIT: sshstate = 21;
pub const SSH_SFTP_QUOTE_INIT: sshstate = 20;
pub const SSH_SFTP_REALPATH: sshstate = 19;
pub const SSH_SFTP_INIT: sshstate = 18;
pub const SSH_AUTH_DONE: sshstate = 17;
pub const SSH_AUTH_GSSAPI: sshstate = 16;
pub const SSH_AUTH_KEY: sshstate = 15;
pub const SSH_AUTH_KEY_INIT: sshstate = 14;
pub const SSH_AUTH_HOST: sshstate = 13;
pub const SSH_AUTH_HOST_INIT: sshstate = 12;
pub const SSH_AUTH_AGENT: sshstate = 11;
pub const SSH_AUTH_AGENT_LIST: sshstate = 10;
pub const SSH_AUTH_AGENT_INIT: sshstate = 9;
pub const SSH_AUTH_PASS: sshstate = 8;
pub const SSH_AUTH_PASS_INIT: sshstate = 7;
pub const SSH_AUTH_PKEY: sshstate = 6;
pub const SSH_AUTH_PKEY_INIT: sshstate = 5;
pub const SSH_AUTHLIST: sshstate = 4;
pub const SSH_HOSTKEY: sshstate = 3;
pub const SSH_S_STARTUP: sshstate = 2;
pub const SSH_INIT: sshstate = 1;
pub const SSH_STOP: sshstate = 0;
pub const SSH_NO_STATE: sshstate = -1;
// #[derive(Copy, Clone)]

pub type http_conn = crate::src::lib::http2::http_conn;
// #[derive(Copy, Clone)]

pub type nghttp2_settings_entry = crate::src::lib::http2::nghttp2_settings_entry;
// #[derive(Copy, Clone)]

pub type h2settings = crate::src::lib::http2::h2settings;
pub type Curl_recv = crate::src::lib::http2::Curl_recv;
pub type Curl_send = crate::src::lib::http2::Curl_send;
// #[derive(Copy, Clone)]

pub type ftp_conn = crate::src::lib::http2::ftp_conn;
pub type ftpstate = crate::src::lib::http2::ftpstate;
pub const FTP_LAST: ftpstate = 35;
pub const FTP_QUIT: ftpstate = 34;
pub const FTP_STOR: ftpstate = 33;
pub const FTP_RETR: ftpstate = 32;
pub const FTP_LIST: ftpstate = 31;
pub const FTP_PASV: ftpstate = 30;
pub const FTP_PRET: ftpstate = 29;
pub const FTP_PORT: ftpstate = 28;
pub const FTP_RETR_REST: ftpstate = 27;
pub const FTP_REST: ftpstate = 26;
pub const FTP_STOR_SIZE: ftpstate = 25;
pub const FTP_RETR_SIZE: ftpstate = 24;
pub const FTP_SIZE: ftpstate = 23;
pub const FTP_STOR_TYPE: ftpstate = 22;
pub const FTP_RETR_TYPE: ftpstate = 21;
pub const FTP_LIST_TYPE: ftpstate = 20;
pub const FTP_TYPE: ftpstate = 19;
pub const FTP_MDTM: ftpstate = 18;
pub const FTP_MKD: ftpstate = 17;
pub const FTP_CWD: ftpstate = 16;
pub const FTP_POSTQUOTE: ftpstate = 15;
pub const FTP_STOR_PREQUOTE: ftpstate = 14;
pub const FTP_RETR_PREQUOTE: ftpstate = 13;
pub const FTP_QUOTE: ftpstate = 12;
pub const FTP_NAMEFMT: ftpstate = 11;
pub const FTP_SYST: ftpstate = 10;
pub const FTP_PWD: ftpstate = 9;
pub const FTP_CCC: ftpstate = 8;
pub const FTP_PROT: ftpstate = 7;
pub const FTP_PBSZ: ftpstate = 6;
pub const FTP_ACCT: ftpstate = 5;
pub const FTP_PASS: ftpstate = 4;
pub const FTP_USER: ftpstate = 3;
pub const FTP_AUTH: ftpstate = 2;
pub const FTP_WAIT220: ftpstate = 1;
pub const FTP_STOP: ftpstate = 0;
// #[derive(Copy, Clone)]

pub type ntlmdata = crate::src::lib::http2::ntlmdata;
pub type curlntlm = crate::src::lib::http2::curlntlm;
pub const NTLMSTATE_LAST: curlntlm = 4;
pub const NTLMSTATE_TYPE3: curlntlm = 3;
pub const NTLMSTATE_TYPE2: curlntlm = 2;
pub const NTLMSTATE_TYPE1: curlntlm = 1;
pub const NTLMSTATE_NONE: curlntlm = 0;
// #[derive(Copy, Clone)]

pub type gsasldata = crate::src::lib::http2::gsasldata;
// #[derive(Copy, Clone)]

pub type Curl_handler = crate::src::lib::http2::Curl_handler;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ConnectBits = crate::src::lib::http2::ConnectBits;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_connect_data = crate::src::lib::http2::ssl_connect_data;
pub type ssl_connect_state = crate::src::lib::http2::ssl_connect_state;
pub const ssl_connect_done: ssl_connect_state = 5;
pub const ssl_connect_3: ssl_connect_state = 4;
pub const ssl_connect_2_writing: ssl_connect_state = 3;
pub const ssl_connect_2_reading: ssl_connect_state = 2;
pub const ssl_connect_2: ssl_connect_state = 1;
pub const ssl_connect_1: ssl_connect_state = 0;
pub type ssl_connection_state = crate::src::lib::http2::ssl_connection_state;
pub const ssl_connection_complete: ssl_connection_state = 2;
pub const ssl_connection_negotiating: ssl_connection_state = 1;
pub const ssl_connection_none: ssl_connection_state = 0;
// #[derive(Copy, Clone)]

pub type proxy_info = crate::src::lib::http2::proxy_info;
// #[derive(Copy, Clone)]

pub type hostname = crate::src::lib::http2::hostname;
pub type C2RustUnnamed_6 = u32;
pub const TRNSPRT_QUIC: C2RustUnnamed_6 = 5;
pub const TRNSPRT_UDP: C2RustUnnamed_6 = 4;
pub const TRNSPRT_TCP: C2RustUnnamed_6 = 3;
// #[derive(Copy, Clone)]

pub type Curl_chunker = crate::src::lib::http2::Curl_chunker;
pub type ChunkyState = crate::src::lib::http2::ChunkyState;
pub const CHUNK_TRAILER_POSTCR: ChunkyState = 7;
pub const CHUNK_TRAILER_CR: ChunkyState = 6;
pub const CHUNK_TRAILER: ChunkyState = 5;
pub const CHUNK_STOP: ChunkyState = 4;
pub const CHUNK_POSTLF: ChunkyState = 3;
pub const CHUNK_DATA: ChunkyState = 2;
pub const CHUNK_LF: ChunkyState = 1;
pub const CHUNK_HEX: ChunkyState = 0;
// #[derive(Copy, Clone)]

pub type connstate = crate::src::lib::http2::connstate;
pub type connect_t = crate::src::lib::http2::connect_t;
pub const CONNECT_DONE: connect_t = 17;
pub const CONNECT_REQ_READ_MORE: connect_t = 16;
pub const CONNECT_REQ_READ: connect_t = 15;
pub const CONNECT_REQ_SENDING: connect_t = 14;
pub const CONNECT_REQ_SEND: connect_t = 13;
pub const CONNECT_RESOLVE_REMOTE: connect_t = 12;
pub const CONNECT_RESOLVED: connect_t = 11;
pub const CONNECT_RESOLVING: connect_t = 10;
pub const CONNECT_REQ_INIT: connect_t = 9;
pub const CONNECT_AUTH_READ: connect_t = 8;
pub const CONNECT_AUTH_SEND: connect_t = 7;
pub const CONNECT_AUTH_INIT: connect_t = 6;
pub const CONNECT_GSSAPI_INIT: connect_t = 5;
pub const CONNECT_SOCKS_READ: connect_t = 4;
pub const CONNECT_SOCKS_READ_INIT: connect_t = 3;
pub const CONNECT_SOCKS_SEND: connect_t = 2;
pub const CONNECT_SOCKS_INIT: connect_t = 1;
pub const CONNECT_INIT: connect_t = 0;
pub type curl_calloc_callback = crate::src::lib::http2::curl_calloc_callback;
pub type CHUNKcode = crate::src::lib::http_chunks::CHUNKcode;
pub const CHUNKE_LAST: CHUNKcode = 7;
pub const CHUNKE_PASSTHRU_ERROR: CHUNKcode = 6;
pub const CHUNKE_OUT_OF_MEMORY: CHUNKcode = 5;
pub const CHUNKE_BAD_ENCODING: CHUNKcode = 4;
pub const CHUNKE_BAD_CHUNK: CHUNKcode = 3;
pub const CHUNKE_ILLEGAL_HEX: CHUNKcode = 2;
pub const CHUNKE_TOO_LONG_HEX: CHUNKcode = 1;
pub const CHUNKE_OK: CHUNKcode = 0;
pub const CHUNKE_STOP: CHUNKcode = -1;
pub type dupstring = crate::src::lib::connect::dupstring;
pub const STRING_LAST: dupstring = 80;
pub const STRING_AWS_SIGV4: dupstring = 79;
pub const STRING_COPYPOSTFIELDS: dupstring = 78;
pub const STRING_LASTZEROTERMINATED: dupstring = 77;
pub const STRING_SSL_EC_CURVES: dupstring = 76;
pub const STRING_DNS_LOCAL_IP6: dupstring = 75;
pub const STRING_DNS_LOCAL_IP4: dupstring = 74;
pub const STRING_DNS_INTERFACE: dupstring = 73;
pub const STRING_DNS_SERVERS: dupstring = 72;
pub const STRING_SASL_AUTHZID: dupstring = 71;
pub const STRING_HSTS: dupstring = 70;
pub const STRING_ALTSVC: dupstring = 69;
pub const STRING_DOH: dupstring = 68;
pub const STRING_TARGET: dupstring = 67;
pub const STRING_UNIX_SOCKET_PATH: dupstring = 66;
pub const STRING_BEARER: dupstring = 65;
pub const STRING_TLSAUTH_PASSWORD_PROXY: dupstring = 64;
pub const STRING_TLSAUTH_PASSWORD: dupstring = 63;
pub const STRING_TLSAUTH_USERNAME_PROXY: dupstring = 62;
pub const STRING_TLSAUTH_USERNAME: dupstring = 61;
pub const STRING_MAIL_AUTH: dupstring = 60;
pub const STRING_MAIL_FROM: dupstring = 59;
pub const STRING_SERVICE_NAME: dupstring = 58;
pub const STRING_PROXY_SERVICE_NAME: dupstring = 57;
pub const STRING_SSH_KNOWNHOSTS: dupstring = 56;
pub const STRING_SSH_HOST_PUBLIC_KEY_MD5: dupstring = 55;
pub const STRING_SSH_PUBLIC_KEY: dupstring = 54;
pub const STRING_SSH_PRIVATE_KEY: dupstring = 53;
pub const STRING_RTSP_TRANSPORT: dupstring = 52;
pub const STRING_RTSP_STREAM_URI: dupstring = 51;
pub const STRING_RTSP_SESSION_ID: dupstring = 50;
pub const STRING_NOPROXY: dupstring = 49;
pub const STRING_PROXYPASSWORD: dupstring = 48;
pub const STRING_PROXYUSERNAME: dupstring = 47;
pub const STRING_OPTIONS: dupstring = 46;
pub const STRING_PASSWORD: dupstring = 45;
pub const STRING_USERNAME: dupstring = 44;
pub const STRING_SSL_ENGINE: dupstring = 43;
pub const STRING_SSL_ISSUERCERT_PROXY: dupstring = 42;
pub const STRING_SSL_ISSUERCERT: dupstring = 41;
pub const STRING_SSL_CRLFILE_PROXY: dupstring = 40;
pub const STRING_SSL_CRLFILE: dupstring = 39;
pub const STRING_USERAGENT: dupstring = 38;
pub const STRING_SSL_RANDOM_FILE: dupstring = 37;
pub const STRING_SSL_EGDSOCKET: dupstring = 36;
pub const STRING_SSL_CIPHER13_LIST_PROXY: dupstring = 35;
pub const STRING_SSL_CIPHER13_LIST: dupstring = 34;
pub const STRING_SSL_CIPHER_LIST_PROXY: dupstring = 33;
pub const STRING_SSL_CIPHER_LIST: dupstring = 32;
pub const STRING_SSL_PINNEDPUBLICKEY_PROXY: dupstring = 31;
pub const STRING_SSL_PINNEDPUBLICKEY: dupstring = 30;
pub const STRING_SSL_CAFILE_PROXY: dupstring = 29;
pub const STRING_SSL_CAFILE: dupstring = 28;
pub const STRING_SSL_CAPATH_PROXY: dupstring = 27;
pub const STRING_SSL_CAPATH: dupstring = 26;
pub const STRING_SET_URL: dupstring = 25;
pub const STRING_SET_REFERER: dupstring = 24;
pub const STRING_SET_RANGE: dupstring = 23;
pub const STRING_PRE_PROXY: dupstring = 22;
pub const STRING_PROXY: dupstring = 21;
pub const STRING_NETRC_FILE: dupstring = 20;
pub const STRING_KRB_LEVEL: dupstring = 19;
pub const STRING_KEY_TYPE_PROXY: dupstring = 18;
pub const STRING_KEY_TYPE: dupstring = 17;
pub const STRING_KEY_PASSWD_PROXY: dupstring = 16;
pub const STRING_KEY_PASSWD: dupstring = 15;
pub const STRING_KEY_PROXY: dupstring = 14;
pub const STRING_KEY: dupstring = 13;
pub const STRING_FTPPORT: dupstring = 12;
pub const STRING_FTP_ALTERNATIVE_TO_USER: dupstring = 11;
pub const STRING_FTP_ACCOUNT: dupstring = 10;
pub const STRING_ENCODING: dupstring = 9;
pub const STRING_DEVICE: dupstring = 8;
pub const STRING_DEFAULT_PROTOCOL: dupstring = 7;
pub const STRING_CUSTOMREQUEST: dupstring = 6;
pub const STRING_COOKIEJAR: dupstring = 5;
pub const STRING_COOKIE: dupstring = 4;
pub const STRING_CERT_TYPE_PROXY: dupstring = 3;
pub const STRING_CERT_TYPE: dupstring = 2;
pub const STRING_CERT_PROXY: dupstring = 1;
pub const STRING_CERT: dupstring = 0;
pub type CURLofft = crate::src::lib::cookie::CURLofft;
pub const CURL_OFFT_INVAL: CURLofft = 2;
pub const CURL_OFFT_FLOW: CURLofft = 1;
pub const CURL_OFFT_OK: CURLofft = 0;
unsafe extern "C" fn https_proxy_connect(
    mut data: *mut Curl_easy,
    mut sockindex: i32,
) -> CURLcode {
    let mut conn: *mut connectdata = (*data).conn;
    let mut result: CURLcode = CURLE_OK;
    if !(*conn).bits.proxy_ssl_connected[sockindex as usize] {
        result = Curl_ssl_connect_nonblocking(
            data,
            conn,
            1 as i32 != 0,
            sockindex,
            &mut *((*conn).bits.proxy_ssl_connected)
                .as_mut_ptr()
                .offset(sockindex as isize),
        );
        if result as u64 != 0 {
            Curl_conncontrol(conn, 1 as i32);
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_proxy_connect(
    mut data: *mut Curl_easy,
    mut sockindex: i32,
) -> CURLcode {
    let mut conn: *mut connectdata = (*data).conn;
    if (*conn).http_proxy.proxytype as u32
        == CURLPROXY_HTTPS as i32 as u32
    {
        let result: CURLcode = https_proxy_connect(data, sockindex);
        if result as u64 != 0 {
            return result;
        }
        if !(*conn).bits.proxy_ssl_connected[sockindex as usize] {
            return result;
        }
    }
    if ((*conn).bits).tunnel_proxy() as i32 != 0
        && ((*conn).bits).httpproxy() as i32 != 0
    {
        let mut hostname: *const i8 = 0 as *const i8;
        let mut remote_port: i32 = 0;
        let mut result_0: CURLcode = CURLE_OK;
        if ((*conn).bits).conn_to_host() != 0 {
            hostname = (*conn).conn_to_host.name;
        } else if sockindex == 1 as i32 {
            hostname = (*conn).secondaryhostname;
        } else {
            hostname = (*conn).host.name;
        }
        if sockindex == 1 as i32 {
            remote_port = (*conn).secondary_port as i32;
        } else if ((*conn).bits).conn_to_port() != 0 {
            remote_port = (*conn).conn_to_port;
        } else {
            remote_port = (*conn).remote_port;
        }
        result_0 = Curl_proxyCONNECT(data, sockindex, hostname, remote_port);
        if CURLE_OK as i32 as u32 != result_0 as u32 {
            return result_0;
        }
        Curl_cfree
            .expect(
                "non-null function pointer",
            )((*data).state.aptr.proxyuserpwd as *mut libc::c_void);
        let ref mut fresh0 = (*data).state.aptr.proxyuserpwd;
        *fresh0 = 0 as *mut i8;
    }
    return CURLE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_connect_complete(mut conn: *mut connectdata) -> bool {
    return ((*conn).connect_state).is_null()
        || (*(*conn).connect_state).tunnel_state as u32
            >= TUNNEL_COMPLETE as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_connect_ongoing(mut conn: *mut connectdata) -> bool {
    return !((*conn).connect_state).is_null()
        && (*(*conn).connect_state).tunnel_state as u32
            <= TUNNEL_COMPLETE as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_connect_getsock(
    mut conn: *mut connectdata,
) -> i32 {
    let mut http: *mut HTTP = 0 as *mut HTTP;
    http = &mut (*(*conn).connect_state).http_proxy;
    if (*http).sending as u32 == HTTPSEND_REQUEST as i32 as u32
    {
        return (1 as i32) << 16 as i32 + 0 as i32;
    }
    return (1 as i32) << 0 as i32;
}
unsafe extern "C" fn connect_init(
    mut data: *mut Curl_easy,
    mut reinit: bool,
) -> CURLcode {
    let mut s: *mut http_connect_state = 0 as *mut http_connect_state;
    let mut conn: *mut connectdata = (*data).conn;
    if !reinit {
        let mut result: CURLcode = CURLE_OK;
        result = Curl_get_upload_buffer(data);
        if result as u64 != 0 {
            return result;
        }
        s = Curl_ccalloc
            .expect(
                "non-null function pointer",
            )(
            1 as i32 as size_t,
            ::std::mem::size_of::<http_connect_state>() as u64,
        ) as *mut http_connect_state;
        if s.is_null() {
            return CURLE_OUT_OF_MEMORY;
        }
        Curl_infof(
            data,
            b"allocate connect buffer!\0" as *const u8 as *const i8,
        );
        let ref mut fresh1 = (*conn).connect_state;
        *fresh1 = s;
        Curl_dyn_init(&mut (*s).rcvbuf, 16384 as i32 as size_t);
        let ref mut fresh2 = (*s).prot_save;
        *fresh2 = (*data).req.p.http;
        let ref mut fresh3 = (*data).req.p.http;
        *fresh3 = &mut (*s).http_proxy;
        Curl_conncontrol(conn, 0 as i32);
    } else {
        s = (*conn).connect_state;
        Curl_dyn_reset(&mut (*s).rcvbuf);
    }
    (*s).tunnel_state = TUNNEL_INIT;
    (*s).keepon = KEEPON_CONNECT;
    (*s).cl = 0 as i32 as curl_off_t;
    (*s).set_close_connection(0 as i32 as bit);
    return CURLE_OK;
}
unsafe extern "C" fn connect_done(mut data: *mut Curl_easy) {
    let mut conn: *mut connectdata = (*data).conn;
    let mut s: *mut http_connect_state = (*conn).connect_state;
    if (*s).tunnel_state as u32 != TUNNEL_EXIT as i32 as u32 {
        (*s).tunnel_state = TUNNEL_EXIT;
        Curl_dyn_free(&mut (*s).rcvbuf);
        Curl_dyn_free(&mut (*s).req);
        let ref mut fresh4 = (*data).req.p.http;
        *fresh4 = (*s).prot_save;
        let ref mut fresh5 = (*s).prot_save;
        *fresh5 = 0 as *mut HTTP;
        Curl_infof(
            data,
            b"CONNECT phase completed!\0" as *const u8 as *const i8,
        );
    }
}
unsafe extern "C" fn CONNECT_host(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut hostname: *const i8,
    mut remote_port: i32,
    mut connecthostp: *mut *mut i8,
    mut hostp: *mut *mut i8,
) -> CURLcode {
    let mut hostheader: *mut i8 = 0 as *mut i8;
    let mut host: *mut i8 = 0 as *mut i8;
    let mut ipv6_ip: bool = ((*conn).bits).ipv6_ip() != 0;
    if hostname != (*conn).host.name as *const i8 {
        ipv6_ip = !(strchr(hostname, ':' as i32)).is_null();
    }
    hostheader = curl_maprintf(
        b"%s%s%s:%d\0" as *const u8 as *const i8,
        if ipv6_ip as i32 != 0 {
            b"[\0" as *const u8 as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
        hostname,
        if ipv6_ip as i32 != 0 {
            b"]\0" as *const u8 as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
        remote_port,
    );
    if hostheader.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    if (Curl_checkProxyheaders(
        data,
        conn,
        b"Host\0" as *const u8 as *const i8,
    ))
        .is_null()
    {
        host = curl_maprintf(
            b"Host: %s\r\n\0" as *const u8 as *const i8,
            hostheader,
        );
        if host.is_null() {
            Curl_cfree
                .expect("non-null function pointer")(hostheader as *mut libc::c_void);
            return CURLE_OUT_OF_MEMORY;
        }
    }
    *connecthostp = hostheader;
    *hostp = host;
    return CURLE_OK;
}
unsafe extern "C" fn CONNECT(
    mut data: *mut Curl_easy,
    mut sockindex: i32,
    mut hostname: *const i8,
    mut remote_port: i32,
) -> CURLcode {
    let mut subversion: i32 = 0 as i32;
    let mut k: *mut SingleRequest = &mut (*data).req;
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut tunnelsocket: curl_socket_t = (*conn).sock[sockindex as usize];
    let mut s: *mut http_connect_state = (*conn).connect_state;
    let mut http: *mut HTTP = (*data).req.p.http;
    let mut linep: *mut i8 = 0 as *mut i8;
    let mut perline: size_t = 0;
    if Curl_connect_complete(conn) {
        return CURLE_OK;
    }
    let ref mut fresh6 = (*conn).bits;
    (*fresh6).set_proxy_connect_closed(0 as i32 as bit);
    loop {
        let mut check: timediff_t = 0;
        if TUNNEL_INIT as i32 as u32
            == (*s).tunnel_state as u32
        {
            let mut req: *mut dynbuf = &mut (*s).req;
            let mut hostheader: *mut i8 = 0 as *mut i8;
            let mut host: *mut i8 = 0 as *mut i8;
            Curl_infof(
                data,
                b"Establish HTTP proxy tunnel to %s:%d\0" as *const u8
                    as *const i8,
                hostname,
                remote_port,
            );
            Curl_cfree
                .expect(
                    "non-null function pointer",
                )((*data).req.newurl as *mut libc::c_void);
            let ref mut fresh7 = (*data).req.newurl;
            *fresh7 = 0 as *mut i8;
            Curl_dyn_init(req, (1024 as i32 * 1024 as i32) as size_t);
            result = CONNECT_host(
                data,
                conn,
                hostname,
                remote_port,
                &mut hostheader,
                &mut host,
            );
            if result as u64 != 0 {
                return result;
            }
            result = Curl_http_output_auth(
                data,
                conn,
                b"CONNECT\0" as *const u8 as *const i8,
                HTTPREQ_GET,
                hostheader,
                1 as i32 != 0,
            );
            if result as u64 == 0 {
                let mut httpv: *const i8 = if (*conn).http_proxy.proxytype
                    as u32 == CURLPROXY_HTTP_1_0 as i32 as u32
                {
                    b"1.0\0" as *const u8 as *const i8
                } else {
                    b"1.1\0" as *const u8 as *const i8
                };
                result = Curl_dyn_addf(
                    req,
                    b"CONNECT %s HTTP/%s\r\n%s%s\0" as *const u8 as *const i8,
                    hostheader,
                    httpv,
                    if !host.is_null() {
                        host as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                    if !((*data).state.aptr.proxyuserpwd).is_null() {
                        (*data).state.aptr.proxyuserpwd as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                );
                if result as u64 == 0
                    && (Curl_checkProxyheaders(
                        data,
                        conn,
                        b"User-Agent\0" as *const u8 as *const i8,
                    ))
                        .is_null()
                    && !((*data).set.str_0[STRING_USERAGENT as i32 as usize])
                        .is_null()
                {
                    result = Curl_dyn_addf(
                        req,
                        b"User-Agent: %s\r\n\0" as *const u8 as *const i8,
                        (*data).set.str_0[STRING_USERAGENT as i32 as usize],
                    );
                }
                if result as u64 == 0
                    && (Curl_checkProxyheaders(
                        data,
                        conn,
                        b"Proxy-Connection\0" as *const u8 as *const i8,
                    ))
                        .is_null()
                {
                    result = Curl_dyn_add(
                        req,
                        b"Proxy-Connection: Keep-Alive\r\n\0" as *const u8
                            as *const i8,
                    );
                }
                if result as u64 == 0 {
                    result = Curl_add_custom_headers(data, 1 as i32 != 0, req);
                }
                if result as u64 == 0 {
                    result = Curl_dyn_add(
                        req,
                        b"\r\n\0" as *const u8 as *const i8,
                    );
                }
                if result as u64 == 0 {
                    result = Curl_buffer_send(
                        req,
                        data,
                        &mut (*data).info.request_size,
                        0 as i32 as curl_off_t,
                        sockindex,
                    );
                }
                if result as u64 != 0 {
                    Curl_failf(
                        data,
                        b"Failed sending CONNECT to proxy\0" as *const u8
                            as *const i8,
                    );
                }
            }
            Curl_cfree.expect("non-null function pointer")(host as *mut libc::c_void);
            Curl_cfree
                .expect("non-null function pointer")(hostheader as *mut libc::c_void);
            if result as u64 != 0 {
                return result;
            }
            (*s).tunnel_state = TUNNEL_CONNECT;
        }
        check = Curl_timeleft(data, 0 as *mut curltime, 1 as i32 != 0);
        if check <= 0 as i32 as i64 {
            Curl_failf(
                data,
                b"Proxy CONNECT aborted due to timeout\0" as *const u8
                    as *const i8,
            );
            return CURLE_OPERATION_TIMEDOUT;
        }
        if !Curl_conn_data_pending(conn, sockindex) && (*http).sending as u64 == 0 {
            return CURLE_OK;
        }
        if (*http).sending as u32
            == HTTPSEND_REQUEST as i32 as u32
        {
            if (*s).nsend == 0 {
                let mut fillcount: size_t = 0;
                let ref mut fresh8 = (*k).upload_fromhere;
                *fresh8 = (*data).state.ulbuf;
                result = Curl_fillreadbuffer(
                    data,
                    (*data).set.upload_buffer_size as size_t,
                    &mut fillcount,
                );
                if result as u64 != 0 {
                    return result;
                }
                (*s).nsend = fillcount;
            }
            if (*s).nsend != 0 {
                let mut bytes_written: ssize_t = 0;
                result = Curl_write(
                    data,
                    (*conn).writesockfd,
                    (*k).upload_fromhere as *const libc::c_void,
                    (*s).nsend,
                    &mut bytes_written,
                );
                if result as u64 == 0 {
                    result = Curl_debug(
                        data,
                        CURLINFO_HEADER_OUT,
                        (*k).upload_fromhere,
                        bytes_written as size_t,
                    ) as CURLcode;
                }
                let ref mut fresh9 = (*s).nsend;
                *fresh9 = (*fresh9 as u64)
                    .wrapping_sub(bytes_written as u64) as size_t as size_t;
                let ref mut fresh10 = (*k).upload_fromhere;
                *fresh10 = (*fresh10).offset(bytes_written as isize);
                return result;
            }
            (*http).sending = HTTPSEND_NADA;
        }
        let mut error: i32 = 0 as i32;
        while (*s).keepon as u64 != 0 {
            let mut gotbytes: ssize_t = 0;
            let mut byte: i8 = 0;
            result = Curl_read(
                data,
                tunnelsocket,
                &mut byte,
                1 as i32 as size_t,
                &mut gotbytes,
            );
            if result as u32 == CURLE_AGAIN as i32 as u32 {
                return CURLE_OK;
            }
            if Curl_pgrsUpdate(data) != 0 {
                return CURLE_ABORTED_BY_CALLBACK;
            }
            if result as u64 != 0 {
                (*s).keepon = KEEPON_DONE;
                break;
            } else if gotbytes <= 0 as i32 as i64 {
                if (*data).set.proxyauth != 0 && (*data).state.authproxy.avail != 0
                    && !((*data).state.aptr.proxyuserpwd).is_null()
                {
                    let ref mut fresh11 = (*conn).bits;
                    (*fresh11).set_proxy_connect_closed(1 as i32 as bit);
                    Curl_infof(
                        data,
                        b"Proxy CONNECT connection closed\0" as *const u8
                            as *const i8,
                    );
                } else {
                    error = 1 as i32;
                    Curl_failf(
                        data,
                        b"Proxy CONNECT aborted\0" as *const u8 as *const i8,
                    );
                }
                (*s).keepon = KEEPON_DONE;
                break;
            } else if (*s).keepon as u32
                    == KEEPON_IGNORE as i32 as u32
                {
                if (*s).cl != 0 {
                    let ref mut fresh12 = (*s).cl;
                    *fresh12 -= 1;
                    if !((*s).cl <= 0 as i32 as i64) {
                        continue;
                    }
                    (*s).keepon = KEEPON_DONE;
                    (*s).tunnel_state = TUNNEL_COMPLETE;
                    break;
                } else {
                    let mut r: CHUNKcode = CHUNKE_OK;
                    let mut extra: CURLcode = CURLE_OK;
                    let mut tookcareof: ssize_t = 0 as i32 as ssize_t;
                    r = Curl_httpchunk_read(
                        data,
                        &mut byte,
                        1 as i32 as ssize_t,
                        &mut tookcareof,
                        &mut extra,
                    );
                    if r as i32 == CHUNKE_STOP as i32 {
                        Curl_infof(
                            data,
                            b"chunk reading DONE\0" as *const u8 as *const i8,
                        );
                        (*s).keepon = KEEPON_DONE;
                        (*s).tunnel_state = TUNNEL_COMPLETE;
                    }
                }
            } else {
                if Curl_dyn_addn(
                    &mut (*s).rcvbuf,
                    &mut byte as *mut i8 as *const libc::c_void,
                    1 as i32 as size_t,
                ) as u64 != 0
                {
                    Curl_failf(
                        data,
                        b"CONNECT response too large!\0" as *const u8
                            as *const i8,
                    );
                    return CURLE_RECV_ERROR;
                }
                if byte as i32 != 0xa as i32 {
                    continue;
                }
                linep = Curl_dyn_ptr(&mut (*s).rcvbuf);
                perline = Curl_dyn_len(&mut (*s).rcvbuf);
                result = CURLE_OK as i32 as CURLcode;
                if result as u64 != 0 {
                    return result;
                }
                Curl_debug(data, CURLINFO_HEADER_IN, linep, perline);
                if ((*data).set).suppress_connect_headers() == 0 {
                    let mut writetype: i32 = (1 as i32)
                        << 1 as i32;
                    if ((*data).set).include_header() != 0 {
                        writetype |= (1 as i32) << 0 as i32;
                    }
                    result = Curl_client_write(data, writetype, linep, perline);
                    if result as u64 != 0 {
                        return result;
                    }
                }
                let ref mut fresh13 = (*data).info.header_size;
                *fresh13 += perline as i64;
                if '\r' as i32 == *linep.offset(0 as i32 as isize) as i32
                    || '\n' as i32
                        == *linep.offset(0 as i32 as isize) as i32
                {
                    if 407 as i32 == (*k).httpcode
                        && ((*data).state).authproblem() == 0
                    {
                        (*s).keepon = KEEPON_IGNORE;
                        if (*s).cl != 0 {
                            Curl_infof(
                                data,
                                b"Ignore %ld bytes of response-body\0" as *const u8
                                    as *const i8,
                                (*s).cl,
                            );
                        } else if (*s).chunked_encoding() != 0 {
                            let mut r_0: CHUNKcode = CHUNKE_OK;
                            let mut extra_0: CURLcode = CURLE_OK;
                            Curl_infof(
                                data,
                                b"Ignore chunked response-body\0" as *const u8
                                    as *const i8,
                            );
                            (*k).set_ignorebody(1 as i32 as bit);
                            if *linep.offset(1 as i32 as isize) as i32
                                == '\n' as i32
                            {
                                linep = linep.offset(1);
                            }
                            r_0 = Curl_httpchunk_read(
                                data,
                                linep.offset(1 as i32 as isize),
                                1 as i32 as ssize_t,
                                &mut gotbytes,
                                &mut extra_0,
                            );
                            if r_0 as i32 == CHUNKE_STOP as i32 {
                                Curl_infof(
                                    data,
                                    b"chunk reading DONE\0" as *const u8 as *const i8,
                                );
                                (*s).keepon = KEEPON_DONE;
                                (*s).tunnel_state = TUNNEL_COMPLETE;
                            }
                        } else {
                            (*s).keepon = KEEPON_DONE;
                        }
                    } else {
                        (*s).keepon = KEEPON_DONE;
                    }
                    if (*s).keepon as u32
                        == KEEPON_DONE as i32 as u32 && (*s).cl == 0
                    {
                        (*s).tunnel_state = TUNNEL_COMPLETE;
                    }
                } else {
                    if curl_strnequal(
                        b"WWW-Authenticate:\0" as *const u8 as *const i8,
                        linep,
                        strlen(
                            b"WWW-Authenticate:\0" as *const u8 as *const i8,
                        ),
                    ) != 0 && 401 as i32 == (*k).httpcode
                        || curl_strnequal(
                            b"Proxy-authenticate:\0" as *const u8 as *const i8,
                            linep,
                            strlen(
                                b"Proxy-authenticate:\0" as *const u8 as *const i8,
                            ),
                        ) != 0 && 407 as i32 == (*k).httpcode
                    {
                        let mut proxy: bool = if (*k).httpcode == 407 as i32 {
                            1 as i32
                        } else {
                            0 as i32
                        } != 0;
                        let mut auth: *mut i8 = Curl_copy_header_value(linep);
                        if auth.is_null() {
                            return CURLE_OUT_OF_MEMORY;
                        }
                        result = Curl_http_input_auth(data, proxy, auth);
                        Curl_cfree
                            .expect(
                                "non-null function pointer",
                            )(auth as *mut libc::c_void);
                        if result as u64 != 0 {
                            return result;
                        }
                    } else if curl_strnequal(
                            b"Content-Length:\0" as *const u8 as *const i8,
                            linep,
                            strlen(
                                b"Content-Length:\0" as *const u8 as *const i8,
                            ),
                        ) != 0
                        {
                        if (*k).httpcode / 100 as i32 == 2 as i32 {
                            Curl_infof(
                                data,
                                b"Ignoring Content-Length in CONNECT %03d response\0"
                                    as *const u8 as *const i8,
                                (*k).httpcode,
                            );
                        } else {
                            curlx_strtoofft(
                                linep
                                    .offset(
                                        strlen(
                                            b"Content-Length:\0" as *const u8 as *const i8,
                                        ) as isize,
                                    ),
                                0 as *mut *mut i8,
                                10 as i32,
                                &mut (*s).cl,
                            );
                        }
                    } else if Curl_compareheader(
                            linep,
                            b"Connection:\0" as *const u8 as *const i8,
                            b"close\0" as *const u8 as *const i8,
                        ) {
                        (*s).set_close_connection(1 as i32 as bit);
                    } else if curl_strnequal(
                            b"Transfer-Encoding:\0" as *const u8 as *const i8,
                            linep,
                            strlen(
                                b"Transfer-Encoding:\0" as *const u8 as *const i8,
                            ),
                        ) != 0
                        {
                        if (*k).httpcode / 100 as i32 == 2 as i32 {
                            Curl_infof(
                                data,
                                b"Ignoring Transfer-Encoding in CONNECT %03d response\0"
                                    as *const u8 as *const i8,
                                (*k).httpcode,
                            );
                        } else if Curl_compareheader(
                                linep,
                                b"Transfer-Encoding:\0" as *const u8 as *const i8,
                                b"chunked\0" as *const u8 as *const i8,
                            ) {
                            Curl_infof(
                                data,
                                b"CONNECT responded chunked\0" as *const u8
                                    as *const i8,
                            );
                            (*s).set_chunked_encoding(1 as i32 as bit);
                            Curl_httpchunk_init(data);
                        }
                    } else if Curl_compareheader(
                            linep,
                            b"Proxy-Connection:\0" as *const u8 as *const i8,
                            b"close\0" as *const u8 as *const i8,
                        ) {
                        (*s).set_close_connection(1 as i32 as bit);
                    } else if 2 as i32
                            == sscanf(
                                linep,
                                b"HTTP/1.%d %d\0" as *const u8 as *const i8,
                                &mut subversion as *mut i32,
                                &mut (*k).httpcode as *mut i32,
                            )
                        {
                        (*data).info.httpproxycode = (*k).httpcode;
                    }
                    Curl_dyn_reset(&mut (*s).rcvbuf);
                }
            }
        }
        if Curl_pgrsUpdate(data) != 0 {
            return CURLE_ABORTED_BY_CALLBACK;
        }
        if error != 0 {
            return CURLE_RECV_ERROR;
        }
        if (*data).info.httpproxycode / 100 as i32 != 2 as i32 {
            result = Curl_http_auth_act(data);
            if result as u64 != 0 {
                return result;
            }
            if ((*conn).bits).close() != 0 {
                (*s).set_close_connection(1 as i32 as bit);
            }
        }
        if (*s).close_connection() as i32 != 0 && !((*data).req.newurl).is_null()
        {
            Curl_closesocket(data, conn, (*conn).sock[sockindex as usize]);
            (*conn).sock[sockindex as usize] = -(1 as i32);
            break;
        } else {
            if !((*data).req.newurl).is_null()
                && TUNNEL_COMPLETE as i32 as u32
                    == (*s).tunnel_state as u32
            {
                connect_init(data, 1 as i32 != 0);
            }
            if ((*data).req.newurl).is_null() {
                break;
            }
        }
    }
    if (*data).info.httpproxycode / 100 as i32 != 2 as i32 {
        if (*s).close_connection() as i32 != 0 && !((*data).req.newurl).is_null()
        {
            let ref mut fresh14 = (*conn).bits;
            (*fresh14).set_proxy_connect_closed(1 as i32 as bit);
            Curl_infof(
                data,
                b"Connect me again please\0" as *const u8 as *const i8,
            );
            connect_done(data);
        } else {
            Curl_cfree
                .expect(
                    "non-null function pointer",
                )((*data).req.newurl as *mut libc::c_void);
            let ref mut fresh15 = (*data).req.newurl;
            *fresh15 = 0 as *mut i8;
            Curl_conncontrol(conn, 2 as i32);
            Curl_closesocket(data, conn, (*conn).sock[sockindex as usize]);
            (*conn).sock[sockindex as usize] = -(1 as i32);
        }
        (*s).tunnel_state = TUNNEL_INIT;
        if ((*conn).bits).proxy_connect_closed() != 0 {
            return CURLE_OK;
        }
        Curl_dyn_free(&mut (*s).rcvbuf);
        Curl_failf(
            data,
            b"Received HTTP code %d from proxy after CONNECT\0" as *const u8
                as *const i8,
            (*data).req.httpcode,
        );
        return CURLE_RECV_ERROR;
    }
    (*s).tunnel_state = TUNNEL_COMPLETE;
    Curl_cfree
        .expect(
            "non-null function pointer",
        )((*data).state.aptr.proxyuserpwd as *mut libc::c_void);
    let ref mut fresh16 = (*data).state.aptr.proxyuserpwd;
    *fresh16 = 0 as *mut i8;
    let ref mut fresh17 = (*data).state.aptr.proxyuserpwd;
    *fresh17 = 0 as *mut i8;
    let ref mut fresh18 = (*data).state.authproxy;
    (*fresh18).set_done(1 as i32 as bit);
    let ref mut fresh19 = (*data).state.authproxy;
    (*fresh19).set_multipass(0 as i32 as bit);
    Curl_infof(
        data,
        b"Proxy replied %d to CONNECT request\0" as *const u8 as *const i8,
        (*data).info.httpproxycode,
    );
    let ref mut fresh20 = (*data).req;
    (*fresh20).set_ignorebody(0 as i32 as bit);
    let ref mut fresh21 = (*conn).bits;
    (*fresh21).set_rewindaftersend(0 as i32 as bit);
    Curl_dyn_free(&mut (*s).rcvbuf);
    return CURLE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_connect_free(mut data: *mut Curl_easy) {
    let mut conn: *mut connectdata = (*data).conn;
    let mut s: *mut http_connect_state = (*conn).connect_state;
    if !s.is_null() {
        Curl_cfree.expect("non-null function pointer")(s as *mut libc::c_void);
        let ref mut fresh22 = (*conn).connect_state;
        *fresh22 = 0 as *mut http_connect_state;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Curl_proxyCONNECT(
    mut data: *mut Curl_easy,
    mut sockindex: i32,
    mut hostname: *const i8,
    mut remote_port: i32,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    if ((*conn).connect_state).is_null() {
        result = connect_init(data, 0 as i32 != 0);
        if result as u64 != 0 {
            return result;
        }
    }
    result = CONNECT(data, sockindex, hostname, remote_port);
    if result as u32 != 0 || Curl_connect_complete(conn) as i32 != 0 {
        connect_done(data);
    }
    return result;
}
