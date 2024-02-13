/// ExternalMetricValue is a metric value for external metric
/// A single metric value is identified by metric name and a set of string labels.
/// For one metric there can be multiple values with different sets of labels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalMetricValue {
    /// the name of the metric
    #[prost(string, optional, tag = "1")]
    pub metric_name: ::core::option::Option<::prost::alloc::string::String>,
    /// a set of labels that identify a single time series for the metric
    #[prost(btree_map = "string, string", tag = "2")]
    pub metric_labels: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// indicates the time at which the metrics were produced
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<
        super::super::super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
    /// indicates the window (\[Timestamp-Window, Timestamp\]) from
    /// which these metrics were calculated, when returning rate
    /// metrics calculated from cumulative metrics (or zero for
    /// non-calculated instantaneous metrics).
    #[prost(int64, optional, tag = "4")]
    pub window: ::core::option::Option<i64>,
    /// the value of the metric
    #[prost(message, optional, tag = "5")]
    pub value: ::core::option::Option<
        super::super::super::super::super::apimachinery::pkg::api::resource::Quantity,
    >,
}
/// ExternalMetricValueList is a list of values for a given metric for some set labels
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalMetricValueList {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// value of the metric matching a given set of labels
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ExternalMetricValue>,
}
