use crate::time_stuff::{days_in_month, CurrentDate};
use regex::Regex;

#[derive(Clone)]
pub enum Performance {
    Done,
    Fail,
    Score(f32),
}

#[derive(Clone)]
pub enum Task {
    Todo(String),
    Objective(String),
}

#[derive(Clone)]
pub struct TaskPerformance((Task, Performance));

impl TaskPerformance {
    pub fn task(&self) -> Task {
        self.0 .0.clone()
    }
    pub fn performance(&self) -> Performance {
        self.0 .1.clone()
    }
}

#[derive(Clone)]
pub struct HabitDayPerformance {
    pub year: String,
    pub month: String,
    pub day: String,
    pub todo_performance: Vec<TaskPerformance>,
    pub objective_performance: Vec<TaskPerformance>,
}

pub struct HabitProgress(pub Vec<HabitDayPerformance>);

impl HabitProgress {
    pub fn hpds(&self) -> &Vec<HabitDayPerformance> {
        &self.0
    }

    pub fn filter_by_month(&self, month: String) -> Vec<HabitDayPerformance> {
        let hdps = &self.0;
        let monthly_hdps = hdps
            .iter()
            .filter(|x| x.month == month)
            .map(|x| x.clone())
            .collect::<Vec<HabitDayPerformance>>();
        monthly_hdps
    }

    pub fn oldest_month_hdp(&self, month: String) -> HabitDayPerformance {
        let hdps = &self.0;
        let month_hdps: Vec<&HabitDayPerformance> =
            hdps.iter().filter(|x| x.month == month).collect();
        if let Some(youngest_hdps) = month_hdps
            .iter()
            .min_by_key(|x| x.day.parse::<i32>().unwrap())
        {
            return youngest_hdps.to_owned().to_owned();
        } else {
            panic!("youngest not found");
        }
    }

    pub fn doc_titles(&self) -> HashSet<(String, String)> {
        let mut year_months: HashSet<(String, String)> = HashSet::new();
        let hdps = &self.0;
        for hdp in hdps.iter() {
            let year = hdp.year.clone();
            let month = hdp.month.clone();
            year_months.insert((year, month));
        }
        year_months
    }

    pub fn get_current_hdp(&self) -> HabitDayPerformance {
        let hdps = &self.0;
        let len = hdps.len();
        let current = &hdps[len - 1];
        let clone = current.clone();
        clone
    }
    pub fn get_oldest_hdp(&self) -> HabitDayPerformance {
        let hdps = &self.0;
        let oldest = &hdps[0];
        let clone = oldest.clone();
        clone
    }

    pub fn ordered_titles<'a>() -> Vec<&'a str> {
        let t: Vec<&str> = vec![
            "#1 Red Light Session",
            "#1 IQoro Mouth Exercise",
            "#1 Brush Teeth",
            "#1 Track Meals",
            "#1 Walk",
            "#2 IQoro Mouth Exercise",
            "#2 Track Meals",
            "#2 Walk",
            "Swim",
            "Stretch",
            "#3 IQoro Mouth Exercise",
            "#3 Track Meals",
            "#3 Walk",
            "#2 Red Light Session",
            "#2 Brush Teeth",
            "Anki Ukrainian Lesson",
            "Pray",
            "Sauna",
            "Parasym",
        ];
        t
    }
    pub fn all_unique_todo_titles(&self) -> Vec<String> {
        let all_hdp = &self.0;

        let mut all_todo_titles: Vec<String> = vec![];

        for hdp in all_hdp {
            let tasks: Vec<String> = hdp
                .todo_performance
                .iter()
                .map(|h| match h.task() {
                    Task::Todo(title) => title,
                    _ => {
                        panic!("todo title is not of type Todo")
                    }
                })
                .collect();
            all_todo_titles.extend(tasks);
        }

        let deduped: Vec<String> = all_todo_titles
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        deduped
    }
}

use std::{collections::HashSet, fmt, mem};

// Implement Display for Task
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Task::Todo(s) => write!(f, "Todo: {}", s),
            Task::Objective(s) => write!(f, "Objective: {}", s),
        }
    }
}

// Implement Display for Performance
impl fmt::Display for Performance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Performance::Done => write!(f, "Done"),
            Performance::Fail => write!(f, "Failed"),
            Performance::Score(score) => write!(f, "Score: {:.1}", score),
        }
    }
}

// Implement Display for TaskPerformance
impl fmt::Display for TaskPerformance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → {}", self.task(), self.performance())
    }
}

// Implement Display for HabitDayPerformance
impl fmt::Display for HabitDayPerformance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Date: {}-{}-{}", self.year, self.month, self.day)?;

        writeln!(f, "Todos:")?;
        for (i, task) in self.todo_performance.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, task)?;
        }

        writeln!(f, "Objectives:")?;
        for (i, obj) in self.objective_performance.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, obj)?;
        }

        Ok(())
    }
}

// Finally, implement Display for HabitProgress
impl fmt::Display for HabitProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Habit Progress ===")?;

        for (i, day) in self.0.iter().enumerate() {
            if i > 0 {
                writeln!(f, "-------------------")?;
            }
            write!(f, "{}", day)?;
        }

        Ok(())
    }
}

pub fn process_org_file(content: String) -> Vec<Vec<TaskPerformance>> {
    // Parse the Org file
    // println!("{}", content);
    let tp = {
        let mut todo_performance: Vec<TaskPerformance> = Vec::new();
        let re = Regex::new(r"\*.*(DONE|FAIL) /HABIT/ (.*)").unwrap();
        for cap in re.captures_iter(&content) {
            let (_, [perf, title]) = cap.extract();
            // println!("{:?}", perf);
            // println!("{:?}", title);
            let t = Task::Todo(title.to_string());
            let p = match perf {
                "FAIL" => Performance::Fail,
                "DONE" => Performance::Done,
                _ => Performance::Fail,
            };
            todo_performance.push(TaskPerformance((t, p)));
        }
        todo_performance
    };

    let op = {
        let mut objective_performance: Vec<TaskPerformance> = Vec::new();
        let ot_re = Regex::new(r"/OBJECTIVE/ (.*)").unwrap();
        for cap in ot_re.captures_iter(&content) {
            let (_, [objective_title]) = cap.extract();
            // println!("{:?}", objective_title);
            let otrp = objective_title.replace("(", r"\(");
            let final_otrp = otrp.replace(")", r"\)");
            let prefix = r"/OBJECTIVE/ ";
            let suffix = r".*\n{1,3}.*- (\d{1,3}\.\d{1}|\d{1,3})";
            let ov_re_str = format!("{}{}{}", prefix, final_otrp, suffix);
            // println!("{:?}", ov_re_str);
            let rds = ov_re_str.replace(r"\\", r"\");
            let ov_re = Regex::new(&rds).unwrap();
            for v_cap in ov_re.captures_iter(&content) {
                let (_, [objective_value]) = v_cap.extract();
                let ov: f32 = objective_value.to_string().parse().unwrap();
                let e = TaskPerformance((
                    Task::Objective(objective_title.to_string()),
                    Performance::Score(ov),
                ));
                objective_performance.push(e);
            }
        }
        objective_performance
    };

    vec![tp, op]
}

pub fn collect_org_data() -> HabitProgress {
    let months = [
        "January",
        "Feburary",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let start_date = 22;
    let start_month = 2;
    let start_year = 2025;
    let cd = CurrentDate::new();
    let current_year: usize = cd.year.parse().unwrap();
    let current_month = cd.month;
    let current_day: usize = cd.day.parse().unwrap();
    let year_len = (current_year - start_year + 1);

    let mut habit_progress: Vec<HabitDayPerformance> = vec![];

    for year in 0..year_len {
        for month in months {
            let mut last_year = false;
            if year_len - 1 == year {
                last_year = true;
            }

            let days = days_in_month(months[start_month]);

            for day_index in 0..days.len() {
                // println!("{day_index}");

                let mut day_string = "01".to_string();

                let day = days[day_index];

                if day < 10 {
                    day_string = format!("0{day}");
                } else {
                    day_string = format!("{day}");
                }

                let string_year_val = start_year + year;
                let file_path = format!(
                    "/home/test/code/notes/habits/{string_year_val}/{month}/{day_string}.org"
                );
                // println!("{file_path}");

                if let Ok(file_contents) = std::fs::read_to_string(file_path) {
                    let mut p_vec = process_org_file(file_contents);
                    let todos = mem::take(&mut p_vec[0]);
                    let objectives = mem::take(&mut p_vec[1]);
                    let hdp_year = string_year_val.to_string();
                    let hdp_month = month.to_string();
                    let hdp_day = day_string;

                    let hdp = HabitDayPerformance {
                        todo_performance: todos,
                        objective_performance: objectives,
                        year: hdp_year,
                        month: hdp_month,
                        day: hdp_day,
                    };
                    habit_progress.push(hdp);
                    // println!("file exists");
                } else {
                    // println!("file does not exist");
                }
            }

            if month == current_month && last_year {
                break;
            }
        }
    }
    HabitProgress(habit_progress)
}
