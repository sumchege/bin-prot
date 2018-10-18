// #[dervive(Debug)]

// enum ComponentError {
//     NotStarted,
//     InvalidConfiguration
// }

// pub trait Component {
//     fn get_name(&self) -> &str;
//     fn start(&mut self) -> !;
//     fn stop(&mut self) -> Result<(), ComponentError>;
// }

// trait ThreadComponent where Self :  Component {
//     fn spawn(mut self) -> Result(JoinHandle<Self>, ComponentError);

// }

