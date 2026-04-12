use std::{collections::BTreeMap, marker::PhantomData};

use kube::core::GroupVersionKind;
use rmcp::schemars;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{gather::selector::Expressions, scanners::interface::ResourceThreadSafe};

use super::filter::{Filter, FilterType};

#[derive(Clone, Default, Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(try_from = "String")]
pub struct SelectorInclude<G>
where
    G: SelectorSource + Send + Sync,
{
    selector: Expressions,
    group: PhantomData<G>,
}

impl<R: ResourceThreadSafe, G> Filter<R> for SelectorInclude<G>
where
    G: SelectorSource + Send + Sync,
{
    #[instrument(skip_all, fields(name = obj.name_any(), include = self.selector.to_string()))]
    fn filter_object(&self, obj: &R, _: &GroupVersionKind) -> Option<bool> {
        Some(self.selector.matches(G::select(obj)))
    }
}

impl<G> TryFrom<String> for SelectorInclude<G>
where
    G: SelectorSource + Send + Sync,
{
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            selector: value.try_into()?,
            group: PhantomData,
        })
    }
}

impl From<Vec<SelectorInclude<LabelGroup>>> for FilterType {
    fn from(val: Vec<SelectorInclude<LabelGroup>>) -> Self {
        Self::LabelSelectorInclude(val)
    }
}

impl From<Vec<SelectorInclude<AnnotationGroup>>> for FilterType {
    fn from(val: Vec<SelectorInclude<AnnotationGroup>>) -> Self {
        Self::AnnotationSelectorInclude(val)
    }
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, schemars::JsonSchema)]
#[serde(try_from = "String")]
pub struct SelectorExclude<G>
where
    G: SelectorSource + Send + Sync,
{
    selector: Expressions,
    group: PhantomData<G>,
}

impl<R: ResourceThreadSafe, G> Filter<R> for SelectorExclude<G>
where
    G: SelectorSource + Send + Sync,
{
    #[instrument(skip_all, fields(name = obj.name_any(), exclude = self.selector.to_string()))]
    fn filter_object(&self, obj: &R, _: &GroupVersionKind) -> Option<bool> {
        Some(!self.selector.matches(G::select(obj)))
    }
}

impl<G> TryFrom<String> for SelectorExclude<G>
where
    G: SelectorSource + Send + Sync,
{
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self {
            selector: value.try_into()?,
            group: PhantomData,
        })
    }
}

impl From<Vec<SelectorExclude<LabelGroup>>> for FilterType {
    fn from(val: Vec<SelectorExclude<LabelGroup>>) -> Self {
        Self::LabelSelectorExclude(val)
    }
}

impl From<Vec<SelectorExclude<AnnotationGroup>>> for FilterType {
    fn from(val: Vec<SelectorExclude<AnnotationGroup>>) -> Self {
        Self::AnnotationSelectorExclude(val)
    }
}

#[derive(Clone, PartialEq, Default, Serialize, Deserialize, Debug, schemars::JsonSchema)]
pub enum SelectorGroup {
    #[default]
    Labels,
    Annotations,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, schemars::JsonSchema)]
pub struct LabelGroup;

#[derive(Clone, Default, Serialize, Deserialize, Debug, schemars::JsonSchema)]
pub struct AnnotationGroup;

pub trait SelectorSource {
    fn select<R: ResourceThreadSafe>(obj: &R) -> &BTreeMap<String, String>;
}

impl SelectorSource for LabelGroup {
    fn select<R: ResourceThreadSafe>(obj: &R) -> &BTreeMap<String, String> {
        obj.labels()
    }
}

impl SelectorSource for AnnotationGroup {
    fn select<R: ResourceThreadSafe>(obj: &R) -> &BTreeMap<String, String> {
        obj.annotations()
    }
}
