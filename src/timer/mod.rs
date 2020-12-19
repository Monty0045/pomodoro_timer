use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

pub struct Timer
{
    count : i32,
    sprintCount: u32,
    isBreak: bool,
    isCounting : bool,
}


impl Timer
{
    pub fn new() -> Self
    {
        Self {
            count : 10,
            sprintCount: 0,
            isBreak: false,
            isCounting : false,
        }
    }

    //message received from GUI
    pub fn messageSent(self: &mut Self, mes : &str)
    {

        if let "play" = mes
        {

            if(self.isCounting)
            {
                self.isCounting = false;
            } else { self.isCounting = true;}

        }
        else if let "reset" = mes
        {
            self.count = 10;
            self.isCounting = false;
            self.sprintCount = 0;
            self.isBreak = false;
        }


    }


}

//Initiates the pomodoro timer from outside, passees a message to GUI containing time and context
//  message
pub fn timerStart(timer : Arc<Mutex<Timer>>, messenger: glib::Sender<(i32, std::string::String)>)
{
    thread::spawn(move || {

        loop{
            thread::sleep(Duration::from_millis(1000));

            let mut timer_copy = timer.lock().unwrap();

            if(timer_copy.isCounting)
            {
                timer_copy.count -=1;

                messenger.send( countdown_cases(timer_copy) );
            }

        }

      });

}

//Deals with actual logic of pomodoro timer (whether on break, how long, etc)
// Returns the current time in seconds (to-do: may want to change how timer counts down) along with
//  the contextual message (on break or not)
fn countdown_cases(mut timer : std::sync::MutexGuard<Timer>) -> (i32, std::string::String)
{

    let context_message : std::string::String;    //Message whether user should be working or on break

    if(timer.count < 0)
    {

        //let notification = gio::Notification::new("");  //Notifies user to work/break

        if(timer.isBreak)   //was on break, now switching back to work
        {
            timer.isBreak = false;
            timer.count = 10;       //TODO : swap this out for 25 mins
            //notification.set_title("Time to work!");
        }
        else {
            if(timer.sprintCount % 3 == 0) //after 4 sprints have occured
            {
                timer.count = 10;  //TODO : swap this out for 25 mins
            }
            else {
                timer.count = 5; //TODO: Swap this out for 5 mins
            }
            //notification.set_title("Time to take a break!");
            //let notifier = gio::ApplicationExt::activate();
            //gio::ApplicationExt::send_notification(notification, "Timer event", notification);

            //TODO : Emit a system notification so user is alerted to change in tasks

            timer.sprintCount+=1;
            timer.isBreak = true;
        }
    }

    if(timer.isBreak)
    {
        context_message = String::from("Break time");
    }
    else {
        context_message = String::from("Time to work.");
    }


    (timer.count, context_message)
}

