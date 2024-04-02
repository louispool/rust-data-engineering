use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize, my_fruits: Option<&mut Vec<String>>) -> Vec<String> {
    
    let fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ];

    let mut random_fruits = if num_fruits > 0 {
        
        let mut rng = thread_rng();
        let mut fruits = fruits;
        fruits.shuffle(&mut rng);

        fruits.into_iter().take(num_fruits).collect()
    } 
    else {
        Vec::new()
    };

    if let Some(my_fruits) = my_fruits {
        random_fruits.append(my_fruits);
    }

    random_fruits
}