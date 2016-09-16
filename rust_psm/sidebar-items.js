initSidebarItems({"fn":[["finalize",""],["init","Can only be called once, using ONCE_INIT as a way arround global state. C version allows for init to be called multiple times, but after the first time, it seems like nothing useful is done. This should also prevent calling init after finalize."],["num_ipath_units",""]],"mod":[["ep","Datatypes and functions related to Ep (endpoints) aka psm_ep_t, (for now) EpAddr aka psm_epaddr_t, and Epid aka psm_epid_t."],["error","Contains the different error codes that were in psm.h in the C version."],["fileops","Contains operations related to the /dev/ipath character files."],["ipath","This module should be equivalent to the ipath directory in the C version. Mostly support functions and functions to interface with the ib_qib driver will live in this module."],["ptl_am","ptl_am - Packet Transport Layer for Active Messages and for shared memory messages."],["ptl_ips","ptl_ips - Packet Transport Layer for the Interconnect Protocol Stack."],["ptl_self","ptl_self - Packet Transport Layer for loop-back/send-to-self"]],"struct":[["Version",""]]});