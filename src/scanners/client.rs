use kube::{Api, Resource};
use kube::core::{ClusterResourceScope, NamespaceResourceScope};

/// Helper trait for getting [`kube::Api`] instances for a Kubernetes resource's scope
///
/// Not intended to be implemented manually, it is blanket-implemented for all types that implement [`Resource`]
/// for either the [namespace](`NamespaceResourceScope`) or [cluster](`ClusterResourceScope`) scopes.
pub trait GetApi: Resource + Sized {
    /// The namespace type for `Self`'s scope.
    ///
    /// This will be [`str`] for namespaced resource, and [`()`] for cluster-scoped resources.
    type Namespace: ?Sized;
    /// Get a [`kube::Api`] for `Self`'s native scope..
    fn get_api(client: kube::Client, ns: &Self::Namespace) -> kube::Api<Self>
    where
        Self::DynamicType: Default;
    /// Get the namespace of `Self`.
    fn get_namespace(&self) -> &Self::Namespace;
}

impl<K> GetApi for K
where
    K: Resource,
    (K, K::Scope): GetApiImpl<Resource = K>,
{
    type Namespace = <(K, K::Scope) as GetApiImpl>::Namespace;
    fn get_api(client: kube::Client, ns: &Self::Namespace) -> kube::Api<Self>
    where
        Self::DynamicType: Default,
    {
        <(K, K::Scope) as GetApiImpl>::get_api(client, ns)
    }
    fn get_namespace(&self) -> &Self::Namespace {
        <(K, K::Scope) as GetApiImpl>::get_namespace(self)
    }
}

#[doc(hidden)]
// Workaround for https://github.com/rust-lang/rust/issues/20400
pub trait GetApiImpl {
    type Resource: Resource;
    type Namespace: ?Sized;
    fn get_api(client: kube::Client, ns: &Self::Namespace) -> kube::Api<Self::Resource>
    where
        <Self::Resource as Resource>::DynamicType: Default;
    fn get_namespace(res: &Self::Resource) -> &Self::Namespace;
}

impl<K> GetApiImpl for (K, NamespaceResourceScope)
where
    K: Resource<Scope = NamespaceResourceScope>,
{
    type Resource = K;
    type Namespace = str;
    fn get_api(client: kube::Client, ns: &Self::Namespace) -> kube::Api<K>
    where
        <Self::Resource as Resource>::DynamicType: Default,
    {
        Api::namespaced(client, ns)
    }
    fn get_namespace(res: &Self::Resource) -> &Self::Namespace {
        res.meta().namespace.as_deref().unwrap_or_default()
    }
}

impl<K> GetApiImpl for (K, ClusterResourceScope)
where
    K: Resource<Scope = ClusterResourceScope>,
{
    type Resource = K;
    type Namespace = ();
    fn get_api(client: kube::Client, (): &Self::Namespace) -> kube::Api<K>
    where
        <Self::Resource as Resource>::DynamicType: Default,
    {
        Api::all(client)
    }
    fn get_namespace(_res: &Self::Resource) -> &Self::Namespace {
        &()
    }
}