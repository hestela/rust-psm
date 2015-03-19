#[allow(non_camel_case_types)]
pub enum ErrorType {
    PSM_OK = 0,
    PSM_OK_NO_PROGRESS = 1,
    PSM_PARAM_ERR = 3,
    PSM_NO_MEMORY = 4,
    PSM_INIT_NOT_INIT = 5,
    PSM_INIT_BAD_API_VERSION = 6,
    PSM_NO_AFFINITY = 7,
    PSM_INTERNAL_ERR = 8,
    PSM_SHMEM_SEGMENT_ERR = 9,
    PSM_OPT_READONLY = 10,
    PSM_TIMEOUT = 11,
    PSM_TOO_MANY_ENDPOINTS = 12,
    PSM_IS_FINALIZED = 13,
    PSM_EP_WAS_CLOSED = 20,
    PSM_EP_NO_DEVICE = 21,
    PSM_EP_UNIT_NOT_FOUND = 22,
    PSM_EP_DEVICE_FAILURE = 23,
    PSM_EP_CLOSE_TIMEOUT = 24,
    PSM_EP_NO_PORTS_AVAIL = 25,
    PSM_EP_NO_NETWORK = 26,
    PSM_EP_INVALID_UUID_KEY = 27,
    PSM_EP_NO_RESOURCES = 28,
    PSM_EPID_UNKNOWN = 40,
    PSM_EPID_UNREACHABLE = 41,
    PSM_EPID_INVALID_NODE = 43,
    PSM_EPID_INVALID_MTU =  44,
    PSM_EPID_INVALID_UUID_KEY = 45,
    PSM_EPID_INVALID_VERSION = 46,
    PSM_EPID_INVALID_CONNECT = 47,
    PSM_EPID_ALREADY_CONNECTED = 48,
    PSM_EPID_NETWORK_ERROR = 49,
    PSM_EPID_INVALID_PKEY = 50,
    PSM_EPID_PATH_RESOLUTION = 51,
    PSM_MQ_NO_COMPLETIONS = 60,
    PSM_MQ_TRUNCATION = 61,
    PSM_AM_INVALID_REPLY = 70,
    PSM_ERROR_LAST = 80
}

pub struct Error {
  pub error: ErrorType,
  pub error_str: &'static str
  // TODO: add a fn for psm_error_token and remane
}

impl Error {
  pub fn new(error: ErrorType, error_str: &'static str)-> Error {
    Error { error: error, error_str: error_str }
  }

  pub fn error_type_to_string(self) -> &'static str {
    "NONE()"
  }
}
