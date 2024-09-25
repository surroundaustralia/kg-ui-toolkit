//! A place where all of the Yew components of the library reside.

use html::Scope;
use implicit_clone::unsync::{IArray, IString};
use web_sys::{wasm_bindgen::JsCast, HtmlAnchorElement};
use yew::prelude::*;

pub mod activity;
pub mod agent;
pub mod entity;
pub mod spatial_entities;

fn onclick_anchor_handler<COMP: BaseComponent<Message = M>, M: 'static>(
    link: &Scope<COMP>,
    message: fn(IString) -> M,
) -> Callback<MouseEvent> {
    link.batch_callback(move |event: MouseEvent| {
        event.prevent_default();
        event.target().and_then(|event_target| {
            event_target
                .dyn_into::<HtmlAnchorElement>()
                .ok()
                .map(|element| message(element.href().into()))
        })
    })
}

#[derive(Properties, PartialEq)]
struct DateTimeProps {
    label: IString,
    id: IString,
    value: Option<chrono::DateTime<chrono::Utc>>,
}

#[function_component]
fn DateTime(props: &DateTimeProps) -> Html {
    props
        .value
        .iter()
        .map(|value| {
            html! {
                <>
                <label for={props.id.clone()}>{props.label.clone()}</label>
                <input id={props.id.clone()} type="text" readonly=true value={format!("{}", value.format("%Y-%m-%d %H:%M:%S UTC"))} />
                </>
            }
        })
        .collect::<Html>()
}

#[derive(Properties, PartialEq)]
struct GenericPropertiesProps {
    properties: IArray<(IString, IString)>,
}

#[function_component]
fn GenericProperties(props: &GenericPropertiesProps) -> Html {
    html! {
        props.properties.iter().enumerate().map(|(i, (label, value))| {
            let id = format!("properties-{i}");
            html! {
                <>
                <label for={id.clone()}>{label}</label>
                <input id={id} type="text" readonly=true value={value} />
                </>
            }
        }).collect::<Html>()
    }
}

#[derive(Properties, PartialEq)]
struct ProvenanceLinksProps {
    label: IString,
    id_prefix: IString,
    links: IArray<(Option<IString>, IString)>,
    onclick: Callback<MouseEvent>,
}

#[function_component]
fn ProvenanceLinks(props: &ProvenanceLinksProps) -> Html {
    props
        .links
        .iter()
        .enumerate()
        .map(|(i, link)| {
            let id = format!("{}-{}", props.id_prefix, i);
            html! {
                <>
                <label for={id.clone()}>{props.label.clone()}</label>
                <a key={id.clone()} id={id} href={link.1.clone()} onclick={props.onclick.clone()}>
                    {link.0.clone().unwrap_or(link.1.clone())}
                </a>
                </>
            }
        })
        .collect::<Html>()
}
