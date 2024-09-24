//! Model types for use in a UI context.

use std::rc::Rc;

use chrono::{DateTime, Utc};
use geo::Geometry;
use implicit_clone::{
    unsync::{IArray, IString},
    ImplicitClone,
};

pub type ActivityLink = (Option<IString>, IString);
pub type AgentLink = (Option<IString>, IString);
pub type EntityLink = (Option<IString>, IString);

#[derive(Clone, Debug, PartialEq)]
pub struct Activity {
    pub ended_at: Option<DateTime<Utc>>,
    pub generated: IArray<EntityLink>,
    pub influenced: IArray<ActivityLink>,
    pub label: Option<IString>,
    pub properties: IArray<(IString, IString)>,
    pub started_at: Option<DateTime<Utc>>,
    pub used: IArray<EntityLink>,
    pub was_associated_with: IArray<AgentLink>,
    pub was_influenced_by: IArray<ActivityLink>,
}

impl ImplicitClone for Activity {}

#[derive(Clone, Debug, PartialEq)]
pub struct Agent {
    pub influenced: IArray<ActivityLink>,
    pub label: Option<IString>,
    pub properties: IArray<(IString, IString)>,
}

impl ImplicitClone for Agent {}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Entity {
    pub geometry: Option<Rc<Geometry>>,
    pub label: Option<IString>,
    pub properties: IArray<(IString, IString)>,
    pub was_attributed_to: IArray<AgentLink>,
    pub was_derived_from: IArray<EntityLink>,
    pub was_generated_by: IArray<ActivityLink>,
}

impl ImplicitClone for Entity {}

#[derive(Clone, Debug, PartialEq)]
pub enum Provenance {
    Activity(Activity),
    Agent(Agent),
    Entity(Entity),
}

impl ImplicitClone for Provenance {}
