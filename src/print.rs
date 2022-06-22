pub fn run() {
    println!("Hello from the print.rs file");

    println!("Number: {}", 1);

    println!(
        "{} is a Coriolis mass flow meter from {}",
        "Promass", "Endress+Hauser B.V."
    );

    // Positional arguments
    println!(
        "{0} is a {1} {2} meter from {3} which measures {2}",
        "Promass", "Coriolis", "Mass flow", "Endress+Hauser B.V."
    );

    // Named arguments
    println!(
        "a {name} meter measures {activity}",
        name = "Promass",
        activity = "mass flow"
    );
}
