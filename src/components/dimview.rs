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

const WIDTH: f32 = 200.0;
const HEIGHT: f32 = 200.0;
const MARGIN: f32 = 20.0;

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

            let mut data_by_series: Vec<(IString, Rc<Data<f32, f32>>)> = ctx
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
            data_by_series.sort_by_key(|(series_name, _)| series_name.clone());

            let scale = Rc::new(LinearScale::new(-1.0..1.0, 1.0)) as Rc<dyn Scale<Scalar = _>>;

            let cx = IString::from((WIDTH / 2.0).to_string());
            let cy = IString::from((HEIGHT / 2.0).to_string());

            html! {
                <svg class="chart" viewBox={format!("0 0 {} {}", WIDTH, HEIGHT)} preserveAspectRatio="none">
                    {
                        series::to_radial(vec![0.8; dim_desc.len()])
                            .into_iter().map(|(x2, y2, _)| {
                                html! {
                                    <line x1={cx.clone()} y1={cy.clone()} x2={(scale.normalise(x2).0 * WIDTH).to_string()} y2={(scale.normalise(-y2).0 * HEIGHT).to_string()} />
                                }
                            }).collect::<Html>()
                    }
                    {
                        (2..=4).map(|i| {
                            let r = ((WIDTH / 2.0) - (MARGIN * i as f32)).to_string();
                            html! {
                                <circle cx={cx.clone()} cy={cy.clone()} r={r} />
                            }
                    }).collect::<Html>() }

                    <Series<f32, f32>
                        series_type={Type::Area}
                        name="labels"
                        data={data_labels}
                        horizontal_scale={scale.clone()}
                        vertical_scale={scale.clone()}
                        x={MARGIN} y={MARGIN} width={WIDTH - (MARGIN * 2.0)} height={HEIGHT - (MARGIN * 2.0)} />

                    { data_by_series.iter().enumerate().map(|(i, (_, data))| {
                        html! {
                            <Series<f32, f32>
                                series_type={Type::Area}
                                name={format!("series-name-{i}")}
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
