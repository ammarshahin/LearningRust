pub fn verse(n: u32) -> String {
    let ret;
    if n == 0 {
        ret = "No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_owned();
    } else if n == 1 {
        ret = "1 bottle of beer on the wall, 1 bottle of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n"
            .to_owned();
    } else if n == 2 {
        ret = "2 bottles of beer on the wall, 2 bottles of beer.\n\
            Take one down and pass it around, 1 bottle of beer on the wall.\n"
            .to_owned();
    } else {
        ret = format!(
            "{} bottles of beer on the wall, {} bottles of beer.\n\
            Take one down and pass it around, {} bottles of beer on the wall.\n",
            n,
            n,
            n - 1
        );
    }
    ret
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = "".to_owned();
    for i in (end..=start).rev() {
        let current_verse = &verse(i).to_string();
        let n = current_verse.len();
        song.push_str(&current_verse.to_string()[0..n]);
        song.push('\n')
    }
    song.pop();
    return song;
}
