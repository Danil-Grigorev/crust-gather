use kube_core::{DynamicObject, GroupVersionKind};
use serde::Deserialize;

use super::filter::{Filter, FilterDefinition, FilterRegex, FilterType};

#[derive(Clone, Default, Deserialize)]
#[serde(try_from = "String")]
pub struct NamespaceInclude {
    namespace: FilterRegex,
}

/// Filters kubernetes namespaced resources based on whether their namespace is in the allowed
/// list. Returns true if the object's namespace is in the allowed namespaces,
/// or the namespace list is empty or the resource is cluster-scoped.
impl Filter for NamespaceInclude {
    fn filter_object(&self, obj: &DynamicObject) -> bool {
        let accepted = obj
            .metadata
            .namespace
            .clone()
            .unwrap_or_default()
            .is_empty()
            || self
                .namespace
                .matches(&obj.metadata.namespace.clone().unwrap_or_default());

        if !accepted {
            log::debug!(
                "NamespaceInclude filter excluded {}/{} as it is not present in the namespace list {}",
                obj.metadata.clone().name.unwrap(),
                obj.metadata.clone().namespace.unwrap(), self.namespace);
        }

        accepted
    }

    fn filter_api(&self, _: &GroupVersionKind) -> bool {
        true
    }
}

impl FilterDefinition for NamespaceInclude {}

impl TryFrom<String> for NamespaceInclude {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(NamespaceInclude {
            namespace: value.try_into()?,
        })
    }
}

impl Into<FilterType> for NamespaceInclude {
    fn into(self) -> FilterType {
        FilterType::NamespaceInclude(self)
    }
}

#[derive(Clone, Default, Deserialize)]
#[serde(try_from = "String")]
pub struct NamespaceExclude {
    namespace: FilterRegex,
}

impl FilterDefinition for NamespaceExclude {}

impl TryFrom<String> for NamespaceExclude {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(NamespaceExclude {
            namespace: value.try_into()?,
        })
    }
}

impl Into<FilterType> for NamespaceExclude {
    fn into(self) -> FilterType {
        FilterType::NamespaceExclude(self)
    }
}

impl Filter for NamespaceExclude {
    fn filter_object(&self, obj: &DynamicObject) -> bool {
        let accepted = obj
            .metadata
            .namespace
            .clone()
            .unwrap_or_default()
            .is_empty()
            || !self
                .namespace
                .matches(&obj.metadata.namespace.clone().unwrap_or_default());

        if !accepted {
            log::debug!(
                "NamespaceExclude filter excluded {}/{} as it is not present in the namespace list {}",
                obj.metadata.clone().name.unwrap(),
                obj.metadata.clone().namespace.unwrap(), self.namespace);
        }

        accepted
    }

    fn filter_api(&self, _: &GroupVersionKind) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {

    use k8s_openapi::api::core::v1::Pod;
    use kube_core::ApiResource;

    use super::*;

    #[test]
    fn test_include_namespace_filter() {
        let filter = NamespaceInclude::try_from("default".to_string()).unwrap();

        let obj = DynamicObject::new("test", &ApiResource::erase::<Pod>(&())).within("default");
        assert!(filter.filter_object(&obj));

        let filter = NamespaceInclude::try_from("default".to_string()).unwrap();

        let obj = DynamicObject::new("other", &ApiResource::erase::<Pod>(&())).within("other");
        assert!(!filter.filter_object(&obj));
    }

    #[test]
    fn test_exclude_namespace() {
        let filter = NamespaceExclude::try_from("default".to_string()).unwrap();

        let obj = DynamicObject::new("test", &ApiResource::erase::<Pod>(&())).within("default");
        assert!(!filter.filter_object(&obj));

        let filter = NamespaceExclude::try_from("default".to_string()).unwrap();

        let obj = DynamicObject::new("test", &ApiResource::erase::<Pod>(&())).within("other");
        assert!(filter.filter_object(&obj));
    }

    #[test]
    fn test_include_from_string() {
        let namespaces = NamespaceInclude::try_from("default".to_string()).unwrap();
        assert_eq!(namespaces.namespace.0.as_str(), "default");
    }

    #[test]
    fn test_exclude_from_string() {
        let namespaces = NamespaceExclude::try_from("default".to_string()).unwrap();
        assert_eq!(namespaces.namespace.0.as_str(), "default");
    }
}
