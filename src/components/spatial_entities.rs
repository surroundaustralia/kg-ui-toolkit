use std::rc::Rc;

use geo::{BoundingRect, Coord, MapCoords, Rect, Scale};
use geo_svg::{Style, ToSvg};
use html::ImplicitClone;
use implicit_clone::unsync::{IArray, IString};
use web_sys::{wasm_bindgen::JsCast, Element};
use yew::prelude::*;

use crate::models;

#[derive(Clone, PartialEq)]
pub struct Map {
    pub src: IString,
    pub extent: Rc<Rect>,
}

impl ImplicitClone for Map {}

pub enum Message {
    EntityClicked(IString),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub entities: IArray<(IString, models::Entity)>,
    pub map: Map,
    #[prop_or_default]
    pub on_entity_click: Option<Callback<IString>>,
    #[prop_or_default]
    pub dynamic_viewport: bool,
}

pub struct SpatialEntities {
    geometry_html: Html,
}

impl SpatialEntities {
    fn to_html(
        dynamic_viewport: bool,
        mut entities: Vec<(IString, models::Entity)>,
        map: &Map,
    ) -> Html {
        let mut entity_bounds: Option<Rect> = None;
        let map_extent = map.extent.map_coords(|Coord { x, y }| Coord { x, y: -y });
        entities.sort_by_key(|(id, _)| id.clone());
        let content = entities
            .iter()
            .enumerate()
            .map(|(i, (_, entity))| {
                if let Some(geometry) = &entity.geometry {
                    let geometry = geometry.map_coords(|Coord { x, y }| Coord { x, y: -y });

                    if let Some(bounding_rect) = geometry.bounding_rect() {
                        if dynamic_viewport {
                            if let Some(entity_bounds) = &mut entity_bounds {
                                let mut min = entity_bounds.min();
                                if bounding_rect.min().x < min.x {
                                    min.x = bounding_rect.min().x;
                                }
                                if bounding_rect.min().y < min.y {
                                    min.y = bounding_rect.min().y;
                                }
                                entity_bounds.set_min(min);

                                let mut max = entity_bounds.max();
                                if bounding_rect.max().x > max.x {
                                    max.x = bounding_rect.max().x;
                                }
                                if bounding_rect.max().y > max.y {
                                    max.y = bounding_rect.max().y;
                                }
                                entity_bounds.set_max(max);
                            } else {
                                entity_bounds = Some(bounding_rect);
                            }
                        }
                    }

                    let geometry_svg_str = geometry
                        .to_svg()
                        .items
                        .iter()
                        .map(|item| item.to_svg_str(&Style::default()))
                        .collect::<Vec<String>>()
                        .into_iter()
                        .collect::<String>();

                    format!(
                        "
                            <g class=\"entity-region-{}\">
                                {}
                            </g>
                        ",
                        i, geometry_svg_str,
                    )
                } else {
                    "".into()
                }
            })
            .collect::<Vec<String>>()
            .into_iter()
            .collect::<String>();
        if dynamic_viewport {
            entity_bounds = entity_bounds.map(|b| b.scale(1.25));
        } else if !entities.is_empty() {
            entity_bounds = Some(map_extent);
        }
        if let Some(entity_bounds) = entity_bounds {
            let viewport = format!(
                "{} {} {} {}",
                entity_bounds.min().x,
                entity_bounds.min().y,
                entity_bounds.width(),
                entity_bounds.height()
            );
            let map_x = map_extent.min().x;
            let map_y = map_extent.min().y;
            let map_width = map_extent.width();
            let map_height = map_extent.height();
            let map_src = &map.src;
            Html::from_html_unchecked(
                format!(
                    "
                    <svg viewbox=\"{viewport}\">
                        <image x={map_x} y={map_y} width={map_width} height={map_height} href={map_src} />
                        {content}
                    </svg>
                "
                )
                .into(),
            )
        } else {
            html!()
        }
    }
}

impl Component for SpatialEntities {
    type Message = Message;

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            geometry_html: Self::to_html(
                ctx.props().dynamic_viewport,
                ctx.props().entities.to_vec(),
                &ctx.props().map,
            ),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.geometry_html = Self::to_html(
            ctx.props().dynamic_viewport,
            ctx.props().entities.to_vec(),
            &ctx.props().map,
        );
        true
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::EntityClicked(entity_id) => {
                if let Some(on_entity_click) = &ctx.props().on_entity_click {
                    on_entity_click.emit(entity_id);
                }
                false
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let onclick = ctx.link().batch_callback(|event: MouseEvent| {
            event.target().and_then(|event_target| {
                event_target
                    .dyn_into::<Element>()
                    .ok()
                    .and_then(|mut element| {
                        while element.get_attribute("class").as_deref() != Some("entity-region") {
                            element = element.parent_element()?;
                        }
                        Some(Message::EntityClicked(element.id().into()))
                    })
            })
        });
        html! {
            <>
            <div onclick={onclick}>{self.geometry_html.clone()}</div>
            </>
        }
    }
}
