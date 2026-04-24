use std::marker::PhantomData;


trait BulbState {
    
}

#[derive(Debug)]
struct LightBulb<State: BulbState> {
    phantom: PhantomData<State>
}

#[derive(Debug)]
struct On;

#[derive(Debug)]
struct Off;

impl BulbState for On {}
impl BulbState for Off {}

impl LightBulb<On> {
    // this method consumes the value and returns a new value
    // we can not change the type parameter on generic structures
    fn turn_off(self) -> LightBulb<Off> {
        LightBulb::<Off>{ phantom: PhantomData }
    }
}

impl LightBulb<Off> {
    fn turn_on(self) -> LightBulb<On> {
        LightBulb::<On>{ phantom: PhantomData }
    }
}

fn main() {
    let off_bulb = LightBulb::<Off>{ phantom: PhantomData };
    let on_bulb = off_bulb.turn_on();
    let another_on_bulb = on_bulb.turn_off().turn_on().turn_off().turn_on();
    
    println!("{another_on_bulb:?}");
}