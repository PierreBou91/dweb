//! Core module
//!
//! This module contains the core implementation of the dweb crate. It contains the types/enums etc
//! that are shared between the client and server implementations.
//!
//! It's an attempt at extracting all the Common Aspects of DICOM Web Services as defined in [this
//! section of the NEMA standard](https://dicom.nema.org/medical/dicom/current/output/chtml/part18/chapter_8.html)
//!
//! It will rely on the reqwest and std crates for the underlying HTTP implementation.
