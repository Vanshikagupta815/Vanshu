use std::io;

struct Employee {
    employee_name: String,
    employee_id: i32,
    email: String,
    age: i32,
    phone_number: String,
}

fn find_employee_by_id(employees: &[Employee], employee_id: i32) -> Option<&Employee> {
    employees.iter().find(|emp| emp.employee_id == employee_id)
}

fn find_employees_by_age(employees: &[Employee], age: i32) -> Vec<&Employee> {
    employees.iter().filter(|emp| emp.age == age).collect()
}

fn main() {
    println!("Enter the number of employees:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_employees: usize = input.trim().parse().expect("Invalid input");

    let mut employees: Vec<Employee> = Vec::with_capacity(num_employees);

    for _ in 0..num_employees {
        println!("Enter details for employee:");
        let mut employee = Employee {
            employee_name: String::new(),
            employee_id: 0,
            email: String::new(),
            age: 0,
            phone_number: String::new(),
        };

        println!("Name:");
        io::stdin().read_line(&mut employee.employee_name).expect("Failed to read input");

        println!("Employee ID:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        employee.employee_id = input.trim().parse().expect("Invalid input");

        println!("Email:");
        io::stdin().read_line(&mut employee.email).expect("Failed to read input");

        println!("Age:");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        employee.age = input.trim().parse().expect("Invalid input");

        println!("Phone Number:");
        io::stdin().read_line(&mut employee.phone_number).expect("Failed to read input");

        employees.push(employee);
    }

    println!("Enter employee ID to search:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let search_id: i32 = input.trim().parse().expect("Invalid input");

    if let Some(found_employee) = find_employee_by_id(&employees, search_id) {
        println!("Employee details found:");
        println!("Name: {}", found_employee.employee_name);
        println!("Employee ID: {}", found_employee.employee_id);
        println!("Email: {}", found_employee.email);
        println!("Age: {}", found_employee.age);
        println!("Phone Number: {}", found_employee.phone_number);
    } else {
        println!("Employee with ID {} not found.", search_id);
    }

    println!("Enter age to search employees with the same age:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let search_age: i32 = input.trim().parse().expect("Invalid input");

    let matching_employees = find_employees_by_age(&employees, search_age);
    if !matching_employees.is_empty() {
        println!("Employees with age {}:", search_age);
        for emp in matching_employees {
            println!("Name: {}, Employee ID: {}", emp.employee_name, emp.employee_id);
        }
    } else {
        println!("No employees found with age {}.", search_age);
    }
}