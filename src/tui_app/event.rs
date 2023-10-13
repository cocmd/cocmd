use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

use super::app::AppResult;

/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
    /// Termination event to stop the main loop.
    Terminate,
}

/// Terminal event handler.
#[allow(dead_code)]

pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    handler: thread::JoinHandle<()>,
    receiver: Arc<Mutex<mpsc::Receiver<Event>>>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let handler = {
            let sender = sender.clone();
            let receiver = Arc::clone(&receiver);
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }

                    // Check if the termination event was received.
                    if let Ok(Event::Terminate) = receiver.lock().unwrap().try_recv() {
                        break; // Break the loop to stop event handling.
                    }
                }
            })
        };
        Self {
            sender,
            handler,
            receiver,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.lock().unwrap().recv()?)
    }

    /// Stop the main loop in the event handler.
    pub fn stop(&self) {
        // Send the termination event to stop the main loop.
        self.sender
            .send(Event::Terminate)
            .expect("failed to send termination event");
    }
}
