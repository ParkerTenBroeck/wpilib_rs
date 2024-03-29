use wpilib_rs::bindings;
use wpilib_rs::frc::errors::{Errors, Warnings};
use wpilib_rs::frc::pwm::PWM;
use wpilib_rs::frc::xbox_controller::{Port, XboxController};
use wpilib_rs::timed_robot::{TimedRobot, TimedRobotTrait};

pub fn main() {
    MyRobot::start_robot();
}

struct MyRobot {
    start: std::time::Instant,
    last: std::time::Instant,
    doff: [f32; 64],
    i: usize,
    driver: XboxController,
    pwm_0: PWM,
    pwm_1: PWM,
    pwm_2: PWM,
    pwm_3: PWM,
    pwm_4: PWM,
    pwm_5: PWM,
    leds: PWM,
    led_index: i32,
}

impl TimedRobotTrait for MyRobot {
    fn robot_init(_context: &TimedRobot<Self>) -> Self {
        println!("Robot Init");

        Self {
            start: std::time::Instant::now(),
            last: std::time::Instant::now(),
            doff: [0.0; 64],
            i: 0,
            driver: XboxController::new(Port::Port0),
            pwm_0: PWM::new(0).unwrap(),
            pwm_1: PWM::new(1).unwrap(),
            pwm_2: PWM::new(2).unwrap(),
            pwm_3: PWM::new(3).unwrap(),
            pwm_4: PWM::new(4).unwrap(),
            pwm_5: PWM::new(5).unwrap(),
            leds: PWM::new(9).unwrap(),
            led_index: -43,
        }
    }

    fn teleop_periodic(&mut self, _context: &TimedRobot<Self>) {
        let turn = self.driver.get_right_x();
        let forward = self.driver.get_left_y();


        let left = forward - turn;
        let right = forward + turn;
        
        self.pwm_0.set_speed(left).unwrap();
        self.pwm_1.set_speed(left).unwrap();
        self.pwm_2.set_speed(left).unwrap();

        self.pwm_3.set_speed(-right).unwrap();
        self.pwm_4.set_speed(-right).unwrap();
        self.pwm_5.set_speed(-right).unwrap();
    }

    fn robot_periodic(&mut self, _context: &TimedRobot<Self>) {
        // let now = std::time::Instant::now();
        // let elapsed = self.last.elapsed();
        // self.i = (self.i + 1) & 0b111111;
        // self.doff[self.i] = elapsed.as_secs_f32();

        // let mut ds = 0.0;
        // let mut worst = 0.0;
        // for i in 0..63 {
        //     let tmp = (self.doff[i] - self.doff[i + 1]).abs();
        //     if tmp > worst {
        //         worst = tmp;
        //     }
        //     ds += tmp;
        // }
        // ds /= 63.0;
        // println!("DS: {}us", ds * 1000000.0);
        // println!("Worst: {}us", worst * 1000000.0);

        if self.driver.get_a_button_pressed(){
            self.led_index += 1;
        }
        if self.driver.get_y_button_pressed(){
            self.led_index -= 1;
        }
        if self.driver.get_x_button(){
            self.led_index = -43;
        }
        println!("{:.2}", self.led_index as f64 / 100.0);

        self.leds.set_speed(self.led_index as f64 / 100.0).unwrap();

        // use wpilib_rs::frc::smart_dashboard;

        // println!("{}", self.driver.get_left_y());

        // smart_dashboard::put_number("Stick X", self.driver.get_left_x());
        // smart_dashboard::put_number("Stick Y", self.driver.get_left_y());
        // smart_dashboard::put_boolean("Boolean", self.driver.get_a_button());
        // smart_dashboard::put_string(
        //     "Time Since Start",
        //     format!("{}us", self.start.elapsed().as_micros()),
        // );
        // smart_dashboard::put_string("Time Since LastLoop", format!("{}us", elapsed.as_micros()));

        // smart_dashboard::set_default_number("In Num", 1.0);
        // smart_dashboard::set_default_boolean("In Bool", false);
        // smart_dashboard::set_default_string("In String", "EMPTY");

        // let num = smart_dashboard::get_number("In Num", 1.0);
        // let bool = smart_dashboard::get_boolean("In Bool", false);
        // let string = smart_dashboard::get_string("In String", "EMPTY");
        // smart_dashboard::put_string(
        //     "InAndBackOut",
        //     format!("num: {num}, bool: {bool}, string: {string}"),
        // );

        // self.last = now;
    }
}
