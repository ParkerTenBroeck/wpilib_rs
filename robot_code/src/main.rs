use wpilib_rs::timed_robot::{TimedRobot, TimedRobotTrait};

pub fn main() {
    MyRobot::start_robot();
}

use wpilib_rs::frc::xbox_controller::{Port, XboxController};

struct MyRobot {
    start: std::time::Instant,
    last: std::time::Instant,
    driver: XboxController,
}

impl TimedRobotTrait for MyRobot {
    fn robot_init(_context: &TimedRobot<Self>) -> Self {
        println!("Robot Init");

        Self {
            start: std::time::Instant::now(),
            last: std::time::Instant::now(),
            driver: XboxController::new(Port::Port0),
        }
    }

    fn robot_periodic(&mut self, _context: &TimedRobot<Self>) {
        let now = std::time::Instant::now();
        let elapsed = self.last.elapsed();

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
        println!("num: {num}, bool: {bool}, string: {string}");

        self.last = now;
    }
}
