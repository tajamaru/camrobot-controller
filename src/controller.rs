use gilrs::{Gilrs, Axis,Event,EventType,Button};
use serde::{Serialize,Deserialize};

pub trait Controller {
    fn next_event(&mut self) -> Option<Action>;
}

pub struct ProCon{
    pub gilrs : Gilrs,
}

//const PRO_CON_NAME : &'static str = "Nintendo Switch Pro Controller";
impl  ProCon {
    pub fn new() -> Result<Self,&'static str>{
        let err_msg = "Pro Con が認識できてません";
        Gilrs::new().map_err(|_| err_msg)
        .and_then(|gilrs|Ok(Self{gilrs}))
    }
    fn get_stick_y(&mut self, state:f32) -> (Rolling,MoterSpeed){
        let rolling = if state > 0.0 {
            Rolling::Normal
        } else {
            Rolling::Reverse
        };

        match state.abs()  {
             d if d < 0.1 => (rolling,MoterSpeed::Stop),
             d if d < 0.3 => (rolling,MoterSpeed::Slow),
             d if d < 0.5 => (rolling,MoterSpeed::Middle),
             d if d <  1.0 => (rolling,MoterSpeed::High), 
             _ => (rolling,MoterSpeed::Stop),
         }

    }
}
impl Controller for ProCon {
    fn next_event(&mut self) -> Option<Action> {
        self.gilrs.next_event()
                .map_or(None, |ev|{
                    match  ev {
                        Event{  event:EventType::AxisChanged(Axis::LeftStickY,val,_),..} => {
                            let (rolling,speed) = self.get_stick_y(val);
                            Some(Action::MoveLeftCrawler(rolling,speed))
                        },
                        Event{  event:EventType::AxisChanged(Axis::RightStickY,val,_),..} => {
                            let (rolling,speed) = self.get_stick_y(val);
                            Some(Action::MoveRightCrawler(rolling,speed))
                        },
                        Event{   event: EventType::ButtonPressed(Button::RightTrigger, _),..} => {
                            Some(Action::ToggleEye)
                        },
                        Event{ event: EventType::ButtonPressed(Button::LeftTrigger2, _),..} => {
                            Some(Action::End)
                        }
                        _ => None,
                    }
                })
    }
    
}

#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub enum Rolling{
    Normal,
    Reverse,
}
#[derive(Debug, PartialEq,Clone,Serialize,Deserialize)]
pub enum MoterSpeed{
    Stop,
    Slow,
    Middle,
    High,
}
#[derive(Debug,PartialEq,Clone,Serialize,Deserialize)]
pub enum Action{
    MoveRightCrawler(Rolling,MoterSpeed),
    MoveLeftCrawler(Rolling,MoterSpeed),
    ToggleEye,
    Stop,
    End
}
