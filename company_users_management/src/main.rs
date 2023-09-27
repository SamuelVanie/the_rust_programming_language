use std::collections::HashMap;
use std::io;

fn add_to_company(depart_name: &str, employee_name: &str, company_map: &mut HashMap<String, Vec<String>>) {
    let employees = company_map.entry(depart_name.to_string()).or_insert(Vec::new());
    employees.push(employee_name.to_string());
}

fn main() {
    println!("Welcome to the company management application\n");

    let mut company_map: HashMap<String, Vec<String>> = HashMap::new();
    
    loop {
	println!("Que souhaitez-vous réaliser ? : \n");
	println!("1. Ajouter un employé\n2. Afficher la liste des employées d'un département\n3. Afficher la liste de tous les employées\n4. Quitter l'application\n");

	let mut choice = String::new();

	io::stdin().read_line(&mut choice).expect("Impossible de lire l'entree");

	let choice = match choice.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};

	match choice {
	    1 => {
		println!("Enter the name of the departement followed by the name of the employee separated by space\n");
		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input).expect("Failed to read input");
		let data: Vec<&str> = user_input.split_whitespace().collect();
		add_to_company(data.get(0).unwrap().trim(), data.get(1).unwrap().trim(), &mut company_map);
	    },
	    2 => {
		println!("Showing the list of users in a departement\nEnter the name of the departement");
		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input).expect("Failed to read input");
		let user_input = user_input.trim();
		println!("{:?}", company_map.get(user_input).unwrap());
	    },
	    3 => {
		println!("List of all employees");
		for (dep, emp) in company_map.iter() {
		    println!("{} : {:?}", dep, emp);
		}
	    },
	    4 => {
		println!("Exiting....");
		break;
	    },
	    _ => println!("Invalid choice")
	};
    };
}
