// use std::collections::HashMap;

// trait Office {
//     fn new() -> Self
//     where
//         Self: Sized;

//     fn add_employees(&mut self, name: &str, department: &str, salary: f64);

//     fn get_salary_of_employee(&self, name: &str, department: &str) -> Option<f64>;

//     fn get_all_department_in_company(&self) -> Vec<String>;

//     fn get_all_employees_in_company(&self) -> Vec<String>;

//     fn get_total_salary_of_employee(&self) -> Option<f64>;

//     fn average_salary(&self) -> Option<f64>;

//     fn average_salary_of_department(&self, department: &str) -> Option<f64>;

//     fn min_salary_of_department(&self, department: &str) -> Option<f64>;

//     fn max_salary_of_department(&self, department: &str) -> Option<f64>;

//     fn min_salary_of_company(&self) -> Option<f64>;

//     fn max_salary_of_company(&self) -> Option<f64>;
// }

// #[derive(Debug)]
// struct Company {
//     employees: HashMap<String, HashMap<String, f64>>,
//     manager: HashMap<String, Vec<String>>,
// }

// impl Office for Company{
//     fn new() -> Company {
//         let company = Company {
//             employees: HashMap::new(),
//             manager: HashMap::new(),
//         };
//         println!("Add new company successfully!!");
//         return company;
//     }

//     fn add_employees(&mut self, name: &str, department: &str, salary: f64) {
//         let employees = self.employees
//             .entry(name.to_string())
//             .or_insert(HashMap::new());
//         employees.insert(department.to_string(), salary);

//         let manager = self.manager.entry(department.to_string()).or_insert(vec![]);
//         manager.push(name.to_string());

//         println!("Add new employee successfully with value!!");
//     }

//     fn get_salary_of_employee(&self, name: &str, department: &str) -> Option<f64> {
//         if let Some(mapping) = self.employees.get(name) {
//             if let Some(salary) = mapping.get(department) {
//                 Some(*salary)
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     }

//     fn get_all_department_in_company(&self) -> Vec<String> {
//         let all_department_in_company = self.manager.keys().cloned().collect();
//         return all_department_in_company
//     }

//     fn get_all_employees_in_company(&self) -> Vec<String> {
//         let all_employees_in_company = self.employees.keys().cloned().collect();
//         return all_employees_in_company;
//     }

//     fn get_total_salary_of_employee(&self) -> Option<f64> {
//         let total_value = self.employees
//             .values()
//             .flat_map(|dept| dept.values())
//             .sum();
//         if total_value != 0.0 {
//             Some(total_value)
//         } else {
//             None
//         }
//     }

//     fn average_salary(&self) -> Option<f64> {
//         let total_salary: f64 = self.employees.values().flat_map(|dept| dept.values()).sum();
//         let total_member = self.employees.len() as f64;
//         let return_value = total_salary / total_member;
//         if return_value != 0.0 {
//             Some(return_value)
//         } else {
//             None
//         }
//     }

//     fn average_salary_of_department(&self, department: &str) -> Option<f64> {
//         if let Some(employee_department) = self.manager.get(department) {
//             let total_salary: f64 = employee_department
//                 .iter()
//                 .filter_map(|employee| self.employees.get(employee))
//                 .filter_map(|dept| dept.get(department))
//                 .sum();
//             let total_member = employee_department.len() as f64;
//             if total_salary != 0.0 && total_member != 0.0 {
//                 let return_value = total_salary / total_member;
//                 Some(return_value)
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     }

//     fn min_salary_of_department(&self, department: &str) -> Option<f64> {
//         if let Some(employee_department) = self.manager.get(department) {
//             let total_salary: f64 = employee_department
//                 .iter()
//                 .filter_map(|employee| self.employees.get(employee))
//                 .filter_map(|dept| dept.get(department))
//                 .fold(f64::INFINITY, |min, &salary| min.min(salary));
//             if total_salary.is_finite() {
//                 Some(total_salary)
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     }

//     fn max_salary_of_department(&self, department: &str) -> Option<f64> {
//         if let Some(employee_department) = self.manager.get(department) {
//             let max_salary: f64 = employee_department
//                 .iter()
//                 .filter_map(|employee| self.employees.get(employee))
//                 .filter_map(|dept| dept.get(department))
//                 .fold(f64::NEG_INFINITY, |max, &salary| max.max(salary));
//             if max_salary.is_finite() {
//                 Some(max_salary)
//             } else {
//                 None
//             }
//         } else {
//             None
//         }
//     }

//     fn min_salary_of_company(&self) -> Option<f64> {
//         let min_value = self.employees
//             .values()
//             .flat_map(|dept| dept.values())
//             .fold(f64::INFINITY, |min, &salary| min.min(salary));
//         if min_value.is_finite() {
//             Some(min_value)
//         } else {
//             None
//         }
//     }

//     fn max_salary_of_company(&self) -> Option<f64> {
//         let min_value = self.employees
//             .values()
//             .flat_map(|dept| dept.values())
//             .fold(f64::NEG_INFINITY, |max, &salary| max.max(salary));
//         if min_value.is_finite() {
//             Some(min_value)
//         } else {
//             None
//         }
//     }
// }

// fn main() {
//     let mut company: Company = Company::new();
//     company.add_employees("Myoonu", "It", 90123213123.0);
//     company.add_employees("John", "It", 100123213123.0);
//     company.add_employees("Marry", "It", 200123213123.0);
//     company.add_employees("Sone", "dev", 300123213123.0);
//     company.add_employees("Collron", "Admin", 500123213123.0);
//     company.add_employees("Onron", "Hr", 600123213123.0);
//     company.add_employees("Horro", "Hr", 700123213123.0);
//     company.add_employees("Camfogo", "Hr", 800123213123.0);

//     let salary_of_employee = company.get_salary_of_employee("Marry", "It");
//     match salary_of_employee {
//         Some(value) => println!("salary_of_employee is {:?}", value),
//         None => println!("Invalid data")
//     }

//     let get_all_employees_in_company = company.get_all_department_in_company();
//     println!("get_all_employees_in_company is {:?}", get_all_employees_in_company);

//     let get_all_employees_in_company = company.get_all_employees_in_company();
//     println!("get_all_employees_in_company is {:?}", get_all_employees_in_company);

//     let get_total_salary_of_employee = company.get_total_salary_of_employee();
//     match get_total_salary_of_employee {
//         Some(value) => println!("get_total_salary_of_employee is {:?}", value),
//         None => print!("Invalid value")
//     }

//     let average_salary = company.average_salary();

//     match average_salary {
//         Some(value) => println!("Value is {}", value),
//         None => println!("Invalid value")
//     }

//     let average_salary_of_department = company.average_salary_of_department("Hr");
//     match average_salary_of_department {
//         Some(value) => println!("Value is {:?}", value),
//         None => println!("Invalid value")
//     }

//     let min_salary_of_department = company.min_salary_of_department("Hr");
//     match min_salary_of_department {
//         Some(value) => println!("min_salary_of_department is {:?}", value),
//         None => println!("Invalid value")
//     }

//     let max_salary_of_department = company.max_salary_of_department("Hr");
//     match max_salary_of_department {
//         Some(value) => println!("max_salary_of_department is {:?}", value),
//         None => println!("Invalid value")
//     }

//     let max_salary_of_department = company.max_salary_of_department("Hr");
//     match max_salary_of_department {
//         Some(value) => println!("max_salary_of_department is {:?}", value),
//         None => println!("Invalid value")
//     }

//     let min_salary_of_company = company.min_salary_of_company();
//     match min_salary_of_company {
//         Some(value) => println!("min_salary_of_company is {:?}", value),
//         None => println!("Invalid value")
//     }

//     let max_salary_of_company = company.max_salary_of_company();
//     match max_salary_of_company {
//         Some(value) => println!("max_salary_of_company is {:?}", value),
//         None => println!("Invalid value")
//     }
// }

use std::{collections::HashMap, sync::Arc};

trait Office {
    fn new() -> Self
    where
        Self: Sized;

    fn add_employees(&mut self, name: &str, department: &str, salary: f64);

    fn get_salary_of_employee(&self, name: &str, department: &str) -> Option<f64>;

    fn get_all_department_in_company(&self) -> Vec<String>;

    fn get_all_employees_in_company(&self) -> Vec<String>;

    fn get_total_salary_of_employee(&self) -> Option<f64>;

    fn get_average_salary(&self) -> Option<f64>;

    fn average_salary_of_department(&self, department: &str) -> Option<f64>;

    fn min_salary_of_department(&self, department: &str) -> Option<f64>;

    fn max_salary_of_department(&self, department: &str) -> Option<f64>;

    fn min_salary_of_company(&self) -> Option<f64>;

    fn max_salary_of_company(&self) -> Option<f64>;
}

#[derive(Debug)]
struct Company {
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
        let employee = self
            .employees
            .entry(name.to_string())
            .or_insert(HashMap::new());
        employee.insert(department.to_string(), salary);

        let manager = self
            .manager
            .entry(department.to_string())
            .or_insert(vec![]);
        manager.push(name.to_string())
    }

    fn get_salary_of_employee(&self, name: &str, department: &str) -> Option<f64> {
        if let Some(employee_department) = self.employees.get(name) {
            if let Some(employee_department) = employee_department.get(department) {
                Some(*employee_department)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_all_department_in_company(&self) -> Vec<String> {
        let all_departments = self.manager.keys().cloned().collect();
        return all_departments;
    }

    fn get_all_employees_in_company(&self) -> Vec<String> {
        let all_employees = self.employees.keys().cloned().collect();
        return all_employees;
    }

    fn get_total_salary_of_employee(&self) -> Option<f64>{
        let total_salary = self.employees
            .values()
            .flat_map(|dept| dept.values())
            .sum();
        if total_salary != 0.0 {
            Some(total_salary)
        } else {
            None
        }
    }

    fn get_average_salary(&self) -> Option<f64> {
        let total_employee = self.employees.len() as f64;
        let total_salary: f64 = self.employees
            .values()
            .flat_map(|dept| dept.values())
            .sum();
        if total_employee != 0.0 && total_salary > 0.0 {
            let avarage =  total_salary / total_employee;
            Some(avarage)
        } else {
            None
        }
    }

    fn average_salary_of_department(&self, department: &str) -> Option<f64> {
        if let Some(employee_department) = self.manager.get(department) {
            let total_salary: f64 = employee_department
                .iter()
                .filter_map(|employee| self.employees.get(employee))
                .filter_map(|dept| dept.get(department))
                .sum();
            let total_employee = employee_department.len() as f64;
            if total_employee != 0.0 && total_salary > 0.0 {
                let average =  total_salary / total_employee;
                Some(average)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn min_salary_of_department(&self, department: &str) -> Option<f64> {
        if let Some(employee_department) = self.manager.get(department) {
            let min_salary_of_department: f64 = employee_department
                .iter()
                .filter_map(|employee| self.employees.get(employee))
                .filter_map(|dept| dept.get(department))
                .fold(f64::INFINITY, |min, &salary| min.min(salary));
            if min_salary_of_department != 0.0 {
                Some(min_salary_of_department)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn max_salary_of_department(&self, department: &str) -> Option<f64> {
        if let Some(employee_department) = self.manager.get(department) {
            let min_salary_of_department: f64 = employee_department
                .iter()
                .filter_map(|employee| self.employees.get(employee))
                .filter_map(|dept| dept.get(department))
                .fold(f64::NEG_INFINITY, |max: f64, &salary| max.max(salary));
            if min_salary_of_department != 0.0 {
                Some(min_salary_of_department)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn min_salary_of_company(&self) -> Option<f64> {
        let min_salary_of_company = self.employees
            .values()
            .flat_map(|dept| dept.values())
            .fold(f64::INFINITY, |min, &salary| min.min(salary));
        if min_salary_of_company != 0.0 {
            Some(min_salary_of_company)
        } else {
            None
        }
    }

    fn max_salary_of_company(&self) -> Option<f64> {
        let min_salary_of_company = self.employees
            .values()
            .flat_map(|dept| dept.values())
            .fold(f64::NEG_INFINITY, |max, &salary| max.max(salary));
        if min_salary_of_company != 0.0 {
            Some(min_salary_of_company)
        } else {
            None
        }
    }
}

fn main() {
    let mut company: Company = Company::new();

    company.add_employees("Myoonu", "It", 90123213123.0);
    company.add_employees("John", "It", 100123213123.0);
    company.add_employees("Marry", "It", 200123213123.0);
    company.add_employees("Sone", "dev", 300123213123.0);
    company.add_employees("Collron", "Admin", 500123213123.0);
    company.add_employees("Onron", "Hr", 600123213123.0);
    company.add_employees("Horro", "Hr", 700123213123.0);
    company.add_employees("Camfogo", "Hr", 800123213123.0);

    let salary_of_employee = company.get_salary_of_employee("Camfogo", "Hr");
    match salary_of_employee {
        Some(value) => println!("Valus is {:?}", value),
        None => println!("Invalid value")
    };

    let all_department_in_company = company.get_all_department_in_company();
    println!("all_department_in_company is {:?}", all_department_in_company);

    let all_employees_in_company = company.get_all_employees_in_company();
    println!("all_employees_in_company is {:?}", all_employees_in_company);

    let get_total_salary_of_employee = company.get_total_salary_of_employee();
    match get_total_salary_of_employee {
        Some(value) => println!("get_total_salary_of_employee is {:?}", value),
        None => println!("Invalid value")
    }; 

    let get_average_salary = company.get_average_salary();
    match get_average_salary {
        Some(value) => println!("get_average_salary is {:?}", value),
        None => println!("Invalid value")
    }; 

    let average_salary_of_department = company.average_salary_of_department("Hr");
    match average_salary_of_department {
        Some(value) => println!("average_salary_of_department is {:?}", value),
        None => println!("Invalid value")
    };

    let min_salary_of_department = company.min_salary_of_department("Hr");
    match min_salary_of_department {
        Some(value) => println!("min_salary_of_department is {:?}", value),
        None => println!("Invalid value")
    };

    let max_salary_of_department = company.max_salary_of_department("Hr");
    match max_salary_of_department {
        Some(value) => println!("max_salary_of_department is {:?}", value),
        None => println!("Invalid value")
    };

    let min_salary_of_company = company.min_salary_of_company();
    match min_salary_of_company {
        Some(value) => println!("min_salary_of_company is {:?}", value),
        None => println!("Invalid value")
    };

    let max_salary_of_company = company.max_salary_of_company();
    match max_salary_of_company {
        Some(value) => println!("max_salary_of_company is {:?}", value),
        None => println!("Invalid value")
    };
}
