use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

pub struct Timer
{
    count : i32,
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

    pub fn initiateTimer(self : &mut Self)
    {


        thread::spawn(|| {
            //self.timer();
        });
    }

    //Actual timer updating the clock
    fn timer(self : &mut Self)
    {
        loop{
            thread::sleep(Duration::from_millis(1000));
            /*
            let isCounting = self.isCounting.lock().unwrap();
            if(*isCounting)
            {
                //Self.count-=1;
            }
            */
            println!("Timer ran");
        }

    }

}

//Experimental function, pass in ARC of Timer class to initiate things
pub fn timerStart(timer : Arc<Mutex<Timer>>)
{
    thread::spawn(move || {
        loop{
            thread::sleep(Duration::from_millis(1000));

            let mut timer_copy = timer.lock().unwrap();

            if(timer_copy.isCounting)
            {
                timer_copy.count -=1;
                println!("Is counting.. {}", timer_copy.count);
            }

            println!("Timer ran");

            }

      });
}

pub fn testing()
{

    println!("From within timer!");
}

pub fn what() -> Timer
{

    return Timer::new();

}
