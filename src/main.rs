use wasmer::{imports, Function, Instance, Module, Store};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wasm_bytes = include_bytes!("../guest/target/wasm32-unknown-unknown/release/guest.wasm");
    let store = Store::default();

    println!("Compiling module...");
    let module = Module::new(&store, wasm_bytes)?;

    fn ocall(thread: i32, nth: i32, p: i32) {
        println!("thread {}, {}-nth ocall, p={}", thread, nth, p);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let import_object = imports! {
        "env" => {
            "ocall" => Function::new_native(&store, ocall),
        }
    };

    println!("Instantiating module...");
    let instance = Instance::new(&module, &import_object)?;

    let mut threads = vec![];
    for i in 0..3 {
        let instance = instance.clone();
        let handle = std::thread::spawn(move || {
            let entry = instance
                .exports
                .get_function("entry")
                .unwrap()
                .native::<i32, ()>()
                .unwrap();
            entry.call(i).unwrap();
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
    Ok(())
}
