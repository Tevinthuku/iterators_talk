use std::collections::HashMap;

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Clone)]
struct StudentId(String);

#[derive(Clone)]
struct Student {
    id: StudentId,
    name: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Score(usize);

#[derive(Clone, Copy)]
struct SubjectScore(usize);

impl Student {
    fn total_scores(&self, subject_score_sheet: &HashMap<StudentId, Vec<SubjectScore>>) -> Score {
        let total_score = subject_score_sheet
            .get(&self.id)
            .cloned()
            .map(|scores| scores.iter().map(|item| item.0).sum::<usize>())
            .unwrap_or_default();

        Score(total_score)
    }
}

fn initial_get_best_scoring_student(
    students: Vec<Student>,
    subject_score_sheet: HashMap<StudentId, Vec<SubjectScore>>,
) -> Option<(Student, Score)> {
    let sorted_by_score = students
        .into_iter()
        .map(|student| {
            let score = student.total_scores(&subject_score_sheet);

            (student, score)
        })
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .collect_vec();

    sorted_by_score.first().cloned()
}

fn better_get_best_scoring_student(
    students: Vec<Student>,
    subject_score_sheet: HashMap<StudentId, Vec<SubjectScore>>,
) -> Option<(Student, Score)> {
    students
        .into_iter()
        .map(|student| {
            let score = student.total_scores(&subject_score_sheet);

            (student, score)
        })
        .sorted_by(|a, b| b.1.cmp(&a.1))
        .next()
}
