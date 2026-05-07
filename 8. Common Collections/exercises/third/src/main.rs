use std::collections::HashMap;
use std::io;

#[derive(Debug , PartialEq)]
enum Department {
    IT,
    FINANCE,
    HR,
}

#[derive(Debug)]
struct EmpDept{
    map : HashMap<String , Department>,
}

impl EmpDept{
    fn new() -> Self{
        Self {
            map:HashMap::new()
        }
    } 

    fn assign_dept(&mut self , emp : &str, dept : Department){
        self.map.insert(emp.to_string() , dept);
    }
}

fn main() {
    let mut map = EmpDept::new();

    // let name = String::from("Darshan");
    // map.assign_dept("Darshan", Department::IT);

    loop{
        let mut opt = String::new() ;

        println!("ENTER THE OPTIONS\n1: INSERT EMP TO DEPT\n2: GET EMPLOYEE AS PER DEPT\n3: EXIT");
        io::stdin().read_line(&mut opt).unwrap();

        match opt.trim(){
            "1" => {
                let mut emp = String::new();

                println!("Enter the Employee name : ");

                io::stdin().read_line(&mut emp).unwrap();
                
                println!("ENTER THE NUMBER FOR DEPT: \n1. IT \n2. FINANCE \n3. HR");

                let mut dept = String::new();
                io::stdin().read_line(&mut dept).unwrap();

                match dept.trim() {
                    "1" => map.assign_dept(&emp.trim() , Department::IT),
                    "2" => map.assign_dept(&emp.trim() , Department::FINANCE),
                    "3" => map.assign_dept(&emp.trim() , Department::HR),
                    _ => println!("INSERT CORRECT OPTION")
                }
            }
            "2" => {
                println!("ENTER THE NUMBER FOR DEPT: \n1. IT \n2. FINANCE \n3. HR");
                
                let mut dept = String::new();
                io::stdin().read_line(&mut dept).unwrap();
   
                let target_dep = match dept.trim(){
                    "1" => Department::IT,
                    "2" => Department::FINANCE,
                    "3" => Department::HR,
                    _ => {
                        println!("INVALID DEPT");
                        continue;
                    }
                };

                let mut emp_list:Vec<&String> = Vec::new();

                for (emp , dept) in &map.map{
                    if *dept == target_dep{
                        emp_list.push(emp);
                    }
                }

                println!("{emp_list:?}");
            }
            "3" => break,

            _ => println!("ENTER CORECT OPTION")
        }
    }

    println!("{map:?}");
}
