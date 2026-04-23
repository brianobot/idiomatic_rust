use std::{rc::Weak, sync::Arc};

trait Observer {
    type Subject;
    
    fn observe(&self, subject: &Self::Subject);
}


trait Observable {
    type Observer;
    
    fn update(&self);
    fn attach(&mut self, observer: Self::Observer);
    fn detach(&mut self, observer: Self::Observer);
    
}

struct Subject {
    observers: Vec<Weak<dyn Observer<Subject=Self>>>,
    state: String,
}

impl Subject {
    fn new(state: &str) -> Self {
        Self {
            observers: vec![],
            state: state.into()
        }
    }
    
    fn state(&self) -> &str {
        self.state.as_ref()
    }
}

impl Observable for Subject {
    type Observer = Arc<dyn Observer<Subject = Self>>;

    fn update(&self) {
        self.observers
            .iter()
            .flat_map(|o| o.upgrade())
            .for_each(|o| o.observe(self));
    }

    fn attach(&mut self, observer: Self::Observer) {
        self.observers.push(Arc::downgrade(&observer));
    }

    fn detach(&mut self, observer: Self::Observer) {
        self.observers
            .retain(
                |f| {
                    !f.ptr_eq(&Arc::downgrade(&observer))
                }
            )
    }
}


struct MyObserver {
    name: String
}


impl MyObserver {
    fn new(name: &str) -> Arc<Self> {
        Arc::new(Self {
            name: name.into()
        })
    }
}

impl Observer for MyObserver {
    type Subject = Subject;

    fn observe(&self, subject: &Self::Subject) {
        println!(
            "observed subject with state={:?} in {}",
            subject.state(),
            self.name
        );
    }
}

fn main() {
    let mut subject = Subject::new("World Clock");
    
    let observer1 = MyObserver::new("1");
    let observer2 = MyObserver::new("2");
    
    subject.attach(observer1.clone());
    subject.attach(observer2.clone());
    
    subject.update();
}