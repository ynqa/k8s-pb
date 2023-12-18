/// DEPRECATED - This group version of DaemonSet is deprecated by apps/v1beta2/DaemonSet. See the release notes for
/// more information.
/// DaemonSet represents the configuration of a daemon set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSet {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// The desired behavior of this daemon set.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<DaemonSetSpec>,
    /// The current status of this daemon set. This data may be
    /// out of date by some window of time.
    /// Populated by the system.
    /// Read-only.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<DaemonSetStatus>,
}
/// DaemonSetCondition describes the state of a DaemonSet at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetCondition {
    /// Type of DaemonSet condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// DaemonSetList is a collection of daemon sets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// A list of daemon sets.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<DaemonSet>,
}
/// DaemonSetSpec is the specification of a daemon set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetSpec {
    /// A label query over pods that are managed by the daemon set.
    /// Must match in order to be controlled.
    /// If empty, defaulted to labels on Pod template.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    /// +optional
    #[prost(message, optional, tag="1")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// An object that describes the pod that will be created.
    /// The DaemonSet will create exactly one copy of this pod on every node
    /// that matches the template's node selector (or on every node if no node
    /// selector is specified).
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    #[prost(message, optional, tag="2")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
    /// An update strategy to replace existing DaemonSet pods with new pods.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub update_strategy: ::core::option::Option<DaemonSetUpdateStrategy>,
    /// The minimum number of seconds for which a newly created DaemonSet pod should
    /// be ready without any of its container crashing, for it to be considered
    /// available. Defaults to 0 (pod will be considered available as soon as it
    /// is ready).
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub min_ready_seconds: ::core::option::Option<i32>,
    /// DEPRECATED.
    /// A sequence number representing a specific generation of the template.
    /// Populated by the system. It can be set only during the creation.
    /// +optional
    #[prost(int64, optional, tag="5")]
    pub template_generation: ::core::option::Option<i64>,
    /// The number of old history to retain to allow rollback.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// Defaults to 10.
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub revision_history_limit: ::core::option::Option<i32>,
}
/// DaemonSetStatus represents the current status of a daemon set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetStatus {
    /// The number of nodes that are running at least 1
    /// daemon pod and are supposed to run the daemon pod.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[prost(int32, optional, tag="1")]
    pub current_number_scheduled: ::core::option::Option<i32>,
    /// The number of nodes that are running the daemon pod, but are
    /// not supposed to run the daemon pod.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[prost(int32, optional, tag="2")]
    pub number_misscheduled: ::core::option::Option<i32>,
    /// The total number of nodes that should be running the daemon
    /// pod (including nodes correctly running the daemon pod).
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/
    #[prost(int32, optional, tag="3")]
    pub desired_number_scheduled: ::core::option::Option<i32>,
    /// The number of nodes that should be running the daemon pod and have one
    /// or more of the daemon pod running and ready.
    #[prost(int32, optional, tag="4")]
    pub number_ready: ::core::option::Option<i32>,
    /// The most recent generation observed by the daemon set controller.
    /// +optional
    #[prost(int64, optional, tag="5")]
    pub observed_generation: ::core::option::Option<i64>,
    /// The total number of nodes that are running updated daemon pod
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub updated_number_scheduled: ::core::option::Option<i32>,
    /// The number of nodes that should be running the
    /// daemon pod and have one or more of the daemon pod running and
    /// available (ready for at least spec.minReadySeconds)
    /// +optional
    #[prost(int32, optional, tag="7")]
    pub number_available: ::core::option::Option<i32>,
    /// The number of nodes that should be running the
    /// daemon pod and have none of the daemon pod running and available
    /// (ready for at least spec.minReadySeconds)
    /// +optional
    #[prost(int32, optional, tag="8")]
    pub number_unavailable: ::core::option::Option<i32>,
    /// Count of hash collisions for the DaemonSet. The DaemonSet controller
    /// uses this field as a collision avoidance mechanism when it needs to
    /// create the name for the newest ControllerRevision.
    /// +optional
    #[prost(int32, optional, tag="9")]
    pub collision_count: ::core::option::Option<i32>,
    /// Represents the latest available observations of a DaemonSet's current state.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="10")]
    pub conditions: ::prost::alloc::vec::Vec<DaemonSetCondition>,
}
/// DaemonSetUpdateStrategy indicates the strategy that the DaemonSet
/// controller will use to perform updates. It includes any additional parameters
/// necessary to perform the update for the indicated strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonSetUpdateStrategy {
    /// Type of daemon set update. Can be "RollingUpdate" or "OnDelete".
    /// Default is OnDelete.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Rolling update config params. Present only if type = "RollingUpdate".
    /// ---
    /// TODO: Update this to follow our convention for oneOf, whatever we decide it
    /// to be. Same as Deployment `strategy.rollingUpdate`.
    /// See https://github.com/kubernetes/kubernetes/issues/35345
    /// +optional
    #[prost(message, optional, tag="2")]
    pub rolling_update: ::core::option::Option<RollingUpdateDaemonSet>,
}
/// DEPRECATED - This group version of Deployment is deprecated by apps/v1beta2/Deployment. See the release notes for
/// more information.
/// Deployment enables declarative updates for Pods and ReplicaSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Standard object metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the Deployment.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<DeploymentSpec>,
    /// Most recently observed status of the Deployment.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<DeploymentStatus>,
}
/// DeploymentCondition describes the state of a deployment at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentCondition {
    /// Type of deployment condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// The last time this condition was updated.
    #[prost(message, optional, tag="6")]
    pub last_update_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Last time the condition transitioned from one status to another.
    #[prost(message, optional, tag="7")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// DeploymentList is a list of Deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentList {
    /// Standard list metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of Deployments.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Deployment>,
}
/// DEPRECATED.
/// DeploymentRollback stores the information required to rollback a deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentRollback {
    /// Required: This must match the Name of a deployment.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The annotations to be updated to a deployment
    /// +optional
    #[prost(map="string, string", tag="2")]
    pub updated_annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The config of this deployment rollback.
    #[prost(message, optional, tag="3")]
    pub rollback_to: ::core::option::Option<RollbackConfig>,
}
/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentSpec {
    /// Number of desired pods. This is a pointer to distinguish between explicit
    /// zero and not specified. Defaults to 1.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// Label selector for pods. Existing ReplicaSets whose pods are
    /// selected by this will be the ones affected by this deployment.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// Template describes the pods that will be created.
    #[prost(message, optional, tag="3")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
    /// The deployment strategy to use to replace existing pods with new ones.
    /// +optional
    /// +patchStrategy=retainKeys
    #[prost(message, optional, tag="4")]
    pub strategy: ::core::option::Option<DeploymentStrategy>,
    /// Minimum number of seconds for which a newly created pod should be ready
    /// without any of its container crashing, for it to be considered available.
    /// Defaults to 0 (pod will be considered available as soon as it is ready)
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub min_ready_seconds: ::core::option::Option<i32>,
    /// The number of old ReplicaSets to retain to allow rollback.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// This is set to the max value of int32 (i.e. 2147483647) by default, which
    /// means "retaining all old ReplicaSets".
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub revision_history_limit: ::core::option::Option<i32>,
    /// Indicates that the deployment is paused and will not be processed by the
    /// deployment controller.
    /// +optional
    #[prost(bool, optional, tag="7")]
    pub paused: ::core::option::Option<bool>,
    /// DEPRECATED.
    /// The config this deployment is rolling back to. Will be cleared after rollback is done.
    /// +optional
    #[prost(message, optional, tag="8")]
    pub rollback_to: ::core::option::Option<RollbackConfig>,
    /// The maximum time in seconds for a deployment to make progress before it
    /// is considered to be failed. The deployment controller will continue to
    /// process failed deployments and a condition with a ProgressDeadlineExceeded
    /// reason will be surfaced in the deployment status. Note that progress will
    /// not be estimated during the time a deployment is paused. This is set to
    /// the max value of int32 (i.e. 2147483647) by default, which means "no deadline".
    /// +optional
    #[prost(int32, optional, tag="9")]
    pub progress_deadline_seconds: ::core::option::Option<i32>,
}
/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentStatus {
    /// The generation observed by the deployment controller.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub replicas: ::core::option::Option<i32>,
    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
    /// +optional
    #[prost(int32, optional, tag="3")]
    pub updated_replicas: ::core::option::Option<i32>,
    /// Total number of ready pods targeted by this deployment.
    /// +optional
    #[prost(int32, optional, tag="7")]
    pub ready_replicas: ::core::option::Option<i32>,
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub available_replicas: ::core::option::Option<i32>,
    /// Total number of unavailable pods targeted by this deployment. This is the total number of
    /// pods that are still required for the deployment to have 100% available capacity. They may
    /// either be pods that are running but not yet available or pods that still have not been created.
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub unavailable_replicas: ::core::option::Option<i32>,
    /// Represents the latest available observations of a deployment's current state.
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="6")]
    pub conditions: ::prost::alloc::vec::Vec<DeploymentCondition>,
    /// Count of hash collisions for the Deployment. The Deployment controller uses this
    /// field as a collision avoidance mechanism when it needs to create the name for the
    /// newest ReplicaSet.
    /// +optional
    #[prost(int32, optional, tag="8")]
    pub collision_count: ::core::option::Option<i32>,
}
/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentStrategy {
    /// Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Rolling update config params. Present only if DeploymentStrategyType =
    /// RollingUpdate.
    /// ---
    /// TODO: Update this to follow our convention for oneOf, whatever we decide it
    /// to be.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub rolling_update: ::core::option::Option<RollingUpdateDeployment>,
}
/// HTTPIngressPath associates a path with a backend. Incoming urls matching the
/// path are forwarded to the backend.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpIngressPath {
    /// Path is matched against the path of an incoming request. Currently it can
    /// contain characters disallowed from the conventional "path" part of a URL
    /// as defined by RFC 3986. Paths must begin with a '/'. When unspecified,
    /// all paths from incoming requests are matched.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// PathType determines the interpretation of the Path matching. PathType can
    /// be one of the following values:
    /// * Exact: Matches the URL path exactly.
    /// * Prefix: Matches based on a URL path prefix split by '/'. Matching is
    ///   done on a path element by element basis. A path element refers is the
    ///   list of labels in the path split by the '/' separator. A request is a
    ///   match for path p if every p is an element-wise prefix of p of the
    ///   request path. Note that if the last element of the path is a substring
    ///   of the last element in request path, it is not a match (e.g. /foo/bar
    ///   matches /foo/bar/baz, but does not match /foo/barbaz).
    /// * ImplementationSpecific: Interpretation of the Path matching is up to
    ///   the IngressClass. Implementations can treat this as a separate PathType
    ///   or treat it identically to Prefix or Exact path types.
    /// Implementations are required to support all path types.
    /// Defaults to ImplementationSpecific.
    #[prost(string, optional, tag="3")]
    pub path_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Backend defines the referenced service endpoint to which the traffic
    /// will be forwarded to.
    #[prost(message, optional, tag="2")]
    pub backend: ::core::option::Option<IngressBackend>,
}
/// HTTPIngressRuleValue is a list of http selectors pointing to backends.
/// In the example: http://<host>/<path>?<searchpart> -> backend where
/// where parts of the url correspond to RFC 3986, this resource will be used
/// to match against everything after the last '/' and before the first '?'
/// or '#'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpIngressRuleValue {
    /// A collection of paths that map requests to backends.
    #[prost(message, repeated, tag="1")]
    pub paths: ::prost::alloc::vec::Vec<HttpIngressPath>,
}
/// DEPRECATED 1.9 - This group version of IPBlock is deprecated by networking/v1/IPBlock.
/// IPBlock describes a particular CIDR (Ex. "192.168.1.0/24","2001:db8::/64") that is allowed
/// to the pods matched by a NetworkPolicySpec's podSelector. The except entry describes CIDRs
/// that should not be included within this rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpBlock {
    /// CIDR is a string representing the IP Block
    /// Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    #[prost(string, optional, tag="1")]
    pub cidr: ::core::option::Option<::prost::alloc::string::String>,
    /// Except is a slice of CIDRs that should not be included within an IP Block
    /// Valid examples are "192.168.1.0/24" or "2001:db8::/64"
    /// Except values will be rejected if they are outside the CIDR range
    /// +optional
    #[prost(string, repeated, tag="2")]
    pub except: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Ingress is a collection of rules that allow inbound connections to reach the
/// endpoints defined by a backend. An Ingress can be configured to give services
/// externally-reachable urls, load balance traffic, terminate SSL, offer name
/// based virtual hosting etc.
/// DEPRECATED - This group version of Ingress is deprecated by networking.k8s.io/v1beta1 Ingress. See the release notes for more information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ingress {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec is the desired state of the Ingress.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<IngressSpec>,
    /// Status is the current state of the Ingress.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<IngressStatus>,
}
/// IngressBackend describes all endpoints for a given service and port.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressBackend {
    /// Specifies the name of the referenced service.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub service_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies the port of the referenced service.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub service_port: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// Resource is an ObjectRef to another Kubernetes resource in the namespace
    /// of the Ingress object. If resource is specified, serviceName and servicePort
    /// must not be specified.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<super::super::core::v1::TypedLocalObjectReference>,
}
/// IngressList is a collection of Ingress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressList {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of Ingress.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Ingress>,
}
/// IngressLoadBalancerIngress represents the status of a load-balancer ingress point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressLoadBalancerIngress {
    /// IP is set for load-balancer ingress points that are IP based.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    /// Hostname is set for load-balancer ingress points that are DNS based.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
    /// Ports provides information about the ports exposed by this LoadBalancer.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag="4")]
    pub ports: ::prost::alloc::vec::Vec<IngressPortStatus>,
}
/// LoadBalancerStatus represents the status of a load-balancer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressLoadBalancerStatus {
    /// Ingress is a list containing ingress points for the load-balancer.
    /// +optional
    #[prost(message, repeated, tag="1")]
    pub ingress: ::prost::alloc::vec::Vec<IngressLoadBalancerIngress>,
}
/// IngressPortStatus represents the error condition of a service port
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressPortStatus {
    /// Port is the port number of the ingress port.
    #[prost(int32, optional, tag="1")]
    pub port: ::core::option::Option<i32>,
    /// Protocol is the protocol of the ingress port.
    /// The supported values are: "TCP", "UDP", "SCTP"
    #[prost(string, optional, tag="2")]
    pub protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// Error is to record the problem with the service port
    /// The format of the error shall comply with the following rules:
    /// - built-in error values shall be specified in this file and those shall use
    ///   CamelCase names
    /// - cloud provider specific error values must have names that comply with the
    ///   format foo.example.com/CamelCase.
    /// ---
    /// The regex it matches is (dns1123SubdomainFmt/)?(qualifiedNameFmt)
    /// +optional
    /// +kubebuilder:validation:Required
    /// +kubebuilder:validation:Pattern=`^([a-z0-9]([-a-z0-9]*[a-z0-9])?(\.[a-z0-9]([-a-z0-9]*[a-z0-9])?)*/)?(([A-Za-z0-9][-A-Za-z0-9_.]*)?[A-Za-z0-9])$`
    /// +kubebuilder:validation:MaxLength=316
    #[prost(string, optional, tag="3")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// IngressRule represents the rules mapping the paths under a specified host to
/// the related backend services. Incoming requests are first evaluated for a host
/// match, then routed to the backend associated with the matching IngressRuleValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressRule {
    /// Host is the fully qualified domain name of a network host, as defined by RFC 3986.
    /// Note the following deviations from the "host" part of the
    /// URI as defined in RFC 3986:
    /// 1. IPs are not allowed. Currently an IngressRuleValue can only apply to
    ///    the IP in the Spec of the parent Ingress.
    /// 2. The `:` delimiter is not respected because ports are not allowed.
    /// 	  Currently the port of an Ingress is implicitly :80 for http and
    /// 	  :443 for https.
    /// Both these may change in the future.
    /// Incoming requests are matched against the host before the
    /// IngressRuleValue. If the host is unspecified, the Ingress routes all
    /// traffic based on the specified IngressRuleValue.
    ///
    /// Host can be "precise" which is a domain name without the terminating dot of
    /// a network host (e.g. "foo.bar.com") or "wildcard", which is a domain name
    /// prefixed with a single wildcard label (e.g. "*.foo.com").
    /// The wildcard character '*' must appear by itself as the first DNS label and
    /// matches only a single label. You cannot have a wildcard label by itself (e.g. Host == "*").
    /// Requests will be matched against the Host field in the following way:
    /// 1. If Host is precise, the request matches this rule if the http host header is equal to Host.
    /// 2. If Host is a wildcard, then the request matches this rule if the http host header
    /// is to equal to the suffix (removing the first label) of the wildcard rule.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    /// IngressRuleValue represents a rule to route requests for this IngressRule.
    /// If unspecified, the rule defaults to a http catch-all. Whether that sends
    /// just traffic matching the host to the default backend or all traffic to the
    /// default backend, is left to the controller fulfilling the Ingress. Http is
    /// currently the only supported IngressRuleValue.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub ingress_rule_value: ::core::option::Option<IngressRuleValue>,
}
/// IngressRuleValue represents a rule to apply against incoming requests. If the
/// rule is satisfied, the request is routed to the specified backend. Currently
/// mixing different types of rules in a single Ingress is disallowed, so exactly
/// one of the following must be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressRuleValue {
    /// http is a list of http selectors pointing to backends.
    /// A path is matched against the path of an incoming request. Currently it can
    /// contain characters disallowed from the conventional "path" part of a URL
    /// as defined by RFC 3986. Paths must begin with a '/'.
    /// A backend defines the referenced service endpoint to which the traffic
    /// will be forwarded to.
    #[prost(message, optional, tag="1")]
    pub http: ::core::option::Option<HttpIngressRuleValue>,
}
/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressSpec {
    /// IngressClassName is the name of the IngressClass cluster resource. The
    /// associated IngressClass defines which controller will implement the
    /// resource. This replaces the deprecated `kubernetes.io/ingress.class`
    /// annotation. For backwards compatibility, when that annotation is set, it
    /// must be given precedence over this field. The controller may emit a
    /// warning if the field and annotation have different values.
    /// Implementations of this API should ignore Ingresses without a class
    /// specified. An IngressClass resource may be marked as default, which can
    /// be used to set a default value for this field. For more information,
    /// refer to the IngressClass documentation.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub ingress_class_name: ::core::option::Option<::prost::alloc::string::String>,
    /// A default backend capable of servicing requests that don't match any
    /// rule. At least one of 'backend' or 'rules' must be specified. This field
    /// is optional to allow the loadbalancer controller or defaulting logic to
    /// specify a global default.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub backend: ::core::option::Option<IngressBackend>,
    /// TLS configuration. Currently the Ingress only supports a single TLS
    /// port, 443. If multiple members of this list specify different hosts, they
    /// will be multiplexed on the same port according to the hostname specified
    /// through the SNI TLS extension, if the ingress controller fulfilling the
    /// ingress supports SNI.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub tls: ::prost::alloc::vec::Vec<IngressTls>,
    /// A list of host rules used to configure the Ingress. If unspecified, or
    /// no rule matches, all traffic is sent to the default backend.
    /// +optional
    #[prost(message, repeated, tag="3")]
    pub rules: ::prost::alloc::vec::Vec<IngressRule>,
}
/// IngressStatus describe the current state of the Ingress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressStatus {
    /// LoadBalancer contains the current status of the load-balancer.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub load_balancer: ::core::option::Option<IngressLoadBalancerStatus>,
}
/// IngressTLS describes the transport layer security associated with an Ingress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressTls {
    /// Hosts are a list of hosts included in the TLS certificate. The values in
    /// this list must match the name/s used in the tlsSecret. Defaults to the
    /// wildcard host setting for the loadbalancer controller fulfilling this
    /// Ingress, if left unspecified.
    /// +optional
    #[prost(string, repeated, tag="1")]
    pub hosts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// SecretName is the name of the secret used to terminate SSL traffic on 443.
    /// Field is left optional to allow SSL routing based on SNI hostname alone.
    /// If the SNI host in a listener conflicts with the "Host" header field used
    /// by an IngressRule, the SNI host is used for termination and value of the
    /// Host header is used for routing.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub secret_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// DEPRECATED 1.9 - This group version of NetworkPolicy is deprecated by networking/v1/NetworkPolicy.
/// NetworkPolicy describes what network traffic is allowed for a set of Pods
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicy {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior for this NetworkPolicy.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<NetworkPolicySpec>,
}
/// DEPRECATED 1.9 - This group version of NetworkPolicyEgressRule is deprecated by networking/v1/NetworkPolicyEgressRule.
/// NetworkPolicyEgressRule describes a particular set of traffic that is allowed out of pods
/// matched by a NetworkPolicySpec's podSelector. The traffic must match both ports and to.
/// This type is beta-level in 1.8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicyEgressRule {
    /// List of destination ports for outgoing traffic.
    /// Each item in this list is combined using a logical OR. If this field is
    /// empty or missing, this rule matches all ports (traffic not restricted by port).
    /// If this field is present and contains at least one item, then this rule allows
    /// traffic only if the traffic matches at least one port in the list.
    /// +optional
    #[prost(message, repeated, tag="1")]
    pub ports: ::prost::alloc::vec::Vec<NetworkPolicyPort>,
    /// List of destinations for outgoing traffic of pods selected for this rule.
    /// Items in this list are combined using a logical OR operation. If this field is
    /// empty or missing, this rule matches all destinations (traffic not restricted by
    /// destination). If this field is present and contains at least one item, this rule
    /// allows traffic only if the traffic matches at least one item in the to list.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub to: ::prost::alloc::vec::Vec<NetworkPolicyPeer>,
}
/// DEPRECATED 1.9 - This group version of NetworkPolicyIngressRule is deprecated by networking/v1/NetworkPolicyIngressRule.
/// This NetworkPolicyIngressRule matches traffic if and only if the traffic matches both ports AND from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicyIngressRule {
    /// List of ports which should be made accessible on the pods selected for this rule.
    /// Each item in this list is combined using a logical OR.
    /// If this field is empty or missing, this rule matches all ports (traffic not restricted by port).
    /// If this field is present and contains at least one item, then this rule allows traffic
    /// only if the traffic matches at least one port in the list.
    /// +optional
    #[prost(message, repeated, tag="1")]
    pub ports: ::prost::alloc::vec::Vec<NetworkPolicyPort>,
    /// List of sources which should be able to access the pods selected for this rule.
    /// Items in this list are combined using a logical OR operation.
    /// If this field is empty or missing, this rule matches all sources (traffic not restricted by source).
    /// If this field is present and contains at least one item, this rule allows traffic only if the
    /// traffic matches at least one item in the from list.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub from: ::prost::alloc::vec::Vec<NetworkPolicyPeer>,
}
/// DEPRECATED 1.9 - This group version of NetworkPolicyList is deprecated by networking/v1/NetworkPolicyList.
/// Network Policy List is a list of NetworkPolicy objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicyList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of schema objects.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<NetworkPolicy>,
}
/// DEPRECATED 1.9 - This group version of NetworkPolicyPeer is deprecated by networking/v1/NetworkPolicyPeer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicyPeer {
    /// This is a label selector which selects Pods. This field follows standard label
    /// selector semantics; if present but empty, it selects all pods.
    ///
    /// If NamespaceSelector is also set, then the NetworkPolicyPeer as a whole selects
    /// the Pods matching PodSelector in the Namespaces selected by NamespaceSelector.
    /// Otherwise it selects the Pods matching PodSelector in the policy's own Namespace.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub pod_selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// Selects Namespaces using cluster-scoped labels. This field follows standard label
    /// selector semantics; if present but empty, it selects all namespaces.
    ///
    /// If PodSelector is also set, then the NetworkPolicyPeer as a whole selects
    /// the Pods matching PodSelector in the Namespaces selected by NamespaceSelector.
    /// Otherwise it selects all Pods in the Namespaces selected by NamespaceSelector.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub namespace_selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// IPBlock defines policy on a particular IPBlock. If this field is set then
    /// neither of the other fields can be.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub ip_block: ::core::option::Option<IpBlock>,
}
/// DEPRECATED 1.9 - This group version of NetworkPolicyPort is deprecated by networking/v1/NetworkPolicyPort.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicyPort {
    /// Optional.  The protocol (TCP, UDP, or SCTP) which traffic must match.
    /// If not specified, this field defaults to TCP.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// The port on the given protocol. This can either be a numerical or named
    /// port on a pod. If this field is not provided, this matches all port names and
    /// numbers.
    /// If present, only traffic on the specified protocol AND port will be matched.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub port: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// If set, indicates that the range of ports from port to endPort, inclusive,
    /// should be allowed by the policy. This field cannot be defined if the port field
    /// is not defined or if the port field is defined as a named (string) port.
    /// The endPort must be equal or greater than port.
    /// +optional
    #[prost(int32, optional, tag="3")]
    pub end_port: ::core::option::Option<i32>,
}
/// DEPRECATED 1.9 - This group version of NetworkPolicySpec is deprecated by networking/v1/NetworkPolicySpec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicySpec {
    /// Selects the pods to which this NetworkPolicy object applies.  The array of ingress rules
    /// is applied to any pods selected by this field. Multiple network policies can select the
    /// same set of pods.  In this case, the ingress rules for each are combined additively.
    /// This field is NOT optional and follows standard label selector semantics.
    /// An empty podSelector matches all pods in this namespace.
    #[prost(message, optional, tag="1")]
    pub pod_selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// List of ingress rules to be applied to the selected pods.
    /// Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod
    /// OR if the traffic source is the pod's local node,
    /// OR if the traffic matches at least one ingress rule across all of the NetworkPolicy
    /// objects whose podSelector matches the pod.
    /// If this field is empty then this NetworkPolicy does not allow any traffic
    /// (and serves solely to ensure that the pods it selects are isolated by default).
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub ingress: ::prost::alloc::vec::Vec<NetworkPolicyIngressRule>,
    /// List of egress rules to be applied to the selected pods. Outgoing traffic is
    /// allowed if there are no NetworkPolicies selecting the pod (and cluster policy
    /// otherwise allows the traffic), OR if the traffic matches at least one egress rule
    /// across all of the NetworkPolicy objects whose podSelector matches the pod. If
    /// this field is empty then this NetworkPolicy limits all outgoing traffic (and serves
    /// solely to ensure that the pods it selects are isolated by default).
    /// This field is beta-level in 1.8
    /// +optional
    #[prost(message, repeated, tag="3")]
    pub egress: ::prost::alloc::vec::Vec<NetworkPolicyEgressRule>,
    /// List of rule types that the NetworkPolicy relates to.
    /// Valid options are ["Ingress"], ["Egress"], or ["Ingress", "Egress"].
    /// If this field is not specified, it will default based on the existence of Ingress or Egress rules;
    /// policies that contain an Egress section are assumed to affect Egress, and all policies
    /// (whether or not they contain an Ingress section) are assumed to affect Ingress.
    /// If you want to write an egress-only policy, you must explicitly specify policyTypes [ "Egress" ].
    /// Likewise, if you want to write a policy that specifies that no egress is allowed,
    /// you must specify a policyTypes value that include "Egress" (since such a policy would not include
    /// an Egress section and would otherwise default to just [ "Ingress" ]).
    /// This field is beta-level in 1.8
    /// +optional
    #[prost(string, repeated, tag="4")]
    pub policy_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DEPRECATED - This group version of ReplicaSet is deprecated by apps/v1beta2/ReplicaSet. See the release notes for
/// more information.
/// ReplicaSet ensures that a specified number of pod replicas are running at any given time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSet {
    /// If the Labels of a ReplicaSet are empty, they are defaulted to
    /// be the same as the Pod(s) that the ReplicaSet manages.
    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the specification of the desired behavior of the ReplicaSet.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ReplicaSetSpec>,
    /// Status is the most recently observed status of the ReplicaSet.
    /// This data may be out of date by some window of time.
    /// Populated by the system.
    /// Read-only.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<ReplicaSetStatus>,
}
/// ReplicaSetCondition describes the state of a replica set at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSetCondition {
    /// Type of replica set condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// The last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// ReplicaSetList is a collection of ReplicaSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSetList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of ReplicaSets.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ReplicaSet>,
}
/// ReplicaSetSpec is the specification of a ReplicaSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSetSpec {
    /// Replicas is the number of desired replicas.
    /// This is a pointer to distinguish between explicit zero and unspecified.
    /// Defaults to 1.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// Minimum number of seconds for which a newly created pod should be ready
    /// without any of its container crashing, for it to be considered available.
    /// Defaults to 0 (pod will be considered available as soon as it is ready)
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub min_ready_seconds: ::core::option::Option<i32>,
    /// Selector is a label query over pods that should match the replica count.
    /// If the selector is empty, it is defaulted to the labels present on the pod template.
    /// Label keys and values that must match in order to be controlled by this replica set.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    /// +optional
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// Template is the object that describes the pod that will be created if
    /// insufficient replicas are detected.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
    /// +optional
    #[prost(message, optional, tag="3")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
}
/// ReplicaSetStatus represents the current status of a ReplicaSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaSetStatus {
    /// Replicas is the most recently observed number of replicas.
    /// More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// The number of pods that have labels matching the labels of the pod template of the replicaset.
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub fully_labeled_replicas: ::core::option::Option<i32>,
    /// The number of ready replicas for this replica set.
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub ready_replicas: ::core::option::Option<i32>,
    /// The number of available replicas (ready for at least minReadySeconds) for this replica set.
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub available_replicas: ::core::option::Option<i32>,
    /// ObservedGeneration reflects the generation of the most recently observed ReplicaSet.
    /// +optional
    #[prost(int64, optional, tag="3")]
    pub observed_generation: ::core::option::Option<i64>,
    /// Represents the latest available observations of a replica set's current state.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="6")]
    pub conditions: ::prost::alloc::vec::Vec<ReplicaSetCondition>,
}
/// DEPRECATED.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackConfig {
    /// The revision to rollback to. If set to 0, rollback to the last revision.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub revision: ::core::option::Option<i64>,
}
/// Spec to control the desired behavior of daemon set rolling update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollingUpdateDaemonSet {
    /// The maximum number of DaemonSet pods that can be unavailable during the
    /// update. Value can be an absolute number (ex: 5) or a percentage of total
    /// number of DaemonSet pods at the start of the update (ex: 10%). Absolute
    /// number is calculated from percentage by rounding up.
    /// This cannot be 0 if MaxSurge is 0
    /// Default value is 1.
    /// Example: when this is set to 30%, at most 30% of the total number of nodes
    /// that should be running the daemon pod (i.e. status.desiredNumberScheduled)
    /// can have their pods stopped for an update at any given time. The update
    /// starts by stopping at most 30% of those DaemonSet pods and then brings
    /// up new DaemonSet pods in their place. Once the new pods are available,
    /// it then proceeds onto other DaemonSet pods, thus ensuring that at least
    /// 70% of original number of DaemonSet pods are available at all times during
    /// the update.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub max_unavailable: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// The maximum number of nodes with an existing available DaemonSet pod that
    /// can have an updated DaemonSet pod during during an update.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// This can not be 0 if MaxUnavailable is 0.
    /// Absolute number is calculated from percentage by rounding up to a minimum of 1.
    /// Default value is 0.
    /// Example: when this is set to 30%, at most 30% of the total number of nodes
    /// that should be running the daemon pod (i.e. status.desiredNumberScheduled)
    /// can have their a new pod created before the old pod is marked as deleted.
    /// The update starts by launching new pods on 30% of nodes. Once an updated
    /// pod is available (Ready for at least minReadySeconds) the old DaemonSet pod
    /// on that node is marked deleted. If the old pod becomes unavailable for any
    /// reason (Ready transitions to false, is evicted, or is drained) an updated
    /// pod is immediatedly created on that node without considering surge limits.
    /// Allowing surge implies the possibility that the resources consumed by the
    /// daemonset on any given node can double if the readiness check fails, and
    /// so resource intensive daemonsets should take into account that they may
    /// cause evictions during disruption.
    /// This is an alpha field and requires enabling DaemonSetUpdateSurge feature gate.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub max_surge: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
}
/// Spec to control the desired behavior of rolling update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollingUpdateDeployment {
    /// The maximum number of pods that can be unavailable during the update.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// Absolute number is calculated from percentage by rounding down.
    /// This can not be 0 if MaxSurge is 0.
    /// By default, a fixed value of 1 is used.
    /// Example: when this is set to 30%, the old RC can be scaled down to 70% of desired pods
    /// immediately when the rolling update starts. Once new pods are ready, old RC
    /// can be scaled down further, followed by scaling up the new RC, ensuring
    /// that the total number of pods available at all times during the update is at
    /// least 70% of desired pods.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub max_unavailable: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// The maximum number of pods that can be scheduled above the desired number of
    /// pods.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// This can not be 0 if MaxUnavailable is 0.
    /// Absolute number is calculated from percentage by rounding up.
    /// By default, a value of 1 is used.
    /// Example: when this is set to 30%, the new RC can be scaled up immediately when
    /// the rolling update starts, such that the total number of old and new pods do not exceed
    /// 130% of desired pods. Once old pods have been killed,
    /// new RC can be scaled up further, ensuring that total number of pods running
    /// at any time during the update is at most 130% of desired pods.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub max_surge: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
}
/// represents a scaling request for a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ScaleSpec>,
    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status. Read-only.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<ScaleStatus>,
}
/// describes the attributes of a scale subresource
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleSpec {
    /// desired number of instances for the scaled object.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
}
/// represents the current status of a scale subresource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleStatus {
    /// actual number of observed instances of the scaled object.
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// selector is a label query over pods that should match the replicas count. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
    /// +optional
    /// +mapType=atomic
    #[prost(map="string, string", tag="2")]
    pub selector: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// label selector for pods that should match the replicas count. This is a serializated
    /// version of both map-based and more expressive set-based selectors. This is done to
    /// avoid introspection in the clients. The string will be in the same format as the
    /// query-param syntax. If the target type only supports map-based selectors, both this
    /// field and map-based selector field are populated.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    /// +optional
    #[prost(string, optional, tag="3")]
    pub target_selector: ::core::option::Option<::prost::alloc::string::String>,
}
