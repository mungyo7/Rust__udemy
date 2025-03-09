// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Employees {
    Maintenance_crews,
    Marketing_department_employees,
    Managers,
    Line_supervisors,
    Kitchen_staff,
    Assembly_technicians,
}

struct emp_struct {
    type_emp: Employees,
    still: bool,
}

fn determine(emp: emp_struct) -> Result< (), String > {
    if (emp.still == True){
        match emp.Employees{
            Employees::Maintenance_crews => Ok("yes"),
        }
    }
    
}
fn main() {

}
