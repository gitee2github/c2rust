use ::libc;
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn snprintf(
        _: *mut i8,
        _: u64,
        _: *const i8,
        _: ...
    ) -> i32;
    
    
    
    
    
    
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: i32,
        _: u64,
    ) -> *mut libc::c_void;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::error::__xmlRaiseError;
pub use crate::src::globals::__xmlGenericError;
pub use crate::src::globals::__xmlGenericErrorContext;
pub use crate::src::hash::xmlHashAddEntry;
pub use crate::src::hash::xmlHashAddEntry2;
pub use crate::src::hash::xmlHashCreate;
pub use crate::src::hash::xmlHashFree;
pub use crate::src::hash::xmlHashLookup;
pub use crate::src::hash::xmlHashLookup2;
pub use crate::src::hash::xmlHashScan;
pub use crate::src::parser::xmlReadFile;
pub use crate::src::parser::xmlReadMemory;
pub use crate::src::tree::xmlAddChild;
pub use crate::src::tree::xmlAddNextSibling;
pub use crate::src::tree::xmlAddPrevSibling;
pub use crate::src::tree::xmlCopyDoc;
pub use crate::src::tree::xmlDocGetRootElement;
pub use crate::src::tree::xmlFreeDoc;
pub use crate::src::tree::xmlFreeNode;
pub use crate::src::tree::xmlGetProp;
pub use crate::src::tree::xmlHasProp;
pub use crate::src::tree::xmlIsBlankNode;
pub use crate::src::tree::xmlNewChild;
pub use crate::src::tree::xmlNewDocNode;
pub use crate::src::tree::xmlNewDocText;
pub use crate::src::tree::xmlNodeGetBase;
pub use crate::src::tree::xmlNodeGetContent;
pub use crate::src::tree::xmlNodeListGetString;
pub use crate::src::tree::xmlNodeSetContent;
pub use crate::src::tree::xmlSearchNs;
pub use crate::src::tree::xmlSetProp;
pub use crate::src::tree::xmlSplitQName2;
pub use crate::src::tree::xmlUnlinkNode;
pub use crate::src::tree::xmlUnsetProp;
pub use crate::src::tree::xmlValidateNCName;
pub use crate::src::uri::xmlBuildURI;
pub use crate::src::uri::xmlFreeURI;
pub use crate::src::uri::xmlParseURI;
pub use crate::src::uri::xmlURIEscapeStr;
pub use crate::src::valid::xmlValidateDocumentFinal;
pub use crate::src::xmlregexp::xmlAutomataCompile;
pub use crate::src::xmlregexp::xmlAutomataGetInitState;
pub use crate::src::xmlregexp::xmlAutomataIsDeterminist;
pub use crate::src::xmlregexp::xmlAutomataNewEpsilon;
pub use crate::src::xmlregexp::xmlAutomataNewTransition;
pub use crate::src::xmlregexp::xmlAutomataNewTransition2;
pub use crate::src::xmlregexp::xmlAutomataSetFinalState;
pub use crate::src::xmlregexp::xmlAutomataSetFlags;
pub use crate::src::xmlregexp::xmlFreeAutomata;
pub use crate::src::xmlregexp::xmlNewAutomata;
pub use crate::src::xmlregexp::xmlRegExecPushString;
pub use crate::src::xmlregexp::xmlRegExecPushString2;
pub use crate::src::xmlregexp::xmlRegFreeExecCtxt;
pub use crate::src::xmlregexp::xmlRegFreeRegexp;
pub use crate::src::xmlregexp::xmlRegNewExecCtxt;
pub use crate::src::xmlregexp::xmlRegexpIsDeterminist;
pub use crate::src::xmlsave::xmlDocDump;
pub use crate::src::xmlschemas::xmlSchemaCheckFacet;
pub use crate::src::xmlschemas::xmlSchemaFreeFacet;
pub use crate::src::xmlschemas::xmlSchemaNewFacet;
pub use crate::src::xmlschemastypes::xmlSchemaCleanupTypes;
pub use crate::src::xmlschemastypes::xmlSchemaCompareValues;
pub use crate::src::xmlschemastypes::xmlSchemaFreeValue;
pub use crate::src::xmlschemastypes::xmlSchemaGetPredefinedType;
pub use crate::src::xmlschemastypes::xmlSchemaValPredefTypeNode;
pub use crate::src::xmlschemastypes::xmlSchemaValidateFacet;
pub use crate::src::xmlstring::xmlCharStrdup;
pub use crate::src::xmlstring::xmlEscapeFormatString;
pub use crate::src::xmlstring::xmlStrEqual;
pub use crate::src::xmlstring::xmlStrcat;
pub use crate::src::xmlstring::xmlStrdup;
pub use crate::src::xmlstring::xmlStrlen;
pub use crate::src::dict::_xmlDict;
pub use crate::src::hash::_xmlHashTable;
pub use crate::src::HTMLtree::_IO_codecvt;
pub use crate::src::buf::_IO_marker;
pub use crate::src::globals::xmlFree;
pub use crate::src::globals::xmlMalloc;
pub use crate::src::globals::xmlMallocAtomic;
pub use crate::src::globals::xmlRealloc;
pub use crate::src::python::types::_IO_wide_data;
pub use crate::src::valid::_xmlValidState;
pub use crate::src::xmlregexp::_xmlAutomata;
pub use crate::src::xmlregexp::_xmlAutomataState;
pub use crate::src::xmlregexp::_xmlRegExecCtxt;
pub use crate::src::xmlregexp::_xmlRegexp;
pub use crate::src::xmlschemas::_xmlSchemaParserCtxt;
pub use crate::src::xmlschemastypes::_xmlSchemaVal;
pub type xmlChar = crate::src::HTMLparser::xmlChar;
pub type size_t = crate::src::HTMLparser::size_t;
pub type __off_t = crate::src::HTMLtree::__off_t;
pub type __off64_t = crate::src::HTMLtree::__off64_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::HTMLtree::_IO_FILE;
pub type _IO_lock_t = crate::src::HTMLtree::_IO_lock_t;
pub type FILE = crate::src::HTMLtree::FILE;
pub type ptrdiff_t = crate::src::HTMLparser::ptrdiff_t;
pub type xmlFreeFunc = crate::src::HTMLparser::xmlFreeFunc;
pub type xmlMallocFunc = crate::src::HTMLparser::xmlMallocFunc;
pub type xmlReallocFunc = crate::src::HTMLparser::xmlReallocFunc;
// #[derive(Copy, Clone)]

pub type _xmlNode = crate::src::HTMLparser::_xmlNode;
pub type xmlNs = crate::src::HTMLparser::xmlNs;
// #[derive(Copy, Clone)]

pub type _xmlNs = crate::src::HTMLparser::_xmlNs;
// #[derive(Copy, Clone)]

pub type _xmlDoc = crate::src::HTMLparser::_xmlDoc;
// #[derive(Copy, Clone)]

pub type _xmlDtd = crate::src::HTMLparser::_xmlDtd;
pub type xmlElementType = crate::src::HTMLparser::xmlElementType;
pub const XML_XINCLUDE_END: xmlElementType = 20;
pub const XML_XINCLUDE_START: xmlElementType = 19;
pub const XML_NAMESPACE_DECL: xmlElementType = 18;
pub const XML_ENTITY_DECL: xmlElementType = 17;
pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const XML_ELEMENT_DECL: xmlElementType = 15;
pub const XML_DTD_NODE: xmlElementType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const XML_NOTATION_NODE: xmlElementType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const XML_DOCUMENT_NODE: xmlElementType = 9;
pub const XML_COMMENT_NODE: xmlElementType = 8;
pub const XML_PI_NODE: xmlElementType = 7;
pub const XML_ENTITY_NODE: xmlElementType = 6;
pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const XML_TEXT_NODE: xmlElementType = 3;
pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const XML_ELEMENT_NODE: xmlElementType = 1;
pub type xmlNsType = crate::src::HTMLparser::xmlNsType;
// #[derive(Copy, Clone)]

pub type _xmlAttr = crate::src::HTMLparser::_xmlAttr;
pub type xmlAttributeType = crate::src::HTMLparser::xmlAttributeType;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlError = crate::src::HTMLparser::xmlError;
// #[derive(Copy, Clone)]

pub type _xmlError = crate::src::HTMLparser::_xmlError;
pub type xmlErrorLevel = crate::src::HTMLparser::xmlErrorLevel;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlAttrPtr = crate::src::HTMLparser::xmlAttrPtr;
pub type xmlAttr = crate::src::HTMLparser::xmlAttr;
pub type xmlNodePtr = crate::src::HTMLparser::xmlNodePtr;
pub type xmlNode = crate::src::HTMLparser::xmlNode;
pub type xmlHashTablePtr = crate::src::HTMLparser::xmlHashTablePtr;
pub type xmlHashTable = crate::src::HTMLparser::xmlHashTable;
pub type xmlValidCtxt = crate::src::HTMLparser::xmlValidCtxt;
// #[derive(Copy, Clone)]

pub type _xmlValidCtxt = crate::src::HTMLparser::_xmlValidCtxt;
pub type xmlAutomataStatePtr = crate::src::HTMLparser::xmlAutomataStatePtr;
pub type xmlAutomataState = crate::src::HTMLparser::xmlAutomataState;
pub type xmlAutomataPtr = crate::src::HTMLparser::xmlAutomataPtr;
pub type xmlAutomata = crate::src::HTMLparser::xmlAutomata;
pub type xmlValidState = crate::src::HTMLparser::xmlValidState;
pub type xmlDocPtr = crate::src::HTMLparser::xmlDocPtr;
pub type xmlDoc = crate::src::HTMLparser::xmlDoc;
pub type xmlValidityWarningFunc = crate::src::HTMLparser::xmlValidityWarningFunc;
pub type xmlValidityErrorFunc = crate::src::HTMLparser::xmlValidityErrorFunc;
pub type xmlStructuredErrorFunc = crate::src::HTMLparser::xmlStructuredErrorFunc;
pub type xmlErrorPtr = crate::src::HTMLparser::xmlErrorPtr;
pub type xmlRegexp = crate::src::SAX2::xmlRegexp;
pub type xmlRegexpPtr = crate::src::SAX2::xmlRegexpPtr;
pub type xmlRegExecCtxt = _xmlRegExecCtxt;
pub type xmlRegExecCtxtPtr = *mut xmlRegExecCtxt;
pub type xmlRegExecCallbacks = Option::<
    unsafe extern "C" fn(
        xmlRegExecCtxtPtr,
        *const xmlChar,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> (),
>;
pub type xmlNsPtr = crate::src::HTMLtree::xmlNsPtr;
pub type xmlHashDeallocator = crate::src::HTMLparser::xmlHashDeallocator;
pub type xmlHashScanner = crate::src::catalog::xmlHashScanner;
pub type C2RustUnnamed = u32;
pub const XML_FROM_URI: C2RustUnnamed = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed = 28;
pub const XML_FROM_I18N: C2RustUnnamed = 27;
pub const XML_FROM_MODULE: C2RustUnnamed = 26;
pub const XML_FROM_WRITER: C2RustUnnamed = 25;
pub const XML_FROM_CHECK: C2RustUnnamed = 24;
pub const XML_FROM_VALID: C2RustUnnamed = 23;
pub const XML_FROM_XSLT: C2RustUnnamed = 22;
pub const XML_FROM_C14N: C2RustUnnamed = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed = 13;
pub const XML_FROM_XPATH: C2RustUnnamed = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed = 11;
pub const XML_FROM_HTTP: C2RustUnnamed = 10;
pub const XML_FROM_FTP: C2RustUnnamed = 9;
pub const XML_FROM_IO: C2RustUnnamed = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed = 6;
pub const XML_FROM_HTML: C2RustUnnamed = 5;
pub const XML_FROM_DTD: C2RustUnnamed = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed = 3;
pub const XML_FROM_TREE: C2RustUnnamed = 2;
pub const XML_FROM_PARSER: C2RustUnnamed = 1;
pub const XML_FROM_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = u32;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_0 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_0 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_0 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_0 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_0 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_0 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_0 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_0 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_0 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_0 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_0 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_0 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_0 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_0 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_0 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_0 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_0 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_0 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_0 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_0 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_0 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_0 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_0 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_0 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_0 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_0 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_0 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_0 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_0 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_0 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_0 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_0 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_0 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_0 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_0 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_0 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_0 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_0 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_0 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_0 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_0 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_0 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_0 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_0 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_0 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_0 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_0 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_0 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_0 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_0 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_0 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_0 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_0 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_0 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_0 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_0 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_0 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_0 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_0 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_0 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_0 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_0 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_0 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_0 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_0 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_0 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_0 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_0 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_0 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_0 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_0 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_0 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_0 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_0 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_0 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_0 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_0 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_0 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_0 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_0 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_0 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_0 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_0 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_0 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_0 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_0 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_0 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_0 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_0 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_0 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_0 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_0 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_0 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_0 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_0 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_0 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_0 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_0 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_0 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_0 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_0 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_0 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_0 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_0 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_0 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_0 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_0 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_0 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_0 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_0 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_0 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_0 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_0 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_0 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_0 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_0 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_0 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_0 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_0 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_0 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_0 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_0 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_0 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_0 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_0 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_0 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_0 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_0 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_0 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_0 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_0 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_0 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_0 = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_0 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_0 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_0 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_0 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_0 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_0 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_0 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_0 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_0 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_0 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_0 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_0 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_0 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_0 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_0 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_0 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_0 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_0 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_0 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_0 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_0 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_0 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_0 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_0 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_0 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_0 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_0 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_0 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_0 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_0 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_0 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_0 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_0 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_0 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_0 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_0 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_0 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_0 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_0 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_0 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_0 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_0 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_0 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_0 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_0 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_0 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_0 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_0 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_0 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_0 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_0 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_0 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_0 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_0 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_0 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_0 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_0 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_0 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_0 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_0 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_0 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_0 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_0 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_0 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_0 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_0 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_0 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_0 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_0 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_0 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_0 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_0 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_0 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_0 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_0 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_0 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_0 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_0 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_0 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_0 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_0 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_0 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_0 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_0 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_0 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_0 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_0 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_0 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_0 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_0 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_0 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_0 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_0 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_0 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_0 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_0 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_0 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_0 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_0 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_0 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_0 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_0 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_0 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_0 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_0 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_0 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_0 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_0 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_0 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_0 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_0 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_0 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_0 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_0 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_0 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_0 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_0 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_0 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_0 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_0 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_0 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_0 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_0 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_0 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_0 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_0 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_0 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_0 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_0 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_0 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_0 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_0 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_0 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_0 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_0 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_0 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_0 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_0 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_0 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_0 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_0 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_0 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_0 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_0 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_0 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_0 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_0 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_0 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_0 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_0 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_0 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_0 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_0 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_0 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_0 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_0 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_0 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_0 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_0 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_0 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_0 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_0 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_0 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_0 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_0 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_0 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_0 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_0 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_0 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_0 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_0 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_0 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_0 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_0 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_0 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_0 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_0 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_0 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_0 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_0 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_0 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_0 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_0 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_0 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_0 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_0 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_0 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_0 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_0 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_0 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_0 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_0 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_0 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_0 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_0 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_0 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_0 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_0 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_0 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_0 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_0 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_0 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_0 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_0 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_0 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_0 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_0 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_0 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_0 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_0 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_0 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_0 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_0 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_0 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_0 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_0 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_0 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_0 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_0 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_0 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_0 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_0 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_0 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_0 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_0 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_0 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_0 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_0 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_0 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_0 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_0 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_0 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_0 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_0 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_0 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_0 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_0 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_0 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_0 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_0 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_0 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_0 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_0 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_0 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_0 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_0 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_0 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_0 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_0 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_0 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_0 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_0 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_0 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_0 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_0 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_0 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_0 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_0 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_0 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_0 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_0 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_0 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_0 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_0 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_0 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_0 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_0 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_0 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_0 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_0 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_0 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_0 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_0 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_0 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_0 = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_0 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_0 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_0 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_0 = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_0 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_0 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_0 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_0 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_0 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_0 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_0 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_0 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_0 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_0 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_0 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_0 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_0 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_0 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_0 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_0 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_0 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_0 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_0 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_0 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_0 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_0 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_0 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_0 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_0 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_0 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_0 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_0 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_0 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_0 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_0 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_0 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_0 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_0 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_0 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_0 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_0 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_0 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_0 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_0 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_0 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_0 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_0 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_0 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_0 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_0 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_0 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_0 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_0 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_0 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_0 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_0 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_0 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_0 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_0 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_0 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_0 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_0 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_0 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_0 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_0 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_0 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_0 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_0 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_0 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_0 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_0 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_0 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_0 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_0 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_0 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_0 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_0 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_0 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_0 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_0 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_0 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_0 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_0 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_0 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_0 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_0 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_0 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_0 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_0 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_0 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_0 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_0 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_0 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_0 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_0 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_0 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_0 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_0 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_0 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_0 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_0 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_0 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_0 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_0 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_0 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_0 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_0 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_0 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_0 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_0 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_0 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_0 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_0 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_0 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_0 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_0 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_0 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_0 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_0 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_0 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_0 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_0 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_0 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_0 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_0 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_0 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_0 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_0 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_0 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_0 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_0 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_0 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_0 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_0 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_0 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_0 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_0 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_0 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_0 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_0 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_0 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1000;
pub const XML_HTML_INCORRECTLY_OPENED_COMMENT: C2RustUnnamed_0 = 802;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_0 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_0 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_0 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_0 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_0 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_0 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_0 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_0 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_0 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_0 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_0 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_0 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_0 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_0 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_0 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_0 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_0 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_0 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_0 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_0 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_0 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_0 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_0 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_0 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_0 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_0 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_0 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_0 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_0 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_0 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_0 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_0 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_0 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_0 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_0 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_0 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_0 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_0 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_0 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_0 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_0 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_0 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_0 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_0 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_0 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_0 = 200;
pub const XML_ERR_COMMENT_ABRUPTLY_ENDED: C2RustUnnamed_0 = 112;
pub const XML_ERR_USER_STOP: C2RustUnnamed_0 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_0 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_0 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_0 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_0 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_0 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_0 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_0 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_0 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_0 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_0 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_0 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_0 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_0 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_0 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_0 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_0 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_0 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_0 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_0 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_0 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_0 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_0 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_0 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_0 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_0 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_0 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_0 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_0 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_0 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_0 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_0 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_0 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_0 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_0 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_0 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_0 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_0 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_0 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_0 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_0 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_0 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_0 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_0 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_0 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_0 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_0 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_0 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_0 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_0 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_0 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_0 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_0 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_0 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_0 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_0 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_0 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_0 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_0 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_0 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_0 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_0 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_0 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_0 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_0 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_0 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_0 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_0 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_0 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_0 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_0 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_0 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_0 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_0 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_0 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_0 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_0 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_0 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_0 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_0 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_0 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_0 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_0 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_0 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_0 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_0 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_0 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_0 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_0 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_0 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_0 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_0 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_0 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_0 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_0 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_0 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_0 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_0 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_0 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_0 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_0 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_0 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_0 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_0 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_0 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_0 = 3;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_0 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_0 = 1;
pub const XML_ERR_OK: C2RustUnnamed_0 = 0;
pub type xmlGenericErrorFunc = crate::src::HTMLparser::xmlGenericErrorFunc;
pub type xmlValidCtxtPtr = crate::src::SAX2::xmlValidCtxtPtr;
// #[derive(Copy, Clone)]

pub type _xmlURI = crate::src::SAX2::_xmlURI;
pub type xmlURI = crate::src::SAX2::xmlURI;
pub type xmlURIPtr = crate::src::SAX2::xmlURIPtr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNG {
    pub _private: *mut libc::c_void,
    pub topgrammar: xmlRelaxNGGrammarPtr,
    pub doc: xmlDocPtr,
    pub idref: i32,
    pub defs: xmlHashTablePtr,
    pub refs: xmlHashTablePtr,
    pub documents: xmlRelaxNGDocumentPtr,
    pub includes: xmlRelaxNGIncludePtr,
    pub defNr: i32,
    pub defTab: *mut xmlRelaxNGDefinePtr,
}
pub type xmlRelaxNGDefinePtr = *mut xmlRelaxNGDefine;
pub type xmlRelaxNGDefine = _xmlRelaxNGDefine;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGDefine {
    pub type_0: xmlRelaxNGType,
    pub node: xmlNodePtr,
    pub name: *mut xmlChar,
    pub ns: *mut xmlChar,
    pub value: *mut xmlChar,
    pub data: *mut libc::c_void,
    pub content: xmlRelaxNGDefinePtr,
    pub parent: xmlRelaxNGDefinePtr,
    pub next: xmlRelaxNGDefinePtr,
    pub attrs: xmlRelaxNGDefinePtr,
    pub nameClass: xmlRelaxNGDefinePtr,
    pub nextHash: xmlRelaxNGDefinePtr,
    pub depth: i16,
    pub dflags: i16,
    pub contModel: xmlRegexpPtr,
}
pub type xmlRelaxNGType = i32;
pub const XML_RELAXNG_START: xmlRelaxNGType = 20;
pub const XML_RELAXNG_INTERLEAVE: xmlRelaxNGType = 19;
pub const XML_RELAXNG_GROUP: xmlRelaxNGType = 18;
pub const XML_RELAXNG_CHOICE: xmlRelaxNGType = 17;
pub const XML_RELAXNG_ONEORMORE: xmlRelaxNGType = 16;
pub const XML_RELAXNG_ZEROORMORE: xmlRelaxNGType = 15;
pub const XML_RELAXNG_OPTIONAL: xmlRelaxNGType = 14;
pub const XML_RELAXNG_PARENTREF: xmlRelaxNGType = 13;
pub const XML_RELAXNG_EXTERNALREF: xmlRelaxNGType = 12;
pub const XML_RELAXNG_REF: xmlRelaxNGType = 11;
pub const XML_RELAXNG_DEF: xmlRelaxNGType = 10;
pub const XML_RELAXNG_ATTRIBUTE: xmlRelaxNGType = 9;
pub const XML_RELAXNG_LIST: xmlRelaxNGType = 8;
pub const XML_RELAXNG_VALUE: xmlRelaxNGType = 7;
pub const XML_RELAXNG_PARAM: xmlRelaxNGType = 6;
pub const XML_RELAXNG_DATATYPE: xmlRelaxNGType = 5;
pub const XML_RELAXNG_ELEMENT: xmlRelaxNGType = 4;
pub const XML_RELAXNG_TEXT: xmlRelaxNGType = 3;
pub const XML_RELAXNG_EXCEPT: xmlRelaxNGType = 2;
pub const XML_RELAXNG_NOT_ALLOWED: xmlRelaxNGType = 1;
pub const XML_RELAXNG_EMPTY: xmlRelaxNGType = 0;
pub const XML_RELAXNG_NOOP: xmlRelaxNGType = -1;
pub type xmlRelaxNGIncludePtr = *mut xmlRelaxNGInclude;
pub type xmlRelaxNGInclude = _xmlRelaxNGInclude;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGInclude {
    pub next: xmlRelaxNGIncludePtr,
    pub href: *mut xmlChar,
    pub doc: xmlDocPtr,
    pub content: xmlRelaxNGDefinePtr,
    pub schema: xmlRelaxNGPtr,
}
pub type xmlRelaxNGPtr = crate::src::debugXML::xmlRelaxNGPtr;
pub type xmlRelaxNG = crate::src::debugXML::xmlRelaxNG;
pub type xmlRelaxNGDocumentPtr = *mut xmlRelaxNGDocument;
pub type xmlRelaxNGDocument = _xmlRelaxNGDocument;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGDocument {
    pub next: xmlRelaxNGDocumentPtr,
    pub href: *mut xmlChar,
    pub doc: xmlDocPtr,
    pub content: xmlRelaxNGDefinePtr,
    pub schema: xmlRelaxNGPtr,
    pub externalRef: i32,
}
pub type xmlRelaxNGGrammarPtr = *mut xmlRelaxNGGrammar;
pub type xmlRelaxNGGrammar = _xmlRelaxNGGrammar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGGrammar {
    pub parent: xmlRelaxNGGrammarPtr,
    pub children: xmlRelaxNGGrammarPtr,
    pub next: xmlRelaxNGGrammarPtr,
    pub start: xmlRelaxNGDefinePtr,
    pub combine: xmlRelaxNGCombine,
    pub startList: xmlRelaxNGDefinePtr,
    pub defs: xmlHashTablePtr,
    pub refs: xmlHashTablePtr,
}
pub type xmlRelaxNGCombine = u32;
pub const XML_RELAXNG_COMBINE_INTERLEAVE: xmlRelaxNGCombine = 2;
pub const XML_RELAXNG_COMBINE_CHOICE: xmlRelaxNGCombine = 1;
pub const XML_RELAXNG_COMBINE_UNDEFINED: xmlRelaxNGCombine = 0;
pub type xmlRelaxNGValidityErrorFunc = crate::src::debugXML::xmlRelaxNGValidityErrorFunc;
pub type xmlRelaxNGValidityWarningFunc = crate::src::debugXML::xmlRelaxNGValidityWarningFunc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGParserCtxt {
    pub userData: *mut libc::c_void,
    pub error: xmlRelaxNGValidityErrorFunc,
    pub warning: xmlRelaxNGValidityWarningFunc,
    pub serror: xmlStructuredErrorFunc,
    pub err: xmlRelaxNGValidErr,
    pub schema: xmlRelaxNGPtr,
    pub grammar: xmlRelaxNGGrammarPtr,
    pub parentgrammar: xmlRelaxNGGrammarPtr,
    pub flags: i32,
    pub nbErrors: i32,
    pub nbWarnings: i32,
    pub define: *const xmlChar,
    pub def: xmlRelaxNGDefinePtr,
    pub nbInterleaves: i32,
    pub interleaves: xmlHashTablePtr,
    pub documents: xmlRelaxNGDocumentPtr,
    pub includes: xmlRelaxNGIncludePtr,
    pub URL: *mut xmlChar,
    pub document: xmlDocPtr,
    pub defNr: i32,
    pub defMax: i32,
    pub defTab: *mut xmlRelaxNGDefinePtr,
    pub buffer: *const i8,
    pub size: i32,
    pub doc: xmlRelaxNGDocumentPtr,
    pub docNr: i32,
    pub docMax: i32,
    pub docTab: *mut xmlRelaxNGDocumentPtr,
    pub inc: xmlRelaxNGIncludePtr,
    pub incNr: i32,
    pub incMax: i32,
    pub incTab: *mut xmlRelaxNGIncludePtr,
    pub idref: i32,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
    pub crng: i32,
    pub freedoc: i32,
}
pub type xmlRelaxNGValidErr = u32;
pub const XML_RELAXNG_ERR_TEXTWRONG: xmlRelaxNGValidErr = 39;
pub const XML_RELAXNG_ERR_ELEMWRONG: xmlRelaxNGValidErr = 38;
pub const XML_RELAXNG_ERR_INTERNAL: xmlRelaxNGValidErr = 37;
pub const XML_RELAXNG_ERR_LACKDATA: xmlRelaxNGValidErr = 36;
pub const XML_RELAXNG_ERR_EXTRADATA: xmlRelaxNGValidErr = 35;
pub const XML_RELAXNG_ERR_NOGRAMMAR: xmlRelaxNGValidErr = 34;
pub const XML_RELAXNG_ERR_LIST: xmlRelaxNGValidErr = 33;
pub const XML_RELAXNG_ERR_VALUE: xmlRelaxNGValidErr = 32;
pub const XML_RELAXNG_ERR_DATATYPE: xmlRelaxNGValidErr = 31;
pub const XML_RELAXNG_ERR_LISTELEM: xmlRelaxNGValidErr = 30;
pub const XML_RELAXNG_ERR_VALELEM: xmlRelaxNGValidErr = 29;
pub const XML_RELAXNG_ERR_DATAELEM: xmlRelaxNGValidErr = 28;
pub const XML_RELAXNG_ERR_INVALIDATTR: xmlRelaxNGValidErr = 27;
pub const XML_RELAXNG_ERR_EXTRACONTENT: xmlRelaxNGValidErr = 26;
pub const XML_RELAXNG_ERR_CONTENTVALID: xmlRelaxNGValidErr = 25;
pub const XML_RELAXNG_ERR_ATTRVALID: xmlRelaxNGValidErr = 24;
pub const XML_RELAXNG_ERR_NOTELEM: xmlRelaxNGValidErr = 23;
pub const XML_RELAXNG_ERR_NOELEM: xmlRelaxNGValidErr = 22;
pub const XML_RELAXNG_ERR_ELEMNOTEMPTY: xmlRelaxNGValidErr = 21;
pub const XML_RELAXNG_ERR_ATTREXTRANS: xmlRelaxNGValidErr = 20;
pub const XML_RELAXNG_ERR_ELEMEXTRANS: xmlRelaxNGValidErr = 19;
pub const XML_RELAXNG_ERR_ATTRWRONGNS: xmlRelaxNGValidErr = 18;
pub const XML_RELAXNG_ERR_ELEMWRONGNS: xmlRelaxNGValidErr = 17;
pub const XML_RELAXNG_ERR_ATTRNONS: xmlRelaxNGValidErr = 16;
pub const XML_RELAXNG_ERR_ELEMNONS: xmlRelaxNGValidErr = 15;
pub const XML_RELAXNG_ERR_ATTRNAME: xmlRelaxNGValidErr = 14;
pub const XML_RELAXNG_ERR_ELEMNAME: xmlRelaxNGValidErr = 13;
pub const XML_RELAXNG_ERR_INTEREXTRA: xmlRelaxNGValidErr = 12;
pub const XML_RELAXNG_ERR_INTERSEQ: xmlRelaxNGValidErr = 11;
pub const XML_RELAXNG_ERR_INTERNODATA: xmlRelaxNGValidErr = 10;
pub const XML_RELAXNG_ERR_LISTEMPTY: xmlRelaxNGValidErr = 9;
pub const XML_RELAXNG_ERR_LISTEXTRA: xmlRelaxNGValidErr = 8;
pub const XML_RELAXNG_ERR_NODEFINE: xmlRelaxNGValidErr = 7;
pub const XML_RELAXNG_ERR_NOSTATE: xmlRelaxNGValidErr = 6;
pub const XML_RELAXNG_ERR_TYPECMP: xmlRelaxNGValidErr = 5;
pub const XML_RELAXNG_ERR_DUPID: xmlRelaxNGValidErr = 4;
pub const XML_RELAXNG_ERR_TYPEVAL: xmlRelaxNGValidErr = 3;
pub const XML_RELAXNG_ERR_TYPE: xmlRelaxNGValidErr = 2;
pub const XML_RELAXNG_ERR_MEMORY: xmlRelaxNGValidErr = 1;
pub const XML_RELAXNG_OK: xmlRelaxNGValidErr = 0;
pub type xmlRelaxNGParserCtxt = crate::src::debugXML::xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = crate::src::debugXML::xmlRelaxNGParserCtxtPtr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGValidCtxt {
    pub userData: *mut libc::c_void,
    pub error: xmlRelaxNGValidityErrorFunc,
    pub warning: xmlRelaxNGValidityWarningFunc,
    pub serror: xmlStructuredErrorFunc,
    pub nbErrors: i32,
    pub schema: xmlRelaxNGPtr,
    pub doc: xmlDocPtr,
    pub flags: i32,
    pub depth: i32,
    pub idref: i32,
    pub errNo: i32,
    pub err: xmlRelaxNGValidErrorPtr,
    pub errNr: i32,
    pub errMax: i32,
    pub errTab: xmlRelaxNGValidErrorPtr,
    pub state: xmlRelaxNGValidStatePtr,
    pub states: xmlRelaxNGStatesPtr,
    pub freeState: xmlRelaxNGStatesPtr,
    pub freeStatesNr: i32,
    pub freeStatesMax: i32,
    pub freeStates: *mut xmlRelaxNGStatesPtr,
    pub elem: xmlRegExecCtxtPtr,
    pub elemNr: i32,
    pub elemMax: i32,
    pub elemTab: *mut xmlRegExecCtxtPtr,
    pub pstate: i32,
    pub pnode: xmlNodePtr,
    pub pdef: xmlRelaxNGDefinePtr,
    pub perr: i32,
}
pub type xmlRelaxNGStatesPtr = *mut xmlRelaxNGStates;
pub type xmlRelaxNGStates = _xmlRelaxNGStates;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGStates {
    pub nbState: i32,
    pub maxState: i32,
    pub tabState: *mut xmlRelaxNGValidStatePtr,
}
pub type xmlRelaxNGValidStatePtr = *mut xmlRelaxNGValidState;
pub type xmlRelaxNGValidState = _xmlRelaxNGValidState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGValidState {
    pub node: xmlNodePtr,
    pub seq: xmlNodePtr,
    pub nbAttrs: i32,
    pub maxAttrs: i32,
    pub nbAttrLeft: i32,
    pub value: *mut xmlChar,
    pub endvalue: *mut xmlChar,
    pub attrs: *mut xmlAttrPtr,
}
pub type xmlRelaxNGValidErrorPtr = *mut xmlRelaxNGValidError;
pub type xmlRelaxNGValidError = _xmlRelaxNGValidError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGValidError {
    pub err: xmlRelaxNGValidErr,
    pub flags: i32,
    pub node: xmlNodePtr,
    pub seq: xmlNodePtr,
    pub arg1: *const xmlChar,
    pub arg2: *const xmlChar,
}
pub type xmlRelaxNGValidCtxt = crate::src::debugXML::xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = crate::src::debugXML::xmlRelaxNGValidCtxtPtr;
pub type C2RustUnnamed_1 = u32;
pub const XML_RELAXNGP_CRNG: C2RustUnnamed_1 = 2;
pub const XML_RELAXNGP_FREE_DOC: C2RustUnnamed_1 = 1;
pub const XML_RELAXNGP_NONE: C2RustUnnamed_1 = 0;
pub type xmlRelaxNGTypeFree = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
pub type xmlRelaxNGFacetCheck = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        *mut libc::c_void,
    ) -> i32,
>;
pub type xmlRelaxNGTypeCompare = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        xmlNodePtr,
        *mut libc::c_void,
        *const xmlChar,
        xmlNodePtr,
    ) -> i32,
>;
pub type xmlRelaxNGTypeCheck = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *mut *mut libc::c_void,
        xmlNodePtr,
    ) -> i32,
>;
pub type xmlRelaxNGTypeHave = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> i32,
>;
pub type xmlRelaxNGTypeLibraryPtr = *mut xmlRelaxNGTypeLibrary;
pub type xmlRelaxNGTypeLibrary = _xmlRelaxNGTypeLibrary;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGTypeLibrary {
    pub namespace: *const xmlChar,
    pub data: *mut libc::c_void,
    pub have: xmlRelaxNGTypeHave,
    pub check: xmlRelaxNGTypeCheck,
    pub comp: xmlRelaxNGTypeCompare,
    pub facet: xmlRelaxNGFacetCheck,
    pub freef: xmlRelaxNGTypeFree,
}
pub type xmlSchemaValPtr = *mut xmlSchemaVal;
pub type xmlSchemaVal = _xmlSchemaVal;
pub type xmlSchemaFacetPtr = *mut xmlSchemaFacet;
pub type xmlSchemaFacet = _xmlSchemaFacet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaFacet {
    pub type_0: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaFacet,
    pub value: *const xmlChar,
    pub id: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub node: xmlNodePtr,
    pub fixed: i32,
    pub whitespace: i32,
    pub val: xmlSchemaValPtr,
    pub regexp: xmlRegexpPtr,
}
pub type xmlSchemaAnnotPtr = *mut xmlSchemaAnnot;
pub type xmlSchemaAnnot = _xmlSchemaAnnot;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaAnnot {
    pub next: *mut _xmlSchemaAnnot,
    pub content: xmlNodePtr,
}
pub type xmlSchemaTypeType = u32;
pub const XML_SCHEMA_EXTRA_ATTR_USE_PROHIB: xmlSchemaTypeType = 2001;
pub const XML_SCHEMA_EXTRA_QNAMEREF: xmlSchemaTypeType = 2000;
pub const XML_SCHEMA_FACET_MINLENGTH: xmlSchemaTypeType = 1011;
pub const XML_SCHEMA_FACET_MAXLENGTH: xmlSchemaTypeType = 1010;
pub const XML_SCHEMA_FACET_LENGTH: xmlSchemaTypeType = 1009;
pub const XML_SCHEMA_FACET_WHITESPACE: xmlSchemaTypeType = 1008;
pub const XML_SCHEMA_FACET_ENUMERATION: xmlSchemaTypeType = 1007;
pub const XML_SCHEMA_FACET_PATTERN: xmlSchemaTypeType = 1006;
pub const XML_SCHEMA_FACET_FRACTIONDIGITS: xmlSchemaTypeType = 1005;
pub const XML_SCHEMA_FACET_TOTALDIGITS: xmlSchemaTypeType = 1004;
pub const XML_SCHEMA_FACET_MAXEXCLUSIVE: xmlSchemaTypeType = 1003;
pub const XML_SCHEMA_FACET_MAXINCLUSIVE: xmlSchemaTypeType = 1002;
pub const XML_SCHEMA_FACET_MINEXCLUSIVE: xmlSchemaTypeType = 1001;
pub const XML_SCHEMA_FACET_MININCLUSIVE: xmlSchemaTypeType = 1000;
pub const XML_SCHEMA_TYPE_ATTRIBUTE_USE: xmlSchemaTypeType = 26;
pub const XML_SCHEMA_TYPE_PARTICLE: xmlSchemaTypeType = 25;
pub const XML_SCHEMA_TYPE_IDC_KEYREF: xmlSchemaTypeType = 24;
pub const XML_SCHEMA_TYPE_IDC_KEY: xmlSchemaTypeType = 23;
pub const XML_SCHEMA_TYPE_IDC_UNIQUE: xmlSchemaTypeType = 22;
pub const XML_SCHEMA_TYPE_ANY_ATTRIBUTE: xmlSchemaTypeType = 21;
pub const XML_SCHEMA_TYPE_UNION: xmlSchemaTypeType = 20;
pub const XML_SCHEMA_TYPE_LIST: xmlSchemaTypeType = 19;
pub const XML_SCHEMA_TYPE_NOTATION: xmlSchemaTypeType = 18;
pub const XML_SCHEMA_TYPE_GROUP: xmlSchemaTypeType = 17;
pub const XML_SCHEMA_TYPE_ATTRIBUTEGROUP: xmlSchemaTypeType = 16;
pub const XML_SCHEMA_TYPE_ATTRIBUTE: xmlSchemaTypeType = 15;
pub const XML_SCHEMA_TYPE_ELEMENT: xmlSchemaTypeType = 14;
pub const XML_SCHEMA_TYPE_EXTENSION: xmlSchemaTypeType = 13;
pub const XML_SCHEMA_TYPE_RESTRICTION: xmlSchemaTypeType = 12;
pub const XML_SCHEMA_TYPE_UR: xmlSchemaTypeType = 11;
pub const XML_SCHEMA_TYPE_COMPLEX_CONTENT: xmlSchemaTypeType = 10;
pub const XML_SCHEMA_TYPE_SIMPLE_CONTENT: xmlSchemaTypeType = 9;
pub const XML_SCHEMA_TYPE_ALL: xmlSchemaTypeType = 8;
pub const XML_SCHEMA_TYPE_CHOICE: xmlSchemaTypeType = 7;
pub const XML_SCHEMA_TYPE_SEQUENCE: xmlSchemaTypeType = 6;
pub const XML_SCHEMA_TYPE_COMPLEX: xmlSchemaTypeType = 5;
pub const XML_SCHEMA_TYPE_SIMPLE: xmlSchemaTypeType = 4;
pub const XML_SCHEMA_TYPE_FACET: xmlSchemaTypeType = 3;
pub const XML_SCHEMA_TYPE_ANY: xmlSchemaTypeType = 2;
pub const XML_SCHEMA_TYPE_BASIC: xmlSchemaTypeType = 1;
pub type xmlSchemaTypePtr = *mut xmlSchemaType;
pub type xmlSchemaType = _xmlSchemaType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaType {
    pub type_0: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaType,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_0: *const xmlChar,
    pub refNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub subtypes: xmlSchemaTypePtr,
    pub attributes: xmlSchemaAttributePtr,
    pub node: xmlNodePtr,
    pub minOccurs: i32,
    pub maxOccurs: i32,
    pub flags: i32,
    pub contentType: xmlSchemaContentType,
    pub base: *const xmlChar,
    pub baseNs: *const xmlChar,
    pub baseType: xmlSchemaTypePtr,
    pub facets: xmlSchemaFacetPtr,
    pub redef: *mut _xmlSchemaType,
    pub recurse: i32,
    pub attributeUses: *mut xmlSchemaAttributeLinkPtr,
    pub attributeWildcard: xmlSchemaWildcardPtr,
    pub builtInType: i32,
    pub memberTypes: xmlSchemaTypeLinkPtr,
    pub facetSet: xmlSchemaFacetLinkPtr,
    pub refPrefix: *const xmlChar,
    pub contentTypeDef: xmlSchemaTypePtr,
    pub contModel: xmlRegexpPtr,
    pub targetNamespace: *const xmlChar,
    pub attrUses: *mut libc::c_void,
}
pub type xmlSchemaFacetLinkPtr = *mut xmlSchemaFacetLink;
pub type xmlSchemaFacetLink = _xmlSchemaFacetLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaFacetLink {
    pub next: *mut _xmlSchemaFacetLink,
    pub facet: xmlSchemaFacetPtr,
}
pub type xmlSchemaTypeLinkPtr = *mut xmlSchemaTypeLink;
pub type xmlSchemaTypeLink = _xmlSchemaTypeLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaTypeLink {
    pub next: *mut _xmlSchemaTypeLink,
    pub type_0: xmlSchemaTypePtr,
}
pub type xmlSchemaWildcardPtr = *mut xmlSchemaWildcard;
pub type xmlSchemaWildcard = _xmlSchemaWildcard;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaWildcard {
    pub type_0: xmlSchemaTypeType,
    pub id: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub node: xmlNodePtr,
    pub minOccurs: i32,
    pub maxOccurs: i32,
    pub processContents: i32,
    pub any: i32,
    pub nsSet: xmlSchemaWildcardNsPtr,
    pub negNsSet: xmlSchemaWildcardNsPtr,
    pub flags: i32,
}
pub type xmlSchemaWildcardNsPtr = *mut xmlSchemaWildcardNs;
pub type xmlSchemaWildcardNs = _xmlSchemaWildcardNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaWildcardNs {
    pub next: *mut _xmlSchemaWildcardNs,
    pub value: *const xmlChar,
}
pub type xmlSchemaAttributeLinkPtr = *mut xmlSchemaAttributeLink;
pub type xmlSchemaAttributeLink = _xmlSchemaAttributeLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaAttributeLink {
    pub next: *mut _xmlSchemaAttributeLink,
    pub attr: *mut _xmlSchemaAttribute,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSchemaAttribute {
    pub type_0: xmlSchemaTypeType,
    pub next: *mut _xmlSchemaAttribute,
    pub name: *const xmlChar,
    pub id: *const xmlChar,
    pub ref_0: *const xmlChar,
    pub refNs: *const xmlChar,
    pub typeName: *const xmlChar,
    pub typeNs: *const xmlChar,
    pub annot: xmlSchemaAnnotPtr,
    pub base: xmlSchemaTypePtr,
    pub occurs: i32,
    pub defValue: *const xmlChar,
    pub subtypes: xmlSchemaTypePtr,
    pub node: xmlNodePtr,
    pub targetNamespace: *const xmlChar,
    pub flags: i32,
    pub refPrefix: *const xmlChar,
    pub defVal: xmlSchemaValPtr,
    pub refDecl: xmlSchemaAttributePtr,
}
pub type xmlSchemaAttributePtr = *mut xmlSchemaAttribute;
pub type xmlSchemaAttribute = _xmlSchemaAttribute;
pub type xmlSchemaContentType = u32;
pub const XML_SCHEMA_CONTENT_ANY: xmlSchemaContentType = 7;
pub const XML_SCHEMA_CONTENT_BASIC: xmlSchemaContentType = 6;
pub const XML_SCHEMA_CONTENT_MIXED_OR_ELEMENTS: xmlSchemaContentType = 5;
pub const XML_SCHEMA_CONTENT_SIMPLE: xmlSchemaContentType = 4;
pub const XML_SCHEMA_CONTENT_MIXED: xmlSchemaContentType = 3;
pub const XML_SCHEMA_CONTENT_ELEMENTS: xmlSchemaContentType = 2;
pub const XML_SCHEMA_CONTENT_EMPTY: xmlSchemaContentType = 1;
pub const XML_SCHEMA_CONTENT_UNKNOWN: xmlSchemaContentType = 0;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlRelaxNGPartitionPtr = *mut xmlRelaxNGPartition;
pub type xmlRelaxNGPartition = _xmlRelaxNGPartition;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGPartition {
    pub nbgroups: i32,
    pub triage: xmlHashTablePtr,
    pub flags: i32,
    pub groups: *mut xmlRelaxNGInterleaveGroupPtr,
}
pub type xmlRelaxNGInterleaveGroupPtr = *mut xmlRelaxNGInterleaveGroup;
pub type xmlRelaxNGInterleaveGroup = _xmlRelaxNGInterleaveGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRelaxNGInterleaveGroup {
    pub rule: xmlRelaxNGDefinePtr,
    pub defs: *mut xmlRelaxNGDefinePtr,
    pub attrs: *mut xmlRelaxNGDefinePtr,
}
pub type xmlRelaxNGContentType = i32;
pub const XML_RELAXNG_CONTENT_COMPLEX: xmlRelaxNGContentType = 2;
pub const XML_RELAXNG_CONTENT_SIMPLE: xmlRelaxNGContentType = 1;
pub const XML_RELAXNG_CONTENT_EMPTY: xmlRelaxNGContentType = 0;
pub const XML_RELAXNG_CONTENT_ERROR: xmlRelaxNGContentType = -1;
static mut xmlRelaxNGNs: *const xmlChar = b"http://relaxng.org/ns/structure/1.0\0"
    as *const u8 as *const i8 as *const xmlChar;
unsafe extern "C" fn xmlRngPErrMemory(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut extra: *const i8,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if !ctxt.is_null() {
        if ((*ctxt).serror).is_some() {
            schannel = (*ctxt).serror;
        } else {
            channel = (*ctxt).error;
        }
        data = (*ctxt).userData;
        let fresh0 = &mut ((*ctxt).nbErrors);
        *fresh0 += 1;
    }
    if !extra.is_null() {
        __xmlRaiseError(
            schannel,
            channel,
            data,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_RELAXNGP as i32,
            XML_ERR_NO_MEMORY as i32,
            XML_ERR_FATAL,
            0 as *const i8,
            0 as i32,
            extra,
            0 as *const i8,
            0 as *const i8,
            0 as i32,
            0 as i32,
            b"Memory allocation failed : %s\n\0" as *const u8 as *const i8,
            extra,
        );
    } else {
        __xmlRaiseError(
            schannel,
            channel,
            data,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_RELAXNGP as i32,
            XML_ERR_NO_MEMORY as i32,
            XML_ERR_FATAL,
            0 as *const i8,
            0 as i32,
            0 as *const i8,
            0 as *const i8,
            0 as *const i8,
            0 as i32,
            0 as i32,
            b"Memory allocation failed\n\0" as *const u8 as *const i8,
        );
    };
}
unsafe extern "C" fn xmlRngVErrMemory(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut extra: *const i8,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if !ctxt.is_null() {
        if ((*ctxt).serror).is_some() {
            schannel = (*ctxt).serror;
        } else {
            channel = (*ctxt).error;
        }
        data = (*ctxt).userData;
        let fresh1 = &mut ((*ctxt).nbErrors);
        *fresh1 += 1;
    }
    if !extra.is_null() {
        __xmlRaiseError(
            schannel,
            channel,
            data,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_RELAXNGV as i32,
            XML_ERR_NO_MEMORY as i32,
            XML_ERR_FATAL,
            0 as *const i8,
            0 as i32,
            extra,
            0 as *const i8,
            0 as *const i8,
            0 as i32,
            0 as i32,
            b"Memory allocation failed : %s\n\0" as *const u8 as *const i8,
            extra,
        );
    } else {
        __xmlRaiseError(
            schannel,
            channel,
            data,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_RELAXNGV as i32,
            XML_ERR_NO_MEMORY as i32,
            XML_ERR_FATAL,
            0 as *const i8,
            0 as i32,
            0 as *const i8,
            0 as *const i8,
            0 as *const i8,
            0 as i32,
            0 as i32,
            b"Memory allocation failed\n\0" as *const u8 as *const i8,
        );
    };
}
unsafe extern "C" fn xmlRngPErr(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
    mut error: i32,
    mut msg: *const i8,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if !ctxt.is_null() {
        if ((*ctxt).serror).is_some() {
            schannel = (*ctxt).serror;
        } else {
            channel = (*ctxt).error;
        }
        data = (*ctxt).userData;
        let fresh2 = &mut ((*ctxt).nbErrors);
        *fresh2 += 1;
    }
    __xmlRaiseError(
        schannel,
        channel,
        data,
        0 as *mut libc::c_void,
        node as *mut libc::c_void,
        XML_FROM_RELAXNGP as i32,
        error,
        XML_ERR_ERROR,
        0 as *const i8,
        0 as i32,
        str1 as *const i8,
        str2 as *const i8,
        0 as *const i8,
        0 as i32,
        0 as i32,
        msg,
        str1,
        str2,
    );
}
unsafe extern "C" fn xmlRngVErr(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut node: xmlNodePtr,
    mut error: i32,
    mut msg: *const i8,
    mut str1: *const xmlChar,
    mut str2: *const xmlChar,
) {
    let mut schannel: xmlStructuredErrorFunc = None;
    let mut channel: xmlGenericErrorFunc = None;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if !ctxt.is_null() {
        if ((*ctxt).serror).is_some() {
            schannel = (*ctxt).serror;
        } else {
            channel = (*ctxt).error;
        }
        data = (*ctxt).userData;
        let fresh3 = &mut ((*ctxt).nbErrors);
        *fresh3 += 1;
    }
    __xmlRaiseError(
        schannel,
        channel,
        data,
        0 as *mut libc::c_void,
        node as *mut libc::c_void,
        XML_FROM_RELAXNGV as i32,
        error,
        XML_ERR_ERROR,
        0 as *const i8,
        0 as i32,
        str1 as *const i8,
        str2 as *const i8,
        0 as *const i8,
        0 as i32,
        0 as i32,
        msg,
        str1,
        str2,
    );
}
unsafe extern "C" fn xmlRelaxNGFreeDocument(mut docu: xmlRelaxNGDocumentPtr) {
    if docu.is_null() {
        return;
    }
    if !((*docu).href).is_null() {
        xmlFree.expect("non-null function pointer")((*docu).href as *mut libc::c_void);
    }
    if !((*docu).doc).is_null() {
        xmlFreeDoc((*docu).doc);
    }
    if !((*docu).schema).is_null() {
        xmlRelaxNGFreeInnerSchema((*docu).schema);
    }
    xmlFree.expect("non-null function pointer")(docu as *mut libc::c_void);
}
unsafe extern "C" fn xmlRelaxNGFreeDocumentList(mut docu: xmlRelaxNGDocumentPtr) {
    let mut next: xmlRelaxNGDocumentPtr = 0 as *mut xmlRelaxNGDocument;
    while !docu.is_null() {
        next = (*docu).next;
        xmlRelaxNGFreeDocument(docu);
        docu = next;
    }
}
unsafe extern "C" fn xmlRelaxNGFreeInclude(mut incl: xmlRelaxNGIncludePtr) {
    if incl.is_null() {
        return;
    }
    if !((*incl).href).is_null() {
        xmlFree.expect("non-null function pointer")((*incl).href as *mut libc::c_void);
    }
    if !((*incl).doc).is_null() {
        xmlFreeDoc((*incl).doc);
    }
    if !((*incl).schema).is_null() {
        xmlRelaxNGFree((*incl).schema);
    }
    xmlFree.expect("non-null function pointer")(incl as *mut libc::c_void);
}
unsafe extern "C" fn xmlRelaxNGFreeIncludeList(mut incl: xmlRelaxNGIncludePtr) {
    let mut next: xmlRelaxNGIncludePtr = 0 as *mut xmlRelaxNGInclude;
    while !incl.is_null() {
        next = (*incl).next;
        xmlRelaxNGFreeInclude(incl);
        incl = next;
    }
}
unsafe extern "C" fn xmlRelaxNGNewRelaxNG(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
) -> xmlRelaxNGPtr {
    let mut ret: xmlRelaxNGPtr = 0 as *mut xmlRelaxNG;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNG>() as u64) as xmlRelaxNGPtr;
    if ret.is_null() {
        xmlRngPErrMemory(ctxt, 0 as *const i8);
        return 0 as xmlRelaxNGPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNG>() as u64,
    );
    return ret;
}
unsafe extern "C" fn xmlRelaxNGFreeInnerSchema(mut schema: xmlRelaxNGPtr) {
    if schema.is_null() {
        return;
    }
    if !((*schema).doc).is_null() {
        xmlFreeDoc((*schema).doc);
    }
    if !((*schema).defTab).is_null() {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < (*schema).defNr {
            xmlRelaxNGFreeDefine(*((*schema).defTab).offset(i as isize));
            i += 1;
        }
        xmlFree
            .expect("non-null function pointer")((*schema).defTab as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(schema as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGFree(mut schema: xmlRelaxNGPtr) {
    if schema.is_null() {
        return;
    }
    if !((*schema).topgrammar).is_null() {
        xmlRelaxNGFreeGrammar((*schema).topgrammar);
    }
    if !((*schema).doc).is_null() {
        xmlFreeDoc((*schema).doc);
    }
    if !((*schema).documents).is_null() {
        xmlRelaxNGFreeDocumentList((*schema).documents);
    }
    if !((*schema).includes).is_null() {
        xmlRelaxNGFreeIncludeList((*schema).includes);
    }
    if !((*schema).defTab).is_null() {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < (*schema).defNr {
            xmlRelaxNGFreeDefine(*((*schema).defTab).offset(i as isize));
            i += 1;
        }
        xmlFree
            .expect("non-null function pointer")((*schema).defTab as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(schema as *mut libc::c_void);
}
unsafe extern "C" fn xmlRelaxNGNewGrammar(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
) -> xmlRelaxNGGrammarPtr {
    let mut ret: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNGGrammar>() as u64)
        as xmlRelaxNGGrammarPtr;
    if ret.is_null() {
        xmlRngPErrMemory(ctxt, 0 as *const i8);
        return 0 as xmlRelaxNGGrammarPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGGrammar>() as u64,
    );
    return ret;
}
unsafe extern "C" fn xmlRelaxNGFreeGrammar(mut grammar: xmlRelaxNGGrammarPtr) {
    if grammar.is_null() {
        return;
    }
    if !((*grammar).children).is_null() {
        xmlRelaxNGFreeGrammar((*grammar).children);
    }
    if !((*grammar).next).is_null() {
        xmlRelaxNGFreeGrammar((*grammar).next);
    }
    if !((*grammar).refs).is_null() {
        xmlHashFree((*grammar).refs, None);
    }
    if !((*grammar).defs).is_null() {
        xmlHashFree((*grammar).defs, None);
    }
    xmlFree.expect("non-null function pointer")(grammar as *mut libc::c_void);
}
unsafe extern "C" fn xmlRelaxNGNewDefine(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if (*ctxt).defMax == 0 as i32 {
        (*ctxt).defMax = 16 as i32;
        (*ctxt).defNr = 0 as i32;
        let fresh4 = &mut ((*ctxt).defTab);
        *fresh4 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).defMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGDefinePtr>() as u64,
                ),
        ) as *mut xmlRelaxNGDefinePtr;
        if ((*ctxt).defTab).is_null() {
            xmlRngPErrMemory(
                ctxt,
                b"allocating define\n\0" as *const u8 as *const i8,
            );
            return 0 as xmlRelaxNGDefinePtr;
        }
    } else if (*ctxt).defMax <= (*ctxt).defNr {
        let mut tmp: *mut xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefinePtr;
        (*ctxt).defMax *= 2 as i32;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).defTab as *mut libc::c_void,
            ((*ctxt).defMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGDefinePtr>() as u64,
                ),
        ) as *mut xmlRelaxNGDefinePtr;
        if tmp.is_null() {
            xmlRngPErrMemory(
                ctxt,
                b"allocating define\n\0" as *const u8 as *const i8,
            );
            return 0 as xmlRelaxNGDefinePtr;
        }
        let fresh5 = &mut ((*ctxt).defTab);
        *fresh5 = tmp;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNGDefine>() as u64)
        as xmlRelaxNGDefinePtr;
    if ret.is_null() {
        xmlRngPErrMemory(
            ctxt,
            b"allocating define\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlRelaxNGDefinePtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGDefine>() as u64,
    );
    let fresh6 = &mut ((*ctxt).defNr);
    let fresh7 = *fresh6;
    *fresh6 = *fresh6 + 1;
    let fresh8 = &mut (*((*ctxt).defTab).offset(fresh7 as isize));
    *fresh8 = ret;
    let fresh9 = &mut ((*ret).node);
    *fresh9 = node;
    (*ret).depth = -(1 as i32) as i16;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGFreePartition(mut partitions: xmlRelaxNGPartitionPtr) {
    let mut group: xmlRelaxNGInterleaveGroupPtr = 0 as *mut xmlRelaxNGInterleaveGroup;
    let mut j: i32 = 0;
    if !partitions.is_null() {
        if !((*partitions).groups).is_null() {
            j = 0 as i32;
            while j < (*partitions).nbgroups {
                group = *((*partitions).groups).offset(j as isize);
                if !group.is_null() {
                    if !((*group).defs).is_null() {
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )((*group).defs as *mut libc::c_void);
                    }
                    if !((*group).attrs).is_null() {
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )((*group).attrs as *mut libc::c_void);
                    }
                    xmlFree
                        .expect("non-null function pointer")(group as *mut libc::c_void);
                }
                j += 1;
            }
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*partitions).groups as *mut libc::c_void);
        }
        if !((*partitions).triage).is_null() {
            xmlHashFree((*partitions).triage, None);
        }
        xmlFree.expect("non-null function pointer")(partitions as *mut libc::c_void);
    }
}
unsafe extern "C" fn xmlRelaxNGFreeDefine(mut define: xmlRelaxNGDefinePtr) {
    if define.is_null() {
        return;
    }
    if (*define).type_0 as i32 == XML_RELAXNG_VALUE as i32
        && !((*define).attrs).is_null()
    {
        let mut lib: xmlRelaxNGTypeLibraryPtr = 0 as *mut xmlRelaxNGTypeLibrary;
        lib = (*define).data as xmlRelaxNGTypeLibraryPtr;
        if !lib.is_null() && ((*lib).freef).is_some() {
            ((*lib).freef)
                .expect(
                    "non-null function pointer",
                )((*lib).data, (*define).attrs as *mut libc::c_void);
        }
    }
    if !((*define).data).is_null()
        && (*define).type_0 as i32 == XML_RELAXNG_INTERLEAVE as i32
    {
        xmlRelaxNGFreePartition((*define).data as xmlRelaxNGPartitionPtr);
    }
    if !((*define).data).is_null()
        && (*define).type_0 as i32 == XML_RELAXNG_CHOICE as i32
    {
        xmlHashFree((*define).data as xmlHashTablePtr, None);
    }
    if !((*define).name).is_null() {
        xmlFree.expect("non-null function pointer")((*define).name as *mut libc::c_void);
    }
    if !((*define).ns).is_null() {
        xmlFree.expect("non-null function pointer")((*define).ns as *mut libc::c_void);
    }
    if !((*define).value).is_null() {
        xmlFree
            .expect("non-null function pointer")((*define).value as *mut libc::c_void);
    }
    if !((*define).contModel).is_null() {
        xmlRegFreeRegexp((*define).contModel);
    }
    xmlFree.expect("non-null function pointer")(define as *mut libc::c_void);
}
unsafe extern "C" fn xmlRelaxNGNewStates(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut size: i32,
) -> xmlRelaxNGStatesPtr {
    let mut ret: xmlRelaxNGStatesPtr = 0 as *mut xmlRelaxNGStates;
    if !ctxt.is_null() && !((*ctxt).freeStates).is_null()
        && (*ctxt).freeStatesNr > 0 as i32
    {
        let fresh10 = &mut ((*ctxt).freeStatesNr);
        *fresh10 -= 1;
        ret = *((*ctxt).freeStates).offset((*ctxt).freeStatesNr as isize);
        (*ret).nbState = 0 as i32;
        return ret;
    }
    if size < 16 as i32 {
        size = 16 as i32;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<xmlRelaxNGStates>() as u64)
            .wrapping_add(
                ((size - 1 as i32) as u64)
                    .wrapping_mul(
                        ::std::mem::size_of::<xmlRelaxNGValidStatePtr>() as u64,
                    ),
            ),
    ) as xmlRelaxNGStatesPtr;
    if ret.is_null() {
        xmlRngVErrMemory(
            ctxt,
            b"allocating states\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlRelaxNGStatesPtr;
    }
    (*ret).nbState = 0 as i32;
    (*ret).maxState = size;
    let fresh11 = &mut ((*ret).tabState);
    *fresh11 = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (size as u64)
            .wrapping_mul(
                ::std::mem::size_of::<xmlRelaxNGValidStatePtr>() as u64,
            ),
    ) as *mut xmlRelaxNGValidStatePtr;
    if ((*ret).tabState).is_null() {
        xmlRngVErrMemory(
            ctxt,
            b"allocating states\n\0" as *const u8 as *const i8,
        );
        xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
        return 0 as xmlRelaxNGStatesPtr;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGAddStatesUniq(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut states: xmlRelaxNGStatesPtr,
    mut state: xmlRelaxNGValidStatePtr,
) -> i32 {
    if state.is_null() {
        return -(1 as i32);
    }
    if (*states).nbState >= (*states).maxState {
        let mut tmp: *mut xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidStatePtr;
        let mut size: i32 = 0;
        size = (*states).maxState * 2 as i32;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*states).tabState as *mut libc::c_void,
            (size as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGValidStatePtr>() as u64,
                ),
        ) as *mut xmlRelaxNGValidStatePtr;
        if tmp.is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"adding states\n\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        let fresh12 = &mut ((*states).tabState);
        *fresh12 = tmp;
        (*states).maxState = size;
    }
    let fresh13 = &mut ((*states).nbState);
    let fresh14 = *fresh13;
    *fresh13 = *fresh13 + 1;
    let fresh15 = &mut (*((*states).tabState).offset(fresh14 as isize));
    *fresh15 = state;
    return 1 as i32;
}
unsafe extern "C" fn xmlRelaxNGAddStates(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut states: xmlRelaxNGStatesPtr,
    mut state: xmlRelaxNGValidStatePtr,
) -> i32 {
    let mut i: i32 = 0;
    if state.is_null() || states.is_null() {
        return -(1 as i32);
    }
    if (*states).nbState >= (*states).maxState {
        let mut tmp: *mut xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidStatePtr;
        let mut size: i32 = 0;
        size = (*states).maxState * 2 as i32;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*states).tabState as *mut libc::c_void,
            (size as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGValidStatePtr>() as u64,
                ),
        ) as *mut xmlRelaxNGValidStatePtr;
        if tmp.is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"adding states\n\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        let fresh16 = &mut ((*states).tabState);
        *fresh16 = tmp;
        (*states).maxState = size;
    }
    i = 0 as i32;
    while i < (*states).nbState {
        if xmlRelaxNGEqualValidState(
            ctxt,
            state,
            *((*states).tabState).offset(i as isize),
        ) != 0
        {
            xmlRelaxNGFreeValidState(ctxt, state);
            return 0 as i32;
        }
        i += 1;
    }
    let fresh17 = &mut ((*states).nbState);
    let fresh18 = *fresh17;
    *fresh17 = *fresh17 + 1;
    let fresh19 = &mut (*((*states).tabState).offset(fresh18 as isize));
    *fresh19 = state;
    return 1 as i32;
}
unsafe extern "C" fn xmlRelaxNGFreeStates(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut states: xmlRelaxNGStatesPtr,
) {
    if states.is_null() {
        return;
    }
    if !ctxt.is_null() && ((*ctxt).freeStates).is_null() {
        (*ctxt).freeStatesMax = 40 as i32;
        (*ctxt).freeStatesNr = 0 as i32;
        let fresh20 = &mut ((*ctxt).freeStates);
        *fresh20 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).freeStatesMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGStatesPtr>() as u64,
                ),
        ) as *mut xmlRelaxNGStatesPtr;
        if ((*ctxt).freeStates).is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"storing states\n\0" as *const u8 as *const i8,
            );
        }
    } else if !ctxt.is_null() && (*ctxt).freeStatesNr >= (*ctxt).freeStatesMax {
        let mut tmp: *mut xmlRelaxNGStatesPtr = 0 as *mut xmlRelaxNGStatesPtr;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).freeStates as *mut libc::c_void,
            ((2 as i32 * (*ctxt).freeStatesMax) as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGStatesPtr>() as u64,
                ),
        ) as *mut xmlRelaxNGStatesPtr;
        if tmp.is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"storing states\n\0" as *const u8 as *const i8,
            );
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*states).tabState as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(states as *mut libc::c_void);
            return;
        }
        let fresh21 = &mut ((*ctxt).freeStates);
        *fresh21 = tmp;
        (*ctxt).freeStatesMax *= 2 as i32;
    }
    if ctxt.is_null() || ((*ctxt).freeStates).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*states).tabState as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(states as *mut libc::c_void);
    } else {
        let fresh22 = &mut ((*ctxt).freeStatesNr);
        let fresh23 = *fresh22;
        *fresh22 = *fresh22 + 1;
        let fresh24 = &mut (*((*ctxt).freeStates).offset(fresh23 as isize));
        *fresh24 = states;
    };
}
unsafe extern "C" fn xmlRelaxNGNewValidState(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGValidStatePtr {
    let mut ret: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut attrs: [xmlAttrPtr; 20] = [0 as *mut xmlAttr; 20];
    let mut nbAttrs: i32 = 0 as i32;
    let mut root: xmlNodePtr = 0 as xmlNodePtr;
    if node.is_null() {
        root = xmlDocGetRootElement((*ctxt).doc as *const xmlDoc);
        if root.is_null() {
            return 0 as xmlRelaxNGValidStatePtr;
        }
    } else {
        attr = (*node).properties;
        while !attr.is_null() {
            if nbAttrs < 20 as i32 {
                let fresh25 = nbAttrs;
                nbAttrs = nbAttrs + 1;
                attrs[fresh25 as usize] = attr;
            } else {
                nbAttrs += 1;
            }
            attr = (*attr).next;
        }
    }
    if !((*ctxt).freeState).is_null() && (*(*ctxt).freeState).nbState > 0 as i32
    {
        let fresh26 = &mut ((*(*ctxt).freeState).nbState);
        *fresh26 -= 1;
        ret = *((*(*ctxt).freeState).tabState)
            .offset((*(*ctxt).freeState).nbState as isize);
    } else {
        ret = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::std::mem::size_of::<xmlRelaxNGValidState>() as u64)
            as xmlRelaxNGValidStatePtr;
        if ret.is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"allocating states\n\0" as *const u8 as *const i8,
            );
            return 0 as xmlRelaxNGValidStatePtr;
        }
        memset(
            ret as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<xmlRelaxNGValidState>() as u64,
        );
    }
    let fresh27 = &mut ((*ret).value);
    *fresh27 = 0 as *mut xmlChar;
    let fresh28 = &mut ((*ret).endvalue);
    *fresh28 = 0 as *mut xmlChar;
    if node.is_null() {
        let fresh29 = &mut ((*ret).node);
        *fresh29 = (*ctxt).doc as xmlNodePtr;
        let fresh30 = &mut ((*ret).seq);
        *fresh30 = root;
    } else {
        let fresh31 = &mut ((*ret).node);
        *fresh31 = node;
        let fresh32 = &mut ((*ret).seq);
        *fresh32 = (*node).children;
    }
    (*ret).nbAttrs = 0 as i32;
    if nbAttrs > 0 as i32 {
        if ((*ret).attrs).is_null() {
            if nbAttrs < 4 as i32 {
                (*ret).maxAttrs = 4 as i32;
            } else {
                (*ret).maxAttrs = nbAttrs;
            }
            let fresh33 = &mut ((*ret).attrs);
            *fresh33 = xmlMalloc
                .expect(
                    "non-null function pointer",
                )(
                ((*ret).maxAttrs as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlAttrPtr>() as u64),
            ) as *mut xmlAttrPtr;
            if ((*ret).attrs).is_null() {
                xmlRngVErrMemory(
                    ctxt,
                    b"allocating states\n\0" as *const u8 as *const i8,
                );
                return ret;
            }
        } else if (*ret).maxAttrs < nbAttrs {
            let mut tmp: *mut xmlAttrPtr = 0 as *mut xmlAttrPtr;
            tmp = xmlRealloc
                .expect(
                    "non-null function pointer",
                )(
                (*ret).attrs as *mut libc::c_void,
                (nbAttrs as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlAttrPtr>() as u64),
            ) as *mut xmlAttrPtr;
            if tmp.is_null() {
                xmlRngVErrMemory(
                    ctxt,
                    b"allocating states\n\0" as *const u8 as *const i8,
                );
                return ret;
            }
            let fresh34 = &mut ((*ret).attrs);
            *fresh34 = tmp;
            (*ret).maxAttrs = nbAttrs;
        }
        (*ret).nbAttrs = nbAttrs;
        if nbAttrs < 20 as i32 {
            memcpy(
                (*ret).attrs as *mut libc::c_void,
                attrs.as_mut_ptr() as *const libc::c_void,
                (::std::mem::size_of::<xmlAttrPtr>() as u64)
                    .wrapping_mul(nbAttrs as u64),
            );
        } else {
            attr = (*node).properties;
            nbAttrs = 0 as i32;
            while !attr.is_null() {
                let fresh35 = nbAttrs;
                nbAttrs = nbAttrs + 1;
                let fresh36 = &mut (*((*ret).attrs).offset(fresh35 as isize));
                *fresh36 = attr;
                attr = (*attr).next;
            }
        }
    }
    (*ret).nbAttrLeft = (*ret).nbAttrs;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGCopyValidState(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut state: xmlRelaxNGValidStatePtr,
) -> xmlRelaxNGValidStatePtr {
    let mut ret: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut maxAttrs: u32 = 0;
    let mut attrs: *mut xmlAttrPtr = 0 as *mut xmlAttrPtr;
    if state.is_null() {
        return 0 as xmlRelaxNGValidStatePtr;
    }
    if !((*ctxt).freeState).is_null() && (*(*ctxt).freeState).nbState > 0 as i32
    {
        let fresh37 = &mut ((*(*ctxt).freeState).nbState);
        *fresh37 -= 1;
        ret = *((*(*ctxt).freeState).tabState)
            .offset((*(*ctxt).freeState).nbState as isize);
    } else {
        ret = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::std::mem::size_of::<xmlRelaxNGValidState>() as u64)
            as xmlRelaxNGValidStatePtr;
        if ret.is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"allocating states\n\0" as *const u8 as *const i8,
            );
            return 0 as xmlRelaxNGValidStatePtr;
        }
        memset(
            ret as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<xmlRelaxNGValidState>() as u64,
        );
    }
    attrs = (*ret).attrs;
    maxAttrs = (*ret).maxAttrs as u32;
    memcpy(
        ret as *mut libc::c_void,
        state as *const libc::c_void,
        ::std::mem::size_of::<xmlRelaxNGValidState>() as u64,
    );
    let fresh38 = &mut ((*ret).attrs);
    *fresh38 = attrs;
    (*ret).maxAttrs = maxAttrs as i32;
    if (*state).nbAttrs > 0 as i32 {
        if ((*ret).attrs).is_null() {
            (*ret).maxAttrs = (*state).maxAttrs;
            let fresh39 = &mut ((*ret).attrs);
            *fresh39 = xmlMalloc
                .expect(
                    "non-null function pointer",
                )(
                ((*ret).maxAttrs as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlAttrPtr>() as u64),
            ) as *mut xmlAttrPtr;
            if ((*ret).attrs).is_null() {
                xmlRngVErrMemory(
                    ctxt,
                    b"allocating states\n\0" as *const u8 as *const i8,
                );
                (*ret).nbAttrs = 0 as i32;
                return ret;
            }
        } else if (*ret).maxAttrs < (*state).nbAttrs {
            let mut tmp: *mut xmlAttrPtr = 0 as *mut xmlAttrPtr;
            tmp = xmlRealloc
                .expect(
                    "non-null function pointer",
                )(
                (*ret).attrs as *mut libc::c_void,
                ((*state).maxAttrs as u64)
                    .wrapping_mul(::std::mem::size_of::<xmlAttrPtr>() as u64),
            ) as *mut xmlAttrPtr;
            if tmp.is_null() {
                xmlRngVErrMemory(
                    ctxt,
                    b"allocating states\n\0" as *const u8 as *const i8,
                );
                (*ret).nbAttrs = 0 as i32;
                return ret;
            }
            (*ret).maxAttrs = (*state).maxAttrs;
            let fresh40 = &mut ((*ret).attrs);
            *fresh40 = tmp;
        }
        memcpy(
            (*ret).attrs as *mut libc::c_void,
            (*state).attrs as *const libc::c_void,
            ((*state).nbAttrs as u64)
                .wrapping_mul(::std::mem::size_of::<xmlAttrPtr>() as u64),
        );
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGEqualValidState(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut state1: xmlRelaxNGValidStatePtr,
    mut state2: xmlRelaxNGValidStatePtr,
) -> i32 {
    let mut i: i32 = 0;
    if state1.is_null() || state2.is_null() {
        return 0 as i32;
    }
    if state1 == state2 {
        return 1 as i32;
    }
    if (*state1).node != (*state2).node {
        return 0 as i32;
    }
    if (*state1).seq != (*state2).seq {
        return 0 as i32;
    }
    if (*state1).nbAttrLeft != (*state2).nbAttrLeft {
        return 0 as i32;
    }
    if (*state1).nbAttrs != (*state2).nbAttrs {
        return 0 as i32;
    }
    if (*state1).endvalue != (*state2).endvalue {
        return 0 as i32;
    }
    if (*state1).value != (*state2).value
        && xmlStrEqual((*state1).value, (*state2).value) == 0
    {
        return 0 as i32;
    }
    i = 0 as i32;
    while i < (*state1).nbAttrs {
        if *((*state1).attrs).offset(i as isize) != *((*state2).attrs).offset(i as isize)
        {
            return 0 as i32;
        }
        i += 1;
    }
    return 1 as i32;
}
unsafe extern "C" fn xmlRelaxNGFreeValidState(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut state: xmlRelaxNGValidStatePtr,
) {
    if state.is_null() {
        return;
    }
    if !ctxt.is_null() && ((*ctxt).freeState).is_null() {
        let fresh41 = &mut ((*ctxt).freeState);
        *fresh41 = xmlRelaxNGNewStates(ctxt, 40 as i32);
    }
    if ctxt.is_null() || ((*ctxt).freeState).is_null() {
        if !((*state).attrs).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*state).attrs as *mut libc::c_void);
        }
        xmlFree.expect("non-null function pointer")(state as *mut libc::c_void);
    } else {
        xmlRelaxNGAddStatesUniq(ctxt, (*ctxt).freeState, state);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxParserSetFlag(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut flags: i32,
) -> i32 {
    if ctxt.is_null() {
        return -(1 as i32);
    }
    if flags & XML_RELAXNGP_FREE_DOC as i32 != 0 {
        (*ctxt).crng |= XML_RELAXNGP_FREE_DOC as i32;
        flags -= XML_RELAXNGP_FREE_DOC as i32;
    }
    if flags & XML_RELAXNGP_CRNG as i32 != 0 {
        (*ctxt).crng |= XML_RELAXNGP_CRNG as i32;
        flags -= XML_RELAXNGP_CRNG as i32;
    }
    if flags != 0 as i32 {
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGIncludePush(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut value: xmlRelaxNGIncludePtr,
) -> i32 {
    if ((*ctxt).incTab).is_null() {
        (*ctxt).incMax = 4 as i32;
        (*ctxt).incNr = 0 as i32;
        let fresh42 = &mut ((*ctxt).incTab);
        *fresh42 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).incMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGIncludePtr>() as u64,
                ),
        ) as *mut xmlRelaxNGIncludePtr;
        if ((*ctxt).incTab).is_null() {
            xmlRngPErrMemory(
                ctxt,
                b"allocating include\n\0" as *const u8 as *const i8,
            );
            return 0 as i32;
        }
    }
    if (*ctxt).incNr >= (*ctxt).incMax {
        (*ctxt).incMax *= 2 as i32;
        let fresh43 = &mut ((*ctxt).incTab);
        *fresh43 = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).incTab as *mut libc::c_void,
            ((*ctxt).incMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGIncludePtr>() as u64,
                ),
        ) as *mut xmlRelaxNGIncludePtr;
        if ((*ctxt).incTab).is_null() {
            xmlRngPErrMemory(
                ctxt,
                b"allocating include\n\0" as *const u8 as *const i8,
            );
            return 0 as i32;
        }
    }
    let fresh44 = &mut (*((*ctxt).incTab).offset((*ctxt).incNr as isize));
    *fresh44 = value;
    let fresh45 = &mut ((*ctxt).inc);
    *fresh45 = value;
    let fresh46 = &mut ((*ctxt).incNr);
    let fresh47 = *fresh46;
    *fresh46 = *fresh46 + 1;
    return fresh47;
}
unsafe extern "C" fn xmlRelaxNGIncludePop(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
) -> xmlRelaxNGIncludePtr {
    let mut ret: xmlRelaxNGIncludePtr = 0 as *mut xmlRelaxNGInclude;
    if (*ctxt).incNr <= 0 as i32 {
        return 0 as xmlRelaxNGIncludePtr;
    }
    let fresh48 = &mut ((*ctxt).incNr);
    *fresh48 -= 1;
    if (*ctxt).incNr > 0 as i32 {
        let fresh49 = &mut ((*ctxt).inc);
        *fresh49 = *((*ctxt).incTab).offset(((*ctxt).incNr - 1 as i32) as isize);
    } else {
        let fresh50 = &mut ((*ctxt).inc);
        *fresh50 = 0 as xmlRelaxNGIncludePtr;
    }
    ret = *((*ctxt).incTab).offset((*ctxt).incNr as isize);
    let fresh51 = &mut (*((*ctxt).incTab).offset((*ctxt).incNr as isize));
    *fresh51 = 0 as xmlRelaxNGIncludePtr;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGRemoveRedefine(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut URL: *const xmlChar,
    mut target: xmlNodePtr,
    mut name: *const xmlChar,
) -> i32 {
    let mut found: i32 = 0 as i32;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp2: xmlNodePtr = 0 as *mut xmlNode;
    let mut name2: *mut xmlChar = 0 as *mut xmlChar;
    tmp = target;
    while !tmp.is_null() {
        tmp2 = (*tmp).next;
        if name.is_null()
            && (!tmp.is_null() && !((*tmp).ns).is_null()
                && (*tmp).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                && xmlStrEqual(
                    (*tmp).name,
                    b"start\0" as *const u8 as *const i8 as *const xmlChar,
                ) != 0 && xmlStrEqual((*(*tmp).ns).href, xmlRelaxNGNs) != 0)
        {
            found = 1 as i32;
            xmlUnlinkNode(tmp);
            xmlFreeNode(tmp);
        } else if !name.is_null()
                && (!tmp.is_null() && !((*tmp).ns).is_null()
                    && (*tmp).type_0 as u32
                        == XML_ELEMENT_NODE as i32 as u32
                    && xmlStrEqual(
                        (*tmp).name,
                        b"define\0" as *const u8 as *const i8 as *const xmlChar,
                    ) != 0 && xmlStrEqual((*(*tmp).ns).href, xmlRelaxNGNs) != 0)
            {
            name2 = xmlGetProp(
                tmp as *const xmlNode,
                b"name\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            xmlRelaxNGNormExtSpace(name2);
            if !name2.is_null() {
                if xmlStrEqual(name, name2) != 0 {
                    found = 1 as i32;
                    xmlUnlinkNode(tmp);
                    xmlFreeNode(tmp);
                }
                xmlFree.expect("non-null function pointer")(name2 as *mut libc::c_void);
            }
        } else if !tmp.is_null() && !((*tmp).ns).is_null()
                && (*tmp).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                && xmlStrEqual(
                    (*tmp).name,
                    b"include\0" as *const u8 as *const i8 as *const xmlChar,
                ) != 0 && xmlStrEqual((*(*tmp).ns).href, xmlRelaxNGNs) != 0
            {
            let mut href: *mut xmlChar = 0 as *mut xmlChar;
            let mut inc: xmlRelaxNGDocumentPtr = (*tmp).psvi as xmlRelaxNGDocumentPtr;
            if !inc.is_null() && !((*inc).doc).is_null()
                && !((*(*inc).doc).children).is_null()
            {
                if xmlStrEqual(
                    (*(*(*inc).doc).children).name,
                    b"grammar\0" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                    if xmlRelaxNGRemoveRedefine(
                        ctxt,
                        href,
                        (*xmlDocGetRootElement((*inc).doc as *const xmlDoc)).children,
                        name,
                    ) == 1 as i32
                    {
                        found = 1 as i32;
                    }
                }
            }
            if xmlRelaxNGRemoveRedefine(ctxt, URL, (*tmp).children, name)
                == 1 as i32
            {
                found = 1 as i32;
            }
        }
        tmp = tmp2;
    }
    return found;
}
unsafe extern "C" fn xmlRelaxNGLoadInclude(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut URL: *const xmlChar,
    mut node: xmlNodePtr,
    mut ns: *const xmlChar,
) -> xmlRelaxNGIncludePtr {
    let mut ret: xmlRelaxNGIncludePtr = 0 as xmlRelaxNGIncludePtr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut i: i32 = 0;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    i = 0 as i32;
    while i < (*ctxt).incNr {
        if xmlStrEqual((**((*ctxt).incTab).offset(i as isize)).href, URL) != 0 {
            xmlRngPErr(
                ctxt,
                0 as xmlNodePtr,
                XML_RNGP_INCLUDE_RECURSE as i32,
                b"Detected an Include recursion for %s\n\0" as *const u8
                    as *const i8,
                URL,
                0 as *const xmlChar,
            );
            return 0 as xmlRelaxNGIncludePtr;
        }
        i += 1;
    }
    doc = xmlReadFile(
        URL as *const i8,
        0 as *const i8,
        0 as i32,
    );
    if doc.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_PARSE_ERROR as i32,
            b"xmlRelaxNG: could not load %s\n\0" as *const u8 as *const i8,
            URL,
            0 as *const xmlChar,
        );
        return 0 as xmlRelaxNGIncludePtr;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNGInclude>() as u64)
        as xmlRelaxNGIncludePtr;
    if ret.is_null() {
        xmlRngPErrMemory(
            ctxt,
            b"allocating include\n\0" as *const u8 as *const i8,
        );
        xmlFreeDoc(doc);
        return 0 as xmlRelaxNGIncludePtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGInclude>() as u64,
    );
    let fresh52 = &mut ((*ret).doc);
    *fresh52 = doc;
    let fresh53 = &mut ((*ret).href);
    *fresh53 = xmlStrdup(URL);
    let fresh54 = &mut ((*ret).next);
    *fresh54 = (*ctxt).includes;
    let fresh55 = &mut ((*ctxt).includes);
    *fresh55 = ret;
    if !ns.is_null() {
        root = xmlDocGetRootElement(doc as *const xmlDoc);
        if !root.is_null() {
            if (xmlHasProp(
                root as *const xmlNode,
                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
            ))
                .is_null()
            {
                xmlSetProp(
                    root,
                    b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                    ns,
                );
            }
        }
    }
    xmlRelaxNGIncludePush(ctxt, ret);
    doc = xmlRelaxNGCleanupDoc(ctxt, doc);
    if doc.is_null() {
        let fresh56 = &mut ((*ctxt).inc);
        *fresh56 = 0 as xmlRelaxNGIncludePtr;
        return 0 as xmlRelaxNGIncludePtr;
    }
    xmlRelaxNGIncludePop(ctxt);
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_EMPTY as i32,
            b"xmlRelaxNG: included document is empty %s\n\0" as *const u8
                as *const i8,
            URL,
            0 as *const xmlChar,
        );
        return 0 as xmlRelaxNGIncludePtr;
    }
    if !(!root.is_null() && !((*root).ns).is_null()
        && (*root).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        && xmlStrEqual(
            (*root).name,
            b"grammar\0" as *const u8 as *const i8 as *const xmlChar,
        ) != 0 && xmlStrEqual((*(*root).ns).href, xmlRelaxNGNs) != 0)
    {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_GRAMMAR_MISSING as i32,
            b"xmlRelaxNG: included document %s root is not a grammar\n\0" as *const u8
                as *const i8,
            URL,
            0 as *const xmlChar,
        );
        return 0 as xmlRelaxNGIncludePtr;
    }
    cur = (*node).children;
    while !cur.is_null() {
        if !cur.is_null() && !((*cur).ns).is_null()
            && (*cur).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*cur).name,
                b"start\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*cur).ns).href, xmlRelaxNGNs) != 0
        {
            let mut found: i32 = 0 as i32;
            found = xmlRelaxNGRemoveRedefine(
                ctxt,
                URL,
                (*root).children,
                0 as *const xmlChar,
            );
            if found == 0 {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_START_MISSING as i32,
                    b"xmlRelaxNG: include %s has a start but not the included grammar\n\0"
                        as *const u8 as *const i8,
                    URL,
                    0 as *const xmlChar,
                );
            }
        } else if !cur.is_null() && !((*cur).ns).is_null()
                && (*cur).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                && xmlStrEqual(
                    (*cur).name,
                    b"define\0" as *const u8 as *const i8 as *const xmlChar,
                ) != 0 && xmlStrEqual((*(*cur).ns).href, xmlRelaxNGNs) != 0
            {
            let mut name: *mut xmlChar = 0 as *mut xmlChar;
            name = xmlGetProp(
                cur as *const xmlNode,
                b"name\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            if name.is_null() {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_NAME_MISSING as i32,
                    b"xmlRelaxNG: include %s has define without name\n\0" as *const u8
                        as *const i8,
                    URL,
                    0 as *const xmlChar,
                );
            } else {
                let mut found_0: i32 = 0;
                xmlRelaxNGNormExtSpace(name);
                found_0 = xmlRelaxNGRemoveRedefine(ctxt, URL, (*root).children, name);
                if found_0 == 0 {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_DEFINE_MISSING as i32,
                        b"xmlRelaxNG: include %s has a define %s but not the included grammar\n\0"
                            as *const u8 as *const i8,
                        URL,
                        name,
                    );
                }
                xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
            }
        }
        if !cur.is_null() && !((*cur).ns).is_null()
            && (*cur).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*cur).name,
                b"div\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*cur).ns).href, xmlRelaxNGNs) != 0
            && !((*cur).children).is_null()
        {
            cur = (*cur).children;
        } else if !((*cur).next).is_null() {
            cur = (*cur).next;
        } else {
            while (*cur).parent != node && ((*(*cur).parent).next).is_null() {
                cur = (*cur).parent;
            }
            cur = if (*cur).parent != node {
                (*(*cur).parent).next
            } else {
                0 as *mut _xmlNode
            };
        }
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGValidErrorPush(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut err: xmlRelaxNGValidErr,
    mut arg1: *const xmlChar,
    mut arg2: *const xmlChar,
    mut dup: i32,
) -> i32 {
    let mut cur: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    if ((*ctxt).errTab).is_null() {
        (*ctxt).errMax = 8 as i32;
        (*ctxt).errNr = 0 as i32;
        let fresh57 = &mut ((*ctxt).errTab);
        *fresh57 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).errMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGValidError>() as u64,
                ),
        ) as xmlRelaxNGValidErrorPtr;
        if ((*ctxt).errTab).is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"pushing error\n\0" as *const u8 as *const i8,
            );
            return 0 as i32;
        }
        let fresh58 = &mut ((*ctxt).err);
        *fresh58 = 0 as xmlRelaxNGValidErrorPtr;
    }
    if (*ctxt).errNr >= (*ctxt).errMax {
        (*ctxt).errMax *= 2 as i32;
        let fresh59 = &mut ((*ctxt).errTab);
        *fresh59 = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).errTab as *mut libc::c_void,
            ((*ctxt).errMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGValidError>() as u64,
                ),
        ) as xmlRelaxNGValidErrorPtr;
        if ((*ctxt).errTab).is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"pushing error\n\0" as *const u8 as *const i8,
            );
            return 0 as i32;
        }
        let fresh60 = &mut ((*ctxt).err);
        *fresh60 = &mut *((*ctxt).errTab)
            .offset(((*ctxt).errNr - 1 as i32) as isize)
            as *mut xmlRelaxNGValidError;
    }
    if !((*ctxt).err).is_null() && !((*ctxt).state).is_null()
        && (*(*ctxt).err).node == (*(*ctxt).state).node
        && (*(*ctxt).err).err as u32 == err as u32
    {
        return (*ctxt).errNr;
    }
    cur = &mut *((*ctxt).errTab).offset((*ctxt).errNr as isize)
        as *mut xmlRelaxNGValidError;
    (*cur).err = err;
    if dup != 0 {
        let fresh61 = &mut ((*cur).arg1);
        *fresh61 = xmlStrdup(arg1);
        let fresh62 = &mut ((*cur).arg2);
        *fresh62 = xmlStrdup(arg2);
        (*cur).flags = 1 as i32;
    } else {
        let fresh63 = &mut ((*cur).arg1);
        *fresh63 = arg1;
        let fresh64 = &mut ((*cur).arg2);
        *fresh64 = arg2;
        (*cur).flags = 0 as i32;
    }
    if !((*ctxt).state).is_null() {
        let fresh65 = &mut ((*cur).node);
        *fresh65 = (*(*ctxt).state).node;
        let fresh66 = &mut ((*cur).seq);
        *fresh66 = (*(*ctxt).state).seq;
    } else {
        let fresh67 = &mut ((*cur).node);
        *fresh67 = 0 as xmlNodePtr;
        let fresh68 = &mut ((*cur).seq);
        *fresh68 = 0 as xmlNodePtr;
    }
    let fresh69 = &mut ((*ctxt).err);
    *fresh69 = cur;
    let fresh70 = &mut ((*ctxt).errNr);
    let fresh71 = *fresh70;
    *fresh70 = *fresh70 + 1;
    return fresh71;
}
unsafe extern "C" fn xmlRelaxNGValidErrorPop(mut ctxt: xmlRelaxNGValidCtxtPtr) {
    let mut cur: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    if (*ctxt).errNr <= 0 as i32 {
        let fresh72 = &mut ((*ctxt).err);
        *fresh72 = 0 as xmlRelaxNGValidErrorPtr;
        return;
    }
    let fresh73 = &mut ((*ctxt).errNr);
    *fresh73 -= 1;
    if (*ctxt).errNr > 0 as i32 {
        let fresh74 = &mut ((*ctxt).err);
        *fresh74 = &mut *((*ctxt).errTab)
            .offset(((*ctxt).errNr - 1 as i32) as isize)
            as *mut xmlRelaxNGValidError;
    } else {
        let fresh75 = &mut ((*ctxt).err);
        *fresh75 = 0 as xmlRelaxNGValidErrorPtr;
    }
    cur = &mut *((*ctxt).errTab).offset((*ctxt).errNr as isize)
        as *mut xmlRelaxNGValidError;
    if (*cur).flags & 1 as i32 != 0 {
        if !((*cur).arg1).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*cur).arg1 as *mut xmlChar as *mut libc::c_void);
        }
        let fresh76 = &mut ((*cur).arg1);
        *fresh76 = 0 as *const xmlChar;
        if !((*cur).arg2).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*cur).arg2 as *mut xmlChar as *mut libc::c_void);
        }
        let fresh77 = &mut ((*cur).arg2);
        *fresh77 = 0 as *const xmlChar;
        (*cur).flags = 0 as i32;
    }
}
unsafe extern "C" fn xmlRelaxNGDocumentPush(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut value: xmlRelaxNGDocumentPtr,
) -> i32 {
    if ((*ctxt).docTab).is_null() {
        (*ctxt).docMax = 4 as i32;
        (*ctxt).docNr = 0 as i32;
        let fresh78 = &mut ((*ctxt).docTab);
        *fresh78 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).docMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGDocumentPtr>() as u64,
                ),
        ) as *mut xmlRelaxNGDocumentPtr;
        if ((*ctxt).docTab).is_null() {
            xmlRngPErrMemory(
                ctxt,
                b"adding document\n\0" as *const u8 as *const i8,
            );
            return 0 as i32;
        }
    }
    if (*ctxt).docNr >= (*ctxt).docMax {
        (*ctxt).docMax *= 2 as i32;
        let fresh79 = &mut ((*ctxt).docTab);
        *fresh79 = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).docTab as *mut libc::c_void,
            ((*ctxt).docMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRelaxNGDocumentPtr>() as u64,
                ),
        ) as *mut xmlRelaxNGDocumentPtr;
        if ((*ctxt).docTab).is_null() {
            xmlRngPErrMemory(
                ctxt,
                b"adding document\n\0" as *const u8 as *const i8,
            );
            return 0 as i32;
        }
    }
    let fresh80 = &mut (*((*ctxt).docTab).offset((*ctxt).docNr as isize));
    *fresh80 = value;
    let fresh81 = &mut ((*ctxt).doc);
    *fresh81 = value;
    let fresh82 = &mut ((*ctxt).docNr);
    let fresh83 = *fresh82;
    *fresh82 = *fresh82 + 1;
    return fresh83;
}
unsafe extern "C" fn xmlRelaxNGDocumentPop(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
) -> xmlRelaxNGDocumentPtr {
    let mut ret: xmlRelaxNGDocumentPtr = 0 as *mut xmlRelaxNGDocument;
    if (*ctxt).docNr <= 0 as i32 {
        return 0 as xmlRelaxNGDocumentPtr;
    }
    let fresh84 = &mut ((*ctxt).docNr);
    *fresh84 -= 1;
    if (*ctxt).docNr > 0 as i32 {
        let fresh85 = &mut ((*ctxt).doc);
        *fresh85 = *((*ctxt).docTab).offset(((*ctxt).docNr - 1 as i32) as isize);
    } else {
        let fresh86 = &mut ((*ctxt).doc);
        *fresh86 = 0 as xmlRelaxNGDocumentPtr;
    }
    ret = *((*ctxt).docTab).offset((*ctxt).docNr as isize);
    let fresh87 = &mut (*((*ctxt).docTab).offset((*ctxt).docNr as isize));
    *fresh87 = 0 as xmlRelaxNGDocumentPtr;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGLoadExternalRef(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut URL: *const xmlChar,
    mut ns: *const xmlChar,
) -> xmlRelaxNGDocumentPtr {
    let mut ret: xmlRelaxNGDocumentPtr = 0 as xmlRelaxNGDocumentPtr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (*ctxt).docNr {
        if xmlStrEqual((**((*ctxt).docTab).offset(i as isize)).href, URL) != 0 {
            xmlRngPErr(
                ctxt,
                0 as xmlNodePtr,
                XML_RNGP_EXTERNALREF_RECURSE as i32,
                b"Detected an externalRef recursion for %s\n\0" as *const u8
                    as *const i8,
                URL,
                0 as *const xmlChar,
            );
            return 0 as xmlRelaxNGDocumentPtr;
        }
        i += 1;
    }
    doc = xmlReadFile(
        URL as *const i8,
        0 as *const i8,
        0 as i32,
    );
    if doc.is_null() {
        xmlRngPErr(
            ctxt,
            0 as xmlNodePtr,
            XML_RNGP_PARSE_ERROR as i32,
            b"xmlRelaxNG: could not load %s\n\0" as *const u8 as *const i8,
            URL,
            0 as *const xmlChar,
        );
        return 0 as xmlRelaxNGDocumentPtr;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNGDocument>() as u64)
        as xmlRelaxNGDocumentPtr;
    if ret.is_null() {
        xmlRngPErr(
            ctxt,
            doc as xmlNodePtr,
            XML_ERR_NO_MEMORY as i32,
            b"xmlRelaxNG: allocate memory for doc %s\n\0" as *const u8
                as *const i8,
            URL,
            0 as *const xmlChar,
        );
        xmlFreeDoc(doc);
        return 0 as xmlRelaxNGDocumentPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGDocument>() as u64,
    );
    let fresh88 = &mut ((*ret).doc);
    *fresh88 = doc;
    let fresh89 = &mut ((*ret).href);
    *fresh89 = xmlStrdup(URL);
    let fresh90 = &mut ((*ret).next);
    *fresh90 = (*ctxt).documents;
    (*ret).externalRef = 1 as i32;
    let fresh91 = &mut ((*ctxt).documents);
    *fresh91 = ret;
    if !ns.is_null() {
        root = xmlDocGetRootElement(doc as *const xmlDoc);
        if !root.is_null() {
            if (xmlHasProp(
                root as *const xmlNode,
                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
            ))
                .is_null()
            {
                xmlSetProp(
                    root,
                    b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                    ns,
                );
            }
        }
    }
    xmlRelaxNGDocumentPush(ctxt, ret);
    doc = xmlRelaxNGCleanupDoc(ctxt, doc);
    if doc.is_null() {
        let fresh92 = &mut ((*ctxt).doc);
        *fresh92 = 0 as xmlRelaxNGDocumentPtr;
        return 0 as xmlRelaxNGDocumentPtr;
    }
    xmlRelaxNGDocumentPop(ctxt);
    return ret;
}
unsafe extern "C" fn xmlRelaxNGDefName(
    mut def: xmlRelaxNGDefinePtr,
) -> *const i8 {
    if def.is_null() {
        return b"none\0" as *const u8 as *const i8;
    }
    match (*def).type_0 as i32 {
        0 => return b"empty\0" as *const u8 as *const i8,
        1 => return b"notAllowed\0" as *const u8 as *const i8,
        2 => return b"except\0" as *const u8 as *const i8,
        3 => return b"text\0" as *const u8 as *const i8,
        4 => return b"element\0" as *const u8 as *const i8,
        5 => return b"datatype\0" as *const u8 as *const i8,
        7 => return b"value\0" as *const u8 as *const i8,
        8 => return b"list\0" as *const u8 as *const i8,
        9 => return b"attribute\0" as *const u8 as *const i8,
        10 => return b"def\0" as *const u8 as *const i8,
        11 => return b"ref\0" as *const u8 as *const i8,
        12 => return b"externalRef\0" as *const u8 as *const i8,
        13 => return b"parentRef\0" as *const u8 as *const i8,
        14 => return b"optional\0" as *const u8 as *const i8,
        15 => return b"zeroOrMore\0" as *const u8 as *const i8,
        16 => return b"oneOrMore\0" as *const u8 as *const i8,
        17 => return b"choice\0" as *const u8 as *const i8,
        18 => return b"group\0" as *const u8 as *const i8,
        19 => return b"interleave\0" as *const u8 as *const i8,
        20 => return b"start\0" as *const u8 as *const i8,
        -1 => return b"noop\0" as *const u8 as *const i8,
        6 => return b"param\0" as *const u8 as *const i8,
        _ => {}
    }
    return b"unknown\0" as *const u8 as *const i8;
}
unsafe extern "C" fn xmlRelaxNGGetErrorString(
    mut err: xmlRelaxNGValidErr,
    mut arg1: *const xmlChar,
    mut arg2: *const xmlChar,
) -> *mut xmlChar {
    let mut msg: [i8; 1000] = [0; 1000];
    let mut result: *mut xmlChar = 0 as *mut xmlChar;
    if arg1.is_null() {
        arg1 = b"\0" as *const u8 as *const i8 as *mut xmlChar;
    }
    if arg2.is_null() {
        arg2 = b"\0" as *const u8 as *const i8 as *mut xmlChar;
    }
    msg[0 as i32 as usize] = 0 as i32 as i8;
    match err as u32 {
        0 => return 0 as *mut xmlChar,
        1 => {
            return xmlCharStrdup(
                b"out of memory\n\0" as *const u8 as *const i8,
            );
        }
        2 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"failed to validate type %s\n\0" as *const u8 as *const i8,
                arg1,
            );
        }
        3 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Type %s doesn't allow value '%s'\n\0" as *const u8
                    as *const i8,
                arg1,
                arg2,
            );
        }
        4 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"ID %s redefined\n\0" as *const u8 as *const i8,
                arg1,
            );
        }
        5 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"failed to compare type %s\n\0" as *const u8 as *const i8,
                arg1,
            );
        }
        6 => {
            return xmlCharStrdup(
                b"Internal error: no state\n\0" as *const u8 as *const i8,
            );
        }
        7 => {
            return xmlCharStrdup(
                b"Internal error: no define\n\0" as *const u8 as *const i8,
            );
        }
        37 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Internal error: %s\n\0" as *const u8 as *const i8,
                arg1,
            );
        }
        8 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Extra data in list: %s\n\0" as *const u8 as *const i8,
                arg1,
            );
        }
        10 => {
            return xmlCharStrdup(
                b"Internal: interleave block has no data\n\0" as *const u8
                    as *const i8,
            );
        }
        11 => {
            return xmlCharStrdup(
                b"Invalid sequence in interleave\n\0" as *const u8 as *const i8,
            );
        }
        12 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Extra element %s in interleave\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        13 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Expecting element %s, got %s\n\0" as *const u8 as *const i8,
                arg1,
                arg2,
            );
        }
        15 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Expecting a namespace for element %s\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        17 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Element %s has wrong namespace: expecting %s\n\0" as *const u8
                    as *const i8,
                arg1,
                arg2,
            );
        }
        38 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Did not expect element %s there\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        39 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Did not expect text in element %s content\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        19 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Expecting no namespace for element %s\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        21 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Expecting element %s to be empty\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        22 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Expecting an element %s, got nothing\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        23 => {
            return xmlCharStrdup(
                b"Expecting an element got text\n\0" as *const u8 as *const i8,
            );
        }
        24 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Element %s failed to validate attributes\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        25 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Element %s failed to validate content\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        26 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Element %s has extra content: %s\n\0" as *const u8
                    as *const i8,
                arg1,
                arg2,
            );
        }
        27 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Invalid attribute %s for element %s\n\0" as *const u8
                    as *const i8,
                arg1,
                arg2,
            );
        }
        36 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Datatype element %s contains no data\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        28 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Datatype element %s has child elements\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        29 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Value element %s has child elements\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        30 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"List element %s has child elements\n\0" as *const u8
                    as *const i8,
                arg1,
            );
        }
        31 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Error validating datatype %s\n\0" as *const u8 as *const i8,
                arg1,
            );
        }
        32 => {
            snprintf(
                msg.as_mut_ptr(),
                1000 as i32 as u64,
                b"Error validating value %s\n\0" as *const u8 as *const i8,
                arg1,
            );
        }
        33 => {
            return xmlCharStrdup(
                b"Error validating list\n\0" as *const u8 as *const i8,
            );
        }
        34 => {
            return xmlCharStrdup(
                b"No top grammar defined\n\0" as *const u8 as *const i8,
            );
        }
        35 => {
            return xmlCharStrdup(
                b"Extra data in the document\n\0" as *const u8 as *const i8,
            );
        }
        _ => {
            return xmlCharStrdup(
                b"Unknown error !\n\0" as *const u8 as *const i8,
            );
        }
    }
    if msg[0 as i32 as usize] as i32 == 0 as i32 {
        snprintf(
            msg.as_mut_ptr(),
            1000 as i32 as u64,
            b"Unknown error code %d\n\0" as *const u8 as *const i8,
            err as u32,
        );
    }
    msg[(1000 as i32 - 1 as i32)
        as usize] = 0 as i32 as i8;
    result = xmlCharStrdup(msg.as_mut_ptr());
    return xmlEscapeFormatString(&mut result);
}
unsafe extern "C" fn xmlRelaxNGShowValidError(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut err: xmlRelaxNGValidErr,
    mut node: xmlNodePtr,
    mut child: xmlNodePtr,
    mut arg1: *const xmlChar,
    mut arg2: *const xmlChar,
) {
    let mut msg: *mut xmlChar = 0 as *mut xmlChar;
    if (*ctxt).flags & 8 as i32 != 0 {
        return;
    }
    msg = xmlRelaxNGGetErrorString(err, arg1, arg2);
    if msg.is_null() {
        return;
    }
    if (*ctxt).errNo == XML_RELAXNG_OK as i32 {
        (*ctxt).errNo = err as i32;
    }
    xmlRngVErr(
        ctxt,
        if child.is_null() { node } else { child },
        err as i32,
        msg as *const i8,
        arg1,
        arg2,
    );
    xmlFree.expect("non-null function pointer")(msg as *mut libc::c_void);
}
unsafe extern "C" fn xmlRelaxNGPopErrors(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut level: i32,
) {
    let mut i: i32 = 0;
    let mut err: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    i = level;
    while i < (*ctxt).errNr {
        err = &mut *((*ctxt).errTab).offset(i as isize) as *mut xmlRelaxNGValidError;
        if (*err).flags & 1 as i32 != 0 {
            if !((*err).arg1).is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*err).arg1 as *mut xmlChar as *mut libc::c_void);
            }
            let fresh93 = &mut ((*err).arg1);
            *fresh93 = 0 as *const xmlChar;
            if !((*err).arg2).is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*err).arg2 as *mut xmlChar as *mut libc::c_void);
            }
            let fresh94 = &mut ((*err).arg2);
            *fresh94 = 0 as *const xmlChar;
            (*err).flags = 0 as i32;
        }
        i += 1;
    }
    (*ctxt).errNr = level;
    if (*ctxt).errNr <= 0 as i32 {
        let fresh95 = &mut ((*ctxt).err);
        *fresh95 = 0 as xmlRelaxNGValidErrorPtr;
    }
}
unsafe extern "C" fn xmlRelaxNGDumpValidError(mut ctxt: xmlRelaxNGValidCtxtPtr) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut err: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    let mut dup: xmlRelaxNGValidErrorPtr = 0 as *mut xmlRelaxNGValidError;
    i = 0 as i32;
    k = 0 as i32;
    while i < (*ctxt).errNr {
        let mut current_block_14: u64;
        err = &mut *((*ctxt).errTab).offset(i as isize) as *mut xmlRelaxNGValidError;
        if k < 5 as i32 {
            j = 0 as i32;
            loop {
                if !(j < i) {
                    current_block_14 = 11812396948646013369;
                    break;
                }
                dup = &mut *((*ctxt).errTab).offset(j as isize)
                    as *mut xmlRelaxNGValidError;
                if (*err).err as u32 == (*dup).err as u32
                    && (*err).node == (*dup).node
                    && xmlStrEqual((*err).arg1, (*dup).arg1) != 0
                    && xmlStrEqual((*err).arg2, (*dup).arg2) != 0
                {
                    current_block_14 = 9509544689317704976;
                    break;
                }
                j += 1;
            }
            match current_block_14 {
                9509544689317704976 => {}
                _ => {
                    xmlRelaxNGShowValidError(
                        ctxt,
                        (*err).err,
                        (*err).node,
                        (*err).seq,
                        (*err).arg1,
                        (*err).arg2,
                    );
                    k += 1;
                }
            }
        }
        if (*err).flags & 1 as i32 != 0 {
            if !((*err).arg1).is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*err).arg1 as *mut xmlChar as *mut libc::c_void);
            }
            let fresh96 = &mut ((*err).arg1);
            *fresh96 = 0 as *const xmlChar;
            if !((*err).arg2).is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*err).arg2 as *mut xmlChar as *mut libc::c_void);
            }
            let fresh97 = &mut ((*err).arg2);
            *fresh97 = 0 as *const xmlChar;
            (*err).flags = 0 as i32;
        }
        i += 1;
    }
    (*ctxt).errNr = 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGAddValidError(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut err: xmlRelaxNGValidErr,
    mut arg1: *const xmlChar,
    mut arg2: *const xmlChar,
    mut dup: i32,
) {
    if ctxt.is_null() {
        return;
    }
    if (*ctxt).flags & 8 as i32 != 0 {
        return;
    }
    if (*ctxt).flags & 1 as i32 == 0 as i32
        || (*ctxt).flags & 2 as i32 != 0
    {
        let mut node: xmlNodePtr = 0 as *mut xmlNode;
        let mut seq: xmlNodePtr = 0 as *mut xmlNode;
        if (*ctxt).errNr != 0 as i32 {
            xmlRelaxNGDumpValidError(ctxt);
        }
        if !((*ctxt).state).is_null() {
            node = (*(*ctxt).state).node;
            seq = (*(*ctxt).state).seq;
        } else {
            seq = 0 as xmlNodePtr;
            node = seq;
        }
        if node.is_null() && seq.is_null() {
            node = (*ctxt).pnode;
        }
        xmlRelaxNGShowValidError(ctxt, err, node, seq, arg1, arg2);
    } else {
        xmlRelaxNGValidErrorPush(ctxt, err, arg1, arg2, dup);
    };
}
unsafe extern "C" fn xmlRelaxNGSchemaTypeHave(
    mut data: *mut libc::c_void,
    mut type_0: *const xmlChar,
) -> i32 {
    let mut typ: xmlSchemaTypePtr = 0 as *mut xmlSchemaType;
    if type_0.is_null() {
        return -(1 as i32);
    }
    typ = xmlSchemaGetPredefinedType(
        type_0,
        b"http://www.w3.org/2001/XMLSchema\0" as *const u8 as *const i8
            as *mut xmlChar,
    );
    if typ.is_null() {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn xmlRelaxNGSchemaTypeCheck(
    mut data: *mut libc::c_void,
    mut type_0: *const xmlChar,
    mut value: *const xmlChar,
    mut result: *mut *mut libc::c_void,
    mut node: xmlNodePtr,
) -> i32 {
    let mut typ: xmlSchemaTypePtr = 0 as *mut xmlSchemaType;
    let mut ret: i32 = 0;
    if type_0.is_null() || value.is_null() {
        return -(1 as i32);
    }
    typ = xmlSchemaGetPredefinedType(
        type_0,
        b"http://www.w3.org/2001/XMLSchema\0" as *const u8 as *const i8
            as *mut xmlChar,
    );
    if typ.is_null() {
        return -(1 as i32);
    }
    ret = xmlSchemaValPredefTypeNode(typ, value, result as *mut xmlSchemaValPtr, node);
    if ret == 2 as i32 {
        return 2 as i32;
    }
    if ret == 0 as i32 {
        return 1 as i32;
    }
    if ret > 0 as i32 {
        return 0 as i32;
    }
    return -(1 as i32);
}
unsafe extern "C" fn xmlRelaxNGSchemaFacetCheck(
    mut data: *mut libc::c_void,
    mut type_0: *const xmlChar,
    mut facetname: *const xmlChar,
    mut val: *const xmlChar,
    mut strval: *const xmlChar,
    mut value: *mut libc::c_void,
) -> i32 {
    let mut facet: xmlSchemaFacetPtr = 0 as *mut xmlSchemaFacet;
    let mut typ: xmlSchemaTypePtr = 0 as *mut xmlSchemaType;
    let mut ret: i32 = 0;
    if type_0.is_null() || strval.is_null() {
        return -(1 as i32);
    }
    typ = xmlSchemaGetPredefinedType(
        type_0,
        b"http://www.w3.org/2001/XMLSchema\0" as *const u8 as *const i8
            as *mut xmlChar,
    );
    if typ.is_null() {
        return -(1 as i32);
    }
    facet = xmlSchemaNewFacet();
    if facet.is_null() {
        return -(1 as i32);
    }
    if xmlStrEqual(
        facetname,
        b"minInclusive\0" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0
    {
        (*facet).type_0 = XML_SCHEMA_FACET_MININCLUSIVE;
    } else if xmlStrEqual(
            facetname,
            b"minExclusive\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_MINEXCLUSIVE;
    } else if xmlStrEqual(
            facetname,
            b"maxInclusive\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_MAXINCLUSIVE;
    } else if xmlStrEqual(
            facetname,
            b"maxExclusive\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_MAXEXCLUSIVE;
    } else if xmlStrEqual(
            facetname,
            b"totalDigits\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_TOTALDIGITS;
    } else if xmlStrEqual(
            facetname,
            b"fractionDigits\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_FRACTIONDIGITS;
    } else if xmlStrEqual(
            facetname,
            b"pattern\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_PATTERN;
    } else if xmlStrEqual(
            facetname,
            b"enumeration\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_ENUMERATION;
    } else if xmlStrEqual(
            facetname,
            b"whiteSpace\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_WHITESPACE;
    } else if xmlStrEqual(
            facetname,
            b"length\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_LENGTH;
    } else if xmlStrEqual(
            facetname,
            b"maxLength\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_MAXLENGTH;
    } else if xmlStrEqual(
            facetname,
            b"minLength\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        (*facet).type_0 = XML_SCHEMA_FACET_MINLENGTH;
    } else {
        xmlSchemaFreeFacet(facet);
        return -(1 as i32);
    }
    let fresh98 = &mut ((*facet).value);
    *fresh98 = val;
    ret = xmlSchemaCheckFacet(facet, typ, 0 as xmlSchemaParserCtxtPtr, type_0);
    if ret != 0 as i32 {
        xmlSchemaFreeFacet(facet);
        return -(1 as i32);
    }
    ret = xmlSchemaValidateFacet(typ, facet, strval, value as xmlSchemaValPtr);
    xmlSchemaFreeFacet(facet);
    if ret != 0 as i32 {
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGSchemaFreeValue(
    mut data: *mut libc::c_void,
    mut value: *mut libc::c_void,
) {
    xmlSchemaFreeValue(value as xmlSchemaValPtr);
}
unsafe extern "C" fn xmlRelaxNGSchemaTypeCompare(
    mut data: *mut libc::c_void,
    mut type_0: *const xmlChar,
    mut value1: *const xmlChar,
    mut ctxt1: xmlNodePtr,
    mut comp1: *mut libc::c_void,
    mut value2: *const xmlChar,
    mut ctxt2: xmlNodePtr,
) -> i32 {
    let mut ret: i32 = 0;
    let mut typ: xmlSchemaTypePtr = 0 as *mut xmlSchemaType;
    let mut res1: xmlSchemaValPtr = 0 as xmlSchemaValPtr;
    let mut res2: xmlSchemaValPtr = 0 as xmlSchemaValPtr;
    if type_0.is_null() || value1.is_null() || value2.is_null() {
        return -(1 as i32);
    }
    typ = xmlSchemaGetPredefinedType(
        type_0,
        b"http://www.w3.org/2001/XMLSchema\0" as *const u8 as *const i8
            as *mut xmlChar,
    );
    if typ.is_null() {
        return -(1 as i32);
    }
    if comp1.is_null() {
        ret = xmlSchemaValPredefTypeNode(typ, value1, &mut res1, ctxt1);
        if ret != 0 as i32 {
            return -(1 as i32);
        }
        if res1.is_null() {
            return -(1 as i32);
        }
    } else {
        res1 = comp1 as xmlSchemaValPtr;
    }
    ret = xmlSchemaValPredefTypeNode(typ, value2, &mut res2, ctxt2);
    if ret != 0 as i32 {
        if res1 != comp1 as xmlSchemaValPtr {
            xmlSchemaFreeValue(res1);
        }
        return -(1 as i32);
    }
    ret = xmlSchemaCompareValues(res1, res2);
    if res1 != comp1 as xmlSchemaValPtr {
        xmlSchemaFreeValue(res1);
    }
    xmlSchemaFreeValue(res2);
    if ret == -(2 as i32) {
        return -(1 as i32);
    }
    if ret == 0 as i32 {
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGDefaultTypeHave(
    mut data: *mut libc::c_void,
    mut type_0: *const xmlChar,
) -> i32 {
    if type_0.is_null() {
        return -(1 as i32);
    }
    if xmlStrEqual(
        type_0,
        b"string\0" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0
    {
        return 1 as i32;
    }
    if xmlStrEqual(
        type_0,
        b"token\0" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0
    {
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGDefaultTypeCheck(
    mut data: *mut libc::c_void,
    mut type_0: *const xmlChar,
    mut value: *const xmlChar,
    mut result: *mut *mut libc::c_void,
    mut node: xmlNodePtr,
) -> i32 {
    if value.is_null() {
        return -(1 as i32);
    }
    if xmlStrEqual(
        type_0,
        b"string\0" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0
    {
        return 1 as i32;
    }
    if xmlStrEqual(
        type_0,
        b"token\0" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0
    {
        return 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGDefaultTypeCompare(
    mut data: *mut libc::c_void,
    mut type_0: *const xmlChar,
    mut value1: *const xmlChar,
    mut ctxt1: xmlNodePtr,
    mut comp1: *mut libc::c_void,
    mut value2: *const xmlChar,
    mut ctxt2: xmlNodePtr,
) -> i32 {
    let mut ret: i32 = -(1 as i32);
    if xmlStrEqual(
        type_0,
        b"string\0" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0
    {
        ret = xmlStrEqual(value1, value2);
    } else if xmlStrEqual(
            type_0,
            b"token\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
        if xmlStrEqual(value1, value2) == 0 {
            let mut nval: *mut xmlChar = 0 as *mut xmlChar;
            let mut nvalue: *mut xmlChar = 0 as *mut xmlChar;
            nval = xmlRelaxNGNormalize(0 as xmlRelaxNGValidCtxtPtr, value1);
            nvalue = xmlRelaxNGNormalize(0 as xmlRelaxNGValidCtxtPtr, value2);
            if nval.is_null() || nvalue.is_null() {
                ret = -(1 as i32);
            } else if xmlStrEqual(nval, nvalue) != 0 {
                ret = 1 as i32;
            } else {
                ret = 0 as i32;
            }
            if !nval.is_null() {
                xmlFree.expect("non-null function pointer")(nval as *mut libc::c_void);
            }
            if !nvalue.is_null() {
                xmlFree.expect("non-null function pointer")(nvalue as *mut libc::c_void);
            }
        } else {
            ret = 1 as i32;
        }
    }
    return ret;
}
static mut xmlRelaxNGTypeInitialized: i32 = 0 as i32;
static mut xmlRelaxNGRegisteredTypes: xmlHashTablePtr = 0 as *const xmlHashTable
    as xmlHashTablePtr;
unsafe extern "C" fn xmlRelaxNGFreeTypeLibrary(
    mut payload: *mut libc::c_void,
    mut namespace: *const xmlChar,
) {
    let mut lib: xmlRelaxNGTypeLibraryPtr = payload as xmlRelaxNGTypeLibraryPtr;
    if lib.is_null() {
        return;
    }
    if !((*lib).namespace).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*lib).namespace as *mut xmlChar as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(lib as *mut libc::c_void);
}
unsafe extern "C" fn xmlRelaxNGRegisterTypeLibrary(
    mut namespace: *const xmlChar,
    mut data: *mut libc::c_void,
    mut have: xmlRelaxNGTypeHave,
    mut check: xmlRelaxNGTypeCheck,
    mut comp: xmlRelaxNGTypeCompare,
    mut facet: xmlRelaxNGFacetCheck,
    mut freef: xmlRelaxNGTypeFree,
) -> i32 {
    let mut lib: xmlRelaxNGTypeLibraryPtr = 0 as *mut xmlRelaxNGTypeLibrary;
    let mut ret: i32 = 0;
    if xmlRelaxNGRegisteredTypes.is_null() || namespace.is_null() || check.is_none()
        || comp.is_none()
    {
        return -(1 as i32);
    }
    if !(xmlHashLookup(xmlRelaxNGRegisteredTypes, namespace)).is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Relax-NG types library '%s' already registered\n\0" as *const u8
                as *const i8,
            namespace,
        );
        return -(1 as i32);
    }
    lib = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNGTypeLibrary>() as u64)
        as xmlRelaxNGTypeLibraryPtr;
    if lib.is_null() {
        xmlRngVErrMemory(
            0 as xmlRelaxNGValidCtxtPtr,
            b"adding types library\n\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    memset(
        lib as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGTypeLibrary>() as u64,
    );
    let fresh99 = &mut ((*lib).namespace);
    *fresh99 = xmlStrdup(namespace);
    let fresh100 = &mut ((*lib).data);
    *fresh100 = data;
    let fresh101 = &mut ((*lib).have);
    *fresh101 = have;
    let fresh102 = &mut ((*lib).comp);
    *fresh102 = comp;
    let fresh103 = &mut ((*lib).check);
    *fresh103 = check;
    let fresh104 = &mut ((*lib).facet);
    *fresh104 = facet;
    let fresh105 = &mut ((*lib).freef);
    *fresh105 = freef;
    ret = xmlHashAddEntry(
        xmlRelaxNGRegisteredTypes,
        namespace,
        lib as *mut libc::c_void,
    );
    if ret < 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Relax-NG types library failed to register '%s'\n\0" as *const u8
                as *const i8,
            namespace,
        );
        xmlRelaxNGFreeTypeLibrary(lib as *mut libc::c_void, namespace);
        return -(1 as i32);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGInitTypes() -> i32 {
    if xmlRelaxNGTypeInitialized != 0 as i32 {
        return 0 as i32;
    }
    xmlRelaxNGRegisteredTypes = xmlHashCreate(10 as i32);
    if xmlRelaxNGRegisteredTypes.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Failed to allocate sh table for Relax-NG types\n\0" as *const u8
                as *const i8,
        );
        return -(1 as i32);
    }
    xmlRelaxNGRegisterTypeLibrary(
        b"http://www.w3.org/2001/XMLSchema-datatypes\0" as *const u8
            as *const i8 as *mut xmlChar,
        0 as *mut libc::c_void,
        Some(
            xmlRelaxNGSchemaTypeHave
                as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> i32,
        ),
        Some(
            xmlRelaxNGSchemaTypeCheck
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *mut *mut libc::c_void,
                    xmlNodePtr,
                ) -> i32,
        ),
        Some(
            xmlRelaxNGSchemaTypeCompare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    xmlNodePtr,
                    *mut libc::c_void,
                    *const xmlChar,
                    xmlNodePtr,
                ) -> i32,
        ),
        Some(
            xmlRelaxNGSchemaFacetCheck
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                    *mut libc::c_void,
                ) -> i32,
        ),
        Some(
            xmlRelaxNGSchemaFreeValue
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
    );
    xmlRelaxNGRegisterTypeLibrary(
        xmlRelaxNGNs,
        0 as *mut libc::c_void,
        Some(
            xmlRelaxNGDefaultTypeHave
                as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> i32,
        ),
        Some(
            xmlRelaxNGDefaultTypeCheck
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *mut *mut libc::c_void,
                    xmlNodePtr,
                ) -> i32,
        ),
        Some(
            xmlRelaxNGDefaultTypeCompare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    xmlNodePtr,
                    *mut libc::c_void,
                    *const xmlChar,
                    xmlNodePtr,
                ) -> i32,
        ),
        None,
        None,
    );
    xmlRelaxNGTypeInitialized = 1 as i32;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGCleanupTypes() {
    xmlSchemaCleanupTypes();
    if xmlRelaxNGTypeInitialized == 0 as i32 {
        return;
    }
    xmlHashFree(
        xmlRelaxNGRegisteredTypes,
        Some(
            xmlRelaxNGFreeTypeLibrary
                as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
        ),
    );
    xmlRelaxNGTypeInitialized = 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGIsCompilable(
    mut def: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = -(1 as i32);
    if def.is_null() {
        return -(1 as i32);
    }
    if (*def).type_0 as i32 != XML_RELAXNG_ELEMENT as i32
        && (*def).dflags as i32 & (1 as i32) << 6 as i32 != 0
    {
        return 1 as i32;
    }
    if (*def).type_0 as i32 != XML_RELAXNG_ELEMENT as i32
        && (*def).dflags as i32 & (1 as i32) << 7 as i32 != 0
    {
        return 0 as i32;
    }
    match (*def).type_0 as i32 {
        -1 => {
            ret = xmlRelaxNGIsCompilable((*def).content);
        }
        3 | 0 => {
            ret = 1 as i32;
        }
        4 => {
            if (*def).dflags as i32 & (1 as i32) << 7 as i32
                == 0 as i32
                && (*def).dflags as i32 & (1 as i32) << 6 as i32
                    == 0 as i32
            {
                let mut list: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                list = (*def).content;
                while !list.is_null() {
                    ret = xmlRelaxNGIsCompilable(list);
                    if ret != 1 as i32 {
                        break;
                    }
                    list = (*list).next;
                }
                if ret == 0 as i32 {
                    let fresh106 = &mut ((*def).dflags);
                    *fresh106 = (*fresh106 as i32
                        & !((1 as i32) << 6 as i32)) as i16;
                    let fresh107 = &mut ((*def).dflags);
                    *fresh107 = (*fresh107 as i32
                        | (1 as i32) << 7 as i32) as i16;
                }
                if ret == 1 as i32
                    && {
                        let fresh108 = &mut ((*def).dflags);
                        *fresh108 = (*fresh108 as i32
                            & (1 as i32) << 7 as i32) as i16;
                        *fresh108 == 0
                    }
                {
                    let fresh109 = &mut ((*def).dflags);
                    *fresh109 = (*fresh109 as i32
                        | (1 as i32) << 6 as i32) as i16;
                }
            }
            if !((*def).nameClass).is_null() || ((*def).name).is_null() {
                ret = 0 as i32;
            } else {
                ret = 1 as i32;
            }
            return ret;
        }
        11 | 12 | 13 => {
            if (*def).depth as i32 == -(20 as i32) {
                return 1 as i32
            } else {
                let mut list_0: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                (*def).depth = -(20 as i32) as i16;
                list_0 = (*def).content;
                while !list_0.is_null() {
                    ret = xmlRelaxNGIsCompilable(list_0);
                    if ret != 1 as i32 {
                        break;
                    }
                    list_0 = (*list_0).next;
                }
            }
        }
        20 | 14 | 15 | 16 | 17 | 18 | 10 => {
            let mut list_1: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
            list_1 = (*def).content;
            while !list_1.is_null() {
                ret = xmlRelaxNGIsCompilable(list_1);
                if ret != 1 as i32 {
                    break;
                }
                list_1 = (*list_1).next;
            }
        }
        2 | 9 | 19 | 5 | 8 | 6 | 7 | 1 => {
            ret = 0 as i32;
        }
        _ => {}
    }
    if ret == 0 as i32 {
        let fresh110 = &mut ((*def).dflags);
        *fresh110 = (*fresh110 as i32 | (1 as i32) << 7 as i32)
            as i16;
    }
    if ret == 1 as i32 {
        let fresh111 = &mut ((*def).dflags);
        *fresh111 = (*fresh111 as i32 | (1 as i32) << 6 as i32)
            as i16;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGCompile(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut def: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut list: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if ctxt.is_null() || def.is_null() {
        return -(1 as i32);
    }
    match (*def).type_0 as i32 {
        20 => {
            if xmlRelaxNGIsCompilable(def) == 1 as i32
                && (*def).depth as i32 != -(25 as i32)
            {
                let mut oldam: xmlAutomataPtr = (*ctxt).am;
                let mut oldstate: xmlAutomataStatePtr = (*ctxt).state;
                (*def).depth = -(25 as i32) as i16;
                list = (*def).content;
                let fresh112 = &mut ((*ctxt).am);
                *fresh112 = xmlNewAutomata();
                if ((*ctxt).am).is_null() {
                    return -(1 as i32);
                }
                xmlAutomataSetFlags((*ctxt).am, 1 as i32);
                let fresh113 = &mut ((*ctxt).state);
                *fresh113 = xmlAutomataGetInitState((*ctxt).am);
                while !list.is_null() {
                    xmlRelaxNGCompile(ctxt, list);
                    list = (*list).next;
                }
                xmlAutomataSetFinalState((*ctxt).am, (*ctxt).state);
                if xmlAutomataIsDeterminist((*ctxt).am) != 0 {
                    let fresh114 = &mut ((*def).contModel);
                    *fresh114 = xmlAutomataCompile((*ctxt).am);
                }
                xmlFreeAutomata((*ctxt).am);
                let fresh115 = &mut ((*ctxt).state);
                *fresh115 = oldstate;
                let fresh116 = &mut ((*ctxt).am);
                *fresh116 = oldam;
            }
        }
        4 => {
            if !((*ctxt).am).is_null() && !((*def).name).is_null() {
                let fresh117 = &mut ((*ctxt).state);
                *fresh117 = xmlAutomataNewTransition2(
                    (*ctxt).am,
                    (*ctxt).state,
                    0 as xmlAutomataStatePtr,
                    (*def).name,
                    (*def).ns,
                    def as *mut libc::c_void,
                );
            }
            if (*def).dflags as i32 & (1 as i32) << 6 as i32 != 0
                && (*def).depth as i32 != -(25 as i32)
            {
                let mut oldam_0: xmlAutomataPtr = (*ctxt).am;
                let mut oldstate_0: xmlAutomataStatePtr = (*ctxt).state;
                (*def).depth = -(25 as i32) as i16;
                list = (*def).content;
                let fresh118 = &mut ((*ctxt).am);
                *fresh118 = xmlNewAutomata();
                if ((*ctxt).am).is_null() {
                    return -(1 as i32);
                }
                xmlAutomataSetFlags((*ctxt).am, 1 as i32);
                let fresh119 = &mut ((*ctxt).state);
                *fresh119 = xmlAutomataGetInitState((*ctxt).am);
                while !list.is_null() {
                    xmlRelaxNGCompile(ctxt, list);
                    list = (*list).next;
                }
                xmlAutomataSetFinalState((*ctxt).am, (*ctxt).state);
                let fresh120 = &mut ((*def).contModel);
                *fresh120 = xmlAutomataCompile((*ctxt).am);
                if xmlRegexpIsDeterminist((*def).contModel) == 0 {
                    xmlRegFreeRegexp((*def).contModel);
                    let fresh121 = &mut ((*def).contModel);
                    *fresh121 = 0 as xmlRegexpPtr;
                }
                xmlFreeAutomata((*ctxt).am);
                let fresh122 = &mut ((*ctxt).state);
                *fresh122 = oldstate_0;
                let fresh123 = &mut ((*ctxt).am);
                *fresh123 = oldam_0;
            } else {
                let mut oldam_1: xmlAutomataPtr = (*ctxt).am;
                ret = xmlRelaxNGTryCompile(ctxt, def);
                let fresh124 = &mut ((*ctxt).am);
                *fresh124 = oldam_1;
            }
        }
        -1 => {
            ret = xmlRelaxNGCompile(ctxt, (*def).content);
        }
        14 => {
            let mut oldstate_1: xmlAutomataStatePtr = (*ctxt).state;
            list = (*def).content;
            while !list.is_null() {
                xmlRelaxNGCompile(ctxt, list);
                list = (*list).next;
            }
            xmlAutomataNewEpsilon((*ctxt).am, oldstate_1, (*ctxt).state);
        }
        15 => {
            let mut oldstate_2: xmlAutomataStatePtr = 0 as *mut xmlAutomataState;
            let fresh125 = &mut ((*ctxt).state);
            *fresh125 = xmlAutomataNewEpsilon(
                (*ctxt).am,
                (*ctxt).state,
                0 as xmlAutomataStatePtr,
            );
            oldstate_2 = (*ctxt).state;
            list = (*def).content;
            while !list.is_null() {
                xmlRelaxNGCompile(ctxt, list);
                list = (*list).next;
            }
            xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, oldstate_2);
            let fresh126 = &mut ((*ctxt).state);
            *fresh126 = xmlAutomataNewEpsilon(
                (*ctxt).am,
                oldstate_2,
                0 as xmlAutomataStatePtr,
            );
        }
        16 => {
            let mut oldstate_3: xmlAutomataStatePtr = 0 as *mut xmlAutomataState;
            list = (*def).content;
            while !list.is_null() {
                xmlRelaxNGCompile(ctxt, list);
                list = (*list).next;
            }
            oldstate_3 = (*ctxt).state;
            list = (*def).content;
            while !list.is_null() {
                xmlRelaxNGCompile(ctxt, list);
                list = (*list).next;
            }
            xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, oldstate_3);
            let fresh127 = &mut ((*ctxt).state);
            *fresh127 = xmlAutomataNewEpsilon(
                (*ctxt).am,
                oldstate_3,
                0 as xmlAutomataStatePtr,
            );
        }
        17 => {
            let mut target: xmlAutomataStatePtr = 0 as xmlAutomataStatePtr;
            let mut oldstate_4: xmlAutomataStatePtr = (*ctxt).state;
            list = (*def).content;
            while !list.is_null() {
                let fresh128 = &mut ((*ctxt).state);
                *fresh128 = oldstate_4;
                ret = xmlRelaxNGCompile(ctxt, list);
                if ret != 0 as i32 {
                    break;
                }
                if target.is_null() {
                    target = (*ctxt).state;
                } else {
                    xmlAutomataNewEpsilon((*ctxt).am, (*ctxt).state, target);
                }
                list = (*list).next;
            }
            let fresh129 = &mut ((*ctxt).state);
            *fresh129 = target;
        }
        11 | 12 | 13 | 18 | 10 => {
            list = (*def).content;
            while !list.is_null() {
                ret = xmlRelaxNGCompile(ctxt, list);
                if ret != 0 as i32 {
                    break;
                }
                list = (*list).next;
            }
        }
        3 => {
            let mut oldstate_5: xmlAutomataStatePtr = 0 as *mut xmlAutomataState;
            let fresh130 = &mut ((*ctxt).state);
            *fresh130 = xmlAutomataNewEpsilon(
                (*ctxt).am,
                (*ctxt).state,
                0 as xmlAutomataStatePtr,
            );
            oldstate_5 = (*ctxt).state;
            xmlRelaxNGCompile(ctxt, (*def).content);
            xmlAutomataNewTransition(
                (*ctxt).am,
                (*ctxt).state,
                (*ctxt).state,
                b"#text\0" as *const u8 as *const i8 as *mut xmlChar,
                0 as *mut libc::c_void,
            );
            let fresh131 = &mut ((*ctxt).state);
            *fresh131 = xmlAutomataNewEpsilon(
                (*ctxt).am,
                oldstate_5,
                0 as xmlAutomataStatePtr,
            );
        }
        0 => {
            let fresh132 = &mut ((*ctxt).state);
            *fresh132 = xmlAutomataNewEpsilon(
                (*ctxt).am,
                (*ctxt).state,
                0 as xmlAutomataStatePtr,
            );
        }
        2 | 9 | 19 | 1 | 5 | 8 | 6 | 7 => {
            fprintf(
                stderr,
                b"RNG internal error trying to compile %s\n\0" as *const u8
                    as *const i8,
                xmlRelaxNGDefName(def),
            );
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGTryCompile(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut def: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut list: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if ctxt.is_null() || def.is_null() {
        return -(1 as i32);
    }
    if (*def).type_0 as i32 == XML_RELAXNG_START as i32
        || (*def).type_0 as i32 == XML_RELAXNG_ELEMENT as i32
    {
        ret = xmlRelaxNGIsCompilable(def);
        if (*def).dflags as i32 & (1 as i32) << 6 as i32 != 0
            && (*def).depth as i32 != -(25 as i32)
        {
            let fresh133 = &mut ((*ctxt).am);
            *fresh133 = 0 as xmlAutomataPtr;
            ret = xmlRelaxNGCompile(ctxt, def);
            return ret;
        }
    }
    match (*def).type_0 as i32 {
        -1 => {
            ret = xmlRelaxNGTryCompile(ctxt, (*def).content);
        }
        3 | 5 | 8 | 6 | 7 | 0 | 4 => {
            ret = 0 as i32;
        }
        14 | 15 | 16 | 17 | 18 | 10 | 20 | 11 | 12 | 13 => {
            list = (*def).content;
            while !list.is_null() {
                ret = xmlRelaxNGTryCompile(ctxt, list);
                if ret != 0 as i32 {
                    break;
                }
                list = (*list).next;
            }
        }
        2 | 9 | 19 | 1 => {
            ret = 0 as i32;
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGIsNullable(
    mut define: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0;
    if define.is_null() {
        return -(1 as i32);
    }
    if (*define).dflags as i32 & (1 as i32) << 0 as i32 != 0 {
        return 1 as i32;
    }
    if (*define).dflags as i32 & (1 as i32) << 1 as i32 != 0 {
        return 0 as i32;
    }
    match (*define).type_0 as i32 {
        0 | 3 => {
            ret = 1 as i32;
        }
        -1 | 10 | 11 | 12 | 13 | 16 => {
            ret = xmlRelaxNGIsNullable((*define).content);
        }
        2 | 1 | 4 | 5 | 6 | 7 | 8 | 9 => {
            ret = 0 as i32;
        }
        17 => {
            let mut list: xmlRelaxNGDefinePtr = (*define).content;
            loop {
                if list.is_null() {
                    current_block = 15089075282327824602;
                    break;
                }
                ret = xmlRelaxNGIsNullable(list);
                if ret != 0 as i32 {
                    current_block = 12056796777849235375;
                    break;
                }
                list = (*list).next;
            }
            match current_block {
                12056796777849235375 => {}
                _ => {
                    ret = 0 as i32;
                }
            }
        }
        20 | 19 | 18 => {
            let mut list_0: xmlRelaxNGDefinePtr = (*define).content;
            loop {
                if list_0.is_null() {
                    current_block = 14359455889292382949;
                    break;
                }
                ret = xmlRelaxNGIsNullable(list_0);
                if ret != 1 as i32 {
                    current_block = 12056796777849235375;
                    break;
                }
                list_0 = (*list_0).next;
            }
            match current_block {
                12056796777849235375 => {}
                _ => return 1 as i32,
            }
        }
        _ => return -(1 as i32),
    }
    if ret == 0 as i32 {
        let fresh134 = &mut ((*define).dflags);
        *fresh134 = (*fresh134 as i32 | (1 as i32) << 1 as i32)
            as i16;
    }
    if ret == 1 as i32 {
        let fresh135 = &mut ((*define).dflags);
        *fresh135 = (*fresh135 as i32 | (1 as i32) << 0 as i32)
            as i16;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGIsBlank(mut str: *mut xmlChar) -> i32 {
    if str.is_null() {
        return 1 as i32;
    }
    while *str as i32 != 0 as i32 {
        if !(*str as i32 == 0x20 as i32
            || 0x9 as i32 <= *str as i32
                && *str as i32 <= 0xa as i32
            || *str as i32 == 0xd as i32)
        {
            return 0 as i32;
        }
        str = str.offset(1);
    }
    return 1 as i32;
}
unsafe extern "C" fn xmlRelaxNGGetDataTypeLibrary(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut escape: *mut xmlChar = 0 as *mut xmlChar;
    if node.is_null() {
        return 0 as *mut xmlChar;
    }
    if !node.is_null() && !((*node).ns).is_null()
        && (*node).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        && xmlStrEqual(
            (*node).name,
            b"data\0" as *const u8 as *const i8 as *const xmlChar,
        ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        || !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"value\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
    {
        ret = xmlGetProp(
            node as *const xmlNode,
            b"datatypeLibrary\0" as *const u8 as *const i8 as *mut xmlChar,
        );
        if !ret.is_null() {
            if *ret.offset(0 as i32 as isize) as i32 == 0 as i32
            {
                xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
                return 0 as *mut xmlChar;
            }
            escape = xmlURIEscapeStr(
                ret,
                b":/#?\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            if escape.is_null() {
                return ret;
            }
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            return escape;
        }
    }
    node = (*node).parent;
    while !node.is_null()
        && (*node).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
    {
        ret = xmlGetProp(
            node as *const xmlNode,
            b"datatypeLibrary\0" as *const u8 as *const i8 as *mut xmlChar,
        );
        if !ret.is_null() {
            if *ret.offset(0 as i32 as isize) as i32 == 0 as i32
            {
                xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
                return 0 as *mut xmlChar;
            }
            escape = xmlURIEscapeStr(
                ret,
                b":/#?\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            if escape.is_null() {
                return ret;
            }
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            return escape;
        }
        node = (*node).parent;
    }
    return 0 as *mut xmlChar;
}
unsafe extern "C" fn xmlRelaxNGParseValue(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut lib: xmlRelaxNGTypeLibraryPtr = 0 as xmlRelaxNGTypeLibraryPtr;
    let mut type_0: *mut xmlChar = 0 as *mut xmlChar;
    let mut library: *mut xmlChar = 0 as *mut xmlChar;
    let mut success: i32 = 0 as i32;
    def = xmlRelaxNGNewDefine(ctxt, node);
    if def.is_null() {
        return 0 as xmlRelaxNGDefinePtr;
    }
    (*def).type_0 = XML_RELAXNG_VALUE;
    type_0 = xmlGetProp(
        node as *const xmlNode,
        b"type\0" as *const u8 as *const i8 as *mut xmlChar,
    );
    if !type_0.is_null() {
        xmlRelaxNGNormExtSpace(type_0);
        if xmlValidateNCName(type_0, 0 as i32) != 0 {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_TYPE_VALUE as i32,
                b"value type '%s' is not an NCName\n\0" as *const u8
                    as *const i8,
                type_0,
                0 as *const xmlChar,
            );
        }
        library = xmlRelaxNGGetDataTypeLibrary(ctxt, node);
        if library.is_null() {
            library = xmlStrdup(
                b"http://relaxng.org/ns/structure/1.0\0" as *const u8
                    as *const i8 as *mut xmlChar,
            );
        }
        let fresh136 = &mut ((*def).name);
        *fresh136 = type_0;
        let fresh137 = &mut ((*def).ns);
        *fresh137 = library;
        lib = xmlHashLookup(xmlRelaxNGRegisteredTypes, library)
            as xmlRelaxNGTypeLibraryPtr;
        if lib.is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_UNKNOWN_TYPE_LIB as i32,
                b"Use of unregistered type library '%s'\n\0" as *const u8
                    as *const i8,
                library,
                0 as *const xmlChar,
            );
            let fresh138 = &mut ((*def).data);
            *fresh138 = 0 as *mut libc::c_void;
        } else {
            let fresh139 = &mut ((*def).data);
            *fresh139 = lib as *mut libc::c_void;
            if ((*lib).have).is_none() {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_ERROR_TYPE_LIB as i32,
                    b"Internal error with type library '%s': no 'have'\n\0" as *const u8
                        as *const i8,
                    library,
                    0 as *const xmlChar,
                );
            } else {
                success = ((*lib).have)
                    .expect("non-null function pointer")((*lib).data, (*def).name);
                if success != 1 as i32 {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_TYPE_NOT_FOUND as i32,
                        b"Error type '%s' is not exported by type library '%s'\n\0"
                            as *const u8 as *const i8,
                        (*def).name,
                        library,
                    );
                }
            }
        }
    }
    if ((*node).children).is_null() {
        let fresh140 = &mut ((*def).value);
        *fresh140 = xmlStrdup(b"\0" as *const u8 as *const i8 as *mut xmlChar);
    } else if (*(*node).children).type_0 as u32
            != XML_TEXT_NODE as i32 as u32
            && (*(*node).children).type_0 as u32
                != XML_CDATA_SECTION_NODE as i32 as u32
            || !((*(*node).children).next).is_null()
        {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_TEXT_EXPECTED as i32,
            b"Expecting a single text value for <value>content\n\0" as *const u8
                as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    } else if !def.is_null() {
        let fresh141 = &mut ((*def).value);
        *fresh141 = xmlNodeGetContent(node as *const xmlNode);
        if ((*def).value).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_VALUE_NO_CONTENT as i32,
                b"Element <value> has no content\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else if !lib.is_null() && ((*lib).check).is_some()
                && success == 1 as i32
            {
            let mut val: *mut libc::c_void = 0 as *mut libc::c_void;
            success = ((*lib).check)
                .expect(
                    "non-null function pointer",
                )((*lib).data, (*def).name, (*def).value, &mut val, node);
            if success != 1 as i32 {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_INVALID_VALUE as i32,
                    b"Value '%s' is not acceptable for type '%s'\n\0" as *const u8
                        as *const i8,
                    (*def).value,
                    (*def).name,
                );
            } else if !val.is_null() {
                let fresh142 = &mut ((*def).attrs);
                *fresh142 = val as xmlRelaxNGDefinePtr;
            }
        }
    }
    return def;
}
unsafe extern "C" fn xmlRelaxNGParseData(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut except: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut param: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut lastparam: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut lib: xmlRelaxNGTypeLibraryPtr = 0 as *mut xmlRelaxNGTypeLibrary;
    let mut type_0: *mut xmlChar = 0 as *mut xmlChar;
    let mut library: *mut xmlChar = 0 as *mut xmlChar;
    let mut content: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp: i32 = 0;
    type_0 = xmlGetProp(
        node as *const xmlNode,
        b"type\0" as *const u8 as *const i8 as *mut xmlChar,
    );
    if type_0.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_TYPE_MISSING as i32,
            b"data has no type\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return 0 as xmlRelaxNGDefinePtr;
    }
    xmlRelaxNGNormExtSpace(type_0);
    if xmlValidateNCName(type_0, 0 as i32) != 0 {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_TYPE_VALUE as i32,
            b"data type '%s' is not an NCName\n\0" as *const u8 as *const i8,
            type_0,
            0 as *const xmlChar,
        );
    }
    library = xmlRelaxNGGetDataTypeLibrary(ctxt, node);
    if library.is_null() {
        library = xmlStrdup(
            b"http://relaxng.org/ns/structure/1.0\0" as *const u8 as *const i8
                as *mut xmlChar,
        );
    }
    def = xmlRelaxNGNewDefine(ctxt, node);
    if def.is_null() {
        xmlFree.expect("non-null function pointer")(library as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(type_0 as *mut libc::c_void);
        return 0 as xmlRelaxNGDefinePtr;
    }
    (*def).type_0 = XML_RELAXNG_DATATYPE;
    let fresh143 = &mut ((*def).name);
    *fresh143 = type_0;
    let fresh144 = &mut ((*def).ns);
    *fresh144 = library;
    lib = xmlHashLookup(xmlRelaxNGRegisteredTypes, library) as xmlRelaxNGTypeLibraryPtr;
    if lib.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_UNKNOWN_TYPE_LIB as i32,
            b"Use of unregistered type library '%s'\n\0" as *const u8
                as *const i8,
            library,
            0 as *const xmlChar,
        );
        let fresh145 = &mut ((*def).data);
        *fresh145 = 0 as *mut libc::c_void;
    } else {
        let fresh146 = &mut ((*def).data);
        *fresh146 = lib as *mut libc::c_void;
        if ((*lib).have).is_none() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_ERROR_TYPE_LIB as i32,
                b"Internal error with type library '%s': no 'have'\n\0" as *const u8
                    as *const i8,
                library,
                0 as *const xmlChar,
            );
        } else {
            tmp = ((*lib).have)
                .expect("non-null function pointer")((*lib).data, (*def).name);
            if tmp != 1 as i32 {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_TYPE_NOT_FOUND as i32,
                    b"Error type '%s' is not exported by type library '%s'\n\0"
                        as *const u8 as *const i8,
                    (*def).name,
                    library,
                );
            } else if xmlStrEqual(
                    library,
                    b"http://www.w3.org/2001/XMLSchema-datatypes\0" as *const u8
                        as *const i8 as *mut xmlChar,
                ) != 0
                    && (xmlStrEqual(
                        (*def).name,
                        b"IDREF\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0
                        || xmlStrEqual(
                            (*def).name,
                            b"IDREFS\0" as *const u8 as *const i8
                                as *mut xmlChar,
                        ) != 0)
                {
                (*ctxt).idref = 1 as i32;
            }
        }
    }
    content = (*node).children;
    while !content.is_null() {
        if xmlStrEqual(
            (*content).name,
            b"param\0" as *const u8 as *const i8 as *mut xmlChar,
        ) == 0
        {
            break;
        }
        if xmlStrEqual(
            library,
            b"http://relaxng.org/ns/structure/1.0\0" as *const u8 as *const i8
                as *mut xmlChar,
        ) != 0
        {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_PARAM_FORBIDDEN as i32,
                b"Type library '%s' does not allow type parameters\n\0" as *const u8
                    as *const i8,
                library,
                0 as *const xmlChar,
            );
            content = (*content).next;
            while !content.is_null()
                && xmlStrEqual(
                    (*content).name,
                    b"param\0" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
            {
                content = (*content).next;
            }
        } else {
            param = xmlRelaxNGNewDefine(ctxt, node);
            if !param.is_null() {
                (*param).type_0 = XML_RELAXNG_PARAM;
                let fresh147 = &mut ((*param).name);
                *fresh147 = xmlGetProp(
                    content as *const xmlNode,
                    b"name\0" as *const u8 as *const i8 as *mut xmlChar,
                );
                if ((*param).name).is_null() {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_PARAM_NAME_MISSING as i32,
                        b"param has no name\n\0" as *const u8 as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
                let fresh148 = &mut ((*param).value);
                *fresh148 = xmlNodeGetContent(content as *const xmlNode);
                if lastparam.is_null() {
                    lastparam = param;
                    let fresh149 = &mut ((*def).attrs);
                    *fresh149 = lastparam;
                } else {
                    let fresh150 = &mut ((*lastparam).next);
                    *fresh150 = param;
                    lastparam = param;
                }
                !lib.is_null();
            }
            content = (*content).next;
        }
    }
    if !content.is_null()
        && xmlStrEqual(
            (*content).name,
            b"except\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
    {
        let mut child: xmlNodePtr = 0 as *mut xmlNode;
        let mut tmp2: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
        except = xmlRelaxNGNewDefine(ctxt, node);
        if except.is_null() {
            return def;
        }
        (*except).type_0 = XML_RELAXNG_EXCEPT;
        child = (*content).children;
        let fresh151 = &mut ((*def).content);
        *fresh151 = except;
        if child.is_null() {
            xmlRngPErr(
                ctxt,
                content,
                XML_RNGP_EXCEPT_NO_CONTENT as i32,
                b"except has no content\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
        while !child.is_null() {
            tmp2 = xmlRelaxNGParsePattern(ctxt, child);
            if !tmp2.is_null() {
                if last.is_null() {
                    last = tmp2;
                    let fresh152 = &mut ((*except).content);
                    *fresh152 = last;
                } else {
                    let fresh153 = &mut ((*last).next);
                    *fresh153 = tmp2;
                    last = tmp2;
                }
            }
            child = (*child).next;
        }
        content = (*content).next;
    }
    if !content.is_null() {
        xmlRngPErr(
            ctxt,
            content,
            XML_RNGP_DATA_CONTENT as i32,
            b"Element data has unexpected content %s\n\0" as *const u8
                as *const i8,
            (*content).name,
            0 as *const xmlChar,
        );
    }
    return def;
}
static mut invalidName: *const xmlChar = b"\x01\0" as *const u8 as *const i8
    as *mut xmlChar;
unsafe extern "C" fn xmlRelaxNGCompareNameClasses(
    mut def1: xmlRelaxNGDefinePtr,
    mut def2: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = 1 as i32;
    let mut node: xmlNode = xmlNode {
        _private: 0 as *mut libc::c_void,
        type_0: 0 as xmlElementType,
        name: 0 as *const xmlChar,
        children: 0 as *mut _xmlNode,
        last: 0 as *mut _xmlNode,
        parent: 0 as *mut _xmlNode,
        next: 0 as *mut _xmlNode,
        prev: 0 as *mut _xmlNode,
        doc: 0 as *mut _xmlDoc,
        ns: 0 as *mut xmlNs,
        content: 0 as *mut xmlChar,
        properties: 0 as *mut _xmlAttr,
        nsDef: 0 as *mut xmlNs,
        psvi: 0 as *mut libc::c_void,
        line: 0,
        extra: 0,
    };
    let mut ns: xmlNs = xmlNs {
        next: 0 as *mut _xmlNs,
        type_0: 0 as xmlElementType,
        href: 0 as *const xmlChar,
        prefix: 0 as *const xmlChar,
        _private: 0 as *mut libc::c_void,
        context: 0 as *mut _xmlDoc,
    };
    let mut ctxt: xmlRelaxNGValidCtxt = xmlRelaxNGValidCtxt {
        userData: 0 as *mut libc::c_void,
        error: None,
        warning: None,
        serror: None,
        nbErrors: 0,
        schema: 0 as *mut xmlRelaxNG,
        doc: 0 as *mut xmlDoc,
        flags: 0,
        depth: 0,
        idref: 0,
        errNo: 0,
        err: 0 as *mut xmlRelaxNGValidError,
        errNr: 0,
        errMax: 0,
        errTab: 0 as *mut xmlRelaxNGValidError,
        state: 0 as *mut xmlRelaxNGValidState,
        states: 0 as *mut xmlRelaxNGStates,
        freeState: 0 as *mut xmlRelaxNGStates,
        freeStatesNr: 0,
        freeStatesMax: 0,
        freeStates: 0 as *mut xmlRelaxNGStatesPtr,
        elem: 0 as *mut xmlRegExecCtxt,
        elemNr: 0,
        elemMax: 0,
        elemTab: 0 as *mut xmlRegExecCtxtPtr,
        pstate: 0,
        pnode: 0 as *mut xmlNode,
        pdef: 0 as *mut xmlRelaxNGDefine,
        perr: 0,
    };
    memset(
        &mut ctxt as *mut xmlRelaxNGValidCtxt as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGValidCtxt>() as u64,
    );
    ctxt.flags = 1 as i32 | 8 as i32;
    if (*def1).type_0 as i32 == XML_RELAXNG_ELEMENT as i32
        || (*def1).type_0 as i32 == XML_RELAXNG_ATTRIBUTE as i32
    {
        if (*def2).type_0 as i32 == XML_RELAXNG_TEXT as i32 {
            return 1 as i32;
        }
        if !((*def1).name).is_null() {
            node.name = (*def1).name;
        } else {
            node.name = invalidName;
        }
        if !((*def1).ns).is_null() {
            if *((*def1).ns).offset(0 as i32 as isize) as i32
                == 0 as i32
            {
                node.ns = 0 as *mut xmlNs;
            } else {
                node.ns = &mut ns;
                ns.href = (*def1).ns;
            }
        } else {
            node.ns = 0 as *mut xmlNs;
        }
        if xmlRelaxNGElementMatch(&mut ctxt, def2, &mut node) != 0 {
            if !((*def1).nameClass).is_null() {
                ret = xmlRelaxNGCompareNameClasses((*def1).nameClass, def2);
            } else {
                ret = 0 as i32;
            }
        } else {
            ret = 1 as i32;
        }
    } else if (*def1).type_0 as i32 == XML_RELAXNG_TEXT as i32 {
        if (*def2).type_0 as i32 == XML_RELAXNG_TEXT as i32 {
            return 0 as i32;
        }
        return 1 as i32;
    } else {
        if (*def1).type_0 as i32 == XML_RELAXNG_EXCEPT as i32 {
            ret = xmlRelaxNGCompareNameClasses((*def1).content, def2);
            if ret == 0 as i32 {
                ret = 1 as i32;
            } else if ret == 1 as i32 {
                ret = 0 as i32;
            }
        } else {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
                b"relaxng.c\0" as *const u8 as *const i8,
                3851 as i32,
            );
            ret = 0 as i32;
        }
    }
    if ret == 0 as i32 {
        return ret;
    }
    if (*def2).type_0 as i32 == XML_RELAXNG_ELEMENT as i32
        || (*def2).type_0 as i32 == XML_RELAXNG_ATTRIBUTE as i32
    {
        if !((*def2).name).is_null() {
            node.name = (*def2).name;
        } else {
            node.name = invalidName;
        }
        node.ns = &mut ns;
        if !((*def2).ns).is_null() {
            if *((*def2).ns).offset(0 as i32 as isize) as i32
                == 0 as i32
            {
                node.ns = 0 as *mut xmlNs;
            } else {
                ns.href = (*def2).ns;
            }
        } else {
            ns.href = invalidName;
        }
        if xmlRelaxNGElementMatch(&mut ctxt, def1, &mut node) != 0 {
            if !((*def2).nameClass).is_null() {
                ret = xmlRelaxNGCompareNameClasses((*def2).nameClass, def1);
            } else {
                ret = 0 as i32;
            }
        } else {
            ret = 1 as i32;
        }
    } else {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
            b"relaxng.c\0" as *const u8 as *const i8,
            3882 as i32,
        );
        ret = 0 as i32;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGCompareElemDefLists(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut def1: *mut xmlRelaxNGDefinePtr,
    mut def2: *mut xmlRelaxNGDefinePtr,
) -> i32 {
    let mut basedef2: *mut xmlRelaxNGDefinePtr = def2;
    if def1.is_null() || def2.is_null() {
        return 1 as i32;
    }
    if (*def1).is_null() || (*def2).is_null() {
        return 1 as i32;
    }
    while !(*def1).is_null() {
        while !(*def2).is_null() {
            if xmlRelaxNGCompareNameClasses(*def1, *def2) == 0 as i32 {
                return 0 as i32;
            }
            def2 = def2.offset(1);
        }
        def2 = basedef2;
        def1 = def1.offset(1);
    }
    return 1 as i32;
}
unsafe extern "C" fn xmlRelaxNGGenerateAttributes(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut def: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut parent: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if (*ctxt).nbErrors != 0 as i32 {
        return -(1 as i32);
    }
    parent = 0 as xmlRelaxNGDefinePtr;
    cur = def;
    while !cur.is_null() {
        if (*cur).type_0 as i32 == XML_RELAXNG_ELEMENT as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_TEXT as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_DATATYPE as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_PARAM as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_LIST as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_VALUE as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_EMPTY as i32
        {
            return 0 as i32;
        }
        if (*cur).type_0 as i32 == XML_RELAXNG_CHOICE as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_INTERLEAVE as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_GROUP as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_ONEORMORE as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_ZEROORMORE as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_OPTIONAL as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_PARENTREF as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_EXTERNALREF as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_REF as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_DEF as i32
        {
            if !((*cur).content).is_null() {
                parent = cur;
                cur = (*cur).content;
                tmp = cur;
                while !tmp.is_null() {
                    let fresh154 = &mut ((*tmp).parent);
                    *fresh154 = parent;
                    tmp = (*tmp).next;
                }
                continue;
            }
        }
        if cur == def {
            break;
        }
        if !((*cur).next).is_null() {
            cur = (*cur).next;
        } else {
            loop {
                cur = (*cur).parent;
                if cur.is_null() {
                    break;
                }
                if cur == def {
                    return 1 as i32;
                }
                if !((*cur).next).is_null() {
                    cur = (*cur).next;
                    break;
                } else if cur.is_null() {
                    break;
                }
            }
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn xmlRelaxNGGetElements(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut def: xmlRelaxNGDefinePtr,
    mut eora: i32,
) -> *mut xmlRelaxNGDefinePtr {
    let mut ret: *mut xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefinePtr;
    let mut parent: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut len: i32 = 0 as i32;
    let mut max: i32 = 0 as i32;
    if (*ctxt).nbErrors != 0 as i32 {
        return 0 as *mut xmlRelaxNGDefinePtr;
    }
    parent = 0 as xmlRelaxNGDefinePtr;
    cur = def;
    while !cur.is_null() {
        if eora == 0 as i32
            && ((*cur).type_0 as i32 == XML_RELAXNG_ELEMENT as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_TEXT as i32)
            || eora == 1 as i32
                && (*cur).type_0 as i32 == XML_RELAXNG_ATTRIBUTE as i32
            || eora == 2 as i32
                && ((*cur).type_0 as i32 == XML_RELAXNG_DATATYPE as i32
                    || (*cur).type_0 as i32 == XML_RELAXNG_ELEMENT as i32
                    || (*cur).type_0 as i32 == XML_RELAXNG_LIST as i32
                    || (*cur).type_0 as i32 == XML_RELAXNG_TEXT as i32
                    || (*cur).type_0 as i32 == XML_RELAXNG_VALUE as i32)
        {
            if ret.is_null() {
                max = 10 as i32;
                ret = xmlMalloc
                    .expect(
                        "non-null function pointer",
                    )(
                    ((max + 1 as i32) as u64)
                        .wrapping_mul(
                            ::std::mem::size_of::<xmlRelaxNGDefinePtr>() as u64,
                        ),
                ) as *mut xmlRelaxNGDefinePtr;
                if ret.is_null() {
                    xmlRngPErrMemory(
                        ctxt,
                        b"getting element list\n\0" as *const u8 as *const i8,
                    );
                    return 0 as *mut xmlRelaxNGDefinePtr;
                }
            } else if max <= len {
                let mut temp: *mut xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefinePtr;
                max *= 2 as i32;
                temp = xmlRealloc
                    .expect(
                        "non-null function pointer",
                    )(
                    ret as *mut libc::c_void,
                    ((max + 1 as i32) as u64)
                        .wrapping_mul(
                            ::std::mem::size_of::<xmlRelaxNGDefinePtr>() as u64,
                        ),
                ) as *mut xmlRelaxNGDefinePtr;
                if temp.is_null() {
                    xmlRngPErrMemory(
                        ctxt,
                        b"getting element list\n\0" as *const u8 as *const i8,
                    );
                    xmlFree
                        .expect("non-null function pointer")(ret as *mut libc::c_void);
                    return 0 as *mut xmlRelaxNGDefinePtr;
                }
                ret = temp;
            }
            let fresh155 = len;
            len = len + 1;
            let fresh156 = &mut (*ret.offset(fresh155 as isize));
            *fresh156 = cur;
            let fresh157 = &mut (*ret.offset(len as isize));
            *fresh157 = 0 as xmlRelaxNGDefinePtr;
        } else if (*cur).type_0 as i32 == XML_RELAXNG_CHOICE as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_INTERLEAVE as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_GROUP as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_ONEORMORE as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_ZEROORMORE as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_OPTIONAL as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_PARENTREF as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_REF as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_DEF as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_EXTERNALREF as i32
            {
            if !((*cur).content).is_null() {
                parent = cur;
                cur = (*cur).content;
                tmp = cur;
                while !tmp.is_null() {
                    let fresh158 = &mut ((*tmp).parent);
                    *fresh158 = parent;
                    tmp = (*tmp).next;
                }
                continue;
            }
        }
        if cur == def {
            break;
        }
        if !((*cur).next).is_null() {
            cur = (*cur).next;
        } else {
            loop {
                cur = (*cur).parent;
                if cur.is_null() {
                    break;
                }
                if cur == def {
                    return ret;
                }
                if !((*cur).next).is_null() {
                    cur = (*cur).next;
                    break;
                } else if cur.is_null() {
                    break;
                }
            }
        }
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGCheckChoiceDeterminism(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut def: xmlRelaxNGDefinePtr,
) {
    let mut list: *mut *mut xmlRelaxNGDefinePtr = 0 as *mut *mut xmlRelaxNGDefinePtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut nbchild: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: i32 = 0;
    let mut is_nullable: i32 = 0 as i32;
    let mut is_indeterminist: i32 = 0 as i32;
    let mut triage: xmlHashTablePtr = 0 as xmlHashTablePtr;
    let mut is_triable: i32 = 1 as i32;
    if def.is_null() || (*def).type_0 as i32 != XML_RELAXNG_CHOICE as i32
    {
        return;
    }
    if (*def).dflags as i32 & (1 as i32) << 5 as i32 != 0 {
        return;
    }
    if (*ctxt).nbErrors != 0 as i32 {
        return;
    }
    is_nullable = xmlRelaxNGIsNullable(def);
    cur = (*def).content;
    while !cur.is_null() {
        nbchild += 1;
        cur = (*cur).next;
    }
    list = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (nbchild as u64)
            .wrapping_mul(
                ::std::mem::size_of::<*mut xmlRelaxNGDefinePtr>() as u64,
            ),
    ) as *mut *mut xmlRelaxNGDefinePtr;
    if list.is_null() {
        xmlRngPErrMemory(
            ctxt,
            b"building choice\n\0" as *const u8 as *const i8,
        );
        return;
    }
    i = 0 as i32;
    if is_nullable == 0 as i32 {
        triage = xmlHashCreate(10 as i32);
    } else {
        is_triable = 0 as i32;
    }
    cur = (*def).content;
    while !cur.is_null() {
        let fresh159 = &mut (*list.offset(i as isize));
        *fresh159 = xmlRelaxNGGetElements(ctxt, cur, 0 as i32);
        if (*list.offset(i as isize)).is_null()
            || (*(*list.offset(i as isize)).offset(0 as i32 as isize)).is_null()
        {
            is_triable = 0 as i32;
        } else if is_triable == 1 as i32 {
            let mut tmp: *mut xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefinePtr;
            let mut res: i32 = 0;
            tmp = *list.offset(i as isize);
            while !(*tmp).is_null() && is_triable == 1 as i32 {
                if (**tmp).type_0 as i32 == XML_RELAXNG_TEXT as i32 {
                    res = xmlHashAddEntry2(
                        triage,
                        b"#text\0" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                        cur as *mut libc::c_void,
                    );
                    if res != 0 as i32 {
                        is_triable = -(1 as i32);
                    }
                } else if (**tmp).type_0 as i32
                        == XML_RELAXNG_ELEMENT as i32
                        && !((**tmp).name).is_null()
                    {
                    if ((**tmp).ns).is_null()
                        || *((**tmp).ns).offset(0 as i32 as isize) as i32
                            == 0 as i32
                    {
                        res = xmlHashAddEntry2(
                            triage,
                            (**tmp).name,
                            0 as *const xmlChar,
                            cur as *mut libc::c_void,
                        );
                    } else {
                        res = xmlHashAddEntry2(
                            triage,
                            (**tmp).name,
                            (**tmp).ns,
                            cur as *mut libc::c_void,
                        );
                    }
                    if res != 0 as i32 {
                        is_triable = -(1 as i32);
                    }
                } else if (**tmp).type_0 as i32
                        == XML_RELAXNG_ELEMENT as i32
                    {
                    if ((**tmp).ns).is_null()
                        || *((**tmp).ns).offset(0 as i32 as isize) as i32
                            == 0 as i32
                    {
                        res = xmlHashAddEntry2(
                            triage,
                            b"#any\0" as *const u8 as *const i8
                                as *mut xmlChar,
                            0 as *const xmlChar,
                            cur as *mut libc::c_void,
                        );
                    } else {
                        res = xmlHashAddEntry2(
                            triage,
                            b"#any\0" as *const u8 as *const i8
                                as *mut xmlChar,
                            (**tmp).ns,
                            cur as *mut libc::c_void,
                        );
                    }
                    if res != 0 as i32 {
                        is_triable = -(1 as i32);
                    }
                } else {
                    is_triable = -(1 as i32);
                }
                tmp = tmp.offset(1);
            }
        }
        i += 1;
        cur = (*cur).next;
    }
    i = 0 as i32;
    while i < nbchild {
        if !(*list.offset(i as isize)).is_null() {
            j = 0 as i32;
            while j < i {
                if !(*list.offset(j as isize)).is_null() {
                    ret = xmlRelaxNGCompareElemDefLists(
                        ctxt,
                        *list.offset(i as isize),
                        *list.offset(j as isize),
                    );
                    if ret == 0 as i32 {
                        is_indeterminist = 1 as i32;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    i = 0 as i32;
    while i < nbchild {
        if !(*list.offset(i as isize)).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )(*list.offset(i as isize) as *mut libc::c_void);
        }
        i += 1;
    }
    xmlFree.expect("non-null function pointer")(list as *mut libc::c_void);
    if is_indeterminist != 0 {
        let fresh160 = &mut ((*def).dflags);
        *fresh160 = (*fresh160 as i32 | (1 as i32) << 2 as i32)
            as i16;
    }
    if is_triable == 1 as i32 {
        let fresh161 = &mut ((*def).dflags);
        *fresh161 = (*fresh161 as i32 | (1 as i32) << 4 as i32)
            as i16;
        let fresh162 = &mut ((*def).data);
        *fresh162 = triage as *mut libc::c_void;
    } else if !triage.is_null() {
        xmlHashFree(triage, None);
    }
    let fresh163 = &mut ((*def).dflags);
    *fresh163 = (*fresh163 as i32 | (1 as i32) << 5 as i32)
        as i16;
}
unsafe extern "C" fn xmlRelaxNGCheckGroupAttrs(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut def: xmlRelaxNGDefinePtr,
) {
    let mut list: *mut *mut xmlRelaxNGDefinePtr = 0 as *mut *mut xmlRelaxNGDefinePtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut nbchild: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: i32 = 0;
    if def.is_null()
        || (*def).type_0 as i32 != XML_RELAXNG_GROUP as i32
            && (*def).type_0 as i32 != XML_RELAXNG_ELEMENT as i32
    {
        return;
    }
    if (*def).dflags as i32 & (1 as i32) << 5 as i32 != 0 {
        return;
    }
    if (*ctxt).nbErrors != 0 as i32 {
        return;
    }
    cur = (*def).attrs;
    while !cur.is_null() {
        nbchild += 1;
        cur = (*cur).next;
    }
    cur = (*def).content;
    while !cur.is_null() {
        nbchild += 1;
        cur = (*cur).next;
    }
    list = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (nbchild as u64)
            .wrapping_mul(
                ::std::mem::size_of::<*mut xmlRelaxNGDefinePtr>() as u64,
            ),
    ) as *mut *mut xmlRelaxNGDefinePtr;
    if list.is_null() {
        xmlRngPErrMemory(
            ctxt,
            b"building group\n\0" as *const u8 as *const i8,
        );
        return;
    }
    i = 0 as i32;
    cur = (*def).attrs;
    while !cur.is_null() {
        let fresh164 = &mut (*list.offset(i as isize));
        *fresh164 = xmlRelaxNGGetElements(ctxt, cur, 1 as i32);
        i += 1;
        cur = (*cur).next;
    }
    cur = (*def).content;
    while !cur.is_null() {
        let fresh165 = &mut (*list.offset(i as isize));
        *fresh165 = xmlRelaxNGGetElements(ctxt, cur, 1 as i32);
        i += 1;
        cur = (*cur).next;
    }
    i = 0 as i32;
    while i < nbchild {
        if !(*list.offset(i as isize)).is_null() {
            j = 0 as i32;
            while j < i {
                if !(*list.offset(j as isize)).is_null() {
                    ret = xmlRelaxNGCompareElemDefLists(
                        ctxt,
                        *list.offset(i as isize),
                        *list.offset(j as isize),
                    );
                    if ret == 0 as i32 {
                        xmlRngPErr(
                            ctxt,
                            (*def).node,
                            XML_RNGP_GROUP_ATTR_CONFLICT as i32,
                            b"Attributes conflicts in group\n\0" as *const u8
                                as *const i8,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                        );
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    i = 0 as i32;
    while i < nbchild {
        if !(*list.offset(i as isize)).is_null() {
            xmlFree
                .expect(
                    "non-null function pointer",
                )(*list.offset(i as isize) as *mut libc::c_void);
        }
        i += 1;
    }
    xmlFree.expect("non-null function pointer")(list as *mut libc::c_void);
    let fresh166 = &mut ((*def).dflags);
    *fresh166 = (*fresh166 as i32 | (1 as i32) << 5 as i32)
        as i16;
}
unsafe extern "C" fn xmlRelaxNGComputeInterleaves(
    mut payload: *mut libc::c_void,
    mut data: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    let mut current_block: u64;
    let mut def: xmlRelaxNGDefinePtr = payload as xmlRelaxNGDefinePtr;
    let mut ctxt: xmlRelaxNGParserCtxtPtr = data as xmlRelaxNGParserCtxtPtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: *mut xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefinePtr;
    let mut partitions: xmlRelaxNGPartitionPtr = 0 as xmlRelaxNGPartitionPtr;
    let mut groups: *mut xmlRelaxNGInterleaveGroupPtr = 0
        as *mut xmlRelaxNGInterleaveGroupPtr;
    let mut group: xmlRelaxNGInterleaveGroupPtr = 0 as *mut xmlRelaxNGInterleaveGroup;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut ret: i32 = 0;
    let mut res: i32 = 0;
    let mut nbgroups: i32 = 0 as i32;
    let mut nbchild: i32 = 0 as i32;
    let mut is_mixed: i32 = 0 as i32;
    let mut is_determinist: i32 = 1 as i32;
    if (*ctxt).nbErrors != 0 as i32 {
        return;
    }
    cur = (*def).content;
    while !cur.is_null() {
        nbchild += 1;
        cur = (*cur).next;
    }
    groups = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (nbchild as u64)
            .wrapping_mul(
                ::std::mem::size_of::<xmlRelaxNGInterleaveGroupPtr>() as u64,
            ),
    ) as *mut xmlRelaxNGInterleaveGroupPtr;
    if !groups.is_null() {
        cur = (*def).content;
        loop {
            if cur.is_null() {
                current_block = 15768484401365413375;
                break;
            }
            let fresh167 = &mut (*groups.offset(nbgroups as isize));
            *fresh167 = xmlMalloc
                .expect(
                    "non-null function pointer",
                )(::std::mem::size_of::<xmlRelaxNGInterleaveGroup>() as u64)
                as xmlRelaxNGInterleaveGroupPtr;
            if (*groups.offset(nbgroups as isize)).is_null() {
                current_block = 12972366742276778717;
                break;
            }
            if (*cur).type_0 as i32 == XML_RELAXNG_TEXT as i32 {
                is_mixed += 1;
            }
            let fresh168 = &mut ((**groups.offset(nbgroups as isize)).rule);
            *fresh168 = cur;
            let fresh169 = &mut ((**groups.offset(nbgroups as isize)).defs);
            *fresh169 = xmlRelaxNGGetElements(ctxt, cur, 2 as i32);
            let fresh170 = &mut ((**groups.offset(nbgroups as isize)).attrs);
            *fresh170 = xmlRelaxNGGetElements(ctxt, cur, 1 as i32);
            nbgroups += 1;
            cur = (*cur).next;
        }
        match current_block {
            12972366742276778717 => {}
            _ => {
                partitions = xmlMalloc
                    .expect(
                        "non-null function pointer",
                    )(::std::mem::size_of::<xmlRelaxNGPartition>() as u64)
                    as xmlRelaxNGPartitionPtr;
                if !partitions.is_null() {
                    memset(
                        partitions as *mut libc::c_void,
                        0 as i32,
                        ::std::mem::size_of::<xmlRelaxNGPartition>() as u64,
                    );
                    (*partitions).nbgroups = nbgroups;
                    let fresh171 = &mut ((*partitions).triage);
                    *fresh171 = xmlHashCreate(nbgroups);
                    i = 0 as i32;
                    while i < nbgroups {
                        group = *groups.offset(i as isize);
                        j = i + 1 as i32;
                        while j < nbgroups {
                            if !(*groups.offset(j as isize)).is_null() {
                                ret = xmlRelaxNGCompareElemDefLists(
                                    ctxt,
                                    (*group).defs,
                                    (**groups.offset(j as isize)).defs,
                                );
                                if ret == 0 as i32 {
                                    xmlRngPErr(
                                        ctxt,
                                        (*def).node,
                                        XML_RNGP_ELEM_TEXT_CONFLICT as i32,
                                        b"Element or text conflicts in interleave\n\0" as *const u8
                                            as *const i8,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar,
                                    );
                                }
                                ret = xmlRelaxNGCompareElemDefLists(
                                    ctxt,
                                    (*group).attrs,
                                    (**groups.offset(j as isize)).attrs,
                                );
                                if ret == 0 as i32 {
                                    xmlRngPErr(
                                        ctxt,
                                        (*def).node,
                                        XML_RNGP_ATTR_CONFLICT as i32,
                                        b"Attributes conflicts in interleave\n\0" as *const u8
                                            as *const i8,
                                        0 as *const xmlChar,
                                        0 as *const xmlChar,
                                    );
                                }
                            }
                            j += 1;
                        }
                        tmp = (*group).defs;
                        if !tmp.is_null() && !(*tmp).is_null() {
                            while !(*tmp).is_null() {
                                if (**tmp).type_0 as i32
                                    == XML_RELAXNG_TEXT as i32
                                {
                                    res = xmlHashAddEntry2(
                                        (*partitions).triage,
                                        b"#text\0" as *const u8 as *const i8
                                            as *mut xmlChar,
                                        0 as *const xmlChar,
                                        (i + 1 as i32) as ptrdiff_t as *mut libc::c_void,
                                    );
                                    if res != 0 as i32 {
                                        is_determinist = -(1 as i32);
                                    }
                                } else if (**tmp).type_0 as i32
                                        == XML_RELAXNG_ELEMENT as i32
                                        && !((**tmp).name).is_null()
                                    {
                                    if ((**tmp).ns).is_null()
                                        || *((**tmp).ns).offset(0 as i32 as isize)
                                            as i32 == 0 as i32
                                    {
                                        res = xmlHashAddEntry2(
                                            (*partitions).triage,
                                            (**tmp).name,
                                            0 as *const xmlChar,
                                            (i + 1 as i32) as ptrdiff_t as *mut libc::c_void,
                                        );
                                    } else {
                                        res = xmlHashAddEntry2(
                                            (*partitions).triage,
                                            (**tmp).name,
                                            (**tmp).ns,
                                            (i + 1 as i32) as ptrdiff_t as *mut libc::c_void,
                                        );
                                    }
                                    if res != 0 as i32 {
                                        is_determinist = -(1 as i32);
                                    }
                                } else if (**tmp).type_0 as i32
                                        == XML_RELAXNG_ELEMENT as i32
                                    {
                                    if ((**tmp).ns).is_null()
                                        || *((**tmp).ns).offset(0 as i32 as isize)
                                            as i32 == 0 as i32
                                    {
                                        res = xmlHashAddEntry2(
                                            (*partitions).triage,
                                            b"#any\0" as *const u8 as *const i8
                                                as *mut xmlChar,
                                            0 as *const xmlChar,
                                            (i + 1 as i32) as ptrdiff_t as *mut libc::c_void,
                                        );
                                    } else {
                                        res = xmlHashAddEntry2(
                                            (*partitions).triage,
                                            b"#any\0" as *const u8 as *const i8
                                                as *mut xmlChar,
                                            (**tmp).ns,
                                            (i + 1 as i32) as ptrdiff_t as *mut libc::c_void,
                                        );
                                    }
                                    if !((**tmp).nameClass).is_null() {
                                        is_determinist = 2 as i32;
                                    }
                                    if res != 0 as i32 {
                                        is_determinist = -(1 as i32);
                                    }
                                } else {
                                    is_determinist = -(1 as i32);
                                }
                                tmp = tmp.offset(1);
                            }
                        } else {
                            is_determinist = 0 as i32;
                        }
                        i += 1;
                    }
                    let fresh172 = &mut ((*partitions).groups);
                    *fresh172 = groups;
                    let fresh173 = &mut ((*def).data);
                    *fresh173 = partitions as *mut libc::c_void;
                    if is_mixed != 0 as i32 {
                        let fresh174 = &mut ((*def).dflags);
                        *fresh174 = (*fresh174 as i32
                            | (1 as i32) << 3 as i32) as i16;
                    }
                    if is_determinist == 1 as i32 {
                        (*partitions).flags = 1 as i32;
                    }
                    if is_determinist == 2 as i32 {
                        (*partitions).flags = 1 as i32 | 2 as i32;
                    }
                    return;
                }
            }
        }
    }
    xmlRngPErrMemory(
        ctxt,
        b"in interleave computation\n\0" as *const u8 as *const i8,
    );
    if !groups.is_null() {
        i = 0 as i32;
        while i < nbgroups {
            if !(*groups.offset(i as isize)).is_null() {
                if !((**groups.offset(i as isize)).defs).is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )((**groups.offset(i as isize)).defs as *mut libc::c_void);
                }
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(*groups.offset(i as isize) as *mut libc::c_void);
            }
            i += 1;
        }
        xmlFree.expect("non-null function pointer")(groups as *mut libc::c_void);
    }
    xmlRelaxNGFreePartition(partitions);
}
unsafe extern "C" fn xmlRelaxNGParseInterleave(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    def = xmlRelaxNGNewDefine(ctxt, node);
    if def.is_null() {
        return 0 as xmlRelaxNGDefinePtr;
    }
    (*def).type_0 = XML_RELAXNG_INTERLEAVE;
    if ((*ctxt).interleaves).is_null() {
        let fresh175 = &mut ((*ctxt).interleaves);
        *fresh175 = xmlHashCreate(10 as i32);
    }
    if ((*ctxt).interleaves).is_null() {
        xmlRngPErrMemory(
            ctxt,
            b"create interleaves\n\0" as *const u8 as *const i8,
        );
    } else {
        let mut name: [i8; 32] = [0; 32];
        let fresh176 = &mut ((*ctxt).nbInterleaves);
        let fresh177 = *fresh176;
        *fresh176 = *fresh176 + 1;
        snprintf(
            name.as_mut_ptr(),
            32 as i32 as u64,
            b"interleave%d\0" as *const u8 as *const i8,
            fresh177,
        );
        if xmlHashAddEntry(
            (*ctxt).interleaves,
            name.as_mut_ptr() as *mut xmlChar,
            def as *mut libc::c_void,
        ) < 0 as i32
        {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_INTERLEAVE_ADD as i32,
                b"Failed to add %s to hash table\n\0" as *const u8
                    as *const i8,
                name.as_mut_ptr() as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    }
    child = (*node).children;
    if child.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_INTERLEAVE_NO_CONTENT as i32,
            b"Element interleave is empty\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    }
    while !child.is_null() {
        if !child.is_null() && !((*child).ns).is_null()
            && (*child).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*child).name,
                b"element\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*child).ns).href, xmlRelaxNGNs) != 0
        {
            cur = xmlRelaxNGParseElement(ctxt, child);
        } else {
            cur = xmlRelaxNGParsePattern(ctxt, child);
        }
        if !cur.is_null() {
            let fresh178 = &mut ((*cur).parent);
            *fresh178 = def;
            if last.is_null() {
                last = cur;
                let fresh179 = &mut ((*def).content);
                *fresh179 = last;
            } else {
                let fresh180 = &mut ((*last).next);
                *fresh180 = cur;
                last = cur;
            }
        }
        child = (*child).next;
    }
    return def;
}
unsafe extern "C" fn xmlRelaxNGParseInclude(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> i32 {
    let mut incl: xmlRelaxNGIncludePtr = 0 as *mut xmlRelaxNGInclude;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: i32 = 0 as i32;
    let mut tmp: i32 = 0;
    incl = (*node).psvi as xmlRelaxNGIncludePtr;
    if incl.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_INCLUDE_EMPTY as i32,
            b"Include node has no data\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return -(1 as i32);
    }
    root = xmlDocGetRootElement((*incl).doc as *const xmlDoc);
    if root.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_EMPTY as i32,
            b"Include document is empty\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return -(1 as i32);
    }
    if xmlStrEqual(
        (*root).name,
        b"grammar\0" as *const u8 as *const i8 as *mut xmlChar,
    ) == 0
    {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_GRAMMAR_MISSING as i32,
            b"Include document root is not a grammar\n\0" as *const u8
                as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return -(1 as i32);
    }
    if !((*root).children).is_null() {
        tmp = xmlRelaxNGParseGrammarContent(ctxt, (*root).children);
        if tmp != 0 as i32 {
            ret = -(1 as i32);
        }
    }
    if !((*node).children).is_null() {
        tmp = xmlRelaxNGParseGrammarContent(ctxt, (*node).children);
        if tmp != 0 as i32 {
            ret = -(1 as i32);
        }
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGParseDefine(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> i32 {
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: i32 = 0 as i32;
    let mut tmp: i32 = 0;
    let mut def: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut olddefine: *const xmlChar = 0 as *const xmlChar;
    name = xmlGetProp(
        node as *const xmlNode,
        b"name\0" as *const u8 as *const i8 as *mut xmlChar,
    );
    if name.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_DEFINE_NAME_MISSING as i32,
            b"define has no name\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    } else {
        xmlRelaxNGNormExtSpace(name);
        if xmlValidateNCName(name, 0 as i32) != 0 {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_INVALID_DEFINE_NAME as i32,
                b"define name '%s' is not an NCName\n\0" as *const u8
                    as *const i8,
                name,
                0 as *const xmlChar,
            );
        }
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
            return -(1 as i32);
        }
        (*def).type_0 = XML_RELAXNG_DEF;
        let fresh181 = &mut ((*def).name);
        *fresh181 = name;
        if ((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_DEFINE_EMPTY as i32,
                b"define has no children\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            olddefine = (*ctxt).define;
            let fresh182 = &mut ((*ctxt).define);
            *fresh182 = name;
            let fresh183 = &mut ((*def).content);
            *fresh183 = xmlRelaxNGParsePatterns(
                ctxt,
                (*node).children,
                0 as i32,
            );
            let fresh184 = &mut ((*ctxt).define);
            *fresh184 = olddefine;
        }
        if ((*(*ctxt).grammar).defs).is_null() {
            let fresh185 = &mut ((*(*ctxt).grammar).defs);
            *fresh185 = xmlHashCreate(10 as i32);
        }
        if ((*(*ctxt).grammar).defs).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_DEFINE_CREATE_FAILED as i32,
                b"Could not create definition hash\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            ret = -(1 as i32);
        } else {
            tmp = xmlHashAddEntry(
                (*(*ctxt).grammar).defs,
                name,
                def as *mut libc::c_void,
            );
            if tmp < 0 as i32 {
                let mut prev: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                prev = xmlHashLookup((*(*ctxt).grammar).defs, name)
                    as xmlRelaxNGDefinePtr;
                if prev.is_null() {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_DEFINE_CREATE_FAILED as i32,
                        b"Internal error on define aggregation of %s\n\0" as *const u8
                            as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                    ret = -(1 as i32);
                } else {
                    while !((*prev).nextHash).is_null() {
                        prev = (*prev).nextHash;
                    }
                    let fresh186 = &mut ((*prev).nextHash);
                    *fresh186 = def;
                }
            }
        }
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGParseImportRef(
    mut payload: *mut libc::c_void,
    mut data: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    let mut ctxt: xmlRelaxNGParserCtxtPtr = data as xmlRelaxNGParserCtxtPtr;
    let mut def: xmlRelaxNGDefinePtr = payload as xmlRelaxNGDefinePtr;
    let mut tmp: i32 = 0;
    let fresh187 = &mut ((*def).dflags);
    *fresh187 = (*fresh187 as i32 | (1 as i32) << 8 as i32)
        as i16;
    tmp = xmlHashAddEntry((*(*ctxt).grammar).refs, name, def as *mut libc::c_void);
    if tmp < 0 as i32 {
        let mut prev: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        prev = xmlHashLookup((*(*ctxt).grammar).refs, (*def).name)
            as xmlRelaxNGDefinePtr;
        if prev.is_null() {
            if !((*def).name).is_null() {
                xmlRngPErr(
                    ctxt,
                    0 as xmlNodePtr,
                    XML_RNGP_REF_CREATE_FAILED as i32,
                    b"Error refs definitions '%s'\n\0" as *const u8
                        as *const i8,
                    (*def).name,
                    0 as *const xmlChar,
                );
            } else {
                xmlRngPErr(
                    ctxt,
                    0 as xmlNodePtr,
                    XML_RNGP_REF_CREATE_FAILED as i32,
                    b"Error refs definitions\n\0" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
        } else {
            let fresh188 = &mut ((*def).nextHash);
            *fresh188 = (*prev).nextHash;
            let fresh189 = &mut ((*prev).nextHash);
            *fresh189 = def;
        }
    }
}
unsafe extern "C" fn xmlRelaxNGParseImportRefs(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut grammar: xmlRelaxNGGrammarPtr,
) -> i32 {
    if ctxt.is_null() || grammar.is_null() || ((*ctxt).grammar).is_null() {
        return -(1 as i32);
    }
    if ((*grammar).refs).is_null() {
        return 0 as i32;
    }
    if ((*(*ctxt).grammar).refs).is_null() {
        let fresh190 = &mut ((*(*ctxt).grammar).refs);
        *fresh190 = xmlHashCreate(10 as i32);
    }
    if ((*(*ctxt).grammar).refs).is_null() {
        xmlRngPErr(
            ctxt,
            0 as xmlNodePtr,
            XML_RNGP_REF_CREATE_FAILED as i32,
            b"Could not create references hash\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return -(1 as i32);
    }
    xmlHashScan(
        (*grammar).refs,
        Some(
            xmlRelaxNGParseImportRef
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *const xmlChar,
                ) -> (),
        ),
        ctxt as *mut libc::c_void,
    );
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGProcessExternalRef(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGDefinePtr {
    let mut docu: xmlRelaxNGDocumentPtr = 0 as *mut xmlRelaxNGDocument;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut ns: *mut xmlChar = 0 as *mut xmlChar;
    let mut newNs: i32 = 0 as i32;
    let mut oldflags: i32 = 0;
    let mut def: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    docu = (*node).psvi as xmlRelaxNGDocumentPtr;
    if !docu.is_null() {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_EXTERNALREF;
        if ((*docu).content).is_null() {
            root = xmlDocGetRootElement((*docu).doc as *const xmlDoc);
            if root.is_null() {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_EXTERNALREF_EMTPY as i32,
                    b"xmlRelaxNGParse: %s is empty\n\0" as *const u8
                        as *const i8,
                    (*ctxt).URL,
                    0 as *const xmlChar,
                );
                return 0 as xmlRelaxNGDefinePtr;
            }
            ns = xmlGetProp(
                root as *const xmlNode,
                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            if ns.is_null() {
                tmp = node;
                while !tmp.is_null()
                    && (*tmp).type_0 as u32
                        == XML_ELEMENT_NODE as i32 as u32
                {
                    ns = xmlGetProp(
                        tmp as *const xmlNode,
                        b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                    );
                    if !ns.is_null() {
                        break;
                    }
                    tmp = (*tmp).parent;
                }
                if !ns.is_null() {
                    xmlSetProp(
                        root,
                        b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                        ns,
                    );
                    newNs = 1 as i32;
                    xmlFree.expect("non-null function pointer")(ns as *mut libc::c_void);
                }
            } else {
                xmlFree.expect("non-null function pointer")(ns as *mut libc::c_void);
            }
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= (1 as i32) << 7 as i32;
            let fresh191 = &mut ((*docu).schema);
            *fresh191 = xmlRelaxNGParseDocument(ctxt, root);
            (*ctxt).flags = oldflags;
            if !((*docu).schema).is_null() && !((*(*docu).schema).topgrammar).is_null() {
                let fresh192 = &mut ((*docu).content);
                *fresh192 = (*(*(*docu).schema).topgrammar).start;
                if !((*(*(*docu).schema).topgrammar).refs).is_null() {
                    xmlRelaxNGParseImportRefs(ctxt, (*(*docu).schema).topgrammar);
                }
            }
            if newNs == 1 as i32 {
                xmlUnsetProp(
                    root,
                    b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                );
            }
        }
        let fresh193 = &mut ((*def).content);
        *fresh193 = (*docu).content;
    } else {
        def = 0 as xmlRelaxNGDefinePtr;
    }
    return def;
}
unsafe extern "C" fn xmlRelaxNGParsePattern(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    if node.is_null() {
        return 0 as xmlRelaxNGDefinePtr;
    }
    if !node.is_null() && !((*node).ns).is_null()
        && (*node).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        && xmlStrEqual(
            (*node).name,
            b"element\0" as *const u8 as *const i8 as *const xmlChar,
        ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
    {
        def = xmlRelaxNGParseElement(ctxt, node);
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"attribute\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGParseAttribute(ctxt, node);
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"empty\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_EMPTY;
        if !((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_EMPTY_NOT_EMPTY as i32,
                b"empty: had a child node\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"text\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_TEXT;
        if !((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_TEXT_HAS_CHILD as i32,
                b"text: had a child node\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"zeroOrMore\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_ZEROORMORE;
        if ((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_EMPTY_CONSTRUCT as i32,
                b"Element %s is empty\n\0" as *const u8 as *const i8,
                (*node).name,
                0 as *const xmlChar,
            );
        } else {
            let fresh194 = &mut ((*def).content);
            *fresh194 = xmlRelaxNGParsePatterns(
                ctxt,
                (*node).children,
                1 as i32,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"oneOrMore\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_ONEORMORE;
        if ((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_EMPTY_CONSTRUCT as i32,
                b"Element %s is empty\n\0" as *const u8 as *const i8,
                (*node).name,
                0 as *const xmlChar,
            );
        } else {
            let fresh195 = &mut ((*def).content);
            *fresh195 = xmlRelaxNGParsePatterns(
                ctxt,
                (*node).children,
                1 as i32,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"optional\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_OPTIONAL;
        if ((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_EMPTY_CONSTRUCT as i32,
                b"Element %s is empty\n\0" as *const u8 as *const i8,
                (*node).name,
                0 as *const xmlChar,
            );
        } else {
            let fresh196 = &mut ((*def).content);
            *fresh196 = xmlRelaxNGParsePatterns(
                ctxt,
                (*node).children,
                1 as i32,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"choice\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_CHOICE;
        if ((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_EMPTY_CONSTRUCT as i32,
                b"Element %s is empty\n\0" as *const u8 as *const i8,
                (*node).name,
                0 as *const xmlChar,
            );
        } else {
            let fresh197 = &mut ((*def).content);
            *fresh197 = xmlRelaxNGParsePatterns(
                ctxt,
                (*node).children,
                0 as i32,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"group\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_GROUP;
        if ((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_EMPTY_CONSTRUCT as i32,
                b"Element %s is empty\n\0" as *const u8 as *const i8,
                (*node).name,
                0 as *const xmlChar,
            );
        } else {
            let fresh198 = &mut ((*def).content);
            *fresh198 = xmlRelaxNGParsePatterns(
                ctxt,
                (*node).children,
                0 as i32,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"ref\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_REF;
        let fresh199 = &mut ((*def).name);
        *fresh199 = xmlGetProp(
            node as *const xmlNode,
            b"name\0" as *const u8 as *const i8 as *mut xmlChar,
        );
        if ((*def).name).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_REF_NO_NAME as i32,
                b"ref has no name\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            xmlRelaxNGNormExtSpace((*def).name);
            if xmlValidateNCName((*def).name, 0 as i32) != 0 {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_REF_NAME_INVALID as i32,
                    b"ref name '%s' is not an NCName\n\0" as *const u8
                        as *const i8,
                    (*def).name,
                    0 as *const xmlChar,
                );
            }
        }
        if !((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_REF_NOT_EMPTY as i32,
                b"ref is not empty\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
        if ((*(*ctxt).grammar).refs).is_null() {
            let fresh200 = &mut ((*(*ctxt).grammar).refs);
            *fresh200 = xmlHashCreate(10 as i32);
        }
        if ((*(*ctxt).grammar).refs).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_REF_CREATE_FAILED as i32,
                b"Could not create references hash\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            def = 0 as xmlRelaxNGDefinePtr;
        } else {
            let mut tmp: i32 = 0;
            tmp = xmlHashAddEntry(
                (*(*ctxt).grammar).refs,
                (*def).name,
                def as *mut libc::c_void,
            );
            if tmp < 0 as i32 {
                let mut prev: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                prev = xmlHashLookup((*(*ctxt).grammar).refs, (*def).name)
                    as xmlRelaxNGDefinePtr;
                if prev.is_null() {
                    if !((*def).name).is_null() {
                        xmlRngPErr(
                            ctxt,
                            node,
                            XML_RNGP_REF_CREATE_FAILED as i32,
                            b"Error refs definitions '%s'\n\0" as *const u8
                                as *const i8,
                            (*def).name,
                            0 as *const xmlChar,
                        );
                    } else {
                        xmlRngPErr(
                            ctxt,
                            node,
                            XML_RNGP_REF_CREATE_FAILED as i32,
                            b"Error refs definitions\n\0" as *const u8
                                as *const i8,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                        );
                    }
                    def = 0 as xmlRelaxNGDefinePtr;
                } else {
                    let fresh201 = &mut ((*def).nextHash);
                    *fresh201 = (*prev).nextHash;
                    let fresh202 = &mut ((*prev).nextHash);
                    *fresh202 = def;
                }
            }
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"data\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGParseData(ctxt, node);
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"value\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGParseValue(ctxt, node);
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"list\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_LIST;
        if ((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_EMPTY_CONSTRUCT as i32,
                b"Element %s is empty\n\0" as *const u8 as *const i8,
                (*node).name,
                0 as *const xmlChar,
            );
        } else {
            let fresh203 = &mut ((*def).content);
            *fresh203 = xmlRelaxNGParsePatterns(
                ctxt,
                (*node).children,
                0 as i32,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"interleave\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGParseInterleave(ctxt, node);
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"externalRef\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGProcessExternalRef(ctxt, node);
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"notAllowed\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_NOT_ALLOWED;
        if !((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_NOTALLOWED_NOT_EMPTY as i32,
                b"xmlRelaxNGParse: notAllowed element is not empty\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"grammar\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        let mut grammar: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        let mut old: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        let mut oldparent: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        oldparent = (*ctxt).parentgrammar;
        old = (*ctxt).grammar;
        let fresh204 = &mut ((*ctxt).parentgrammar);
        *fresh204 = old;
        grammar = xmlRelaxNGParseGrammar(ctxt, (*node).children);
        if !old.is_null() {
            let fresh205 = &mut ((*ctxt).grammar);
            *fresh205 = old;
            let fresh206 = &mut ((*ctxt).parentgrammar);
            *fresh206 = oldparent;
        }
        if !grammar.is_null() {
            def = (*grammar).start;
        } else {
            def = 0 as xmlRelaxNGDefinePtr;
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"parentRef\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        if ((*ctxt).parentgrammar).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_PARENTREF_NO_PARENT as i32,
                b"Use of parentRef without a parent grammar\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            return 0 as xmlRelaxNGDefinePtr;
        }
        def = xmlRelaxNGNewDefine(ctxt, node);
        if def.is_null() {
            return 0 as xmlRelaxNGDefinePtr;
        }
        (*def).type_0 = XML_RELAXNG_PARENTREF;
        let fresh207 = &mut ((*def).name);
        *fresh207 = xmlGetProp(
            node as *const xmlNode,
            b"name\0" as *const u8 as *const i8 as *mut xmlChar,
        );
        if ((*def).name).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_PARENTREF_NO_NAME as i32,
                b"parentRef has no name\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            xmlRelaxNGNormExtSpace((*def).name);
            if xmlValidateNCName((*def).name, 0 as i32) != 0 {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_PARENTREF_NAME_INVALID as i32,
                    b"parentRef name '%s' is not an NCName\n\0" as *const u8
                        as *const i8,
                    (*def).name,
                    0 as *const xmlChar,
                );
            }
        }
        if !((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_PARENTREF_NOT_EMPTY as i32,
                b"parentRef is not empty\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
        if ((*(*ctxt).parentgrammar).refs).is_null() {
            let fresh208 = &mut ((*(*ctxt).parentgrammar).refs);
            *fresh208 = xmlHashCreate(10 as i32);
        }
        if ((*(*ctxt).parentgrammar).refs).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_PARENTREF_CREATE_FAILED as i32,
                b"Could not create references hash\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            def = 0 as xmlRelaxNGDefinePtr;
        } else if !((*def).name).is_null() {
            let mut tmp_0: i32 = 0;
            tmp_0 = xmlHashAddEntry(
                (*(*ctxt).parentgrammar).refs,
                (*def).name,
                def as *mut libc::c_void,
            );
            if tmp_0 < 0 as i32 {
                let mut prev_0: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                prev_0 = xmlHashLookup((*(*ctxt).parentgrammar).refs, (*def).name)
                    as xmlRelaxNGDefinePtr;
                if prev_0.is_null() {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_PARENTREF_CREATE_FAILED as i32,
                        b"Internal error parentRef definitions '%s'\n\0" as *const u8
                            as *const i8,
                        (*def).name,
                        0 as *const xmlChar,
                    );
                    def = 0 as xmlRelaxNGDefinePtr;
                } else {
                    let fresh209 = &mut ((*def).nextHash);
                    *fresh209 = (*prev_0).nextHash;
                    let fresh210 = &mut ((*prev_0).nextHash);
                    *fresh210 = def;
                }
            }
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"mixed\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        if ((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_EMPTY_CONSTRUCT as i32,
                b"Mixed is empty\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            def = 0 as xmlRelaxNGDefinePtr;
        } else {
            def = xmlRelaxNGParseInterleave(ctxt, node);
            if !def.is_null() {
                let mut tmp_1: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                if !((*def).content).is_null() && !((*(*def).content).next).is_null() {
                    tmp_1 = xmlRelaxNGNewDefine(ctxt, node);
                    if !tmp_1.is_null() {
                        (*tmp_1).type_0 = XML_RELAXNG_GROUP;
                        let fresh211 = &mut ((*tmp_1).content);
                        *fresh211 = (*def).content;
                        let fresh212 = &mut ((*def).content);
                        *fresh212 = tmp_1;
                    }
                }
                tmp_1 = xmlRelaxNGNewDefine(ctxt, node);
                if tmp_1.is_null() {
                    return def;
                }
                (*tmp_1).type_0 = XML_RELAXNG_TEXT;
                let fresh213 = &mut ((*tmp_1).next);
                *fresh213 = (*def).content;
                let fresh214 = &mut ((*def).content);
                *fresh214 = tmp_1;
            }
        }
    } else {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_UNKNOWN_CONSTRUCT as i32,
            b"Unexpected node %s is not a pattern\n\0" as *const u8
                as *const i8,
            (*node).name,
            0 as *const xmlChar,
        );
        def = 0 as xmlRelaxNGDefinePtr;
    }
    return def;
}
unsafe extern "C" fn xmlRelaxNGParseAttribute(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    let mut old_flags: i32 = 0;
    ret = xmlRelaxNGNewDefine(ctxt, node);
    if ret.is_null() {
        return 0 as xmlRelaxNGDefinePtr;
    }
    (*ret).type_0 = XML_RELAXNG_ATTRIBUTE;
    let fresh215 = &mut ((*ret).parent);
    *fresh215 = (*ctxt).def;
    child = (*node).children;
    if child.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_ATTRIBUTE_EMPTY as i32,
            b"xmlRelaxNGParseattribute: attribute has no children\n\0" as *const u8
                as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return ret;
    }
    old_flags = (*ctxt).flags;
    (*ctxt).flags |= (1 as i32) << 0 as i32;
    cur = xmlRelaxNGParseNameClass(ctxt, child, ret);
    if !cur.is_null() {
        child = (*child).next;
    }
    if !child.is_null() {
        cur = xmlRelaxNGParsePattern(ctxt, child);
        if !cur.is_null() {
            match (*cur).type_0 as i32 {
                0 | 1 | 3 | 4 | 5 | 7 | 8 | 11 | 13 | 12 | 10 | 16 | 15 | 14 | 17 | 18
                | 19 | 9 => {
                    let fresh216 = &mut ((*ret).content);
                    *fresh216 = cur;
                    let fresh217 = &mut ((*cur).parent);
                    *fresh217 = ret;
                }
                20 | 6 | 2 => {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_ATTRIBUTE_CONTENT as i32,
                        b"attribute has invalid content\n\0" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
                -1 => {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_ATTRIBUTE_NOOP as i32,
                        b"RNG Internal error, noop found in attribute\n\0" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
                _ => {}
            }
        }
        child = (*child).next;
    }
    if !child.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_ATTRIBUTE_CHILDREN as i32,
            b"attribute has multiple children\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    }
    (*ctxt).flags = old_flags;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGParseExceptNameClass(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
    mut attr: i32,
) -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    if !(!node.is_null() && !((*node).ns).is_null()
        && (*node).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        && xmlStrEqual(
            (*node).name,
            b"except\0" as *const u8 as *const i8 as *const xmlChar,
        ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0)
    {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_EXCEPT_MISSING as i32,
            b"Expecting an except node\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return 0 as xmlRelaxNGDefinePtr;
    }
    if !((*node).next).is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_EXCEPT_MULTIPLE as i32,
            b"exceptNameClass allows only a single except node\n\0" as *const u8
                as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    }
    if ((*node).children).is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_EXCEPT_EMPTY as i32,
            b"except has no content\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return 0 as xmlRelaxNGDefinePtr;
    }
    ret = xmlRelaxNGNewDefine(ctxt, node);
    if ret.is_null() {
        return 0 as xmlRelaxNGDefinePtr;
    }
    (*ret).type_0 = XML_RELAXNG_EXCEPT;
    child = (*node).children;
    while !child.is_null() {
        cur = xmlRelaxNGNewDefine(ctxt, child);
        if cur.is_null() {
            break;
        }
        if attr != 0 {
            (*cur).type_0 = XML_RELAXNG_ATTRIBUTE;
        } else {
            (*cur).type_0 = XML_RELAXNG_ELEMENT;
        }
        if !(xmlRelaxNGParseNameClass(ctxt, child, cur)).is_null() {
            if last.is_null() {
                let fresh218 = &mut ((*ret).content);
                *fresh218 = cur;
            } else {
                let fresh219 = &mut ((*last).next);
                *fresh219 = cur;
            }
            last = cur;
        }
        child = (*child).next;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGParseNameClass(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
    mut def: xmlRelaxNGDefinePtr,
) -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut val: *mut xmlChar = 0 as *mut xmlChar;
    ret = def;
    if !node.is_null() && !((*node).ns).is_null()
        && (*node).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        && xmlStrEqual(
            (*node).name,
            b"name\0" as *const u8 as *const i8 as *const xmlChar,
        ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        || !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"anyName\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        || !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"nsName\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
    {
        if (*def).type_0 as i32 != XML_RELAXNG_ELEMENT as i32
            && (*def).type_0 as i32 != XML_RELAXNG_ATTRIBUTE as i32
        {
            ret = xmlRelaxNGNewDefine(ctxt, node);
            if ret.is_null() {
                return 0 as xmlRelaxNGDefinePtr;
            }
            let fresh220 = &mut ((*ret).parent);
            *fresh220 = def;
            if (*ctxt).flags & (1 as i32) << 0 as i32 != 0 {
                (*ret).type_0 = XML_RELAXNG_ATTRIBUTE;
            } else {
                (*ret).type_0 = XML_RELAXNG_ELEMENT;
            }
        }
    }
    if !node.is_null() && !((*node).ns).is_null()
        && (*node).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        && xmlStrEqual(
            (*node).name,
            b"name\0" as *const u8 as *const i8 as *const xmlChar,
        ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
    {
        val = xmlNodeGetContent(node as *const xmlNode);
        xmlRelaxNGNormExtSpace(val);
        if xmlValidateNCName(val, 0 as i32) != 0 {
            if !((*node).parent).is_null() {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_ELEMENT_NAME as i32,
                    b"Element %s name '%s' is not an NCName\n\0" as *const u8
                        as *const i8,
                    (*(*node).parent).name,
                    val,
                );
            } else {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_ELEMENT_NAME as i32,
                    b"name '%s' is not an NCName\n\0" as *const u8
                        as *const i8,
                    val,
                    0 as *const xmlChar,
                );
            }
        }
        let fresh221 = &mut ((*ret).name);
        *fresh221 = val;
        val = xmlGetProp(
            node as *const xmlNode,
            b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
        );
        let fresh222 = &mut ((*ret).ns);
        *fresh222 = val;
        if (*ctxt).flags & (1 as i32) << 0 as i32 != 0 && !val.is_null()
            && xmlStrEqual(
                val,
                b"http://www.w3.org/2000/xmlns\0" as *const u8 as *const i8
                    as *mut xmlChar,
            ) != 0
        {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_XML_NS as i32,
                b"Attribute with namespace '%s' is not allowed\n\0" as *const u8
                    as *const i8,
                val,
                0 as *const xmlChar,
            );
        }
        if (*ctxt).flags & (1 as i32) << 0 as i32 != 0 && !val.is_null()
            && *val.offset(0 as i32 as isize) as i32 == 0 as i32
            && xmlStrEqual(
                (*ret).name,
                b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
        {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_XMLNS_NAME as i32,
                b"Attribute with QName 'xmlns' is not allowed\n\0" as *const u8
                    as *const i8,
                val,
                0 as *const xmlChar,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"anyName\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        let fresh223 = &mut ((*ret).name);
        *fresh223 = 0 as *mut xmlChar;
        let fresh224 = &mut ((*ret).ns);
        *fresh224 = 0 as *mut xmlChar;
        if !((*node).children).is_null() {
            let fresh225 = &mut ((*ret).nameClass);
            *fresh225 = xmlRelaxNGParseExceptNameClass(
                ctxt,
                (*node).children,
                ((*def).type_0 as i32 == XML_RELAXNG_ATTRIBUTE as i32)
                    as i32,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"nsName\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        let fresh226 = &mut ((*ret).name);
        *fresh226 = 0 as *mut xmlChar;
        let fresh227 = &mut ((*ret).ns);
        *fresh227 = xmlGetProp(
            node as *const xmlNode,
            b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
        );
        if ((*ret).ns).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_NSNAME_NO_NS as i32,
                b"nsName has no ns attribute\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
        if (*ctxt).flags & (1 as i32) << 0 as i32 != 0
            && !((*ret).ns).is_null()
            && xmlStrEqual(
                (*ret).ns,
                b"http://www.w3.org/2000/xmlns\0" as *const u8 as *const i8
                    as *mut xmlChar,
            ) != 0
        {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_XML_NS as i32,
                b"Attribute with namespace '%s' is not allowed\n\0" as *const u8
                    as *const i8,
                (*ret).ns,
                0 as *const xmlChar,
            );
        }
        if !((*node).children).is_null() {
            let fresh228 = &mut ((*ret).nameClass);
            *fresh228 = xmlRelaxNGParseExceptNameClass(
                ctxt,
                (*node).children,
                ((*def).type_0 as i32 == XML_RELAXNG_ATTRIBUTE as i32)
                    as i32,
            );
        }
    } else if !node.is_null() && !((*node).ns).is_null()
            && (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*node).name,
                b"choice\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
        {
        let mut child: xmlNodePtr = 0 as *mut xmlNode;
        let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
        if (*def).type_0 as i32 == XML_RELAXNG_CHOICE as i32 {
            ret = def;
        } else {
            ret = xmlRelaxNGNewDefine(ctxt, node);
            if ret.is_null() {
                return 0 as xmlRelaxNGDefinePtr;
            }
            let fresh229 = &mut ((*ret).parent);
            *fresh229 = def;
            (*ret).type_0 = XML_RELAXNG_CHOICE;
        }
        if ((*node).children).is_null() {
            xmlRngPErr(
                ctxt,
                node,
                XML_RNGP_CHOICE_EMPTY as i32,
                b"Element choice is empty\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            child = (*node).children;
            while !child.is_null() {
                tmp = xmlRelaxNGParseNameClass(ctxt, child, ret);
                if !tmp.is_null() {
                    if last.is_null() {
                        last = tmp;
                    } else {
                        let fresh230 = &mut ((*last).next);
                        *fresh230 = tmp;
                        last = tmp;
                    }
                }
                child = (*child).next;
            }
        }
    } else {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_CHOICE_CONTENT as i32,
            b"expecting name, anyName, nsName or choice : got %s\n\0" as *const u8
                as *const i8,
            if node.is_null() {
                b"nothing\0" as *const u8 as *const i8 as *const xmlChar
            } else {
                (*node).name
            },
            0 as *const xmlChar,
        );
        return 0 as xmlRelaxNGDefinePtr;
    }
    if ret != def {
        if ((*def).nameClass).is_null() {
            let fresh231 = &mut ((*def).nameClass);
            *fresh231 = ret;
        } else {
            tmp = (*def).nameClass;
            while !((*tmp).next).is_null() {
                tmp = (*tmp).next;
            }
            let fresh232 = &mut ((*tmp).next);
            *fresh232 = ret;
        }
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGParseElement(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGDefinePtr {
    let mut ret: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut last: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut child: xmlNodePtr = 0 as *mut xmlNode;
    let mut olddefine: *const xmlChar = 0 as *const xmlChar;
    ret = xmlRelaxNGNewDefine(ctxt, node);
    if ret.is_null() {
        return 0 as xmlRelaxNGDefinePtr;
    }
    (*ret).type_0 = XML_RELAXNG_ELEMENT;
    let fresh233 = &mut ((*ret).parent);
    *fresh233 = (*ctxt).def;
    child = (*node).children;
    if child.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_ELEMENT_EMPTY as i32,
            b"xmlRelaxNGParseElement: element has no children\n\0" as *const u8
                as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return ret;
    }
    cur = xmlRelaxNGParseNameClass(ctxt, child, ret);
    if !cur.is_null() {
        child = (*child).next;
    }
    if child.is_null() {
        xmlRngPErr(
            ctxt,
            node,
            XML_RNGP_ELEMENT_NO_CONTENT as i32,
            b"xmlRelaxNGParseElement: element has no content\n\0" as *const u8
                as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return ret;
    }
    olddefine = (*ctxt).define;
    let fresh234 = &mut ((*ctxt).define);
    *fresh234 = 0 as *const xmlChar;
    last = 0 as xmlRelaxNGDefinePtr;
    while !child.is_null() {
        cur = xmlRelaxNGParsePattern(ctxt, child);
        if !cur.is_null() {
            let fresh235 = &mut ((*cur).parent);
            *fresh235 = ret;
            match (*cur).type_0 as i32 {
                0 | 1 | 3 | 4 | 5 | 7 | 8 | 11 | 13 | 12 | 10 | 15 | 16 | 14 | 17 | 18
                | 19 => {
                    if last.is_null() {
                        last = cur;
                        let fresh236 = &mut ((*ret).content);
                        *fresh236 = last;
                    } else {
                        if (*last).type_0 as i32
                            == XML_RELAXNG_ELEMENT as i32
                            && (*ret).content == last
                        {
                            let fresh237 = &mut ((*ret).content);
                            *fresh237 = xmlRelaxNGNewDefine(ctxt, node);
                            if !((*ret).content).is_null() {
                                (*(*ret).content).type_0 = XML_RELAXNG_GROUP;
                                let fresh238 = &mut ((*(*ret).content).content);
                                *fresh238 = last;
                            } else {
                                let fresh239 = &mut ((*ret).content);
                                *fresh239 = last;
                            }
                        }
                        let fresh240 = &mut ((*last).next);
                        *fresh240 = cur;
                        last = cur;
                    }
                }
                9 => {
                    let fresh241 = &mut ((*cur).next);
                    *fresh241 = (*ret).attrs;
                    let fresh242 = &mut ((*ret).attrs);
                    *fresh242 = cur;
                }
                20 => {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_ELEMENT_CONTENT as i32,
                        b"RNG Internal error, start found in element\n\0" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
                6 => {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_ELEMENT_CONTENT as i32,
                        b"RNG Internal error, param found in element\n\0" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
                2 => {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_ELEMENT_CONTENT as i32,
                        b"RNG Internal error, except found in element\n\0" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
                -1 => {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_ELEMENT_CONTENT as i32,
                        b"RNG Internal error, noop found in element\n\0" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
                _ => {}
            }
        }
        child = (*child).next;
    }
    let fresh243 = &mut ((*ctxt).define);
    *fresh243 = olddefine;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGParsePatterns(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut nodes: xmlNodePtr,
    mut group: i32,
) -> xmlRelaxNGDefinePtr {
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut last: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut parent: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    parent = (*ctxt).def;
    while !nodes.is_null() {
        if !nodes.is_null() && !((*nodes).ns).is_null()
            && (*nodes).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*nodes).name,
                b"element\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0
        {
            cur = xmlRelaxNGParseElement(ctxt, nodes);
            if cur.is_null() {
                return 0 as xmlRelaxNGDefinePtr;
            }
            if def.is_null() {
                last = cur;
                def = last;
            } else {
                if group == 1 as i32
                    && (*def).type_0 as i32 == XML_RELAXNG_ELEMENT as i32
                    && def == last
                {
                    def = xmlRelaxNGNewDefine(ctxt, nodes);
                    if def.is_null() {
                        return 0 as xmlRelaxNGDefinePtr;
                    }
                    (*def).type_0 = XML_RELAXNG_GROUP;
                    let fresh244 = &mut ((*def).content);
                    *fresh244 = last;
                }
                let fresh245 = &mut ((*last).next);
                *fresh245 = cur;
                last = cur;
            }
            let fresh246 = &mut ((*cur).parent);
            *fresh246 = parent;
        } else {
            cur = xmlRelaxNGParsePattern(ctxt, nodes);
            if !cur.is_null() {
                if def.is_null() {
                    last = cur;
                    def = last;
                } else {
                    let fresh247 = &mut ((*last).next);
                    *fresh247 = cur;
                    last = cur;
                }
            }
        }
        nodes = (*nodes).next;
    }
    return def;
}
unsafe extern "C" fn xmlRelaxNGParseStart(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut nodes: xmlNodePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut def: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    let mut last: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if nodes.is_null() {
        xmlRngPErr(
            ctxt,
            nodes,
            XML_RNGP_START_EMPTY as i32,
            b"start has no children\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return -(1 as i32);
    }
    if !nodes.is_null() && !((*nodes).ns).is_null()
        && (*nodes).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        && xmlStrEqual(
            (*nodes).name,
            b"empty\0" as *const u8 as *const i8 as *const xmlChar,
        ) != 0 && xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0
    {
        def = xmlRelaxNGNewDefine(ctxt, nodes);
        if def.is_null() {
            return -(1 as i32);
        }
        (*def).type_0 = XML_RELAXNG_EMPTY;
        if !((*nodes).children).is_null() {
            xmlRngPErr(
                ctxt,
                nodes,
                XML_RNGP_EMPTY_CONTENT as i32,
                b"element empty is not empty\n\0" as *const u8 as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    } else if !nodes.is_null() && !((*nodes).ns).is_null()
            && (*nodes).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*nodes).name,
                b"notAllowed\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0
        {
        def = xmlRelaxNGNewDefine(ctxt, nodes);
        if def.is_null() {
            return -(1 as i32);
        }
        (*def).type_0 = XML_RELAXNG_NOT_ALLOWED;
        if !((*nodes).children).is_null() {
            xmlRngPErr(
                ctxt,
                nodes,
                XML_RNGP_NOTALLOWED_NOT_EMPTY as i32,
                b"element notAllowed is not empty\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
    } else {
        def = xmlRelaxNGParsePatterns(ctxt, nodes, 1 as i32);
    }
    if !((*(*ctxt).grammar).start).is_null() {
        last = (*(*ctxt).grammar).start;
        while !((*last).next).is_null() {
            last = (*last).next;
        }
        let fresh248 = &mut ((*last).next);
        *fresh248 = def;
    } else {
        let fresh249 = &mut ((*(*ctxt).grammar).start);
        *fresh249 = def;
    }
    nodes = (*nodes).next;
    if !nodes.is_null() {
        xmlRngPErr(
            ctxt,
            nodes,
            XML_RNGP_START_CONTENT as i32,
            b"start more than one children\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return -(1 as i32);
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGParseGrammarContent(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut nodes: xmlNodePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut tmp: i32 = 0;
    if nodes.is_null() {
        xmlRngPErr(
            ctxt,
            nodes,
            XML_RNGP_GRAMMAR_EMPTY as i32,
            b"grammar has no children\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return -(1 as i32);
    }
    while !nodes.is_null() {
        if !nodes.is_null() && !((*nodes).ns).is_null()
            && (*nodes).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            && xmlStrEqual(
                (*nodes).name,
                b"start\0" as *const u8 as *const i8 as *const xmlChar,
            ) != 0 && xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0
        {
            if ((*nodes).children).is_null() {
                xmlRngPErr(
                    ctxt,
                    nodes,
                    XML_RNGP_START_EMPTY as i32,
                    b"start has no children\n\0" as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            } else {
                tmp = xmlRelaxNGParseStart(ctxt, (*nodes).children);
                if tmp != 0 as i32 {
                    ret = -(1 as i32);
                }
            }
        } else if !nodes.is_null() && !((*nodes).ns).is_null()
                && (*nodes).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                && xmlStrEqual(
                    (*nodes).name,
                    b"define\0" as *const u8 as *const i8 as *const xmlChar,
                ) != 0 && xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0
            {
            tmp = xmlRelaxNGParseDefine(ctxt, nodes);
            if tmp != 0 as i32 {
                ret = -(1 as i32);
            }
        } else if !nodes.is_null() && !((*nodes).ns).is_null()
                && (*nodes).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                && xmlStrEqual(
                    (*nodes).name,
                    b"include\0" as *const u8 as *const i8 as *const xmlChar,
                ) != 0 && xmlStrEqual((*(*nodes).ns).href, xmlRelaxNGNs) != 0
            {
            tmp = xmlRelaxNGParseInclude(ctxt, nodes);
            if tmp != 0 as i32 {
                ret = -(1 as i32);
            }
        } else {
            xmlRngPErr(
                ctxt,
                nodes,
                XML_RNGP_GRAMMAR_CONTENT as i32,
                b"grammar has unexpected child %s\n\0" as *const u8
                    as *const i8,
                (*nodes).name,
                0 as *const xmlChar,
            );
            ret = -(1 as i32);
        }
        nodes = (*nodes).next;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGCheckReference(
    mut payload: *mut libc::c_void,
    mut data: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    let mut ref_0: xmlRelaxNGDefinePtr = payload as xmlRelaxNGDefinePtr;
    let mut ctxt: xmlRelaxNGParserCtxtPtr = data as xmlRelaxNGParserCtxtPtr;
    let mut grammar: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    let mut def: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if (*ref_0).dflags as i32 & (1 as i32) << 8 as i32 != 0 {
        return;
    }
    grammar = (*ctxt).grammar;
    if grammar.is_null() {
        xmlRngPErr(
            ctxt,
            (*ref_0).node,
            XML_ERR_INTERNAL_ERROR as i32,
            b"Internal error: no grammar in CheckReference %s\n\0" as *const u8
                as *const i8,
            name,
            0 as *const xmlChar,
        );
        return;
    }
    if !((*ref_0).content).is_null() {
        xmlRngPErr(
            ctxt,
            (*ref_0).node,
            XML_ERR_INTERNAL_ERROR as i32,
            b"Internal error: reference has content in CheckReference %s\n\0"
                as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
        return;
    }
    if !((*grammar).defs).is_null() {
        def = xmlHashLookup((*grammar).defs, name) as xmlRelaxNGDefinePtr;
        if !def.is_null() {
            cur = ref_0;
            while !cur.is_null() {
                let fresh250 = &mut ((*cur).content);
                *fresh250 = def;
                cur = (*cur).nextHash;
            }
        } else {
            xmlRngPErr(
                ctxt,
                (*ref_0).node,
                XML_RNGP_REF_NO_DEF as i32,
                b"Reference %s has no matching definition\n\0" as *const u8
                    as *const i8,
                name,
                0 as *const xmlChar,
            );
        }
    } else {
        xmlRngPErr(
            ctxt,
            (*ref_0).node,
            XML_RNGP_REF_NO_DEF as i32,
            b"Reference %s has no matching definition\n\0" as *const u8
                as *const i8,
            name,
            0 as *const xmlChar,
        );
    };
}
unsafe extern "C" fn xmlRelaxNGCheckCombine(
    mut payload: *mut libc::c_void,
    mut data: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    let mut define: xmlRelaxNGDefinePtr = payload as xmlRelaxNGDefinePtr;
    let mut ctxt: xmlRelaxNGParserCtxtPtr = data as xmlRelaxNGParserCtxtPtr;
    let mut combine: *mut xmlChar = 0 as *mut xmlChar;
    let mut choiceOrInterleave: i32 = -(1 as i32);
    let mut missing: i32 = 0 as i32;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut last: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut tmp2: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if ((*define).nextHash).is_null() {
        return;
    }
    cur = define;
    while !cur.is_null() {
        combine = xmlGetProp(
            (*cur).node as *const xmlNode,
            b"combine\0" as *const u8 as *const i8 as *mut xmlChar,
        );
        if !combine.is_null() {
            if xmlStrEqual(
                combine,
                b"choice\0" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
            {
                if choiceOrInterleave == -(1 as i32) {
                    choiceOrInterleave = 1 as i32;
                } else if choiceOrInterleave == 0 as i32 {
                    xmlRngPErr(
                        ctxt,
                        (*define).node,
                        XML_RNGP_DEF_CHOICE_AND_INTERLEAVE as i32,
                        b"Defines for %s use both 'choice' and 'interleave'\n\0"
                            as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                }
            } else if xmlStrEqual(
                    combine,
                    b"interleave\0" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                if choiceOrInterleave == -(1 as i32) {
                    choiceOrInterleave = 0 as i32;
                } else if choiceOrInterleave == 1 as i32 {
                    xmlRngPErr(
                        ctxt,
                        (*define).node,
                        XML_RNGP_DEF_CHOICE_AND_INTERLEAVE as i32,
                        b"Defines for %s use both 'choice' and 'interleave'\n\0"
                            as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                }
            } else {
                xmlRngPErr(
                    ctxt,
                    (*define).node,
                    XML_RNGP_UNKNOWN_COMBINE as i32,
                    b"Defines for %s use unknown combine value '%s''\n\0" as *const u8
                        as *const i8,
                    name,
                    combine,
                );
            }
            xmlFree.expect("non-null function pointer")(combine as *mut libc::c_void);
        } else if missing == 0 as i32 {
            missing = 1 as i32;
        } else {
            xmlRngPErr(
                ctxt,
                (*define).node,
                XML_RNGP_NEED_COMBINE as i32,
                b"Some defines for %s needs the combine attribute\n\0" as *const u8
                    as *const i8,
                name,
                0 as *const xmlChar,
            );
        }
        cur = (*cur).nextHash;
    }
    if choiceOrInterleave == -(1 as i32) {
        choiceOrInterleave = 0 as i32;
    }
    cur = xmlRelaxNGNewDefine(ctxt, (*define).node);
    if cur.is_null() {
        return;
    }
    if choiceOrInterleave == 0 as i32 {
        (*cur).type_0 = XML_RELAXNG_INTERLEAVE;
    } else {
        (*cur).type_0 = XML_RELAXNG_CHOICE;
    }
    tmp = define;
    last = 0 as xmlRelaxNGDefinePtr;
    while !tmp.is_null() {
        if !((*tmp).content).is_null() {
            if !((*(*tmp).content).next).is_null() {
                tmp2 = xmlRelaxNGNewDefine(ctxt, (*(*tmp).content).node);
                if tmp2.is_null() {
                    break;
                }
                (*tmp2).type_0 = XML_RELAXNG_GROUP;
                let fresh251 = &mut ((*tmp2).content);
                *fresh251 = (*tmp).content;
            } else {
                tmp2 = (*tmp).content;
            }
            if last.is_null() {
                let fresh252 = &mut ((*cur).content);
                *fresh252 = tmp2;
            } else {
                let fresh253 = &mut ((*last).next);
                *fresh253 = tmp2;
            }
            last = tmp2;
        }
        let fresh254 = &mut ((*tmp).content);
        *fresh254 = cur;
        tmp = (*tmp).nextHash;
    }
    let fresh255 = &mut ((*define).content);
    *fresh255 = cur;
    if choiceOrInterleave == 0 as i32 {
        if ((*ctxt).interleaves).is_null() {
            let fresh256 = &mut ((*ctxt).interleaves);
            *fresh256 = xmlHashCreate(10 as i32);
        }
        if ((*ctxt).interleaves).is_null() {
            xmlRngPErr(
                ctxt,
                (*define).node,
                XML_RNGP_INTERLEAVE_CREATE_FAILED as i32,
                b"Failed to create interleaves hash table\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            let mut tmpname: [i8; 32] = [0; 32];
            let fresh257 = &mut ((*ctxt).nbInterleaves);
            let fresh258 = *fresh257;
            *fresh257 = *fresh257 + 1;
            snprintf(
                tmpname.as_mut_ptr(),
                32 as i32 as u64,
                b"interleave%d\0" as *const u8 as *const i8,
                fresh258,
            );
            if xmlHashAddEntry(
                (*ctxt).interleaves,
                tmpname.as_mut_ptr() as *mut xmlChar,
                cur as *mut libc::c_void,
            ) < 0 as i32
            {
                xmlRngPErr(
                    ctxt,
                    (*define).node,
                    XML_RNGP_INTERLEAVE_CREATE_FAILED as i32,
                    b"Failed to add %s to hash table\n\0" as *const u8
                        as *const i8,
                    tmpname.as_mut_ptr() as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
        }
    }
}
unsafe extern "C" fn xmlRelaxNGCombineStart(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut grammar: xmlRelaxNGGrammarPtr,
) {
    let mut starts: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut combine: *mut xmlChar = 0 as *mut xmlChar;
    let mut choiceOrInterleave: i32 = -(1 as i32);
    let mut missing: i32 = 0 as i32;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    starts = (*grammar).start;
    if starts.is_null() || ((*starts).next).is_null() {
        return;
    }
    cur = starts;
    while !cur.is_null() {
        if ((*cur).node).is_null() || ((*(*cur).node).parent).is_null()
            || xmlStrEqual(
                (*(*(*cur).node).parent).name,
                b"start\0" as *const u8 as *const i8 as *mut xmlChar,
            ) == 0
        {
            combine = 0 as *mut xmlChar;
            xmlRngPErr(
                ctxt,
                (*cur).node,
                XML_RNGP_START_MISSING as i32,
                b"Internal error: start element not found\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            combine = xmlGetProp(
                (*(*cur).node).parent,
                b"combine\0" as *const u8 as *const i8 as *mut xmlChar,
            );
        }
        if !combine.is_null() {
            if xmlStrEqual(
                combine,
                b"choice\0" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
            {
                if choiceOrInterleave == -(1 as i32) {
                    choiceOrInterleave = 1 as i32;
                } else if choiceOrInterleave == 0 as i32 {
                    xmlRngPErr(
                        ctxt,
                        (*cur).node,
                        XML_RNGP_START_CHOICE_AND_INTERLEAVE as i32,
                        b"<start> use both 'choice' and 'interleave'\n\0" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
            } else if xmlStrEqual(
                    combine,
                    b"interleave\0" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                if choiceOrInterleave == -(1 as i32) {
                    choiceOrInterleave = 0 as i32;
                } else if choiceOrInterleave == 1 as i32 {
                    xmlRngPErr(
                        ctxt,
                        (*cur).node,
                        XML_RNGP_START_CHOICE_AND_INTERLEAVE as i32,
                        b"<start> use both 'choice' and 'interleave'\n\0" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
            } else {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_UNKNOWN_COMBINE as i32,
                    b"<start> uses unknown combine value '%s''\n\0" as *const u8
                        as *const i8,
                    combine,
                    0 as *const xmlChar,
                );
            }
            xmlFree.expect("non-null function pointer")(combine as *mut libc::c_void);
        } else if missing == 0 as i32 {
            missing = 1 as i32;
        } else {
            xmlRngPErr(
                ctxt,
                (*cur).node,
                XML_RNGP_NEED_COMBINE as i32,
                b"Some <start> element miss the combine attribute\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        }
        cur = (*cur).next;
    }
    if choiceOrInterleave == -(1 as i32) {
        choiceOrInterleave = 0 as i32;
    }
    cur = xmlRelaxNGNewDefine(ctxt, (*starts).node);
    if cur.is_null() {
        return;
    }
    if choiceOrInterleave == 0 as i32 {
        (*cur).type_0 = XML_RELAXNG_INTERLEAVE;
    } else {
        (*cur).type_0 = XML_RELAXNG_CHOICE;
    }
    let fresh259 = &mut ((*cur).content);
    *fresh259 = (*grammar).start;
    let fresh260 = &mut ((*grammar).start);
    *fresh260 = cur;
    if choiceOrInterleave == 0 as i32 {
        if ((*ctxt).interleaves).is_null() {
            let fresh261 = &mut ((*ctxt).interleaves);
            *fresh261 = xmlHashCreate(10 as i32);
        }
        if ((*ctxt).interleaves).is_null() {
            xmlRngPErr(
                ctxt,
                (*cur).node,
                XML_RNGP_INTERLEAVE_CREATE_FAILED as i32,
                b"Failed to create interleaves hash table\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
        } else {
            let mut tmpname: [i8; 32] = [0; 32];
            let fresh262 = &mut ((*ctxt).nbInterleaves);
            let fresh263 = *fresh262;
            *fresh262 = *fresh262 + 1;
            snprintf(
                tmpname.as_mut_ptr(),
                32 as i32 as u64,
                b"interleave%d\0" as *const u8 as *const i8,
                fresh263,
            );
            if xmlHashAddEntry(
                (*ctxt).interleaves,
                tmpname.as_mut_ptr() as *mut xmlChar,
                cur as *mut libc::c_void,
            ) < 0 as i32
            {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_INTERLEAVE_CREATE_FAILED as i32,
                    b"Failed to add %s to hash table\n\0" as *const u8
                        as *const i8,
                    tmpname.as_mut_ptr() as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
        }
    }
}
unsafe extern "C" fn xmlRelaxNGCheckCycles(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut cur: xmlRelaxNGDefinePtr,
    mut depth: i32,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    while ret == 0 as i32 && !cur.is_null() {
        if (*cur).type_0 as i32 == XML_RELAXNG_REF as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_PARENTREF as i32
        {
            if (*cur).depth as i32 == -(1 as i32) {
                (*cur).depth = depth as i16;
                ret = xmlRelaxNGCheckCycles(ctxt, (*cur).content, depth);
                (*cur).depth = -(2 as i32) as i16;
            } else if depth == (*cur).depth as i32 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_REF_CYCLE as i32,
                    b"Detected a cycle in %s references\n\0" as *const u8
                        as *const i8,
                    (*cur).name,
                    0 as *const xmlChar,
                );
                return -(1 as i32);
            }
        } else if (*cur).type_0 as i32 == XML_RELAXNG_ELEMENT as i32 {
            ret = xmlRelaxNGCheckCycles(ctxt, (*cur).content, depth + 1 as i32);
        } else {
            ret = xmlRelaxNGCheckCycles(ctxt, (*cur).content, depth);
        }
        cur = (*cur).next;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGTryUnlink(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut cur: xmlRelaxNGDefinePtr,
    mut parent: xmlRelaxNGDefinePtr,
    mut prev: xmlRelaxNGDefinePtr,
) -> xmlRelaxNGDefinePtr {
    if !prev.is_null() {
        let fresh264 = &mut ((*prev).next);
        *fresh264 = (*cur).next;
    } else if !parent.is_null() {
        if (*parent).content == cur {
            let fresh265 = &mut ((*parent).content);
            *fresh265 = (*cur).next;
        } else if (*parent).attrs == cur {
            let fresh266 = &mut ((*parent).attrs);
            *fresh266 = (*cur).next;
        } else if (*parent).nameClass == cur {
            let fresh267 = &mut ((*parent).nameClass);
            *fresh267 = (*cur).next;
        }
    } else {
        (*cur).type_0 = XML_RELAXNG_NOOP;
        prev = cur;
    }
    return prev;
}
unsafe extern "C" fn xmlRelaxNGSimplify(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut cur: xmlRelaxNGDefinePtr,
    mut parent: xmlRelaxNGDefinePtr,
) {
    let mut prev: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
    while !cur.is_null() {
        if (*cur).type_0 as i32 == XML_RELAXNG_REF as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_PARENTREF as i32
        {
            if (*cur).depth as i32 != -(3 as i32) {
                (*cur).depth = -(3 as i32) as i16;
                xmlRelaxNGSimplify(ctxt, (*cur).content, cur);
            }
        } else if (*cur).type_0 as i32 == XML_RELAXNG_NOT_ALLOWED as i32
            {
            let fresh268 = &mut ((*cur).parent);
            *fresh268 = parent;
            if !parent.is_null()
                && ((*parent).type_0 as i32
                    == XML_RELAXNG_ATTRIBUTE as i32
                    || (*parent).type_0 as i32 == XML_RELAXNG_LIST as i32
                    || (*parent).type_0 as i32
                        == XML_RELAXNG_GROUP as i32
                    || (*parent).type_0 as i32
                        == XML_RELAXNG_INTERLEAVE as i32
                    || (*parent).type_0 as i32
                        == XML_RELAXNG_ONEORMORE as i32
                    || (*parent).type_0 as i32
                        == XML_RELAXNG_ZEROORMORE as i32)
            {
                (*parent).type_0 = XML_RELAXNG_NOT_ALLOWED;
                break;
            } else if !parent.is_null()
                    && (*parent).type_0 as i32
                        == XML_RELAXNG_CHOICE as i32
                {
                prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev);
            } else {
                prev = cur;
            }
        } else if (*cur).type_0 as i32 == XML_RELAXNG_EMPTY as i32 {
            let fresh269 = &mut ((*cur).parent);
            *fresh269 = parent;
            if !parent.is_null()
                && ((*parent).type_0 as i32
                    == XML_RELAXNG_ONEORMORE as i32
                    || (*parent).type_0 as i32
                        == XML_RELAXNG_ZEROORMORE as i32)
            {
                (*parent).type_0 = XML_RELAXNG_EMPTY;
                break;
            } else if !parent.is_null()
                    && ((*parent).type_0 as i32
                        == XML_RELAXNG_GROUP as i32
                        || (*parent).type_0 as i32
                            == XML_RELAXNG_INTERLEAVE as i32)
                {
                prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev);
            } else {
                prev = cur;
            }
        } else {
            let fresh270 = &mut ((*cur).parent);
            *fresh270 = parent;
            if !((*cur).content).is_null() {
                xmlRelaxNGSimplify(ctxt, (*cur).content, cur);
            }
            if (*cur).type_0 as i32 != XML_RELAXNG_VALUE as i32
                && !((*cur).attrs).is_null()
            {
                xmlRelaxNGSimplify(ctxt, (*cur).attrs, cur);
            }
            if !((*cur).nameClass).is_null() {
                xmlRelaxNGSimplify(ctxt, (*cur).nameClass, cur);
            }
            if (*cur).type_0 as i32 == XML_RELAXNG_ELEMENT as i32 {
                let mut attronly: i32 = 0;
                let mut tmp: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                let mut pre: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
                while !((*cur).content).is_null() {
                    attronly = xmlRelaxNGGenerateAttributes(ctxt, (*cur).content);
                    if !(attronly == 1 as i32) {
                        break;
                    }
                    tmp = (*cur).content;
                    let fresh271 = &mut ((*cur).content);
                    *fresh271 = (*tmp).next;
                    let fresh272 = &mut ((*tmp).next);
                    *fresh272 = (*cur).attrs;
                    let fresh273 = &mut ((*cur).attrs);
                    *fresh273 = tmp;
                }
                pre = (*cur).content;
                while !pre.is_null() && !((*pre).next).is_null() {
                    tmp = (*pre).next;
                    attronly = xmlRelaxNGGenerateAttributes(ctxt, tmp);
                    if attronly == 1 as i32 {
                        let fresh274 = &mut ((*pre).next);
                        *fresh274 = (*tmp).next;
                        let fresh275 = &mut ((*tmp).next);
                        *fresh275 = (*cur).attrs;
                        let fresh276 = &mut ((*cur).attrs);
                        *fresh276 = tmp;
                    } else {
                        pre = tmp;
                    }
                }
            }
            if (*cur).type_0 as i32 == XML_RELAXNG_GROUP as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_INTERLEAVE as i32
            {
                if ((*cur).content).is_null() {
                    (*cur).type_0 = XML_RELAXNG_EMPTY;
                } else if ((*(*cur).content).next).is_null() {
                    if parent.is_null() && prev.is_null() {
                        (*cur).type_0 = XML_RELAXNG_NOOP;
                    } else if prev.is_null() {
                        let fresh277 = &mut ((*parent).content);
                        *fresh277 = (*cur).content;
                        let fresh278 = &mut ((*(*cur).content).next);
                        *fresh278 = (*cur).next;
                        cur = (*cur).content;
                    } else {
                        let fresh279 = &mut ((*(*cur).content).next);
                        *fresh279 = (*cur).next;
                        let fresh280 = &mut ((*prev).next);
                        *fresh280 = (*cur).content;
                        cur = (*cur).content;
                    }
                }
            }
            if (*cur).type_0 as i32 == XML_RELAXNG_EXCEPT as i32
                && !((*cur).content).is_null()
                && (*(*cur).content).type_0 as i32
                    == XML_RELAXNG_NOT_ALLOWED as i32
            {
                prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev);
            } else if (*cur).type_0 as i32
                    == XML_RELAXNG_NOT_ALLOWED as i32
                {
                if !parent.is_null()
                    && ((*parent).type_0 as i32
                        == XML_RELAXNG_ATTRIBUTE as i32
                        || (*parent).type_0 as i32
                            == XML_RELAXNG_LIST as i32
                        || (*parent).type_0 as i32
                            == XML_RELAXNG_GROUP as i32
                        || (*parent).type_0 as i32
                            == XML_RELAXNG_INTERLEAVE as i32
                        || (*parent).type_0 as i32
                            == XML_RELAXNG_ONEORMORE as i32
                        || (*parent).type_0 as i32
                            == XML_RELAXNG_ZEROORMORE as i32)
                {
                    (*parent).type_0 = XML_RELAXNG_NOT_ALLOWED;
                    break;
                } else if !parent.is_null()
                        && (*parent).type_0 as i32
                            == XML_RELAXNG_CHOICE as i32
                    {
                    prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev);
                } else {
                    prev = cur;
                }
            } else if (*cur).type_0 as i32 == XML_RELAXNG_EMPTY as i32 {
                if !parent.is_null()
                    && ((*parent).type_0 as i32
                        == XML_RELAXNG_ONEORMORE as i32
                        || (*parent).type_0 as i32
                            == XML_RELAXNG_ZEROORMORE as i32)
                {
                    (*parent).type_0 = XML_RELAXNG_EMPTY;
                    break;
                } else if !parent.is_null()
                        && ((*parent).type_0 as i32
                            == XML_RELAXNG_GROUP as i32
                            || (*parent).type_0 as i32
                                == XML_RELAXNG_INTERLEAVE as i32
                            || (*parent).type_0 as i32
                                == XML_RELAXNG_CHOICE as i32)
                    {
                    prev = xmlRelaxNGTryUnlink(ctxt, cur, parent, prev);
                } else {
                    prev = cur;
                }
            } else {
                prev = cur;
            }
        }
        cur = (*cur).next;
    }
}
 extern "C" fn xmlRelaxNGGroupContentType(
    mut ct1: xmlRelaxNGContentType,
    mut ct2: xmlRelaxNGContentType,
) -> xmlRelaxNGContentType {
    if ct1 as i32 == XML_RELAXNG_CONTENT_ERROR as i32
        || ct2 as i32 == XML_RELAXNG_CONTENT_ERROR as i32
    {
        return XML_RELAXNG_CONTENT_ERROR;
    }
    if ct1 as i32 == XML_RELAXNG_CONTENT_EMPTY as i32 {
        return ct2;
    }
    if ct2 as i32 == XML_RELAXNG_CONTENT_EMPTY as i32 {
        return ct1;
    }
    if ct1 as i32 == XML_RELAXNG_CONTENT_COMPLEX as i32
        && ct2 as i32 == XML_RELAXNG_CONTENT_COMPLEX as i32
    {
        return XML_RELAXNG_CONTENT_COMPLEX;
    }
    return XML_RELAXNG_CONTENT_ERROR;
}
 extern "C" fn xmlRelaxNGMaxContentType(
    mut ct1: xmlRelaxNGContentType,
    mut ct2: xmlRelaxNGContentType,
) -> xmlRelaxNGContentType {
    if ct1 as i32 == XML_RELAXNG_CONTENT_ERROR as i32
        || ct2 as i32 == XML_RELAXNG_CONTENT_ERROR as i32
    {
        return XML_RELAXNG_CONTENT_ERROR;
    }
    if ct1 as i32 == XML_RELAXNG_CONTENT_SIMPLE as i32
        || ct2 as i32 == XML_RELAXNG_CONTENT_SIMPLE as i32
    {
        return XML_RELAXNG_CONTENT_SIMPLE;
    }
    if ct1 as i32 == XML_RELAXNG_CONTENT_COMPLEX as i32
        || ct2 as i32 == XML_RELAXNG_CONTENT_COMPLEX as i32
    {
        return XML_RELAXNG_CONTENT_COMPLEX;
    }
    return XML_RELAXNG_CONTENT_EMPTY;
}
unsafe extern "C" fn xmlRelaxNGCheckRules(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut cur: xmlRelaxNGDefinePtr,
    mut flags: i32,
    mut ptype: xmlRelaxNGType,
) -> xmlRelaxNGContentType {
    let mut nflags: i32 = 0;
    let mut ret: xmlRelaxNGContentType = XML_RELAXNG_CONTENT_EMPTY;
    let mut tmp: xmlRelaxNGContentType = XML_RELAXNG_CONTENT_EMPTY;
    let mut val: xmlRelaxNGContentType = XML_RELAXNG_CONTENT_EMPTY;
    while !cur.is_null() {
        ret = XML_RELAXNG_CONTENT_EMPTY;
        if (*cur).type_0 as i32 == XML_RELAXNG_REF as i32
            || (*cur).type_0 as i32 == XML_RELAXNG_PARENTREF as i32
        {
            if flags & (1 as i32) << 3 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_REF as i32,
                    b"Found forbidden pattern data/except//ref\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if ((*cur).content).is_null() {
                if (*cur).type_0 as i32 == XML_RELAXNG_PARENTREF as i32 {
                    xmlRngPErr(
                        ctxt,
                        (*cur).node,
                        XML_RNGP_REF_NO_DEF as i32,
                        b"Internal found no define for parent refs\n\0" as *const u8
                            as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                } else {
                    xmlRngPErr(
                        ctxt,
                        (*cur).node,
                        XML_RNGP_REF_NO_DEF as i32,
                        b"Internal found no define for ref %s\n\0" as *const u8
                            as *const i8,
                        if !((*cur).name).is_null() {
                            (*cur).name
                        } else {
                            b"null\0" as *const u8 as *const i8 as *mut xmlChar
                        },
                        0 as *const xmlChar,
                    );
                }
            }
            if (*cur).depth as i32 > -(4 as i32) {
                (*cur).depth = -(4 as i32) as i16;
                ret = xmlRelaxNGCheckRules(ctxt, (*cur).content, flags, (*cur).type_0);
                (*cur).depth = (ret as i32 - 15 as i32) as i16;
            } else if (*cur).depth as i32 == -(4 as i32) {
                ret = XML_RELAXNG_CONTENT_COMPLEX;
            } else {
                ret = ((*cur).depth as i32 + 15 as i32)
                    as xmlRelaxNGContentType;
            }
        } else if (*cur).type_0 as i32 == XML_RELAXNG_ELEMENT as i32 {
            xmlRelaxNGCheckGroupAttrs(ctxt, cur);
            if flags & (1 as i32) << 3 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_ELEM as i32,
                    b"Found forbidden pattern data/except//element(ref)\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 2 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_LIST_ELEM as i32,
                    b"Found forbidden pattern list//element(ref)\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 0 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_ATTR_ELEM as i32,
                    b"Found forbidden pattern attribute//element(ref)\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 0 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_ATTR_ELEM as i32,
                    b"Found forbidden pattern attribute//element(ref)\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            nflags = 0 as i32;
            ret = xmlRelaxNGCheckRules(ctxt, (*cur).attrs, nflags, (*cur).type_0);
            if ret as i32 != XML_RELAXNG_CONTENT_EMPTY as i32 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_ELEM_CONTENT_EMPTY as i32,
                    b"Element %s attributes have a content type error\n\0" as *const u8
                        as *const i8,
                    (*cur).name,
                    0 as *const xmlChar,
                );
            }
            ret = xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags, (*cur).type_0);
            if ret as i32 == XML_RELAXNG_CONTENT_ERROR as i32 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_ELEM_CONTENT_ERROR as i32,
                    b"Element %s has a content type error\n\0" as *const u8
                        as *const i8,
                    (*cur).name,
                    0 as *const xmlChar,
                );
            } else {
                ret = XML_RELAXNG_CONTENT_COMPLEX;
            }
        } else if (*cur).type_0 as i32 == XML_RELAXNG_ATTRIBUTE as i32 {
            if flags & (1 as i32) << 0 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_ATTR_ATTR as i32,
                    b"Found forbidden pattern attribute//attribute\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 2 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_LIST_ATTR as i32,
                    b"Found forbidden pattern list//attribute\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 5 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_ONEMORE_GROUP_ATTR as i32,
                    b"Found forbidden pattern oneOrMore//group//attribute\n\0"
                        as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 6 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR as i32,
                    b"Found forbidden pattern oneOrMore//interleave//attribute\n\0"
                        as *const u8 as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 3 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_ATTR as i32,
                    b"Found forbidden pattern data/except//attribute\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 4 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_START_ATTR as i32,
                    b"Found forbidden pattern start//attribute\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 1 as i32 == 0
                && ((*cur).name).is_null() && ((*cur).nameClass).is_null()
            {
                if ((*cur).ns).is_null() {
                    xmlRngPErr(
                        ctxt,
                        (*cur).node,
                        XML_RNGP_ANYNAME_ATTR_ANCESTOR as i32,
                        b"Found anyName attribute without oneOrMore ancestor\n\0"
                            as *const u8 as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                } else {
                    xmlRngPErr(
                        ctxt,
                        (*cur).node,
                        XML_RNGP_NSNAME_ATTR_ANCESTOR as i32,
                        b"Found nsName attribute without oneOrMore ancestor\n\0"
                            as *const u8 as *const i8,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                    );
                }
            }
            nflags = flags | (1 as i32) << 0 as i32;
            xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags, (*cur).type_0);
            ret = XML_RELAXNG_CONTENT_EMPTY;
        } else if (*cur).type_0 as i32 == XML_RELAXNG_ONEORMORE as i32
                || (*cur).type_0 as i32 == XML_RELAXNG_ZEROORMORE as i32
            {
            if flags & (1 as i32) << 3 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_ONEMORE as i32,
                    b"Found forbidden pattern data/except//oneOrMore\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 4 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_START_ONEMORE as i32,
                    b"Found forbidden pattern start//oneOrMore\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            nflags = flags | (1 as i32) << 1 as i32;
            ret = xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags, (*cur).type_0);
            ret = xmlRelaxNGGroupContentType(ret, ret);
        } else if (*cur).type_0 as i32 == XML_RELAXNG_LIST as i32 {
            if flags & (1 as i32) << 2 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_LIST_LIST as i32,
                    b"Found forbidden pattern list//list\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 3 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_LIST as i32,
                    b"Found forbidden pattern data/except//list\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 4 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_START_LIST as i32,
                    b"Found forbidden pattern start//list\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            nflags = flags | (1 as i32) << 2 as i32;
            ret = xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags, (*cur).type_0);
        } else if (*cur).type_0 as i32 == XML_RELAXNG_GROUP as i32 {
            if flags & (1 as i32) << 3 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_GROUP as i32,
                    b"Found forbidden pattern data/except//group\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 4 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_START_GROUP as i32,
                    b"Found forbidden pattern start//group\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 1 as i32 != 0 {
                nflags = flags | (1 as i32) << 5 as i32;
            } else {
                nflags = flags;
            }
            ret = xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags, (*cur).type_0);
            xmlRelaxNGCheckGroupAttrs(ctxt, cur);
        } else if (*cur).type_0 as i32 == XML_RELAXNG_INTERLEAVE as i32 {
            if flags & (1 as i32) << 2 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_LIST_INTERLEAVE as i32,
                    b"Found forbidden pattern list//interleave\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 3 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE as i32,
                    b"Found forbidden pattern data/except//interleave\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 4 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE as i32,
                    b"Found forbidden pattern start//interleave\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 1 as i32 != 0 {
                nflags = flags | (1 as i32) << 6 as i32;
            } else {
                nflags = flags;
            }
            ret = xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags, (*cur).type_0);
        } else if (*cur).type_0 as i32 == XML_RELAXNG_EXCEPT as i32 {
            if !((*cur).parent).is_null()
                && (*(*cur).parent).type_0 as i32
                    == XML_RELAXNG_DATATYPE as i32
            {
                nflags = flags | (1 as i32) << 3 as i32;
            } else {
                nflags = flags;
            }
            ret = xmlRelaxNGCheckRules(ctxt, (*cur).content, nflags, (*cur).type_0);
        } else if (*cur).type_0 as i32 == XML_RELAXNG_DATATYPE as i32 {
            if flags & (1 as i32) << 4 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_START_DATA as i32,
                    b"Found forbidden pattern start//data\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            xmlRelaxNGCheckRules(ctxt, (*cur).content, flags, (*cur).type_0);
            ret = XML_RELAXNG_CONTENT_SIMPLE;
        } else if (*cur).type_0 as i32 == XML_RELAXNG_VALUE as i32 {
            if flags & (1 as i32) << 4 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_START_VALUE as i32,
                    b"Found forbidden pattern start//value\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            xmlRelaxNGCheckRules(ctxt, (*cur).content, flags, (*cur).type_0);
            ret = XML_RELAXNG_CONTENT_SIMPLE;
        } else if (*cur).type_0 as i32 == XML_RELAXNG_TEXT as i32 {
            if flags & (1 as i32) << 2 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_LIST_TEXT as i32,
                    b"Found forbidden pattern list//text\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 3 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_TEXT as i32,
                    b"Found forbidden pattern data/except//text\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 4 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_START_TEXT as i32,
                    b"Found forbidden pattern start//text\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            ret = XML_RELAXNG_CONTENT_COMPLEX;
        } else if (*cur).type_0 as i32 == XML_RELAXNG_EMPTY as i32 {
            if flags & (1 as i32) << 3 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_DATA_EXCEPT_EMPTY as i32,
                    b"Found forbidden pattern data/except//empty\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            if flags & (1 as i32) << 4 as i32 != 0 {
                xmlRngPErr(
                    ctxt,
                    (*cur).node,
                    XML_RNGP_PAT_START_EMPTY as i32,
                    b"Found forbidden pattern start//empty\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
            ret = XML_RELAXNG_CONTENT_EMPTY;
        } else if (*cur).type_0 as i32 == XML_RELAXNG_CHOICE as i32 {
            xmlRelaxNGCheckChoiceDeterminism(ctxt, cur);
            ret = xmlRelaxNGCheckRules(ctxt, (*cur).content, flags, (*cur).type_0);
        } else {
            ret = xmlRelaxNGCheckRules(ctxt, (*cur).content, flags, (*cur).type_0);
        }
        cur = (*cur).next;
        if ptype as i32 == XML_RELAXNG_GROUP as i32 {
            val = xmlRelaxNGGroupContentType(val, ret);
        } else if ptype as i32 == XML_RELAXNG_INTERLEAVE as i32 {
            tmp = xmlRelaxNGGroupContentType(val, ret);
            if tmp as i32 != XML_RELAXNG_CONTENT_ERROR as i32 {
                tmp = xmlRelaxNGMaxContentType(val, ret);
            }
        } else if ptype as i32 == XML_RELAXNG_CHOICE as i32 {
            val = xmlRelaxNGMaxContentType(val, ret);
        } else if ptype as i32 == XML_RELAXNG_LIST as i32 {
            val = XML_RELAXNG_CONTENT_SIMPLE;
        } else if ptype as i32 == XML_RELAXNG_EXCEPT as i32 {
            if ret as i32 == XML_RELAXNG_CONTENT_ERROR as i32 {
                val = XML_RELAXNG_CONTENT_ERROR;
            } else {
                val = XML_RELAXNG_CONTENT_SIMPLE;
            }
        } else {
            val = xmlRelaxNGGroupContentType(val, ret);
        }
    }
    return val;
}
unsafe extern "C" fn xmlRelaxNGParseGrammar(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut nodes: xmlNodePtr,
) -> xmlRelaxNGGrammarPtr {
    let mut ret: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    let mut tmp: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    let mut old: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    ret = xmlRelaxNGNewGrammar(ctxt);
    if ret.is_null() {
        return 0 as xmlRelaxNGGrammarPtr;
    }
    let fresh281 = &mut ((*ret).parent);
    *fresh281 = (*ctxt).grammar;
    if !((*ctxt).grammar).is_null() {
        tmp = (*(*ctxt).grammar).children;
        if tmp.is_null() {
            let fresh282 = &mut ((*(*ctxt).grammar).children);
            *fresh282 = ret;
        } else {
            while !((*tmp).next).is_null() {
                tmp = (*tmp).next;
            }
            let fresh283 = &mut ((*tmp).next);
            *fresh283 = ret;
        }
    }
    old = (*ctxt).grammar;
    let fresh284 = &mut ((*ctxt).grammar);
    *fresh284 = ret;
    xmlRelaxNGParseGrammarContent(ctxt, nodes);
    let fresh285 = &mut ((*ctxt).grammar);
    *fresh285 = ret;
    if ((*ctxt).grammar).is_null() {
        xmlRngPErr(
            ctxt,
            nodes,
            XML_RNGP_GRAMMAR_CONTENT as i32,
            b"Failed to parse <grammar> content\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    } else if ((*(*ctxt).grammar).start).is_null() {
        xmlRngPErr(
            ctxt,
            nodes,
            XML_RNGP_GRAMMAR_NO_START as i32,
            b"Element <grammar> has no <start>\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
    }
    xmlRelaxNGCombineStart(ctxt, ret);
    if !((*ret).defs).is_null() {
        xmlHashScan(
            (*ret).defs,
            Some(
                xmlRelaxNGCheckCombine
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            ctxt as *mut libc::c_void,
        );
    }
    if !((*ret).refs).is_null() {
        xmlHashScan(
            (*ret).refs,
            Some(
                xmlRelaxNGCheckReference
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            ctxt as *mut libc::c_void,
        );
    }
    let fresh286 = &mut ((*ctxt).grammar);
    *fresh286 = old;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGParseDocument(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlRelaxNGPtr {
    let mut schema: xmlRelaxNGPtr = 0 as xmlRelaxNGPtr;
    let mut olddefine: *const xmlChar = 0 as *const xmlChar;
    let mut old: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    if ctxt.is_null() || node.is_null() {
        return 0 as xmlRelaxNGPtr;
    }
    schema = xmlRelaxNGNewRelaxNG(ctxt);
    if schema.is_null() {
        return 0 as xmlRelaxNGPtr;
    }
    olddefine = (*ctxt).define;
    let fresh287 = &mut ((*ctxt).define);
    *fresh287 = 0 as *const xmlChar;
    if !node.is_null() && !((*node).ns).is_null()
        && (*node).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        && xmlStrEqual(
            (*node).name,
            b"grammar\0" as *const u8 as *const i8 as *const xmlChar,
        ) != 0 && xmlStrEqual((*(*node).ns).href, xmlRelaxNGNs) != 0
    {
        let fresh288 = &mut ((*schema).topgrammar);
        *fresh288 = xmlRelaxNGParseGrammar(ctxt, (*node).children);
        if ((*schema).topgrammar).is_null() {
            xmlRelaxNGFree(schema);
            return 0 as xmlRelaxNGPtr;
        }
    } else {
        let mut tmp: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        let mut ret: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        ret = xmlRelaxNGNewGrammar(ctxt);
        let fresh289 = &mut ((*schema).topgrammar);
        *fresh289 = ret;
        if ((*schema).topgrammar).is_null() {
            xmlRelaxNGFree(schema);
            return 0 as xmlRelaxNGPtr;
        }
        let fresh290 = &mut ((*ret).parent);
        *fresh290 = (*ctxt).grammar;
        if !((*ctxt).grammar).is_null() {
            tmp = (*(*ctxt).grammar).children;
            if tmp.is_null() {
                let fresh291 = &mut ((*(*ctxt).grammar).children);
                *fresh291 = ret;
            } else {
                while !((*tmp).next).is_null() {
                    tmp = (*tmp).next;
                }
                let fresh292 = &mut ((*tmp).next);
                *fresh292 = ret;
            }
        }
        old = (*ctxt).grammar;
        let fresh293 = &mut ((*ctxt).grammar);
        *fresh293 = ret;
        xmlRelaxNGParseStart(ctxt, node);
        if !old.is_null() {
            let fresh294 = &mut ((*ctxt).grammar);
            *fresh294 = old;
        }
    }
    let fresh295 = &mut ((*ctxt).define);
    *fresh295 = olddefine;
    if !((*(*schema).topgrammar).start).is_null() {
        xmlRelaxNGCheckCycles(ctxt, (*(*schema).topgrammar).start, 0 as i32);
        if (*ctxt).flags & (1 as i32) << 7 as i32 == 0 as i32 {
            xmlRelaxNGSimplify(
                ctxt,
                (*(*schema).topgrammar).start,
                0 as xmlRelaxNGDefinePtr,
            );
            while !((*(*schema).topgrammar).start).is_null()
                && (*(*(*schema).topgrammar).start).type_0 as i32
                    == XML_RELAXNG_NOOP as i32
                && !((*(*(*schema).topgrammar).start).next).is_null()
            {
                let fresh296 = &mut ((*(*schema).topgrammar).start);
                *fresh296 = (*(*(*schema).topgrammar).start).content;
            }
            xmlRelaxNGCheckRules(
                ctxt,
                (*(*schema).topgrammar).start,
                (1 as i32) << 4 as i32,
                XML_RELAXNG_NOOP,
            );
        }
    }
    return schema;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewParserCtxt(
    mut URL: *const i8,
) -> xmlRelaxNGParserCtxtPtr {
    let mut ret: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
    if URL.is_null() {
        return 0 as xmlRelaxNGParserCtxtPtr;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNGParserCtxt>() as u64)
        as xmlRelaxNGParserCtxtPtr;
    if ret.is_null() {
        xmlRngPErrMemory(
            0 as xmlRelaxNGParserCtxtPtr,
            b"building parser\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlRelaxNGParserCtxtPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGParserCtxt>() as u64,
    );
    let fresh297 = &mut ((*ret).URL);
    *fresh297 = xmlStrdup(URL as *const xmlChar);
    let fresh298 = &mut ((*ret).error);
    *fresh298 = *__xmlGenericError();
    let fresh299 = &mut ((*ret).userData);
    *fresh299 = *__xmlGenericErrorContext();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewMemParserCtxt(
    mut buffer: *const i8,
    mut size: i32,
) -> xmlRelaxNGParserCtxtPtr {
    let mut ret: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
    if buffer.is_null() || size <= 0 as i32 {
        return 0 as xmlRelaxNGParserCtxtPtr;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNGParserCtxt>() as u64)
        as xmlRelaxNGParserCtxtPtr;
    if ret.is_null() {
        xmlRngPErrMemory(
            0 as xmlRelaxNGParserCtxtPtr,
            b"building parser\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlRelaxNGParserCtxtPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGParserCtxt>() as u64,
    );
    let fresh300 = &mut ((*ret).buffer);
    *fresh300 = buffer;
    (*ret).size = size;
    let fresh301 = &mut ((*ret).error);
    *fresh301 = *__xmlGenericError();
    let fresh302 = &mut ((*ret).userData);
    *fresh302 = *__xmlGenericErrorContext();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewDocParserCtxt(
    mut doc: xmlDocPtr,
) -> xmlRelaxNGParserCtxtPtr {
    let mut ret: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
    let mut copy: xmlDocPtr = 0 as *mut xmlDoc;
    if doc.is_null() {
        return 0 as xmlRelaxNGParserCtxtPtr;
    }
    copy = xmlCopyDoc(doc, 1 as i32);
    if copy.is_null() {
        return 0 as xmlRelaxNGParserCtxtPtr;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNGParserCtxt>() as u64)
        as xmlRelaxNGParserCtxtPtr;
    if ret.is_null() {
        xmlRngPErrMemory(
            0 as xmlRelaxNGParserCtxtPtr,
            b"building parser\n\0" as *const u8 as *const i8,
        );
        xmlFreeDoc(copy);
        return 0 as xmlRelaxNGParserCtxtPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGParserCtxt>() as u64,
    );
    let fresh303 = &mut ((*ret).document);
    *fresh303 = copy;
    (*ret).freedoc = 1 as i32;
    let fresh304 = &mut ((*ret).userData);
    *fresh304 = *__xmlGenericErrorContext();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGFreeParserCtxt(mut ctxt: xmlRelaxNGParserCtxtPtr) {
    if ctxt.is_null() {
        return;
    }
    if !((*ctxt).URL).is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).URL as *mut libc::c_void);
    }
    if !((*ctxt).doc).is_null() {
        xmlRelaxNGFreeDocument((*ctxt).doc);
    }
    if !((*ctxt).interleaves).is_null() {
        xmlHashFree((*ctxt).interleaves, None);
    }
    if !((*ctxt).documents).is_null() {
        xmlRelaxNGFreeDocumentList((*ctxt).documents);
    }
    if !((*ctxt).includes).is_null() {
        xmlRelaxNGFreeIncludeList((*ctxt).includes);
    }
    if !((*ctxt).docTab).is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).docTab as *mut libc::c_void);
    }
    if !((*ctxt).incTab).is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).incTab as *mut libc::c_void);
    }
    if !((*ctxt).defTab).is_null() {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < (*ctxt).defNr {
            xmlRelaxNGFreeDefine(*((*ctxt).defTab).offset(i as isize));
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*ctxt).defTab as *mut libc::c_void);
    }
    if !((*ctxt).document).is_null() && (*ctxt).freedoc != 0 {
        xmlFreeDoc((*ctxt).document);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
}
unsafe extern "C" fn xmlRelaxNGNormExtSpace(mut value: *mut xmlChar) {
    let mut start: *mut xmlChar = value;
    let mut cur: *mut xmlChar = value;
    if value.is_null() {
        return;
    }
    while *cur as i32 == 0x20 as i32
        || 0x9 as i32 <= *cur as i32
            && *cur as i32 <= 0xa as i32
        || *cur as i32 == 0xd as i32
    {
        cur = cur.offset(1);
    }
    if cur == start {
        loop {
            while *cur as i32 != 0 as i32
                && !(*cur as i32 == 0x20 as i32
                    || 0x9 as i32 <= *cur as i32
                        && *cur as i32 <= 0xa as i32
                    || *cur as i32 == 0xd as i32)
            {
                cur = cur.offset(1);
            }
            if *cur as i32 == 0 as i32 {
                return;
            }
            start = cur;
            while *cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *cur as i32
                    && *cur as i32 <= 0xa as i32
                || *cur as i32 == 0xd as i32
            {
                cur = cur.offset(1);
            }
            if *cur as i32 == 0 as i32 {
                *start = 0 as i32 as xmlChar;
                return;
            }
        }
    } else {
        loop {
            while *cur as i32 != 0 as i32
                && !(*cur as i32 == 0x20 as i32
                    || 0x9 as i32 <= *cur as i32
                        && *cur as i32 <= 0xa as i32
                    || *cur as i32 == 0xd as i32)
            {
                let fresh305 = cur;
                cur = cur.offset(1);
                let fresh306 = start;
                start = start.offset(1);
                *fresh306 = *fresh305;
            }
            if *cur as i32 == 0 as i32 {
                *start = 0 as i32 as xmlChar;
                return;
            }
            while *cur as i32 == 0x20 as i32
                || 0x9 as i32 <= *cur as i32
                    && *cur as i32 <= 0xa as i32
                || *cur as i32 == 0xd as i32
            {
                cur = cur.offset(1);
            }
            if *cur as i32 == 0 as i32 {
                *start = 0 as i32 as xmlChar;
                return;
            }
            let fresh307 = cur;
            cur = cur.offset(1);
            let fresh308 = start;
            start = start.offset(1);
            *fresh308 = *fresh307;
        }
    };
}
unsafe extern "C" fn xmlRelaxNGCleanupAttributes(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut node: xmlNodePtr,
) {
    let mut cur: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut next: xmlAttrPtr = 0 as *mut xmlAttr;
    cur = (*node).properties;
    while !cur.is_null() {
        next = (*cur).next;
        if ((*cur).ns).is_null() || xmlStrEqual((*(*cur).ns).href, xmlRelaxNGNs) != 0 {
            if xmlStrEqual(
                (*cur).name,
                b"name\0" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
            {
                if xmlStrEqual(
                    (*node).name,
                    b"element\0" as *const u8 as *const i8 as *mut xmlChar,
                ) == 0
                    && xmlStrEqual(
                        (*node).name,
                        b"attribute\0" as *const u8 as *const i8
                            as *mut xmlChar,
                    ) == 0
                    && xmlStrEqual(
                        (*node).name,
                        b"ref\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) == 0
                    && xmlStrEqual(
                        (*node).name,
                        b"parentRef\0" as *const u8 as *const i8
                            as *mut xmlChar,
                    ) == 0
                    && xmlStrEqual(
                        (*node).name,
                        b"param\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) == 0
                    && xmlStrEqual(
                        (*node).name,
                        b"define\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) == 0
                {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_FORBIDDEN_ATTRIBUTE as i32,
                        b"Attribute %s is not allowed on %s\n\0" as *const u8
                            as *const i8,
                        (*cur).name,
                        (*node).name,
                    );
                }
            } else if xmlStrEqual(
                    (*cur).name,
                    b"type\0" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                if xmlStrEqual(
                    (*node).name,
                    b"value\0" as *const u8 as *const i8 as *mut xmlChar,
                ) == 0
                    && xmlStrEqual(
                        (*node).name,
                        b"data\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) == 0
                {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_FORBIDDEN_ATTRIBUTE as i32,
                        b"Attribute %s is not allowed on %s\n\0" as *const u8
                            as *const i8,
                        (*cur).name,
                        (*node).name,
                    );
                }
            } else if xmlStrEqual(
                    (*cur).name,
                    b"href\0" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                if xmlStrEqual(
                    (*node).name,
                    b"externalRef\0" as *const u8 as *const i8 as *mut xmlChar,
                ) == 0
                    && xmlStrEqual(
                        (*node).name,
                        b"include\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) == 0
                {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_FORBIDDEN_ATTRIBUTE as i32,
                        b"Attribute %s is not allowed on %s\n\0" as *const u8
                            as *const i8,
                        (*cur).name,
                        (*node).name,
                    );
                }
            } else if xmlStrEqual(
                    (*cur).name,
                    b"combine\0" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                if xmlStrEqual(
                    (*node).name,
                    b"start\0" as *const u8 as *const i8 as *mut xmlChar,
                ) == 0
                    && xmlStrEqual(
                        (*node).name,
                        b"define\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) == 0
                {
                    xmlRngPErr(
                        ctxt,
                        node,
                        XML_RNGP_FORBIDDEN_ATTRIBUTE as i32,
                        b"Attribute %s is not allowed on %s\n\0" as *const u8
                            as *const i8,
                        (*cur).name,
                        (*node).name,
                    );
                }
            } else if xmlStrEqual(
                    (*cur).name,
                    b"datatypeLibrary\0" as *const u8 as *const i8
                        as *mut xmlChar,
                ) != 0
                {
                let mut val: *mut xmlChar = 0 as *mut xmlChar;
                let mut uri: xmlURIPtr = 0 as *mut xmlURI;
                val = xmlNodeListGetString(
                    (*node).doc,
                    (*cur).children,
                    1 as i32,
                );
                if !val.is_null() {
                    if *val.offset(0 as i32 as isize) as i32
                        != 0 as i32
                    {
                        uri = xmlParseURI(val as *const i8);
                        if uri.is_null() {
                            xmlRngPErr(
                                ctxt,
                                node,
                                XML_RNGP_INVALID_URI as i32,
                                b"Attribute %s contains invalid URI %s\n\0" as *const u8
                                    as *const i8,
                                (*cur).name,
                                val,
                            );
                        } else {
                            if ((*uri).scheme).is_null() {
                                xmlRngPErr(
                                    ctxt,
                                    node,
                                    XML_RNGP_URI_NOT_ABSOLUTE as i32,
                                    b"Attribute %s URI %s is not absolute\n\0" as *const u8
                                        as *const i8,
                                    (*cur).name,
                                    val,
                                );
                            }
                            if !((*uri).fragment).is_null() {
                                xmlRngPErr(
                                    ctxt,
                                    node,
                                    XML_RNGP_URI_FRAGMENT as i32,
                                    b"Attribute %s URI %s has a fragment ID\n\0" as *const u8
                                        as *const i8,
                                    (*cur).name,
                                    val,
                                );
                            }
                            xmlFreeURI(uri);
                        }
                    }
                    xmlFree
                        .expect("non-null function pointer")(val as *mut libc::c_void);
                }
            } else if xmlStrEqual(
                    (*cur).name,
                    b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                ) == 0
                {
                xmlRngPErr(
                    ctxt,
                    node,
                    XML_RNGP_UNKNOWN_ATTRIBUTE as i32,
                    b"Unknown attribute %s on %s\n\0" as *const u8
                        as *const i8,
                    (*cur).name,
                    (*node).name,
                );
            }
        }
        cur = next;
    }
}
unsafe extern "C" fn xmlRelaxNGCleanupTree(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut root: xmlNodePtr,
) {
    let mut current_block: u64;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut delete: xmlNodePtr = 0 as *mut xmlNode;
    delete = 0 as xmlNodePtr;
    cur = root;
    while !cur.is_null() {
        if !delete.is_null() {
            xmlUnlinkNode(delete);
            xmlFreeNode(delete);
            delete = 0 as xmlNodePtr;
        }
        if (*cur).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        {
            if ((*cur).ns).is_null() || xmlStrEqual((*(*cur).ns).href, xmlRelaxNGNs) == 0
            {
                if !((*cur).parent).is_null()
                    && (*(*cur).parent).type_0 as u32
                        == XML_ELEMENT_NODE as i32 as u32
                    && (xmlStrEqual(
                        (*(*cur).parent).name,
                        b"name\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0
                        || xmlStrEqual(
                            (*(*cur).parent).name,
                            b"value\0" as *const u8 as *const i8
                                as *mut xmlChar,
                        ) != 0
                        || xmlStrEqual(
                            (*(*cur).parent).name,
                            b"param\0" as *const u8 as *const i8
                                as *mut xmlChar,
                        ) != 0)
                {
                    xmlRngPErr(
                        ctxt,
                        cur,
                        XML_RNGP_FOREIGN_ELEMENT as i32,
                        b"element %s doesn't allow foreign elements\n\0" as *const u8
                            as *const i8,
                        (*(*cur).parent).name,
                        0 as *const xmlChar,
                    );
                }
                delete = cur;
                current_block = 6652497021943658697;
            } else {
                xmlRelaxNGCleanupAttributes(ctxt, cur);
                if xmlStrEqual(
                    (*cur).name,
                    b"externalRef\0" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                    let mut href: *mut xmlChar = 0 as *mut xmlChar;
                    let mut ns: *mut xmlChar = 0 as *mut xmlChar;
                    let mut base: *mut xmlChar = 0 as *mut xmlChar;
                    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
                    let mut docu: xmlRelaxNGDocumentPtr = 0 as *mut xmlRelaxNGDocument;
                    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                    let mut uri: xmlURIPtr = 0 as *mut xmlURI;
                    ns = xmlGetProp(
                        cur as *const xmlNode,
                        b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                    );
                    if ns.is_null() {
                        tmp = (*cur).parent;
                        while !tmp.is_null()
                            && (*tmp).type_0 as u32
                                == XML_ELEMENT_NODE as i32 as u32
                        {
                            ns = xmlGetProp(
                                tmp as *const xmlNode,
                                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                            );
                            if !ns.is_null() {
                                break;
                            }
                            tmp = (*tmp).parent;
                        }
                    }
                    href = xmlGetProp(
                        cur as *const xmlNode,
                        b"href\0" as *const u8 as *const i8 as *mut xmlChar,
                    );
                    if href.is_null() {
                        xmlRngPErr(
                            ctxt,
                            cur,
                            XML_RNGP_MISSING_HREF as i32,
                            b"xmlRelaxNGParse: externalRef has no href attribute\n\0"
                                as *const u8 as *const i8,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                        );
                        if !ns.is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(ns as *mut libc::c_void);
                        }
                        delete = cur;
                        current_block = 6652497021943658697;
                    } else {
                        uri = xmlParseURI(href as *const i8);
                        if uri.is_null() {
                            xmlRngPErr(
                                ctxt,
                                cur,
                                XML_RNGP_HREF_ERROR as i32,
                                b"Incorrect URI for externalRef %s\n\0" as *const u8
                                    as *const i8,
                                href,
                                0 as *const xmlChar,
                            );
                            if !ns.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(ns as *mut libc::c_void);
                            }
                            if !href.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(href as *mut libc::c_void);
                            }
                            delete = cur;
                            current_block = 6652497021943658697;
                        } else if !((*uri).fragment).is_null() {
                            xmlRngPErr(
                                ctxt,
                                cur,
                                XML_RNGP_HREF_ERROR as i32,
                                b"Fragment forbidden in URI for externalRef %s\n\0"
                                    as *const u8 as *const i8,
                                href,
                                0 as *const xmlChar,
                            );
                            if !ns.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(ns as *mut libc::c_void);
                            }
                            xmlFreeURI(uri);
                            if !href.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(href as *mut libc::c_void);
                            }
                            delete = cur;
                            current_block = 6652497021943658697;
                        } else {
                            xmlFreeURI(uri);
                            base = xmlNodeGetBase((*cur).doc, cur as *const xmlNode);
                            URL = xmlBuildURI(href, base);
                            if URL.is_null() {
                                xmlRngPErr(
                                    ctxt,
                                    cur,
                                    XML_RNGP_HREF_ERROR as i32,
                                    b"Failed to compute URL for externalRef %s\n\0" as *const u8
                                        as *const i8,
                                    href,
                                    0 as *const xmlChar,
                                );
                                if !ns.is_null() {
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(ns as *mut libc::c_void);
                                }
                                if !href.is_null() {
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(href as *mut libc::c_void);
                                }
                                if !base.is_null() {
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(base as *mut libc::c_void);
                                }
                                delete = cur;
                                current_block = 6652497021943658697;
                            } else {
                                if !href.is_null() {
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(href as *mut libc::c_void);
                                }
                                if !base.is_null() {
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(base as *mut libc::c_void);
                                }
                                docu = xmlRelaxNGLoadExternalRef(ctxt, URL, ns);
                                if docu.is_null() {
                                    xmlRngPErr(
                                        ctxt,
                                        cur,
                                        XML_RNGP_EXTERNAL_REF_FAILURE as i32,
                                        b"Failed to load externalRef %s\n\0" as *const u8
                                            as *const i8,
                                        URL,
                                        0 as *const xmlChar,
                                    );
                                    if !ns.is_null() {
                                        xmlFree
                                            .expect(
                                                "non-null function pointer",
                                            )(ns as *mut libc::c_void);
                                    }
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(URL as *mut libc::c_void);
                                    delete = cur;
                                    current_block = 6652497021943658697;
                                } else {
                                    if !ns.is_null() {
                                        xmlFree
                                            .expect(
                                                "non-null function pointer",
                                            )(ns as *mut libc::c_void);
                                    }
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(URL as *mut libc::c_void);
                                    let fresh309 = &mut ((*cur).psvi);
                                    *fresh309 = docu as *mut libc::c_void;
                                    current_block = 1771738965274008886;
                                }
                            }
                        }
                    }
                } else if xmlStrEqual(
                        (*cur).name,
                        b"include\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0
                    {
                    let mut href_0: *mut xmlChar = 0 as *mut xmlChar;
                    let mut ns_0: *mut xmlChar = 0 as *mut xmlChar;
                    let mut base_0: *mut xmlChar = 0 as *mut xmlChar;
                    let mut URL_0: *mut xmlChar = 0 as *mut xmlChar;
                    let mut incl: xmlRelaxNGIncludePtr = 0 as *mut xmlRelaxNGInclude;
                    let mut tmp_0: xmlNodePtr = 0 as *mut xmlNode;
                    href_0 = xmlGetProp(
                        cur as *const xmlNode,
                        b"href\0" as *const u8 as *const i8 as *mut xmlChar,
                    );
                    if href_0.is_null() {
                        xmlRngPErr(
                            ctxt,
                            cur,
                            XML_RNGP_MISSING_HREF as i32,
                            b"xmlRelaxNGParse: include has no href attribute\n\0"
                                as *const u8 as *const i8,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                        );
                        delete = cur;
                        current_block = 6652497021943658697;
                    } else {
                        base_0 = xmlNodeGetBase((*cur).doc, cur as *const xmlNode);
                        URL_0 = xmlBuildURI(href_0, base_0);
                        if URL_0.is_null() {
                            xmlRngPErr(
                                ctxt,
                                cur,
                                XML_RNGP_HREF_ERROR as i32,
                                b"Failed to compute URL for include %s\n\0" as *const u8
                                    as *const i8,
                                href_0,
                                0 as *const xmlChar,
                            );
                            if !href_0.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(href_0 as *mut libc::c_void);
                            }
                            if !base_0.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(base_0 as *mut libc::c_void);
                            }
                            delete = cur;
                            current_block = 6652497021943658697;
                        } else {
                            if !href_0.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(href_0 as *mut libc::c_void);
                            }
                            if !base_0.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(base_0 as *mut libc::c_void);
                            }
                            ns_0 = xmlGetProp(
                                cur as *const xmlNode,
                                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                            );
                            if ns_0.is_null() {
                                tmp_0 = (*cur).parent;
                                while !tmp_0.is_null()
                                    && (*tmp_0).type_0 as u32
                                        == XML_ELEMENT_NODE as i32 as u32
                                {
                                    ns_0 = xmlGetProp(
                                        tmp_0 as *const xmlNode,
                                        b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                                    );
                                    if !ns_0.is_null() {
                                        break;
                                    }
                                    tmp_0 = (*tmp_0).parent;
                                }
                            }
                            incl = xmlRelaxNGLoadInclude(ctxt, URL_0, cur, ns_0);
                            if !ns_0.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(ns_0 as *mut libc::c_void);
                            }
                            if incl.is_null() {
                                xmlRngPErr(
                                    ctxt,
                                    cur,
                                    XML_RNGP_INCLUDE_FAILURE as i32,
                                    b"Failed to load include %s\n\0" as *const u8
                                        as *const i8,
                                    URL_0,
                                    0 as *const xmlChar,
                                );
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(URL_0 as *mut libc::c_void);
                                delete = cur;
                                current_block = 6652497021943658697;
                            } else {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(URL_0 as *mut libc::c_void);
                                let fresh310 = &mut ((*cur).psvi);
                                *fresh310 = incl as *mut libc::c_void;
                                current_block = 1771738965274008886;
                            }
                        }
                    }
                } else if xmlStrEqual(
                        (*cur).name,
                        b"element\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0
                        || xmlStrEqual(
                            (*cur).name,
                            b"attribute\0" as *const u8 as *const i8
                                as *mut xmlChar,
                        ) != 0
                    {
                    let mut name: *mut xmlChar = 0 as *mut xmlChar;
                    let mut ns_1: *mut xmlChar = 0 as *mut xmlChar;
                    let mut text: xmlNodePtr = 0 as xmlNodePtr;
                    name = xmlGetProp(
                        cur as *const xmlNode,
                        b"name\0" as *const u8 as *const i8 as *mut xmlChar,
                    );
                    if !name.is_null() {
                        if ((*cur).children).is_null() {
                            text = xmlNewChild(
                                cur,
                                (*cur).ns,
                                b"name\0" as *const u8 as *const i8
                                    as *mut xmlChar,
                                name,
                            );
                        } else {
                            let mut node: xmlNodePtr = 0 as *mut xmlNode;
                            node = xmlNewDocNode(
                                (*cur).doc,
                                (*cur).ns,
                                b"name\0" as *const u8 as *const i8
                                    as *mut xmlChar,
                                0 as *const xmlChar,
                            );
                            if !node.is_null() {
                                xmlAddPrevSibling((*cur).children, node);
                                text = xmlNewDocText((*node).doc, name);
                                xmlAddChild(node, text);
                                text = node;
                            }
                        }
                        if text.is_null() {
                            xmlRngPErr(
                                ctxt,
                                cur,
                                XML_RNGP_CREATE_FAILURE as i32,
                                b"Failed to create a name %s element\n\0" as *const u8
                                    as *const i8,
                                name,
                                0 as *const xmlChar,
                            );
                        }
                        xmlUnsetProp(
                            cur,
                            b"name\0" as *const u8 as *const i8 as *mut xmlChar,
                        );
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(name as *mut libc::c_void);
                        ns_1 = xmlGetProp(
                            cur as *const xmlNode,
                            b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                        );
                        if !ns_1.is_null() {
                            if !text.is_null() {
                                xmlSetProp(
                                    text,
                                    b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                                    ns_1,
                                );
                            }
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(ns_1 as *mut libc::c_void);
                        } else if xmlStrEqual(
                                (*cur).name,
                                b"attribute\0" as *const u8 as *const i8
                                    as *mut xmlChar,
                            ) != 0
                            {
                            xmlSetProp(
                                text,
                                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                                b"\0" as *const u8 as *const i8 as *mut xmlChar,
                            );
                        }
                    }
                    current_block = 1771738965274008886;
                } else if xmlStrEqual(
                        (*cur).name,
                        b"name\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0
                        || xmlStrEqual(
                            (*cur).name,
                            b"nsName\0" as *const u8 as *const i8
                                as *mut xmlChar,
                        ) != 0
                        || xmlStrEqual(
                            (*cur).name,
                            b"value\0" as *const u8 as *const i8
                                as *mut xmlChar,
                        ) != 0
                    {
                    if (xmlHasProp(
                        cur as *const xmlNode,
                        b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                    ))
                        .is_null()
                    {
                        let mut node_0: xmlNodePtr = 0 as *mut xmlNode;
                        let mut ns_2: *mut xmlChar = 0 as *mut xmlChar;
                        node_0 = (*cur).parent;
                        while !node_0.is_null()
                            && (*node_0).type_0 as u32
                                == XML_ELEMENT_NODE as i32 as u32
                        {
                            ns_2 = xmlGetProp(
                                node_0 as *const xmlNode,
                                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                            );
                            if !ns_2.is_null() {
                                break;
                            }
                            node_0 = (*node_0).parent;
                        }
                        if ns_2.is_null() {
                            xmlSetProp(
                                cur,
                                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                                b"\0" as *const u8 as *const i8 as *mut xmlChar,
                            );
                        } else {
                            xmlSetProp(
                                cur,
                                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                                ns_2,
                            );
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(ns_2 as *mut libc::c_void);
                        }
                    }
                    if xmlStrEqual(
                        (*cur).name,
                        b"name\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0
                    {
                        let mut name_0: *mut xmlChar = 0 as *mut xmlChar;
                        let mut local: *mut xmlChar = 0 as *mut xmlChar;
                        let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
                        name_0 = xmlNodeGetContent(cur as *const xmlNode);
                        if !name_0.is_null() {
                            local = xmlSplitQName2(name_0, &mut prefix);
                            if !local.is_null() {
                                let mut ns_3: xmlNsPtr = 0 as *mut xmlNs;
                                ns_3 = xmlSearchNs((*cur).doc, cur, prefix);
                                if ns_3.is_null() {
                                    xmlRngPErr(
                                        ctxt,
                                        cur,
                                        XML_RNGP_PREFIX_UNDEFINED as i32,
                                        b"xmlRelaxNGParse: no namespace for prefix %s\n\0"
                                            as *const u8 as *const i8,
                                        prefix,
                                        0 as *const xmlChar,
                                    );
                                } else {
                                    xmlSetProp(
                                        cur,
                                        b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                                        (*ns_3).href,
                                    );
                                    xmlNodeSetContent(cur, local);
                                }
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(local as *mut libc::c_void);
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(prefix as *mut libc::c_void);
                            }
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(name_0 as *mut libc::c_void);
                        }
                    }
                    if xmlStrEqual(
                        (*cur).name,
                        b"nsName\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0
                    {
                        if (*ctxt).flags & (1 as i32) << 9 as i32 != 0 {
                            xmlRngPErr(
                                ctxt,
                                cur,
                                XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME as i32,
                                b"Found nsName/except//nsName forbidden construct\n\0"
                                    as *const u8 as *const i8,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                            );
                        }
                    }
                    current_block = 1771738965274008886;
                } else if xmlStrEqual(
                        (*cur).name,
                        b"except\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0 && cur != root
                    {
                    let mut oldflags: i32 = (*ctxt).flags;
                    if !((*cur).parent).is_null()
                        && xmlStrEqual(
                            (*(*cur).parent).name,
                            b"anyName\0" as *const u8 as *const i8
                                as *mut xmlChar,
                        ) != 0
                    {
                        (*ctxt).flags |= (1 as i32) << 8 as i32;
                        xmlRelaxNGCleanupTree(ctxt, cur);
                        (*ctxt).flags = oldflags;
                        current_block = 6652497021943658697;
                    } else if !((*cur).parent).is_null()
                            && xmlStrEqual(
                                (*(*cur).parent).name,
                                b"nsName\0" as *const u8 as *const i8
                                    as *mut xmlChar,
                            ) != 0
                        {
                        (*ctxt).flags |= (1 as i32) << 9 as i32;
                        xmlRelaxNGCleanupTree(ctxt, cur);
                        (*ctxt).flags = oldflags;
                        current_block = 6652497021943658697;
                    } else {
                        current_block = 1771738965274008886;
                    }
                } else {
                    if xmlStrEqual(
                        (*cur).name,
                        b"anyName\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0
                    {
                        if (*ctxt).flags & (1 as i32) << 8 as i32 != 0 {
                            xmlRngPErr(
                                ctxt,
                                cur,
                                XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME as i32,
                                b"Found anyName/except//anyName forbidden construct\n\0"
                                    as *const u8 as *const i8,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                            );
                        } else if (*ctxt).flags & (1 as i32) << 9 as i32
                                != 0
                            {
                            xmlRngPErr(
                                ctxt,
                                cur,
                                XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME as i32,
                                b"Found nsName/except//anyName forbidden construct\n\0"
                                    as *const u8 as *const i8,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                            );
                        }
                    }
                    current_block = 1771738965274008886;
                }
                match current_block {
                    6652497021943658697 => {}
                    _ => {
                        if xmlStrEqual(
                            (*cur).name,
                            b"div\0" as *const u8 as *const i8 as *mut xmlChar,
                        ) != 0
                        {
                            let mut ns_4: *mut xmlChar = 0 as *mut xmlChar;
                            let mut child: xmlNodePtr = 0 as *mut xmlNode;
                            let mut ins: xmlNodePtr = 0 as *mut xmlNode;
                            let mut tmp_1: xmlNodePtr = 0 as *mut xmlNode;
                            ns_4 = xmlGetProp(
                                cur as *const xmlNode,
                                b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                            );
                            child = (*cur).children;
                            ins = cur;
                            while !child.is_null() {
                                if !ns_4.is_null() {
                                    if (xmlHasProp(
                                        child as *const xmlNode,
                                        b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                                    ))
                                        .is_null()
                                    {
                                        xmlSetProp(
                                            child,
                                            b"ns\0" as *const u8 as *const i8 as *mut xmlChar,
                                            ns_4,
                                        );
                                    }
                                }
                                tmp_1 = (*child).next;
                                xmlUnlinkNode(child);
                                ins = xmlAddNextSibling(ins, child);
                                child = tmp_1;
                            }
                            if !ns_4.is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(ns_4 as *mut libc::c_void);
                            }
                            if !((*cur).nsDef).is_null() && !((*cur).parent).is_null() {
                                let mut parDef: xmlNsPtr = &mut (*(*cur).parent).nsDef
                                    as *mut *mut xmlNs as xmlNsPtr;
                                while !((*parDef).next).is_null() {
                                    parDef = (*parDef).next;
                                }
                                let fresh311 = &mut ((*parDef).next);
                                *fresh311 = (*cur).nsDef;
                                let fresh312 = &mut ((*cur).nsDef);
                                *fresh312 = 0 as *mut xmlNs;
                            }
                            delete = cur;
                            current_block = 6652497021943658697;
                        } else {
                            current_block = 3788568606521286043;
                        }
                    }
                }
            }
        } else if (*cur).type_0 as u32
                == XML_TEXT_NODE as i32 as u32
                || (*cur).type_0 as u32
                    == XML_CDATA_SECTION_NODE as i32 as u32
            {
            if xmlRelaxNGIsBlank((*cur).content) != 0 {
                if !((*cur).parent).is_null()
                    && (*(*cur).parent).type_0 as u32
                        == XML_ELEMENT_NODE as i32 as u32
                {
                    if xmlStrEqual(
                        (*(*cur).parent).name,
                        b"value\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) == 0
                        && xmlStrEqual(
                            (*(*cur).parent).name,
                            b"param\0" as *const u8 as *const i8
                                as *mut xmlChar,
                        ) == 0
                    {
                        delete = cur;
                    }
                    current_block = 3788568606521286043;
                } else {
                    delete = cur;
                    current_block = 6652497021943658697;
                }
            } else {
                current_block = 3788568606521286043;
            }
        } else {
            delete = cur;
            current_block = 6652497021943658697;
        }
        match current_block {
            3788568606521286043 => {
                if !((*cur).children).is_null() {
                    if (*(*cur).children).type_0 as u32
                        != XML_ENTITY_DECL as i32 as u32
                        && (*(*cur).children).type_0 as u32
                            != XML_ENTITY_REF_NODE as i32 as u32
                        && (*(*cur).children).type_0 as u32
                            != XML_ENTITY_NODE as i32 as u32
                    {
                        cur = (*cur).children;
                        continue;
                    }
                }
            }
            _ => {}
        }
        if !((*cur).next).is_null() {
            cur = (*cur).next;
        } else {
            loop {
                cur = (*cur).parent;
                if cur.is_null() {
                    break;
                }
                if cur == root {
                    cur = 0 as xmlNodePtr;
                    break;
                } else if !((*cur).next).is_null() {
                    cur = (*cur).next;
                    break;
                } else if cur.is_null() {
                    break;
                }
            }
        }
    }
    if !delete.is_null() {
        xmlUnlinkNode(delete);
        xmlFreeNode(delete);
        delete = 0 as xmlNodePtr;
    }
}
unsafe extern "C" fn xmlRelaxNGCleanupDoc(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut doc: xmlDocPtr,
) -> xmlDocPtr {
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        xmlRngPErr(
            ctxt,
            doc as xmlNodePtr,
            XML_RNGP_EMPTY as i32,
            b"xmlRelaxNGParse: %s is empty\n\0" as *const u8 as *const i8,
            (*ctxt).URL,
            0 as *const xmlChar,
        );
        return 0 as xmlDocPtr;
    }
    xmlRelaxNGCleanupTree(ctxt, root);
    return doc;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGParse(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
) -> xmlRelaxNGPtr {
    let mut ret: xmlRelaxNGPtr = 0 as xmlRelaxNGPtr;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut root: xmlNodePtr = 0 as *mut xmlNode;
    xmlRelaxNGInitTypes();
    if ctxt.is_null() {
        return 0 as xmlRelaxNGPtr;
    }
    if !((*ctxt).URL).is_null() {
        doc = xmlReadFile(
            (*ctxt).URL as *const i8,
            0 as *const i8,
            0 as i32,
        );
        if doc.is_null() {
            xmlRngPErr(
                ctxt,
                0 as xmlNodePtr,
                XML_RNGP_PARSE_ERROR as i32,
                b"xmlRelaxNGParse: could not load %s\n\0" as *const u8
                    as *const i8,
                (*ctxt).URL,
                0 as *const xmlChar,
            );
            return 0 as xmlRelaxNGPtr;
        }
    } else if !((*ctxt).buffer).is_null() {
        doc = xmlReadMemory(
            (*ctxt).buffer,
            (*ctxt).size,
            0 as *const i8,
            0 as *const i8,
            0 as i32,
        );
        if doc.is_null() {
            xmlRngPErr(
                ctxt,
                0 as xmlNodePtr,
                XML_RNGP_PARSE_ERROR as i32,
                b"xmlRelaxNGParse: could not parse schemas\n\0" as *const u8
                    as *const i8,
                0 as *const xmlChar,
                0 as *const xmlChar,
            );
            return 0 as xmlRelaxNGPtr;
        }
        let fresh313 = &mut ((*doc).URL);
        *fresh313 = xmlStrdup(
            b"in_memory_buffer\0" as *const u8 as *const i8 as *mut xmlChar,
        );
        let fresh314 = &mut ((*ctxt).URL);
        *fresh314 = xmlStrdup(
            b"in_memory_buffer\0" as *const u8 as *const i8 as *mut xmlChar,
        );
    } else if !((*ctxt).document).is_null() {
        doc = (*ctxt).document;
    } else {
        xmlRngPErr(
            ctxt,
            0 as xmlNodePtr,
            XML_RNGP_EMPTY as i32,
            b"xmlRelaxNGParse: nothing to parse\n\0" as *const u8 as *const i8,
            0 as *const xmlChar,
            0 as *const xmlChar,
        );
        return 0 as xmlRelaxNGPtr;
    }
    let fresh315 = &mut ((*ctxt).document);
    *fresh315 = doc;
    doc = xmlRelaxNGCleanupDoc(ctxt, doc);
    if doc.is_null() {
        xmlFreeDoc((*ctxt).document);
        let fresh316 = &mut ((*ctxt).document);
        *fresh316 = 0 as xmlDocPtr;
        return 0 as xmlRelaxNGPtr;
    }
    root = xmlDocGetRootElement(doc as *const xmlDoc);
    if root.is_null() {
        xmlRngPErr(
            ctxt,
            doc as xmlNodePtr,
            XML_RNGP_EMPTY as i32,
            b"xmlRelaxNGParse: %s is empty\n\0" as *const u8 as *const i8,
            if !((*ctxt).URL).is_null() {
                (*ctxt).URL
            } else {
                b"schemas\0" as *const u8 as *const i8 as *mut xmlChar
            },
            0 as *const xmlChar,
        );
        xmlFreeDoc((*ctxt).document);
        let fresh317 = &mut ((*ctxt).document);
        *fresh317 = 0 as xmlDocPtr;
        return 0 as xmlRelaxNGPtr;
    }
    ret = xmlRelaxNGParseDocument(ctxt, root);
    if ret.is_null() {
        xmlFreeDoc((*ctxt).document);
        let fresh318 = &mut ((*ctxt).document);
        *fresh318 = 0 as xmlDocPtr;
        return 0 as xmlRelaxNGPtr;
    }
    if !((*ctxt).interleaves).is_null() {
        xmlHashScan(
            (*ctxt).interleaves,
            Some(
                xmlRelaxNGComputeInterleaves
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> (),
            ),
            ctxt as *mut libc::c_void,
        );
    }
    if (*ctxt).nbErrors > 0 as i32 {
        xmlRelaxNGFree(ret);
        let fresh319 = &mut ((*ctxt).document);
        *fresh319 = 0 as xmlDocPtr;
        xmlFreeDoc(doc);
        return 0 as xmlRelaxNGPtr;
    }
    if !((*ret).topgrammar).is_null() && !((*(*ret).topgrammar).start).is_null() {
        if (*(*(*ret).topgrammar).start).type_0 as i32
            != XML_RELAXNG_START as i32
        {
            let mut def: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
            def = xmlRelaxNGNewDefine(ctxt, 0 as xmlNodePtr);
            if !def.is_null() {
                (*def).type_0 = XML_RELAXNG_START;
                let fresh320 = &mut ((*def).content);
                *fresh320 = (*(*ret).topgrammar).start;
                let fresh321 = &mut ((*(*ret).topgrammar).start);
                *fresh321 = def;
            }
        }
        xmlRelaxNGTryCompile(ctxt, (*(*ret).topgrammar).start);
    }
    let fresh322 = &mut ((*ret).doc);
    *fresh322 = doc;
    let fresh323 = &mut ((*ctxt).document);
    *fresh323 = 0 as xmlDocPtr;
    let fresh324 = &mut ((*ret).documents);
    *fresh324 = (*ctxt).documents;
    let fresh325 = &mut ((*ctxt).documents);
    *fresh325 = 0 as xmlRelaxNGDocumentPtr;
    let fresh326 = &mut ((*ret).includes);
    *fresh326 = (*ctxt).includes;
    let fresh327 = &mut ((*ctxt).includes);
    *fresh327 = 0 as xmlRelaxNGIncludePtr;
    (*ret).defNr = (*ctxt).defNr;
    let fresh328 = &mut ((*ret).defTab);
    *fresh328 = (*ctxt).defTab;
    let fresh329 = &mut ((*ctxt).defTab);
    *fresh329 = 0 as *mut xmlRelaxNGDefinePtr;
    if (*ctxt).idref == 1 as i32 {
        (*ret).idref = 1 as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGSetParserErrors(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut err: xmlRelaxNGValidityErrorFunc,
    mut warn: xmlRelaxNGValidityWarningFunc,
    mut ctx: *mut libc::c_void,
) {
    if ctxt.is_null() {
        return;
    }
    let fresh330 = &mut ((*ctxt).error);
    *fresh330 = err;
    let fresh331 = &mut ((*ctxt).warning);
    *fresh331 = warn;
    let fresh332 = &mut ((*ctxt).serror);
    *fresh332 = None;
    let fresh333 = &mut ((*ctxt).userData);
    *fresh333 = ctx;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGGetParserErrors(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut err: *mut xmlRelaxNGValidityErrorFunc,
    mut warn: *mut xmlRelaxNGValidityWarningFunc,
    mut ctx: *mut *mut libc::c_void,
) -> i32 {
    if ctxt.is_null() {
        return -(1 as i32);
    }
    if !err.is_null() {
        *err = (*ctxt).error;
    }
    if !warn.is_null() {
        *warn = (*ctxt).warning;
    }
    if !ctx.is_null() {
        *ctx = (*ctxt).userData;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGSetParserStructuredErrors(
    mut ctxt: xmlRelaxNGParserCtxtPtr,
    mut serror: xmlStructuredErrorFunc,
    mut ctx: *mut libc::c_void,
) {
    if ctxt.is_null() {
        return;
    }
    let fresh334 = &mut ((*ctxt).serror);
    *fresh334 = serror;
    let fresh335 = &mut ((*ctxt).error);
    *fresh335 = None;
    let fresh336 = &mut ((*ctxt).warning);
    *fresh336 = None;
    let fresh337 = &mut ((*ctxt).userData);
    *fresh337 = ctx;
}
unsafe extern "C" fn xmlRelaxNGDumpDefines(
    mut output: *mut FILE,
    mut defines: xmlRelaxNGDefinePtr,
) {
    while !defines.is_null() {
        xmlRelaxNGDumpDefine(output, defines);
        defines = (*defines).next;
    }
}
unsafe extern "C" fn xmlRelaxNGDumpDefine(
    mut output: *mut FILE,
    mut define: xmlRelaxNGDefinePtr,
) {
    if define.is_null() {
        return;
    }
    match (*define).type_0 as i32 {
        0 => {
            fprintf(output, b"<empty/>\n\0" as *const u8 as *const i8);
        }
        1 => {
            fprintf(output, b"<notAllowed/>\n\0" as *const u8 as *const i8);
        }
        3 => {
            fprintf(output, b"<text/>\n\0" as *const u8 as *const i8);
        }
        4 => {
            fprintf(output, b"<element>\n\0" as *const u8 as *const i8);
            if !((*define).name).is_null() {
                fprintf(output, b"<name\0" as *const u8 as *const i8);
                if !((*define).ns).is_null() {
                    fprintf(
                        output,
                        b" ns=\"%s\"\0" as *const u8 as *const i8,
                        (*define).ns,
                    );
                }
                fprintf(
                    output,
                    b">%s</name>\n\0" as *const u8 as *const i8,
                    (*define).name,
                );
            }
            xmlRelaxNGDumpDefines(output, (*define).attrs);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</element>\n\0" as *const u8 as *const i8);
        }
        8 => {
            fprintf(output, b"<list>\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</list>\n\0" as *const u8 as *const i8);
        }
        16 => {
            fprintf(output, b"<oneOrMore>\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</oneOrMore>\n\0" as *const u8 as *const i8);
        }
        15 => {
            fprintf(output, b"<zeroOrMore>\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</zeroOrMore>\n\0" as *const u8 as *const i8);
        }
        17 => {
            fprintf(output, b"<choice>\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</choice>\n\0" as *const u8 as *const i8);
        }
        18 => {
            fprintf(output, b"<group>\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</group>\n\0" as *const u8 as *const i8);
        }
        19 => {
            fprintf(output, b"<interleave>\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</interleave>\n\0" as *const u8 as *const i8);
        }
        14 => {
            fprintf(output, b"<optional>\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</optional>\n\0" as *const u8 as *const i8);
        }
        9 => {
            fprintf(output, b"<attribute>\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</attribute>\n\0" as *const u8 as *const i8);
        }
        10 => {
            fprintf(output, b"<define\0" as *const u8 as *const i8);
            if !((*define).name).is_null() {
                fprintf(
                    output,
                    b" name=\"%s\"\0" as *const u8 as *const i8,
                    (*define).name,
                );
            }
            fprintf(output, b">\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</define>\n\0" as *const u8 as *const i8);
        }
        11 => {
            fprintf(output, b"<ref\0" as *const u8 as *const i8);
            if !((*define).name).is_null() {
                fprintf(
                    output,
                    b" name=\"%s\"\0" as *const u8 as *const i8,
                    (*define).name,
                );
            }
            fprintf(output, b">\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</ref>\n\0" as *const u8 as *const i8);
        }
        13 => {
            fprintf(output, b"<parentRef\0" as *const u8 as *const i8);
            if !((*define).name).is_null() {
                fprintf(
                    output,
                    b" name=\"%s\"\0" as *const u8 as *const i8,
                    (*define).name,
                );
            }
            fprintf(output, b">\n\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</parentRef>\n\0" as *const u8 as *const i8);
        }
        12 => {
            fprintf(output, b"<externalRef>\0" as *const u8 as *const i8);
            xmlRelaxNGDumpDefines(output, (*define).content);
            fprintf(output, b"</externalRef>\n\0" as *const u8 as *const i8);
        }
        5 | 7 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
                b"relaxng.c\0" as *const u8 as *const i8,
                7841 as i32,
            );
        }
        20 | 2 | 6 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
                b"relaxng.c\0" as *const u8 as *const i8,
                7845 as i32,
            );
        }
        -1 => {
            xmlRelaxNGDumpDefines(output, (*define).content);
        }
        _ => {}
    };
}
unsafe extern "C" fn xmlRelaxNGDumpGrammar(
    mut output: *mut FILE,
    mut grammar: xmlRelaxNGGrammarPtr,
    mut top: i32,
) {
    if grammar.is_null() {
        return;
    }
    fprintf(output, b"<grammar\0" as *const u8 as *const i8);
    if top != 0 {
        fprintf(
            output,
            b" xmlns=\"http://relaxng.org/ns/structure/1.0\"\0" as *const u8
                as *const i8,
        );
    }
    match (*grammar).combine as u32 {
        0 => {}
        1 => {
            fprintf(
                output,
                b" combine=\"choice\"\0" as *const u8 as *const i8,
            );
        }
        2 => {
            fprintf(
                output,
                b" combine=\"interleave\"\0" as *const u8 as *const i8,
            );
        }
        _ => {
            fprintf(
                output,
                b" <!-- invalid combine value -->\0" as *const u8 as *const i8,
            );
        }
    }
    fprintf(output, b">\n\0" as *const u8 as *const i8);
    if ((*grammar).start).is_null() {
        fprintf(
            output,
            b" <!-- grammar had no start -->\0" as *const u8 as *const i8,
        );
    } else {
        fprintf(output, b"<start>\n\0" as *const u8 as *const i8);
        xmlRelaxNGDumpDefine(output, (*grammar).start);
        fprintf(output, b"</start>\n\0" as *const u8 as *const i8);
    }
    fprintf(output, b"</grammar>\n\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGDump(
    mut output: *mut FILE,
    mut schema: xmlRelaxNGPtr,
) {
    if output.is_null() {
        return;
    }
    if schema.is_null() {
        fprintf(
            output,
            b"RelaxNG empty or failed to compile\n\0" as *const u8 as *const i8,
        );
        return;
    }
    fprintf(output, b"RelaxNG: \0" as *const u8 as *const i8);
    if ((*schema).doc).is_null() {
        fprintf(output, b"no document\n\0" as *const u8 as *const i8);
    } else if !((*(*schema).doc).URL).is_null() {
        fprintf(
            output,
            b"%s\n\0" as *const u8 as *const i8,
            (*(*schema).doc).URL,
        );
    } else {
        fprintf(output, b"\n\0" as *const u8 as *const i8);
    }
    if ((*schema).topgrammar).is_null() {
        fprintf(
            output,
            b"RelaxNG has no top grammar\n\0" as *const u8 as *const i8,
        );
        return;
    }
    xmlRelaxNGDumpGrammar(output, (*schema).topgrammar, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGDumpTree(
    mut output: *mut FILE,
    mut schema: xmlRelaxNGPtr,
) {
    if output.is_null() {
        return;
    }
    if schema.is_null() {
        fprintf(
            output,
            b"RelaxNG empty or failed to compile\n\0" as *const u8 as *const i8,
        );
        return;
    }
    if ((*schema).doc).is_null() {
        fprintf(output, b"no document\n\0" as *const u8 as *const i8);
    } else {
        xmlDocDump(output, (*schema).doc);
    };
}
unsafe extern "C" fn xmlRelaxNGValidateCompiledCallback(
    mut exec: xmlRegExecCtxtPtr,
    mut token: *const xmlChar,
    mut transdata: *mut libc::c_void,
    mut inputdata: *mut libc::c_void,
) {
    let mut ctxt: xmlRelaxNGValidCtxtPtr = inputdata as xmlRelaxNGValidCtxtPtr;
    let mut define: xmlRelaxNGDefinePtr = transdata as xmlRelaxNGDefinePtr;
    let mut ret: i32 = 0;
    if ctxt.is_null() {
        fprintf(
            stderr,
            b"callback on %s missing context\n\0" as *const u8 as *const i8,
            token,
        );
        return;
    }
    if define.is_null() {
        if *token.offset(0 as i32 as isize) as i32 == '#' as i32 {
            return;
        }
        fprintf(
            stderr,
            b"callback on %s missing define\n\0" as *const u8 as *const i8,
            token,
        );
        if !ctxt.is_null() && (*ctxt).errNo == XML_RELAXNG_OK as i32 {
            (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as i32;
        }
        return;
    }
    if ctxt.is_null() || define.is_null() {
        fprintf(
            stderr,
            b"callback on %s missing info\n\0" as *const u8 as *const i8,
            token,
        );
        if !ctxt.is_null() && (*ctxt).errNo == XML_RELAXNG_OK as i32 {
            (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as i32;
        }
        return;
    } else {
        if (*define).type_0 as i32 != XML_RELAXNG_ELEMENT as i32 {
            fprintf(
                stderr,
                b"callback on %s define is not element\n\0" as *const u8
                    as *const i8,
                token,
            );
            if (*ctxt).errNo == XML_RELAXNG_OK as i32 {
                (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as i32;
            }
            return;
        }
    }
    ret = xmlRelaxNGValidateDefinition(ctxt, define);
    if ret != 0 as i32 {
        (*ctxt).perr = ret;
    }
}
unsafe extern "C" fn xmlRelaxNGValidateCompiledContent(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut regexp: xmlRegexpPtr,
    mut content: xmlNodePtr,
) -> i32 {
    let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: i32 = 0 as i32;
    let mut oldperr: i32 = 0;
    if ctxt.is_null() || regexp.is_null() {
        return -(1 as i32);
    }
    oldperr = (*ctxt).perr;
    exec = xmlRegNewExecCtxt(
        regexp,
        Some(
            xmlRelaxNGValidateCompiledCallback
                as unsafe extern "C" fn(
                    xmlRegExecCtxtPtr,
                    *const xmlChar,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        ctxt as *mut libc::c_void,
    );
    (*ctxt).perr = 0 as i32;
    cur = content;
    while !cur.is_null() {
        let fresh338 = &mut ((*(*ctxt).state).seq);
        *fresh338 = cur;
        match (*cur).type_0 as u32 {
            3 | 4 => {
                if !(xmlIsBlankNode(cur as *const xmlNode) != 0) {
                    ret = xmlRegExecPushString(
                        exec,
                        b"#text\0" as *const u8 as *const i8 as *mut xmlChar,
                        ctxt as *mut libc::c_void,
                    );
                    if ret < 0 as i32 {
                        xmlRelaxNGAddValidError(
                            ctxt,
                            XML_RELAXNG_ERR_TEXTWRONG,
                            (*(*cur).parent).name,
                            0 as *const xmlChar,
                            0 as i32,
                        );
                    }
                }
            }
            1 => {
                if !((*cur).ns).is_null() {
                    ret = xmlRegExecPushString2(
                        exec,
                        (*cur).name,
                        (*(*cur).ns).href,
                        ctxt as *mut libc::c_void,
                    );
                } else {
                    ret = xmlRegExecPushString(
                        exec,
                        (*cur).name,
                        ctxt as *mut libc::c_void,
                    );
                }
                if ret < 0 as i32 {
                    xmlRelaxNGAddValidError(
                        ctxt,
                        XML_RELAXNG_ERR_ELEMWRONG,
                        (*cur).name,
                        0 as *const xmlChar,
                        0 as i32,
                    );
                }
            }
            _ => {}
        }
        if ret < 0 as i32 {
            break;
        }
        cur = (*cur).next;
    }
    ret = xmlRegExecPushString(exec, 0 as *const xmlChar, 0 as *mut libc::c_void);
    if ret == 1 as i32 {
        ret = 0 as i32;
        let fresh339 = &mut ((*(*ctxt).state).seq);
        *fresh339 = 0 as xmlNodePtr;
    } else if ret == 0 as i32 {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_NOELEM,
            b"\0" as *const u8 as *const i8 as *mut xmlChar,
            0 as *const xmlChar,
            0 as i32,
        );
        ret = -(1 as i32);
        if (*ctxt).flags & 1 as i32 == 0 as i32 {
            xmlRelaxNGDumpValidError(ctxt);
        }
    } else {
        ret = -(1 as i32);
    }
    xmlRegFreeExecCtxt(exec);
    if ret == 0 as i32 && (*ctxt).perr != 0 as i32 {
        ret = (*ctxt).perr;
    }
    (*ctxt).perr = oldperr;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGElemPush(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut exec: xmlRegExecCtxtPtr,
) -> i32 {
    if ((*ctxt).elemTab).is_null() {
        (*ctxt).elemMax = 10 as i32;
        let fresh340 = &mut ((*ctxt).elemTab);
        *fresh340 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).elemMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRegExecCtxtPtr>() as u64,
                ),
        ) as *mut xmlRegExecCtxtPtr;
        if ((*ctxt).elemTab).is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"validating\n\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
    }
    if (*ctxt).elemNr >= (*ctxt).elemMax {
        (*ctxt).elemMax *= 2 as i32;
        let fresh341 = &mut ((*ctxt).elemTab);
        *fresh341 = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).elemTab as *mut libc::c_void,
            ((*ctxt).elemMax as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRegExecCtxtPtr>() as u64,
                ),
        ) as *mut xmlRegExecCtxtPtr;
        if ((*ctxt).elemTab).is_null() {
            xmlRngVErrMemory(
                ctxt,
                b"validating\n\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
    }
    let fresh342 = &mut ((*ctxt).elemNr);
    let fresh343 = *fresh342;
    *fresh342 = *fresh342 + 1;
    let fresh344 = &mut (*((*ctxt).elemTab).offset(fresh343 as isize));
    *fresh344 = exec;
    let fresh345 = &mut ((*ctxt).elem);
    *fresh345 = exec;
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGElemPop(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
) -> xmlRegExecCtxtPtr {
    let mut ret: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
    if (*ctxt).elemNr <= 0 as i32 {
        return 0 as xmlRegExecCtxtPtr;
    }
    let fresh346 = &mut ((*ctxt).elemNr);
    *fresh346 -= 1;
    ret = *((*ctxt).elemTab).offset((*ctxt).elemNr as isize);
    let fresh347 = &mut (*((*ctxt).elemTab).offset((*ctxt).elemNr as isize));
    *fresh347 = 0 as xmlRegExecCtxtPtr;
    if (*ctxt).elemNr > 0 as i32 {
        let fresh348 = &mut ((*ctxt).elem);
        *fresh348 = *((*ctxt).elemTab)
            .offset(((*ctxt).elemNr - 1 as i32) as isize);
    } else {
        let fresh349 = &mut ((*ctxt).elem);
        *fresh349 = 0 as xmlRegExecCtxtPtr;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGValidateProgressiveCallback(
    mut exec: xmlRegExecCtxtPtr,
    mut token: *const xmlChar,
    mut transdata: *mut libc::c_void,
    mut inputdata: *mut libc::c_void,
) {
    let mut ctxt: xmlRelaxNGValidCtxtPtr = inputdata as xmlRelaxNGValidCtxtPtr;
    let mut define: xmlRelaxNGDefinePtr = transdata as xmlRelaxNGDefinePtr;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut oldstate: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: i32 = 0 as i32;
    let mut oldflags: i32 = 0;
    if ctxt.is_null() {
        fprintf(
            stderr,
            b"callback on %s missing context\n\0" as *const u8 as *const i8,
            token,
        );
        return;
    }
    node = (*ctxt).pnode;
    (*ctxt).pstate = 1 as i32;
    if define.is_null() {
        if *token.offset(0 as i32 as isize) as i32 == '#' as i32 {
            return;
        }
        fprintf(
            stderr,
            b"callback on %s missing define\n\0" as *const u8 as *const i8,
            token,
        );
        if !ctxt.is_null() && (*ctxt).errNo == XML_RELAXNG_OK as i32 {
            (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as i32;
        }
        (*ctxt).pstate = -(1 as i32);
        return;
    }
    if ctxt.is_null() || define.is_null() {
        fprintf(
            stderr,
            b"callback on %s missing info\n\0" as *const u8 as *const i8,
            token,
        );
        if !ctxt.is_null() && (*ctxt).errNo == XML_RELAXNG_OK as i32 {
            (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as i32;
        }
        (*ctxt).pstate = -(1 as i32);
        return;
    } else {
        if (*define).type_0 as i32 != XML_RELAXNG_ELEMENT as i32 {
            fprintf(
                stderr,
                b"callback on %s define is not element\n\0" as *const u8
                    as *const i8,
                token,
            );
            if (*ctxt).errNo == XML_RELAXNG_OK as i32 {
                (*ctxt).errNo = XML_RELAXNG_ERR_INTERNAL as i32;
            }
            (*ctxt).pstate = -(1 as i32);
            return;
        }
    }
    if (*node).type_0 as u32 != XML_ELEMENT_NODE as i32 as u32
    {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_NOTELEM,
            0 as *const xmlChar,
            0 as *const xmlChar,
            0 as i32,
        );
        if (*ctxt).flags & 1 as i32 == 0 as i32 {
            xmlRelaxNGDumpValidError(ctxt);
        }
        (*ctxt).pstate = -(1 as i32);
        return;
    }
    if ((*define).contModel).is_null() {
        (*ctxt).pstate = 0 as i32;
        let fresh350 = &mut ((*ctxt).pdef);
        *fresh350 = define;
        return;
    }
    exec = xmlRegNewExecCtxt(
        (*define).contModel,
        Some(
            xmlRelaxNGValidateProgressiveCallback
                as unsafe extern "C" fn(
                    xmlRegExecCtxtPtr,
                    *const xmlChar,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        ctxt as *mut libc::c_void,
    );
    if exec.is_null() {
        (*ctxt).pstate = -(1 as i32);
        return;
    }
    xmlRelaxNGElemPush(ctxt, exec);
    state = xmlRelaxNGNewValidState(ctxt, node);
    if state.is_null() {
        (*ctxt).pstate = -(1 as i32);
        return;
    }
    oldstate = (*ctxt).state;
    let fresh351 = &mut ((*ctxt).state);
    *fresh351 = state;
    if !((*define).attrs).is_null() {
        ret = xmlRelaxNGValidateAttributeList(ctxt, (*define).attrs);
        if ret != 0 as i32 {
            (*ctxt).pstate = -(1 as i32);
            xmlRelaxNGAddValidError(
                ctxt,
                XML_RELAXNG_ERR_ATTRVALID,
                (*node).name,
                0 as *const xmlChar,
                0 as i32,
            );
        }
    }
    if !((*ctxt).state).is_null() {
        let fresh352 = &mut ((*(*ctxt).state).seq);
        *fresh352 = 0 as xmlNodePtr;
        ret = xmlRelaxNGValidateElementEnd(ctxt, 1 as i32);
        if ret != 0 as i32 {
            (*ctxt).pstate = -(1 as i32);
        }
        xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
    } else if !((*ctxt).states).is_null() {
        let mut tmp: i32 = -(1 as i32);
        let mut i: i32 = 0;
        oldflags = (*ctxt).flags;
        i = 0 as i32;
        while i < (*(*ctxt).states).nbState {
            state = *((*(*ctxt).states).tabState).offset(i as isize);
            let fresh353 = &mut ((*ctxt).state);
            *fresh353 = state;
            let fresh354 = &mut ((*(*ctxt).state).seq);
            *fresh354 = 0 as xmlNodePtr;
            if xmlRelaxNGValidateElementEnd(ctxt, 0 as i32) == 0 as i32 {
                tmp = 0 as i32;
                break;
            } else {
                i += 1;
            }
        }
        if tmp != 0 as i32 {
            (*ctxt).flags |= 1 as i32;
            xmlRelaxNGLogBestError(ctxt);
        }
        i = 0 as i32;
        while i < (*(*ctxt).states).nbState {
            xmlRelaxNGFreeValidState(
                ctxt,
                *((*(*ctxt).states).tabState).offset(i as isize),
            );
            i += 1;
        }
        xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
        let fresh355 = &mut ((*ctxt).states);
        *fresh355 = 0 as xmlRelaxNGStatesPtr;
        if ret == 0 as i32 && tmp == -(1 as i32) {
            (*ctxt).pstate = -(1 as i32);
        }
        (*ctxt).flags = oldflags;
    }
    if (*ctxt).pstate == -(1 as i32) {
        if (*ctxt).flags & 1 as i32 == 0 as i32 {
            xmlRelaxNGDumpValidError(ctxt);
        }
    }
    let fresh356 = &mut ((*ctxt).state);
    *fresh356 = oldstate;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidatePushElement(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
) -> i32 {
    let mut ret: i32 = 1 as i32;
    if ctxt.is_null() || elem.is_null() {
        return -(1 as i32);
    }
    if ((*ctxt).elem).is_null() {
        let mut schema: xmlRelaxNGPtr = 0 as *mut xmlRelaxNG;
        let mut grammar: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
        let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
        let mut define: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        schema = (*ctxt).schema;
        if schema.is_null() {
            xmlRelaxNGAddValidError(
                ctxt,
                XML_RELAXNG_ERR_NOGRAMMAR,
                0 as *const xmlChar,
                0 as *const xmlChar,
                0 as i32,
            );
            return -(1 as i32);
        }
        grammar = (*schema).topgrammar;
        if grammar.is_null() || ((*grammar).start).is_null() {
            xmlRelaxNGAddValidError(
                ctxt,
                XML_RELAXNG_ERR_NOGRAMMAR,
                0 as *const xmlChar,
                0 as *const xmlChar,
                0 as i32,
            );
            return -(1 as i32);
        }
        define = (*grammar).start;
        if ((*define).contModel).is_null() {
            let fresh357 = &mut ((*ctxt).pdef);
            *fresh357 = define;
            return 0 as i32;
        }
        exec = xmlRegNewExecCtxt(
            (*define).contModel,
            Some(
                xmlRelaxNGValidateProgressiveCallback
                    as unsafe extern "C" fn(
                        xmlRegExecCtxtPtr,
                        *const xmlChar,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            ctxt as *mut libc::c_void,
        );
        if exec.is_null() {
            return -(1 as i32);
        }
        xmlRelaxNGElemPush(ctxt, exec);
    }
    let fresh358 = &mut ((*ctxt).pnode);
    *fresh358 = elem;
    (*ctxt).pstate = 0 as i32;
    if !((*elem).ns).is_null() {
        ret = xmlRegExecPushString2(
            (*ctxt).elem,
            (*elem).name,
            (*(*elem).ns).href,
            ctxt as *mut libc::c_void,
        );
    } else {
        ret = xmlRegExecPushString(
            (*ctxt).elem,
            (*elem).name,
            ctxt as *mut libc::c_void,
        );
    }
    if ret < 0 as i32 {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_ELEMWRONG,
            (*elem).name,
            0 as *const xmlChar,
            0 as i32,
        );
    } else if (*ctxt).pstate == 0 as i32 {
        ret = 0 as i32;
    } else if (*ctxt).pstate < 0 as i32 {
        ret = -(1 as i32);
    } else {
        ret = 1 as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidatePushCData(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut data: *const xmlChar,
    mut len: i32,
) -> i32 {
    let mut ret: i32 = 1 as i32;
    if ctxt.is_null() || ((*ctxt).elem).is_null() || data.is_null() {
        return -(1 as i32);
    }
    while *data as i32 != 0 as i32 {
        if !(*data as i32 == 0x20 as i32
            || 0x9 as i32 <= *data as i32
                && *data as i32 <= 0xa as i32
            || *data as i32 == 0xd as i32)
        {
            break;
        }
        data = data.offset(1);
    }
    if *data as i32 == 0 as i32 {
        return 1 as i32;
    }
    ret = xmlRegExecPushString(
        (*ctxt).elem,
        b"#text\0" as *const u8 as *const i8 as *mut xmlChar,
        ctxt as *mut libc::c_void,
    );
    if ret < 0 as i32 {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_TEXTWRONG,
            b" TODO \0" as *const u8 as *const i8 as *mut xmlChar,
            0 as *const xmlChar,
            0 as i32,
        );
        return -(1 as i32);
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidatePopElement(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
) -> i32 {
    let mut ret: i32 = 0;
    let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
    if ctxt.is_null() || ((*ctxt).elem).is_null() || elem.is_null() {
        return -(1 as i32);
    }
    exec = xmlRelaxNGElemPop(ctxt);
    ret = xmlRegExecPushString(exec, 0 as *const xmlChar, 0 as *mut libc::c_void);
    if ret == 0 as i32 {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_NOELEM,
            b"\0" as *const u8 as *const i8 as *mut xmlChar,
            0 as *const xmlChar,
            0 as i32,
        );
        ret = -(1 as i32);
    } else if ret < 0 as i32 {
        ret = -(1 as i32);
    } else {
        ret = 1 as i32;
    }
    xmlRegFreeExecCtxt(exec);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidateFullElement(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut doc: xmlDocPtr,
    mut elem: xmlNodePtr,
) -> i32 {
    let mut ret: i32 = 0;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    if ctxt.is_null() || ((*ctxt).pdef).is_null() || elem.is_null() {
        return -(1 as i32);
    }
    state = xmlRelaxNGNewValidState(ctxt, (*elem).parent);
    if state.is_null() {
        return -(1 as i32);
    }
    let fresh359 = &mut ((*state).seq);
    *fresh359 = elem;
    let fresh360 = &mut ((*ctxt).state);
    *fresh360 = state;
    (*ctxt).errNo = XML_RELAXNG_OK as i32;
    ret = xmlRelaxNGValidateDefinition(ctxt, (*ctxt).pdef);
    if ret != 0 as i32 || (*ctxt).errNo != XML_RELAXNG_OK as i32 {
        ret = -(1 as i32);
    } else {
        ret = 1 as i32;
    }
    xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
    let fresh361 = &mut ((*ctxt).state);
    *fresh361 = 0 as xmlRelaxNGValidStatePtr;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGSkipIgnored(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut node: xmlNodePtr,
) -> xmlNodePtr {
    while !node.is_null()
        && ((*node).type_0 as u32
            == XML_COMMENT_NODE as i32 as u32
            || (*node).type_0 as u32
                == XML_PI_NODE as i32 as u32
            || (*node).type_0 as u32
                == XML_XINCLUDE_START as i32 as u32
            || (*node).type_0 as u32
                == XML_XINCLUDE_END as i32 as u32
            || ((*node).type_0 as u32
                == XML_TEXT_NODE as i32 as u32
                || (*node).type_0 as u32
                    == XML_CDATA_SECTION_NODE as i32 as u32)
                && ((*ctxt).flags & 4 as i32 != 0
                    || xmlRelaxNGIsBlank((*node).content) != 0))
    {
        node = (*node).next;
    }
    return node;
}
unsafe extern "C" fn xmlRelaxNGNormalize(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut str: *const xmlChar,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut p: *mut xmlChar = 0 as *mut xmlChar;
    let mut tmp: *const xmlChar = 0 as *const xmlChar;
    let mut len: i32 = 0;
    if str.is_null() {
        return 0 as *mut xmlChar;
    }
    tmp = str;
    while *tmp as i32 != 0 as i32 {
        tmp = tmp.offset(1);
    }
    len = tmp.offset_from(str) as i64 as i32;
    ret = xmlMallocAtomic
        .expect(
            "non-null function pointer",
        )(
        ((len + 1 as i32) as u64)
            .wrapping_mul(::std::mem::size_of::<xmlChar>() as u64),
    ) as *mut xmlChar;
    if ret.is_null() {
        xmlRngVErrMemory(ctxt, b"validating\n\0" as *const u8 as *const i8);
        return 0 as *mut xmlChar;
    }
    p = ret;
    while *str as i32 == 0x20 as i32
        || 0x9 as i32 <= *str as i32
            && *str as i32 <= 0xa as i32
        || *str as i32 == 0xd as i32
    {
        str = str.offset(1);
    }
    while *str as i32 != 0 as i32 {
        if *str as i32 == 0x20 as i32
            || 0x9 as i32 <= *str as i32
                && *str as i32 <= 0xa as i32
            || *str as i32 == 0xd as i32
        {
            while *str as i32 == 0x20 as i32
                || 0x9 as i32 <= *str as i32
                    && *str as i32 <= 0xa as i32
                || *str as i32 == 0xd as i32
            {
                str = str.offset(1);
            }
            if *str as i32 == 0 as i32 {
                break;
            }
            let fresh362 = p;
            p = p.offset(1);
            *fresh362 = ' ' as i32 as xmlChar;
        } else {
            let fresh363 = str;
            str = str.offset(1);
            let fresh364 = p;
            p = p.offset(1);
            *fresh364 = *fresh363;
        }
    }
    *p = 0 as i32 as xmlChar;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGValidateDatatype(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut value: *const xmlChar,
    mut define: xmlRelaxNGDefinePtr,
    mut node: xmlNodePtr,
) -> i32 {
    let mut ret: i32 = 0;
    let mut tmp: i32 = 0;
    let mut lib: xmlRelaxNGTypeLibraryPtr = 0 as *mut xmlRelaxNGTypeLibrary;
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    if define.is_null() || ((*define).data).is_null() {
        return -(1 as i32);
    }
    lib = (*define).data as xmlRelaxNGTypeLibraryPtr;
    if ((*lib).check).is_some() {
        if !((*define).attrs).is_null()
            && (*(*define).attrs).type_0 as i32
                == XML_RELAXNG_PARAM as i32
        {
            ret = ((*lib).check)
                .expect(
                    "non-null function pointer",
                )((*lib).data, (*define).name, value, &mut result, node);
        } else {
            ret = ((*lib).check)
                .expect(
                    "non-null function pointer",
                )((*lib).data, (*define).name, value, 0 as *mut *mut libc::c_void, node);
        }
    } else {
        ret = -(1 as i32);
    }
    if ret < 0 as i32 {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_TYPE,
            (*define).name,
            0 as *const xmlChar,
            0 as i32,
        );
        if !result.is_null() && !lib.is_null() && ((*lib).freef).is_some() {
            ((*lib).freef).expect("non-null function pointer")((*lib).data, result);
        }
        return -(1 as i32);
    } else {
        if ret == 1 as i32 {
            ret = 0 as i32;
        } else if ret == 2 as i32 {
            xmlRelaxNGAddValidError(
                ctxt,
                XML_RELAXNG_ERR_DUPID,
                value,
                0 as *const xmlChar,
                1 as i32,
            );
        } else {
            xmlRelaxNGAddValidError(
                ctxt,
                XML_RELAXNG_ERR_TYPEVAL,
                (*define).name,
                value,
                1 as i32,
            );
            ret = -(1 as i32);
        }
    }
    cur = (*define).attrs;
    while ret == 0 as i32 && !cur.is_null()
        && (*cur).type_0 as i32 == XML_RELAXNG_PARAM as i32
    {
        if ((*lib).facet).is_some() {
            tmp = ((*lib).facet)
                .expect(
                    "non-null function pointer",
                )((*lib).data, (*define).name, (*cur).name, (*cur).value, value, result);
            if tmp != 0 as i32 {
                ret = -(1 as i32);
            }
        }
        cur = (*cur).next;
    }
    if ret == 0 as i32 && !((*define).content).is_null() {
        let mut oldvalue: *const xmlChar = 0 as *const xmlChar;
        let mut oldendvalue: *const xmlChar = 0 as *const xmlChar;
        oldvalue = (*(*ctxt).state).value;
        oldendvalue = (*(*ctxt).state).endvalue;
        let fresh365 = &mut ((*(*ctxt).state).value);
        *fresh365 = value as *mut xmlChar;
        let fresh366 = &mut ((*(*ctxt).state).endvalue);
        *fresh366 = 0 as *mut xmlChar;
        ret = xmlRelaxNGValidateValue(ctxt, (*define).content);
        let fresh367 = &mut ((*(*ctxt).state).value);
        *fresh367 = oldvalue as *mut xmlChar;
        let fresh368 = &mut ((*(*ctxt).state).endvalue);
        *fresh368 = oldendvalue as *mut xmlChar;
    }
    if !result.is_null() && !lib.is_null() && ((*lib).freef).is_some() {
        ((*lib).freef).expect("non-null function pointer")((*lib).data, result);
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGNextValue(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
) -> i32 {
    let mut cur: *mut xmlChar = 0 as *mut xmlChar;
    cur = (*(*ctxt).state).value;
    if cur.is_null() || ((*(*ctxt).state).endvalue).is_null() {
        let fresh369 = &mut ((*(*ctxt).state).value);
        *fresh369 = 0 as *mut xmlChar;
        let fresh370 = &mut ((*(*ctxt).state).endvalue);
        *fresh370 = 0 as *mut xmlChar;
        return 0 as i32;
    }
    while *cur as i32 != 0 as i32 {
        cur = cur.offset(1);
    }
    while cur != (*(*ctxt).state).endvalue && *cur as i32 == 0 as i32 {
        cur = cur.offset(1);
    }
    if cur == (*(*ctxt).state).endvalue {
        let fresh371 = &mut ((*(*ctxt).state).value);
        *fresh371 = 0 as *mut xmlChar;
    } else {
        let fresh372 = &mut ((*(*ctxt).state).value);
        *fresh372 = cur;
    }
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGValidateValueList(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut defines: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    while !defines.is_null() {
        ret = xmlRelaxNGValidateValue(ctxt, defines);
        if ret != 0 as i32 {
            break;
        }
        defines = (*defines).next;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGValidateValue(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut define: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut oldflags: i32 = 0;
    let mut value: *mut xmlChar = 0 as *mut xmlChar;
    value = (*(*ctxt).state).value;
    let mut current_block_141: u64;
    match (*define).type_0 as i32 {
        0 => {
            if !value.is_null()
                && *value.offset(0 as i32 as isize) as i32
                    != 0 as i32
            {
                let mut idx: i32 = 0 as i32;
                while *value.offset(idx as isize) as i32 == 0x20 as i32
                    || 0x9 as i32 <= *value.offset(idx as isize) as i32
                        && *value.offset(idx as isize) as i32
                            <= 0xa as i32
                    || *value.offset(idx as isize) as i32 == 0xd as i32
                {
                    idx += 1;
                }
                if *value.offset(idx as isize) as i32 != 0 as i32 {
                    ret = -(1 as i32);
                }
            }
            current_block_141 = 6328367678128271922;
        }
        3 => {
            current_block_141 = 6328367678128271922;
        }
        7 => {
            if xmlStrEqual(value, (*define).value) == 0 {
                if !((*define).name).is_null() {
                    let mut lib: xmlRelaxNGTypeLibraryPtr = 0
                        as *mut xmlRelaxNGTypeLibrary;
                    lib = (*define).data as xmlRelaxNGTypeLibraryPtr;
                    if !lib.is_null() && ((*lib).comp).is_some() {
                        ret = ((*lib).comp)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*lib).data,
                            (*define).name,
                            (*define).value,
                            (*define).node,
                            (*define).attrs as *mut libc::c_void,
                            value,
                            (*(*ctxt).state).node,
                        );
                    } else {
                        ret = -(1 as i32);
                    }
                    if ret < 0 as i32 {
                        xmlRelaxNGAddValidError(
                            ctxt,
                            XML_RELAXNG_ERR_TYPECMP,
                            (*define).name,
                            0 as *const xmlChar,
                            0 as i32,
                        );
                        return -(1 as i32);
                    } else {
                        if ret == 1 as i32 {
                            ret = 0 as i32;
                        } else {
                            ret = -(1 as i32);
                        }
                    }
                } else {
                    let mut nval: *mut xmlChar = 0 as *mut xmlChar;
                    let mut nvalue: *mut xmlChar = 0 as *mut xmlChar;
                    nval = xmlRelaxNGNormalize(ctxt, (*define).value);
                    nvalue = xmlRelaxNGNormalize(ctxt, value);
                    if nval.is_null() || nvalue.is_null()
                        || xmlStrEqual(nval, nvalue) == 0
                    {
                        ret = -(1 as i32);
                    }
                    if !nval.is_null() {
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(nval as *mut libc::c_void);
                    }
                    if !nvalue.is_null() {
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(nvalue as *mut libc::c_void);
                    }
                }
            }
            if ret == 0 as i32 {
                xmlRelaxNGNextValue(ctxt);
            }
            current_block_141 = 6328367678128271922;
        }
        5 => {
            ret = xmlRelaxNGValidateDatatype(ctxt, value, define, (*(*ctxt).state).seq);
            if ret == 0 as i32 {
                xmlRelaxNGNextValue(ctxt);
            }
            current_block_141 = 6328367678128271922;
        }
        17 => {
            let mut list: xmlRelaxNGDefinePtr = (*define).content;
            let mut oldvalue: *mut xmlChar = 0 as *mut xmlChar;
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= 1 as i32;
            oldvalue = (*(*ctxt).state).value;
            while !list.is_null() {
                ret = xmlRelaxNGValidateValue(ctxt, list);
                if ret == 0 as i32 {
                    break;
                }
                let fresh373 = &mut ((*(*ctxt).state).value);
                *fresh373 = oldvalue;
                list = (*list).next;
            }
            (*ctxt).flags = oldflags;
            if ret != 0 as i32 {
                if (*ctxt).flags & 1 as i32 == 0 as i32 {
                    xmlRelaxNGDumpValidError(ctxt);
                }
            } else if (*ctxt).errNr > 0 as i32 {
                xmlRelaxNGPopErrors(ctxt, 0 as i32);
            }
            current_block_141 = 6328367678128271922;
        }
        8 => {
            let mut list_0: xmlRelaxNGDefinePtr = (*define).content;
            let mut oldvalue_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut oldend: *mut xmlChar = 0 as *mut xmlChar;
            let mut val: *mut xmlChar = 0 as *mut xmlChar;
            let mut cur: *mut xmlChar = 0 as *mut xmlChar;
            oldvalue_0 = (*(*ctxt).state).value;
            oldend = (*(*ctxt).state).endvalue;
            val = xmlStrdup(oldvalue_0);
            if val.is_null() {
                val = xmlStrdup(
                    b"\0" as *const u8 as *const i8 as *mut xmlChar,
                );
            }
            if val.is_null() {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_NOSTATE,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                    0 as i32,
                );
                return -(1 as i32);
            }
            cur = val;
            while *cur as i32 != 0 as i32 {
                if *cur as i32 == 0x20 as i32
                    || 0x9 as i32 <= *cur as i32
                        && *cur as i32 <= 0xa as i32
                    || *cur as i32 == 0xd as i32
                {
                    *cur = 0 as i32 as xmlChar;
                    cur = cur.offset(1);
                    while *cur as i32 == 0x20 as i32
                        || 0x9 as i32 <= *cur as i32
                            && *cur as i32 <= 0xa as i32
                        || *cur as i32 == 0xd as i32
                    {
                        let fresh374 = cur;
                        cur = cur.offset(1);
                        *fresh374 = 0 as i32 as xmlChar;
                    }
                } else {
                    cur = cur.offset(1);
                }
            }
            let fresh375 = &mut ((*(*ctxt).state).endvalue);
            *fresh375 = cur;
            cur = val;
            while *cur as i32 == 0 as i32
                && cur != (*(*ctxt).state).endvalue
            {
                cur = cur.offset(1);
            }
            let fresh376 = &mut ((*(*ctxt).state).value);
            *fresh376 = cur;
            while !list_0.is_null() {
                if (*(*ctxt).state).value == (*(*ctxt).state).endvalue {
                    let fresh377 = &mut ((*(*ctxt).state).value);
                    *fresh377 = 0 as *mut xmlChar;
                }
                ret = xmlRelaxNGValidateValue(ctxt, list_0);
                if ret != 0 as i32 {
                    break;
                }
                list_0 = (*list_0).next;
            }
            if ret == 0 as i32 && !((*(*ctxt).state).value).is_null()
                && (*(*ctxt).state).value != (*(*ctxt).state).endvalue
            {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_LISTEXTRA,
                    (*(*ctxt).state).value,
                    0 as *const xmlChar,
                    0 as i32,
                );
                ret = -(1 as i32);
            }
            xmlFree.expect("non-null function pointer")(val as *mut libc::c_void);
            let fresh378 = &mut ((*(*ctxt).state).value);
            *fresh378 = oldvalue_0;
            let fresh379 = &mut ((*(*ctxt).state).endvalue);
            *fresh379 = oldend;
            current_block_141 = 6328367678128271922;
        }
        16 => {
            ret = xmlRelaxNGValidateValueList(ctxt, (*define).content);
            if ret != 0 as i32 {
                current_block_141 = 6328367678128271922;
            } else {
                current_block_141 = 9521147444787763968;
            }
        }
        15 => {
            current_block_141 = 9521147444787763968;
        }
        14 => {
            let mut temp_0: *mut xmlChar = 0 as *mut xmlChar;
            if ((*(*ctxt).state).value).is_null()
                || *(*(*ctxt).state).value as i32 == 0 as i32
            {
                ret = 0 as i32;
            } else {
                oldflags = (*ctxt).flags;
                (*ctxt).flags |= 1 as i32;
                temp_0 = (*(*ctxt).state).value;
                ret = xmlRelaxNGValidateValue(ctxt, (*define).content);
                (*ctxt).flags = oldflags;
                if ret != 0 as i32 {
                    let fresh381 = &mut ((*(*ctxt).state).value);
                    *fresh381 = temp_0;
                    if (*ctxt).errNr > 0 as i32 {
                        xmlRelaxNGPopErrors(ctxt, 0 as i32);
                    }
                    ret = 0 as i32;
                } else if (*ctxt).errNr > 0 as i32 {
                    xmlRelaxNGPopErrors(ctxt, 0 as i32);
                }
            }
            current_block_141 = 6328367678128271922;
        }
        2 => {
            let mut list_1: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
            list_1 = (*define).content;
            while !list_1.is_null() {
                ret = xmlRelaxNGValidateValue(ctxt, list_1);
                if ret == 0 as i32 {
                    ret = -(1 as i32);
                    break;
                } else {
                    ret = 0 as i32;
                    list_1 = (*list_1).next;
                }
            }
            current_block_141 = 6328367678128271922;
        }
        10 | 18 => {
            let mut list_2: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
            list_2 = (*define).content;
            while !list_2.is_null() {
                ret = xmlRelaxNGValidateValue(ctxt, list_2);
                if ret != 0 as i32 {
                    ret = -(1 as i32);
                    break;
                } else {
                    ret = 0 as i32;
                    list_2 = (*list_2).next;
                }
            }
            current_block_141 = 6328367678128271922;
        }
        11 | 13 => {
            if ((*define).content).is_null() {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_NODEFINE,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                    0 as i32,
                );
                ret = -(1 as i32);
            } else {
                ret = xmlRelaxNGValidateValue(ctxt, (*define).content);
            }
            current_block_141 = 6328367678128271922;
        }
        _ => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
                b"relaxng.c\0" as *const u8 as *const i8,
                9025 as i32,
            );
            ret = -(1 as i32);
            current_block_141 = 6328367678128271922;
        }
    }
    match current_block_141 {
        9521147444787763968 => {
            let mut cur_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut temp: *mut xmlChar = 0 as *mut xmlChar;
            if ((*(*ctxt).state).value).is_null()
                || *(*(*ctxt).state).value as i32 == 0 as i32
            {
                ret = 0 as i32;
            } else {
                oldflags = (*ctxt).flags;
                (*ctxt).flags |= 1 as i32;
                cur_0 = (*(*ctxt).state).value;
                temp = 0 as *mut xmlChar;
                while !cur_0.is_null() && cur_0 != (*(*ctxt).state).endvalue
                    && temp != cur_0
                {
                    temp = cur_0;
                    ret = xmlRelaxNGValidateValueList(ctxt, (*define).content);
                    if ret != 0 as i32 {
                        let fresh380 = &mut ((*(*ctxt).state).value);
                        *fresh380 = temp;
                        ret = 0 as i32;
                        break;
                    } else {
                        cur_0 = (*(*ctxt).state).value;
                    }
                }
                (*ctxt).flags = oldflags;
                if (*ctxt).errNr > 0 as i32 {
                    xmlRelaxNGPopErrors(ctxt, 0 as i32);
                }
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGValidateValueContent(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut defines: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    while !defines.is_null() {
        ret = xmlRelaxNGValidateValue(ctxt, defines);
        if ret != 0 as i32 {
            break;
        }
        defines = (*defines).next;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGAttributeMatch(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut define: xmlRelaxNGDefinePtr,
    mut prop: xmlAttrPtr,
) -> i32 {
    let mut ret: i32 = 0;
    if !((*define).name).is_null() {
        if xmlStrEqual((*define).name, (*prop).name) == 0 {
            return 0 as i32;
        }
    }
    if !((*define).ns).is_null() {
        if *((*define).ns).offset(0 as i32 as isize) as i32
            == 0 as i32
        {
            if !((*prop).ns).is_null() {
                return 0 as i32;
            }
        } else if ((*prop).ns).is_null()
                || xmlStrEqual((*define).ns, (*(*prop).ns).href) == 0
            {
            return 0 as i32
        }
    }
    if ((*define).nameClass).is_null() {
        return 1 as i32;
    }
    define = (*define).nameClass;
    if (*define).type_0 as i32 == XML_RELAXNG_EXCEPT as i32 {
        let mut list: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        list = (*define).content;
        while !list.is_null() {
            ret = xmlRelaxNGAttributeMatch(ctxt, list, prop);
            if ret == 1 as i32 {
                return 0 as i32;
            }
            if ret < 0 as i32 {
                return ret;
            }
            list = (*list).next;
        }
    } else if (*define).type_0 as i32 == XML_RELAXNG_CHOICE as i32 {
        let mut list_0: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        list_0 = (*define).nameClass;
        while !list_0.is_null() {
            ret = xmlRelaxNGAttributeMatch(ctxt, list_0, prop);
            if ret == 1 as i32 {
                return 1 as i32;
            }
            if ret < 0 as i32 {
                return ret;
            }
            list_0 = (*list_0).next;
        }
        return 0 as i32;
    } else {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
            b"relaxng.c\0" as *const u8 as *const i8,
            9113 as i32,
        );
    }
    return 1 as i32;
}
unsafe extern "C" fn xmlRelaxNGValidateAttribute(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut define: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut value: *mut xmlChar = 0 as *mut xmlChar;
    let mut oldvalue: *mut xmlChar = 0 as *mut xmlChar;
    let mut prop: xmlAttrPtr = 0 as xmlAttrPtr;
    let mut tmp: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut oldseq: xmlNodePtr = 0 as *mut xmlNode;
    if (*(*ctxt).state).nbAttrLeft <= 0 as i32 {
        return -(1 as i32);
    }
    if !((*define).name).is_null() {
        i = 0 as i32;
        while i < (*(*ctxt).state).nbAttrs {
            tmp = *((*(*ctxt).state).attrs).offset(i as isize);
            if !tmp.is_null() && xmlStrEqual((*define).name, (*tmp).name) != 0 {
                if (((*define).ns).is_null()
                    || *((*define).ns).offset(0 as i32 as isize) as i32
                        == 0 as i32) && ((*tmp).ns).is_null()
                    || !((*tmp).ns).is_null()
                        && xmlStrEqual((*define).ns, (*(*tmp).ns).href) != 0
                {
                    prop = tmp;
                    break;
                }
            }
            i += 1;
        }
        if !prop.is_null() {
            value = xmlNodeListGetString(
                (*prop).doc,
                (*prop).children,
                1 as i32,
            );
            oldvalue = (*(*ctxt).state).value;
            oldseq = (*(*ctxt).state).seq;
            let fresh382 = &mut ((*(*ctxt).state).seq);
            *fresh382 = prop as xmlNodePtr;
            let fresh383 = &mut ((*(*ctxt).state).value);
            *fresh383 = value;
            let fresh384 = &mut ((*(*ctxt).state).endvalue);
            *fresh384 = 0 as *mut xmlChar;
            ret = xmlRelaxNGValidateValueContent(ctxt, (*define).content);
            if !((*(*ctxt).state).value).is_null() {
                value = (*(*ctxt).state).value;
            }
            if !value.is_null() {
                xmlFree.expect("non-null function pointer")(value as *mut libc::c_void);
            }
            let fresh385 = &mut ((*(*ctxt).state).value);
            *fresh385 = oldvalue;
            let fresh386 = &mut ((*(*ctxt).state).seq);
            *fresh386 = oldseq;
            if ret == 0 as i32 {
                let fresh387 = &mut (*((*(*ctxt).state).attrs).offset(i as isize));
                *fresh387 = 0 as xmlAttrPtr;
                let fresh388 = &mut ((*(*ctxt).state).nbAttrLeft);
                *fresh388 -= 1;
            }
        } else {
            ret = -(1 as i32);
        }
    } else {
        i = 0 as i32;
        while i < (*(*ctxt).state).nbAttrs {
            tmp = *((*(*ctxt).state).attrs).offset(i as isize);
            if !tmp.is_null()
                && xmlRelaxNGAttributeMatch(ctxt, define, tmp) == 1 as i32
            {
                prop = tmp;
                break;
            } else {
                i += 1;
            }
        }
        if !prop.is_null() {
            value = xmlNodeListGetString(
                (*prop).doc,
                (*prop).children,
                1 as i32,
            );
            oldvalue = (*(*ctxt).state).value;
            oldseq = (*(*ctxt).state).seq;
            let fresh389 = &mut ((*(*ctxt).state).seq);
            *fresh389 = prop as xmlNodePtr;
            let fresh390 = &mut ((*(*ctxt).state).value);
            *fresh390 = value;
            ret = xmlRelaxNGValidateValueContent(ctxt, (*define).content);
            if !((*(*ctxt).state).value).is_null() {
                value = (*(*ctxt).state).value;
            }
            if !value.is_null() {
                xmlFree.expect("non-null function pointer")(value as *mut libc::c_void);
            }
            let fresh391 = &mut ((*(*ctxt).state).value);
            *fresh391 = oldvalue;
            let fresh392 = &mut ((*(*ctxt).state).seq);
            *fresh392 = oldseq;
            if ret == 0 as i32 {
                let fresh393 = &mut (*((*(*ctxt).state).attrs).offset(i as isize));
                *fresh393 = 0 as xmlAttrPtr;
                let fresh394 = &mut ((*(*ctxt).state).nbAttrLeft);
                *fresh394 -= 1;
            }
        } else {
            ret = -(1 as i32);
        }
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGValidateAttributeList(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut defines: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut res: i32 = 0;
    let mut needmore: i32 = 0 as i32;
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    cur = defines;
    while !cur.is_null() {
        if (*cur).type_0 as i32 == XML_RELAXNG_ATTRIBUTE as i32 {
            if xmlRelaxNGValidateAttribute(ctxt, cur) != 0 as i32 {
                ret = -(1 as i32);
            }
        } else {
            needmore = 1 as i32;
        }
        cur = (*cur).next;
    }
    if needmore == 0 {
        return ret;
    }
    cur = defines;
    while !cur.is_null() {
        if (*cur).type_0 as i32 != XML_RELAXNG_ATTRIBUTE as i32 {
            if !((*ctxt).state).is_null() || !((*ctxt).states).is_null() {
                res = xmlRelaxNGValidateDefinition(ctxt, cur);
                if res < 0 as i32 {
                    ret = -(1 as i32);
                }
            } else {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_NOSTATE,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                    0 as i32,
                );
                return -(1 as i32);
            }
            if res == -(1 as i32) {
                break;
            }
        }
        cur = (*cur).next;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGNodeMatchesList(
    mut node: xmlNodePtr,
    mut list: *mut xmlRelaxNGDefinePtr,
) -> i32 {
    let mut cur: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
    let mut i: i32 = 0 as i32;
    let mut tmp: i32 = 0;
    if node.is_null() || list.is_null() {
        return 0 as i32;
    }
    let fresh395 = i;
    i = i + 1;
    cur = *list.offset(fresh395 as isize);
    while !cur.is_null() {
        if (*node).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
            && (*cur).type_0 as i32 == XML_RELAXNG_ELEMENT as i32
        {
            tmp = xmlRelaxNGElementMatch(0 as xmlRelaxNGValidCtxtPtr, cur, node);
            if tmp == 1 as i32 {
                return 1 as i32;
            }
        } else if ((*node).type_0 as u32
                == XML_TEXT_NODE as i32 as u32
                || (*node).type_0 as u32
                    == XML_CDATA_SECTION_NODE as i32 as u32)
                && ((*cur).type_0 as i32 == XML_RELAXNG_DATATYPE as i32
                    || (*cur).type_0 as i32 == XML_RELAXNG_LIST as i32
                    || (*cur).type_0 as i32 == XML_RELAXNG_TEXT as i32
                    || (*cur).type_0 as i32 == XML_RELAXNG_VALUE as i32)
            {
            return 1 as i32
        }
        let fresh396 = i;
        i = i + 1;
        cur = *list.offset(fresh396 as isize);
    }
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGValidateInterleave(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut define: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut nbgroups: i32 = 0;
    let mut errNr: i32 = (*ctxt).errNr;
    let mut oldflags: i32 = 0;
    let mut oldstate: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut partitions: xmlRelaxNGPartitionPtr = 0 as *mut xmlRelaxNGPartition;
    let mut group: xmlRelaxNGInterleaveGroupPtr = 0 as xmlRelaxNGInterleaveGroupPtr;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut start: xmlNodePtr = 0 as *mut xmlNode;
    let mut last: xmlNodePtr = 0 as xmlNodePtr;
    let mut lastchg: xmlNodePtr = 0 as xmlNodePtr;
    let mut lastelem: xmlNodePtr = 0 as *mut xmlNode;
    let mut list: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
    let mut lasts: *mut xmlNodePtr = 0 as *mut xmlNodePtr;
    if !((*define).data).is_null() {
        partitions = (*define).data as xmlRelaxNGPartitionPtr;
        nbgroups = (*partitions).nbgroups;
    } else {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_INTERNODATA,
            0 as *const xmlChar,
            0 as *const xmlChar,
            0 as i32,
        );
        return -(1 as i32);
    }
    oldflags = (*ctxt).flags;
    if (*define).dflags as i32 & (1 as i32) << 3 as i32 != 0 {
        (*ctxt).flags |= 4 as i32;
        if nbgroups == 2 as i32 {
            if !((*ctxt).state).is_null() {
                let fresh397 = &mut ((*(*ctxt).state).seq);
                *fresh397 = xmlRelaxNGSkipIgnored(ctxt, (*(*ctxt).state).seq);
            }
            if (*(**((*partitions).groups).offset(0 as i32 as isize)).rule)
                .type_0 as i32 == XML_RELAXNG_TEXT as i32
            {
                ret = xmlRelaxNGValidateDefinition(
                    ctxt,
                    (**((*partitions).groups).offset(1 as i32 as isize)).rule,
                );
            } else {
                ret = xmlRelaxNGValidateDefinition(
                    ctxt,
                    (**((*partitions).groups).offset(0 as i32 as isize)).rule,
                );
            }
            if ret == 0 as i32 {
                if !((*ctxt).state).is_null() {
                    let fresh398 = &mut ((*(*ctxt).state).seq);
                    *fresh398 = xmlRelaxNGSkipIgnored(ctxt, (*(*ctxt).state).seq);
                }
            }
            (*ctxt).flags = oldflags;
            return ret;
        }
    }
    list = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (nbgroups as u64)
            .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
    ) as *mut xmlNodePtr;
    if list.is_null() {
        xmlRngVErrMemory(ctxt, b"validating\n\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    memset(
        list as *mut libc::c_void,
        0 as i32,
        (nbgroups as u64)
            .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
    );
    lasts = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (nbgroups as u64)
            .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
    ) as *mut xmlNodePtr;
    if lasts.is_null() {
        xmlRngVErrMemory(ctxt, b"validating\n\0" as *const u8 as *const i8);
        return -(1 as i32);
    }
    memset(
        lasts as *mut libc::c_void,
        0 as i32,
        (nbgroups as u64)
            .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
    );
    cur = (*(*ctxt).state).seq;
    cur = xmlRelaxNGSkipIgnored(ctxt, cur);
    start = cur;
    while !cur.is_null() {
        let fresh399 = &mut ((*(*ctxt).state).seq);
        *fresh399 = cur;
        if !((*partitions).triage).is_null()
            && (*partitions).flags & 1 as i32 != 0
        {
            let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
            if (*cur).type_0 as u32
                == XML_TEXT_NODE as i32 as u32
                || (*cur).type_0 as u32
                    == XML_CDATA_SECTION_NODE as i32 as u32
            {
                tmp = xmlHashLookup2(
                    (*partitions).triage,
                    b"#text\0" as *const u8 as *const i8 as *mut xmlChar,
                    0 as *const xmlChar,
                );
            } else if (*cur).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                {
                if !((*cur).ns).is_null() {
                    tmp = xmlHashLookup2(
                        (*partitions).triage,
                        (*cur).name,
                        (*(*cur).ns).href,
                    );
                    if tmp.is_null() {
                        tmp = xmlHashLookup2(
                            (*partitions).triage,
                            b"#any\0" as *const u8 as *const i8
                                as *mut xmlChar,
                            (*(*cur).ns).href,
                        );
                    }
                } else {
                    tmp = xmlHashLookup2(
                        (*partitions).triage,
                        (*cur).name,
                        0 as *const xmlChar,
                    );
                }
                if tmp.is_null() {
                    tmp = xmlHashLookup2(
                        (*partitions).triage,
                        b"#any\0" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    );
                }
            }
            if tmp.is_null() {
                i = nbgroups;
            } else {
                i = (tmp as ptrdiff_t - 1 as i32 as i64) as i32;
                if (*partitions).flags & 2 as i32 != 0 {
                    group = *((*partitions).groups).offset(i as isize);
                    if xmlRelaxNGNodeMatchesList(cur, (*group).defs) == 0 {
                        i = nbgroups;
                    }
                }
            }
        } else {
            i = 0 as i32;
            while i < nbgroups {
                group = *((*partitions).groups).offset(i as isize);
                if !group.is_null() {
                    if xmlRelaxNGNodeMatchesList(cur, (*group).defs) != 0 {
                        break;
                    }
                }
                i += 1;
            }
        }
        if i >= nbgroups {
            break;
        }
        if !(*lasts.offset(i as isize)).is_null() {
            let fresh400 = &mut ((**lasts.offset(i as isize)).next);
            *fresh400 = cur;
            let fresh401 = &mut (*lasts.offset(i as isize));
            *fresh401 = cur;
        } else {
            let fresh402 = &mut (*list.offset(i as isize));
            *fresh402 = cur;
            let fresh403 = &mut (*lasts.offset(i as isize));
            *fresh403 = cur;
        }
        if !((*cur).next).is_null() {
            lastchg = (*cur).next;
        } else {
            lastchg = cur;
        }
        cur = xmlRelaxNGSkipIgnored(ctxt, (*cur).next);
    }
    if ret != 0 as i32 {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_INTERSEQ,
            0 as *const xmlChar,
            0 as *const xmlChar,
            0 as i32,
        );
        ret = -(1 as i32);
    } else {
        lastelem = cur;
        oldstate = (*ctxt).state;
        i = 0 as i32;
        loop {
            if !(i < nbgroups) {
                current_block = 5482373152242628851;
                break;
            }
            let fresh404 = &mut ((*ctxt).state);
            *fresh404 = xmlRelaxNGCopyValidState(ctxt, oldstate);
            if ((*ctxt).state).is_null() {
                ret = -(1 as i32);
                current_block = 5482373152242628851;
                break;
            } else {
                group = *((*partitions).groups).offset(i as isize);
                if !(*lasts.offset(i as isize)).is_null() {
                    last = (**lasts.offset(i as isize)).next;
                    let fresh405 = &mut ((**lasts.offset(i as isize)).next);
                    *fresh405 = 0 as *mut _xmlNode;
                }
                let fresh406 = &mut ((*(*ctxt).state).seq);
                *fresh406 = *list.offset(i as isize);
                ret = xmlRelaxNGValidateDefinition(ctxt, (*group).rule);
                if ret != 0 as i32 {
                    current_block = 5482373152242628851;
                    break;
                }
                if !((*ctxt).state).is_null() {
                    cur = (*(*ctxt).state).seq;
                    cur = xmlRelaxNGSkipIgnored(ctxt, cur);
                    xmlRelaxNGFreeValidState(ctxt, oldstate);
                    oldstate = (*ctxt).state;
                    let fresh407 = &mut ((*ctxt).state);
                    *fresh407 = 0 as xmlRelaxNGValidStatePtr;
                    if !cur.is_null()
                        && ((*(*define).parent).type_0 as i32
                            != XML_RELAXNG_DEF as i32
                            || xmlStrEqual(
                                (*(*define).parent).name,
                                b"open-name-class\0" as *const u8 as *const i8
                                    as *const xmlChar,
                            ) == 0)
                    {
                        xmlRelaxNGAddValidError(
                            ctxt,
                            XML_RELAXNG_ERR_INTEREXTRA,
                            (*cur).name,
                            0 as *const xmlChar,
                            0 as i32,
                        );
                        ret = -(1 as i32);
                        let fresh408 = &mut ((*ctxt).state);
                        *fresh408 = oldstate;
                        current_block = 790223741556965965;
                        break;
                    }
                } else if !((*ctxt).states).is_null() {
                    let mut j: i32 = 0;
                    let mut found: i32 = 0 as i32;
                    let mut best: i32 = -(1 as i32);
                    let mut lowattr: i32 = -(1 as i32);
                    j = 0 as i32;
                    while j < (*(*ctxt).states).nbState {
                        cur = (**((*(*ctxt).states).tabState).offset(j as isize)).seq;
                        cur = xmlRelaxNGSkipIgnored(ctxt, cur);
                        if cur.is_null() {
                            if found == 0 as i32 {
                                lowattr = (**((*(*ctxt).states).tabState)
                                    .offset(j as isize))
                                    .nbAttrLeft;
                                best = j;
                            }
                            found = 1 as i32;
                            if (**((*(*ctxt).states).tabState).offset(j as isize))
                                .nbAttrLeft <= lowattr
                            {
                                lowattr = (**((*(*ctxt).states).tabState)
                                    .offset(j as isize))
                                    .nbAttrLeft;
                                best = j;
                            }
                            if lowattr == 0 as i32 {
                                break;
                            }
                        } else if found == 0 as i32 {
                            if lowattr == -(1 as i32) {
                                lowattr = (**((*(*ctxt).states).tabState)
                                    .offset(j as isize))
                                    .nbAttrLeft;
                                best = j;
                            } else if (**((*(*ctxt).states).tabState).offset(j as isize))
                                    .nbAttrLeft <= lowattr
                                {
                                lowattr = (**((*(*ctxt).states).tabState)
                                    .offset(j as isize))
                                    .nbAttrLeft;
                                best = j;
                            }
                        }
                        j += 1;
                    }
                    if (*(*ctxt).states).nbState > 0 as i32 {
                        xmlRelaxNGFreeValidState(ctxt, oldstate);
                        if best != -(1 as i32) {
                            oldstate = *((*(*ctxt).states).tabState)
                                .offset(best as isize);
                            let fresh409 = &mut (*((*(*ctxt).states).tabState)
                                .offset(best as isize));
                            *fresh409 = 0 as xmlRelaxNGValidStatePtr;
                        } else {
                            oldstate = *((*(*ctxt).states).tabState)
                                .offset(
                                    ((*(*ctxt).states).nbState - 1 as i32) as isize,
                                );
                            let fresh410 = &mut (*((*(*ctxt).states).tabState)
                                .offset(
                                    ((*(*ctxt).states).nbState - 1 as i32) as isize,
                                ));
                            *fresh410 = 0 as xmlRelaxNGValidStatePtr;
                            let fresh411 = &mut ((*(*ctxt).states).nbState);
                            *fresh411 -= 1;
                        }
                    }
                    j = 0 as i32;
                    while j < (*(*ctxt).states).nbState {
                        xmlRelaxNGFreeValidState(
                            ctxt,
                            *((*(*ctxt).states).tabState).offset(j as isize),
                        );
                        j += 1;
                    }
                    xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                    let fresh412 = &mut ((*ctxt).states);
                    *fresh412 = 0 as xmlRelaxNGStatesPtr;
                    if found == 0 as i32 {
                        if cur.is_null() {
                            xmlRelaxNGAddValidError(
                                ctxt,
                                XML_RELAXNG_ERR_INTEREXTRA,
                                b"noname\0" as *const u8 as *const i8
                                    as *const xmlChar,
                                0 as *const xmlChar,
                                0 as i32,
                            );
                        } else {
                            xmlRelaxNGAddValidError(
                                ctxt,
                                XML_RELAXNG_ERR_INTEREXTRA,
                                (*cur).name,
                                0 as *const xmlChar,
                                0 as i32,
                            );
                        }
                        ret = -(1 as i32);
                        let fresh413 = &mut ((*ctxt).state);
                        *fresh413 = oldstate;
                        current_block = 790223741556965965;
                        break;
                    }
                } else {
                    ret = -(1 as i32);
                    current_block = 5482373152242628851;
                    break;
                }
                if !(*lasts.offset(i as isize)).is_null() {
                    let fresh414 = &mut ((**lasts.offset(i as isize)).next);
                    *fresh414 = last;
                }
                i += 1;
            }
        }
        match current_block {
            790223741556965965 => {}
            _ => {
                if !((*ctxt).state).is_null() {
                    xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
                }
                let fresh415 = &mut ((*ctxt).state);
                *fresh415 = oldstate;
                let fresh416 = &mut ((*(*ctxt).state).seq);
                *fresh416 = lastelem;
                if ret != 0 as i32 {
                    xmlRelaxNGAddValidError(
                        ctxt,
                        XML_RELAXNG_ERR_INTERSEQ,
                        0 as *const xmlChar,
                        0 as *const xmlChar,
                        0 as i32,
                    );
                    ret = -(1 as i32);
                }
            }
        }
    }
    (*ctxt).flags = oldflags;
    cur = lastchg;
    while !cur.is_null() {
        if cur == start || ((*cur).prev).is_null() {
            break;
        }
        let fresh417 = &mut ((*(*cur).prev).next);
        *fresh417 = cur;
        cur = (*cur).prev;
    }
    if ret == 0 as i32 {
        if (*ctxt).errNr > errNr {
            xmlRelaxNGPopErrors(ctxt, errNr);
        }
    }
    xmlFree.expect("non-null function pointer")(list as *mut libc::c_void);
    xmlFree.expect("non-null function pointer")(lasts as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn xmlRelaxNGValidateDefinitionList(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut defines: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut res: i32 = 0;
    if defines.is_null() {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_INTERNAL,
            b"NULL definition list\0" as *const u8 as *const i8
                as *mut xmlChar,
            0 as *const xmlChar,
            0 as i32,
        );
        return -(1 as i32);
    }
    while !defines.is_null() {
        if !((*ctxt).state).is_null() || !((*ctxt).states).is_null() {
            res = xmlRelaxNGValidateDefinition(ctxt, defines);
            if res < 0 as i32 {
                ret = -(1 as i32);
            }
        } else {
            xmlRelaxNGAddValidError(
                ctxt,
                XML_RELAXNG_ERR_NOSTATE,
                0 as *const xmlChar,
                0 as *const xmlChar,
                0 as i32,
            );
            return -(1 as i32);
        }
        if res == -(1 as i32) {
            break;
        }
        defines = (*defines).next;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGElementMatch(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut define: xmlRelaxNGDefinePtr,
    mut elem: xmlNodePtr,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut oldflags: i32 = 0 as i32;
    if !((*define).name).is_null() {
        if xmlStrEqual((*elem).name, (*define).name) == 0 {
            xmlRelaxNGAddValidError(
                ctxt,
                XML_RELAXNG_ERR_ELEMNAME,
                (*define).name,
                (*elem).name,
                0 as i32,
            );
            return 0 as i32;
        }
    }
    if !((*define).ns).is_null()
        && *((*define).ns).offset(0 as i32 as isize) as i32
            != 0 as i32
    {
        if ((*elem).ns).is_null() {
            xmlRelaxNGAddValidError(
                ctxt,
                XML_RELAXNG_ERR_ELEMNONS,
                (*elem).name,
                0 as *const xmlChar,
                0 as i32,
            );
            return 0 as i32;
        } else {
            if xmlStrEqual((*(*elem).ns).href, (*define).ns) == 0 {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_ELEMWRONGNS,
                    (*elem).name,
                    (*define).ns,
                    0 as i32,
                );
                return 0 as i32;
            }
        }
    } else if !((*elem).ns).is_null() && !((*define).ns).is_null()
            && ((*define).name).is_null()
        {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_ELEMEXTRANS,
            (*elem).name,
            0 as *const xmlChar,
            0 as i32,
        );
        return 0 as i32;
    } else {
        if !((*elem).ns).is_null() && !((*define).name).is_null() {
            xmlRelaxNGAddValidError(
                ctxt,
                XML_RELAXNG_ERR_ELEMEXTRANS,
                (*define).name,
                0 as *const xmlChar,
                0 as i32,
            );
            return 0 as i32;
        }
    }
    if ((*define).nameClass).is_null() {
        return 1 as i32;
    }
    define = (*define).nameClass;
    if (*define).type_0 as i32 == XML_RELAXNG_EXCEPT as i32 {
        let mut list: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        if !ctxt.is_null() {
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= 1 as i32;
        }
        list = (*define).content;
        while !list.is_null() {
            ret = xmlRelaxNGElementMatch(ctxt, list, elem);
            if ret == 1 as i32 {
                if !ctxt.is_null() {
                    (*ctxt).flags = oldflags;
                }
                return 0 as i32;
            }
            if ret < 0 as i32 {
                if !ctxt.is_null() {
                    (*ctxt).flags = oldflags;
                }
                return ret;
            }
            list = (*list).next;
        }
        ret = 1 as i32;
        if !ctxt.is_null() {
            (*ctxt).flags = oldflags;
        }
    } else if (*define).type_0 as i32 == XML_RELAXNG_CHOICE as i32 {
        let mut list_0: xmlRelaxNGDefinePtr = 0 as *mut xmlRelaxNGDefine;
        if !ctxt.is_null() {
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= 1 as i32;
        }
        list_0 = (*define).nameClass;
        while !list_0.is_null() {
            ret = xmlRelaxNGElementMatch(ctxt, list_0, elem);
            if ret == 1 as i32 {
                if !ctxt.is_null() {
                    (*ctxt).flags = oldflags;
                }
                return 1 as i32;
            }
            if ret < 0 as i32 {
                if !ctxt.is_null() {
                    (*ctxt).flags = oldflags;
                }
                return ret;
            }
            list_0 = (*list_0).next;
        }
        if !ctxt.is_null() {
            if ret != 0 as i32 {
                if (*ctxt).flags & 1 as i32 == 0 as i32 {
                    xmlRelaxNGDumpValidError(ctxt);
                }
            } else if (*ctxt).errNr > 0 as i32 {
                xmlRelaxNGPopErrors(ctxt, 0 as i32);
            }
        }
        ret = 0 as i32;
        if !ctxt.is_null() {
            (*ctxt).flags = oldflags;
        }
    } else {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
            b"relaxng.c\0" as *const u8 as *const i8,
            9764 as i32,
        );
        ret = -(1 as i32);
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGBestState(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
) -> i32 {
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut i: i32 = 0;
    let mut tmp: i32 = 0;
    let mut best: i32 = -(1 as i32);
    let mut value: i32 = 1000000 as i32;
    if ctxt.is_null() || ((*ctxt).states).is_null()
        || (*(*ctxt).states).nbState <= 0 as i32
    {
        return -(1 as i32);
    }
    i = 0 as i32;
    while i < (*(*ctxt).states).nbState {
        state = *((*(*ctxt).states).tabState).offset(i as isize);
        if !state.is_null() {
            if !((*state).seq).is_null() {
                if best == -(1 as i32) || value > 100000 as i32 {
                    value = 100000 as i32;
                    best = i;
                }
            } else {
                tmp = (*state).nbAttrLeft;
                if best == -(1 as i32) || value > tmp {
                    value = tmp;
                    best = i;
                }
            }
        }
        i += 1;
    }
    return best;
}
unsafe extern "C" fn xmlRelaxNGLogBestError(mut ctxt: xmlRelaxNGValidCtxtPtr) {
    let mut best: i32 = 0;
    if ctxt.is_null() || ((*ctxt).states).is_null()
        || (*(*ctxt).states).nbState <= 0 as i32
    {
        return;
    }
    best = xmlRelaxNGBestState(ctxt);
    if best >= 0 as i32 && best < (*(*ctxt).states).nbState {
        let fresh418 = &mut ((*ctxt).state);
        *fresh418 = *((*(*ctxt).states).tabState).offset(best as isize);
        xmlRelaxNGValidateElementEnd(ctxt, 1 as i32);
    }
}
unsafe extern "C" fn xmlRelaxNGValidateElementEnd(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut dolog: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    state = (*ctxt).state;
    if !((*state).seq).is_null() {
        let fresh419 = &mut ((*state).seq);
        *fresh419 = xmlRelaxNGSkipIgnored(ctxt, (*state).seq);
        if !((*state).seq).is_null() {
            if dolog != 0 {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_EXTRACONTENT,
                    (*(*state).node).name,
                    (*(*state).seq).name,
                    0 as i32,
                );
            }
            return -(1 as i32);
        }
    }
    i = 0 as i32;
    while i < (*state).nbAttrs {
        if !(*((*state).attrs).offset(i as isize)).is_null() {
            if dolog != 0 {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_INVALIDATTR,
                    (**((*state).attrs).offset(i as isize)).name,
                    (*(*state).node).name,
                    0 as i32,
                );
            }
            return -(1 as i32) - i;
        }
        i += 1;
    }
    return 0 as i32;
}
unsafe extern "C" fn xmlRelaxNGValidateState(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut define: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut tmp: i32 = 0;
    let mut oldflags: i32 = 0;
    let mut errNr: i32 = 0;
    let mut oldstate: xmlRelaxNGValidStatePtr = 0 as xmlRelaxNGValidStatePtr;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    if define.is_null() {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_NODEFINE,
            0 as *const xmlChar,
            0 as *const xmlChar,
            0 as i32,
        );
        return -(1 as i32);
    }
    if !((*ctxt).state).is_null() {
        node = (*(*ctxt).state).seq;
    } else {
        node = 0 as xmlNodePtr;
    }
    let fresh420 = &mut ((*ctxt).depth);
    *fresh420 += 1;
    let mut current_block_441: u64;
    match (*define).type_0 as i32 {
        0 => {
            ret = 0 as i32;
            current_block_441 = 4115481825097854405;
        }
        1 => {
            ret = -(1 as i32);
            current_block_441 = 4115481825097854405;
        }
        3 => {
            while !node.is_null()
                && ((*node).type_0 as u32
                    == XML_TEXT_NODE as i32 as u32
                    || (*node).type_0 as u32
                        == XML_COMMENT_NODE as i32 as u32
                    || (*node).type_0 as u32
                        == XML_PI_NODE as i32 as u32
                    || (*node).type_0 as u32
                        == XML_CDATA_SECTION_NODE as i32 as u32)
            {
                node = (*node).next;
            }
            let fresh421 = &mut ((*(*ctxt).state).seq);
            *fresh421 = node;
            current_block_441 = 4115481825097854405;
        }
        4 => {
            errNr = (*ctxt).errNr;
            node = xmlRelaxNGSkipIgnored(ctxt, node);
            if node.is_null() {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_NOELEM,
                    (*define).name,
                    0 as *const xmlChar,
                    0 as i32,
                );
                ret = -(1 as i32);
                if (*ctxt).flags & 1 as i32 == 0 as i32 {
                    xmlRelaxNGDumpValidError(ctxt);
                }
            } else if (*node).type_0 as u32
                    != XML_ELEMENT_NODE as i32 as u32
                {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_NOTELEM,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                    0 as i32,
                );
                ret = -(1 as i32);
                if (*ctxt).flags & 1 as i32 == 0 as i32 {
                    xmlRelaxNGDumpValidError(ctxt);
                }
            } else if (*node).psvi == define as *mut libc::c_void {
                let fresh422 = &mut ((*(*ctxt).state).seq);
                *fresh422 = xmlRelaxNGSkipIgnored(ctxt, (*node).next);
                if (*ctxt).errNr > errNr {
                    xmlRelaxNGPopErrors(ctxt, errNr);
                }
                if (*ctxt).errNr != 0 as i32 {
                    while !((*ctxt).err).is_null()
                        && ((*(*ctxt).err).err as u32
                            == XML_RELAXNG_ERR_ELEMNAME as i32 as u32
                            && xmlStrEqual((*(*ctxt).err).arg2, (*node).name) != 0
                            || (*(*ctxt).err).err as u32
                                == XML_RELAXNG_ERR_ELEMEXTRANS as i32
                                    as u32
                                && xmlStrEqual((*(*ctxt).err).arg1, (*node).name) != 0
                            || (*(*ctxt).err).err as u32
                                == XML_RELAXNG_ERR_NOELEM as i32 as u32
                            || (*(*ctxt).err).err as u32
                                == XML_RELAXNG_ERR_NOTELEM as i32 as u32)
                    {
                        xmlRelaxNGValidErrorPop(ctxt);
                    }
                }
            } else {
                ret = xmlRelaxNGElementMatch(ctxt, define, node);
                if ret <= 0 as i32 {
                    ret = -(1 as i32);
                    if (*ctxt).flags & 1 as i32 == 0 as i32 {
                        xmlRelaxNGDumpValidError(ctxt);
                    }
                } else {
                    ret = 0 as i32;
                    if (*ctxt).errNr != 0 as i32 {
                        if (*ctxt).errNr > errNr {
                            xmlRelaxNGPopErrors(ctxt, errNr);
                        }
                        while !((*ctxt).err).is_null()
                            && ((*(*ctxt).err).err as u32
                                == XML_RELAXNG_ERR_ELEMNAME as i32 as u32
                                && xmlStrEqual((*(*ctxt).err).arg2, (*node).name) != 0
                                || (*(*ctxt).err).err as u32
                                    == XML_RELAXNG_ERR_ELEMEXTRANS as i32
                                        as u32
                                    && xmlStrEqual((*(*ctxt).err).arg1, (*node).name) != 0
                                || (*(*ctxt).err).err as u32
                                    == XML_RELAXNG_ERR_NOELEM as i32 as u32
                                || (*(*ctxt).err).err as u32
                                    == XML_RELAXNG_ERR_NOTELEM as i32 as u32)
                        {
                            xmlRelaxNGValidErrorPop(ctxt);
                        }
                    }
                    errNr = (*ctxt).errNr;
                    oldflags = (*ctxt).flags;
                    if (*ctxt).flags & 4 as i32 != 0 {
                        (*ctxt).flags -= 4 as i32;
                    }
                    state = xmlRelaxNGNewValidState(ctxt, node);
                    if state.is_null() {
                        ret = -(1 as i32);
                        if (*ctxt).flags & 1 as i32 == 0 as i32 {
                            xmlRelaxNGDumpValidError(ctxt);
                        }
                    } else {
                        oldstate = (*ctxt).state;
                        let fresh423 = &mut ((*ctxt).state);
                        *fresh423 = state;
                        if !((*define).attrs).is_null() {
                            tmp = xmlRelaxNGValidateAttributeList(ctxt, (*define).attrs);
                            if tmp != 0 as i32 {
                                ret = -(1 as i32);
                                xmlRelaxNGAddValidError(
                                    ctxt,
                                    XML_RELAXNG_ERR_ATTRVALID,
                                    (*node).name,
                                    0 as *const xmlChar,
                                    0 as i32,
                                );
                            }
                        }
                        if !((*define).contModel).is_null() {
                            let mut nstate: xmlRelaxNGValidStatePtr = 0
                                as *mut xmlRelaxNGValidState;
                            let mut tmpstate: xmlRelaxNGValidStatePtr = (*ctxt).state;
                            let mut tmpstates: xmlRelaxNGStatesPtr = (*ctxt).states;
                            let mut nseq: xmlNodePtr = 0 as *mut xmlNode;
                            nstate = xmlRelaxNGNewValidState(ctxt, node);
                            let fresh424 = &mut ((*ctxt).state);
                            *fresh424 = nstate;
                            let fresh425 = &mut ((*ctxt).states);
                            *fresh425 = 0 as xmlRelaxNGStatesPtr;
                            tmp = xmlRelaxNGValidateCompiledContent(
                                ctxt,
                                (*define).contModel,
                                (*(*ctxt).state).seq,
                            );
                            nseq = (*(*ctxt).state).seq;
                            let fresh426 = &mut ((*ctxt).state);
                            *fresh426 = tmpstate;
                            let fresh427 = &mut ((*ctxt).states);
                            *fresh427 = tmpstates;
                            xmlRelaxNGFreeValidState(ctxt, nstate);
                            if tmp != 0 as i32 {
                                ret = -(1 as i32);
                            }
                            if !((*ctxt).states).is_null() {
                                tmp = -(1 as i32);
                                i = 0 as i32;
                                while i < (*(*ctxt).states).nbState {
                                    state = *((*(*ctxt).states).tabState).offset(i as isize);
                                    let fresh428 = &mut ((*ctxt).state);
                                    *fresh428 = state;
                                    let fresh429 = &mut ((*(*ctxt).state).seq);
                                    *fresh429 = nseq;
                                    if xmlRelaxNGValidateElementEnd(ctxt, 0 as i32)
                                        == 0 as i32
                                    {
                                        tmp = 0 as i32;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                                if tmp != 0 as i32 {
                                    (*ctxt).flags |= 1 as i32;
                                    xmlRelaxNGLogBestError(ctxt);
                                }
                                i = 0 as i32;
                                while i < (*(*ctxt).states).nbState {
                                    xmlRelaxNGFreeValidState(
                                        ctxt,
                                        *((*(*ctxt).states).tabState).offset(i as isize),
                                    );
                                    i += 1;
                                }
                                xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                                (*ctxt).flags = oldflags;
                                let fresh430 = &mut ((*ctxt).states);
                                *fresh430 = 0 as xmlRelaxNGStatesPtr;
                                if ret == 0 as i32 && tmp == -(1 as i32) {
                                    ret = -(1 as i32);
                                }
                            } else {
                                state = (*ctxt).state;
                                if !((*ctxt).state).is_null() {
                                    let fresh431 = &mut ((*(*ctxt).state).seq);
                                    *fresh431 = nseq;
                                }
                                if ret == 0 as i32 {
                                    ret = xmlRelaxNGValidateElementEnd(ctxt, 1 as i32);
                                }
                                xmlRelaxNGFreeValidState(ctxt, state);
                            }
                        } else {
                            if !((*define).content).is_null() {
                                tmp = xmlRelaxNGValidateDefinitionList(
                                    ctxt,
                                    (*define).content,
                                );
                                if tmp != 0 as i32 {
                                    ret = -(1 as i32);
                                    if ((*ctxt).state).is_null() {
                                        let fresh432 = &mut ((*ctxt).state);
                                        *fresh432 = oldstate;
                                        xmlRelaxNGAddValidError(
                                            ctxt,
                                            XML_RELAXNG_ERR_CONTENTVALID,
                                            (*node).name,
                                            0 as *const xmlChar,
                                            0 as i32,
                                        );
                                        let fresh433 = &mut ((*ctxt).state);
                                        *fresh433 = 0 as xmlRelaxNGValidStatePtr;
                                    } else {
                                        xmlRelaxNGAddValidError(
                                            ctxt,
                                            XML_RELAXNG_ERR_CONTENTVALID,
                                            (*node).name,
                                            0 as *const xmlChar,
                                            0 as i32,
                                        );
                                    }
                                }
                            }
                            if !((*ctxt).states).is_null() {
                                tmp = -(1 as i32);
                                i = 0 as i32;
                                while i < (*(*ctxt).states).nbState {
                                    state = *((*(*ctxt).states).tabState).offset(i as isize);
                                    let fresh434 = &mut ((*ctxt).state);
                                    *fresh434 = state;
                                    if xmlRelaxNGValidateElementEnd(ctxt, 0 as i32)
                                        == 0 as i32
                                    {
                                        tmp = 0 as i32;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                                if tmp != 0 as i32 {
                                    (*ctxt).flags |= 1 as i32;
                                    xmlRelaxNGLogBestError(ctxt);
                                }
                                i = 0 as i32;
                                while i < (*(*ctxt).states).nbState {
                                    xmlRelaxNGFreeValidState(
                                        ctxt,
                                        *((*(*ctxt).states).tabState).offset(i as isize),
                                    );
                                    let fresh435 = &mut (*((*(*ctxt).states).tabState)
                                        .offset(i as isize));
                                    *fresh435 = 0 as xmlRelaxNGValidStatePtr;
                                    i += 1;
                                }
                                xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                                (*ctxt).flags = oldflags;
                                let fresh436 = &mut ((*ctxt).states);
                                *fresh436 = 0 as xmlRelaxNGStatesPtr;
                                if ret == 0 as i32 && tmp == -(1 as i32) {
                                    ret = -(1 as i32);
                                }
                            } else {
                                state = (*ctxt).state;
                                if ret == 0 as i32 {
                                    ret = xmlRelaxNGValidateElementEnd(ctxt, 1 as i32);
                                }
                                xmlRelaxNGFreeValidState(ctxt, state);
                            }
                        }
                        if ret == 0 as i32 {
                            let fresh437 = &mut ((*node).psvi);
                            *fresh437 = define as *mut libc::c_void;
                        }
                        (*ctxt).flags = oldflags;
                        let fresh438 = &mut ((*ctxt).state);
                        *fresh438 = oldstate;
                        if !oldstate.is_null() {
                            let fresh439 = &mut ((*oldstate).seq);
                            *fresh439 = xmlRelaxNGSkipIgnored(ctxt, (*node).next);
                        }
                        if ret != 0 as i32 {
                            if (*ctxt).flags & 1 as i32 == 0 as i32 {
                                xmlRelaxNGDumpValidError(ctxt);
                                ret = 0 as i32;
                            }
                        } else if (*ctxt).errNr > errNr {
                            xmlRelaxNGPopErrors(ctxt, errNr);
                        }
                    }
                }
            }
            current_block_441 = 4115481825097854405;
        }
        14 => {
            errNr = (*ctxt).errNr;
            oldflags = (*ctxt).flags;
            (*ctxt).flags |= 1 as i32;
            oldstate = xmlRelaxNGCopyValidState(ctxt, (*ctxt).state);
            ret = xmlRelaxNGValidateDefinitionList(ctxt, (*define).content);
            if ret != 0 as i32 {
                if !((*ctxt).state).is_null() {
                    xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
                }
                let fresh440 = &mut ((*ctxt).state);
                *fresh440 = oldstate;
                (*ctxt).flags = oldflags;
                ret = 0 as i32;
                if (*ctxt).errNr > errNr {
                    xmlRelaxNGPopErrors(ctxt, errNr);
                }
            } else {
                if !((*ctxt).states).is_null() {
                    xmlRelaxNGAddStates(ctxt, (*ctxt).states, oldstate);
                    current_block_441 = 11620251134136044114;
                } else {
                    let fresh441 = &mut ((*ctxt).states);
                    *fresh441 = xmlRelaxNGNewStates(ctxt, 1 as i32);
                    if ((*ctxt).states).is_null() {
                        xmlRelaxNGFreeValidState(ctxt, oldstate);
                        (*ctxt).flags = oldflags;
                        ret = -(1 as i32);
                        if (*ctxt).errNr > errNr {
                            xmlRelaxNGPopErrors(ctxt, errNr);
                        }
                        current_block_441 = 4115481825097854405;
                    } else {
                        xmlRelaxNGAddStates(ctxt, (*ctxt).states, oldstate);
                        xmlRelaxNGAddStates(ctxt, (*ctxt).states, (*ctxt).state);
                        let fresh442 = &mut ((*ctxt).state);
                        *fresh442 = 0 as xmlRelaxNGValidStatePtr;
                        current_block_441 = 11620251134136044114;
                    }
                }
                match current_block_441 {
                    4115481825097854405 => {}
                    _ => {
                        (*ctxt).flags = oldflags;
                        ret = 0 as i32;
                        if (*ctxt).errNr > errNr {
                            xmlRelaxNGPopErrors(ctxt, errNr);
                        }
                    }
                }
            }
            current_block_441 = 4115481825097854405;
        }
        16 => {
            errNr = (*ctxt).errNr;
            ret = xmlRelaxNGValidateDefinitionList(ctxt, (*define).content);
            if ret != 0 as i32 {
                current_block_441 = 4115481825097854405;
            } else {
                if (*ctxt).errNr > errNr {
                    xmlRelaxNGPopErrors(ctxt, errNr);
                }
                current_block_441 = 13526015532137226550;
            }
        }
        15 => {
            current_block_441 = 13526015532137226550;
        }
        17 => {
            let mut list: xmlRelaxNGDefinePtr = 0 as xmlRelaxNGDefinePtr;
            let mut states_0: xmlRelaxNGStatesPtr = 0 as xmlRelaxNGStatesPtr;
            node = xmlRelaxNGSkipIgnored(ctxt, node);
            errNr = (*ctxt).errNr;
            if (*define).dflags as i32 & (1 as i32) << 4 as i32
                != 0 && !((*define).data).is_null() && !node.is_null()
            {
                let mut triage: xmlHashTablePtr = (*define).data as xmlHashTablePtr;
                if (*node).type_0 as u32
                    == XML_TEXT_NODE as i32 as u32
                    || (*node).type_0 as u32
                        == XML_CDATA_SECTION_NODE as i32 as u32
                {
                    list = xmlHashLookup2(
                        triage,
                        b"#text\0" as *const u8 as *const i8 as *mut xmlChar,
                        0 as *const xmlChar,
                    ) as xmlRelaxNGDefinePtr;
                } else if (*node).type_0 as u32
                        == XML_ELEMENT_NODE as i32 as u32
                    {
                    if !((*node).ns).is_null() {
                        list = xmlHashLookup2(triage, (*node).name, (*(*node).ns).href)
                            as xmlRelaxNGDefinePtr;
                        if list.is_null() {
                            list = xmlHashLookup2(
                                triage,
                                b"#any\0" as *const u8 as *const i8
                                    as *mut xmlChar,
                                (*(*node).ns).href,
                            ) as xmlRelaxNGDefinePtr;
                        }
                    } else {
                        list = xmlHashLookup2(triage, (*node).name, 0 as *const xmlChar)
                            as xmlRelaxNGDefinePtr;
                    }
                    if list.is_null() {
                        list = xmlHashLookup2(
                            triage,
                            b"#any\0" as *const u8 as *const i8
                                as *mut xmlChar,
                            0 as *const xmlChar,
                        ) as xmlRelaxNGDefinePtr;
                    }
                }
                if list.is_null() {
                    ret = -(1 as i32);
                    xmlRelaxNGAddValidError(
                        ctxt,
                        XML_RELAXNG_ERR_ELEMWRONG,
                        (*node).name,
                        0 as *const xmlChar,
                        0 as i32,
                    );
                } else {
                    ret = xmlRelaxNGValidateDefinition(ctxt, list);
                    ret == 0 as i32;
                }
            } else {
                list = (*define).content;
                oldflags = (*ctxt).flags;
                (*ctxt).flags |= 1 as i32;
                while !list.is_null() {
                    oldstate = xmlRelaxNGCopyValidState(ctxt, (*ctxt).state);
                    ret = xmlRelaxNGValidateDefinition(ctxt, list);
                    if ret == 0 as i32 {
                        if states_0.is_null() {
                            states_0 = xmlRelaxNGNewStates(ctxt, 1 as i32);
                        }
                        if !((*ctxt).state).is_null() {
                            xmlRelaxNGAddStates(ctxt, states_0, (*ctxt).state);
                        } else if !((*ctxt).states).is_null() {
                            i = 0 as i32;
                            while i < (*(*ctxt).states).nbState {
                                xmlRelaxNGAddStates(
                                    ctxt,
                                    states_0,
                                    *((*(*ctxt).states).tabState).offset(i as isize),
                                );
                                i += 1;
                            }
                            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                            let fresh454 = &mut ((*ctxt).states);
                            *fresh454 = 0 as xmlRelaxNGStatesPtr;
                        }
                    } else {
                        xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
                    }
                    let fresh455 = &mut ((*ctxt).state);
                    *fresh455 = oldstate;
                    list = (*list).next;
                }
                if !states_0.is_null() {
                    xmlRelaxNGFreeValidState(ctxt, oldstate);
                    let fresh456 = &mut ((*ctxt).states);
                    *fresh456 = states_0;
                    let fresh457 = &mut ((*ctxt).state);
                    *fresh457 = 0 as xmlRelaxNGValidStatePtr;
                    ret = 0 as i32;
                } else {
                    let fresh458 = &mut ((*ctxt).states);
                    *fresh458 = 0 as xmlRelaxNGStatesPtr;
                }
                (*ctxt).flags = oldflags;
                if ret != 0 as i32 {
                    if (*ctxt).flags & 1 as i32 == 0 as i32 {
                        xmlRelaxNGDumpValidError(ctxt);
                    }
                } else if (*ctxt).errNr > errNr {
                    xmlRelaxNGPopErrors(ctxt, errNr);
                }
            }
            current_block_441 = 4115481825097854405;
        }
        10 | 18 => {
            ret = xmlRelaxNGValidateDefinitionList(ctxt, (*define).content);
            current_block_441 = 4115481825097854405;
        }
        19 => {
            ret = xmlRelaxNGValidateInterleave(ctxt, define);
            current_block_441 = 4115481825097854405;
        }
        9 => {
            ret = xmlRelaxNGValidateAttribute(ctxt, define);
            current_block_441 = 4115481825097854405;
        }
        20 | -1 | 11 | 12 | 13 => {
            ret = xmlRelaxNGValidateDefinition(ctxt, (*define).content);
            current_block_441 = 4115481825097854405;
        }
        5 => {
            let mut child: xmlNodePtr = 0 as *mut xmlNode;
            let mut content: *mut xmlChar = 0 as *mut xmlChar;
            child = node;
            while !child.is_null() {
                if (*child).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                {
                    xmlRelaxNGAddValidError(
                        ctxt,
                        XML_RELAXNG_ERR_DATAELEM,
                        (*(*node).parent).name,
                        0 as *const xmlChar,
                        0 as i32,
                    );
                    ret = -(1 as i32);
                    break;
                } else {
                    if (*child).type_0 as u32
                        == XML_TEXT_NODE as i32 as u32
                        || (*child).type_0 as u32
                            == XML_CDATA_SECTION_NODE as i32 as u32
                    {
                        content = xmlStrcat(content, (*child).content);
                    }
                    child = (*child).next;
                }
            }
            if ret == -(1 as i32) {
                if !content.is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(content as *mut libc::c_void);
                }
            } else {
                if content.is_null() {
                    content = xmlStrdup(
                        b"\0" as *const u8 as *const i8 as *mut xmlChar,
                    );
                    if content.is_null() {
                        xmlRngVErrMemory(
                            ctxt,
                            b"validating\n\0" as *const u8 as *const i8,
                        );
                        ret = -(1 as i32);
                        current_block_441 = 4115481825097854405;
                    } else {
                        current_block_441 = 16903451103323879633;
                    }
                } else {
                    current_block_441 = 16903451103323879633;
                }
                match current_block_441 {
                    4115481825097854405 => {}
                    _ => {
                        ret = xmlRelaxNGValidateDatatype(
                            ctxt,
                            content,
                            define,
                            (*(*ctxt).state).seq,
                        );
                        if ret == -(1 as i32) {
                            xmlRelaxNGAddValidError(
                                ctxt,
                                XML_RELAXNG_ERR_DATATYPE,
                                (*define).name,
                                0 as *const xmlChar,
                                0 as i32,
                            );
                        } else if ret == 0 as i32 {
                            let fresh459 = &mut ((*(*ctxt).state).seq);
                            *fresh459 = 0 as xmlNodePtr;
                        }
                        if !content.is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(content as *mut libc::c_void);
                        }
                    }
                }
            }
            current_block_441 = 4115481825097854405;
        }
        7 => {
            let mut content_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut oldvalue: *mut xmlChar = 0 as *mut xmlChar;
            let mut child_0: xmlNodePtr = 0 as *mut xmlNode;
            child_0 = node;
            while !child_0.is_null() {
                if (*child_0).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                {
                    xmlRelaxNGAddValidError(
                        ctxt,
                        XML_RELAXNG_ERR_VALELEM,
                        (*(*node).parent).name,
                        0 as *const xmlChar,
                        0 as i32,
                    );
                    ret = -(1 as i32);
                    break;
                } else {
                    if (*child_0).type_0 as u32
                        == XML_TEXT_NODE as i32 as u32
                        || (*child_0).type_0 as u32
                            == XML_CDATA_SECTION_NODE as i32 as u32
                    {
                        content_0 = xmlStrcat(content_0, (*child_0).content);
                    }
                    child_0 = (*child_0).next;
                }
            }
            if ret == -(1 as i32) {
                if !content_0.is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(content_0 as *mut libc::c_void);
                }
            } else {
                if content_0.is_null() {
                    content_0 = xmlStrdup(
                        b"\0" as *const u8 as *const i8 as *mut xmlChar,
                    );
                    if content_0.is_null() {
                        xmlRngVErrMemory(
                            ctxt,
                            b"validating\n\0" as *const u8 as *const i8,
                        );
                        ret = -(1 as i32);
                        current_block_441 = 4115481825097854405;
                    } else {
                        current_block_441 = 7943534109044948052;
                    }
                } else {
                    current_block_441 = 7943534109044948052;
                }
                match current_block_441 {
                    4115481825097854405 => {}
                    _ => {
                        oldvalue = (*(*ctxt).state).value;
                        let fresh460 = &mut ((*(*ctxt).state).value);
                        *fresh460 = content_0;
                        ret = xmlRelaxNGValidateValue(ctxt, define);
                        let fresh461 = &mut ((*(*ctxt).state).value);
                        *fresh461 = oldvalue;
                        if ret == -(1 as i32) {
                            xmlRelaxNGAddValidError(
                                ctxt,
                                XML_RELAXNG_ERR_VALUE,
                                (*define).name,
                                0 as *const xmlChar,
                                0 as i32,
                            );
                        } else if ret == 0 as i32 {
                            let fresh462 = &mut ((*(*ctxt).state).seq);
                            *fresh462 = 0 as xmlNodePtr;
                        }
                        if !content_0.is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(content_0 as *mut libc::c_void);
                        }
                    }
                }
            }
            current_block_441 = 4115481825097854405;
        }
        8 => {
            let mut content_1: *mut xmlChar = 0 as *mut xmlChar;
            let mut child_1: xmlNodePtr = 0 as *mut xmlNode;
            let mut oldvalue_0: *mut xmlChar = 0 as *mut xmlChar;
            let mut oldendvalue: *mut xmlChar = 0 as *mut xmlChar;
            let mut len: i32 = 0;
            content_1 = 0 as *mut xmlChar;
            child_1 = node;
            while !child_1.is_null() {
                if (*child_1).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                {
                    xmlRelaxNGAddValidError(
                        ctxt,
                        XML_RELAXNG_ERR_LISTELEM,
                        (*(*node).parent).name,
                        0 as *const xmlChar,
                        0 as i32,
                    );
                    ret = -(1 as i32);
                    break;
                } else {
                    if (*child_1).type_0 as u32
                        == XML_TEXT_NODE as i32 as u32
                        || (*child_1).type_0 as u32
                            == XML_CDATA_SECTION_NODE as i32 as u32
                    {
                        content_1 = xmlStrcat(content_1, (*child_1).content);
                    }
                    child_1 = (*child_1).next;
                }
            }
            if ret == -(1 as i32) {
                if !content_1.is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(content_1 as *mut libc::c_void);
                }
            } else {
                if content_1.is_null() {
                    content_1 = xmlStrdup(
                        b"\0" as *const u8 as *const i8 as *mut xmlChar,
                    );
                    if content_1.is_null() {
                        xmlRngVErrMemory(
                            ctxt,
                            b"validating\n\0" as *const u8 as *const i8,
                        );
                        ret = -(1 as i32);
                        current_block_441 = 4115481825097854405;
                    } else {
                        current_block_441 = 16891331120537520283;
                    }
                } else {
                    current_block_441 = 16891331120537520283;
                }
                match current_block_441 {
                    4115481825097854405 => {}
                    _ => {
                        len = xmlStrlen(content_1);
                        oldvalue_0 = (*(*ctxt).state).value;
                        oldendvalue = (*(*ctxt).state).endvalue;
                        let fresh463 = &mut ((*(*ctxt).state).value);
                        *fresh463 = content_1;
                        let fresh464 = &mut ((*(*ctxt).state).endvalue);
                        *fresh464 = content_1.offset(len as isize);
                        ret = xmlRelaxNGValidateValue(ctxt, define);
                        let fresh465 = &mut ((*(*ctxt).state).value);
                        *fresh465 = oldvalue_0;
                        let fresh466 = &mut ((*(*ctxt).state).endvalue);
                        *fresh466 = oldendvalue;
                        if ret == -(1 as i32) {
                            xmlRelaxNGAddValidError(
                                ctxt,
                                XML_RELAXNG_ERR_LIST,
                                0 as *const xmlChar,
                                0 as *const xmlChar,
                                0 as i32,
                            );
                        } else if ret == 0 as i32 && !node.is_null() {
                            let fresh467 = &mut ((*(*ctxt).state).seq);
                            *fresh467 = (*node).next;
                        }
                        if !content_1.is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(content_1 as *mut libc::c_void);
                        }
                    }
                }
            }
            current_block_441 = 4115481825097854405;
        }
        2 | 6 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
                b"relaxng.c\0" as *const u8 as *const i8,
                10624 as i32,
            );
            ret = -(1 as i32);
            current_block_441 = 4115481825097854405;
        }
        _ => {
            current_block_441 = 4115481825097854405;
        }
    }
    match current_block_441 {
        13526015532137226550 => {
            let mut progress: i32 = 0;
            let mut states: xmlRelaxNGStatesPtr = 0 as xmlRelaxNGStatesPtr;
            let mut res: xmlRelaxNGStatesPtr = 0 as xmlRelaxNGStatesPtr;
            let mut base: i32 = 0;
            let mut j: i32 = 0;
            errNr = (*ctxt).errNr;
            res = xmlRelaxNGNewStates(ctxt, 1 as i32);
            if res.is_null() {
                ret = -(1 as i32);
            } else {
                if !((*ctxt).state).is_null() {
                    xmlRelaxNGAddStates(
                        ctxt,
                        res,
                        xmlRelaxNGCopyValidState(ctxt, (*ctxt).state),
                    );
                } else {
                    j = 0 as i32;
                    while j < (*(*ctxt).states).nbState {
                        xmlRelaxNGAddStates(
                            ctxt,
                            res,
                            xmlRelaxNGCopyValidState(
                                ctxt,
                                *((*(*ctxt).states).tabState).offset(j as isize),
                            ),
                        );
                        j += 1;
                    }
                }
                oldflags = (*ctxt).flags;
                (*ctxt).flags |= 1 as i32;
                loop {
                    progress = 0 as i32;
                    base = (*res).nbState;
                    if !((*ctxt).states).is_null() {
                        states = (*ctxt).states;
                        i = 0 as i32;
                        while i < (*states).nbState {
                            let fresh443 = &mut ((*ctxt).state);
                            *fresh443 = *((*states).tabState).offset(i as isize);
                            let fresh444 = &mut ((*ctxt).states);
                            *fresh444 = 0 as xmlRelaxNGStatesPtr;
                            ret = xmlRelaxNGValidateDefinitionList(
                                ctxt,
                                (*define).content,
                            );
                            if ret == 0 as i32 {
                                if !((*ctxt).state).is_null() {
                                    tmp = xmlRelaxNGAddStates(ctxt, res, (*ctxt).state);
                                    let fresh445 = &mut ((*ctxt).state);
                                    *fresh445 = 0 as xmlRelaxNGValidStatePtr;
                                    if tmp == 1 as i32 {
                                        progress = 1 as i32;
                                    }
                                } else if !((*ctxt).states).is_null() {
                                    j = 0 as i32;
                                    while j < (*(*ctxt).states).nbState {
                                        tmp = xmlRelaxNGAddStates(
                                            ctxt,
                                            res,
                                            *((*(*ctxt).states).tabState).offset(j as isize),
                                        );
                                        if tmp == 1 as i32 {
                                            progress = 1 as i32;
                                        }
                                        j += 1;
                                    }
                                    xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                                    let fresh446 = &mut ((*ctxt).states);
                                    *fresh446 = 0 as xmlRelaxNGStatesPtr;
                                }
                            } else if !((*ctxt).state).is_null() {
                                xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
                                let fresh447 = &mut ((*ctxt).state);
                                *fresh447 = 0 as xmlRelaxNGValidStatePtr;
                            }
                            i += 1;
                        }
                    } else {
                        ret = xmlRelaxNGValidateDefinitionList(ctxt, (*define).content);
                        if ret != 0 as i32 {
                            xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
                            let fresh448 = &mut ((*ctxt).state);
                            *fresh448 = 0 as xmlRelaxNGValidStatePtr;
                        } else {
                            base = (*res).nbState;
                            if !((*ctxt).state).is_null() {
                                tmp = xmlRelaxNGAddStates(ctxt, res, (*ctxt).state);
                                let fresh449 = &mut ((*ctxt).state);
                                *fresh449 = 0 as xmlRelaxNGValidStatePtr;
                                if tmp == 1 as i32 {
                                    progress = 1 as i32;
                                }
                            } else if !((*ctxt).states).is_null() {
                                j = 0 as i32;
                                while j < (*(*ctxt).states).nbState {
                                    tmp = xmlRelaxNGAddStates(
                                        ctxt,
                                        res,
                                        *((*(*ctxt).states).tabState).offset(j as isize),
                                    );
                                    if tmp == 1 as i32 {
                                        progress = 1 as i32;
                                    }
                                    j += 1;
                                }
                                if states.is_null() {
                                    states = (*ctxt).states;
                                } else {
                                    xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                                }
                                let fresh450 = &mut ((*ctxt).states);
                                *fresh450 = 0 as xmlRelaxNGStatesPtr;
                            }
                        }
                    }
                    if progress != 0 {
                        if (*res).nbState - base == 1 as i32 {
                            let fresh451 = &mut ((*ctxt).state);
                            *fresh451 = xmlRelaxNGCopyValidState(
                                ctxt,
                                *((*res).tabState).offset(base as isize),
                            );
                        } else {
                            if states.is_null() {
                                xmlRelaxNGNewStates(ctxt, (*res).nbState - base);
                                states = (*ctxt).states;
                                if states.is_null() {
                                    progress = 0 as i32;
                                    break;
                                }
                            }
                            (*states).nbState = 0 as i32;
                            i = base;
                            while i < (*res).nbState {
                                xmlRelaxNGAddStates(
                                    ctxt,
                                    states,
                                    xmlRelaxNGCopyValidState(
                                        ctxt,
                                        *((*res).tabState).offset(i as isize),
                                    ),
                                );
                                i += 1;
                            }
                            let fresh452 = &mut ((*ctxt).states);
                            *fresh452 = states;
                        }
                    }
                    if !(progress == 1 as i32) {
                        break;
                    }
                }
                if !states.is_null() {
                    xmlRelaxNGFreeStates(ctxt, states);
                }
                let fresh453 = &mut ((*ctxt).states);
                *fresh453 = res;
                (*ctxt).flags = oldflags;
                ret = 0 as i32;
            }
        }
        _ => {}
    }
    let fresh468 = &mut ((*ctxt).depth);
    *fresh468 -= 1;
    return ret;
}
unsafe extern "C" fn xmlRelaxNGValidateDefinition(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut define: xmlRelaxNGDefinePtr,
) -> i32 {
    let mut states: xmlRelaxNGStatesPtr = 0 as *mut xmlRelaxNGStates;
    let mut res: xmlRelaxNGStatesPtr = 0 as *mut xmlRelaxNGStates;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ret: i32 = 0;
    let mut oldflags: i32 = 0;
    if !((*ctxt).state).is_null() && !((*ctxt).states).is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
            b"relaxng.c\0" as *const u8 as *const i8,
            10663 as i32,
        );
        xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
        let fresh469 = &mut ((*ctxt).state);
        *fresh469 = 0 as xmlRelaxNGValidStatePtr;
    }
    if ((*ctxt).states).is_null() || (*(*ctxt).states).nbState == 1 as i32 {
        if !((*ctxt).states).is_null() {
            let fresh470 = &mut ((*ctxt).state);
            *fresh470 = *((*(*ctxt).states).tabState).offset(0 as i32 as isize);
            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
            let fresh471 = &mut ((*ctxt).states);
            *fresh471 = 0 as xmlRelaxNGStatesPtr;
        }
        ret = xmlRelaxNGValidateState(ctxt, define);
        if !((*ctxt).state).is_null() && !((*ctxt).states).is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
                b"relaxng.c\0" as *const u8 as *const i8,
                10675 as i32,
            );
            xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
            let fresh472 = &mut ((*ctxt).state);
            *fresh472 = 0 as xmlRelaxNGValidStatePtr;
        }
        if !((*ctxt).states).is_null() && (*(*ctxt).states).nbState == 1 as i32 {
            let fresh473 = &mut ((*ctxt).state);
            *fresh473 = *((*(*ctxt).states).tabState).offset(0 as i32 as isize);
            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
            let fresh474 = &mut ((*ctxt).states);
            *fresh474 = 0 as xmlRelaxNGStatesPtr;
        }
        return ret;
    }
    states = (*ctxt).states;
    let fresh475 = &mut ((*ctxt).states);
    *fresh475 = 0 as xmlRelaxNGStatesPtr;
    res = 0 as xmlRelaxNGStatesPtr;
    j = 0 as i32;
    oldflags = (*ctxt).flags;
    (*ctxt).flags |= 1 as i32;
    i = 0 as i32;
    while i < (*states).nbState {
        let fresh476 = &mut ((*ctxt).state);
        *fresh476 = *((*states).tabState).offset(i as isize);
        let fresh477 = &mut ((*ctxt).states);
        *fresh477 = 0 as xmlRelaxNGStatesPtr;
        ret = xmlRelaxNGValidateState(ctxt, define);
        if !((*ctxt).state).is_null() && !((*ctxt).states).is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
                b"relaxng.c\0" as *const u8 as *const i8,
                10700 as i32,
            );
            xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
            let fresh478 = &mut ((*ctxt).state);
            *fresh478 = 0 as xmlRelaxNGValidStatePtr;
        }
        if ret == 0 as i32 {
            if ((*ctxt).states).is_null() {
                if !res.is_null() {
                    xmlRelaxNGAddStates(ctxt, res, (*ctxt).state);
                    let fresh479 = &mut ((*ctxt).state);
                    *fresh479 = 0 as xmlRelaxNGValidStatePtr;
                } else {
                    let fresh480 = j;
                    j = j + 1;
                    let fresh481 = &mut (*((*states).tabState)
                        .offset(fresh480 as isize));
                    *fresh481 = (*ctxt).state;
                    let fresh482 = &mut ((*ctxt).state);
                    *fresh482 = 0 as xmlRelaxNGValidStatePtr;
                }
            } else if res.is_null() {
                res = (*ctxt).states;
                let fresh483 = &mut ((*ctxt).states);
                *fresh483 = 0 as xmlRelaxNGStatesPtr;
                k = 0 as i32;
                while k < j {
                    xmlRelaxNGAddStates(
                        ctxt,
                        res,
                        *((*states).tabState).offset(k as isize),
                    );
                    k += 1;
                }
            } else {
                k = 0 as i32;
                while k < (*(*ctxt).states).nbState {
                    xmlRelaxNGAddStates(
                        ctxt,
                        res,
                        *((*(*ctxt).states).tabState).offset(k as isize),
                    );
                    k += 1;
                }
                xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
                let fresh484 = &mut ((*ctxt).states);
                *fresh484 = 0 as xmlRelaxNGStatesPtr;
            }
        } else if !((*ctxt).state).is_null() {
            xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
            let fresh485 = &mut ((*ctxt).state);
            *fresh485 = 0 as xmlRelaxNGValidStatePtr;
        } else if !((*ctxt).states).is_null() {
            k = 0 as i32;
            while k < (*(*ctxt).states).nbState {
                xmlRelaxNGFreeValidState(
                    ctxt,
                    *((*(*ctxt).states).tabState).offset(k as isize),
                );
                k += 1;
            }
            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
            let fresh486 = &mut ((*ctxt).states);
            *fresh486 = 0 as xmlRelaxNGStatesPtr;
        }
        i += 1;
    }
    (*ctxt).flags = oldflags;
    if !res.is_null() {
        xmlRelaxNGFreeStates(ctxt, states);
        let fresh487 = &mut ((*ctxt).states);
        *fresh487 = res;
        ret = 0 as i32;
    } else if j > 1 as i32 {
        (*states).nbState = j;
        let fresh488 = &mut ((*ctxt).states);
        *fresh488 = states;
        ret = 0 as i32;
    } else if j == 1 as i32 {
        let fresh489 = &mut ((*ctxt).state);
        *fresh489 = *((*states).tabState).offset(0 as i32 as isize);
        xmlRelaxNGFreeStates(ctxt, states);
        ret = 0 as i32;
    } else {
        ret = -(1 as i32);
        xmlRelaxNGFreeStates(ctxt, states);
        if !((*ctxt).states).is_null() {
            xmlRelaxNGFreeStates(ctxt, (*ctxt).states);
            let fresh490 = &mut ((*ctxt).states);
            *fresh490 = 0 as xmlRelaxNGStatesPtr;
        }
    }
    if !((*ctxt).state).is_null() && !((*ctxt).states).is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
            b"relaxng.c\0" as *const u8 as *const i8,
            10766 as i32,
        );
        xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
        let fresh491 = &mut ((*ctxt).state);
        *fresh491 = 0 as xmlRelaxNGValidStatePtr;
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGValidateDocument(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut doc: xmlDocPtr,
) -> i32 {
    let mut ret: i32 = 0;
    let mut schema: xmlRelaxNGPtr = 0 as *mut xmlRelaxNG;
    let mut grammar: xmlRelaxNGGrammarPtr = 0 as *mut xmlRelaxNGGrammar;
    let mut state: xmlRelaxNGValidStatePtr = 0 as *mut xmlRelaxNGValidState;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if ctxt.is_null() || ((*ctxt).schema).is_null() || doc.is_null() {
        return -(1 as i32);
    }
    (*ctxt).errNo = XML_RELAXNG_OK as i32;
    schema = (*ctxt).schema;
    grammar = (*schema).topgrammar;
    if grammar.is_null() {
        xmlRelaxNGAddValidError(
            ctxt,
            XML_RELAXNG_ERR_NOGRAMMAR,
            0 as *const xmlChar,
            0 as *const xmlChar,
            0 as i32,
        );
        return -(1 as i32);
    }
    state = xmlRelaxNGNewValidState(ctxt, 0 as xmlNodePtr);
    let fresh492 = &mut ((*ctxt).state);
    *fresh492 = state;
    ret = xmlRelaxNGValidateDefinition(ctxt, (*grammar).start);
    if !((*ctxt).state).is_null() && !((*state).seq).is_null() {
        state = (*ctxt).state;
        node = (*state).seq;
        node = xmlRelaxNGSkipIgnored(ctxt, node);
        if !node.is_null() {
            if ret != -(1 as i32) {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_EXTRADATA,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                    0 as i32,
                );
                ret = -(1 as i32);
            }
        }
    } else if !((*ctxt).states).is_null() {
        let mut i: i32 = 0;
        let mut tmp: i32 = -(1 as i32);
        i = 0 as i32;
        while i < (*(*ctxt).states).nbState {
            state = *((*(*ctxt).states).tabState).offset(i as isize);
            node = (*state).seq;
            node = xmlRelaxNGSkipIgnored(ctxt, node);
            if node.is_null() {
                tmp = 0 as i32;
            }
            xmlRelaxNGFreeValidState(ctxt, state);
            i += 1;
        }
        if tmp == -(1 as i32) {
            if ret != -(1 as i32) {
                xmlRelaxNGAddValidError(
                    ctxt,
                    XML_RELAXNG_ERR_EXTRADATA,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                    0 as i32,
                );
                ret = -(1 as i32);
            }
        }
    }
    if !((*ctxt).state).is_null() {
        xmlRelaxNGFreeValidState(ctxt, (*ctxt).state);
        let fresh493 = &mut ((*ctxt).state);
        *fresh493 = 0 as xmlRelaxNGValidStatePtr;
    }
    if ret != 0 as i32 {
        xmlRelaxNGDumpValidError(ctxt);
    }
    if (*ctxt).idref == 1 as i32 {
        let mut vctxt: xmlValidCtxt = xmlValidCtxt {
            userData: 0 as *mut libc::c_void,
            error: None,
            warning: None,
            node: 0 as *mut xmlNode,
            nodeNr: 0,
            nodeMax: 0,
            nodeTab: 0 as *mut xmlNodePtr,
            flags: 0,
            doc: 0 as *mut xmlDoc,
            valid: 0,
            vstate: 0 as *mut xmlValidState,
            vstateNr: 0,
            vstateMax: 0,
            vstateTab: 0 as *mut xmlValidState,
            am: 0 as *mut xmlAutomata,
            state: 0 as *mut xmlAutomataState,
        };
        memset(
            &mut vctxt as *mut xmlValidCtxt as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<xmlValidCtxt>() as u64,
        );
        vctxt.valid = 1 as i32;
        vctxt.error = (*ctxt).error;
        vctxt.warning = (*ctxt).warning;
        vctxt.userData = (*ctxt).userData;
        if xmlValidateDocumentFinal(&mut vctxt, doc) != 1 as i32 {
            ret = -(1 as i32);
        }
    }
    if ret == 0 as i32 && (*ctxt).errNo != XML_RELAXNG_OK as i32 {
        ret = -(1 as i32);
    }
    return ret;
}
unsafe extern "C" fn xmlRelaxNGCleanPSVI(mut node: xmlNodePtr) {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if node.is_null()
        || (*node).type_0 as u32
            != XML_ELEMENT_NODE as i32 as u32
            && (*node).type_0 as u32
                != XML_DOCUMENT_NODE as i32 as u32
            && (*node).type_0 as u32
                != XML_HTML_DOCUMENT_NODE as i32 as u32
    {
        return;
    }
    if (*node).type_0 as u32 == XML_ELEMENT_NODE as i32 as u32
    {
        let fresh494 = &mut ((*node).psvi);
        *fresh494 = 0 as *mut libc::c_void;
    }
    cur = (*node).children;
    while !cur.is_null() {
        if (*cur).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        {
            let fresh495 = &mut ((*cur).psvi);
            *fresh495 = 0 as *mut libc::c_void;
            if !((*cur).children).is_null() {
                cur = (*cur).children;
                continue;
            }
        }
        if !((*cur).next).is_null() {
            cur = (*cur).next;
        } else {
            loop {
                cur = (*cur).parent;
                if cur.is_null() {
                    break;
                }
                if cur == node {
                    cur = 0 as xmlNodePtr;
                    break;
                } else if !((*cur).next).is_null() {
                    cur = (*cur).next;
                    break;
                } else if cur.is_null() {
                    break;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGNewValidCtxt(
    mut schema: xmlRelaxNGPtr,
) -> xmlRelaxNGValidCtxtPtr {
    let mut ret: xmlRelaxNGValidCtxtPtr = 0 as *mut xmlRelaxNGValidCtxt;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRelaxNGValidCtxt>() as u64)
        as xmlRelaxNGValidCtxtPtr;
    if ret.is_null() {
        xmlRngVErrMemory(
            0 as xmlRelaxNGValidCtxtPtr,
            b"building context\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlRelaxNGValidCtxtPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlRelaxNGValidCtxt>() as u64,
    );
    let fresh496 = &mut ((*ret).schema);
    *fresh496 = schema;
    let fresh497 = &mut ((*ret).error);
    *fresh497 = *__xmlGenericError();
    let fresh498 = &mut ((*ret).userData);
    *fresh498 = *__xmlGenericErrorContext();
    (*ret).errNr = 0 as i32;
    (*ret).errMax = 0 as i32;
    let fresh499 = &mut ((*ret).err);
    *fresh499 = 0 as xmlRelaxNGValidErrorPtr;
    let fresh500 = &mut ((*ret).errTab);
    *fresh500 = 0 as xmlRelaxNGValidErrorPtr;
    if !schema.is_null() {
        (*ret).idref = (*schema).idref;
    }
    let fresh501 = &mut ((*ret).states);
    *fresh501 = 0 as xmlRelaxNGStatesPtr;
    let fresh502 = &mut ((*ret).freeState);
    *fresh502 = 0 as xmlRelaxNGStatesPtr;
    let fresh503 = &mut ((*ret).freeStates);
    *fresh503 = 0 as *mut xmlRelaxNGStatesPtr;
    (*ret).errNo = XML_RELAXNG_OK as i32;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGFreeValidCtxt(mut ctxt: xmlRelaxNGValidCtxtPtr) {
    let mut k: i32 = 0;
    if ctxt.is_null() {
        return;
    }
    if !((*ctxt).states).is_null() {
        xmlRelaxNGFreeStates(0 as xmlRelaxNGValidCtxtPtr, (*ctxt).states);
    }
    if !((*ctxt).freeState).is_null() {
        k = 0 as i32;
        while k < (*(*ctxt).freeState).nbState {
            xmlRelaxNGFreeValidState(
                0 as xmlRelaxNGValidCtxtPtr,
                *((*(*ctxt).freeState).tabState).offset(k as isize),
            );
            k += 1;
        }
        xmlRelaxNGFreeStates(0 as xmlRelaxNGValidCtxtPtr, (*ctxt).freeState);
    }
    if !((*ctxt).freeStates).is_null() {
        k = 0 as i32;
        while k < (*ctxt).freeStatesNr {
            xmlRelaxNGFreeStates(
                0 as xmlRelaxNGValidCtxtPtr,
                *((*ctxt).freeStates).offset(k as isize),
            );
            k += 1;
        }
        xmlFree
            .expect(
                "non-null function pointer",
            )((*ctxt).freeStates as *mut libc::c_void);
    }
    if !((*ctxt).errTab).is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).errTab as *mut libc::c_void);
    }
    if !((*ctxt).elemTab).is_null() {
        let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
        exec = xmlRelaxNGElemPop(ctxt);
        while !exec.is_null() {
            xmlRegFreeExecCtxt(exec);
            exec = xmlRelaxNGElemPop(ctxt);
        }
        xmlFree
            .expect("non-null function pointer")((*ctxt).elemTab as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGSetValidErrors(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut err: xmlRelaxNGValidityErrorFunc,
    mut warn: xmlRelaxNGValidityWarningFunc,
    mut ctx: *mut libc::c_void,
) {
    if ctxt.is_null() {
        return;
    }
    let fresh504 = &mut ((*ctxt).error);
    *fresh504 = err;
    let fresh505 = &mut ((*ctxt).warning);
    *fresh505 = warn;
    let fresh506 = &mut ((*ctxt).userData);
    *fresh506 = ctx;
    let fresh507 = &mut ((*ctxt).serror);
    *fresh507 = None;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGSetValidStructuredErrors(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut serror: xmlStructuredErrorFunc,
    mut ctx: *mut libc::c_void,
) {
    if ctxt.is_null() {
        return;
    }
    let fresh508 = &mut ((*ctxt).serror);
    *fresh508 = serror;
    let fresh509 = &mut ((*ctxt).error);
    *fresh509 = None;
    let fresh510 = &mut ((*ctxt).warning);
    *fresh510 = None;
    let fresh511 = &mut ((*ctxt).userData);
    *fresh511 = ctx;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGGetValidErrors(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut err: *mut xmlRelaxNGValidityErrorFunc,
    mut warn: *mut xmlRelaxNGValidityWarningFunc,
    mut ctx: *mut *mut libc::c_void,
) -> i32 {
    if ctxt.is_null() {
        return -(1 as i32);
    }
    if !err.is_null() {
        *err = (*ctxt).error;
    }
    if !warn.is_null() {
        *warn = (*ctxt).warning;
    }
    if !ctx.is_null() {
        *ctx = (*ctxt).userData;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRelaxNGValidateDoc(
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut doc: xmlDocPtr,
) -> i32 {
    let mut ret: i32 = 0;
    if ctxt.is_null() || doc.is_null() {
        return -(1 as i32);
    }
    let fresh512 = &mut ((*ctxt).doc);
    *fresh512 = doc;
    ret = xmlRelaxNGValidateDocument(ctxt, doc);
    xmlRelaxNGCleanPSVI(doc as xmlNodePtr);
    if ret == -(1 as i32) {
        return 1 as i32;
    }
    return ret;
}
