use rand::Rng;

fn add_fancy_hat() {
    println!("Adding fancy hat")
}

fn remove_fancy_hat() {
    println!("Removing fancy hat")
}

fn move_player(num_spaces: u8) {
    println!("Moving player {} spaces", num_spaces)
}

fn re_roll(){
    println!("Rolling again...")
}

pub(crate) fn dice_roll(){
    let dice_roll =  rand::thread_rng().gen_range(0..10);

    match dice_roll{
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        9 =>  re_roll(),
        other => move_player(other),
    }

}
