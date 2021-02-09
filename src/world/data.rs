#[derive(Debug, PartialEq)]
pub struct Data {}

impl Data {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_data() {
        let data = Data::new();
    }
}
