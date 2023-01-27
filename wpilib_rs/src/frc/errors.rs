pub enum Warnings {
    SampleRateTooHigh = 1,
    VoltageOutOfRange = 2,
    CompressorTaskError = 3,
    LoopTimingError = 4,
    NonBinaryDigitalValue = 5,
    IncorrectBatteryChannel = 6,
    BadJoystickIndex = 7,
    BadJoystickAxis = 8,
    InvalidMotorIndex = 9,
    DriverStationTaskError = 10,
    EnhancedIOPWMPeriodOutOfRange = 11,
    SPIWriteNoMOSI = 12,
    SPIReadNoMISO = 13,
    SPIReadNoData = 14,
    IncompatibleState = 15,
    Warning = 16,
}

pub enum Errors {
    ModuleIndexOutOfRange = -1,
    ChannelIndexOutOfRange = -45,
    NotAllocated = -2,
    ResourceAlreadyAllocated = -3,
    NoAvailableResources = -4,
    NullParameter = -5,
    Timeout = -6,
    CompassManufacturerError = -7,
    CompassTypeError = -8,
    IncompatibleMode = -9,
    AnalogTriggerLimitOrderError = -10,
    AnalogTriggerPulseOutputError = -11,
    TaskError = -12,
    TaskIDError = -13,
    TaskDeletedError = -14,
    TaskOptionsError = -15,
    TaskMemoryError = -16,
    TaskPriorityError = -17,
    DriveUninitialized = -18,
    CompressorNonMatching = -19,
    CompressorAlreadyDefined = -20,
    CompressorUndefined = -21,
    InconsistentArrayValueAdded = -22,
    MismatchedComplexTypeClose = -23,
    DashboardDataOverflow = -24,
    DashboardDataCollision = -25,
    EnhancedIOMissing = -26,
    LineNotOutput = -27,
    ParameterOutOfRange = -28,
    SPIClockRateTooLow = -29,
    JaguarVersionError = -30,
    JaguarMessageNotFound = -31,
    NetworkTablesReadError = -40,
    NetworkTablesBufferFull = -41,
    NetworkTablesWrongType = -42,
    NetworkTablesCorrupt = -43,
    SmartDashboardMissingKey = -44,
    CommandIllegalUse = -50,
    UnsupportedInSimulation = -80,
    CameraServerError = -90,
    InvalidParameter = -100,
    AssertionFailure = -110,
    Error = -111,
}

#[macro_export]
macro_rules! ReportHalErrorBackTrace {
    ($status:expr) => {{
        let location = std::format!("{}:{}:{}\0", file!(), line!(), column!());
        let mut backtrace = std::backtrace::Backtrace::force_capture().to_string();
        backtrace.push('\0');
        let status = $status as i32;
        $crate::bindings::HAL_SendError(i32::from(status < 0), status, 0,
            "\0".as_ptr().cast(), location.as_ptr().cast(), backtrace.as_ptr().cast(),1);
    }};
    ($status:expr, $($arg:tt)*) => {{
        let mut details = std::format!($($arg)*);
        details.push('\0');
        let location = std::format!("{}:{}:{}\0", file!(), line!(), column!());
        let mut backtrace = std::backtrace::Backtrace::force_capture().to_string();
        backtrace.push('\0');
        let status = $status as i32;
        $crate::bindings::HAL_SendError(i32::from(status < 0), status, 0,
        details.as_ptr().cast(), location.as_ptr().cast(), backtrace.as_ptr().cast(),1);
    }};
}

#[macro_export]
macro_rules! ReportHalError {
    ($status:expr) => {{
        let location = std::format!("{}:{}:{}\0", file!(), line!(), column!());
        let status = $status as i32;
        $crate::bindings::HAL_SendError(i32::from(status < 0), status, 0,
            "\0".as_ptr().cast(), location.as_ptr().cast(), "\0".as_ptr().cast(),1);
    }};
    ($status:expr, $($arg:tt)*) => {{
        let mut details = std::format!($($arg)*);
        details.push('\0');
        let location = std::format!("{}:{}:{}\0", file!(), line!(), column!());
        let status = $status as i32;
        $crate::bindings::HAL_SendError(i32::from(status < 0), status, 0,
        details.as_ptr().cast(), location.as_ptr().cast(), "\0".as_ptr().cast(),1);
    }};
}

#[macro_export]
macro_rules! CheckErrorStatus {
    ($status:expr) => {{
        let status = $status as i32;
        if status < 0{
            $crate::ReportHalError!($status);
            panic!();
        }else if status > 0{
            $crate::ReportHalError!($status);
        }
    }};
    ($status:expr, $($arg:tt)*) => {{
        let status = $status as i32;
        if status < 0{
            $crate::ReportHalError!($status, $($arg)*);
            panic!($($arg)*);
        }else if status > 0{
            $crate::ReportHalError!($status, $($arg)*);
        }
    }};
}

impl Warnings {
    pub fn to_str(&self) -> &'static str {
        use Warnings::*;
        match self {
            SampleRateTooHigh => "Analog module sample rate is too high",
            VoltageOutOfRange => "Voltage to convert to raw value is out of range [-10; 10]",
            CompressorTaskError => "Compressor task won't start",
            LoopTimingError => "Digital module loop timing is not the expected value",
            NonBinaryDigitalValue => "Digital output value is not 0 or 1",
            IncorrectBatteryChannel => "Battery measurement channel is not correct value",
            BadJoystickIndex => "Joystick index is out of range = should be 0-5",
            BadJoystickAxis => "Joystick axis or POV is out of range",
            InvalidMotorIndex => "Motor index is out of range = should be 0-3",
            DriverStationTaskError => "Driver Station task won't start",
            EnhancedIOPWMPeriodOutOfRange => {
                "Driver Station Enhanced IO PWM Output period out of range"
            }
            SPIWriteNoMOSI => "Cannot write to SPI port with no MOSI output",
            SPIReadNoMISO => "Cannot read from SPI port with no MISO input",
            SPIReadNoData => "No data available to read from SPI",
            IncompatibleState => "Incompatible State: The operation cannot be completed",
            Warning => "Warning",
        }
    }
}

impl Errors {
    pub fn to_str(&self) -> &'static str {
        use Errors::*;
        match self{
            ModuleIndexOutOfRange => "Allocating module that is out of range or not found",
            ChannelIndexOutOfRange => "Allocating channel that is out of range",
            NotAllocated => "Attempting to free unallocated resource",
            ResourceAlreadyAllocated => "Attempted to reuse an allocated resource",
            NoAvailableResources => "No available resources to allocate",
            NullParameter => "A pointer parameter to a method is nullptr",
            Timeout => "A timeout has been exceeded",
            CompassManufacturerError => "Compass manufacturer doesn't match HiTechnic",
            CompassTypeError => "Compass type doesn't match expected type for HiTechnic compass",
            IncompatibleMode => "The object is in an incompatible mode",
            AnalogTriggerLimitOrderError => "AnalogTrigger limits error.  Lower limit > Upper Limit",
            AnalogTriggerPulseOutputError => "Attempted to read AnalogTrigger pulse output",
            TaskError => "Task can't be started",
            TaskIDError => "Task error: Invalid ID",
            TaskDeletedError => "Task error: Task already deleted",
            TaskOptionsError => "Task error: Invalid options",
            TaskMemoryError => "Task can't be started due to insufficient memory",
            TaskPriorityError => "Task error: Invalid priority [1-255]",
            DriveUninitialized => "RobotDrive not initialized for the C interface",
            CompressorNonMatching => "Compressor slot/channel doesn't match previous instance",
            CompressorAlreadyDefined => "Creating a second compressor instance",
            CompressorUndefined => "Using compressor functions without defining compressor",
            InconsistentArrayValueAdded => "When packing data into an array to the dashboard = not all values added were of the same type",
            MismatchedComplexTypeClose => "When packing data to the dashboard = a Close for a complex type was called without a matching Open",
            DashboardDataOverflow => "When packing data to the dashboard = too much data was packed and the buffer overflowed",
            DashboardDataCollision => "The same buffer was used for packing data and for printing",
            EnhancedIOMissing => "IO is not attached or Enhanced IO is not enabled",
            LineNotOutput => "Cannot SetDigitalOutput for a line not configured for output",
            ParameterOutOfRange => "A parameter is out of range",
            SPIClockRateTooLow => "SPI clock rate was below the minimum supported",
            JaguarVersionError => "Jaguar firmware version error",
            JaguarMessageNotFound => "Jaguar message not found",
            NetworkTablesReadError => "Error reading NetworkTables socket",
            NetworkTablesBufferFull => "Buffer full writing to NetworkTables socket",
            NetworkTablesWrongType => "The wrong type was read from the NetworkTables entry",
            NetworkTablesCorrupt => "NetworkTables data stream is corrupt",
            SmartDashboardMissingKey => "SmartDashboard data does not exist",
            CommandIllegalUse => "Illegal use of Command",
            UnsupportedInSimulation => "Unsupported in simulation",
            CameraServerError => "CameraServer error",
            InvalidParameter => "Invalid parameter value",
            AssertionFailure => "Assertion failed",
            Error => "Error",
        }
    }
}
