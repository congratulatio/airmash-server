
#![allow(dead_code, unused_imports)]
#![feature(optin_builtin_traits)]

// Crates with macros
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate dimensioned;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate shred_derive;

// Regular Dependencies
extern crate simple_logger;
extern crate rand;
extern crate specs;
extern crate shrev;
extern crate shred;
extern crate fnv;
extern crate uuid;
extern crate tokio;
extern crate tokio_core;
extern crate websocket;
extern crate airmash_protocol;

use websocket::futures as futures;

// Modules
mod types;
mod server;
mod timers;
mod systems;
mod handlers;
mod timeloop;

use std::env;
use std::thread;
use std::sync::Mutex;
use std::time::{Instant, Duration};
use std::sync::mpsc::{channel, Receiver};

use specs::{World, DispatcherBuilder, Dispatcher};
use tokio::reactor::Reactor;
use tokio::runtime::current_thread::Runtime;

use types::{ThisFrame, LastFrame};
use timeloop::timeloop;

fn build_dispatcher<'a, 'b>(
    event_recv: Receiver<types::ConnectionEvent>,
    timer_recv: Receiver<types::TimerEvent>
) -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        // Add systems here
        .with(systems::PacketHandler::new(event_recv), "packet", &[])
        .with(systems::TimerHandler::new(timer_recv), "timer", &[])
        
        // Add handlers here
        .with(handlers::OnOpenHandler::new(),  "onopen",  &["packet"])
        .with(handlers::OnCloseHandler::new(), "onclose", &["onopen"])
        .with(handlers::LoginHandler::new(),   "onlogin", &["onclose"])
        .with(handlers::KeyHandler::new(),     "onkey",   &["onclose"])
        .with(handlers::ScoreBoardTimerHandler::new(), "scoreboard", &["timer"])

        // Systems with dependencies on handlers
        .with(systems::PositionUpdate::new(),  "pos_update", &["onkey"])

        // This needs to run after systems which send messages
        .with_thread_local(systems::PollComplete::new())

        // Build
        .build()
}

fn setup_panic_handler() {
    use std::panic;
    use std::process;

	let orig_handler = panic::take_hook();
	panic::set_hook(Box::new(move |panic_info| {
		error!(
			target: "server",
			"A fatal error occurred within a server thread. Aborting!",
		);
		error!(
			target: "server",
			"Error Info: {}",
			panic_info
		);

		orig_handler(panic_info);
		process::exit(1);
	}));
}

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    env::set_var("RUST_BACKTRACE", "1");

    setup_panic_handler();

    let addr = "127.0.0.1:3501";

    let mut world = World::new();

    let (event_send, event_recv) = channel::<types::ConnectionEvent>();
    let (timer_send, timer_recv) = channel::<types::TimerEvent>();

    // Add resources
    info!(target: "server", "Setting up resources");

    world.add_resource(types::Connections::new());

    // Add systems
    info!(target: "server", "Setting up systems");

    let mut dispatcher = build_dispatcher(event_recv, timer_recv);

    // Start websocket server
    info!(target: "server", "Starting websocket server!");
    let server_thread = thread::spawn(move || {
        server::run_acceptor(addr, event_send);
    });

    // Start gameloop
    info!(target: "server", "Starting gameloop!");
    
    // Need to run the event loop on the current
    // thread since Dispatcher doesn't implement Send
    let mut runtime = Runtime::new().unwrap();

    // Start timer loops
    let timers = thread::spawn(move || {
        tokio::run(futures::lazy(move || {
            timers::start_timer_events(timer_send);
            Ok(())
        }));
    });

    world.add_resource(types::StartTime(Instant::now()));
    dispatcher.setup(&mut world.res);

    // Run the gameloop at 60 Hz
    runtime.spawn(timeloop(move |now| {
        world.add_resource(ThisFrame(now));
        dispatcher.dispatch(&mut world.res);
        world.maintain();
        world.add_resource(LastFrame(now));

        //debug::print_all_entities(&world);
    }, Duration::from_nanos(16666667)));

    runtime.run().unwrap();

    // Shut down
    info!(target: "server", "Exited gameloop, shutting down");
    server_thread.join().unwrap();
    timers.join().unwrap();

    info!(target: "server", "Shutdown completed successfully");
}
