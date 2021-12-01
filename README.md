Simple rust environment variable loader.  
Used for practicing my rust experience.  

Example Usage: `[./src/main.rs](./src/main.rs)`  

main.rs
```rs
mod env_loader;
use env_loader::EnvLoader;

fn main() {
    let loader = EnvLoader::new(".env");
    println!("{}", loader.get("MY_VAR").unwrap());
    // Hello, World!
}
```  

env file
```env
# Comment is supported too!
MY_VAR=Hello, World!
```
