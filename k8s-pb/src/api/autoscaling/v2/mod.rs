/// ContainerResourceMetricSource indicates how to scale on a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  The values will be averaged
/// together before being compared to the target.  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.  Only one "target" type
/// should be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerResourceMetricSource {
    /// name is the name of the resource in question.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// target specifies the target value for the given metric
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<MetricTarget>,
    /// container is the name of the container in the pods of the scaling target
    #[prost(string, optional, tag="3")]
    pub container: ::core::option::Option<::prost::alloc::string::String>,
}
/// ContainerResourceMetricStatus indicates the current value of a resource metric known to
/// Kubernetes, as specified in requests and limits, describing a single container in each pod in the
/// current scale target (e.g. CPU or memory).  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerResourceMetricStatus {
    /// name is the name of the resource in question.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// current contains the current value for the given metric
    #[prost(message, optional, tag="2")]
    pub current: ::core::option::Option<MetricValueStatus>,
    /// container is the name of the container in the pods of the scaling target
    #[prost(string, optional, tag="3")]
    pub container: ::core::option::Option<::prost::alloc::string::String>,
}
/// CrossVersionObjectReference contains enough information to let you identify the referred resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrossVersionObjectReference {
    /// kind is the kind of the referent; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[prost(string, optional, tag="1")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// name is the name of the referent; More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// apiVersion is the API version of the referent
    /// +optional
    #[prost(string, optional, tag="3")]
    pub api_version: ::core::option::Option<::prost::alloc::string::String>,
}
/// ExternalMetricSource indicates how to scale on a metric not associated with
/// any Kubernetes object (for example length of queue in cloud
/// messaging service, or QPS from loadbalancer running outside of cluster).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalMetricSource {
    /// metric identifies the target metric by name and selector
    #[prost(message, optional, tag="1")]
    pub metric: ::core::option::Option<MetricIdentifier>,
    /// target specifies the target value for the given metric
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<MetricTarget>,
}
/// ExternalMetricStatus indicates the current value of a global metric
/// not associated with any Kubernetes object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalMetricStatus {
    /// metric identifies the target metric by name and selector
    #[prost(message, optional, tag="1")]
    pub metric: ::core::option::Option<MetricIdentifier>,
    /// current contains the current value for the given metric
    #[prost(message, optional, tag="2")]
    pub current: ::core::option::Option<MetricValueStatus>,
}
/// HPAScalingPolicy is a single policy which must hold true for a specified past interval.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HpaScalingPolicy {
    /// type is used to specify the scaling policy.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// value contains the amount of change which is permitted by the policy.
    /// It must be greater than zero
    #[prost(int32, optional, tag="2")]
    pub value: ::core::option::Option<i32>,
    /// periodSeconds specifies the window of time for which the policy should hold true.
    /// PeriodSeconds must be greater than zero and less than or equal to 1800 (30 min).
    #[prost(int32, optional, tag="3")]
    pub period_seconds: ::core::option::Option<i32>,
}
/// HPAScalingRules configures the scaling behavior for one direction.
/// These Rules are applied after calculating DesiredReplicas from metrics for the HPA.
/// They can limit the scaling velocity by specifying scaling policies.
/// They can prevent flapping by specifying the stabilization window, so that the
/// number of replicas is not set instantly, instead, the safest value from the stabilization
/// window is chosen.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HpaScalingRules {
    /// stabilizationWindowSeconds is the number of seconds for which past recommendations should be
    /// considered while scaling up or scaling down.
    /// StabilizationWindowSeconds must be greater than or equal to zero and less than or equal to 3600 (one hour).
    /// If not set, use the default values:
    /// - For scale up: 0 (i.e. no stabilization is done).
    /// - For scale down: 300 (i.e. the stabilization window is 300 seconds long).
    /// +optional
    #[prost(int32, optional, tag="3")]
    pub stabilization_window_seconds: ::core::option::Option<i32>,
    /// selectPolicy is used to specify which policy should be used.
    /// If not set, the default value Max is used.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub select_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// policies is a list of potential scaling polices which can be used during scaling.
    /// At least one policy must be specified, otherwise the HPAScalingRules will be discarded as invalid
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub policies: ::prost::alloc::vec::Vec<HpaScalingPolicy>,
}
/// HorizontalPodAutoscaler is the configuration for a horizontal pod
/// autoscaler, which automatically manages the replica count of any resource
/// implementing the scale subresource based on the metrics specified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscaler {
    /// metadata is the standard object metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// spec is the specification for the behaviour of the autoscaler.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<HorizontalPodAutoscalerSpec>,
    /// status is the current information about the autoscaler.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<HorizontalPodAutoscalerStatus>,
}
/// HorizontalPodAutoscalerBehavior configures the scaling behavior of the target
/// in both Up and Down directions (scaleUp and scaleDown fields respectively).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscalerBehavior {
    /// scaleUp is scaling policy for scaling Up.
    /// If not set, the default value is the higher of:
    ///   * increase no more than 4 pods per 60 seconds
    ///   * double the number of pods per 60 seconds
    /// No stabilization is used.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub scale_up: ::core::option::Option<HpaScalingRules>,
    /// scaleDown is scaling policy for scaling Down.
    /// If not set, the default value is to allow to scale down to minReplicas pods, with a
    /// 300 second stabilization window (i.e., the highest recommendation for
    /// the last 300sec is used).
    /// +optional
    #[prost(message, optional, tag="2")]
    pub scale_down: ::core::option::Option<HpaScalingRules>,
}
/// HorizontalPodAutoscalerCondition describes the state of
/// a HorizontalPodAutoscaler at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscalerCondition {
    /// type describes the current condition
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// status is the status of the condition (True, False, Unknown)
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// lastTransitionTime is the last time the condition transitioned from
    /// one status to another
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// reason is the reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// message is a human-readable explanation containing details about
    /// the transition
    /// +optional
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// HorizontalPodAutoscalerList is a list of horizontal pod autoscaler objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscalerList {
    /// metadata is the standard list metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of horizontal pod autoscaler objects.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<HorizontalPodAutoscaler>,
}
/// HorizontalPodAutoscalerSpec describes the desired functionality of the HorizontalPodAutoscaler.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscalerSpec {
    /// scaleTargetRef points to the target resource to scale, and is used to the pods for which metrics
    /// should be collected, as well as to actually change the replica count.
    #[prost(message, optional, tag="1")]
    pub scale_target_ref: ::core::option::Option<CrossVersionObjectReference>,
    /// minReplicas is the lower limit for the number of replicas to which the autoscaler
    /// can scale down.  It defaults to 1 pod.  minReplicas is allowed to be 0 if the
    /// alpha feature gate HPAScaleToZero is enabled and at least one Object or External
    /// metric is configured.  Scaling is active as long as at least one metric value is
    /// available.
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub min_replicas: ::core::option::Option<i32>,
    /// maxReplicas is the upper limit for the number of replicas to which the autoscaler can scale up.
    /// It cannot be less that minReplicas.
    #[prost(int32, optional, tag="3")]
    pub max_replicas: ::core::option::Option<i32>,
    /// metrics contains the specifications for which to use to calculate the
    /// desired replica count (the maximum replica count across all metrics will
    /// be used).  The desired replica count is calculated multiplying the
    /// ratio between the target value and the current value by the current
    /// number of pods.  Ergo, metrics used must decrease as the pod count is
    /// increased, and vice-versa.  See the individual metric source types for
    /// more information about how each type of metric must respond.
    /// If not set, the default metric will be set to 80% average CPU utilization.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag="4")]
    pub metrics: ::prost::alloc::vec::Vec<MetricSpec>,
    /// behavior configures the scaling behavior of the target
    /// in both Up and Down directions (scaleUp and scaleDown fields respectively).
    /// If not set, the default HPAScalingRules for scale up and scale down are used.
    /// +optional
    #[prost(message, optional, tag="5")]
    pub behavior: ::core::option::Option<HorizontalPodAutoscalerBehavior>,
}
/// HorizontalPodAutoscalerStatus describes the current status of a horizontal pod autoscaler.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscalerStatus {
    /// observedGeneration is the most recent generation observed by this autoscaler.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods,
    /// used by the autoscaler to control how often the number of pods is changed.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub last_scale_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// currentReplicas is current number of replicas of pods managed by this autoscaler,
    /// as last seen by the autoscaler.
    /// +optional
    #[prost(int32, optional, tag="3")]
    pub current_replicas: ::core::option::Option<i32>,
    /// desiredReplicas is the desired number of replicas of pods managed by this autoscaler,
    /// as last calculated by the autoscaler.
    #[prost(int32, optional, tag="4")]
    pub desired_replicas: ::core::option::Option<i32>,
    /// currentMetrics is the last read state of the metrics used by this autoscaler.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag="5")]
    pub current_metrics: ::prost::alloc::vec::Vec<MetricStatus>,
    /// conditions is the set of conditions required for this autoscaler to scale its target,
    /// and indicates whether or not those conditions are met.
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=type
    /// +optional
    #[prost(message, repeated, tag="6")]
    pub conditions: ::prost::alloc::vec::Vec<HorizontalPodAutoscalerCondition>,
}
/// MetricIdentifier defines the name and optionally selector for a metric
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricIdentifier {
    /// name is the name of the given metric
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// selector is the string-encoded form of a standard kubernetes label selector for the given metric
    /// When set, it is passed as an additional parameter to the metrics server for more specific metrics scoping.
    /// When unset, just the metricName will be used to gather metrics.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
/// MetricSpec specifies how to scale based on a single metric
/// (only `type` and one other matching field should be set at once).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricSpec {
    /// type is the type of metric source.  It should be one of "ContainerResource", "External",
    /// "Object", "Pods" or "Resource", each mapping to a matching field in the object.
    /// Note: "ContainerResource" type is available on when the feature-gate
    /// HPAContainerMetrics is enabled
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// object refers to a metric describing a single kubernetes object
    /// (for example, hits-per-second on an Ingress object).
    /// +optional
    #[prost(message, optional, tag="2")]
    pub object: ::core::option::Option<ObjectMetricSource>,
    /// pods refers to a metric describing each pod in the current scale target
    /// (for example, transactions-processed-per-second).  The values will be
    /// averaged together before being compared to the target value.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub pods: ::core::option::Option<PodsMetricSource>,
    /// resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub resource: ::core::option::Option<ResourceMetricSource>,
    /// containerResource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing a single container in
    /// each pod of the current scale target (e.g. CPU or memory). Such metrics are
    /// built in to Kubernetes, and have special scaling options on top of those
    /// available to normal per-pod metrics using the "pods" source.
    /// This is an alpha feature and can be enabled by the HPAContainerMetrics feature flag.
    /// +optional
    #[prost(message, optional, tag="7")]
    pub container_resource: ::core::option::Option<ContainerResourceMetricSource>,
    /// external refers to a global metric that is not associated
    /// with any Kubernetes object. It allows autoscaling based on information
    /// coming from components running outside of cluster
    /// (for example length of queue in cloud messaging service, or
    /// QPS from loadbalancer running outside of cluster).
    /// +optional
    #[prost(message, optional, tag="5")]
    pub external: ::core::option::Option<ExternalMetricSource>,
}
/// MetricStatus describes the last-read state of a single metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricStatus {
    /// type is the type of metric source.  It will be one of "ContainerResource", "External",
    /// "Object", "Pods" or "Resource", each corresponds to a matching field in the object.
    /// Note: "ContainerResource" type is available on when the feature-gate
    /// HPAContainerMetrics is enabled
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// object refers to a metric describing a single kubernetes object
    /// (for example, hits-per-second on an Ingress object).
    /// +optional
    #[prost(message, optional, tag="2")]
    pub object: ::core::option::Option<ObjectMetricStatus>,
    /// pods refers to a metric describing each pod in the current scale target
    /// (for example, transactions-processed-per-second).  The values will be
    /// averaged together before being compared to the target value.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub pods: ::core::option::Option<PodsMetricStatus>,
    /// resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub resource: ::core::option::Option<ResourceMetricStatus>,
    /// container resource refers to a resource metric (such as those specified in
    /// requests and limits) known to Kubernetes describing a single container in each pod in the
    /// current scale target (e.g. CPU or memory). Such metrics are built in to
    /// Kubernetes, and have special scaling options on top of those available
    /// to normal per-pod metrics using the "pods" source.
    /// +optional
    #[prost(message, optional, tag="7")]
    pub container_resource: ::core::option::Option<ContainerResourceMetricStatus>,
    /// external refers to a global metric that is not associated
    /// with any Kubernetes object. It allows autoscaling based on information
    /// coming from components running outside of cluster
    /// (for example length of queue in cloud messaging service, or
    /// QPS from loadbalancer running outside of cluster).
    /// +optional
    #[prost(message, optional, tag="5")]
    pub external: ::core::option::Option<ExternalMetricStatus>,
}
/// MetricTarget defines the target value, average value, or average utilization of a specific metric
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricTarget {
    /// type represents whether the metric type is Utilization, Value, or AverageValue
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// value is the target value of the metric (as a quantity).
    /// +optional
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// averageValue is the target value of the average of the
    /// metric across all relevant pods (as a quantity)
    /// +optional
    #[prost(message, optional, tag="3")]
    pub average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// averageUtilization is the target value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.
    /// Currently only valid for Resource metric source type
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub average_utilization: ::core::option::Option<i32>,
}
/// MetricValueStatus holds the current value for a metric
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricValueStatus {
    /// value is the current value of the metric (as a quantity).
    /// +optional
    #[prost(message, optional, tag="1")]
    pub value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// averageValue is the current value of the average of the
    /// metric across all relevant pods (as a quantity)
    /// +optional
    #[prost(message, optional, tag="2")]
    pub average_value: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// currentAverageUtilization is the current value of the average of the
    /// resource metric across all relevant pods, represented as a percentage of
    /// the requested value of the resource for the pods.
    /// +optional
    #[prost(int32, optional, tag="3")]
    pub average_utilization: ::core::option::Option<i32>,
}
/// ObjectMetricSource indicates how to scale on a metric describing a
/// kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMetricSource {
    /// describedObject specifies the descriptions of a object,such as kind,name apiVersion
    #[prost(message, optional, tag="1")]
    pub described_object: ::core::option::Option<CrossVersionObjectReference>,
    /// target specifies the target value for the given metric
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<MetricTarget>,
    /// metric identifies the target metric by name and selector
    #[prost(message, optional, tag="3")]
    pub metric: ::core::option::Option<MetricIdentifier>,
}
/// ObjectMetricStatus indicates the current value of a metric describing a
/// kubernetes object (for example, hits-per-second on an Ingress object).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMetricStatus {
    /// metric identifies the target metric by name and selector
    #[prost(message, optional, tag="1")]
    pub metric: ::core::option::Option<MetricIdentifier>,
    /// current contains the current value for the given metric
    #[prost(message, optional, tag="2")]
    pub current: ::core::option::Option<MetricValueStatus>,
    /// DescribedObject specifies the descriptions of a object,such as kind,name apiVersion
    #[prost(message, optional, tag="3")]
    pub described_object: ::core::option::Option<CrossVersionObjectReference>,
}
/// PodsMetricSource indicates how to scale on a metric describing each pod in
/// the current scale target (for example, transactions-processed-per-second).
/// The values will be averaged together before being compared to the target
/// value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodsMetricSource {
    /// metric identifies the target metric by name and selector
    #[prost(message, optional, tag="1")]
    pub metric: ::core::option::Option<MetricIdentifier>,
    /// target specifies the target value for the given metric
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<MetricTarget>,
}
/// PodsMetricStatus indicates the current value of a metric describing each pod in
/// the current scale target (for example, transactions-processed-per-second).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodsMetricStatus {
    /// metric identifies the target metric by name and selector
    #[prost(message, optional, tag="1")]
    pub metric: ::core::option::Option<MetricIdentifier>,
    /// current contains the current value for the given metric
    #[prost(message, optional, tag="2")]
    pub current: ::core::option::Option<MetricValueStatus>,
}
/// ResourceMetricSource indicates how to scale on a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  The values will be averaged
/// together before being compared to the target.  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.  Only one "target" type
/// should be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMetricSource {
    /// name is the name of the resource in question.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// target specifies the target value for the given metric
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<MetricTarget>,
}
/// ResourceMetricStatus indicates the current value of a resource metric known to
/// Kubernetes, as specified in requests and limits, describing each pod in the
/// current scale target (e.g. CPU or memory).  Such metrics are built in to
/// Kubernetes, and have special scaling options on top of those available to
/// normal per-pod metrics using the "pods" source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMetricStatus {
    /// name is the name of the resource in question.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// current contains the current value for the given metric
    #[prost(message, optional, tag="2")]
    pub current: ::core::option::Option<MetricValueStatus>,
}

impl crate::Resource for HorizontalPodAutoscaler {
    const API_VERSION: &'static str = "autoscaling/v2";
    const GROUP: &'static str = "autoscaling";
    const VERSION: &'static str = "v2";
    const KIND: &'static str = "HorizontalPodAutoscaler";
    const NAME: &'static str = "horizontalpodautoscalers";
}
impl crate::HasMetadata for HorizontalPodAutoscaler {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for HorizontalPodAutoscaler {
    type Spec = crate::api::autoscaling::v2::HorizontalPodAutoscalerSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for HorizontalPodAutoscaler {
    type Status = crate::api::autoscaling::v2::HorizontalPodAutoscalerStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}
impl crate::HasConditions for HorizontalPodAutoscaler {
    type Condition = crate::api::autoscaling::v2::HorizontalPodAutoscalerCondition;
    fn conditions(&self) -> Option<&[<Self as crate::HasConditions>::Condition]> {
        self.status.as_ref().map(|s| s.conditions.as_slice())
    }
    fn conditions_mut(&mut self) -> Option<&mut Vec<<Self as crate::HasConditions>::Condition>> {
        self.status
            .as_mut()
            .and_then(|s| Some(s.conditions.as_mut()))
    }
}

