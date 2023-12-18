/// Lease defines a lease concept.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lease {
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// spec contains the specification of the Lease.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<LeaseSpec>,
}
/// LeaseList is a list of Lease objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// items is a list of schema objects.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Lease>,
}
/// LeaseSpec is a specification of a Lease.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseSpec {
    /// holderIdentity contains the identity of the holder of a current lease.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub holder_identity: ::core::option::Option<::prost::alloc::string::String>,
    /// leaseDurationSeconds is a duration that candidates for a lease need
    /// to wait to force acquire it. This is measure against time of last
    /// observed renewTime.
    /// +optional
    #[prost(int32, optional, tag = "2")]
    pub lease_duration_seconds: ::core::option::Option<i32>,
    /// acquireTime is a time when the current lease was acquired.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub acquire_time: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime,
    >,
    /// renewTime is a time when the current holder of a lease has last
    /// updated the lease.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub renew_time: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime,
    >,
    /// leaseTransitions is the number of transitions of a lease between
    /// holders.
    /// +optional
    #[prost(int32, optional, tag = "5")]
    pub lease_transitions: ::core::option::Option<i32>,
}

impl crate::Resource for Lease {
    const API_VERSION: &'static str = "coordination.k8s.io/v1";
    const GROUP: &'static str = "coordination.k8s.io";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "Lease";
    const NAME: &'static str = "leases";
}
impl crate::HasMetadata for Lease {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for Lease {
    type Spec = crate::api::coordination::v1::LeaseSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}

