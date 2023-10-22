fn main() {
    let s = match std::fs::read_to_string("./ivsort/src/main.rs") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("dread:\n{}", e);
            return;
        }
    };

    match syn::parse_str::<syn::File>(&s) {
        Ok(file) => {
            println!("{}", quote::quote!(#file));
        }
        Err(e) => eprintln!("dies:\n{}", e),
    }
}
