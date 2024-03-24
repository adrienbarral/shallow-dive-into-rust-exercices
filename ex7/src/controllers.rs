use std::{num::ParseFloatError, thread, time::Duration};

use emulator::SerialPort;

use crate::emulator;

struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
}
trait MotorController {
    fn set_speed(&mut self, speed_ms: f32);
    fn get_position(&mut self) -> Result<f32, ParseFloatError>;
    fn get_pid(&self) -> &PID;
}

struct ACMotorController<'a> {
    serial_port: &'a mut dyn SerialPort,
    pid: PID,
}

impl<'a> MotorController for ACMotorController<'a> {
    fn set_speed(&mut self, speed_ms: f32) {
        todo!()
    }
    fn get_position(&mut self) -> Result<f32, ParseFloatError> {
        todo!()
    }
    fn get_pid(&self) -> &PID {
        todo!()
    }
}

struct DCMotorController<'a> {
    serial_port: &'a mut dyn SerialPort,
    pid: PID,
}

impl<'a> MotorController for DCMotorController<'a> {
    fn set_speed(&mut self, speed_ms: f32) {
        todo!();
    }
    fn get_position(&mut self) -> Result<f32, ParseFloatError> {
        todo!()
    }
    fn get_pid(&self) -> &PID {
        todo!()
    }
}

pub fn controller_main(mut serial_port: Box<dyn SerialPort>) {
    let controller_type = serial_port.send_and_receive("?");
    // Construct the good controller (AC or DC Motor Controller) accordingly to the
    // response in the string controller_type. Store it in a Box.
    
    // Then call the infinite control loop :
}

fn control_loop(mut controller: Box<dyn MotorController>) {
    let command = 10.0_f32;

    loop {
        let error = command - controller.get_position().unwrap();
        let pid = controller.get_pid();
        controller.set_speed(pid.kp * error + pid.ki * error + pid.kd * error);
        thread::sleep(Duration::from_millis(10));
    }
}