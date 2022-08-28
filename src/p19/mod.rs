use std::sync::Mutex;

#[derive(Default)]
struct Temperature(Mutex<f32>);

//this is wrapper for mutex to avoid deadlock
impl Temperature {
    // mutex guard live only in this methods and deadlock is highly improbable
    pub fn get(&self) -> f32 {
        *self.0.lock().unwrap()
    }

    pub fn set(&self, val: f32) {
        *self.0.lock().unwrap() = val
    }
}
