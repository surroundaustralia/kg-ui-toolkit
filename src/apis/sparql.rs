use gloo_net::http::Request;
use implicit_clone::unsync::{IArray, IString};
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
pub struct ObjectBinding {
    pub p: ObjectPropertyBinding,
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

pub async fn get_entity(object_id: &str) -> Result<Response<ObjectBinding>, gloo_net::Error> {
    let result = Request::get(&format!(
        "/prov-chains/query?query=getObject&$object=<{object_id}>"
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
    entity_id: &str,
) -> Result<Response<SpatialEntityBinding>, gloo_net::Error> {
    let result = Request::get(&format!(
        "/prov-chains/query?query=getSpatialEntity&$entity=<{entity_id}>"
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

pub fn entity_from_response(response: Response<ObjectBinding>) -> models::Entity {
    #[derive(Default)]
    struct State {
        label: Option<IString>,
        value: Option<IString>,
        value_label: Option<IString>,
        was_derived_from: Vec<models::Link>,
        was_generated_by: Vec<models::Link>,
        was_influenced_by: Vec<models::Link>,
    }

    let s = response
        .results
        .bindings
        .into_iter()
        .fold(State::default(), |mut s, o| {
            if o.p.binding_type == BindingType::Uri {
                if o.p.value == "http://www.w3.org/2000/01/rdf-schema#label" {
                    s.label = Some(o.o.value.into());
                } else if o.p.value == "https://schema.org/value" {
                    s.value = Some(o.o.value.into());
                    s.value_label = extract_label(o.olabel);
                } else if o.p.value == "http://www.w3.org/ns/prov#wasDerivedFrom" {
                    s.was_derived_from
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.p.value == "http://www.w3.org/ns/prov#wasGeneratedBy" {
                    s.was_generated_by
                        .push((extract_label(o.olabel), o.o.value.into()));
                } else if o.p.value == "http://www.w3.org/ns/prov#wasInfluencedBy" {
                    s.was_influenced_by
                        .push((extract_label(o.olabel), o.o.value.into()));
                }
            }
            s
        });

    models::Entity {
        geometry: None,
        label: s.label,
        value: s.value,
        value_label: s.value_label,
        was_derived_from: s.was_derived_from.into(),
        was_generated_by: s.was_generated_by.into(),
        was_influenced_by: s.was_influenced_by.into(),
    }
}

pub fn spatial_entities_from_response(
    response: Response<SpatialEntityBinding>,
) -> IArray<(IString, models::Entity)> {
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
            "[(Rc(\"http://example.com/data/c\"), Entity { geometry: Some(Polygon(Polygon { exterior: LineString([Coord { x: 150.5, y: -34.0 }, Coord { x: 150.502, y: -34.0005 }, Coord { x: 150.504, y: -34.001 }, Coord { x: 150.506, y: -34.0015 }, Coord { x: 150.508, y: -34.002 }, Coord { x: 150.51, y: -34.0025 }, Coord { x: 150.512, y: -34.003 }, Coord { x: 150.514, y: -34.0035 }, Coord { x: 150.516, y: -34.004 }, Coord { x: 150.518, y: -34.0045 }, Coord { x: 150.52, y: -34.005 }, Coord { x: 150.522, y: -34.0045 }, Coord { x: 150.524, y: -34.004 }, Coord { x: 150.526, y: -34.0035 }, Coord { x: 150.528, y: -34.003 }, Coord { x: 150.53, y: -34.0025 }, Coord { x: 150.528, y: -34.002 }, Coord { x: 150.526, y: -34.0015 }, Coord { x: 150.524, y: -34.001 }, Coord { x: 150.522, y: -34.0005 }, Coord { x: 150.52, y: -34.0 }, Coord { x: 150.518, y: -34.0005 }, Coord { x: 150.516, y: -34.001 }, Coord { x: 150.514, y: -34.0015 }, Coord { x: 150.512, y: -34.002 }, Coord { x: 150.51, y: -34.0025 }, Coord { x: 150.508, y: -34.003 }, Coord { x: 150.506, y: -34.0025 }, Coord { x: 150.504, y: -34.002 }, Coord { x: 150.502, y: -34.0015 }, Coord { x: 150.5, y: -34.001 }, Coord { x: 150.5, y: -34.0 }]), interiors: [] })), label: Some(Rc(\"C\")), value: None, value_label: None, was_derived_from: [], was_generated_by: [], was_influenced_by: [] })]"
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
            "Entity { geometry: None, label: Some(Rc(\"C\")), value: Some(Rc(\"3\")), value_label: None, was_derived_from: [(Some(Rc(\"A\")), Rc(\"http://example.com/data/a\")), (Some(Rc(\"B\")), Rc(\"http://example.com/data/b\"))], was_generated_by: [(Some(Rc(\"Adder-run1\")), Rc(\"http://example.com/activities/add1\"))], was_influenced_by: [(Some(Rc(\"Adder-run1\")), Rc(\"http://example.com/activities/add1\")), (Some(Rc(\"A\")), Rc(\"http://example.com/data/a\")), (Some(Rc(\"B\")), Rc(\"http://example.com/data/b\"))] }"
        );
    }
}
