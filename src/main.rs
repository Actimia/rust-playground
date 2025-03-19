use better::space_mission_smart;
use borrow::the_borrow_checker;

mod naive;
mod better;
mod orthogonal;
mod borrow;
mod generic;

fn main() {
    the_borrow_checker();
    //space_mission_naive();
    space_mission_smart();
    //space_mission_orthogonal();
}
