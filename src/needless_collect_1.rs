// There's no need to collect, if you then need to join the initial results with the results of the next computation. Make use of .chain()

use itertools::Itertools;

#[derive(Clone)]
struct Employee {
    name: String,
    age: usize,
    current_salary: usize,
}

struct TotalExpenditure(usize);

const RETIREMENT_AGE: usize = 60;
const RETIREMENT_BENEFITS_MULTIPLIER: usize = 10;

fn initial_total_monthly_expenditure(employees: Vec<Employee>) -> TotalExpenditure {
    let retiring_employees = employees
        .iter()
        .filter(|employee| employee.age >= RETIREMENT_AGE)
        .cloned()
        .collect_vec();

    let retiring_employee_benefits = {
        // Some logic for the old timers
        retiring_employees
            .iter()
            .map(|employee| employee.current_salary + employee.age * RETIREMENT_BENEFITS_MULTIPLIER)
            .sum::<usize>()
    };

    let non_retiring_employees = employees
        .iter()
        .filter(|employee| employee.age < RETIREMENT_AGE)
        .cloned()
        .collect_vec();

    let non_retiring_employee_benefits = {
        // logic for the young employees
        non_retiring_employees
            .iter()
            .map(|employee| employee.current_salary + employee.age)
            .sum::<usize>()
    };

    let all_benefits = retiring_employee_benefits + non_retiring_employee_benefits;

    TotalExpenditure(all_benefits)
}

fn better_total_monthly_expenditure(employees: &[Employee]) -> TotalExpenditure {
    let retiring_employees = employees
        .iter()
        .filter(|employee| employee.age >= RETIREMENT_AGE);

    let retiring_employee_benefits = {
        // Some logic for the old timers
        retiring_employees
            .map(|employee| employee.current_salary + employee.age * RETIREMENT_BENEFITS_MULTIPLIER)
            .sum::<usize>()
    };

    let non_retiring_employees = employees
        .iter()
        .filter(|employee| employee.age < RETIREMENT_AGE);

    let non_retiring_employee_benefits = {
        // logic for the young employees
        non_retiring_employees
            .map(|employee| employee.current_salary + employee.age)
            .sum::<usize>()
    };

    TotalExpenditure(retiring_employee_benefits + non_retiring_employee_benefits)
}
