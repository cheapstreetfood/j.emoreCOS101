fn main() {
    let mut recipe : Vec<String> = Vec::new();
    println!("The recipe vector has element{}",recipe.len());

    let mut input1 = String::new();
    println!("How many recipies do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let recipe_num:u16 = input1.trim().parse().expect("Invalid input");

    for count in 0..recipe_num {
        let mut input2 = String::new();
        println!("Enter Recipe {}",count+1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_recipe:String = input2.trim().parse().expect("Invalid input");
        recipe.push(new_recipe);
    }
    println!("Your preferred recipes are\n");
    let mut count =1;

    for i in recipe 
    {
        println!("{} {}",count,i);
        count+=1;
    }

}
