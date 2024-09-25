use chrono::{DateTime, Utc};
use gloo_net::http::Request;
use implicit_clone::unsync::IString;
use std::rc::Rc;

use geojson::GeoJson;

use serde::Deserialize;

use crate::models;

// Base types

#[derive(Debug, Deserialize, PartialEq)]
pub enum BindingType {
    #[serde(rename = "bnode")]
    Bnode,
    #[serde(rename = "literal")]
    Literal,
    #[serde(rename = "uri")]
    Uri,
}

#[derive(Debug, Deserialize)]
pub struct GeoJsonBinding {
    #[serde(rename = "type")]
    pub binding_type: BindingType,
    pub datatype: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ObjectPropertyBinding {
    #[serde(rename = "type")]
    pub binding_type: BindingType,
    pub value: String,
}

// High level types

#[derive(Debug, Deserialize)]
pub struct DimDescBinding {
    pub name: ObjectPropertyBinding,
    pub order: ObjectPropertyBinding,
    #[serde(rename = "dimRange")]
    pub range: ObjectPropertyBinding,
    #[serde(rename = "targetRange")]
    pub target_range: Option<ObjectPropertyBinding>,
}

#[derive(Debug, Deserialize)]
pub struct DimValueBinding {
    pub d: ObjectPropertyBinding,
    pub v: ObjectPropertyBinding,
}

#[derive(Debug, Deserialize)]
pub struct ObjectBinding {
    pub p: ObjectPropertyBinding,
    pub plabel: Option<ObjectPropertyBinding>,
    pub o: ObjectPropertyBinding,
    pub olabel: Option<ObjectPropertyBinding>,
}

#[derive(Debug, Deserialize)]
pub struct SpatialEntityBinding {
    pub entity: ObjectPropertyBinding,
    pub label: Option<ObjectPropertyBinding>,
    pub geojson: GeoJsonBinding,
}

#[derive(Debug, Deserialize)]
pub struct Results<B> {
    pub bindings: Vec<B>,
}

#[derive(Debug, Deserialize)]
pub struct Response<B> {
    pub results: Results<B>,
}

// Requests

pub async fn get_activity(
    api_path: &str,
    activity_id: &str,
) -> Result<Response<ObjectBinding>, gloo_net::Error> {
    let result = Request::get(&format!(
        "{api_path}/query?query=getActivity&$activity=<{activity_id}>"
    ))
    .header("Accept", "application/sparql-results+json")
    .send()
    .await;

    match result {
        Ok(r) => r.json().await,
        Err(e) => Err(e),
    }
}

pub async fn get_agent(
    api_path: &str,
    agent_id: &str,
) -> Result<Response<ObjectBinding>, gloo_net::Error> {
    let result = Request::get(&format!(
        "{api_path}/query?query=getAgent&$agent=<{agent_id}>"
    ))
    .header("Accept", "application/sparql-results+json")
    .send()
    .await;

    match result {
        Ok(r) => r.json().await,
        Err(e) => Err(e),
    }
}

pub async fn get_dim_desc(
    api_path: &str,
    entity_id: &str,
) -> Result<Response<DimDescBinding>, gloo_net::Error> {
    let result = Request::get(&format!(
        "{api_path}/query?query=getDimDesc&$object=<{entity_id}>"
    ))
    .header("Accept", "application/sparql-results+json")
    .send()
    .await;

    match result {
        Ok(r) => r.json().await,
        Err(e) => Err(e),
    }
}

pub async fn get_dim_values(
    api_path: &str,
    entity_id: &str,
) -> Result<Response<DimValueBinding>, gloo_net::Error> {
    let result = Request::get(&format!(
        "{api_path}/query?query=getDimValues&$object=<{entity_id}>"
    ))
    .header("Accept", "application/sparql-results+json")
    .send()
    .await;

    match result {
        Ok(r) => r.json().await,
        Err(e) => Err(e),
    }
}

pub async fn get_entity(
    api_path: &str,
    entity_id: &str,
) -> Result<Response<ObjectBinding>, gloo_net::Error> {
    let result = Request::get(&format!(
        "{api_path}/query?query=getEntity&$entity=<{entity_id}>"
    ))
    .header("Accept", "application/sparql-results+json")
    .send()
    .await;

    match result {
        Ok(r) => r.json().await,
        Err(e) => Err(e),
    }
}

pub async fn get_spatial_entity(
    api_path: &str,
    entity_id: &str,
) -> Result<Response<SpatialEntityBinding>, gloo_net::Error> {
    let result = Request::get(&format!(
        "{api_path}/query?query=getSpatialEntity&$entity=<{entity_id}>"
    ))
    .header("Accept", "application/sparql-results+json")
    .send()
    .await;

    match result {
        Ok(r) => r.json().await,
        Err(e) => Err(e),
    }
}

// Response processing

fn extract_label(label: Option<ObjectPropertyBinding>) -> Option<IString> {
    label.and_then(|l| {
        if l.binding_type == BindingType::Literal {
            Some(l.value.into())
        } else {
            None
        }
    })
}

pub fn activity_from_response(response: Response<ObjectBinding>) -> models::Activity {
    #[derive(Default)]
    struct State {
        ended_at: Option<DateTime<Utc>>,
        generated: Vec<models::EntityLink>,
        influenced: Vec<models::ActivityLink>,
        label: Option<IString>,
        properties: Vec<(IString, IString)>,
        started_at: Option<DateTime<Utc>>,
        used: Vec<models::EntityLink>,
        was_associated_with: Vec<models::AgentLink>,
        was_influenced_by: Vec<models::ActivityLink>,
    }

    let s = response
        .results
        .bindings
        .into_iter()
        .fold(State::default(), |mut s, o| {
            if o.p.binding_type == BindingType::Uri {
                if o.p.value == "http://www.w3.org/ns/prov#endedAtTime" {
                    s.ended_at = o.o.value.parse().ok();
                } else if o.p.value == "http://www.w3.org/2000/01/rdf-schema#label" {
                    s.label = Some(o.o.value.into());
                } else if o.p.value == "http://www.w3.org/ns/prov#generated" {
                    s.generated
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.p.value == "http://www.w3.org/ns/prov#influenced" {
                    s.influenced
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.p.value == "http://www.w3.org/ns/prov#startedAtTime" {
                    s.started_at = o.o.value.parse().ok();
                } else if o.p.value == "http://www.w3.org/ns/prov#used" {
                    s.used.push((extract_label(o.olabel), o.o.value.into()));
                } else if o.p.value == "http://www.w3.org/ns/prov#wasAssociatedWith" {
                    s.was_associated_with
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.p.value == "http://www.w3.org/ns/prov#wasInfluencedBy" {
                    s.was_influenced_by
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.o.binding_type == BindingType::Literal {
                    if let Some(label) = extract_label(o.plabel) {
                        s.properties.push((label, o.o.value.into()));
                    }
                }
            }
            s
        });

    models::Activity {
        ended_at: s.ended_at,
        label: s.label,
        generated: s.generated.into(),
        influenced: s.influenced.into(),
        properties: s.properties.into(),
        started_at: s.started_at,
        used: s.used.into(),
        was_associated_with: s.was_associated_with.into(),
        was_influenced_by: s.was_influenced_by.into(),
    }
}

pub fn agent_from_response(response: Response<ObjectBinding>) -> models::Agent {
    #[derive(Default)]
    struct State {
        influenced: Vec<models::ActivityLink>,
        label: Option<IString>,
        properties: Vec<(IString, IString)>,
    }

    let s = response
        .results
        .bindings
        .into_iter()
        .fold(State::default(), |mut s, o| {
            if o.p.binding_type == BindingType::Uri {
                if o.p.value == "http://www.w3.org/ns/prov#influenced" {
                    s.influenced
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.p.value == "http://www.w3.org/2000/01/rdf-schema#label" {
                    s.label = Some(o.o.value.into());
                } else if o.o.binding_type == BindingType::Literal {
                    if let Some(label) = extract_label(o.plabel) {
                        s.properties.push((label, o.o.value.into()));
                    }
                }
            }
            s
        });

    models::Agent {
        influenced: s.influenced.into(),
        label: s.label,
        properties: s.properties.into(),
    }
}

pub fn dim_desc_from_response(response: Response<DimDescBinding>) -> Vec<models::DimDesc> {
    response
        .results
        .bindings
        .into_iter()
        .filter_map(|dd| {
            if dd.name.binding_type != BindingType::Literal {
                return None;
            }
            let name = dd.name.value.into();

            if dd.order.binding_type != BindingType::Literal {
                return None;
            }
            let order = dd.order.value.parse().ok()?;

            if dd.range.binding_type != BindingType::Literal {
                return None;
            }
            let range = 0f32..dd.range.value.parse().ok()?;

            let target_range = dd.target_range.and_then(|target_range_binding| {
                if target_range_binding.binding_type != BindingType::Literal {
                    return None;
                }
                Some(0f32..target_range_binding.value.parse().ok()?)
            });

            Some(models::DimDesc {
                name,
                order,
                range,
                target_range,
            })
        })
        .collect()
}

pub fn dim_values_from_response(response: Response<DimValueBinding>) -> Vec<models::DimValue> {
    response
        .results
        .bindings
        .into_iter()
        .filter_map(|dv| {
            if dv.d.binding_type != BindingType::Literal {
                return None;
            }
            let d = dv.d.value.into();

            if dv.v.binding_type != BindingType::Literal {
                return None;
            }
            let v = dv.v.value.parse().ok()?;

            Some(models::DimValue { d, v })
        })
        .collect()
}

pub fn entity_from_response(response: Response<ObjectBinding>) -> models::Entity {
    #[derive(Default)]
    struct State {
        label: Option<IString>,
        properties: Vec<(IString, IString)>,
        was_derived_from: Vec<models::EntityLink>,
        was_generated_by: Vec<models::ActivityLink>,
        was_attributed_to: Vec<models::ActivityLink>,
    }

    let s = response
        .results
        .bindings
        .into_iter()
        .fold(State::default(), |mut s, o| {
            if o.p.binding_type == BindingType::Uri {
                if o.p.value == "http://www.w3.org/2000/01/rdf-schema#label" {
                    s.label = Some(o.o.value.into());
                } else if o.p.value == "http://www.w3.org/ns/prov#wasAttributedTo" {
                    s.was_attributed_to
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.p.value == "http://www.w3.org/ns/prov#wasDerivedFrom" {
                    s.was_derived_from
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.p.value == "http://www.w3.org/ns/prov#wasGeneratedBy" {
                    s.was_generated_by
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.o.binding_type == BindingType::Literal {
                    if let Some(label) = extract_label(o.plabel) {
                        s.properties.push((label, o.o.value.into()));
                    }
                }
            }
            s
        });

    models::Entity {
        geometry: None,
        label: s.label,
        properties: s.properties.into(),
        was_attributed_to: s.was_attributed_to.into(),
        was_derived_from: s.was_derived_from.into(),
        was_generated_by: s.was_generated_by.into(),
    }
}

pub fn spatial_entities_from_response(
    response: Response<SpatialEntityBinding>,
) -> Vec<(IString, models::Entity)> {
    response
        .results
        .bindings
        .into_iter()
        .filter_map(|b| {
            if b.entity.binding_type != BindingType::Uri {
                return None;
            }
            let id = b.entity.value.into();

            if b.geojson.binding_type != BindingType::Literal
                || &b.geojson.datatype != "http://www.opengis.net/ont/geosparql#geoJSONLiteral"
            {
                return None;
            }
            let geometry = b
                .geojson
                .value
                .parse::<GeoJson>()
                .ok()
                .and_then(|g| g.try_into().ok().map(Rc::new))?;

            let label = extract_label(b.label);

            Some((
                id,
                models::Entity {
                    geometry: Some(geometry),
                    label,
                    ..models::Entity::default()
                },
            ))
        })
        .collect()
}

//

#[cfg(test)]
mod test {
    use std::{fs, io::Read};

    use super::*;

    #[test]
    fn test_deser_q1() {
        let mut f = fs::File::open(format!(
            "{}/sample_data/q1.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();

        let mut raw_response = String::new();
        f.read_to_string(&mut raw_response).unwrap();
        let response = serde_json::from_str(&raw_response).unwrap();

        assert_eq!(
            format!("{:?}", spatial_entities_from_response(response)),
            "[(Rc(\"http://example.com/data/c\"), Entity { geometry: Some(Polygon(Polygon { exterior: LineString([Coord { x: 150.5, y: -34.0 }, Coord { x: 150.502, y: -34.0005 }, Coord { x: 150.504, y: -34.001 }, Coord { x: 150.506, y: -34.0015 }, Coord { x: 150.508, y: -34.002 }, Coord { x: 150.51, y: -34.0025 }, Coord { x: 150.512, y: -34.003 }, Coord { x: 150.514, y: -34.0035 }, Coord { x: 150.516, y: -34.004 }, Coord { x: 150.518, y: -34.0045 }, Coord { x: 150.52, y: -34.005 }, Coord { x: 150.522, y: -34.0045 }, Coord { x: 150.524, y: -34.004 }, Coord { x: 150.526, y: -34.0035 }, Coord { x: 150.528, y: -34.003 }, Coord { x: 150.53, y: -34.0025 }, Coord { x: 150.528, y: -34.002 }, Coord { x: 150.526, y: -34.0015 }, Coord { x: 150.524, y: -34.001 }, Coord { x: 150.522, y: -34.0005 }, Coord { x: 150.52, y: -34.0 }, Coord { x: 150.518, y: -34.0005 }, Coord { x: 150.516, y: -34.001 }, Coord { x: 150.514, y: -34.0015 }, Coord { x: 150.512, y: -34.002 }, Coord { x: 150.51, y: -34.0025 }, Coord { x: 150.508, y: -34.003 }, Coord { x: 150.506, y: -34.0025 }, Coord { x: 150.504, y: -34.002 }, Coord { x: 150.502, y: -34.0015 }, Coord { x: 150.5, y: -34.001 }, Coord { x: 150.5, y: -34.0 }]), interiors: [] })), label: Some(Rc(\"C\")), properties: [], was_attributed_to: [], was_derived_from: [], was_generated_by: [] })]"
        );
    }

    #[test]
    fn test_deser_q2() {
        let mut f = fs::File::open(format!(
            "{}/sample_data/q2.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();

        let mut raw_response = String::new();
        f.read_to_string(&mut raw_response).unwrap();
        let response = serde_json::from_str(&raw_response).unwrap();

        assert_eq!(
            format!("{:?}", entity_from_response(response)),
            "Entity { geometry: None, label: Some(Rc(\"C\")), properties: [], was_attributed_to: [(Some(Rc(\"Adder-run1\")), Rc(\"http://example.com/activities/add1\")), (Some(Rc(\"A\")), Rc(\"http://example.com/data/a\")), (Some(Rc(\"B\")), Rc(\"http://example.com/data/b\"))], was_derived_from: [(Some(Rc(\"A\")), Rc(\"http://example.com/data/a\")), (Some(Rc(\"B\")), Rc(\"http://example.com/data/b\"))], was_generated_by: [(Some(Rc(\"Adder-run1\")), Rc(\"http://example.com/activities/add1\"))] }"
        );
    }

    #[test]
    fn test_deser_q3() {
        let mut f = fs::File::open(format!(
            "{}/sample_data/q3.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();

        let mut raw_response = String::new();
        f.read_to_string(&mut raw_response).unwrap();
        let response = serde_json::from_str(&raw_response).unwrap();

        assert_eq!(
            format!("{:?}", activity_from_response(response)),
            "Activity { ended_at: Some(2029-01-01T20:05:19Z), generated: [(Some(Rc(\"C\")), Rc(\"http://example.com/data/c\"))], influenced: [(Some(Rc(\"C\")), Rc(\"http://example.com/data/c\"))], label: Some(Rc(\"Adder-run1\")), properties: [], started_at: None, used: [(Some(Rc(\"A\")), Rc(\"http://example.com/data/a\")), (Some(Rc(\"B\")), Rc(\"http://example.com/data/b\"))], was_associated_with: [(Some(Rc(\"Add\")), Rc(\"http://example.com/agents/adder\"))], was_influenced_by: [(Some(Rc(\"A\")), Rc(\"http://example.com/data/a\")), (Some(Rc(\"B\")), Rc(\"http://example.com/data/b\")), (Some(Rc(\"Add\")), Rc(\"http://example.com/agents/adder\"))] }"
        );
    }

    #[test]
    fn test_deser_agent() {
        let mut f = fs::File::open(format!(
            "{}/sample_data/agent.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();

        let mut raw_response = String::new();
        f.read_to_string(&mut raw_response).unwrap();
        let response = serde_json::from_str(&raw_response).unwrap();

        assert_eq!(
            format!("{:?}", agent_from_response(response)),
            "Agent { influenced: [(Some(Rc(\"Route Geometry Extraction\")), Rc(\"http://example.com/activities/router-q2\"))], label: Some(Rc(\"ChatGPT (OpenAI) generic model\")), properties: [] }"
        );
    }

    #[test]
    fn test_deser_dimdescs() {
        let mut f = fs::File::open(format!(
            "{}/sample_data/path_q3.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();

        let mut raw_response = String::new();
        f.read_to_string(&mut raw_response).unwrap();
        let response = serde_json::from_str(&raw_response).unwrap();

        assert_eq!(
            format!("{:?}", dim_desc_from_response(response)),
            "[DimDesc { name: Rc(\"Foo\"), order: 1, range: 0.0..5.0, target_range: Some(0.0..5.0) }, DimDesc { name: Rc(\"Bar\"), order: 3, range: 0.0..5.0, target_range: Some(0.0..5.0) }, DimDesc { name: Rc(\"Dang\"), order: 2, range: 0.0..5.0, target_range: Some(0.0..5.0) }]"
        );
    }

    #[test]
    fn test_deser_dimvalues() {
        let mut f = fs::File::open(format!(
            "{}/sample_data/path_q4.json",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();

        let mut raw_response = String::new();
        f.read_to_string(&mut raw_response).unwrap();
        let response = serde_json::from_str(&raw_response).unwrap();

        assert_eq!(
            format!("{:?}", dim_values_from_response(response)),
            "[DimValue { d: Rc(\"Foo\"), v: 3.0 }, DimValue { d: Rc(\"Bar\"), v: 1.0 }, DimValue { d: Rc(\"Dang\"), v: 2.0 }]"
        );
    }
}
