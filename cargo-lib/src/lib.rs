use std::collections::HashMap;
pub trait Office {
    fn new() -> Self
    where
        Self: Sized;

    fn add_employees(&mut self, name: &str, department: &str, salary: f64);

    fn get_salary_of_employee(&self, name: &str, department: &str) -> Option<f64>;

    fn get_all_department_in_company(&self) -> Vec<String>;

    fn get_all_employees_in_company(&self) -> Vec<String>;

    fn get_all_salary_of_employee(&self) -> Option<f64>;

    fn average_salary(&self) -> Option<f64>;

    fn average_salary_of_department(&self, department: &str) -> Option<f64>;

    fn min_salary_of_department(&self, department: &str) -> Option<f64>;

    fn max_salary_of_department(&self, department: &str) -> Option<f64>;

    fn min_salary_of_company(&self) -> Option<f64>;

    fn max_salary_of_company(&self) -> Option<f64>;
}

#[derive(Debug)]
pub struct Company {
    employees: HashMap<String, HashMap<String, f64>>,
    manager: HashMap<String, Vec<String>>,
}

impl Office for Company {
    fn new() -> Company {
        let company = Company {
            employees: HashMap::new(),
            manager: HashMap::new(),
        };
        return company;
    }

    fn add_employees(&mut self, name: &str, department: &str, salary: f64) {
        let employees = self
            .employees
            .entry(name.to_string())
            .or_insert(HashMap::new());
        employees.insert(department.to_string(), salary);

        let manager = self.manager.entry(department.to_string()).or_insert(vec![]);
        manager.push(name.to_string());
    }

    fn get_salary_of_employee(&self, name: &str, department: &str) -> Option<f64> {
        match self.employees.get(name) {
            Some(department_employee) => department_employee.get(department).map(|&salary| salary),
            None => None,
        }
    }

    fn get_all_employees_in_company(&self) -> Vec<String> {
        let all_employees = self.employees.keys().cloned().collect();
        return all_employees;
    }

    fn get_all_department_in_company(&self) -> Vec<String> {
        let all_department = self.manager.keys().cloned().collect();
        return all_department;
    }

    fn get_all_salary_of_employee(&self) -> Option<f64> {
        let total_salary: f64 = self.employees.values().flat_map(|dept| dept.values()).sum();
        if total_salary > 0.0 {
            Some(total_salary as f64)
        } else {
            None
        }
    }

    fn average_salary(&self) -> Option<f64> {
        let total_salary: f64 = self.employees.values().flat_map(|dept| dept.values()).sum();
        let total_employee = self.employees.len() as f64;
        if total_employee > 0.0 {
            let avarage: f64 = total_salary / total_employee;

            Some(avarage)
        } else {
            None
        }
    }

    fn average_salary_of_department(&self, department: &str) -> Option<f64> {
        if let Some(employee_of_department) = self.manager.get(department) {
            let total_menber = employee_of_department.len() as f64;
            let total_salary: f64 = employee_of_department
                .iter()
                .filter_map(|employee| self.employees.get(employee))
                .filter_map(|dept| dept.get(department))
                .sum();
            if total_menber != 0.0 && total_salary != 0.0 {
                let return_value = total_salary / total_menber;
                Some(return_value)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn min_salary_of_department(&self, department: &str) -> Option<f64> {
        if let Some(employee_department) = self.manager.get(department) {
            let min_salary = employee_department
                .iter()
                .filter_map(|employee| self.employees.get(employee))
                .filter_map(|dept| dept.get(department))
                .fold(f64::INFINITY, |min, &salary| min.min(salary));
            if min_salary != 0.0 {
                Some(min_salary)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn max_salary_of_department(&self, department: &str) -> Option<f64> {
        if let Some(employee_department) = self.manager.get(department) {
            let max_salary = employee_department
                .iter()
                .filter_map(|employee| self.employees.get(employee))
                .filter_map(|dept| dept.get(department))
                .fold(f64::NEG_INFINITY, |max, &salary| max.max(salary));
            if max_salary != 0.0 {
                Some(max_salary)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn min_salary_of_company(&self) -> Option<f64> {
        let min_salary = self
            .employees
            .values()
            .flat_map(|dept| dept.values())
            .fold(f64::INFINITY, |min, &salary| min.min(salary));
        if min_salary.is_finite() {
            Some(min_salary)
        } else {
            None
        }
    }

    fn max_salary_of_company(&self) -> Option<f64> {
        let max_salary = self
            .employees
            .values()
            .flat_map(|dept| dept.values())
            .fold(f64::NEG_INFINITY, |max, &salary| max.max(salary));
        if max_salary.is_finite() {
            Some(max_salary)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_employees() {
        let mut company: Company = Company::new();
        company.add_employees("Le Hong Vo", "IT", 100.0);

        let get_employees_salary = company.get_salary_of_employee("Le Hong Vo", "IT").unwrap();
        assert_eq!(get_employees_salary, 100.0, "Invalid value")
    }
    
}