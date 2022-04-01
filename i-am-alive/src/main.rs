use actix::{Actor, Addr, Context, System};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("I am alive!");
        System::current().stop(); // <- stop system
    }
}

fn main() {
    let mut system = System::new();

    let addr = system.block_on(async { MyActor.start() });

    system.run();
}
