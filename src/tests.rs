use {Script, Command, rect};

fn draw(x: u32, y: u32, w: u32, h: u32, fill: char) -> Command {
    rect(x, y, w, h, fill)
}

#[test]
fn empty_space() {
    assert_eq!(Script::new(5, 3).run().as_slice(),
               ".....\n\
                .....\n\
                .....\n");
}

#[test]
fn three_by_three_b() {
    let cmds = vec![
        draw(1, 1, 3, 3, 'b')
            ];
    assert_eq!(Script::new_commands(5, 5, cmds[]).run().as_slice(),
               ".....\n\
                .+-+.\n\
                .|b|.\n\
                .+-+.\n\
                .....\n");
}

#[test]
fn three_by_four_b() {
    let cmds = vec![
        draw(1, 1, 3, 4, 'b')
            ];
    assert_eq!(Script::new_commands(5, 5, cmds[]).run().as_slice(),
               ".....\n\
                .+-+.\n\
                .|b|.\n\
                .|b|.\n\
                .+-+.\n");
}

#[test]
fn three_by_four_c() {
    let cmds = vec![
        draw(1, 1, 3, 4, 'c')
            ];
    assert_eq!(Script::new_commands(5, 5, cmds[]).run().as_slice(),
               ".....\n\
                .+-+.\n\
                .|c|.\n\
                .|c|.\n\
                .+-+.\n");
}

#[test]
fn side_by_side() {
    let cmds = vec![
        draw(1, 0, 3, 4, 'b'),
        draw(6, 2, 3, 3, 'c')
            ];
    assert_eq!(Script::new_commands(10, 5, cmds[]).run().as_slice(),
               ".+-+......\n\
                .|b|......\n\
                .|b|..+-+.\n\
                .+-+..|c|.\n\
                ......+-+.\n");
}

#[test]
fn overlapping() {
    let cmds = vec![
        draw(0, 0, 3, 4, 'b'),
        draw(1, 2, 3, 3, 'c')
            ];
    assert_eq!(Script::new_commands(5, 5, cmds[]).run().as_slice(),
               "+-+..\n\
                |b|..\n\
                |+-+.\n\
                +|c|.\n\
                .+-+.\n");
}

#[test]
fn stout() {
    let cmds = vec![
        draw(1, 1, 5, 2, 'b'),
            ];
    assert_eq!(Script::new_commands(7, 7, cmds[]).run().as_slice(),
               ".......\n\
                .+---+.\n\
                .+---+.\n\
                .......\n\
                .......\n\
                .......\n\
                .......\n");
}

#[test]
fn skinny() {
    let cmds = vec![
        draw(1, 1, 2, 5, 'b'),
            ];
    assert_eq!(Script::new_commands(7, 7, cmds[]).run().as_slice(),
               ".......\n\
                .++....\n\
                .||....\n\
                .||....\n\
                .||....\n\
                .++....\n\
                .......\n");
}

#[test]
fn four_by_one_b() {
    let cmds = vec![
        draw(1, 1, 4, 1, 'b'),
            ];
    assert_eq!(Script::new_commands(7, 7, cmds[]).run().as_slice(),
               ".......\n\
                .bbbb..\n\
                .......\n\
                .......\n\
                .......\n\
                .......\n\
                .......\n");
}

#[test]
fn one_by_four_c() {
    let cmds = vec![
        draw(2, 1, 1, 4, 'c'),
            ];
    assert_eq!(Script::new_commands(7, 7, cmds[]).run().as_slice(),
               ".......\n\
                ..c....\n\
                ..c....\n\
                ..c....\n\
                ..c....\n\
                .......\n\
                .......\n");
}

#[test]
fn max_box() {
    let cmds = vec![
        draw(0, 0, 5, 3, 'b'),
        ];
    assert_eq!(Script::new_commands(5, 3, cmds[]).run().as_slice(),
               "+---+\n\
                |bbb|\n\
                +---+\n");
}
