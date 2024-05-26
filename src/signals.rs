use std::{cell::RefCell, fmt::Display, rc::Rc};

pub type SignalObserver<'c> = Rc<dyn Fn() + 'c>;

#[derive(Clone)]
pub struct SignalContext<'c> {
    observer: Rc<RefCell<Option<SignalObserver<'c>>>>,
}

#[derive(Clone)]
pub struct Signal<'c, T: Clone> {
    value: Rc<RefCell<T>>,
    observers: Rc<RefCell<Vec<SignalObserver<'c>>>>,
    context: SignalContext<'c>,
}

impl<'c, T: Clone + Display> Signal<'c, T> {
    pub fn set(&self, value: T) {
        self.value.replace(value);
        // println!("Debug: @set -> will be borrowing observers");
        let observers = self.observers.borrow().clone();

        // println!(
        //     "Debug: @set => '{}' has {} observer(s)",
        //     self.value.borrow(),
        //     self.observers.borrow().len()
        // );
        // println!("Debug: {}", observers.len());
        observers.iter().for_each(|obs| {
            obs();
        });
        // println!("Debug: @set -> borrow of observers should be released after this");
    }

    pub fn get(&self) -> T {
        let current_observer = self.context.observer.borrow();

        // println!(
        //     "Debug: @get => '{}' has {} observer(s)",
        //     self.value.borrow(),
        //     self.observers.borrow().len()
        // );
        // println!(
        //     "Debug: @get -> context has observer = {}",
        //     current_observer.is_some()
        // );
        if let Some(obs) = current_observer.clone() {
            // println!("Debug: @get -> will be borrowing observers");
            self.observers.borrow_mut().push(obs);
            // println!("Debug: @get -> borrow of observers should be released after this");
        }
        // println!(
        //     "Debug: @get => '{}' now has {} observer(s)",
        //     self.value.borrow(),
        //     self.observers.borrow().len()
        // );

        self.value.borrow().clone()
    }
}

impl<'s, 'c: 's> SignalContext<'c> {
    fn set_current_observer(&self, observer: SignalObserver<'c>) {
        self.observer.borrow_mut().replace(observer);
    }

    fn remove_current_observer(&self) {
        self.observer.borrow_mut().take();
    }

    pub fn new() -> Self {
        SignalContext {
            observer: Rc::new(RefCell::new(None)),
        }
    }

    pub fn create_signal<T: Clone + Display>(&'s self, value: T) -> Signal<'c, T> {
        Signal {
            value: Rc::new(RefCell::new(value)),
            observers: Rc::new(RefCell::new(Vec::new())),
            context: self.clone(),
        }
    }

    pub fn create_effect(&self, effect: impl Fn() + 'c) {
        let exe: Rc<RefCell<Option<SignalObserver>>> = Rc::new(RefCell::new(None));
        let exe_clone = exe.clone();
        let self_clone = self.clone();

        exe.borrow_mut().replace(Rc::new(move || {
            // println!(
            //     "Debug: calling effect with exe present: {}",
            //     exe_clone.borrow().is_some()
            // );
            if let Some(exe) = exe_clone.borrow().clone() {
                self_clone.set_current_observer(exe);
            }

            effect();
            self_clone.remove_current_observer();
        }));

        exe.borrow().clone().unwrap()();
        exe.borrow_mut().take();
    }
}
