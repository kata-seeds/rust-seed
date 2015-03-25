pub struct Person {
    greeting: &'static str,
}

impl Person {
    pub fn new() -> Person {
        Person { greeting: "Hello!"}
    }

    pub fn greet(&self) -> &'static str {
        self.greeting
    }
}
