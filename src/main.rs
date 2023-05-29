use notify_rust::Notification;
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let message = [
        "Take a break from the monitor.",
        "Stare off into the distance.",
        "Grab a snack.",
        "Do whatever.",
        "Just make sure it does not have a screen.",
    ]
    .join(" ");

    loop {
        thread::sleep(Duration::from_secs(15 * 60));
        Notification::new()
            .summary("Reminder to blink")
            .body(&message)
            .timeout(0) // do not expire, the user should be forced to dismiss
            .show()?;
    }
}
