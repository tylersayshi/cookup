use inquire::Text;

pub fn recipes() {
    let answer = Text::new("Name of the recipe: ").prompt().unwrap();
    println!("your recipe is called {}", &answer);
}
