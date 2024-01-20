use std::io;

struct Student {
    name: String,
    ssn: String

}
impl std::fmt::Display for Student{
    fn fmt(&self, f: & mut std::fmt::Formatter)-> std::fmt::Result {
        write!(f,"Student( name={}, ssn={})",self.name, self.ssn)
    }
}
fn main() {
    println!("Enter your name: ");
    let mut name=String::new();
    io::stdin().read_line(&mut name).unwrap();
    name = (*name.trim()).to_string();
    let stud = Student {
        name: name,
        ssn: String::from("1234")
    };

    println!("Hello! {}",stud);
}
