pub struct IsTransient {
    value: i32,
}

impl IsTransient {
    pub fn new() -> Self {
        IsTransient { value: 0 }
    }
    pub fn increment(&mut self) {
        self.value += 1;
    }
    pub fn report(&self) -> i32 {
        self.value
    }
}
