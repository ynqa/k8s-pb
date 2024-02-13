/// AdmissionRequest describes the admission.Attributes for the admission request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdmissionRequest {
    /// UID is an identifier for the individual request/response. It allows us to distinguish instances of requests which are
    /// otherwise identical (parallel requests, requests when earlier requests did not modify etc)
    /// The UID is meant to track the round trip (request/response) between the KAS and the WebHook, not the user request.
    /// It is suitable for correlating log entries between the webhook and apiserver, for either auditing or debugging.
    #[prost(string, optional, tag = "1")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// Kind is the fully-qualified type of object being submitted (for example, v1.Pod or autoscaling.v1.Scale)
    #[prost(message, optional, tag = "2")]
    pub kind: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::GroupVersionKind,
    >,
    /// Resource is the fully-qualified resource being requested (for example, v1.pods)
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::GroupVersionResource,
    >,
    /// SubResource is the subresource being requested, if any (for example, "status" or "scale")
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub sub_resource: ::core::option::Option<::prost::alloc::string::String>,
    /// RequestKind is the fully-qualified type of the original API request (for example, v1.Pod or autoscaling.v1.Scale).
    /// If this is specified and differs from the value in "kind", an equivalent match and conversion was performed.
    ///
    /// For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
    /// `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]` and `matchPolicy: Equivalent`,
    /// an API request to apps/v1beta1 deployments would be converted and sent to the webhook
    /// with `kind: {group:"apps", version:"v1", kind:"Deployment"}` (matching the rule the webhook registered for),
    /// and `requestKind: {group:"apps", version:"v1beta1", kind:"Deployment"}` (indicating the kind of the original API request).
    ///
    /// See documentation for the "matchPolicy" field in the webhook configuration type for more details.
    /// +optional
    #[prost(message, optional, tag = "13")]
    pub request_kind: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::GroupVersionKind,
    >,
    /// RequestResource is the fully-qualified resource of the original API request (for example, v1.pods).
    /// If this is specified and differs from the value in "resource", an equivalent match and conversion was performed.
    ///
    /// For example, if deployments can be modified via apps/v1 and apps/v1beta1, and a webhook registered a rule of
    /// `apiGroups:\["apps"\], apiVersions:\["v1"\], resources: \["deployments"\]` and `matchPolicy: Equivalent`,
    /// an API request to apps/v1beta1 deployments would be converted and sent to the webhook
    /// with `resource: {group:"apps", version:"v1", resource:"deployments"}` (matching the resource the webhook registered for),
    /// and `requestResource: {group:"apps", version:"v1beta1", resource:"deployments"}` (indicating the resource of the original API request).
    ///
    /// See documentation for the "matchPolicy" field in the webhook configuration type.
    /// +optional
    #[prost(message, optional, tag = "14")]
    pub request_resource: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::GroupVersionResource,
    >,
    /// RequestSubResource is the name of the subresource of the original API request, if any (for example, "status" or "scale")
    /// If this is specified and differs from the value in "subResource", an equivalent match and conversion was performed.
    /// See documentation for the "matchPolicy" field in the webhook configuration type.
    /// +optional
    #[prost(string, optional, tag = "15")]
    pub request_sub_resource: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of the object as presented in the request.  On a CREATE operation, the client may omit name and
    /// rely on the server to generate the name.  If that is the case, this field will contain an empty string.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace is the namespace associated with the request (if any).
    /// +optional
    #[prost(string, optional, tag = "6")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Operation is the operation being performed. This may be different than the operation
    /// requested. e.g. a patch can result in either a CREATE or UPDATE Operation.
    #[prost(string, optional, tag = "7")]
    pub operation: ::core::option::Option<::prost::alloc::string::String>,
    /// UserInfo is information about the requesting user
    #[prost(message, optional, tag = "8")]
    pub user_info: ::core::option::Option<super::super::authentication::v1::UserInfo>,
    /// Object is the object from the incoming request.
    /// +optional
    #[prost(message, optional, tag = "9")]
    pub object: ::core::option::Option<
        super::super::super::apimachinery::pkg::runtime::RawExtension,
    >,
    /// OldObject is the existing object. Only populated for DELETE and UPDATE requests.
    /// +optional
    #[prost(message, optional, tag = "10")]
    pub old_object: ::core::option::Option<
        super::super::super::apimachinery::pkg::runtime::RawExtension,
    >,
    /// DryRun indicates that modifications will definitely not be persisted for this request.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag = "11")]
    pub dry_run: ::core::option::Option<bool>,
    /// Options is the operation option structure of the operation being performed.
    /// e.g. `meta.k8s.io/v1.DeleteOptions` or `meta.k8s.io/v1.CreateOptions`. This may be
    /// different than the options the caller provided. e.g. for a patch request the performed
    /// Operation might be a CREATE, in which case the Options will a
    /// `meta.k8s.io/v1.CreateOptions` even though the caller provided `meta.k8s.io/v1.PatchOptions`.
    /// +optional
    #[prost(message, optional, tag = "12")]
    pub options: ::core::option::Option<
        super::super::super::apimachinery::pkg::runtime::RawExtension,
    >,
}
/// AdmissionResponse describes an admission response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdmissionResponse {
    /// UID is an identifier for the individual request/response.
    /// This should be copied over from the corresponding AdmissionRequest.
    #[prost(string, optional, tag = "1")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// Allowed indicates whether or not the admission request was permitted.
    #[prost(bool, optional, tag = "2")]
    pub allowed: ::core::option::Option<bool>,
    /// Result contains extra details into why an admission request was denied.
    /// This field IS NOT consulted in any way if "Allowed" is "true".
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::Status,
    >,
    /// The patch body. Currently we only support "JSONPatch" which implements RFC 6902.
    /// +optional
    #[prost(bytes = "vec", optional, tag = "4")]
    pub patch: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// The type of Patch. Currently we only allow "JSONPatch".
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub patch_type: ::core::option::Option<::prost::alloc::string::String>,
    /// AuditAnnotations is an unstructured key value map set by remote admission controller (e.g. error=image-blacklisted).
    /// MutatingAdmissionWebhook and ValidatingAdmissionWebhook admission controller will prefix the keys with
    /// admission webhook name (e.g. imagepolicy.example.com/error=image-blacklisted). AuditAnnotations will be provided by
    /// the admission webhook to add additional context to the audit log for this request.
    /// +optional
    #[prost(btree_map = "string, string", tag = "6")]
    pub audit_annotations: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// warnings is a list of warning messages to return to the requesting API client.
    /// Warning messages describe a problem the client making the API request should correct or be aware of.
    /// Limit warnings to 120 characters if possible.
    /// Warnings over 256 characters and large numbers of warnings may be truncated.
    /// +optional
    #[prost(string, repeated, tag = "7")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AdmissionReview describes an admission review request/response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdmissionReview {
    /// Request describes the attributes for the admission request.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<AdmissionRequest>,
    /// Response describes the attributes for the admission response.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub response: ::core::option::Option<AdmissionResponse>,
}
