use itertools::Itertools;

#[derive(Clone)]
struct FullStudentData {
    name: String,
    weight: usize,
    age: usize,
    grade: u8,
    home_address: String,
}

// used by the transport department
struct StudentTransportationDTO {
    name: String,
    home_address: String,
}

fn initial_transport_students_belonging_to_grade(
    students: Vec<FullStudentData>,
    grade: u8,
) -> Vec<StudentTransportationDTO> {
    students
        .iter()
        .filter(|student| student.grade == grade)
        .cloned()
        .map(|student| StudentTransportationDTO {
            name: student.name,
            home_address: student.home_address,
        })
        .collect_vec()
}

// just clone the fields you need
fn better_transport_students_belonging_to_grade(
    students: Vec<FullStudentData>,
    grade: u8,
) -> Vec<StudentTransportationDTO> {
    students
        .iter()
        .filter(|student| student.grade == grade)
        .map(|student| StudentTransportationDTO {
            name: student.name.clone(),
            home_address: student.home_address.clone(),
        })
        .collect_vec()
}

fn even_better_transport_students_belonging_to_grade(
    students: Vec<FullStudentData>,
    grade: u8,
) -> Vec<StudentTransportationDTO> {
    students
        .iter()
        .filter_map(|student| {
            if student.grade == grade {
                Some(StudentTransportationDTO {
                    name: student.name.clone(),
                    home_address: student.home_address.clone(),
                })
            } else {
                None
            }
        })
        .collect_vec()
}
