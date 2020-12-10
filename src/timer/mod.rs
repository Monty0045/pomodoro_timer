use std::thread;
use std::time::Duration;

pub struct Timer
{
    count : i32,
}


impl Timer
{
    pub fn new() -> Self
    {
        println!("This happens");

        thread::spawn(|| {
            Timer::timer();
        });


        Self {count : 0}
    }

    pub fn messageSent(self: &mut Self, mes : &str)
    {
        println!("Within timer {}", mes);
        println!("Does this happen??");

        if let "play" = mes
        {
            self.count+=1;
        }
        else if let "reset" = mes
        {
            self.count-=1;
        }

        println!("Count equals {}", self.count);


    }

    //Actually timer updating the clock
    fn timer()
    {
        loop{
            thread::sleep(Duration::from_millis(1000));
            println!("Timer ran");
        }

    }

}

pub fn testing()
{

    println!("From within timer!");
}

pub fn what() -> Timer
{

    return Timer::new();

}
