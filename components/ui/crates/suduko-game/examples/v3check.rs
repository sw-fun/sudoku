fn main() {
    let code = suduko_game::save(0, None, &std::collections::BTreeMap::new());
    println!("code: {code}");
    println!("fields: {}", code.split('|').count());
    println!(
        "restore: {:?}",
        suduko_game::restore(&code).map(|s| (s.level, s.stats, s.game.is_some()))
    );
}
