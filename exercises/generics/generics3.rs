// An imaginary magical school has a new report card generation system written in rust!
// Currently the system only supports creating report cards where the student's grade 
// is represented numerically (e.g. 1.0 -> 5.5). However, the school also issues alphabetical grades
// (A+ -> F-) and needs to be able to print both types of report card!

// Make the necessary code changes to support alphabetical report cards, thereby making the second
// test pass.

pub enum Grade{
    Num(f32),
    Alpha(String),
}

pub struct ReportCard {
    pub grade: Grade,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, &self.grade)
    }
}

impl std::fmt::Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Grade::Num(n) => write!(f, "{}", n),
            Grade::Alpha(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Num(2.1),
            student_name: "Tom Wriggle".to_string(), 
            student_age: 12,
        };
        assert_eq!(report_card.print(), "Tom Wriggle (12) - achieved a grade of 2.1");
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: Grade::Alpha(String::from("A+")),
            student_name: "Gary Plotter".to_string(), 
            student_age: 11,
        };
        assert_eq!(report_card.print(), "Gary Plotter (11) - achieved a grade of A+");
    }
}