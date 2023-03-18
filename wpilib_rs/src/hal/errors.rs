use std::{fmt::Display, error::Error};

macro_rules! GenerateErrors {
    ($macro_ident:ident, $($code_ident:ident = ($($code_expr:expr)?, $($msg_expr:expr)?) ,)*) => {
        
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        #[allow(unused)]
        pub struct $macro_ident{
            val: i32,
        }   

        $($(#[allow(unused, non_upper_case_globals)]pub static $code_ident: $macro_ident = $macro_ident::from_i32($code_expr);)?)*

        impl $macro_ident{
            #[allow(unused)]
            pub const fn get_raw(self) -> i32{
                self.val
            }

            #[allow(unused)]
            pub const fn get_name(self) -> Option<&'static str>{
                match self.val{
                    $($($code_expr => Some(stringify!($code_ident)),)?)*
                    _ => None

                }
            }

            #[allow(unused)]
            pub const fn from_i32(val: i32) -> Self{
                Self{
                    val
                }
            }
        }
    };
}


impl HalError{
    pub fn get_message(&self) -> Option<&'static str>{
        let code = self.get_raw();
        unsafe{
            let msg = crate::bindings::HAL_GetErrorMessage(code);
            if let Ok(msg) = std::ffi::CStr::from_ptr(msg).to_str(){
                Some(msg)
            }else{
                None
            }
        }
    }
}

impl Display for HalError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.get_raw();
        if let Some(ident) = self.get_name(){
            if let Some(msg) = self.get_message(){
                write!(f, "HALError: {ident}({code}) -> {msg}")
            }else{
                write!(f, "HALError: Unknown({code})")
            }
        }else{
            write!(f, "HALError: Unknown({code})")
        }
    }
}

impl Error for HalError{
}


GenerateErrors!(HalError, 

    HAL_SUCCESS = (0, ),

    CTR_RxTimeout_MESSAGE = (,"CTRE CAN Receive Timeout"),
    CTR_TxTimeout_MESSAGE = (,"CTRE CAN Transmit Timeout"),
    CTR_InvalidParamValue_MESSAGE = (,"CTRE CAN Invalid Parameter"),
    CTR_UnexpectedArbId_MESSAGE = (,"CTRE Unexpected Arbitration ID (CAN Node ID)"),
    CTR_TxFailed_MESSAGE = (,"CTRE CAN Transmit Error"),
    CTR_SigNotUpdated_MESSAGE = (,"CTRE CAN Signal Not Updated"),
    
    NiFpga_Status_FifoTimeout = (-50400, "NIFPGA: FIFO timeout error"),
    NiFpga_Status_TransferAborted = (-50405, "NIFPGA: Transfer aborted error"),
    NiFpga_Status_MemoryFull = (-52000, "NIFPGA: Memory Allocation failed, memory full"),
    NiFpga_Status_SoftwareFault = (-52003, "NIFPGA: Unexpected software error"),
    NiFpga_Status_InvalidParameter = (-52005, "NIFPGA: Invalid Parameter"),
    NiFpga_Status_ResourceNotFound = (-52006, "NIFPGA: Resource not found"),
    NiFpga_Status_ResourceNotInitialized = (-52010, "NIFPGA: Resource not initialized"),
    NiFpga_Status_HardwareFault = (-63150, "NIFPGA: Hardware Fault"),
    NiFpga_Status_IrqTimeout = (-61060, "NIFPGA: Interrupt timeout"),

    ERR_CANSessionMux_InvalidBuffer = (-44086, "CAN: Invalid Buffer"),
    ERR_CANSessionMux_MessageNotFound = (-44087, "CAN: Message not found"),
    WARN_CANSessionMux_NoToken = (44087, "CAN: No token"),
    ERR_CANSessionMux_NotAllowed = (-44088, "CAN: Not allowed"),
    ERR_CANSessionMux_NotInitialized = (-44089, "CAN: Not initialized"),

    ERR_FRCSystem_NetCommNotResponding = (-44049, "FRCSystem: NetComm not responding"),
    ERR_FRCSystem_NoDSConnection = (-44018, "FRCSystem: No driver station connected"),

    SAMPLE_RATE_TOO_HIGH = (1001, "HAL: Analog module sample rate is too high"),
    VOLTAGE_OUT_OF_RANGE = (1002, "HAL: Voltage to convert to raw value is out of range [0), 5]"),
    LOOP_TIMING_ERROR = (1004, "HAL: Digital module loop timing is not the expected value"),
    SPI_WRITE_NO_MOSI = (1012, "HAL: Cannot write to SPI port with no MOSI output"),
    SPI_READ_NO_MISO = (1013, "HAL: Cannot read from SPI port with no MISO input"),
    SPI_READ_NO_DATA = (1014, "HAL: No data available to read from SPI"),
    INCOMPATIBLE_STATE = (1015, "HAL: Incompatible State: The operation cannot be completed"),
    NO_AVAILABLE_RESOURCES = (-1004, "HAL: No available resources to allocate"),
    NULL_PARAMETER = (-1005, "HAL: A pointer parameter to a method is NULL"),
    ANALOG_TRIGGER_LIMIT_ORDER_ERROR = (-1010, "HAL: AnalogTrigger limits error.  Lower limit > Upper Limit"),
    ANALOG_TRIGGER_PULSE_OUTPUT_ERROR = (-1011, "HAL: Attempted to read AnalogTrigger pulse output."),
    PARAMETER_OUT_OF_RANGE = (-1028, "HAL: A parameter is out of range."),
    RESOURCE_IS_ALLOCATED = (-1029, "HAL: Resource already allocated"),
    RESOURCE_OUT_OF_RANGE = (-1030, "HAL: The requested resource is out of range."),
    HAL_INVALID_ACCUMULATOR_CHANNEL = (-1035, "HAL: The requested input is not an accumulator channel"),
    HAL_COUNTER_NOT_SUPPORTED = (-1058, "HAL: Counter mode not supported for encoder method"),
    HAL_PWM_SCALE_ERROR = (-1072, "HAL: The PWM Scale Factors are out of range"),
    HAL_HANDLE_ERROR = (-1098, "HAL: A handle parameter was passed incorrectly"),

    HAL_LED_CHANNEL_ERROR = (-1099, "HAL: Addressable LEDs only supported on PWM Headers, not MXP or DIO"),

    HAL_INVALID_DMA_ADDITION = (-1102, "HAL_AddDMA(), only works before HAL_StartDMA(),"),

    HAL_INVALID_DMA_STATE = (-1103, "HAL_SetPause(), only works before HAL_StartDMA(),"),

    HAL_SERIAL_PORT_NOT_FOUND = (-1123, "HAL: The specified serial port device was not found"),

    HAL_SERIAL_PORT_OPEN_ERROR = (-1124, "HAL: The serial port could not be opened"),

    HAL_SERIAL_PORT_ERROR = (-1125, "HAL: There was an error on the serial port"),

    HAL_THREAD_PRIORITY_ERROR = (-1152, "HAL: Getting or setting the priority of a thread has failed"),

    HAL_THREAD_PRIORITY_RANGE_ERROR = (-1153, "HAL: The priority requested to be set is invalid"),

    HAL_CAN_TIMEOUT = (-1154, "HAL: CAN Receive has Timed Out"),

    HAL_SIM_NOT_SUPPORTED = (-1155, "HAL: Method not supported in sim"),

    HAL_USE_LAST_ERROR = (-1156, "HAL: Use HAL_GetLastError(status), to get last error"),

    HAL_CONSOLE_OUT_ENABLED_ERROR = (-1157, "HAL: Onboard serial port is requested, but Console Out is enabled. Disable Console Out using imaging tool"),

    HAL_CAN_BUFFER_OVERRUN = (-35007, "HAL: CAN Output Buffer Full. Ensure a device is attached"),

    VI_ERROR_SYSTEM_ERROR_MESSAGE = (, "HAL: i32 = - VISA: System Error"),
    VI_ERROR_INV_OBJECT_MESSAGE = (, "HAL: i32 = - VISA: Invalid Object"),
    VI_ERROR_RSRC_LOCKED_MESSAGE = (, "HAL: i32 = - VISA: Resource Locked"),
    VI_ERROR_RSRC_NFOUND_MESSAGE = (, "HAL: i32 = - VISA: Resource Not Found"),
    VI_ERROR_INV_RSRC_NAME_MESSAGE = (, "HAL: i32 = - VISA: Invalid Resource Name"),
    VI_ERROR_QUEUE_OVERFLOW_MESSAGE = (, "HAL: i32 = - VISA: Queue Overflow"),
    VI_ERROR_IO_MESSAGE = (, "HAL: i32 = - VISA: General IO Error"),
    VI_ERROR_ASRL_PARITY_MESSAGE = (, "HAL: i32 = - VISA: Parity Error"),
    VI_ERROR_ASRL_FRAMING_MESSAGE = (, "HAL: i32 = - VISA: Framing Error"),
    VI_ERROR_ASRL_OVERRUN_MESSAGE = (, "HAL: i32 = - VISA: Buffer Overrun Error"),
    VI_ERROR_RSRC_BUSY_MESSAGE = (, "HAL: i32 = - VISA: Resource Busy"),
    VI_ERROR_INV_PARAMETER_MESSAGE = (, "HAL: i32 = - VISA: Invalid Parameter"),
);