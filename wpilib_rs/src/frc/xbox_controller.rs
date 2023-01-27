use crate::bindings::frc::XboxController as NativeXboxController;

pub enum Port {
    Port0 = 0,
    Port1 = 1,
    Port2 = 2,
    Port3 = 3,
    Port4 = 4,
    Port5 = 5,
}

pub struct XboxController {
    internal: NativeXboxController,
}

unsafe impl Send for XboxController {}

impl XboxController {
    pub fn new(port: Port) -> Self {
        unsafe {
            Self {
                internal: NativeXboxController::new(port as i32),
            }
        }
    }

    pub fn get_left_x(&mut self) -> f64 {
        unsafe { self.internal.GetLeftX() }
    }

    pub fn get_left_y(&mut self) -> f64 {
        unsafe { self.internal.GetLeftY() }
    }

    pub fn get_right_x(&mut self) -> f64 {
        unsafe { self.internal.GetRightX() }
    }

    pub fn get_right_y(&mut self) -> f64 {
        unsafe { self.internal.GetRightY() }
    }

    pub fn get_left_trigger_axis(&mut self) -> f64 {
        unsafe { self.internal.GetLeftTriggerAxis() }
    }

    pub fn get_right_trigger_axis(&mut self) -> f64 {
        unsafe { self.internal.GetRightTriggerAxis() }
    }

    pub fn get_a_button(&mut self) -> bool {
        unsafe { self.internal.GetAButton() }
    }

    pub fn get_a_button_pressed(&mut self) -> bool {
        unsafe { self.internal.GetAButtonPressed() }
    }

    pub fn get_a_button_released(&mut self) -> bool {
        unsafe { self.internal.GetAButtonReleased() }
    }

    pub fn get_b_button(&mut self) -> bool {
        unsafe { self.internal.GetBButton() }
    }

    pub fn get_b_button_pressed(&mut self) -> bool {
        unsafe { self.internal.GetBButtonPressed() }
    }

    pub fn get_b_button_released(&mut self) -> bool {
        unsafe { self.internal.GetBButtonReleased() }
    }

    pub fn get_back_button(&mut self) -> bool {
        unsafe { self.internal.GetBackButton() }
    }

    pub fn get_back_button_pressed(&mut self) -> bool {
        unsafe { self.internal.GetBackButtonPressed() }
    }

    pub fn get_back_button_released(&mut self) -> bool {
        unsafe { self.internal.GetBackButtonReleased() }
    }

    pub fn get_left_bumper(&mut self) -> bool {
        unsafe { self.internal.GetLeftBumper() }
    }

    pub fn get_left_bumper_pressed(&mut self) -> bool {
        unsafe { self.internal.GetLeftBumperPressed() }
    }

    pub fn get_left_bumper_released(&mut self) -> bool {
        unsafe { self.internal.GetLeftBumperReleased() }
    }

    pub fn get_left_stick_button(&mut self) -> bool {
        unsafe { self.internal.GetLeftStickButton() }
    }

    pub fn get_left_stick_button_pressed(&mut self) -> bool {
        unsafe { self.internal.GetLeftStickButtonPressed() }
    }

    pub fn get_left_stick_button_released(&mut self) -> bool {
        unsafe { self.internal.GetLeftStickButtonReleased() }
    }

    pub fn get_right_bumper(&mut self) -> bool {
        unsafe { self.internal.GetRightBumper() }
    }

    pub fn get_right_bumper_pressed(&mut self) -> bool {
        unsafe { self.internal.GetRightBumperPressed() }
    }

    pub fn get_right_bumber_released(&mut self) -> bool {
        unsafe { self.internal.GetRightBumperReleased() }
    }

    pub fn get_right_stick_button(&mut self) -> bool {
        unsafe { self.internal.GetRightStickButton() }
    }

    pub fn get_right_stick_button_pressed(&mut self) -> bool {
        unsafe { self.internal.GetRightStickButtonPressed() }
    }

    pub fn get_right_stick_button_released(&mut self) -> bool {
        unsafe { self.internal.GetRightStickButtonReleased() }
    }

    pub fn get_start_button(&mut self) -> bool {
        unsafe { self.internal.GetStartButton() }
    }

    pub fn get_start_button_pressed(&mut self) -> bool {
        unsafe { self.internal.GetStartButtonPressed() }
    }

    pub fn get_start_button_released(&mut self) -> bool {
        unsafe { self.internal.GetStartButtonReleased() }
    }

    pub fn get_x_button(&mut self) -> bool {
        unsafe { self.internal.GetXButton() }
    }

    pub fn get_x_button_pressed(&mut self) -> bool {
        unsafe { self.internal.GetXButtonPressed() }
    }

    pub fn get_x_button_released(&mut self) -> bool {
        unsafe { self.internal.GetXButtonReleased() }
    }

    pub fn get_y_button(&mut self) -> bool {
        unsafe { self.internal.GetYButton() }
    }

    pub fn get_y_button_pressed(&mut self) -> bool {
        unsafe { self.internal.GetYButtonPressed() }
    }

    pub fn get_y_button_released(&mut self) -> bool {
        unsafe { self.internal.GetYButtonReleased() }
    }
}
