//! Model types for use in a UI context.

use std::rc::Rc;

use geo::Geometry;
use implicit_clone::{
    unsync::{IArray, IString},
    ImplicitClone,
};

pub type Link = (Option<IString>, IString);

#[derive(Clone, Debug, PartialEq)]
pub struct Activity {
    pub label: Option<IString>,
}

impl ImplicitClone for Activity {}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Entity {
    pub geometry: Option<Rc<Geometry>>,
    pub label: Option<IString>,
    pub properties: IArray<(IString, IString)>,
    pub was_derived_from: IArray<Link>,
    pub was_generated_by: IArray<Link>,
    pub was_influenced_by: IArray<Link>,
}

impl ImplicitClone for Entity {}
