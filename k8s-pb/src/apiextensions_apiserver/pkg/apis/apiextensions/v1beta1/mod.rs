/// ConversionRequest describes the conversion request parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionRequest {
    /// uid is an identifier for the individual request/response. It allows distinguishing instances of requests which are
    /// otherwise identical (parallel requests, etc).
    /// The UID is meant to track the round trip (request/response) between the Kubernetes API server and the webhook, not the user request.
    /// It is suitable for correlating log entries between the webhook and apiserver, for either auditing or debugging.
    #[prost(string, optional, tag = "1")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// desiredAPIVersion is the version to convert given objects to. e.g. "myapi.example.com/v1"
    #[prost(string, optional, tag = "2")]
    pub desired_api_version: ::core::option::Option<::prost::alloc::string::String>,
    /// objects is the list of custom resource objects to be converted.
    #[prost(message, repeated, tag = "3")]
    pub objects: ::prost::alloc::vec::Vec<
        super::super::super::super::super::apimachinery::pkg::runtime::RawExtension,
    >,
}
/// ConversionResponse describes a conversion response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionResponse {
    /// uid is an identifier for the individual request/response.
    /// This should be copied over from the corresponding `request.uid`.
    #[prost(string, optional, tag = "1")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// convertedObjects is the list of converted version of `request.objects` if the `result` is successful, otherwise empty.
    /// The webhook is expected to set `apiVersion` of these objects to the `request.desiredAPIVersion`. The list
    /// must also have the same size as the input list with the same objects in the same order (equal kind, metadata.uid, metadata.name and metadata.namespace).
    /// The webhook is allowed to mutate labels and annotations. Any other change to the metadata is silently ignored.
    #[prost(message, repeated, tag = "2")]
    pub converted_objects: ::prost::alloc::vec::Vec<
        super::super::super::super::super::apimachinery::pkg::runtime::RawExtension,
    >,
    /// result contains the result of conversion with extra details if the conversion failed. `result.status` determines if
    /// the conversion failed or succeeded. The `result.status` field is required and represents the success or failure of the
    /// conversion. A successful conversion must set `result.status` to `Success`. A failed conversion must set
    /// `result.status` to `Failure` and provide more details in `result.message` and return http status 200. The `result.message`
    /// will be used to construct an error message for the end user.
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<
        super::super::super::super::super::apimachinery::pkg::apis::meta::v1::Status,
    >,
}
/// ConversionReview describes a conversion request/response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionReview {
    /// request describes the attributes for the conversion request.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<ConversionRequest>,
    /// response describes the attributes for the conversion response.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub response: ::core::option::Option<ConversionResponse>,
}
/// CustomResourceColumnDefinition specifies a column for server side printing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceColumnDefinition {
    /// name is a human readable name for the column.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// type is an OpenAPI type definition for this column.
    /// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types> for details.
    #[prost(string, optional, tag = "2")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// format is an optional OpenAPI type definition for this column. The 'name' format is applied
    /// to the primary identifier column to assist in clients identifying column is the resource name.
    /// See <https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types> for details.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub format: ::core::option::Option<::prost::alloc::string::String>,
    /// description is a human readable description of this column.
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// priority is an integer defining the relative importance of this column compared to others. Lower
    /// numbers are considered higher priority. Columns that may be omitted in limited space scenarios
    /// should be given a priority greater than 0.
    /// +optional
    #[prost(int32, optional, tag = "5")]
    pub priority: ::core::option::Option<i32>,
    /// JSONPath is a simple JSON path (i.e. with array notation) which is evaluated against
    /// each custom resource to produce the value for this column.
    #[prost(string, optional, tag = "6")]
    pub json_path: ::core::option::Option<::prost::alloc::string::String>,
}
/// CustomResourceConversion describes how to convert different versions of a CR.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceConversion {
    /// strategy specifies how custom resources are converted between versions. Allowed values are:
    /// - `None`: The converter only change the apiVersion and would not touch any other field in the custom resource.
    /// - `Webhook`: API Server will call to an external webhook to do the conversion. Additional information
    ///    is needed for this option. This requires spec.preserveUnknownFields to be false, and spec.conversion.webhookClientConfig to be set.
    #[prost(string, optional, tag = "1")]
    pub strategy: ::core::option::Option<::prost::alloc::string::String>,
    /// webhookClientConfig is the instructions for how to call the webhook if strategy is `Webhook`.
    /// Required when `strategy` is set to `Webhook`.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub webhook_client_config: ::core::option::Option<WebhookClientConfig>,
    /// conversionReviewVersions is an ordered list of preferred `ConversionReview`
    /// versions the Webhook expects. The API server will use the first version in
    /// the list which it supports. If none of the versions specified in this list
    /// are supported by API server, conversion will fail for the custom resource.
    /// If a persisted Webhook configuration specifies allowed versions and does not
    /// include any versions known to the API Server, calls to the webhook will fail.
    /// Defaults to `\["v1beta1"\]`.
    /// +optional
    #[prost(string, repeated, tag = "3")]
    pub conversion_review_versions: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// CustomResourceDefinition represents a resource that should be exposed on the API server.  Its name MUST be in the format
/// <.spec.name>.<.spec.group>.
/// Deprecated in v1.16, planned for removal in v1.22. Use apiextensions.k8s.io/v1 CustomResourceDefinition instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceDefinition {
    /// Standard object's metadata
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// spec describes how the user wants the resources to appear
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<CustomResourceDefinitionSpec>,
    /// status indicates the actual state of the CustomResourceDefinition
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<CustomResourceDefinitionStatus>,
}
/// CustomResourceDefinitionCondition contains details for the current condition of this pod.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceDefinitionCondition {
    /// type is the type of the condition. Types include Established, NamesAccepted and Terminating.
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// status is the status of the condition.
    /// Can be True, False, Unknown.
    #[prost(string, optional, tag = "2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// lastTransitionTime last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub last_transition_time: ::core::option::Option<
        super::super::super::super::super::apimachinery::pkg::apis::meta::v1::Time,
    >,
    /// reason is a unique, one-word, CamelCase reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// message is a human-readable message indicating details about last transition.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// CustomResourceDefinitionList is a list of CustomResourceDefinition objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceDefinitionList {
    /// Standard object's metadata
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// items list individual CustomResourceDefinition objects
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<CustomResourceDefinition>,
}
/// CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceDefinitionNames {
    /// plural is the plural name of the resource to serve.
    /// The custom resources are served under `/apis/<group>/<version>/.../<plural>`.
    /// Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`).
    /// Must be all lowercase.
    #[prost(string, optional, tag = "1")]
    pub plural: ::core::option::Option<::prost::alloc::string::String>,
    /// singular is the singular name of the resource. It must be all lowercase. Defaults to lowercased `kind`.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub singular: ::core::option::Option<::prost::alloc::string::String>,
    /// shortNames are short names for the resource, exposed in API discovery documents,
    /// and used by clients to support invocations like `kubectl get <shortname>`.
    /// It must be all lowercase.
    /// +optional
    #[prost(string, repeated, tag = "3")]
    pub short_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// kind is the serialized kind of the resource. It is normally CamelCase and singular.
    /// Custom resource instances will use this value as the `kind` attribute in API calls.
    #[prost(string, optional, tag = "4")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// listKind is the serialized kind of the list for this resource. Defaults to "`kind`List".
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub list_kind: ::core::option::Option<::prost::alloc::string::String>,
    /// categories is a list of grouped resources this custom resource belongs to (e.g. 'all').
    /// This is published in API discovery documents, and used by clients to support invocations like
    /// `kubectl get all`.
    /// +optional
    #[prost(string, repeated, tag = "6")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CustomResourceDefinitionSpec describes how a user wants their resource to appear
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceDefinitionSpec {
    /// group is the API group of the defined custom resource.
    /// The custom resources are served under `/apis/<group>/...`.
    /// Must match the name of the CustomResourceDefinition (in the form `<names.plural>.<group>`).
    #[prost(string, optional, tag = "1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// version is the API version of the defined custom resource.
    /// The custom resources are served under `/apis/<group>/<version>/...`.
    /// Must match the name of the first item in the `versions` list if `version` and `versions` are both specified.
    /// Optional if `versions` is specified.
    /// Deprecated: use `versions` instead.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// names specify the resource and kind names for the custom resource.
    #[prost(message, optional, tag = "3")]
    pub names: ::core::option::Option<CustomResourceDefinitionNames>,
    /// scope indicates whether the defined custom resource is cluster- or namespace-scoped.
    /// Allowed values are `Cluster` and `Namespaced`. Default is `Namespaced`.
    #[prost(string, optional, tag = "4")]
    pub scope: ::core::option::Option<::prost::alloc::string::String>,
    /// validation describes the schema used for validation and pruning of the custom resource.
    /// If present, this validation schema is used to validate all versions.
    /// Top-level and per-version schemas are mutually exclusive.
    /// +optional
    #[prost(message, optional, tag = "5")]
    pub validation: ::core::option::Option<CustomResourceValidation>,
    /// subresources specify what subresources the defined custom resource has.
    /// If present, this field configures subresources for all versions.
    /// Top-level and per-version subresources are mutually exclusive.
    /// +optional
    #[prost(message, optional, tag = "6")]
    pub subresources: ::core::option::Option<CustomResourceSubresources>,
    /// versions is the list of all API versions of the defined custom resource.
    /// Optional if `version` is specified.
    /// The name of the first item in the `versions` list must match the `version` field if `version` and `versions` are both specified.
    /// Version names are used to compute the order in which served versions are listed in API discovery.
    /// If the version string is "kube-like", it will sort above non "kube-like" version strings, which are ordered
    /// lexicographically. "Kube-like" versions start with a "v", then are followed by a number (the major version),
    /// then optionally the string "alpha" or "beta" and another number (the minor version). These are sorted first
    /// by GA > beta > alpha (where GA is a version with no suffix such as beta or alpha), and then by comparing
    /// major version, then minor version. An example sorted list of versions:
    /// v10, v2, v1, v11beta2, v10beta3, v3beta1, v12alpha1, v11alpha2, foo1, foo10.
    /// +optional
    #[prost(message, repeated, tag = "7")]
    pub versions: ::prost::alloc::vec::Vec<CustomResourceDefinitionVersion>,
    /// additionalPrinterColumns specifies additional columns returned in Table output.
    /// See <https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables> for details.
    /// If present, this field configures columns for all versions.
    /// Top-level and per-version columns are mutually exclusive.
    /// If no top-level or per-version columns are specified, a single column displaying the age of the custom resource is used.
    /// +optional
    #[prost(message, repeated, tag = "8")]
    pub additional_printer_columns: ::prost::alloc::vec::Vec<
        CustomResourceColumnDefinition,
    >,
    /// conversion defines conversion settings for the CRD.
    /// +optional
    #[prost(message, optional, tag = "9")]
    pub conversion: ::core::option::Option<CustomResourceConversion>,
    /// preserveUnknownFields indicates that object fields which are not specified
    /// in the OpenAPI schema should be preserved when persisting to storage.
    /// apiVersion, kind, metadata and known fields inside metadata are always preserved.
    /// If false, schemas must be defined for all versions.
    /// Defaults to true in v1beta for backwards compatibility.
    /// Deprecated: will be required to be false in v1. Preservation of unknown fields can be specified
    /// in the validation schema using the `x-kubernetes-preserve-unknown-fields: true` extension.
    /// See <https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/#field-pruning> for details.
    /// +optional
    #[prost(bool, optional, tag = "10")]
    pub preserve_unknown_fields: ::core::option::Option<bool>,
}
/// CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceDefinitionStatus {
    /// conditions indicate state for particular aspects of a CustomResourceDefinition
    /// +optional
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag = "1")]
    pub conditions: ::prost::alloc::vec::Vec<CustomResourceDefinitionCondition>,
    /// acceptedNames are the names that are actually being used to serve discovery.
    /// They may be different than the names in spec.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub accepted_names: ::core::option::Option<CustomResourceDefinitionNames>,
    /// storedVersions lists all versions of CustomResources that were ever persisted. Tracking these
    /// versions allows a migration path for stored versions in etcd. The field is mutable
    /// so a migration controller can finish a migration to another version (ensuring
    /// no old objects are left in storage), and then remove the rest of the
    /// versions from this list.
    /// Versions may not be removed from `spec.versions` while they exist in this list.
    /// +optional
    #[prost(string, repeated, tag = "3")]
    pub stored_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CustomResourceDefinitionVersion describes a version for CRD.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceDefinitionVersion {
    /// name is the version name, e.g. “v1”, “v2beta1”, etc.
    /// The custom resources are served under this version at `/apis/<group>/<version>/...` if `served` is true.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// served is a flag enabling/disabling this version from being served via REST APIs
    #[prost(bool, optional, tag = "2")]
    pub served: ::core::option::Option<bool>,
    /// storage indicates this version should be used when persisting custom resources to storage.
    /// There must be exactly one version with storage=true.
    #[prost(bool, optional, tag = "3")]
    pub storage: ::core::option::Option<bool>,
    /// deprecated indicates this version of the custom resource API is deprecated.
    /// When set to true, API requests to this version receive a warning header in the server response.
    /// Defaults to false.
    /// +optional
    #[prost(bool, optional, tag = "7")]
    pub deprecated: ::core::option::Option<bool>,
    /// deprecationWarning overrides the default warning returned to API clients.
    /// May only be set when `deprecated` is true.
    /// The default warning indicates this version is deprecated and recommends use
    /// of the newest served version of equal or greater stability, if one exists.
    /// +optional
    #[prost(string, optional, tag = "8")]
    pub deprecation_warning: ::core::option::Option<::prost::alloc::string::String>,
    /// schema describes the schema used for validation and pruning of this version of the custom resource.
    /// Top-level and per-version schemas are mutually exclusive.
    /// Per-version schemas must not all be set to identical values (top-level validation schema should be used instead).
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub schema: ::core::option::Option<CustomResourceValidation>,
    /// subresources specify what subresources this version of the defined custom resource have.
    /// Top-level and per-version subresources are mutually exclusive.
    /// Per-version subresources must not all be set to identical values (top-level subresources should be used instead).
    /// +optional
    #[prost(message, optional, tag = "5")]
    pub subresources: ::core::option::Option<CustomResourceSubresources>,
    /// additionalPrinterColumns specifies additional columns returned in Table output.
    /// See <https://kubernetes.io/docs/reference/using-api/api-concepts/#receiving-resources-as-tables> for details.
    /// Top-level and per-version columns are mutually exclusive.
    /// Per-version columns must not all be set to identical values (top-level columns should be used instead).
    /// If no top-level or per-version columns are specified, a single column displaying the age of the custom resource is used.
    /// +optional
    #[prost(message, repeated, tag = "6")]
    pub additional_printer_columns: ::prost::alloc::vec::Vec<
        CustomResourceColumnDefinition,
    >,
}
/// CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceSubresourceScale {
    /// specReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `spec.replicas`.
    /// Only JSON paths without the array notation are allowed.
    /// Must be a JSON Path under `.spec`.
    /// If there is no value under the given path in the custom resource, the `/scale` subresource will return an error on GET.
    #[prost(string, optional, tag = "1")]
    pub spec_replicas_path: ::core::option::Option<::prost::alloc::string::String>,
    /// statusReplicasPath defines the JSON path inside of a custom resource that corresponds to Scale `status.replicas`.
    /// Only JSON paths without the array notation are allowed.
    /// Must be a JSON Path under `.status`.
    /// If there is no value under the given path in the custom resource, the `status.replicas` value in the `/scale` subresource
    /// will default to 0.
    #[prost(string, optional, tag = "2")]
    pub status_replicas_path: ::core::option::Option<::prost::alloc::string::String>,
    /// labelSelectorPath defines the JSON path inside of a custom resource that corresponds to Scale `status.selector`.
    /// Only JSON paths without the array notation are allowed.
    /// Must be a JSON Path under `.status` or `.spec`.
    /// Must be set to work with HorizontalPodAutoscaler.
    /// The field pointed by this JSON path must be a string field (not a complex selector struct)
    /// which contains a serialized label selector in string form.
    /// More info: <https://kubernetes.io/docs/tasks/access-kubernetes-api/custom-resources/custom-resource-definitions#scale-subresource>
    /// If there is no value under the given path in the custom resource, the `status.selector` value in the `/scale`
    /// subresource will default to the empty string.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub label_selector_path: ::core::option::Option<::prost::alloc::string::String>,
}
/// CustomResourceSubresourceStatus defines how to serve the status subresource for CustomResources.
/// Status is represented by the `.status` JSON path inside of a CustomResource. When set,
/// * exposes a /status subresource for the custom resource
/// * PUT requests to the /status subresource take a custom resource object, and ignore changes to anything except the status stanza
/// * PUT/POST/PATCH requests to the custom resource ignore changes to the status stanza
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceSubresourceStatus {}
/// CustomResourceSubresources defines the status and scale subresources for CustomResources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceSubresources {
    /// status indicates the custom resource should serve a `/status` subresource.
    /// When enabled:
    /// 1. requests to the custom resource primary endpoint ignore changes to the `status` stanza of the object.
    /// 2. requests to the custom resource `/status` subresource ignore changes to anything other than the `status` stanza of the object.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<CustomResourceSubresourceStatus>,
    /// scale indicates the custom resource should serve a `/scale` subresource that returns an `autoscaling/v1` Scale object.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub scale: ::core::option::Option<CustomResourceSubresourceScale>,
}
/// CustomResourceValidation is a list of validation methods for CustomResources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomResourceValidation {
    /// openAPIV3Schema is the OpenAPI v3 schema to use for validation and pruning.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub open_apiv3_schema: ::core::option::Option<JsonSchemaProps>,
}
/// ExternalDocumentation allows referencing an external resource for extended documentation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalDocumentation {
    #[prost(string, optional, tag = "1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
/// JSON represents any valid JSON value.
/// These types are supported: bool, int64, float64, string, \[\]interface{}, map\[string\]interface{} and nil.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Json {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub raw: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// JSONSchemaProps is a JSON-Schema following Specification Draft 4 (<http://json-schema.org/>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonSchemaProps {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub schema: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub r#ref: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// format is an OpenAPI v3 format string. Unknown formats are ignored. The following formats are validated:
    ///
    /// - bsonobjectid: a bson object ID, i.e. a 24 characters hex string
    /// - uri: an URI as parsed by Golang net/url.ParseRequestURI
    /// - email: an email address as parsed by Golang net/mail.ParseAddress
    /// - hostname: a valid representation for an Internet host name, as defined by RFC 1034, section 3.1 \[RFC1034\].
    /// - ipv4: an IPv4 IP as parsed by Golang net.ParseIP
    /// - ipv6: an IPv6 IP as parsed by Golang net.ParseIP
    /// - cidr: a CIDR as parsed by Golang net.ParseCIDR
    /// - mac: a MAC address as parsed by Golang net.ParseMAC
    /// - uuid: an UUID that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?\[0-9a-f\]{4}-?\[0-9a-f\]{4}-?\[0-9a-f\]{12}$
    /// - uuid3: an UUID3 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?3\[0-9a-f\]{3}-?\[0-9a-f\]{4}-?\[0-9a-f\]{12}$
    /// - uuid4: an UUID4 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?4\[0-9a-f\]{3}-?[89ab][0-9a-f]{3}-?\[0-9a-f\]{12}$
    /// - uuid5: an UUID5 that allows uppercase defined by the regex (?i)^\[0-9a-f\]{8}-?\[0-9a-f\]{4}-?5\[0-9a-f\]{3}-?[89ab][0-9a-f]{3}-?\[0-9a-f\]{12}$
    /// - isbn: an ISBN10 or ISBN13 number string like "0321751043" or "978-0321751041"
    /// - isbn10: an ISBN10 number string like "0321751043"
    /// - isbn13: an ISBN13 number string like "978-0321751041"
    /// - creditcard: a credit card number defined by the regex ^(?:4\[0-9\]{12}(?:\[0-9\]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9][0-9])\[0-9\]{12}|3[47][0-9]{13}|3(?:0\[0-5\]|[68][0-9])\[0-9\]{11}|(?:2131|1800|35\\d{3})\\d{11})$ with any non digit characters mixed in
    /// - ssn: a U.S. social security number following the regex ^\\d{3}\[- \]?\\d{2}\[- \]?\\d{4}$
    /// - hexcolor: an hexadecimal color code like "#FFFFFF: following the regex ^#?(\[0-9a-fA-F\]{3}|\[0-9a-fA-F\]{6})$
    /// - rgbcolor: an RGB color code like rgb like "rgb(255,255,2559"
    /// - byte: base64 encoded binary data
    /// - password: any kind of string
    /// - date: a date string like "2006-01-02" as defined by full-date in RFC3339
    /// - duration: a duration string like "22 ns" as parsed by Golang time.ParseDuration or compatible with Scala duration format
    /// - datetime: a date time string like "2014-12-15T19:30:20.000Z" as defined by date-time in RFC3339.
    #[prost(string, optional, tag = "6")]
    pub format: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// default is a default value for undefined object fields.
    /// Defaulting is a beta feature under the CustomResourceDefaulting feature gate.
    /// CustomResourceDefinitions with defaults must be created using the v1 (or newer) CustomResourceDefinition API.
    #[prost(message, optional, tag = "8")]
    pub default: ::core::option::Option<Json>,
    #[prost(double, optional, tag = "9")]
    pub maximum: ::core::option::Option<f64>,
    #[prost(bool, optional, tag = "10")]
    pub exclusive_maximum: ::core::option::Option<bool>,
    #[prost(double, optional, tag = "11")]
    pub minimum: ::core::option::Option<f64>,
    #[prost(bool, optional, tag = "12")]
    pub exclusive_minimum: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "13")]
    pub max_length: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "14")]
    pub min_length: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "15")]
    pub pattern: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "16")]
    pub max_items: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "17")]
    pub min_items: ::core::option::Option<i64>,
    #[prost(bool, optional, tag = "18")]
    pub unique_items: ::core::option::Option<bool>,
    #[prost(double, optional, tag = "19")]
    pub multiple_of: ::core::option::Option<f64>,
    #[prost(message, repeated, tag = "20")]
    pub r#enum: ::prost::alloc::vec::Vec<Json>,
    #[prost(int64, optional, tag = "21")]
    pub max_properties: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "22")]
    pub min_properties: ::core::option::Option<i64>,
    #[prost(string, repeated, tag = "23")]
    pub required: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, boxed, tag = "24")]
    pub items: ::core::option::Option<
        ::prost::alloc::boxed::Box<JsonSchemaPropsOrArray>,
    >,
    #[prost(message, repeated, tag = "25")]
    pub all_of: ::prost::alloc::vec::Vec<JsonSchemaProps>,
    #[prost(message, repeated, tag = "26")]
    pub one_of: ::prost::alloc::vec::Vec<JsonSchemaProps>,
    #[prost(message, repeated, tag = "27")]
    pub any_of: ::prost::alloc::vec::Vec<JsonSchemaProps>,
    #[prost(message, optional, boxed, tag = "28")]
    pub not: ::core::option::Option<::prost::alloc::boxed::Box<JsonSchemaProps>>,
    #[prost(map = "string, message", tag = "29")]
    pub properties: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JsonSchemaProps,
    >,
    #[prost(message, optional, boxed, tag = "30")]
    pub additional_properties: ::core::option::Option<
        ::prost::alloc::boxed::Box<JsonSchemaPropsOrBool>,
    >,
    #[prost(map = "string, message", tag = "31")]
    pub pattern_properties: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JsonSchemaProps,
    >,
    #[prost(map = "string, message", tag = "32")]
    pub dependencies: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JsonSchemaPropsOrStringArray,
    >,
    #[prost(message, optional, boxed, tag = "33")]
    pub additional_items: ::core::option::Option<
        ::prost::alloc::boxed::Box<JsonSchemaPropsOrBool>,
    >,
    #[prost(map = "string, message", tag = "34")]
    pub definitions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JsonSchemaProps,
    >,
    #[prost(message, optional, tag = "35")]
    pub external_docs: ::core::option::Option<ExternalDocumentation>,
    #[prost(message, optional, tag = "36")]
    pub example: ::core::option::Option<Json>,
    #[prost(bool, optional, tag = "37")]
    pub nullable: ::core::option::Option<bool>,
    /// x-kubernetes-preserve-unknown-fields stops the API server
    /// decoding step from pruning fields which are not specified
    /// in the validation schema. This affects fields recursively,
    /// but switches back to normal pruning behaviour if nested
    /// properties or additionalProperties are specified in the schema.
    /// This can either be true or undefined. False is forbidden.
    #[prost(bool, optional, tag = "38")]
    pub x_kubernetes_preserve_unknown_fields: ::core::option::Option<bool>,
    /// x-kubernetes-embedded-resource defines that the value is an
    /// embedded Kubernetes runtime.Object, with TypeMeta and
    /// ObjectMeta. The type must be object. It is allowed to further
    /// restrict the embedded object. kind, apiVersion and metadata
    /// are validated automatically. x-kubernetes-preserve-unknown-fields
    /// is allowed to be true, but does not have to be if the object
    /// is fully specified (up to kind, apiVersion, metadata).
    #[prost(bool, optional, tag = "39")]
    pub x_kubernetes_embedded_resource: ::core::option::Option<bool>,
    /// x-kubernetes-int-or-string specifies that this value is
    /// either an integer or a string. If this is true, an empty
    /// type is allowed and type as child of anyOf is permitted
    /// if following one of the following patterns:
    ///
    /// 1) anyOf:
    ///     - type: integer
    ///     - type: string
    /// 2) allOf:
    ///     - anyOf:
    ///       - type: integer
    ///       - type: string
    ///     - ... zero or more
    #[prost(bool, optional, tag = "40")]
    pub x_kubernetes_int_or_string: ::core::option::Option<bool>,
    /// x-kubernetes-list-map-keys annotates an array with the x-kubernetes-list-type `map` by specifying the keys used
    /// as the index of the map.
    ///
    /// This tag MUST only be used on lists that have the "x-kubernetes-list-type"
    /// extension set to "map". Also, the values specified for this attribute must
    /// be a scalar typed field of the child structure (no nesting is supported).
    ///
    /// The properties specified must either be required or have a default value,
    /// to ensure those properties are present for all list items.
    ///
    /// +optional
    #[prost(string, repeated, tag = "41")]
    pub x_kubernetes_list_map_keys: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// x-kubernetes-list-type annotates an array to further describe its topology.
    /// This extension must only be used on lists and may have 3 possible values:
    ///
    /// 1) `atomic`: the list is treated as a single entity, like a scalar.
    ///       Atomic lists will be entirely replaced when updated. This extension
    ///       may be used on any type of list (struct, scalar, ...).
    /// 2) `set`:
    ///       Sets are lists that must not have multiple items with the same value. Each
    ///       value must be a scalar, an object with x-kubernetes-map-type `atomic` or an
    ///       array with x-kubernetes-list-type `atomic`.
    /// 3) `map`:
    ///       These lists are like maps in that their elements have a non-index key
    ///       used to identify them. Order is preserved upon merge. The map tag
    ///       must only be used on a list with elements of type object.
    /// Defaults to atomic for arrays.
    /// +optional
    #[prost(string, optional, tag = "42")]
    pub x_kubernetes_list_type: ::core::option::Option<::prost::alloc::string::String>,
    /// x-kubernetes-map-type annotates an object to further describe its topology.
    /// This extension must only be used when type is object and may have 2 possible values:
    ///
    /// 1) `granular`:
    ///       These maps are actual maps (key-value pairs) and each fields are independent
    ///       from each other (they can each be manipulated by separate actors). This is
    ///       the default behaviour for all maps.
    /// 2) `atomic`: the list is treated as a single entity, like a scalar.
    ///       Atomic maps will be entirely replaced when updated.
    /// +optional
    #[prost(string, optional, tag = "43")]
    pub x_kubernetes_map_type: ::core::option::Option<::prost::alloc::string::String>,
    /// x-kubernetes-validations describes a list of validation rules written in the CEL expression language.
    /// This field is an alpha-level. Using this field requires the feature gate `CustomResourceValidationExpressions` to be enabled.
    /// +patchMergeKey=rule
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=rule
    #[prost(message, repeated, tag = "44")]
    pub x_kubernetes_validations: ::prost::alloc::vec::Vec<ValidationRule>,
}
/// JSONSchemaPropsOrArray represents a value that can either be a JSONSchemaProps
/// or an array of JSONSchemaProps. Mainly here for serialization purposes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonSchemaPropsOrArray {
    #[prost(message, optional, boxed, tag = "1")]
    pub schema: ::core::option::Option<::prost::alloc::boxed::Box<JsonSchemaProps>>,
    #[prost(message, repeated, tag = "2")]
    pub j_son_schemas: ::prost::alloc::vec::Vec<JsonSchemaProps>,
}
/// JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value.
/// Defaults to true for the boolean property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonSchemaPropsOrBool {
    #[prost(bool, optional, tag = "1")]
    pub allows: ::core::option::Option<bool>,
    #[prost(message, optional, boxed, tag = "2")]
    pub schema: ::core::option::Option<::prost::alloc::boxed::Box<JsonSchemaProps>>,
}
/// JSONSchemaPropsOrStringArray represents a JSONSchemaProps or a string array.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonSchemaPropsOrStringArray {
    #[prost(message, optional, tag = "1")]
    pub schema: ::core::option::Option<JsonSchemaProps>,
    #[prost(string, repeated, tag = "2")]
    pub property: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ServiceReference holds a reference to Service.legacy.k8s.io
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceReference {
    /// namespace is the namespace of the service.
    /// Required
    #[prost(string, optional, tag = "1")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// name is the name of the service.
    /// Required
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// path is an optional URL path at which the webhook will be contacted.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// port is an optional service port at which the webhook will be contacted.
    /// `port` should be a valid port number (1-65535, inclusive).
    /// Defaults to 443 for backward compatibility.
    /// +optional
    #[prost(int32, optional, tag = "4")]
    pub port: ::core::option::Option<i32>,
}
/// ValidationRule describes a validation rule written in the CEL expression language.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationRule {
    /// Rule represents the expression which will be evaluated by CEL.
    /// ref: <https://github.com/google/cel-spec>
    /// The Rule is scoped to the location of the x-kubernetes-validations extension in the schema.
    /// The `self` variable in the CEL expression is bound to the scoped value.
    /// Example:
    /// - Rule scoped to the root of a resource with a status subresource: {"rule": "self.status.actual <= self.spec.maxDesired"}
    ///
    /// If the Rule is scoped to an object with properties, the accessible properties of the object are field selectable
    /// via `self.field` and field presence can be checked via `has(self.field)`. Null valued fields are treated as
    /// absent fields in CEL expressions.
    /// If the Rule is scoped to an object with additionalProperties (i.e. a map) the value of the map
    /// are accessible via `self\[mapKey\]`, map containment can be checked via `mapKey in self` and all entries of the map
    /// are accessible via CEL macros and functions such as `self.all(...)`.
    /// If the Rule is scoped to an array, the elements of the array are accessible via `self\[i\]` and also by macros and
    /// functions.
    /// If the Rule is scoped to a scalar, `self` is bound to the scalar value.
    /// Examples:
    /// - Rule scoped to a map of objects: {"rule": "self.components\['Widget'\].priority < 10"}
    /// - Rule scoped to a list of integers: {"rule": "self.values.all(value, value >= 0 && value < 100)"}
    /// - Rule scoped to a string value: {"rule": "self.startsWith('kube')"}
    ///
    /// The `apiVersion`, `kind`, `metadata.name` and `metadata.generateName` are always accessible from the root of the
    /// object and from any x-kubernetes-embedded-resource annotated objects. No other metadata properties are accessible.
    ///
    /// Unknown data preserved in custom resources via x-kubernetes-preserve-unknown-fields is not accessible in CEL
    /// expressions. This includes:
    /// - Unknown field values that are preserved by object schemas with x-kubernetes-preserve-unknown-fields.
    /// - Object properties where the property schema is of an "unknown type". An "unknown type" is recursively defined as:
    ///    - A schema with no type and x-kubernetes-preserve-unknown-fields set to true
    ///    - An array where the items schema is of an "unknown type"
    ///    - An object where the additionalProperties schema is of an "unknown type"
    ///
    /// Only property names of the form `[a-zA-Z_.-/][a-zA-Z0-9_.-/]*` are accessible.
    /// Accessible property names are escaped according to the following rules when accessed in the expression:
    /// - '__' escapes to '__underscores__'
    /// - '.' escapes to '__dot__'
    /// - '-' escapes to '__dash__'
    /// - '/' escapes to '__slash__'
    /// - Property names that exactly match a CEL RESERVED keyword escape to '__{keyword}__'. The keywords are:
    /// 	  "true", "false", "null", "in", "as", "break", "const", "continue", "else", "for", "function", "if",
    /// 	  "import", "let", "loop", "package", "namespace", "return".
    /// Examples:
    ///    - Rule accessing a property named "namespace": {"rule": "self.__namespace__ > 0"}
    ///    - Rule accessing a property named "x-prop": {"rule": "self.x__dash__prop > 0"}
    ///    - Rule accessing a property named "redact__d": {"rule": "self.redact__underscores__d > 0"}
    ///
    /// Equality on arrays with x-kubernetes-list-type of 'set' or 'map' ignores element order, i.e. \[1, 2\] == \[2, 1\].
    /// Concatenation on arrays with x-kubernetes-list-type use the semantics of the list type:
    ///    - 'set': `X + Y` performs a union where the array positions of all elements in `X` are preserved and
    ///      non-intersecting elements in `Y` are appended, retaining their partial order.
    ///    - 'map': `X + Y` performs a merge where the array positions of all keys in `X` are preserved but the values
    ///      are overwritten by values in `Y` when the key sets of `X` and `Y` intersect. Elements in `Y` with
    ///      non-intersecting keys are appended, retaining their partial order.
    #[prost(string, optional, tag = "1")]
    pub rule: ::core::option::Option<::prost::alloc::string::String>,
    /// Message represents the message displayed when validation fails. The message is required if the Rule contains
    /// line breaks. The message must not contain line breaks.
    /// If unset, the message is "failed rule: {Rule}".
    /// e.g. "must be a URL with the host matching spec.host"
    #[prost(string, optional, tag = "2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// MessageExpression declares a CEL expression that evaluates to the validation failure message that is returned when this rule fails.
    /// Since messageExpression is used as a failure message, it must evaluate to a string.
    /// If both message and messageExpression are present on a rule, then messageExpression will be used if validation
    /// fails. If messageExpression results in a runtime error, the runtime error is logged, and the validation failure message is produced
    /// as if the messageExpression field were unset. If messageExpression evaluates to an empty string, a string with only spaces, or a string
    /// that contains line breaks, then the validation failure message will also be produced as if the messageExpression field were unset, and
    /// the fact that messageExpression produced an empty string/string with only spaces/string with line breaks will be logged.
    /// messageExpression has access to all the same variables as the rule; the only difference is the return type.
    /// Example:
    /// "x must be less than max ("+string(self.max)+")"
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub message_expression: ::core::option::Option<::prost::alloc::string::String>,
    /// reason provides a machine-readable validation failure reason that is returned to the caller when a request fails this validation rule.
    /// The HTTP status code returned to the caller will match the reason of the reason of the first failed validation rule.
    /// The currently supported reasons are: "FieldValueInvalid", "FieldValueForbidden", "FieldValueRequired", "FieldValueDuplicate".
    /// If not set, default to use "FieldValueInvalid".
    /// All future added reasons must be accepted by clients when reading this value and unknown reasons should be treated as FieldValueInvalid.
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// fieldPath represents the field path returned when the validation fails.
    /// It must be a relative JSON path (i.e. with array notation) scoped to the location of this x-kubernetes-validations extension in the schema and refer to an existing field.
    /// e.g. when validation checks if a specific attribute `foo` under a map `testMap`, the fieldPath could be set to `.testMap.foo`
    /// If the validation checks two lists must have unique attributes, the fieldPath could be set to either of the list: e.g. `.testList`
    /// It does not support list numeric index.
    /// It supports child operation to refer to an existing field currently. Refer to [JSONPath support in Kubernetes](<https://kubernetes.io/docs/reference/kubectl/jsonpath/>) for more info.
    /// Numeric index of array is not supported.
    /// For field name which contains special characters, use `\['specialName'\]` to refer the field name.
    /// e.g. for attribute `foo.34$` appears in a list `testList`, the fieldPath could be set to `.testList\['foo.34$'\]`
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub field_path: ::core::option::Option<::prost::alloc::string::String>,
}
/// WebhookClientConfig contains the information to make a TLS connection with the webhook.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookClientConfig {
    /// url gives the location of the webhook, in standard URL form
    /// (`scheme://host:port/path`). Exactly one of `url` or `service`
    /// must be specified.
    ///
    /// The `host` should not refer to a service running in the cluster; use
    /// the `service` field instead. The host might be resolved via external
    /// DNS in some apiservers (e.g., `kube-apiserver` cannot resolve
    /// in-cluster DNS as that would be a layering violation). `host` may
    /// also be an IP address.
    ///
    /// Please note that using `localhost` or `127.0.0.1` as a `host` is
    /// risky unless you take great care to run this webhook on all hosts
    /// which run an apiserver which might need to make calls to this
    /// webhook. Such installs are likely to be non-portable, i.e., not easy
    /// to turn up in a new cluster.
    ///
    /// The scheme must be "https"; the URL must begin with "<https://".>
    ///
    /// A path is optional, and if present may be any string permissible in
    /// a URL. You may use the path to pass an arbitrary string to the
    /// webhook, for example, a cluster identifier.
    ///
    /// Attempting to use a user or basic auth e.g. "user:password@" is not
    /// allowed. Fragments ("#...") and query parameters ("?...") are not
    /// allowed, either.
    ///
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// service is a reference to the service for this webhook. Either
    /// service or url must be specified.
    ///
    /// If the webhook is running within the cluster, then you should use `service`.
    ///
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub service: ::core::option::Option<ServiceReference>,
    /// caBundle is a PEM encoded CA bundle which will be used to validate the webhook's server certificate.
    /// If unspecified, system trust roots on the apiserver are used.
    /// +optional
    #[prost(bytes = "vec", optional, tag = "2")]
    pub ca_bundle: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
