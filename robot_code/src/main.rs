use wpilib_rs::frc::errors::{Errors, Warnings};
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
        }
    }

    fn robot_periodic(&mut self, _context: &TimedRobot<Self>) {
        println!("This is very silly :<");
        wpilib_rs::ReportHalError!(Warnings::Warning, "asdasdlkjasdlkjasd, {}", 12);
        wpilib_rs::ReportHalError!(Errors::Error, "123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567");
        wpilib_rs::ReportHalError!(Errors::Error, "1BRUHSAODJASDLKj, {}", 12);
        wpilib_rs::ReportHalError!(Errors::Error, "2BRUHSAODJASDLKj, {}", 12);

        wpilib_rs::ReportHalErrorBackTrace!(Errors::Error, "STACK BABY");

        let now = std::time::Instant::now();
        let elapsed = self.last.elapsed();
        self.i = (self.i + 1) & 0b111111;
        self.doff[self.i] = elapsed.as_secs_f32();

        let mut ds = 0.0;
        let mut worst = 0.0;
        for i in 0..63 {
            let tmp = (self.doff[i] - self.doff[i + 1]).abs();
            if tmp > worst {
                worst = tmp;
            }
            ds += tmp;
        }
        ds /= 63.0;
        println!("DS: {}us", ds * 1000000.0);
        println!("Worst: {}us", worst * 1000000.0);

        use wpilib_rs::frc::smart_dashboard;

        smart_dashboard::put_number("Stick X", self.driver.get_left_x());
        smart_dashboard::put_number("Stick Y", self.driver.get_left_y());
        smart_dashboard::put_boolean("Boolean", self.driver.get_a_button());
        smart_dashboard::put_string(
            "Time Since Start",
            format!("{}us", self.start.elapsed().as_micros()),
        );
        smart_dashboard::put_string("Time Since LastLoop", format!("{}us", elapsed.as_micros()));

        smart_dashboard::set_default_number("In Num", 1.0);
        smart_dashboard::set_default_boolean("In Bool", false);
        smart_dashboard::set_default_string("In String", "EMPTY");

        let num = smart_dashboard::get_number("In Num", 1.0);
        let bool = smart_dashboard::get_boolean("In Bool", false);
        let string = smart_dashboard::get_string("In String", "EMPTY");
        smart_dashboard::put_string(
            "InAndBackOut",
            format!("num: {num}, bool: {bool}, string: {string}"),
        );

        self.last = now;
    }
}
