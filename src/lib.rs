#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
//! # dweb
//!
//! The National Electrical Manufacturers Association (NEMA) has defined a standard for services
//! that provide access to DICOM data over the web. This crate is an abstraction over the reqwest
//! crate that provides a client and server implementation of the DICOM Web standard.
//!
//! The NEMA documentation for DICOM web services can be found [here](https://dicom.nema.org/medical/dicom/current/output/chtml/part18/PS3.18.html)

pub mod client;
pub mod core;
pub mod server;
