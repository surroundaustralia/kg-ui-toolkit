use implicit_clone::unsync::IString;
use yew::prelude::*;

use crate::{
    components::{onclick_anchor_handler, DateTime, GenericProperties, ProvenanceLinks},
    models,
};

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
        html! {
            <form>
                <fieldset>
                    <legend>{format!("Activity: {}", ctx.props().activity.label.clone().unwrap_or(AttrValue::from("<unknown>")))}</legend>
                    <DateTime id="started-at" label="Started at" value={ctx.props().activity.started_at} />
                    <DateTime id="ended-at" label="Ended at" value={ctx.props().activity.ended_at} />
                    <GenericProperties properties={ctx.props().activity.properties.clone()} />
                    <ProvenanceLinks id_prefix="generated" label="Generated" links={ctx.props().activity.generated.clone()} onclick={onclick_anchor_handler(ctx.link(), Message::EntityClicked)} />
                    <ProvenanceLinks id_prefix="influenced" label="Influenced" links={ctx.props().activity.influenced.clone()} onclick={onclick_anchor_handler(ctx.link(), Message::ActivityClicked)} />
                    <ProvenanceLinks id_prefix="used" label="Used" links={ctx.props().activity.used.clone()} onclick={onclick_anchor_handler(ctx.link(), Message::EntityClicked)} />
                    <ProvenanceLinks id_prefix="associated-with" label="Associated with" links={ctx.props().activity.was_associated_with.clone()} onclick={onclick_anchor_handler(ctx.link(), Message::AgentClicked)} />
                    <ProvenanceLinks id_prefix="influenced-by" label="Influenced by" links={ctx.props().activity.was_influenced_by.clone()} onclick={onclick_anchor_handler(ctx.link(), Message::ActivityClicked)} />
                </fieldset>
            </form>
        }
    }
}
