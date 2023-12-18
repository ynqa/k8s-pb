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
/// SelfSubjectReview contains the user information that the kube-apiserver has about the user making this request.
/// When using impersonation, users will receive the user info of the user being impersonated.  If impersonation or
/// request header authentication is used, any extra keys will have their case ignored and returned as lowercase.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfSubjectReview {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Status is filled in by the server with the user attributes.
    #[prost(message, optional, tag="2")]
    pub status: ::core::option::Option<SelfSubjectReviewStatus>,
}
/// SelfSubjectReviewStatus is filled by the kube-apiserver and sent back to a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfSubjectReviewStatus {
    /// User attributes of the user making this request.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub user_info: ::core::option::Option<super::v1::UserInfo>,
}
/// TokenReview attempts to authenticate a token to a known user.
/// Note: TokenReview requests may be cached by the webhook token authenticator
/// plugin in the kube-apiserver.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenReview {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec holds information about the request being evaluated
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<TokenReviewSpec>,
    /// Status is filled in by the server and indicates whether the token can be authenticated.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<TokenReviewStatus>,
}
/// TokenReviewSpec is a description of the token authentication request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenReviewSpec {
    /// Token is the opaque bearer token.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub token: ::core::option::Option<::prost::alloc::string::String>,
    /// Audiences is a list of the identifiers that the resource server presented
    /// with the token identifies as. Audience-aware token authenticators will
    /// verify that the token was intended for at least one of the audiences in
    /// this list. If no audiences are provided, the audience will default to the
    /// audience of the Kubernetes apiserver.
    /// +optional
    #[prost(string, repeated, tag="2")]
    pub audiences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// TokenReviewStatus is the result of the token authentication request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenReviewStatus {
    /// Authenticated indicates that the token was associated with a known user.
    /// +optional
    #[prost(bool, optional, tag="1")]
    pub authenticated: ::core::option::Option<bool>,
    /// User is the UserInfo associated with the provided token.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub user: ::core::option::Option<UserInfo>,
    /// Audiences are audience identifiers chosen by the authenticator that are
    /// compatible with both the TokenReview and token. An identifier is any
    /// identifier in the intersection of the TokenReviewSpec audiences and the
    /// token's audiences. A client of the TokenReview API that sets the
    /// spec.audiences field should validate that a compatible audience identifier
    /// is returned in the status.audiences field to ensure that the TokenReview
    /// server is audience aware. If a TokenReview returns an empty
    /// status.audience field where status.authenticated is "true", the token is
    /// valid against the audience of the Kubernetes API server.
    /// +optional
    #[prost(string, repeated, tag="4")]
    pub audiences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Error indicates that the token couldn't be checked
    /// +optional
    #[prost(string, optional, tag="3")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// UserInfo holds the information about the user needed to implement the
/// user.Info interface.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// The name that uniquely identifies this user among all active users.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub username: ::core::option::Option<::prost::alloc::string::String>,
    /// A unique value that identifies this user across time. If this user is
    /// deleted and another user by the same name is added, they will have
    /// different UIDs.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    /// The names of groups this user is a part of.
    /// +optional
    #[prost(string, repeated, tag="3")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Any additional information provided by the authenticator.
    /// +optional
    #[prost(map="string, message", tag="4")]
    pub extra: ::std::collections::HashMap<::prost::alloc::string::String, ExtraValue>,
}

impl crate::Resource for SelfSubjectReview {
    const API_VERSION: &'static str = "authentication.k8s.io/v1beta1";
    const GROUP: &'static str = "authentication.k8s.io";
    const VERSION: &'static str = "v1beta1";
    const KIND: &'static str = "SelfSubjectReview";
    const NAME: &'static str = "selfsubjectreviews";
}
impl crate::HasMetadata for SelfSubjectReview {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasStatus for SelfSubjectReview {
    type Status = crate::api::authentication::v1beta1::SelfSubjectReviewStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}

