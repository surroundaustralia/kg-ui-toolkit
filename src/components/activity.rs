use implicit_clone::unsync::IString;
use web_sys::{wasm_bindgen::JsCast, HtmlAnchorElement};
use yew::prelude::*;

use crate::models;

pub enum Message {
    ActivityClicked(IString),
    AgentClicked(IString),
    EntityClicked(IString),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub activity: models::Activity,
    #[prop_or_default]
    pub on_activity_click: Option<Callback<IString>>,
    #[prop_or_default]
    pub on_agent_click: Option<Callback<IString>>,
    #[prop_or_default]
    pub on_entity_click: Option<Callback<IString>>,
}

pub struct Activity;

impl Component for Activity {
    type Message = Message;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ActivityClicked(activity_id) => {
                if let Some(on_activity_click) = &ctx.props().on_activity_click {
                    on_activity_click.emit(activity_id);
                }
                false
            }
            Message::AgentClicked(agent_id) => {
                if let Some(on_agent_click) = &ctx.props().on_agent_click {
                    on_agent_click.emit(agent_id);
                }
                false
            }
            Message::EntityClicked(entity_id) => {
                if let Some(on_entity_click) = &ctx.props().on_entity_click {
                    on_entity_click.emit(entity_id);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = |message: fn(IString) -> Message| {
            ctx.link().batch_callback(move |event: MouseEvent| {
                event.prevent_default();
                event.target().and_then(|event_target| {
                    event_target
                        .dyn_into::<HtmlAnchorElement>()
                        .ok()
                        .map(|element| message(element.href().into()))
                })
            })
        };

        html! {
            <form>
                <fieldset>
                    <legend>{format!("Activity: {}", ctx.props().activity.label.clone().unwrap_or(AttrValue::from("<unknown>")))}</legend>
                    {
                        ctx.props().activity.started_at.iter().map(|value| {
                            let id = "ended-at";
                            html! {
                                <>
                                <label for={id}>{"Started at"}</label>
                                <input id={id} type="text" readonly=true value={format!("{}", value.format("%Y-%m-%d %H:%M:%S UTC"))} />
                                </>
                            }
                        }).collect::<Html>()
                    }
                    {
                        ctx.props().activity.ended_at.iter().map(|value| {
                            let id = "ended-at";
                            html! {
                                <>
                                <label for={id}>{"Ended at"}</label>
                                <input id={id} type="text" readonly=true value={format!("{}", value.format("%Y-%m-%d %H:%M:%S UTC"))} />
                                </>
                            }
                        }).collect::<Html>()
                    }
                    {
                        ctx.props().activity.properties.iter().enumerate().map(|(i, (label, value))| {
                            let id = format!("properties-{i}");
                            html! {
                                <>
                                <label for={id.clone()}>{label}</label>
                                <input id={id} type="text" readonly=true value={value} />
                                </>
                            }
                        }).collect::<Html>()
                    }
                    {
                        ctx.props().activity.generated.iter().enumerate().map(|(i, link)| {
                            let id = format!("generated-{i}");
                            html! {
                                <>
                                <label for={id.clone()}>{"Generated"}</label>
                                <a key={id.clone()} id={id} href={link.1.clone()} onclick={onclick(Message::EntityClicked)}>{link.0.clone().unwrap_or(link.1.clone())}</a>
                                </>
                            }
                        }).collect::<Html>()
                    }
                    {
                        ctx.props().activity.influenced.iter().enumerate().map(|(i, link)| {
                            let id = format!("influenced-{i}");
                            html! {
                                <>
                                <label for={id.clone()}>{"Influenced"}</label>
                                <a key={id.clone()} id={id} href={link.1.clone()} onclick={onclick(Message::ActivityClicked)}>{link.0.clone().unwrap_or(link.1.clone())}</a>
                                </>
                            }
                        }).collect::<Html>()
                    }
                    {
                        ctx.props().activity.used.iter().enumerate().map(|(i, link)| {
                            let id = format!("used-{i}");
                            html! {
                                <>
                                <label for={id.clone()}>{"Used"}</label>
                                <a key={id.clone()} id={id} href={link.1.clone()} onclick={onclick(Message::EntityClicked)}>{link.0.clone().unwrap_or(link.1.clone())}</a>
                                </>
                            }
                        }).collect::<Html>()
                    }
                    {
                        ctx.props().activity.was_associated_with.iter().enumerate().map(|(i, link)| {
                            let id = format!("was-associated-with-{i}");
                            html! {
                                <>
                                <label for={id.clone()}>{"Associated with"}</label>
                                <a key={id.clone()} id={id} href={link.1.clone()} onclick={onclick(Message::AgentClicked)}>{link.0.clone().unwrap_or(link.1.clone())}</a>
                                </>
                            }
                        }).collect::<Html>()
                    }
                    {
                        ctx.props().activity.was_influenced_by.iter().enumerate().map(|(i, link)| {
                            let id = format!("was-influenced-by-{i}");
                            html! {
                                <>
                                <label for={id.clone()}>{"Influenced by"}</label>
                                <a key={id.clone()} id={id} href={link.1.clone()} onclick={onclick(Message::ActivityClicked)}>{link.0.clone().unwrap_or(link.1.clone())}</a>
                                </>
                            }
                        }).collect::<Html>()
                    }
                </fieldset>
            </form>
        }
    }
}
