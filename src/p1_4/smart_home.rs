struct PowerSocket {
    voltage: u32,
    current: u32,
}

impl PowerSocket {
    fn description(&self) -> String {
        todo!()
    }

    fn power(&self) -> u32 {
        self.voltage * self.current
    }

    fn enable(&self) {
        todo!()
    }

    fn disable(&self) {
        todo!()
    }
}

struct TemperatureSensor {}

impl TemperatureSensor {
    fn temperature(&self) -> f32 {
        todo!()
    }
}
