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
use plotters::coord::types::RangedCoordi32;
use plotters::coord::types::Yearly;
use plotters::coord::Shift;
use plotters::prelude::*;

use crate::analyze_stuff::TimeFrame;
use crate::analyze_stuff::TodoTimeFrameSettings;
use crate::data_stuff::HabitDayPerformance;
use crate::data_stuff::HabitProgress;
use crate::data_stuff::Performance;
use crate::data_stuff::Task;

type HeatmapSettings<'a> = ChartContext<
    'a,
    BitMapBackend<'a>,
    Cartesian2d<SegmentedCoord<RangedCoordi32>, SegmentedCoord<RangedCoordi32>>,
>;

pub fn build_chart<'a, 'b>(
    root: &'a DrawingArea<BitMapBackend<'b>, Shift>,
    settings: &TimeFrame<TodoTimeFrameSettings<'b>>,
) -> Result<HeatmapSettings<'b>, String> {
    let s: &TodoTimeFrameSettings = match settings {
        TimeFrame::Year(y) => y,
        TimeFrame::Month(m) => m,
    };

    let chart = ChartBuilder::on(root)
        .set_label_area_size(LabelAreaPosition::Top, 30)
        .set_label_area_size(LabelAreaPosition::Left, 30)
        .caption(&s.doc_title, ("sans-serif", 20))
        .margin(20)
        .build_cartesian_2d(
            (0..(s.x_axis_size as i32)).into_segmented(),
            (0..((s.y_axis_size - 1) as i32)).into_segmented(),
        )
        .unwrap();
    Ok(chart)
}

fn april_days_inverted() -> Vec<i32> {
    // April has 30 days, in reverse order
    (1..=30).rev().collect()
}

pub fn style_chart<'a>(
    mut chart: HeatmapSettings<'a>,
    settings: &TimeFrame<TodoTimeFrameSettings<'a>>,
) -> Result<HeatmapSettings<'a>, String> {
    let s: &TodoTimeFrameSettings = match settings {
        TimeFrame::Year(y) => y,
        TimeFrame::Month(m) => m,
    };

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_labels(s.x_axis_size as usize)
        .x_label_formatter(&|x| {
            let idx = match x {
                SegmentValue::CenterOf(idx) => *idx,
                SegmentValue::Exact(idx) => *idx,
                _ => return "".to_string(),
            };
            let title = s.column_titles[idx as usize];
            let mut title_string = title.to_string();
            let title_len = title_string.len();
            if title_len > 6 {
                title_string.truncate(6);
                title_string.push_str("...");
            }
            title_string
        })
        .x_label_style(("sans-serif", 15).into_font().color(&BLACK))
        .y_labels(s.y_axis_size as usize)
        .y_label_formatter(&|x| {
            let idx: i32 = match x {
                SegmentValue::CenterOf(idx) => *idx,
                SegmentValue::Exact(idx) => *idx,
                _ => return "".to_string(),
            };
            let inverse_label = ((idx) - s.y_axis_size as i32).abs();
            inverse_label.to_string()
        })
        .draw()
        .unwrap();
    Ok(chart)
}

pub fn draw_rect<'a>(
    chart: HeatmapSettings<'a>,
    hdps: &Vec<HabitDayPerformance>,
    settings: &TimeFrame<TodoTimeFrameSettings<'a>>,
) -> HeatmapSettings<'a> {
    let s: &TodoTimeFrameSettings = match settings {
        TimeFrame::Year(y) => y,
        TimeFrame::Month(m) => m,
    };

    let mut style = ShapeStyle {
        color: RED.into(),
        filled: true,
        stroke_width: 2,
    };

    let y_offset = s.y_offset;
    let y_axis_size = s.y_axis_size;
    let titles = &s.column_titles;

    for (index, hdp) in hdps.iter().enumerate() {
        let doy = index + y_offset as usize;
        for todo in &hdp.todo_performance {
            match todo.performance() {
                Performance::Fail => {
                    style = ShapeStyle {
                        color: GREY_700.into(),
                        filled: true,
                        stroke_width: 2,
                    };
                }
                Performance::Done => {
                    style = ShapeStyle {
                        color: GREEN_200.into(),
                        filled: true,
                        stroke_width: 2,
                    };
                }
                _ => {}
            };
            let td_title = match todo.task() {
                Task::Todo(s) => s,
                _ => panic!("expected todo"),
            };
            let title_index = titles.iter().position(|e| e == &td_title).unwrap();
            // println!("y_offset {}", y_offset);
            // println!("day of year {}", doy);
            // println!("todo title {}, title index {}", td_title, title_index);
            let left_lower_x = title_index;
            let left_lower_y = ((doy as i32 - 1) - y_axis_size as i32).abs() as usize;
            let right_upper_x = title_index + 1;
            let right_upper_y = (doy as i32 - y_axis_size as i32).abs() as usize;
            // println!(
            //    "llx {}, lly {}, rux {}, ruy {}\n\n",
            //   left_lower_x, left_lower_y, right_upper_x, right_upper_y
            // );

            let rect = Rectangle::new(
                [
                    // left lower
                    // x = titleindex, y = (day of year - 1) - y axis length
                    (
                        SegmentValue::Exact(left_lower_x as i32),
                        SegmentValue::Exact(left_lower_y as i32),
                    ),
                    // right upper
                    // x = titleindex + 1, y = day of year - y axis length
                    (
                        SegmentValue::Exact(right_upper_x as i32),
                        SegmentValue::Exact(right_upper_y as i32),
                    ),
                ],
                style,
            );
            chart.plotting_area().draw(&rect).unwrap();
        }
    }

    chart
}

pub fn draw_grid_lines<'a>(
    chart: HeatmapSettings<'a>,
    settings: &TimeFrame<TodoTimeFrameSettings<'a>>,
) -> HeatmapSettings<'a> {
    let s: &TodoTimeFrameSettings = match settings {
        TimeFrame::Year(y) => y,
        TimeFrame::Month(m) => m,
    };
    let x_axis_size = s.x_axis_size as i32;
    let y_axis_size = s.y_axis_size as i32;

    let drawing_area = chart.plotting_area();
    let black_thin = ShapeStyle {
        color: BLACK.into(),
        filled: false,
        stroke_width: 1,
    };

    // Draw vertical grid lines at integer positions
    for x in 0..=x_axis_size + 1 {
        drawing_area
            .draw(&PathElement::new(
                [
                    (SegmentValue::Exact(x), SegmentValue::Exact(0)),
                    (SegmentValue::Exact(x), SegmentValue::Exact(y_axis_size + 1)),
                ],
                black_thin,
            ))
            .unwrap();
    }

    // Draw horizontal grid lines at integer positions
    for y in 0..=y_axis_size {
        drawing_area
            .draw(&PathElement::new(
                [
                    (SegmentValue::Exact(0), SegmentValue::Exact(y)),
                    (SegmentValue::Exact(x_axis_size + 1), SegmentValue::Exact(y)),
                ],
                black_thin,
            ))
            .unwrap();
    }
    chart
}
