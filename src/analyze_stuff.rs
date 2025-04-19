use crate::data_stuff::{collect_org_data, HabitDayPerformance, HabitProgress};
use crate::draw_stuff::heatmap::draw_grid_lines;
use crate::draw_stuff::linechart;
use crate::time_stuff::{day_of_year, days_in_month};

use crate::draw_stuff::heatmap::{build_chart, draw_rect, style_chart};
use plotters::coord::Shift;
use plotters::prelude::*;

pub enum TimeFrame<T> {
    Month(T),
    Year(T),
}

pub struct TodoTimeFrameSettings<'a> {
    pub tf_name: String,
    pub filename: String,
    pub doc_title: String,
    pub column_titles: Vec<&'a str>,
    pub y_axis_size: u32,
    pub x_axis_size: u32,
    pub y_offset: u32,
}

pub struct ObjTimeFrameSettings {
    pub tf_name: String,
    pub filename: String,
    pub doc_title: String,
    pub month_name: Option<String>,
    pub y_title: String,
    pub y_range: (f32, f32),
    pub x_axis_size: u32,
    pub x_offset: u32,
}

pub struct Analyzer {
    org_data: HabitProgress,
}

impl Analyzer {
    pub fn new() -> Self {
        let org_data = collect_org_data();
        Analyzer { org_data }
    }

    fn get_monthly_objective_settings(
        &self,
        y_title: String,
        y_range: (f32, f32),
        tf_name: String,
        filename_pre: String,
    ) -> Vec<TimeFrame<ObjTimeFrameSettings>> {
        let mut settings = Vec::<TimeFrame<ObjTimeFrameSettings>>::new();
        let org_data = &self.org_data;
        let doc_titles = org_data.doc_titles();
        for doc_title in doc_titles.iter() {
            let (year, month) = doc_title;
            let days_in_month = days_in_month(month);
            let x_axis_size = days_in_month.last().unwrap().to_owned();
            let oldest_hdp = org_data.oldest_month_hdp(month.to_owned());
            let x_offset = oldest_hdp.day.parse::<u32>().unwrap();
            let doc_title = format!("{month} {year}");
            let filename = format!("{filename_pre}-{month}-{year}.png");

            let tf = ObjTimeFrameSettings {
                tf_name: tf_name.clone(),
                filename,
                doc_title,
                month_name: Some(month.to_string()),
                y_title: y_title.clone(),
                y_range,
                x_axis_size,
                x_offset,
            };

            settings.push(TimeFrame::Month(tf));
        }
        settings
    }

    fn get_yearly_objective_settings(
        &self,
        y_title: String,
        y_range: (f32, f32),
        tf_name: String,
        filename: String,
    ) -> TimeFrame<ObjTimeFrameSettings> {
        let org_data = &self.org_data;
        let doc_title = y_title.clone();
        let todays_progress = org_data.get_current_hdp();
        let oldest_progress = org_data.get_oldest_hdp();
        let doy_offset = day_of_year(&oldest_progress);
        let day_of_year_current_hdp = day_of_year(&todays_progress);
        let x_axis_size = day_of_year_current_hdp;
        let x_offset = doy_offset;

        let tf = ObjTimeFrameSettings {
            tf_name,
            filename,
            doc_title,
            month_name: None,
            y_title,
            y_range,
            x_axis_size,
            x_offset,
        };
        TimeFrame::Year(tf)
    }

    fn get_yearly_settings<'a>(&self) -> TimeFrame<TodoTimeFrameSettings<'a>> {
        let org_data = &self.org_data;
        let column_titles = HabitProgress::ordered_titles();
        let todays_progress = org_data.get_current_hdp();
        let oldest_progress = org_data.get_oldest_hdp();
        let doy_offset = day_of_year(&oldest_progress);
        let day_of_year_current_hdp = day_of_year(&todays_progress);
        let x_axis_size = (column_titles.len() - 1) as u32;
        let y_axis_size = day_of_year_current_hdp;
        let y_offset = doy_offset;
        let filename = "yearly_habit_performance.png".to_string();

        let tf = TodoTimeFrameSettings {
            tf_name: "2025".to_string(),
            filename,
            doc_title: "2025".to_string(),
            column_titles,
            y_axis_size,
            x_axis_size,
            y_offset,
        };
        TimeFrame::Year(tf)
    }

    fn get_monthly_settings<'a>(&self) -> Vec<TimeFrame<TodoTimeFrameSettings<'a>>> {
        let mut settings = Vec::<TimeFrame<TodoTimeFrameSettings<'a>>>::new();
        let org_data = &self.org_data;
        let doc_titles = org_data.doc_titles();
        let column_titles = HabitProgress::ordered_titles();
        let x_axis_size = (column_titles.len() - 1) as u32;
        for doc_title in doc_titles.iter() {
            let (year, month) = doc_title;
            let days_in_month = days_in_month(month);
            let y_axis_size = days_in_month.last().unwrap().to_owned();
            let oldest_hdp = org_data.oldest_month_hdp(month.to_owned());
            let y_offset = oldest_hdp.day.parse::<u32>().unwrap();
            let filename = format!("{year}-{month}.png");
            let doc_title = format!("{month} {year}");

            println!("monthly\ny axis size: {y_axis_size}, x_axis_size: {x_axis_size}");
            let tf = TodoTimeFrameSettings {
                tf_name: month.to_owned(),
                filename,
                doc_title,
                column_titles: column_titles.clone(),
                y_axis_size,
                x_axis_size,
                y_offset,
            };

            settings.push(TimeFrame::Month(tf));
        }
        settings
    }

    fn draw_todos(&self, settings: &TimeFrame<TodoTimeFrameSettings>) {
        let (hdps, filename): (&Vec<HabitDayPerformance>, String) = {
            match &settings {
                TimeFrame::Year(s) => (self.org_data.hpds(), s.filename.clone()),
                TimeFrame::Month(s) => (
                    &self.org_data.filter_by_month(s.tf_name.clone()),
                    s.filename.clone(),
                ),
            }
        };
        println!("{filename}");
        // Draw, Size, Style Canvas
        let root: DrawingArea<BitMapBackend, Shift> =
            BitMapBackend::new(&filename, (1000, 1000)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        // Set chart type
        let mut chart = build_chart(&root, settings).unwrap();

        // Set chart style
        chart = style_chart(chart, settings).unwrap();

        // Draw org data onto chart
        chart = draw_rect(chart, hdps, settings);

        // Draw custom gridlines
        let _chart = draw_grid_lines(chart, settings);

        root.present().unwrap();
    }

    pub fn monthly_habit_performance(&self) -> Result<String, String> {
        let settings = &self.get_monthly_settings();
        for tf in settings.iter() {
            let _ = &self.draw_todos(tf);
        }
        Ok("completed monthly analysis".to_string())
    }

    pub fn yearly_habit_performance(&self) -> Result<String, String> {
        let settings = &self.get_yearly_settings();
        let _ = &self.draw_todos(settings);

        Ok("completed yearly analysis".to_string())
    }

    pub fn draw_objectives(&self, settings: &TimeFrame<ObjTimeFrameSettings>) {
        let (hdps, filename): (&Vec<HabitDayPerformance>, String) = {
            match &settings {
                TimeFrame::Year(s) => (self.org_data.hpds(), s.filename.clone()),
                TimeFrame::Month(s) => (
                    &self.org_data.filter_by_month(s.month_name.clone().unwrap()),
                    s.filename.clone(),
                ),
            }
        };

        let root = BitMapBackend::new(&filename, (1800, 1400)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.margin(10, 10, 10, 10);

        // After this point, we should be able to construct a chart context
        let mut chart = linechart::build_chart(&root, settings).unwrap();

        // Then we can draw a mesh
        chart = linechart::style_chart(chart, settings);

        let _chart = linechart::draw_data(chart, hdps, settings);

        root.present().unwrap();
    }

    pub fn monthly_objective_performance(&self) -> Result<String, String> {
        let settings = &self.get_monthly_objective_settings(
            "Weight".to_string(),
            (50.0, 100.0),
            "Weight=".to_string(),
            "monthly_objective_performance_weight".to_string(),
        );
        for tf in settings {
            let _ = &self.draw_objectives(tf);
        }
        let g_settings = &self.get_monthly_objective_settings(
            "GERD Symptoms".to_string(),
            (0.0, 10.0),
            "GERD-Symptoms(0-10)=".to_string(),
            "monthly_objective_performance_gerd".to_string(),
        );
        for tf in g_settings {
            let _ = &self.draw_objectives(tf);
        }
        let m_settings = &self.get_monthly_objective_settings(
            "Mood".to_string(),
            (0.0, 10.0),
            "Mood(0-10)=".to_string(),
            "monthly_objective_performance_mood".to_string(),
        );
        for tf in g_settings {
            let _ = &self.draw_objectives(tf);
        }
        Ok("completed monthly objective report".to_string())
    }

    pub fn yearly_objective_performance(&self) -> Result<String, String> {
        // Weight Chart
        let settings = &self.get_yearly_objective_settings(
            "Weight".to_string(),
            (50.0, 100.0),
            "Weight=".to_string(),
            "yearly_objective_performance_weight.png".to_string(),
        );

        // Gerd Symptoms Chart
        let g_settings = &self.get_yearly_objective_settings(
            "GERD Symptoms".to_string(),
            (0.0, 10.0),
            "GERD-Symptoms(0-10)=".to_string(),
            "yearly_objective_performance_gerd.png".to_string(),
        );

        // Mood Chart
        let m_settings = &self.get_yearly_objective_settings(
            "Mood".to_string(),
            (0.0, 10.0),
            "Mood(0-10)=".to_string(),
            "yearly_objective_performance_mood.png".to_string(),
        );
        let _ = &self.draw_objectives(settings);
        let _ = &self.draw_objectives(g_settings);
        let _ = &self.draw_objectives(m_settings);
        Ok("completed yearly objective analysis".to_string())
    }
}
