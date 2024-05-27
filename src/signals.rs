use std::{cell::RefCell, rc::Rc};

pub type SignalObserver<'c> = Rc<dyn Fn() + 'c>;

#[derive(Clone)]
pub struct SignalContext<'c> {
    current_observer: Rc<RefCell<Option<SignalObserver<'c>>>>,
}

#[derive(Clone)]
pub struct Signal<'c, T: Clone> {
    value: Rc<RefCell<T>>,
    observers: Rc<RefCell<Vec<SignalObserver<'c>>>>,
    context: SignalContext<'c>,
}

impl<'c, T: Clone> Signal<'c, T> {
    pub fn set(&self, value: T) {
        self.value.replace(value);
        let observers = self.observers.borrow().clone();

        for observer in observers {
            observer();
        }
    }

    pub fn get(&self) -> T {
        let current_observer = self.context.current_observer.borrow().clone();

        if let Some(obs) = current_observer {
            self.observers.borrow_mut().push(obs);
        }

        self.value.borrow().clone()
    }
}

impl<'c> SignalContext<'c> {
    fn set_current_observer(&self, observer: SignalObserver<'c>) {
        self.current_observer.borrow_mut().replace(observer);
    }

    fn remove_current_observer(&self) {
        self.current_observer.borrow_mut().take();
    }

    pub fn new() -> Self {
        SignalContext {
            current_observer: Rc::new(RefCell::new(None)),
        }
    }

    pub fn create_signal<T: Clone>(&self, value: T) -> Signal<'c, T> {
        Signal {
            value: Rc::new(RefCell::new(value)),
            observers: Rc::new(RefCell::new(Vec::new())),
            context: self.clone(),
        }
    }

    pub fn create_effect(&self, effect: impl Fn() + 'c) {
        let effect_rc = Rc::new(effect);
        
        self.set_current_observer(effect_rc.clone());
        effect_rc();
        self.remove_current_observer();
    }
}
