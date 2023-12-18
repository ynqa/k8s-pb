/// ClusterTrustBundle is a cluster-scoped container for X.509 trust anchors
/// (root certificates).
///
/// ClusterTrustBundle objects are considered to be readable by any authenticated
/// user in the cluster, because they can be mounted by pods using the
/// `clusterTrustBundle` projection.  All service accounts have read access to
/// ClusterTrustBundles by default.  Users who only have namespace-level access
/// to a cluster can read ClusterTrustBundles by impersonating a serviceaccount
/// that they have access to.
///
/// It can be optionally associated with a particular assigner, in which case it
/// contains one valid set of trust anchors for that signer. Signers may have
/// multiple associated ClusterTrustBundles; each is an independent set of trust
/// anchors for that signer. Admission control is used to enforce that only users
/// with permissions on the signer can create or modify the corresponding bundle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterTrustBundle {
    /// metadata contains the object metadata.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// spec contains the signer (if any) and trust anchors.
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<ClusterTrustBundleSpec>,
}
/// ClusterTrustBundleList is a collection of ClusterTrustBundle objects
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterTrustBundleList {
    /// metadata contains the list metadata.
    ///
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// items is a collection of ClusterTrustBundle objects
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ClusterTrustBundle>,
}
/// ClusterTrustBundleSpec contains the signer and trust anchors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterTrustBundleSpec {
    /// signerName indicates the associated signer, if any.
    ///
    /// In order to create or update a ClusterTrustBundle that sets signerName,
    /// you must have the following cluster-scoped permission:
    /// group=certificates.k8s.io resource=signers resourceName=<the signer name>
    /// verb=attest.
    ///
    /// If signerName is not empty, then the ClusterTrustBundle object must be
    /// named with the signer name as a prefix (translating slashes to colons).
    /// For example, for the signer name `example.com/foo`, valid
    /// ClusterTrustBundle object names include `example.com:foo:abc` and
    /// `example.com:foo:v1`.
    ///
    /// If signerName is empty, then the ClusterTrustBundle object's name must
    /// not have such a prefix.
    ///
    /// List/watch requests for ClusterTrustBundles can filter on this field
    /// using a `spec.signerName=NAME` field selector.
    ///
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub signer_name: ::core::option::Option<::prost::alloc::string::String>,
    /// trustBundle contains the individual X.509 trust anchors for this
    /// bundle, as PEM bundle of PEM-wrapped, DER-formatted X.509 certificates.
    ///
    /// The data must consist only of PEM certificate blocks that parse as valid
    /// X.509 certificates.  Each certificate must include a basic constraints
    /// extension with the CA bit set.  The API server will reject objects that
    /// contain duplicate certificates, or that use PEM block headers.
    ///
    /// Users of ClusterTrustBundles, including Kubelet, are free to reorder and
    /// deduplicate certificate blocks in this file according to their own logic,
    /// as well as to drop PEM block headers and inter-block data.
    #[prost(string, optional, tag = "2")]
    pub trust_bundle: ::core::option::Option<::prost::alloc::string::String>,
}

impl crate::Resource for ClusterTrustBundle {
    const API_VERSION: &'static str = "certificates.k8s.io/v1alpha1";
    const GROUP: &'static str = "certificates.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "ClusterTrustBundle";
    const NAME: &'static str = "clustertrustbundles";
}
impl crate::HasMetadata for ClusterTrustBundle {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for ClusterTrustBundle {
    type Spec = crate::api::certificates::v1alpha1::ClusterTrustBundleSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}

