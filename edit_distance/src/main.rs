use edit_distance::*;

fn main() {
    let source = "alignment";
    let target = "assignment";

    println!(
        "1    It's necessary to make {} change(s) to {:?} to get {:?}",
        edit_distance(source, target),
        source,
        target
    );
    println!(
        "2    It's necessary to make {} change(s) to {:?} to get {:?}",
        edit_distance("gumbo", "gambol"),
        "gumbo",
        "gambol"
    );
    println!(
        "3    It's necessary to make {} change(s) to {:?} to get {:?}",
        edit_distance("kitten", "sitting"),
        "kitten",
        "sitting"
    );
    println!(
        "4    It's necessary to make {} change(s) to {:?} to get {:?}",
        edit_distance("rosettacode", "raisethysword"),
        "rosettacode",
        "raisethysword"
    );
}