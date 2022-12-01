macro_rules! loaders {
    ($($fn: ident => $file: expr$(,)*)*) => {
	$(
	    pub fn $fn() -> String {
		std::fs::read_to_string($file).unwrap()
	    }
	)*
    }
}

loaders! {
    load_sample => "sample",
    load_input => "input",
}
