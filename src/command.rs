use crate::app::App;

pub trait Command {
    fn execute(&mut self, app: &mut App);
}