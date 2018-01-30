use pwm;

pub struct Servo {
    servo: pwm::Pwm,
}
impl Servo {
    pub fn init(pin: pwm::Pin) -> Servo {
        let servo = pwm::Pwm::init(pin);
        servo.set_frequency(50);
        let duty = 2.0 + ((90.0 / 180.0) * 10.0);
        servo.set_duty(duty);
        servo.enable();
        Servo { servo }
    }
    pub fn set_position(&self, mut degree: f32) {
        // min 0° max 180°
        // for the MG90S
        // min 0.4ms max 2.4ms
        // offset 0.4ms => 0.4ms/20ms = 2%
        // delta 2ms => 2ms/20ms = 10%
        // max 2.4ms => 2.4ms/20ms = 12%
        if degree as i32 * 1000 < 0 {
            degree = 0.0;
        }
        if degree as u32 * 1000 > 180_000 {
            degree = 180.0;
        }
        let duty = 2.0 + ((degree / 180.0) * 10.0);
        self.servo.set_duty(duty);
    }
}
