mod api_parser;
mod godot_exe;

pub use api_parser::ApiParser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
