use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use rand::Rng;
use thiserror::Error;

trait Parser {
    fn parse(&self, msg: &str) -> anyhow::Result<Command>;
}

struct ACParser {}
impl Parser for ACParser {
    fn parse(&self, msg: &str) -> anyhow::Result<Command> {
        if msg == "?" {
            return Ok(Command::WHO_ARE_YOU);
        } else if msg.starts_with("SET_POINT_SPEED,") {
            let set_point = msg[16..].parse::<f32>()?;
            return Ok(Command::SET_SPEED(set_point));
        } else if msg.starts_with("POSITION?") {
            return Ok(Command::POSITION);
        }
        return Err(MessageParsingError::UnknowCommand.into());
    }
}

struct DCParser {}
impl Parser for DCParser {
    fn parse(&self, msg: &str) -> anyhow::Result<Command> {
        if msg == "?" {
            return Ok(Command::WHO_ARE_YOU);
        } else if msg.starts_with("SET_SPEED,") {
            let set_point = msg[10..].parse::<f32>()?;
            return Ok(Command::SET_SPEED(set_point));
        } else if msg.starts_with("POS?") {
            return Ok(Command::POSITION);
        }
        return Err(MessageParsingError::UnknowCommand.into());
    }
}
enum ControllerType {
    AC,
    DC,
}

pub fn init(tx_position: mpsc::Sender<f32>) -> Box<dyn SerialPort> {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.5) {
        Box::new(Controller::new(tx_position, ControllerType::AC))
    } else {
        Box::new(Controller::new(tx_position, ControllerType::DC))
    }
}
pub trait SerialPort {
    fn send_and_receive(&mut self, msg: &str) -> String;
}

#[derive(PartialEq, Debug)]
enum Command {
    WHO_ARE_YOU,
    SET_SPEED(f32),
    POSITION,
}

#[derive(Error, Debug)]
pub enum MessageParsingError {
    #[error("Unknow command")]
    UnknowCommand,
}

struct Controller {
    name: String,
    actual_position_m: Arc<Mutex<f32>>,
    speed_setpoint_ms: Arc<Mutex<f32>>,
    parser: Box<dyn Parser>,
}
impl Controller {
    pub fn new(tx_position: mpsc::Sender<f32>, controller_type: ControllerType) -> Self {
        let res = match controller_type {
            ControllerType::AC => Controller {
                name: "AC".to_string(),
                actual_position_m: Arc::new(Mutex::<f32>::new(0.)),
                speed_setpoint_ms: Arc::new(Mutex::<f32>::new(0.)),
                parser: Box::new(ACParser {}),
            },
            ControllerType::DC => Controller {
                name: "DC".to_string(),
                actual_position_m: Arc::new(Mutex::<f32>::new(0.)),
                speed_setpoint_ms: Arc::new(Mutex::<f32>::new(0.)),
                parser: Box::new(DCParser {}),
            },
        };
        let speed_sp = res.speed_setpoint_ms.clone();
        let actual_pos = res.actual_position_m.clone();

        let _ = thread::spawn(move || {
            let dt = Duration::from_millis(10);
            loop {
                sleep(dt);
                let mut position = *actual_pos.lock().unwrap();
                position += *speed_sp.lock().unwrap() * dt.as_secs_f32();
                if let Err(_) = tx_position.send(position) {
                    // Le channel est fermé, plus personne ne s'interesse à notre donnée ☹️. On sort
                    return;
                }
                *actual_pos.lock().unwrap() = position;
            }
        });

        res
    }
}
impl SerialPort for Controller {
    /*
    Communication protocol of a controller is as follow :
    DC Controller :
    ? will return name of the controller "DC" OR AC
    SPEED,<float> will start moving the motor at the desired speed in m/s as a float stringified. Will return "OK".
    POS? will return the actual position of the motor as a float stringified

    AC Controller :
    ? will return name of the controller "DC" OR AC
    SET_POINT_SPEED,<float> will start moving the motor at the desired speed in m/s as a float stringified. Will return "OK".
    POSITION? will return the actual position of the motor as a float stringified

    For both controllerd "PARSE ERROR" will be returned in case of malfromed message.
     */
    fn send_and_receive(&mut self, msg: &str) -> String {
        match self.parser.parse(msg) {
            Ok(Command::WHO_ARE_YOU) => self.name.clone(),
            Ok(Command::POSITION) => {
                format!("{}", self.actual_position_m.lock().unwrap())
            }
            Ok(Command::SET_SPEED(speed_setpoint)) => {
                *self.speed_setpoint_ms.lock().unwrap() = speed_setpoint;
                "OK".to_string()
            }
            _ => "PARSE ERROR".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn we_can_parse_command() {
        let dc_parser = DCParser {};
        assert_eq!(dc_parser.parse("?").unwrap(), Command::WHO_ARE_YOU);
        let speed_set = dc_parser.parse("SET_SPEED,12.2").unwrap();
        match speed_set {
            Command::SET_SPEED(speed) => assert_eq!((speed - 12.2).abs() < 0.001, true),
            _ => assert!(false, "Parse error"),
        };
        assert_eq!(dc_parser.parse("POS?").unwrap(), Command::POSITION);
    }

    #[test]
    pub fn we_can_parse_and_send_command_for_ac_controller() {
        let (tx, _) = mpsc::channel::<f32>();
        let mut controller = Controller::new(tx, ControllerType::AC);
        assert_eq!(controller.send_and_receive("?"), "AC");
        assert_eq!(controller.send_and_receive("SET_POINT_SPEED,2.56"), "OK");
        assert_eq!(
            (*controller.speed_setpoint_ms.lock().unwrap() - 2.56).abs() < 0.0001,
            true
        );
        assert_eq!(controller.send_and_receive("POSITION?"), "0");
    }
}
