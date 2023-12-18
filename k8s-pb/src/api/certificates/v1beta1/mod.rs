/// Describes a certificate signing request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSigningRequest {
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// spec contains the certificate request, and is immutable after creation.
    /// Only the request, signerName, expirationSeconds, and usages fields can be set on creation.
    /// Other fields are derived by Kubernetes and cannot be modified by users.
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<CertificateSigningRequestSpec>,
    /// Derived information about the request.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<CertificateSigningRequestStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSigningRequestCondition {
    /// type of the condition. Known conditions include "Approved", "Denied", and "Failed".
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    /// Approved, Denied, and Failed conditions may not be "False" or "Unknown".
    /// Defaults to "True".
    /// If unset, should be treated as "True".
    /// +optional
    #[prost(string, optional, tag="6")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// brief reason for the request state
    /// +optional
    #[prost(string, optional, tag="2")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// human readable message with details about the request state
    /// +optional
    #[prost(string, optional, tag="3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// timestamp for the last update to this condition
    /// +optional
    #[prost(message, optional, tag="4")]
    pub last_update_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// lastTransitionTime is the time the condition last transitioned from one status to another.
    /// If unset, when a new condition type is added or an existing condition's status is changed,
    /// the server defaults this to the current time.
    /// +optional
    #[prost(message, optional, tag="5")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSigningRequestList {
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<CertificateSigningRequest>,
}
/// CertificateSigningRequestSpec contains the certificate request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSigningRequestSpec {
    /// Base64-encoded PKCS#10 CSR data
    /// +listType=atomic
    #[prost(bytes="vec", optional, tag="1")]
    pub request: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// Requested signer for the request. It is a qualified name in the form:
    /// `scope-hostname.io/name`.
    /// If empty, it will be defaulted:
    ///  1. If it's a kubelet client certificate, it is assigned
    ///     "kubernetes.io/kube-apiserver-client-kubelet".
    ///  2. If it's a kubelet serving certificate, it is assigned
    ///     "kubernetes.io/kubelet-serving".
    ///  3. Otherwise, it is assigned "kubernetes.io/legacy-unknown".
    /// Distribution of trust for signers happens out of band.
    /// You can select on this field using `spec.signerName`.
    /// +optional
    #[prost(string, optional, tag="7")]
    pub signer_name: ::core::option::Option<::prost::alloc::string::String>,
    /// expirationSeconds is the requested duration of validity of the issued
    /// certificate. The certificate signer may issue a certificate with a different
    /// validity duration so a client must check the delta between the notBefore and
    /// and notAfter fields in the issued certificate to determine the actual duration.
    ///
    /// The v1.22+ in-tree implementations of the well-known Kubernetes signers will
    /// honor this field as long as the requested duration is not greater than the
    /// maximum duration they will honor per the --cluster-signing-duration CLI
    /// flag to the Kubernetes controller manager.
    ///
    /// Certificate signers may not honor this field for various reasons:
    ///
    ///   1. Old signer that is unaware of the field (such as the in-tree
    ///      implementations prior to v1.22)
    ///   2. Signer whose configured maximum is shorter than the requested duration
    ///   3. Signer whose configured minimum is longer than the requested duration
    ///
    /// The minimum valid value for expirationSeconds is 600, i.e. 10 minutes.
    ///
    /// +optional
    #[prost(int32, optional, tag="8")]
    pub expiration_seconds: ::core::option::Option<i32>,
    /// allowedUsages specifies a set of usage contexts the key will be
    /// valid for.
    /// See:
    /// 	https://tools.ietf.org/html/rfc5280#section-4.2.1.3
    /// 	https://tools.ietf.org/html/rfc5280#section-4.2.1.12
    ///
    /// Valid values are:
    ///  "signing",
    ///  "digital signature",
    ///  "content commitment",
    ///  "key encipherment",
    ///  "key agreement",
    ///  "data encipherment",
    ///  "cert sign",
    ///  "crl sign",
    ///  "encipher only",
    ///  "decipher only",
    ///  "any",
    ///  "server auth",
    ///  "client auth",
    ///  "code signing",
    ///  "email protection",
    ///  "s/mime",
    ///  "ipsec end system",
    ///  "ipsec tunnel",
    ///  "ipsec user",
    ///  "timestamping",
    ///  "ocsp signing",
    ///  "microsoft sgc",
    ///  "netscape sgc"
    /// +listType=atomic
    #[prost(string, repeated, tag="5")]
    pub usages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Information about the requesting user.
    /// See user.Info interface for details.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    /// UID information about the requesting user.
    /// See user.Info interface for details.
    /// +optional
    #[prost(string, optional, tag="3")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// Group information about the requesting user.
    /// See user.Info interface for details.
    /// +listType=atomic
    /// +optional
    #[prost(string, repeated, tag="4")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Extra information about the requesting user.
    /// See user.Info interface for details.
    /// +optional
    #[prost(map="string, message", tag="6")]
    pub extra: ::std::collections::HashMap<::prost::alloc::string::String, ExtraValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSigningRequestStatus {
    /// Conditions applied to the request, such as approval or denial.
    /// +listType=map
    /// +listMapKey=type
    /// +optional
    #[prost(message, repeated, tag="1")]
    pub conditions: ::prost::alloc::vec::Vec<CertificateSigningRequestCondition>,
    /// If request was approved, the controller will place the issued certificate here.
    /// +listType=atomic
    /// +optional
    #[prost(bytes="vec", optional, tag="2")]
    pub certificate: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// ExtraValue masks the value so protobuf can generate
/// +protobuf.nullable=true
/// +protobuf.options.(gogoproto.goproto_stringer)=false
///
/// items, if empty, will result in an empty slice
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraValue {
    #[prost(string, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
