#[allow(unused)]
use better::space_mission_better;
#[allow(unused)]
use borrow::the_borrow_checker;
#[allow(unused)]
use naive::space_mission_naive;

mod better;
mod borrow;
mod generic;
mod naive;
mod orthogonal;

fn main() {
    //the_borrow_checker();
    //space_mission_naive();
    space_mission_better();
}
