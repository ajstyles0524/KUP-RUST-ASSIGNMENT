/// Score Structure
pub struct Score {
    hindi: f32,
    english: f32,
    maths: f32,
    science: f32,
}
/// Student Structure
pub struct Student {
    name: String,
    roll_no: i32,
    scoreofeachsubject: Score,
    department: String,
    school: String,
}

impl Student {
    /// The new method initializes Student objects
    ///
    /// #Arguments
    ///
    /// Student Structure
    ///
    /// #Return
    ///
    /// Return the Student type objects
    fn new(
        string: String,
        roll: i32,
        score_subject: Score,
        department_name: String,
        school_name: String,
    ) -> Student {
        Student {
            name: string,
            roll_no: roll,
            scoreofeachsubject: score_subject,
            department: department_name,
            school: school_name,
        }
    }
    /// compare_student method is to print difference of each subject's score b/w students
    ///
    /// #Arguments
    ///
    /// Student Structure
    ///
    /// #Return
    ///
    /// No Return
    fn compare_student(&self, another_student: &Student) {
        println!(
            "Difference of Hindi score is {}",
            self.scoreofeachsubject.hindi - another_student.scoreofeachsubject.hindi
        );
        println!(
            "Difference of English score is {}",
            self.scoreofeachsubject.english - another_student.scoreofeachsubject.english
        );
        println!(
            "Difference of Maths score is {}",
            self.scoreofeachsubject.maths - another_student.scoreofeachsubject.maths
        );
        println!(
            "Difference of Science score is {}",
            self.scoreofeachsubject.science - another_student.scoreofeachsubject.science
        );
    }
}

impl Score {
    /// get_average method is to get average of all scores
    ///
    /// #Arguments
    ///
    /// Score Structure
    ///
    /// #Return
    ///
    /// Return the average of marks
    fn get_average(&self) -> f32 {
        (self.hindi + self.english + self.maths + self.science) / 4.0
    }
    /// pass_student method add numbers to student's score if score < 35
    ///
    /// #Arguments
    ///
    /// Score Structure
    ///
    /// #Return
    ///
    /// No return
    fn pass_student(&mut self) {
        let passing_score = 35.0;
        let mut less_score: f32 = passing_score - self.hindi;
        if less_score > 0.0 {
            self.hindi += less_score;
            println!(
                "Updated Hindi score is {} and less score is {}",
                self.hindi, less_score
            );
        }
        less_score = passing_score - self.english;
        if less_score > 0.0 {
            self.english  += less_score;
            println!(
                "Updated English score is {} and less score is {}",
                self.english, less_score
            );
        }
        less_score = passing_score - self.maths;
        if less_score > 0.0 {
            self.maths += less_score;
            println!(
                "Updated Maths score is {} and less score is {}",
                self.maths, less_score
            );
        }
        less_score = passing_score - self.science;
        if less_score > 0.0 {
            self.science  +=  less_score;
            println!(
                "Updated Science score is {} and less score is {}",
                self.science, less_score
            );
        }
    }
}

/// main function
fn main() {
    /// Student-1
    let student_1 = Student::new(
        "Prem".to_string(),
        77,
        Score {
            hindi: 31.5,
            english: 32.5,
            maths: 33.5,
            science: 34.0,
        },
        "Civil".to_string(),
        "JRS".to_string(),
    );
    /// Student-2
    let student_2 = Student::new(
        "Prem".to_string(),
        77,
        Score {
            hindi: 32.5,
            english: 25.5,
            maths: 25.5,
            science: 30.0,
        },
        "Civil".to_string(),
        "JRS".to_string(),
    );
    /// To get average of all scores.
    println!(
        "average of all scores is {}",
        Score {
            hindi: 31.5,
            english: 32.5,
            maths: 33.5,
            science: 34.0,
        }
        .get_average()
    );
    /// Add numbers to student’s subject score if score is less than 35.
    Score {
        hindi: 31.5,
        english: 32.5,
        maths: 33.5,
        science: 34.0,
    }
    .pass_student();
    /// Print difference of each subject’s score.
    Student::compare_student(&student_1, &student_2);
}
