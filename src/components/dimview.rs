use std::rc::Rc;

use implicit_clone::unsync::{IArray, IString};
use yew::prelude::*;
use yew_chart::{
    axis::Scale,
    linear_axis_scale::LinearScale,
    series::{self, Data, Labeller, Series, Type},
};

use crate::models;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub dim_desc: IArray<models::DimDesc>,
    pub dim_values: IArray<(IString, IArray<models::DimValue>)>,
}

pub struct DimView;

const WIDTH: f32 = 300.0;
const HEIGHT: f32 = 300.0;
const MARGIN: f32 = 50.0;

impl Component for DimView {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !ctx.props().dim_desc.is_empty() {
            let mut dim_desc = ctx.props().dim_desc.to_vec();
            dim_desc.sort_by_key(|dd| dd.order);

            let labels = dim_desc.iter().map(|dd| dd.name.clone());

            let data_labels = Rc::new(
                series::to_radial(vec![1.0; dim_desc.len()])
                    .into_iter()
                    .zip(labels)
                    .map(|((x, y, _), label)| {
                        (
                            x,
                            y,
                            Some(Rc::from(series::circle_text_label(label.to_string()))
                                as Rc<dyn Labeller>),
                        )
                    })
                    .flat_map(|d| [d.clone(), (0.0, 0.0, None), d])
                    .collect(),
            );

            let data_by_series: Vec<(IString, Rc<Data<f32, f32>>)> = ctx
                .props()
                .dim_values
                .iter()
                .map(|(series_name, dvs)| {
                    let values = dim_desc
                        .iter()
                        .flat_map(|dd| {
                            let target_range_end =
                                dd.target_range.clone().unwrap_or(dd.range.clone()).end;
                            let scale = 1.0 / (target_range_end / dd.range.end * dd.range.end);
                            dvs.iter().find_map(|dv| {
                                if dv.d == dd.name {
                                    Some(dv.v * scale)
                                } else {
                                    None
                                }
                            })
                        })
                        .collect();
                    let data = Rc::new(series::to_radial(values));
                    (series_name, data)
                })
                .collect();

            let scale = Rc::new(LinearScale::new(-1.0..1.0, 1.0)) as Rc<dyn Scale<Scalar = _>>;

            html! {
                <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
                    <Series<f32, f32>
                        series_type={Type::Area}
                        name="labels"
                        data={data_labels}
                        horizontal_scale={scale.clone()}
                        vertical_scale={scale.clone()}
                        x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                    { data_by_series.iter().map(|(series_name, data)| {
                        html! {
                            <Series<f32, f32>
                                series_type={Type::Area}
                                name={series_name}
                                data={data}
                                horizontal_scale={scale.clone()}
                                vertical_scale={scale.clone()}
                                x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />
                        }
                    }).collect::<Html>() }

                </svg>

            }
        } else {
            html!()
        }
    }
}
