use std::{collections::HashMap, marker::PhantomData};
use std::fmt::Debug;

trait SessionState {}


#[allow(dead_code)]
#[derive(Debug)]
struct Session<State: SessionState = Initial> {
    session_id: String,
    props: HashMap<String, String>,
    phatom: PhantomData<State>
}


impl Session<Initial> {
    fn new() -> Session<Anonymous> {
        Session::<Anonymous>{
            session_id: String::from("session_id"),
            props: HashMap::new(),
            phatom: PhantomData
        }
    }
}

impl Session<Anonymous> {
    fn authenticate(self, _username: &str, _password: &str) -> Result<Session<Authenticated>, Session<Anonymous>> {
        Ok(Session::<Authenticated>{
            session_id: String::from("session_id"),
            props: HashMap::new(),
            phatom: PhantomData
        })
    }
}

impl Session<Authenticated> {
    fn update_property(&mut self, key: &str, value: &str) {
        self.props.insert(key.into(), value.into());
    }
    
    fn logout(self) -> Session<LoggedOut> {
        Session::<LoggedOut>{
            session_id: String::from("logged_out"),
            props: self.props,
            phatom: PhantomData
        }
    }
}

#[derive(Debug)]
struct Initial;

#[derive(Debug)]
struct Anonymous;

#[derive(Debug)]
struct Authenticated;

#[derive(Debug)]
struct LoggedOut;

impl SessionState for Initial{}
impl SessionState for Anonymous{}
impl SessionState for Authenticated{}
impl SessionState for LoggedOut{}



fn main() -> std::io::Result<()> {
    let session = Session::new();
    println!("{session:?}");
    
    let mut session = session.authenticate(
            "brian", 
            "password")
        .expect("Failed to Authenticate");
    println!("{session:?}");
    
    session.update_property("Name", "Brian");
    println!("{session:?}");
    
    let session = session.logout();
    println!("{session:?}");
    
    Ok(())
}