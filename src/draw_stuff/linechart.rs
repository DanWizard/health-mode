use full_palette::GREEN_200;
use full_palette::GREEN_300;
use full_palette::GREEN_500;
use full_palette::GREY;
use full_palette::GREY_200;
use full_palette::GREY_300;
use full_palette::GREY_500;
use full_palette::GREY_700;
use full_palette::GREY_900;
use plotters::coord::ranged1d::SegmentedCoord;
use plotters::coord::types::Monthly;
use plotters::coord::types::Yearly;
use plotters::coord::types::{RangedCoordf32, RangedCoordi32};
use plotters::coord::Shift;
use plotters::prelude::*;

use crate::analyze_stuff::ObjTimeFrameSettings;
use crate::analyze_stuff::TimeFrame;
use crate::data_stuff::HabitDayPerformance;
use crate::data_stuff::Performance;
use crate::data_stuff::Task;

type LineChartSettings<'a> =
    ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedCoordf32, RangedCoordf32>>;

pub fn build_chart<'a, 'b>(
    root: &'a DrawingArea<BitMapBackend<'b>, Shift>,
    settings: &TimeFrame<ObjTimeFrameSettings>,
) -> Result<LineChartSettings<'b>, String> {
    let s: &ObjTimeFrameSettings = match settings {
        TimeFrame::Year(y) => y,
        TimeFrame::Month(m) => m,
    };

    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption(&s.doc_title, ("sans-serif", 40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(1.0..s.x_axis_size as f32, s.y_range.0..s.y_range.1 as f32)
        .unwrap();
    Ok(chart)
}

pub fn style_chart<'a>(
    mut chart: LineChartSettings<'a>,
    settings: &TimeFrame<ObjTimeFrameSettings>,
) -> LineChartSettings<'a> {
    let s: &ObjTimeFrameSettings = match settings {
        TimeFrame::Year(y) => y,
        TimeFrame::Month(m) => m,
    };

    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(s.x_axis_size as usize)
        .y_labels(s.y_range.1 as usize)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()
        .unwrap();
    chart
}

pub fn draw_data<'a>(
    mut chart: LineChartSettings<'a>,
    hdps: &Vec<HabitDayPerformance>,
    settings: &TimeFrame<ObjTimeFrameSettings>,
) -> LineChartSettings<'a> {
    let s: &ObjTimeFrameSettings = match settings {
        TimeFrame::Year(y) => y,
        TimeFrame::Month(m) => m,
    };

    let series_data = hdps
        .iter()
        .enumerate()
        .map(|(index, x)| {
            let x_val = index as f32 + s.x_offset as f32;
            let obj_title = s.tf_name.clone();

            let obj = x.objective_performance.iter().find_map(|v| {
                let task = v.task();
                let title = match task {
                    Task::Objective(t) => t,
                    _ => {
                        panic!("obj not found")
                    }
                };
                println!("{title} {obj_title}");
                if title == obj_title {
                    Some(v)
                } else {
                    None
                }
            });

            let y_val = match obj.unwrap().performance() {
                Performance::Score(ps) => ps,
                _ => {
                    panic!("not type score")
                }
            };

            (x_val, y_val)
        })
        .collect::<Vec<(f32, f32)>>();

    chart
        .draw_series(LineSeries::new(series_data.clone(), &RED))
        .unwrap();
    // Similarly, we can draw point series
    chart
        .draw_series(PointSeries::of_element(
            series_data,
            5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
            },
        ))
        .unwrap();
    chart
}
