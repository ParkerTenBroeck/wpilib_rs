use std::{
    collections::BinaryHeap,
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Duration,
};

use crate::{
    bindings::{self},
    frc::watchdog::WatchdogOverrun,
    robot_base::RobotBaseTrait,
};

pub trait TimedRobotTrait: Send + 'static + Sized {
    const P: std::time::Duration = std::time::Duration::from_millis(20);

    fn robot_init(context: &TimedRobot<Self>) -> Self;
    fn simulation_init(&mut self, _context: &TimedRobot<Self>) {}
    fn auton_init(&mut self, _context: &TimedRobot<Self>) {}
    fn disabled_init(&mut self, _context: &TimedRobot<Self>) {}
    fn teleop_init(&mut self, _context: &TimedRobot<Self>) {}
    fn test_init(&mut self, _context: &TimedRobot<Self>) {}

    fn robot_periodic(&mut self, _context: &TimedRobot<Self>) {}
    fn simulation_periodic(&mut self, _context: &TimedRobot<Self>) {}
    fn disabled_periodic(&mut self, _context: &TimedRobot<Self>) {}
    fn auton_periodic(&mut self, _context: &TimedRobot<Self>) {}
    fn teleop_periodic(&mut self, _context: &TimedRobot<Self>) {}
    fn test_periodic(&mut self, _context: &TimedRobot<Self>) {}

    fn disabled_exit(&mut self, _context: &TimedRobot<Self>) {}
    fn auton_exit(&mut self, _context: &TimedRobot<Self>) {}
    fn teleop_exit(&mut self, _context: &TimedRobot<Self>) {}
    fn test_exit(&mut self, _context: &TimedRobot<Self>) {}

    fn time_overrun(&mut self, _context: &TimedRobot<Self>, timings: WatchdogOverrun) {
        crate::ReportHalError!(crate::frc::errors::Errors::Error, "{}", timings);
    }

    fn start_robot() {
        crate::robot_base::RobotBase::start_robot::<TimedRobot<Self>>();
    }
}

struct Callback {
    period: Duration,
    expiration_time: Duration,
    func: Box<dyn FnMut() + 'static + Send + std::panic::UnwindSafe>,
}

impl PartialEq for Callback {
    fn eq(&self, other: &Self) -> bool {
        self.expiration_time == other.expiration_time
    }
}

impl Eq for Callback {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Callback {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.expiration_time.partial_cmp(&other.expiration_time)
    }
}

impl Ord for Callback {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.expiration_time.cmp(&other.expiration_time)
    }
}

struct NotifierHandle(i32);

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq)]
enum Mode {
    #[default]
    None,
    Disabled,
    Auton,
    Teleop,
    Test,
}

struct RobotLoopState<T> {
    robot: Option<T>,
    last_mode: Mode,
    nt_flush: bool,
}

impl<T: TimedRobotTrait> RobotLoopState<T> {
    pub fn get_robot(&mut self, context: &TimedRobot<T>) -> &mut T {
        if self.robot.is_none() {
            self.robot = Some(T::robot_init(context));
        }
        self.robot.as_mut().unwrap()
    }
}

impl<T> Default for RobotLoopState<T> {
    fn default() -> Self {
        Self {
            robot: None,
            last_mode: Mode::None,
            nt_flush: true,
        }
    }
}

pub type Time = Duration;

pub struct TimedRobot<T: TimedRobotTrait> {
    callbacks: Mutex<BinaryHeap<Callback>>,
    notifier: NotifierHandle,
    exit: AtomicBool,
    robot: crate::robot_base::RobotBase,
    timed_robot: Mutex<RobotLoopState<T>>,
}

impl<T: TimedRobotTrait> TimedRobot<T> {
    pub fn add_periodic(
        &self,
        callback: impl FnMut() + 'static + Send + std::panic::UnwindSafe,
        period: Time,
    ) {
        let mut guard = self.callbacks.lock().unwrap();

        let start_time = unsafe { bindings::frc_RobotController_GetFPGATime() };
        let expiration_time = Time::from_micros(start_time) + period;

        guard.push(Callback {
            period,
            expiration_time,
            func: Box::new(callback),
        });
    }

    pub fn get_robot_base(&self) -> &crate::robot_base::RobotBase {
        &self.robot
    }

    fn loop_func(&self) {
        let mut watchdog = crate::frc::watchdog::Watchdog::start(T::P);

        watchdog.add_epoch("DriverStation::RefreshData");
        unsafe {
            bindings::frc_DriverStation_RefreshData();
        }
        watchdog.end_epoch();

        let mode = unsafe {
            let control = bindings::frc_DSControlWord::new();
            if control.IsDisabled() {
                Mode::Disabled
            } else if control.IsAutonomous() {
                Mode::Auton
            } else if control.IsTeleop() {
                Mode::Teleop
            } else if control.IsTest() {
                Mode::Test
            } else {
                Mode::None
            }
        };

        let mut lock = self.timed_robot.lock().unwrap();
        if mode != lock.last_mode {
            match lock.last_mode {
                Mode::None => {}
                Mode::Disabled => {
                    watchdog.add_epoch("disabled_exit");
                    lock.get_robot(self).disabled_exit(self)
                }
                Mode::Auton => {
                    watchdog.add_epoch("auton_exit");
                    lock.get_robot(self).auton_exit(self)
                }
                Mode::Teleop => {
                    watchdog.add_epoch("teleop_exit");
                    lock.get_robot(self).teleop_exit(self)
                }
                Mode::Test => {
                    watchdog.add_epoch("test_exit");
                    lock.get_robot(self).test_exit(self)
                }
            }

            match mode {
                Mode::None => {}
                Mode::Disabled => {
                    watchdog.add_epoch("disabled_init");
                    lock.get_robot(self).disabled_init(self)
                }
                Mode::Auton => {
                    watchdog.add_epoch("auton_init");
                    lock.get_robot(self).auton_init(self)
                }
                Mode::Teleop => {
                    watchdog.add_epoch("teleop_init");
                    lock.get_robot(self).teleop_init(self)
                }
                Mode::Test => {
                    watchdog.add_epoch("test_init");
                    lock.get_robot(self).test_init(self)
                }
            }

            lock.last_mode = mode;
        }

        match mode {
            Mode::None => {}
            Mode::Disabled => {
                watchdog.add_epoch("disabled_periodic");
                unsafe { bindings::HAL_ObserveUserProgramDisabled() }
                lock.get_robot(self).disabled_periodic(self)
            }
            Mode::Auton => {
                watchdog.add_epoch("auton_periodic");
                unsafe { bindings::HAL_ObserveUserProgramAutonomous() }
                lock.get_robot(self).auton_periodic(self)
            }
            Mode::Teleop => {
                watchdog.add_epoch("teleop_periodic");
                unsafe { bindings::HAL_ObserveUserProgramTeleop() }
                lock.get_robot(self).teleop_periodic(self)
            }
            Mode::Test => {
                watchdog.add_epoch("test_periodic");
                unsafe { bindings::HAL_ObserveUserProgramTeleop() }
                lock.get_robot(self).test_periodic(self)
            }
        }

        watchdog.add_epoch("robot_periodic");
        lock.get_robot(self).robot_periodic(self);

        unsafe {
            watchdog.add_epoch("SmartDashBoard::UpdateValues");
            bindings::frc_SmartDashboard::UpdateValues();
            watchdog.add_epoch("LiveWindow::UpdateValues");
            bindings::frc_LiveWindow::UpdateValues();
            watchdog.add_epoch("Shuffleboard::Update");
            bindings::frc_Shuffleboard::Update();

            if lock.nt_flush {
                watchdog.add_epoch("NetworkTables::FlushLocal");
                //nt::NetworkTableInstance::GetDefault().FlushLocal();
                bindings::nt_FlushLocal(bindings::nt_GetDefaultInstance());
            }
        }

        match watchdog.end() {
            Ok(ok) => {
                crate::frc::smart_dashboard::put_number("LoopTime(ms)", ok.as_secs_f64() * 1000.0);
            }
            Err(err) => {
                lock.get_robot(self).time_overrun(self, err);
            }
        }
    }
}

impl<T: TimedRobotTrait> RobotBaseTrait for TimedRobot<T> {
    fn new(robot: crate::robot_base::RobotBase) -> Arc<Self> {
        unsafe {
            let mut status = 0;
            let notifier = bindings::HAL_InitializeNotifier((&mut status) as *mut i32);
            crate::CheckErrorStatus!(status, "InitializeNotifier");

            bindings::HAL_SetNotifierName(
                notifier,
                b"TimedRobot\0".as_ptr().cast(),
                (&mut status) as *mut i32,
            );
            bindings::HAL_Report(
                bindings::HALUsageReporting_tResourceType_kResourceType_Framework,
                bindings::HALUsageReporting_tInstances_kFramework_Timed,
                0,
                std::ptr::null(),
            );

            let s = Self {
                callbacks: Mutex::new(BinaryHeap::new()),
                notifier: NotifierHandle(notifier),
                exit: false.into(),
                robot,
                timed_robot: Mutex::new(RobotLoopState::default()),
            };
            let arc = Arc::new(s);
            let arc_c = arc.clone();
            arc.add_periodic(move || arc_c.loop_func(), T::P);

            arc
        }
    }

    fn start_competition(self: Arc<Self>) {
        unsafe {
            println!("\n********** Robot program startup complete **********");
            bindings::HAL_ObserveUserProgramStarting();

            while !self.exit.load(std::sync::atomic::Ordering::Relaxed) {
                let mut callback = self
                    .callbacks
                    .lock()
                    .unwrap()
                    .pop()
                    .expect("Expected at least one callback");

                let mut status = 0i32;

                bindings::HAL_UpdateNotifierAlarm(
                    self.notifier.0,
                    callback.expiration_time.as_micros() as u64,
                    (&mut status) as *mut i32,
                );
                crate::CheckErrorStatus!(status, "UpdateNotifierAlarm");

                let curtime =
                    bindings::HAL_WaitForNotifierAlarm(self.notifier.0, (&mut status) as *mut i32);
                if curtime == 0 || status != 0 {
                    break;
                }

                (callback.func)();
                callback.expiration_time += callback.period;
                self.callbacks.lock().unwrap().push(callback);

                let mut guard = self.callbacks.lock().unwrap();
                while guard
                    .peek()
                    .expect("Expect at least One call back")
                    .expiration_time
                    .as_micros() as u64
                    <= curtime
                {
                    let mut callback = guard.pop().unwrap();
                    drop(guard);

                    (callback.func)();
                    callback.expiration_time += callback.period;

                    guard = self.callbacks.lock().unwrap();
                    guard.push(callback);
                }
            }
        };
    }

    fn end_competion(self: Arc<Self>) {
        self.exit.store(true, std::sync::atomic::Ordering::Relaxed);
        let mut status = 0i32;
        unsafe {
            bindings::HAL_StopNotifier(self.notifier.0, (&mut status) as *mut i32);
        }
    }
}

impl<T: TimedRobotTrait> Drop for TimedRobot<T> {
    fn drop(&mut self) {
        self.exit.store(true, std::sync::atomic::Ordering::Relaxed);
        let mut status = 0i32;
        unsafe {
            bindings::HAL_StopNotifier(self.notifier.0, (&mut status) as *mut i32);
        }

        let mut status = 0i32;
        unsafe {
            bindings::HAL_CleanNotifier(self.notifier.0, (&mut status) as *mut i32);
        }
    }
}
