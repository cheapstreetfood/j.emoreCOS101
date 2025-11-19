fn main() {
    let mut recipe_levels = ("Cinnamon",3,"Baked Alaska",5);

    println!("Baking recipes and their difficulty (1-5) = {:?}",recipe_levels);

    recipe_levels.2 = "Chiffon Cake";
    recipe_levels.3 = 3;

    println!("Baking Recipes and difficulty(1-5) 2.0 = {:?}",recipe_levels );
}
