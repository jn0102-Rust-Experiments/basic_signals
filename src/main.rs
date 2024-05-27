
use crate::signals::SignalContext;

mod signals;

fn main() {
    println!("Hello, world!");
    let ctx = SignalContext::new();
    ctx.create_effect(|| {
        println!("Effect 1");
    });
    ctx.create_effect(|| {
        println!("Effect 2 line 1");
        println!("Effect 2 line 2");
    });
    println!("Testing signals...");
    
    let s1 = ctx.create_signal(1);
    let s2 = ctx.create_signal("signal 2@1");

    let s1_clone = s1.clone();
    let s2_clone = s2.clone();
    
    ctx.create_effect(move || {
        let s1_value = s1_clone.get();
        let s2_value = s2_clone.get();
        println!("s1: {s1_value}");
        println!("s2: {s2_value}");
    });
    
    s1.set(2);
    s1.set(3);
    s1.set(4);
    s1.set(5);
    s2.set("signal 2@2");
}
