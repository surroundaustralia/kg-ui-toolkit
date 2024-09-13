use std::rc::Rc;

use geo::{BoundingRect, Coord, Geometry, MapCoords};
use geo_svg::{Style, ToSvg};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub id: AttrValue,
    pub geometry: Rc<Geometry>,
    pub label: Option<AttrValue>,
    pub onclick: Option<Callback<MouseEvent>>,
}

pub struct SpatialEntity {
    geometry_html: Html,
}

impl SpatialEntity {
    fn to_html(id: AttrValue, label: Option<&str>, geometry: &Geometry) -> Html {
        let geometry = geometry.map_coords(|Coord { x, y }| Coord { x, y: y * -1.0 });

        let label_pos = geometry
            .bounding_rect()
            .map(|r| r.center())
            .unwrap_or_default();

        let geometry_svg_str = geometry
            .to_svg()
            .items
            .iter()
            .map(|item| item.to_svg_str(&Style::default()))
            .collect::<Vec<String>>()
            .into_iter()
            .collect::<String>();

        Html::from_html_unchecked(format!("
            <svg id={id} viewbox=\"145.5866158041706 16.89348230365816 0.3125592028077 0.16828638841675\">
                {}
                <text x={} y={} font-size=\"0.01\">{}</text>
            </svg>
        ", geometry_svg_str, label_pos.x, label_pos.y, label.unwrap_or_default()).into())
    }
}

impl Component for SpatialEntity {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            geometry_html: Self::to_html(
                ctx.props().id.clone(),
                ctx.props().label.as_deref(),
                &ctx.props().geometry,
            ),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().geometry != old_props.geometry {
            self.geometry_html = Self::to_html(
                ctx.props().id.clone(),
                ctx.props().label.as_deref(),
                &ctx.props().geometry,
            );
        }
        true
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
            <div onclick={ctx.props().onclick.clone()}>{self.geometry_html.clone()}</div>
            </>
        }
    }
}
