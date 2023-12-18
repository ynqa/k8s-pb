/// PriorityClass defines mapping from a priority class name to the priority
/// integer value. The value can be any valid integer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriorityClass {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// value represents the integer value of this priority class. This is the actual priority that pods
    /// receive when they have the name of this class in their pod spec.
    #[prost(int32, optional, tag = "2")]
    pub value: ::core::option::Option<i32>,
    /// globalDefault specifies whether this PriorityClass should be considered as
    /// the default priority for pods that do not have any priority class.
    /// Only one PriorityClass can be marked as `globalDefault`. However, if more than
    /// one PriorityClasses exists with their `globalDefault` field set to true,
    /// the smallest value of such global default PriorityClasses will be used as the default priority.
    /// +optional
    #[prost(bool, optional, tag = "3")]
    pub global_default: ::core::option::Option<bool>,
    /// description is an arbitrary string that usually provides guidelines on
    /// when this priority class should be used.
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// preemptionPolicy is the Policy for preempting pods with lower priority.
    /// One of Never, PreemptLowerPriority.
    /// Defaults to PreemptLowerPriority if unset.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub preemption_policy: ::core::option::Option<::prost::alloc::string::String>,
}
/// PriorityClassList is a collection of priority classes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriorityClassList {
    /// Standard list metadata
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// items is the list of PriorityClasses
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<PriorityClass>,
}

impl crate::Resource for PriorityClass {
    const API_VERSION: &'static str = "scheduling.k8s.io/v1";
    const GROUP: &'static str = "scheduling.k8s.io";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "PriorityClass";
    const NAME: &'static str = "priorityclasses";
}
impl crate::HasMetadata for PriorityClass {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}

