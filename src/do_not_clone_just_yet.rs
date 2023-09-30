use itertools::Itertools;
use std::collections::HashMap;

#[derive(Clone)]
struct Employee {
    id: String,
    name: String,
    age: usize,
}

struct Benefits(usize);

const RETIREMENT_AGE: usize = 60;

fn initial_compute_monthly_allowance(employees: Vec<Employee>) -> HashMap<String, Benefits> {
    let retiring_employees = employees
        .clone()
        .into_iter()
        .filter(|employee| employee.age >= RETIREMENT_AGE)
        .collect_vec();
    let mut retiring_employee_benefits = {
        // Some logic for the old timers
        drop(retiring_employees);
        HashMap::new()
    };
    let non_retiring_employees = employees
        .into_iter()
        .filter(|employee| employee.age < RETIREMENT_AGE)
        .collect_vec();
    let non_retiring_employee_benefits = {
        // logic for the new employees
        drop(non_retiring_employees);
        HashMap::new()
    };

    retiring_employee_benefits.extend(non_retiring_employee_benefits);
    retiring_employee_benefits
}

fn better_compute_monthly_allowance(employees: Vec<Employee>) -> HashMap<String, Benefits> {
    let retiring_employees = employees
        .iter()
        .filter(|employee| employee.age >= RETIREMENT_AGE)
        .cloned()
        .collect_vec();

    let mut retiring_employee_benefits = {
        // Some logic for the old timers
        drop(retiring_employees);
        HashMap::new()
    };

    let non_retiring_employees = employees
        .into_iter()
        .filter(|employee| employee.age < RETIREMENT_AGE)
        .collect_vec();
    let non_retiring_employee_benefits = {
        // logic for the new employees
        drop(non_retiring_employees);
        HashMap::new()
    };

    retiring_employee_benefits.extend(non_retiring_employee_benefits);
    retiring_employee_benefits
}
