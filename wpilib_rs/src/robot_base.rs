use std::sync::Arc;

use crate::{
    bindings::{self},
    hal,
};

use crate::bindings::frc::RobotBase as NativeRobotBase;

pub trait RobotBaseTrait: Sync + Send + std::panic::RefUnwindSafe + 'static {
    fn new(robot: RobotBase) -> Arc<Self>;
    fn start_competition(self: Arc<Self>);
    fn end_competion(self: Arc<Self>);
}

pub struct RobotBase {
    robot_base: NativeRobotBase,
    thread_id: std::thread::ThreadId,
}

unsafe impl Send for RobotBase {}

unsafe impl Sync for RobotBase {}

impl Default for RobotBase {
    fn default() -> Self {
        Self::new()
    }
}

impl RobotBase {
    pub fn new() -> Self {
        unsafe {
            Self {
                thread_id: std::thread::current().id(),
                robot_base: bindings::frc::RobotBase::new(),
            }
        }
    }

    pub fn start_robot<T: RobotBaseTrait>() {
        // We probably want backtrace support on
        std::env::set_var("RUST_BACKTRACE", "1");

        let robot = T::new(Self::new());

        unsafe {
            hal::run_hal_initialization();

            //TODO check if this is correct or not
            if bindings::HAL_HasMain() != 0 {
                let robot_c = robot.clone();

                let thread = std::thread::spawn(move || {
                    let res = std::panic::catch_unwind(move || {
                        Self::run_robot(robot);
                    });

                    bindings::HAL_ExitMain();

                    match res {
                        Ok(_) => {}
                        Err(err) => {
                            std::panic::resume_unwind(err);
                        }
                    }
                });

                bindings::HAL_RunMain();

                robot_c.end_competion();

                thread
                    .join()
                    .expect("Failed to join robot thread in main thread");
            } else {
                Self::run_robot(robot);
            }

            bindings::HAL_Shutdown();
        }
    }

    fn run_robot<T: RobotBaseTrait>(robot: Arc<T>) {
        let res = std::panic::catch_unwind(|| {
            robot.start_competition();
        });

        match res {
            Ok(ok) => ok,
            Err(err) => {
                //TODO
                // frc_ReportError
                std::panic::resume_unwind(err);
            }
        }
    }

    pub fn get_native_robot_base(&self) -> &NativeRobotBase {
        &self.robot_base
    }

    pub fn is_teleop_enabled(&self) -> bool {
        unsafe { self.robot_base.IsTeleop() }
    }

    pub fn is_test(&self) -> bool {
        unsafe { self.robot_base.IsTest() }
    }

    pub fn get_thread_id(&self) -> std::thread::ThreadId {
        self.thread_id
    }
}
