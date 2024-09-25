use implicit_clone::unsync::IString;
use yew::prelude::*;

use crate::{
    components::{onclick_anchor_handler, GenericProperties, ProvenanceLinks},
    models,
};

pub enum Message {
    ActivityClicked(IString),
    AgentClicked(IString),
    EntityClicked(IString),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub agent: models::Agent,
    #[prop_or_default]
    pub on_activity_click: Option<Callback<IString>>,
    #[prop_or_default]
    pub on_agent_click: Option<Callback<IString>>,
    #[prop_or_default]
    pub on_entity_click: Option<Callback<IString>>,
}

pub struct Agent;

impl Component for Agent {
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
        html! {
            <form>
                <fieldset>
                    <legend>{format!("Agent: {}", ctx.props().agent.label.clone().unwrap_or(AttrValue::from("<unknown>")))}</legend>
                    <GenericProperties properties={ctx.props().agent.properties.clone()} />
                    <ProvenanceLinks id_prefix="influenced" label="Influenced" links={ctx.props().agent.influenced.clone()} onclick={onclick_anchor_handler(ctx.link(), Message::ActivityClicked)} />
                </fieldset>
            </form>
        }
    }
}
