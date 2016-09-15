# rust-psm
[![Build Status](https://travis-ci.org/hestela/rust-psm.svg?branch=master)](https://travis-ci.org/hestela/rust-psm)
[![Stories in Ready](https://badge.waffle.io/hestela/rust-psm.png?label=ready&title=Ready)](http://waffle.io/hestela/rust-psm)

A Rust implementation of Intel's Performance Scaled Messaging (PSM) library.

At a high level, PSM allows users to send and revcieve messages. PSM is intended for use by MPI.
When this library starts becoming more functional, there will not be an immediate use case available since at the moment, there are no MPI implementations written in rust. Test programs that test PSM setup and communication can be created but won't be useful besides for demonstration purposes. It may be possible with Open MPI to create a separate rust-psm mtl that calls into rust-psm from C or even Java.

Plans at the moment are to implement an API similar to the existing PSM library. Some C speicifc features may not be available such as debugging and some changes may have to be made due to the existing usage of pointers and data structures.
