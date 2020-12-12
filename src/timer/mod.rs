use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

pub struct Timer
{
    count : i32,
    sprintCount: u32,
    isBreak: bool,
    //isCounting : Arc<Mutex<bool>>,
    isCounting : bool,
}


impl Timer
{
    pub fn new() -> Self
    {
        println!("This happens");


        Self {
            count : 100,
            sprintCount: 0,
            isBreak: false,
            //isCounting: Arc::new(Mutex::new(false)),
            isCounting : false,
        }
    }

    pub fn messageSent(self: &mut Self, mes : &str)
    {
        println!("Within timer {}", mes);
        println!("Does this happen??");

        if let "play" = mes
        {

            if(self.isCounting)
            {
                self.isCounting = false;
            } else { self.isCounting = true;}

        }
        else if let "reset" = mes
        {
            self.count = 100;
        }

        println!("Count equals {}", self.count);


    }


}

//Experimental function, pass in ARC of Timer class to initiate things
//pub fn timerStart(timer : Arc<Mutex<Timer>>, messenger: mpsc::Sender<&str>) -> mpsc::Receiver<&str>
pub fn timerStart(timer : Arc<Mutex<Timer>>, messenger: mpsc::Sender<std::string::String>)
{
    //let (messenger, receiver) = mpsc::channel();

    countDown(timer, messenger);

    //return receiver;
}

fn countDown(timer : Arc<Mutex<Timer>>, messenger: mpsc::Sender<std::string::String>)
{
    thread::spawn(move || {

        loop{
            thread::sleep(Duration::from_millis(1000));

            let mut timer_copy = timer.lock().unwrap();

            if(timer_copy.isCounting)
            {
                timer_copy.count -=1;
                println!("Is counting.. {}", timer_copy.count);
                messenger.send(timer_copy.count.to_string());
            }

            println!("Timer ran");

            }

      });

}

