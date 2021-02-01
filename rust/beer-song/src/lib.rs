pub fn verse(n: u32) -> String {
    let phrase = match n {
        1 => one_bottle(),
        0 => no_bottle(),
        _ => some_bottle(n),
    };

    return phrase.to_string();
}

pub fn some_bottle(n: u32) -> String {
    return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", n, n, n-1, plural(n-1));
}

pub fn one_bottle() -> String {
    return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
}

pub fn no_bottle() -> String {
    return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
}

pub fn plural(n: u32) -> String {
    return match n {
        1 => "bottle".to_string(),
        _ => "bottles".to_string(),
    };
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::from("");

    for n in (end..start + 1).rev() {
        song.push_str(verse(n).as_str());

        if n != end {
            song.push_str("\n");
        }
    }

    return song;
}
