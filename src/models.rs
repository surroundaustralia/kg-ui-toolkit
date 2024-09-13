//! Model types for use in a UI context.

use std::rc::Rc;

use geo::Geometry;
use implicit_clone::unsync::{IArray, IString};

pub type Link = (Option<IString>, IString);

#[derive(Debug, PartialEq)]
pub struct Activity {
    pub label: Option<IString>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialEntity {
    pub geometry: Rc<Geometry>,
    pub label: Option<IString>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub label: Option<IString>,
    pub value: Option<IString>,
    pub value_label: Option<IString>,
    pub was_derived_from: IArray<Link>,
    pub was_generated_by: IArray<Link>,
    pub was_influenced_by: IArray<Link>,
}
