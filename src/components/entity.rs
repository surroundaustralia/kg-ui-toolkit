use implicit_clone::unsync::IArray;
use yew::prelude::*;

use crate::models;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: Option<AttrValue>,
    pub value: Option<AttrValue>,
    pub value_label: Option<AttrValue>,
    pub was_derived_from: IArray<models::Link>,
    pub was_generated_by: IArray<models::Link>,
    pub was_influenced_by: IArray<models::Link>,
}

#[function_component]
pub fn Entity(props: &Props) -> Html {
    html! {
        <>
        <h2>{format!("Entity: {}", props.label.clone().unwrap_or(AttrValue::from("<unknown>")))}</h2>
        <label for="value">{props.value_label.clone().unwrap_or(AttrValue::from("Value"))}</label>
        <input id="value" type="text" readonly=true value={props.value.clone().unwrap_or(AttrValue::from("-"))} />
        {
            props.was_derived_from.iter().enumerate().map(|(i, link)| {
                let id = format!("was-derived-from-{i}");
                html! {
                    <>
                    <label for={id.clone()}>{"Derived from"}</label>
                    <a key={id.clone()} id={id} href={link.1.clone()}>{link.0.clone().unwrap_or(link.1.clone())}</a>
                    </>
                }
            }).collect::<Html>()
        }
        {
            props.was_generated_by.iter().enumerate().map(|(i, link)| {
                let id = format!("was-generated-by-{i}");
                html! {
                    <>
                    <label for={id.clone()}>{"Generated by"}</label>
                    <a key={id.clone()} id={id} href={link.1.clone()}>{link.0.clone().unwrap_or(link.1.clone())}</a>
                    </>
                }
            }).collect::<Html>()
        }
        {
            props.was_influenced_by.iter().enumerate().map(|(i, link)| {
                let id = format!("was-influenced-by-{i}");
                html! {
                    <>
                    <label for={id.clone()}>{"Influenced by"}</label>
                    <a key={id.clone()} id={id} href={link.1.clone()}>{link.0.clone().unwrap_or(link.1.clone())}</a>
                    </>
                }
            }).collect::<Html>()
        }
        </>
    }
}
