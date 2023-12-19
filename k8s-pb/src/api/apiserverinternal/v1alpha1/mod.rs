/// An API server instance reports the version it can decode and the version it
/// encodes objects to when persisting objects in the backend.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStorageVersion {
    /// The ID of the reporting API server.
    #[prost(string, optional, tag = "1")]
    pub api_server_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The API server encodes the object to this version when persisting it in
    /// the backend (e.g., etcd).
    #[prost(string, optional, tag = "2")]
    pub encoding_version: ::core::option::Option<::prost::alloc::string::String>,
    /// The API server can decode objects encoded in these versions.
    /// The encodingVersion must be included in the decodableVersions.
    /// +listType=set
    #[prost(string, repeated, tag = "3")]
    pub decodable_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The API server can serve these versions.
    /// DecodableVersions must include all ServedVersions.
    /// +listType=set
    #[prost(string, repeated, tag = "4")]
    pub served_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Storage version of a specific resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageVersion {
    /// The name is <group>.<resource>.
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Spec is an empty spec. It is here to comply with Kubernetes API style.
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<StorageVersionSpec>,
    /// API server instances report the version they can decode and the version they
    /// encode objects to when persisting objects in the backend.
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<StorageVersionStatus>,
}
/// Describes the state of the storageVersion at a certain point.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageVersionCondition {
    /// Type of the condition.
    /// +required
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    /// +required
    #[prost(string, optional, tag = "2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// If set, this represents the .metadata.generation that the condition was set based upon.
    /// +optional
    #[prost(int64, optional, tag = "3")]
    pub observed_generation: ::core::option::Option<i64>,
    /// Last time the condition transitioned from one status to another.
    /// +required
    #[prost(message, optional, tag = "4")]
    pub last_transition_time: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
    /// The reason for the condition's last transition.
    /// +required
    #[prost(string, optional, tag = "5")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    /// +required
    #[prost(string, optional, tag = "6")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// A list of StorageVersions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageVersionList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// Items holds a list of StorageVersion
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<StorageVersion>,
}
/// StorageVersionSpec is an empty spec.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageVersionSpec {}
/// API server instances report the versions they can decode and the version they
/// encode objects to when persisting objects in the backend.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageVersionStatus {
    /// The reported versions per API server instance.
    /// +optional
    /// +listType=map
    /// +listMapKey=apiServerID
    #[prost(message, repeated, tag = "1")]
    pub storage_versions: ::prost::alloc::vec::Vec<ServerStorageVersion>,
    /// If all API server instances agree on the same encoding storage version,
    /// then this field is set to that version. Otherwise this field is left empty.
    /// API servers should finish updating its storageVersionStatus entry before
    /// serving write operations, so that this field will be in sync with the reality.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub common_encoding_version: ::core::option::Option<::prost::alloc::string::String>,
    /// The latest available observations of the storageVersion's state.
    /// +optional
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag = "3")]
    pub conditions: ::prost::alloc::vec::Vec<StorageVersionCondition>,
}

impl crate::Resource for StorageVersion {
    const API_VERSION: &'static str = "internal.apiserver.k8s.io/v1alpha1";
    const GROUP: &'static str = "internal.apiserver.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "StorageVersion";
    const URL_PATH_SEGMENT: &'static str = "storageversions";
    type Scope = crate::ClusterResourceScope;
}
impl crate::HasMetadata for StorageVersion {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for StorageVersion {
    type Spec = crate::api::apiserverinternal::v1alpha1::StorageVersionSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for StorageVersion {
    type Status = crate::api::apiserverinternal::v1alpha1::StorageVersionStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}
impl crate::HasConditions for StorageVersion {
    type Condition = crate::api::apiserverinternal::v1alpha1::StorageVersionCondition;
    fn conditions(&self) -> Option<&[<Self as crate::HasConditions>::Condition]> {
        self.status.as_ref().map(|s| s.conditions.as_slice())
    }
    fn conditions_mut(&mut self) -> Option<&mut Vec<<Self as crate::HasConditions>::Condition>> {
        self.status
            .as_mut()
            .and_then(|s| Some(s.conditions.as_mut()))
    }
}

