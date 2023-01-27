

// #include <wpi/json.h>
// #include <wpi/json_serializer.h>



#include <hal/Accelerometer.h>
#include <hal/AddressableLED.h>
#include <hal/AddressableLEDTypes.h>
#include <hal/AnalogAccumulator.h>
#include <hal/AnalogGyro.h>
#include <hal/AnalogInput.h>
#include <hal/AnalogOutput.h>
#include <hal/AnalogTrigger.h>
#include <hal/CAN.h>
#include <hal/CANAPI.h>
#include <hal/CANAPITypes.h>
#include <hal/Constants.h>
#include <hal/Counter.h>
#include <hal/CTREPCM.h>
#include <hal/DIO.h>
#include <hal/DMA.h>
#include <hal/DriverStation.h>
#include <hal/DriverStationTypes.h>
#include <hal/DutyCycle.h>
#include <hal/Encoder.h>
#include <hal/Errors.h>
#include <hal/Extensions.h>
#include <hal/HAL.h>
#include <hal/HALBase.h>
#include <hal/I2C.h>
#include <hal/I2CTypes.h>
#include <hal/Interrupts.h>
#include <hal/Main.h>
#include <hal/Notifier.h>
#include <hal/Ports.h>
#include <hal/Power.h>
#include <hal/PowerDistribution.h>
#include <hal/PWM.h>
#include <hal/Relay.h>
#include <hal/REVPH.h>
#include <hal/SerialPort.h>
#include <hal/SimDevice.h>
#include <hal/SPI.h>
#include <hal/SPITypes.h>
#include <hal/Types.h>
#include <hal/Value.h>
#include <hal/HALBase.h>
#include <hal/Main.h>
#include <hal/Types.h>
#include <hal/Notifier.h>


#include <frc/AddressableLED.h>
#include <frc/ADXL345_I2C.h>
#include <frc/ADXL345_SPI.h>
#include <frc/ADXL362.h>
#include <frc/ADXRS450_Gyro.h>
#include <frc/AnalogAccelerometer.h>
#include <frc/AnalogEncoder.h>
#include <frc/AnalogGyro.h>
#include <frc/AnalogInput.h>
#include <frc/AnalogOutput.h>
#include <frc/AnalogPotentiometer.h>
#include <frc/AnalogTrigger.h>
#include <frc/AnalogTriggerOutput.h>
#include <frc/AnalogTriggerType.h>
#include <frc/AsynchronousInterrupt.h>
#include <frc/BuiltInAccelerometer.h>
#include <frc/CAN.h>
#include <frc/Compressor.h>
#include <frc/CompressorConfigType.h>
#include <frc/Counter.h>
#include <frc/CounterBase.h>
#include <frc/DigitalGlitchFilter.h>
#include <frc/DigitalInput.h>
#include <frc/DigitalOutput.h>
#include <frc/DigitalSource.h>
#include <frc/DMA.h>
#include <frc/DMASample.h>
#include <frc/DoubleSolenoid.h>
#include <frc/DriverStation.h>
#include <frc/DSControlWord.h>
#include <frc/DutyCycle.h>
#include <frc/DutyCycleEncoder.h>
#include <frc/Encoder.h>
#include <frc/Errors.h>
#include <frc/Filesystem.h>
#include <frc/GenericHID.h>
#include <frc/I2C.h>
#include <frc/IterativeRobotBase.h>
#include <frc/Joystick.h>
#include <frc/MotorSafety.h>
#include <frc/Notifier.h>
#include <frc/PneumaticHub.h>
#include <frc/PneumaticsBase.h>
#include <frc/PneumaticsControlModule.h>
#include <frc/PneumaticsModuleType.h>
#include <frc/PowerDistribution.h>
#include <frc/Preferences.h>
#include <frc/PS4Controller.h>
#include <frc/PWM.h>
#include <frc/Relay.h>
#include <frc/Resource.h>
#include <frc/RobotBase.h>
#include <frc/RobotController.h>
#include <frc/RobotState.h>
#include <frc/RuntimeType.h>
#include <frc/ScopedTracer.h>
#include <frc/SensorUtil.h>
#include <frc/SerialPort.h>
#include <frc/Servo.h>
#include <frc/Solenoid.h>
#include <frc/SPI.h>
#include <frc/SynchronousInterrupt.h>
#include <frc/Threads.h>
#include <frc/TimedRobot.h>
#include <frc/Timer.h>
#include <frc/TimesliceRobot.h>
#include <frc/Tracer.h>
#include <frc/Ultrasonic.h>
#include <frc/Watchdog.h>
#include <frc/XboxController.h>

#include <frc/util/Color.h>
#include <frc/util/Color8Bit.h>

#include <WPILibVersion.h>
// #include <frc/system/Discretization.h>
// #include <frc/system/LinearSystem.h>
// #include <frc/system/LinearSystemLoop.h>
// #include <frc/system/NumericalIntegration.h>
// #include <frc/system/NumericalJacobian.h>
#include <frc/system/plant/DCMotor.h>
// #include <frc/system/plant/LinearSystemId.h>

#include <frc/smartdashboard/Field2d.h>
#include <frc/smartdashboard/FieldObject2d.h>
#include <frc/smartdashboard/ListenerExecutor.h>
#include <frc/smartdashboard/Mechanism2d.h>
#include <frc/smartdashboard/MechanismLigament2d.h>
#include <frc/smartdashboard/MechanismObject2d.h>
#include <frc/smartdashboard/MechanismRoot2d.h>
#include <frc/smartdashboard/SendableBuilderImpl.h>
// #include <frc/smartdashboard/SendableChooser.h>
// #include <frc/smartdashboard/SendableChooser.inc>
// #include <frc/smartdashboard/SendableChooserBase.h>
// these will break it with weird template stuff
#include <frc/smartdashboard/SmartDashboard.h>


#include <ntcore.h>
#include <ntcore_c.h>
#include <ntcore_cpp.h>
#include <ntcore_cpp_types.h>
#include <ntcore_c_types.h>
#include <ntcore_test.h>

// #include<ntcore/networktables/NetworkTable.h>
// #include<ntcore/N>
// #include<networktables/BooleanArrayTopic.h>
// #include<networktables/IntegerArrayTopic.h>
#include<networktables/NetworkTableValue.h>
// #include<networktables/BooleanArrayTopic.inc>
// #include<networktables/IntegerArrayTopic.inc>
// #include<networktables/NTSendableBuilder.h>
// #include<networktables/BooleanTopic.h>
// #include<networktables/IntegerTopic.h>
// #include<networktables/NTSendable.h>
// #include<networktables/BooleanTopic.inc>
// #include<networktables/IntegerTopic.inc>
// #include<networktables/RawTopic.h>
// #include<networktables/DoubleArrayTopic.h>
// #include<networktables/MultiSubscriber.h>
// #include<networktables/RawTopic.inc>
// #include<networktables/DoubleArrayTopic.inc>
// #include<networktables/MultiSubscriber.inc>
// #include<networktables/StringArrayTopic.h>
// #include<networktables/DoubleTopic.h>
#include<networktables/NetworkTableEntry.h>
// #include<networktables/StringArrayTopic.inc>
// #include<networktables/DoubleTopic.inc>
// #include<networktables/NetworkTableEntry.inc>
// #include<networktables/StringTopic.h>
// #include<networktables/FloatArrayTopic.h>
// #include<networktables/NetworkTable.h>
// #include<networktables/StringTopic.inc>
// #include<networktables/FloatArrayTopic.inc>
#include<networktables/NetworkTableInstance.h>
// #include<networktables/Topic.h>
// #include<networktables/FloatTopic.h>
#include<networktables/NetworkTableInstance.inc>
// #include<networktables/Topic.inc>
// #include<networktables/FloatTopic.inc>
// #include<networktables/NetworkTableListener.h>
// #include<networktables/UnitTopic.h>
// #include<networktables/GenericEntry.h>
// #include<networktables/NetworkTableListener.inc>
// #include<networktables/UnitTopic.inc>
// #include<networktables/GenericEntry.inc>
// #include<networktables/NetworkTableType.h>
