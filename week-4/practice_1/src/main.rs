fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Km 52 lekki-Epe Expressway, Ibeju-lekki, lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}",uni,addr);


    let department:&'static str = "Computer science";
    let school:&'static str = "School of science and Technology";
    println!("department: {}, \nschool: {}",department,school);
}
