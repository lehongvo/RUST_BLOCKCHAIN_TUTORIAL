// fn main() {
//     let vector = vec![1, 2, 3, 4];

//     let dose_not_exist = &vector.get(100);
//     match dose_not_exist {
//         Some(value) => println!("Value is {}", value),
//         None => println!("Error: Out of range from 0 to {}", vector.len()),
//     }
// }

// fn main () {
//     let mut vector = vec![1, 2, 3, 4];
//     let first_value = &vector.get(0).cloned();
//     vector.push(10);
//     println!("First element is {:?}", first_value);
// }

// fn main() {
//     let vector = vec![100, 32, 57];
//     for (index, value) in vector.iter().enumerate() {
//         println!("Value {index} is  {value}");
//     }
// }

// fn main() {
//     let mut vector = vec![100, 120, 123];
//     for i in &mut vector {
//         *i += 11;
//     }
//     println!("Value is: {:?}", vector);
// }

// #[derive(Debug)]
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// fn main() {
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Float(4.5),
//         SpreadsheetCell::Text(
//             String::from("Vo Le Hong")
//         )
//     ];
//     println!("Value is {:?}", row)
// }

// fn main() {
//     {
//         let vector = vec![100, 12, 234];
//         for i in vector {
//             println!("Value is {:?}", i);
//         }
//     }
//     print!("vector: {:?}", vector)

// }

// fn main() {
//     // let my_str: &str = "Hello, world!";
//     // let my_string: String = String::from("Hello, world");
//     // println!("Value is {}", my_str);
//     // println!("Value is {}", my_string);

//     // let mut empty_string = String::new();

//     // let data = "Initial contents";
//     // let data_to_string = data.to_string();

//     // let mut my_string = String::from("foo");
//     // my_string.push_str("Hi");
//     // println!("my_string {}", my_string)

//     // let s1 = String::from("Hello,!");
//     // let s2 = String::from("world!");
//     // let s3 = s1 + &s2;
//     // println!("Values is s3 {}", s3);

//     let hello = "Здравствуйте";
//     for value in hello.chars() {
//         println!("Value is {}", value);
//     }
//  }
// use std::collections::HashMap;
// fn main() {
//     let mut scores: HashMap<String, u8> = HashMap::new();
//     scores.insert("Blue".to_string(), 10);
//     scores.insert("Yellow".to_string(), 50);

//     let team_name = String::from("Blue");
//     let value = scores.get(&team_name).copied().unwrap_or(0);
// println!("Score for {team_name} is {value}");

// for (key, value) in &scores {
//     println!("Key is: {}, Value is: {:?}", key, value);
// }
// scores.insert(String::from("Blue"), 25);
// println!("Score is {:?}", scores);

// scores.entry(String::from("Green")).or_insert(30);
// println!("Score is {:?}", scores);

// let mut word_count: HashMap<String, u8> = HashMap::new();
// let text = "Hello world wonderful world";
// let spit_text: Vec<&str> = text.split(' ').collect();

// for value in &spit_text {
//     println!("Value is: {:?}", value);
//     let count = word_count.entry(value.to_string()).or_insert(0);
//     *count += 1;
// }
// println!("Value is {:?}", word_count);

// let text = "Hello world wonderful world";
// let mut word_count: HashMap<String, u8> = HashMap::new();

// let split_text = text.split_whitespace();
// for value in split_text {
//     let count = word_count.entry(String::from(value)).or_insert(0);
//     *count += 1;
// }
// println!("Value is {:?}", word_count);
// }

// fn calculate_median(numbers: &Vec<i32>) -> f64 {
//     let mut numbers_clone = numbers.clone();
//     numbers_clone.sort_by(|a, b| a.cmp(b));
//     let length = numbers_clone.len();
//     if length % 2 == 0 {
//         let middle1: i32 = numbers_clone[length / 2 - 1];
//         let middle2: i32 = numbers_clone[length / 2];
//         let value: f64 = ((middle1 + middle2) / 2) as f64;
//         return value;
//     } else {
//         let value = numbers_clone[length / 2] as f64;
//         return value;
//     }
// }

/*
let mut mapping_hashmap: HashMap<i32, i32> = HashMap::new();
    for &value in numbers {
        let count = mapping_hashmap.entry(value).or_insert(0);
        *count += 1;
    }

    let (result_value, result_count) = mapping_hashmap
        .iter()
        .max_by_key(|&(_, count)| count)
        .unwrap_or((&0, &0));

    return (*result_value, *result_count)
*/
// fn same_max_number(numbers: &Vec<i32>) -> (i32, i32) {
//     let mut new_mapping: HashMap<i32, i32> = HashMap::new();
//     for number in numbers  {
//         let count = new_mapping.entry(*number).or_insert(0);
//         *count += 1;
//     }

//     let (result_value, result_count) = new_mapping.iter().max_by_key(|&(_, count)| count).unwrap_or((&0, &0));
//     return (*result_value, *result_count);
// }

// fn main() {
//     let numbers = vec![8, 8, 8, 8, 8, 1, 2, 3, 3, 5, 5, 1, 1, 7, 2, 2, 1];
//     // let value_median = calculate_median(&numbers);
//     // println!("value_median is {:?}", value_median)
//     let same_max_number = same_max_number(&numbers);
//     println!("Same max number is {:?}", same_max_number)
// }

// use std::collections::HashMap;

// trait Office {
//     fn new() -> Self
//     where
//         Self: Sized;

//     fn add_enployee(&mut self, name: &str, department: &str);

//     fn get_department_employees(&self, department: &str) -> Option<&Vec<String>>;

//     fn get_all_employees(&self) -> Vec<String>;
// }

// #[derive(Debug)]
// struct Company {
//     employee_department: HashMap<String, String>,
//     department_employees: HashMap<String, Vec<String>>,
// }

// impl Office for Company {
//     fn new() -> Company {
//         let company: Company = Company {
//             employee_department: HashMap::new(),
//             department_employees: HashMap::new(),
//         };
//         return company;
//     }

//     fn add_enployee(&mut self, name: &str, department: &str) {
//         self.employee_department
//             .insert(name.to_string(), department.to_string());

//         let department_employees = self.department_employees
//             .entry(department.to_string())
//             .or_insert(vec![]);
//         department_employees.push(name.to_string());
//     }

//     fn get_department_employees(&self, department: &str) -> Option<&Vec<String>> {
//         let department_employees: Option<&Vec<String>> = self.department_employees.get(department);
//         return department_employees;
//     }

//     fn get_all_employees(&self) -> Vec<String> {
//         let mut all_employees: Vec<String> = self.employee_department.keys().cloned().collect();
//         all_employees.sort();
//         return all_employees;
//     }
// }
// fn main() {
//     let mut company: Company = Company::new();
//     let department_employees_before = company.get_department_employees( "hr");
//     println!("Department is {:?}", department_employees_before);

//     company.add_enployee("Le Hong Vo", "Dev");
//     company.add_enployee("Le Hong Van", "Dev");
//     company.add_enployee("Le Hong Vi", "Dev");
//     company.add_enployee("Mai Thi Van Anh", "Hr");
//     company.add_enployee("Mai Thi Van van", "Hr");
//     company.add_enployee("Mai Thi Van Linh", "Hr");
//     company.add_enployee("Tran Van Dat", "Admin");

//     let department_employees_after = company.get_department_employees( "Hr");
//     // println!("Department is {:?}", department_employees_before);
//     match department_employees_after  {
//         Some(department_employees) => println!("Department is {:?}", department_employees),
//         None => println!("Department not found")
//     }

//     let all_employees = company.get_all_employees();
//     for (index, value) in all_employees.iter().enumerate()  {
//         println!("Employee at {index} has name {value}");
//     }
// }

// use std::collections::HashMap;

// trait Office {
//     fn new() -> Self
//     where
//         Self: Sized;

//     fn add_employee(&mut self, name: &str, department: &str);

//     fn get_employees_in_department(&self, department: &str) -> Option<&Vec<String>>;

//     fn get_all_employees(&self) -> Vec<String>;
// }

// struct Company {
//     employee_from_department: HashMap<String, String>,
//     department_from_employees: HashMap<String, Vec<String>>,
// }

// impl Office for Company {
//     fn new() -> Company {
//         let company: Company = Company {
//             employee_from_department: HashMap::new(),
//             department_from_employees: HashMap::new(),
//         };
//         return company;
//     }

//     fn add_employee(&mut self, name: &str, department: &str) {
//         self.employee_from_department
//             .insert(name.to_string(), department.to_string());
//         let department_from_employees = self
//             .department_from_employees
//             .entry(department.to_string())
//             .or_insert(vec![]);
//         department_from_employees.push(name.to_string());
//     }

//     fn get_employees_in_department(&self, department: &str) -> Option<&Vec<String>> {
//         let all_employees_in_department = self.department_from_employees.get(&department.to_string());
//         return all_employees_in_department;
//     }

//     fn get_all_employees(&self) -> Vec<String> {
//         let mut all_employees_in_company:  Vec<String> = self.employee_from_department.keys().cloned().collect();
//         all_employees_in_company.sort();
//         return all_employees_in_company;
//     }
// }

// fn main() {
//     let mut company: Company = Company::new();

//     let get_employees_in_department_after =  company.get_employees_in_department("Dev");
//     match get_employees_in_department_after {
//         Some(value) => println!("Value is {:?}", value),
//         None => println!("Department not found!")
//     }

//     let get_all_employees = company.get_all_employees();
//     println!("get_all_employees is {:?}", get_all_employees);

//     company.add_employee("Le Hong Vo1", "Hr");
//     company.add_employee("Le Hong Vo2", "Hr");
//     company.add_employee("Le Hong Vo3", "Hr");
//     company.add_employee("Le Hong Vo4", "Hr");
//     company.add_employee("Le Hong Vo5", "Admin");
//     company.add_employee("Le Hong Vo6", "Admin");
//     company.add_employee("Le Hong Vo7", "Admin");
//     company.add_employee("Le Hong Vo8", "Dev");
//     company.add_employee("Le Hong Vo9", "Dev");

//     let get_employees_in_department_before=  company.get_employees_in_department("Dev");
//     match get_employees_in_department_before {
//         Some(value) => println!("Value is {:?}", value),
//         None => println!("Department not found!")
//     }

//     let get_all_employees = company.get_all_employees();
//     println!("get_all_employees is {:?}", get_all_employees);
// }

// use std::{collections::HashMap, sync::Arc};
// struct Company {
//     employee_from_department: HashMap<String, HashMap<String, f64>>,
//     department_from_employees: HashMap<String, Vec<String>>,
// }

// impl Company {
//     fn new_company() -> Company {
//         let company = Company {
//             employee_from_department: HashMap::new(),
//             department_from_employees: HashMap::new(),
//         };
//         return company;
//     }

//     fn add_employee(&mut self, name: &str, department: &str, salary_per_month: f64) {
//         let department_from_employees = self
//             .employee_from_department
//             .entry(name.to_string())
//             .or_insert(HashMap::new());
//         department_from_employees.insert(department.to_string(), salary_per_month);
//     }

//     fn get_salary_per_month_1(&self, name: &str, department: &str) -> Option<f64> {
//         self.employee_from_department
//             .get(name)
//             .and_then(|department_employees| department_employees.get(department))
//             .map_or(None, |&salary| Some(salary))
//     }

//     fn get_salary_per_month_2(&self, name: &str, department: &str) -> Option<f64> {
//         match self.employee_from_department.get(name) {
//             Some(department_employees) => department_employees.get(department).map(|&salary| salary),
//             None => None
//         }
//     }
// }

// use std::collections::HashMap;

// struct Company {
//     employees: HashMap<String, HashMap<String, f64>>,
//     managers: HashMap<String, Vec<String>>,
// }

// impl Company {
//     fn new() -> Company {
//         let company = Company {
//             employees: HashMap::new(),
//             managers: HashMap::new(),
//         };
//         return company;
//     }

//     fn add_employees(&mut self, name: &str, department: &str, salary: f64) {
//         let employees = self
//             .employees
//             .entry(name.to_string())
//             .or_insert(HashMap::new());
//         employees.insert(department.to_string(), salary);

//         let managers = self
//             .managers
//             .entry(department.to_string())
//             .or_insert(vec![]);
//         managers.push(name.to_string());
//     }

//     fn get_salary_per_month_1(&self, name: &str, department: &str) -> Option<f64> {
//         self.employees
//             .get(name)
//             .and_then(|department_employees| department_employees.get(department))
//             .map_or(None, |&salary| Some(salary))
//     }

//     fn get_salary_per_month_2(&self, name: &str, department: &str) -> Option<f64> {
//         match self.employees.get(name) {
//             Some(department_employees) => {
//                 department_employees.get(department).map(|&salary| salary)
//             }
//             None => None,
//         }
//     }

//     fn get_all_department(&self) -> Vec<String> {
//         let all_department: Vec<String> = self.managers.keys().cloned().collect();
//         return all_department;
//     }

//     fn get_all_employees(&self) -> Vec<String> {
//         let all_employees: Vec<String> = self.employees.keys().cloned().collect();
//         return all_employees;
//     }

//     fn get_all_member_in_department(&self, department: &str) -> Option<Vec<String>> {
//         if let Some(members) = self.managers.get(department) {
//             Some(members.clone())
//         } else {
//             None
//         }
//     }

//     fn average_salary(&self) -> Option<f64> {
//         let total_salary: f64 = self.employees.values().flat_map(|dept| dept.values()).sum();
//         let total_member = self.employees.len() as f64;
//         if total_salary > 0.0 {
//             let return_value = total_salary / total_member;
//             Some(return_value)
//         } else {
//             None
//         }
//     }

// }

// fn main() {
//     let mut company: Company = Company::new();
//     company.add_employees("Le Hong Vo1", "Admin", 200000000.01);
//     company.add_employees("Le Hong Vo2", "Admin", 300000000.02);
//     company.add_employees("Le Hong Vo3", "Dev", 400000000.03);
//     company.add_employees("Le Hong Vo4", "Dev", 500000000.04);
//     company.add_employees("Le Hong Vo5", "Dev", 600000000.05);
//     company.add_employees("Le Hong Vo6", "HR", 700000000.04);

//     let all_employees: Vec<String> = company.get_all_employees();
//     println!("All employees is {:?}", all_employees);

//     let all_department: Vec<String> = company.get_all_department();
//     println!("All employees is {:?}", all_department);

//     let all_member_in_department: Option<Vec<String>> = company.get_all_member_in_department("Dev");
//     match all_member_in_department {
//         Some(value) =>  println!("Value is {:?}", value),
//         None => println!("Department not found")
//     }

//     let average_salary = company.average_salary();
//     match average_salary {
//         Some(value) => println!("Value is {:?}", value),
//         None => println!("Department not found")
//     }
// }

use std::collections::HashMap;

trait Office {
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
        let min_salary = self.employees
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
        let max_salary = self.employees
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

    let John_salary = company.get_salary_of_employee("John", "It");
    match John_salary {
        Some(salary) => println!("Value is {:?}", salary),
        None => println!("Invalid salary"),
    }

    let all_employees = company.get_all_employees_in_company();
    println!("all_employees {:?}", all_employees);

    let all_department = company.get_all_department_in_company();
    println!("all_department {:?}", all_department);

    let get_all_salary_of_employee = company.get_all_salary_of_employee();
    match get_all_salary_of_employee {
        Some(total_salary) => println!("Total salary is: {:?}", total_salary),
        None => println!("Employee not found"),
    };

    let average_salary = company.average_salary();
    match average_salary {
        Some(avarage) => println!("Avarage: {:?}", avarage),
        None => println!("Employee not found"),
    };

    let average_salary_of_department = company.average_salary_of_department("Hr");
    match average_salary_of_department {
        Some(valus) => println!("average_salary_of_department: {:?}", valus),
        None => println!("Invalid department"),
    }

    let min_salary_of_department = company.min_salary_of_department("Admin");
    match min_salary_of_department {
        Some(value) => println!("min_salary_of_department: {:?}", value),
        None => println!("Invalid department"),
    }

    let max_salary_of_department = company.max_salary_of_department("Admin");
    match max_salary_of_department {
        Some(value) => println!("max_salary_of_department: {:?}", value),
        None => println!("Invalid department"),
    }

    let min_salary_of_company = company.min_salary_of_company();
    match min_salary_of_company {
        Some(value) => println!("min_salary_of_company: {:?}", value),
        None => println!("Invalid value"),
    }

    let max_salary_of_company = company.max_salary_of_company();
    match max_salary_of_company {
        Some(value) => println!("max_salary_of_company: {:?}", value),
        None => println!("Invalid value"),
    }
}
