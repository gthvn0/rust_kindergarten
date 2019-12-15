//
// l is the width of the letter
// h is the height of the letter
// t is the text to represent
// into row we have the string of characters (ABC..Z?) in ASCII art
//
// For exemple:
// l: 4
// h: 5
// t: E
// row:  #  ##   ## ##  ### ###  ## # # ###  ## # # #   # # ###  #  ##   #  ##   ## ### # # # # # # # # # # ### ###
// row: # # # # #   # # #   #   #   # #  #    # # # #   ### # # # # # # # # # # #    #  # # # # # # # # # #   #   #
// row: ### ##  #   # # ##  ##  # # ###  #    # ##  #   ### # # # # ##  # # ##   #   #  # # # # ###  #   #   #   ##
// row: # # # # #   # # #   #   # # # #  #  # # # # #   # # # # # # #    ## # #   #  #  # # # # ### # #  #  #
// row: # # ##   ## ##  ### #    ## # # ###  #  # # ### # # # #  #  #     # # # ##   #  ###  #  # # # #  #  ###  #
//
// should return
//
// ###
// #
// ##
// #
// ###

fn get_slice_for_letter(c: char, width: usize, ascii: &str) -> &str {
    let diff = (c.to_ascii_uppercase() as usize) - ('A' as usize);
    &ascii[diff * width..(diff + 1) * width]
}

fn display_ascii(slice: &str, width: usize, ascii: &Vec<&str>) {
    let mut iterator = ascii.iter();
    loop {
        match iterator.next() {
            None => {
                break
            },
            Some(line) => {
                let mut display = Vec::new();
                for c in slice.chars() {
                    display.push(get_slice_for_letter(c, width, line));
                }
                println!("{}", display.concat());
            }
        }
    }
}

fn main() {
    let l = 4;
    let _h = 5;
    // A -> 0..l
    // B -> l..2l
    // C -> 2l..3l
    let mut ascii = Vec::new();
    ascii.push(" #  ##   ## ##  ### ###  ## # # ###  ## # # #   # # ###  #  ##   #  ##   ## ### # # # # # # # # # # ### ###");
    ascii.push("# # # # #   # # #   #   #   # #  #    # # # #   ### # # # # # # # # # # #    #  # # # # # # # # # #   #   #");
    ascii.push("### ##  #   # # ##  ##  # # ###  #    # ##  #   ### # # # # ##  # # ##   #   #  # # # # ###  #   #   #   ##");
    ascii.push("# # # # #   # # #   #   # # # #  #  # # # # #   # # # # # # #    ## # #   #  #  # # # # ### # #  #  #      ");
    ascii.push("# # ##   ## ##  ### #    ## # # ###  #  # # ### # # # #  #  #     # # # ##   #  ###  #  # # # #  #  ###  # ");

    let t = "HellO";
    display_ascii(&t, l, &ascii);

    println!("\n");

    let t = "Manhattan";
    display_ascii(&t, l, &ascii);
}
