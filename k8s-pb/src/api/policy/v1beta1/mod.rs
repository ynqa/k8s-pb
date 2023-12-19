/// Eviction evicts a pod from its node subject to certain policies and safety constraints.
/// This is a subresource of Pod.  A request to cause such an eviction is
/// created by POSTing to .../pods/<pod name>/evictions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Eviction {
    /// ObjectMeta describes the pod that is being evicted.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// DeleteOptions may be provided
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub delete_options: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::DeleteOptions,
    >,
}
/// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDisruptionBudget {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Specification of the desired behavior of the PodDisruptionBudget.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<PodDisruptionBudgetSpec>,
    /// Most recently observed status of the PodDisruptionBudget.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<PodDisruptionBudgetStatus>,
}
/// PodDisruptionBudgetList is a collection of PodDisruptionBudgets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDisruptionBudgetList {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// items list individual PodDisruptionBudget objects
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<PodDisruptionBudget>,
}
/// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDisruptionBudgetSpec {
    /// An eviction is allowed if at least "minAvailable" pods selected by
    /// "selector" will still be available after the eviction, i.e. even in the
    /// absence of the evicted pod.  So for example you can prevent all voluntary
    /// evictions by specifying "100%".
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub min_available: ::core::option::Option<
        super::super::super::apimachinery::pkg::util::intstr::IntOrString,
    >,
    /// Label query over pods whose evictions are managed by the disruption
    /// budget.
    /// A null selector selects no pods.
    /// An empty selector ({}) also selects no pods, which differs from standard behavior of selecting all pods.
    /// In policy/v1, an empty selector will select all pods in the namespace.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub selector: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector,
    >,
    /// An eviction is allowed if at most "maxUnavailable" pods selected by
    /// "selector" are unavailable after the eviction, i.e. even in absence of
    /// the evicted pod. For example, one can prevent all voluntary evictions
    /// by specifying 0. This is a mutually exclusive setting with "minAvailable".
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub max_unavailable: ::core::option::Option<
        super::super::super::apimachinery::pkg::util::intstr::IntOrString,
    >,
    /// UnhealthyPodEvictionPolicy defines the criteria for when unhealthy pods
    /// should be considered for eviction. Current implementation considers healthy pods,
    /// as pods that have status.conditions item with type="Ready",status="True".
    ///
    /// Valid policies are IfHealthyBudget and AlwaysAllow.
    /// If no policy is specified, the default behavior will be used,
    /// which corresponds to the IfHealthyBudget policy.
    ///
    /// IfHealthyBudget policy means that running pods (status.phase="Running"),
    /// but not yet healthy can be evicted only if the guarded application is not
    /// disrupted (status.currentHealthy is at least equal to status.desiredHealthy).
    /// Healthy pods will be subject to the PDB for eviction.
    ///
    /// AlwaysAllow policy means that all running pods (status.phase="Running"),
    /// but not yet healthy are considered disrupted and can be evicted regardless
    /// of whether the criteria in a PDB is met. This means perspective running
    /// pods of a disrupted application might not get a chance to become healthy.
    /// Healthy pods will be subject to the PDB for eviction.
    ///
    /// Additional policies may be added in the future.
    /// Clients making eviction decisions should disallow eviction of unhealthy pods
    /// if they encounter an unrecognized policy in this field.
    ///
    /// This field is beta-level. The eviction API uses this field when
    /// the feature gate PDBUnhealthyPodEvictionPolicy is enabled (enabled by default).
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub unhealthy_pod_eviction_policy: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
/// PodDisruptionBudgetStatus represents information about the status of a
/// PodDisruptionBudget. Status may trail the actual state of a system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDisruptionBudgetStatus {
    /// Most recent generation observed when updating this PDB status. DisruptionsAllowed and other
    /// status information is valid only if observedGeneration equals to PDB's object generation.
    /// +optional
    #[prost(int64, optional, tag = "1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// DisruptedPods contains information about pods whose eviction was
    /// processed by the API server eviction subresource handler but has not
    /// yet been observed by the PodDisruptionBudget controller.
    /// A pod will be in this map from the time when the API server processed the
    /// eviction request to the time when the pod is seen by PDB controller
    /// as having been marked for deletion (or after a timeout). The key in the map is the name of the pod
    /// and the value is the time when the API server processed the eviction request. If
    /// the deletion didn't occur and a pod is still there it will be removed from
    /// the list automatically by PodDisruptionBudget controller after some time.
    /// If everything goes smooth this map should be empty for the most of the time.
    /// Large number of entries in the map may indicate problems with pod deletions.
    /// +optional
    #[prost(map = "string, message", tag = "2")]
    pub disrupted_pods: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
    /// Number of pod disruptions that are currently allowed.
    #[prost(int32, optional, tag = "3")]
    pub disruptions_allowed: ::core::option::Option<i32>,
    /// current number of healthy pods
    #[prost(int32, optional, tag = "4")]
    pub current_healthy: ::core::option::Option<i32>,
    /// minimum desired number of healthy pods
    #[prost(int32, optional, tag = "5")]
    pub desired_healthy: ::core::option::Option<i32>,
    /// total number of pods counted by this disruption budget
    #[prost(int32, optional, tag = "6")]
    pub expected_pods: ::core::option::Option<i32>,
    /// Conditions contain conditions for PDB. The disruption controller sets the
    /// DisruptionAllowed condition. The following are known values for the reason field
    /// (additional reasons could be added in the future):
    /// - SyncFailed: The controller encountered an error and wasn't able to compute
    ///                the number of allowed disruptions. Therefore no disruptions are
    ///                allowed and the status of the condition will be False.
    /// - InsufficientPods: The number of pods are either at or below the number
    ///                      required by the PodDisruptionBudget. No disruptions are
    ///                      allowed and the status of the condition will be False.
    /// - SufficientPods: There are more pods than required by the PodDisruptionBudget.
    ///                    The condition will be True, and the number of allowed
    ///                    disruptions are provided by the disruptionsAllowed property.
    ///
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag = "7")]
    pub conditions: ::prost::alloc::vec::Vec<
        super::super::super::apimachinery::pkg::apis::meta::v1::Condition,
    >,
}
