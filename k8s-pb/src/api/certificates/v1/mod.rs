/// CertificateSigningRequest objects provide a mechanism to obtain x509 certificates
/// by submitting a certificate signing request, and having it asynchronously approved and issued.
///
/// Kubelets use this API to obtain:
///  1. client certificates to authenticate to kube-apiserver (with the "kubernetes.io/kube-apiserver-client-kubelet" signerName).
///  2. serving certificates for TLS endpoints kube-apiserver can connect to securely (with the "kubernetes.io/kubelet-serving" signerName).
///
/// This API can be used to request client certificates to authenticate to kube-apiserver
/// (with the "kubernetes.io/kube-apiserver-client" signerName),
/// or to obtain certificates from custom non-Kubernetes signers.
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
    /// status contains information about whether the request is approved or denied,
    /// and the certificate issued by the signer, or the failure condition indicating signer failure.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<CertificateSigningRequestStatus>,
}
/// CertificateSigningRequestCondition describes a condition of a CertificateSigningRequest object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSigningRequestCondition {
    /// type of the condition. Known conditions are "Approved", "Denied", and "Failed".
    ///
    /// An "Approved" condition is added via the /approval subresource,
    /// indicating the request was approved and should be issued by the signer.
    ///
    /// A "Denied" condition is added via the /approval subresource,
    /// indicating the request was denied and should not be issued by the signer.
    ///
    /// A "Failed" condition is added via the /status subresource,
    /// indicating the signer failed to issue the certificate.
    ///
    /// Approved and Denied conditions are mutually exclusive.
    /// Approved, Denied, and Failed conditions cannot be removed once added.
    ///
    /// Only one condition of a given type is allowed.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// status of the condition, one of True, False, Unknown.
    /// Approved, Denied, and Failed conditions may not be "False" or "Unknown".
    #[prost(string, optional, tag="6")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// reason indicates a brief reason for the request state
    /// +optional
    #[prost(string, optional, tag="2")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// message contains a human readable message with details about the request state
    /// +optional
    #[prost(string, optional, tag="3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// lastUpdateTime is the time of the last update to this condition
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
/// CertificateSigningRequestList is a collection of CertificateSigningRequest objects
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSigningRequestList {
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is a collection of CertificateSigningRequest objects
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<CertificateSigningRequest>,
}
/// CertificateSigningRequestSpec contains the certificate request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSigningRequestSpec {
    /// request contains an x509 certificate signing request encoded in a "CERTIFICATE REQUEST" PEM block.
    /// When serialized as JSON or YAML, the data is additionally base64-encoded.
    /// +listType=atomic
    #[prost(bytes="vec", optional, tag="1")]
    pub request: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// signerName indicates the requested signer, and is a qualified name.
    ///
    /// List/watch requests for CertificateSigningRequests can filter on this field using a "spec.signerName=NAME" fieldSelector.
    ///
    /// Well-known Kubernetes signers are:
    ///  1. "kubernetes.io/kube-apiserver-client": issues client certificates that can be used to authenticate to kube-apiserver.
    ///   Requests for this signer are never auto-approved by kube-controller-manager, can be issued by the "csrsigning" controller in kube-controller-manager.
    ///  2. "kubernetes.io/kube-apiserver-client-kubelet": issues client certificates that kubelets use to authenticate to kube-apiserver.
    ///   Requests for this signer can be auto-approved by the "csrapproving" controller in kube-controller-manager, and can be issued by the "csrsigning" controller in kube-controller-manager.
    ///  3. "kubernetes.io/kubelet-serving" issues serving certificates that kubelets use to serve TLS endpoints, which kube-apiserver can connect to securely.
    ///   Requests for this signer are never auto-approved by kube-controller-manager, and can be issued by the "csrsigning" controller in kube-controller-manager.
    ///
    /// More details are available at https://k8s.io/docs/reference/access-authn-authz/certificate-signing-requests/#kubernetes-signers
    ///
    /// Custom signerNames can also be specified. The signer defines:
    ///  1. Trust distribution: how trust (CA bundles) are distributed.
    ///  2. Permitted subjects: and behavior when a disallowed subject is requested.
    ///  3. Required, permitted, or forbidden x509 extensions in the request (including whether subjectAltNames are allowed, which types, restrictions on allowed values) and behavior when a disallowed extension is requested.
    ///  4. Required, permitted, or forbidden key usages / extended key usages.
    ///  5. Expiration/certificate lifetime: whether it is fixed by the signer, configurable by the admin.
    ///  6. Whether or not requests for CA certificates are allowed.
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
    /// As of v1.22, this field is beta and is controlled via the CSRDuration feature gate.
    ///
    /// +optional
    #[prost(int32, optional, tag="8")]
    pub expiration_seconds: ::core::option::Option<i32>,
    /// usages specifies a set of key usages requested in the issued certificate.
    ///
    /// Requests for TLS client certificates typically request: "digital signature", "key encipherment", "client auth".
    ///
    /// Requests for TLS serving certificates typically request: "key encipherment", "digital signature", "server auth".
    ///
    /// Valid values are:
    ///  "signing", "digital signature", "content commitment",
    ///  "key encipherment", "key agreement", "data encipherment",
    ///  "cert sign", "crl sign", "encipher only", "decipher only", "any",
    ///  "server auth", "client auth",
    ///  "code signing", "email protection", "s/mime",
    ///  "ipsec end system", "ipsec tunnel", "ipsec user",
    ///  "timestamping", "ocsp signing", "microsoft sgc", "netscape sgc"
    /// +listType=atomic
    #[prost(string, repeated, tag="5")]
    pub usages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// username contains the name of the user that created the CertificateSigningRequest.
    /// Populated by the API server on creation and immutable.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    /// uid contains the uid of the user that created the CertificateSigningRequest.
    /// Populated by the API server on creation and immutable.
    /// +optional
    #[prost(string, optional, tag="3")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// groups contains group membership of the user that created the CertificateSigningRequest.
    /// Populated by the API server on creation and immutable.
    /// +listType=atomic
    /// +optional
    #[prost(string, repeated, tag="4")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// extra contains extra attributes of the user that created the CertificateSigningRequest.
    /// Populated by the API server on creation and immutable.
    /// +optional
    #[prost(map="string, message", tag="6")]
    pub extra: ::std::collections::HashMap<::prost::alloc::string::String, ExtraValue>,
}
/// CertificateSigningRequestStatus contains conditions used to indicate
/// approved/denied/failed status of the request, and the issued certificate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateSigningRequestStatus {
    /// conditions applied to the request. Known conditions are "Approved", "Denied", and "Failed".
    /// +listType=map
    /// +listMapKey=type
    /// +optional
    #[prost(message, repeated, tag="1")]
    pub conditions: ::prost::alloc::vec::Vec<CertificateSigningRequestCondition>,
    /// certificate is populated with an issued certificate by the signer after an Approved condition is present.
    /// This field is set via the /status subresource. Once populated, this field is immutable.
    ///
    /// If the certificate signing request is denied, a condition of type "Denied" is added and this field remains empty.
    /// If the signer cannot issue the certificate, a condition of type "Failed" is added and this field remains empty.
    ///
    /// Validation requirements:
    ///  1. certificate must contain one or more PEM blocks.
    ///  2. All PEM blocks must have the "CERTIFICATE" label, contain no headers, and the encoded data
    ///   must be a BER-encoded ASN.1 Certificate structure as described in section 4 of RFC5280.
    ///  3. Non-PEM content may appear before or after the "CERTIFICATE" PEM blocks and is unvalidated,
    ///   to allow for explanatory text as described in section 5.2 of RFC7468.
    ///
    /// If more than one PEM block is present, and the definition of the requested spec.signerName
    /// does not indicate otherwise, the first block is the issued certificate,
    /// and subsequent blocks should be treated as intermediate certificates and presented in TLS handshakes.
    ///
    /// The certificate is encoded in PEM format.
    ///
    /// When serialized as JSON or YAML, the data is additionally base64-encoded, so it consists of:
    ///
    ///     base64(
    ///     -----BEGIN CERTIFICATE-----
    ///     ...
    ///     -----END CERTIFICATE-----
    ///     )
    ///
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

impl crate::Resource for CertificateSigningRequest {
    const API_VERSION: &'static str = "certificates.k8s.io/v1";
    const GROUP: &'static str = "certificates.k8s.io";
    const VERSION: &'static str = "v1";
    const KIND: &'static str = "CertificateSigningRequest";
    const NAME: &'static str = "certificatesigningrequests";
}
impl crate::HasMetadata for CertificateSigningRequest {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for CertificateSigningRequest {
    type Spec = crate::api::certificates::v1::CertificateSigningRequestSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for CertificateSigningRequest {
    type Status = crate::api::certificates::v1::CertificateSigningRequestStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}
impl crate::HasConditions for CertificateSigningRequest {
    type Condition = crate::api::certificates::v1::CertificateSigningRequestCondition;
    fn conditions(&self) -> Option<&[<Self as crate::HasConditions>::Condition]> {
        self.status.as_ref().map(|s| s.conditions.as_slice())
    }
    fn conditions_mut(&mut self) -> Option<&mut Vec<<Self as crate::HasConditions>::Condition>> {
        self.status
            .as_mut()
            .and_then(|s| Some(s.conditions.as_mut()))
    }
}
