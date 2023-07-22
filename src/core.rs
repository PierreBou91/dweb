//! # Core module
//!
//! This module contains the core traits/structs/enums of the dweb crate.

/// ## DICOM Web Service Type
/// [standard link](https://dicom.nema.org/medical/dicom/current/output/chtml/part18/chapter_7.html)
///
/// The DICOM Web Service Type is defined in the DICOM standard in these terms:
/// > "This Part of the
/// > Standard defines DICOM Web Services. Each service allows a user agent to interact with an
/// > origin server to manage a set of DICOM Resources. Each DICOM Web Service operates on a set of
/// > resources and defines a set of Transactions that operate on those resources. All Transactions
/// > are defined in terms of HTTP request/response message pairs.
///
/// > [...]
///
/// > There are two general types of DICOM Web Services: URI and RESTful. This distinction is based
/// > on the type of web service protocol used to specify resources and transactions."
#[derive(Debug)]
pub enum DicomWebServiceType {
    /// ### URI Web Service
    ///
    /// > "The URI Web Service retrieves representations of its resources, those resources being
    /// > Composite SOP Instances (Instance). The URI service defines two transactions that retrieve
    /// > Instances in different media types. All URI transactions use the query component of the
    /// > URI in the request message to specify the transaction.
    ///
    /// > The functionality of the URI Web Service Transactions is similar to, but more limited
    /// > than, the Retrieve Transaction of the Studies Web Service."
    Uri,

    /// ### RESTful Web Service and Resources
    ///
    /// > "Each RESTful Web Service defines the set of resources, and the transactions that can be
    /// > applied to those resources."
    Restful(RestfulWebService),
}

/// ## RESTful Web Service
/// [standard link](https://dicom.nema.org/medical/dicom/current/output/chtml/part18/chapter_7.html)
///
/// > "Each RESTful Web Service defines the set of resources, and the transactions that can be
/// > applied to those resources."
#[derive(Debug)]
pub enum RestfulWebService {
    /// ### Studies Web Service
    ///
    /// > "Enables a user agent to manage Studies stored on an origin server."
    Studies,

    ///  ### Worklist Web Service
    ///
    /// > "Enables a user agent to manage the Worklist containing Workitems stored on an origin
    /// > server."
    Worklist,

    ///  ### Non-Patient Instance Web Service
    ///
    /// > "Enables a user agent to manage Non-Patient Instances, e.g., Color Palettes, stored on an
    /// > origin server."
    NonPatientInstance,
}

/// ## RESTful Resource
/// [standard link](https://dicom.nema.org/medical/dicom/current/output/chtml/part18/sect_7.2.html)
///
/// > "The DICOM Resources defined in this Part of the Standard are typically either a DICOM Web
/// > Services or DICOM Information Objects. Examples include Studies, Series, Instances, Worklists,
/// > and Workitems.
///
/// > DICOM Resources are grouped into collections and hierarchies. The following resources are
/// > examples of collections:
///
/// > | Resource Path | Contents |
/// > | --- | --- |
/// > | /studies | A collection of Studies. |
/// > | /series | A collection of Series. |
/// > | /instance | A collection of Instances. |
/// > | /frames | A sequence of Frames. |
/// > | /worklists | A collection of Worklists. |
///
/// > The following resources are examples of hierarchies:
///
/// > | Resource Path | Contents |
/// > | --- | --- |
/// > | /studies/{study}/series | Contains a collection of Series. |
/// > | /studies/{study}/series/{series}/instances | Contains a collection of Instances. |
/// > | /studies/{study}/series/{series}/instances/{instance}/frames | Contains a sequence of Frames. |
///
/// > A DICOM Web Service origin server manages a collection of resources. This might not be done
/// > directly; for example, an origin server could act as a proxy, converting RESTful requests into
/// > DIMSE requests, and DIMSE responses into RESTful responses.
///
/// > Resources are typically created and/or accessed by user agents."
#[derive(Debug)]
pub enum RestfulResource {
    /// "A collection of Studies."
    Studies,

    /// > "A collection of Series."
    Series,

    /// > "A collection of Instances."
    Instances,

    /// > "A sequence of Frames."
    Frames,

    /// > "A collection of Worklists."
    Worklists,

    /// > "A collection of Workitems."
    Workitems,
}

#[derive(Debug)]
pub enum MediaType {}
