mod env_loader;
use env_loader::EnvLoader;

fn main() {
    // if run at the root of the project, the following line will work
    const VAR: &[&str] = &
    ["HELLO", "TEST1", "TEST2"];

    print_all(VAR);
}

fn print_all(vars: &[&str]) -> () {
    let env = EnvLoader::new(".env.example");
    for var in vars {
        println!("{} = \"{}\"", var,
            match &env {
                Ok(v) => match v.get(&var as &str) {
                    Some(k) => k.to_string(),
                    None => "No variable found.".to_string()
                },
                Err(_) => "Something went wrong while loading .env file!".to_string()
            }
        )
    }
}