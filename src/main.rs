use cio::{printf, input};
use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet, VecDeque, LinkedList, BinaryHeap};
use std::cmp::Reverse;

fn main() {
    // Program title
    printf!("=== Demonstration of printf! with std::collections ===");

    // 1. Input and simple variable display
    printf!("1. Input and simple variable display:");
    let first_name: String = input!("Your first name: ");
    let last_name: String = input!("Your last name: ");
    let age: i32 = input!("Your age: ");
    let height: f64 = input!("Your height (in meters): ");
    let married: bool = input!("Are you married? (true/false): ");
    let favorite_letter: char = input!("What is your favorite letter? ");
    let status = if married { "you are" } else { "you are not" };
    printf!("Hello, {first_name} {last_name}, you are {age} years old, you are {height} m tall, your favorite letter is '{favorite_letter}', and {status} married.");
    printf!("------------------------------------------------");

    // 2. Using expressions in placeholders
    printf!("2. Expressions in placeholders:");
    printf!("Age in months: {age * 12}");
    printf!("Height in cm: {height * 100.0:.0}");
    printf!("Last name in uppercase: {last_name.to_uppercase()}");
    printf!("First letter of the last name: {last_name.chars().next().unwrap()}");
    printf!("Is your favorite letter uppercase? {favorite_letter.is_uppercase()}");
    printf!("Your favorite letter ASCII value: {favorite_letter as u8}");
    printf!("------------------------------------------------");

    // 3. Number formatting
    printf!("3. Number formatting:");
    let pi = std::f64::consts::PI;
    printf!("PI without format: {pi}");
    printf!("PI with 2 decimals: {pi:.2}");
    printf!("PI with 6 decimals: {pi:.6}");
    printf!("Integer with padding: {age:04}");
    printf!("Integer as hexadecimal: {age:x}");
    printf!("Integer as binary: {age:b}");
    printf!("Float with scientific notation: {pi:e}");
    printf!("------------------------------------------------");

    // 4. Array-type containers (sequence containers)
    printf!("4. Array-type containers (sequence containers):");

    // Creating different array-type containers
    let vector = vec![1, 2, 3, 4, 5];

    let mut vecdeque = VecDeque::new();
    vecdeque.push_back(1);
    vecdeque.push_back(2);
    vecdeque.push_front(0);
    vecdeque.push_back(3);

    let mut linked_list = LinkedList::new();
    linked_list.push_back("first");
    linked_list.push_back("second");
    linked_list.push_back("third");

    let mut binary_heap = BinaryHeap::new();
    binary_heap.push(5);
    binary_heap.push(1);
    binary_heap.push(3);
    binary_heap.push(2);

    // Displaying simple 1D containers with :a format
    printf!("\nSimple 1D containers (:a format):");
    printf!("Vector:     {vector:a}");
    printf!("VecDeque:   {vecdeque:a}");
    printf!("LinkedList: {linked_list:a}");
    printf!("BinaryHeap: {binary_heap:a}");

    // Multidimensional arrays (with :a and :c formats)
    printf!("\nMultidimensional arrays:");
    let matrix_2d = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let matrix_3d = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];
    let matrix_4d = vec![
        vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]],
        vec![vec![vec![9, 10], vec![11, 12]], vec![vec![13, 14], vec![15, 16]]]
    ];

    printf!("2D Matrix (:a):\n{matrix_2d:a}");
    printf!("2D Matrix (:c): {matrix_2d:c}");
    printf!("3D Matrix (:a):\n{matrix_3d:a}");
    printf!("3D Matrix (:c): {matrix_3d:c}");
    printf!("4D Matrix (:a):\n{matrix_4d:a}");
    printf!("4D Matrix (:c): {matrix_4d:c}");

    // Array operations
    printf!("\nArray operations:");
    printf!("Vector first element: {vector[0]}");
    printf!("Vector last element: {vector[vector.len() - 1]}");
    printf!("Sum of vector elements: {vector.iter().sum::<i32>()}");
    printf!("Access to matrix element [1][0][1][0]: {matrix_4d[1][0][1][0]}");
    printf!("------------------------------------------------");

    // 5. Map-type containers (associative containers)
    printf!("5. Map-type containers (associative containers):");

    // Creating different map-type containers
    let mut hash_map = HashMap::new();
    hash_map.insert("France", "Paris");
    hash_map.insert("Germany", "Berlin");
    hash_map.insert("Italy", "Rome");

    let mut btree_map = BTreeMap::new();
    btree_map.insert("A", 1);
    btree_map.insert("C", 3);
    btree_map.insert("B", 2);

    let mut hash_set = HashSet::new();
    hash_set.insert("apple");
    hash_set.insert("banana");
    hash_set.insert("orange");

    let mut btree_set = BTreeSet::new();
    btree_set.insert(5);
    btree_set.insert(2);
    btree_set.insert(8);
    btree_set.insert(1);

    // Displaying simple map containers with :j format
    printf!("\nSimple map containers (:j format):");
    printf!("HashMap:\n{hash_map:j}");
    printf!("BTreeMap:\n{btree_map:j}");
    printf!("HashSet:\n{hash_set:j}");
    printf!("BTreeSet:\n{btree_set:j}");

    // Compact display with :c format
    printf!("\nCompact display (:c format):");
    printf!("HashMap: {hash_map:c}");
    printf!("BTreeMap: {btree_map:c}");
    printf!("HashSet: {hash_set:c}");
    printf!("BTreeSet: {btree_set:c}");

    // Map operations
    printf!("\nMap operations:");
    printf!("Capital of France: {hash_map.get(\"France\").unwrap()}");
    printf!("Value for key 'B' in BTreeMap: {btree_map.get(\"B\").unwrap()}");
    printf!("Is 'apple' in the HashSet? {hash_set.contains(\"apple\")}");
    printf!("Minimum value in BTreeSet: {btree_set.iter().next().unwrap()}");
    printf!("------------------------------------------------");

    // 6. Complex nested structures
    printf!("6. Complex nested structures:");

    // HashMap of vectors
    let mut map_of_nums = HashMap::new();
    map_of_nums.insert("Numbers", vec![1, 2, 3, 4, 5]);

    let mut map_of_strings = HashMap::new();
    map_of_strings.insert("Letters", vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string(), "e".to_string()]);

    // HashMap of HashMaps
    let mut map_of_maps = HashMap::new();

    let mut france_info = HashMap::new();
    france_info.insert("capital", "Paris");
    france_info.insert("population", "67 million");
    france_info.insert("language", "French");

    let mut germany_info = HashMap::new();
    germany_info.insert("capital", "Berlin");
    germany_info.insert("population", "83 million");
    germany_info.insert("language", "German");

    map_of_maps.insert("France", france_info);
    map_of_maps.insert("Germany", germany_info);

    // 3-level nested structure
    let mut cities_data = HashMap::new();

    let mut france = HashMap::new();
    let mut paris = HashMap::new();
    paris.insert("population".to_string(), "2.2M".to_string());
    paris.insert("attractions".to_string(), "Eiffel Tower, Louvre".to_string());
    france.insert("Paris".to_string(), paris);

    let mut usa = HashMap::new();
    let mut new_york = HashMap::new();
    new_york.insert("population".to_string(), "8.4M".to_string());
    new_york.insert("attractions".to_string(), "Statue of Liberty, Times Square".to_string());
    usa.insert("New York".to_string(), new_york);

    cities_data.insert("France".to_string(), france);
    cities_data.insert("USA".to_string(), usa);

    // Displaying complex structures
    printf!("\nHashMap of numbers (:j):\n{map_of_nums:j}");
    printf!("HashMap of strings (:j):\n{map_of_strings:j}");
    printf!("HashMap of HashMaps (:j):\n{map_of_maps:j}");
    printf!("3-level nested structure (:j):\n{cities_data:j}");

    // Compact format for complex structures
    printf!("\nCompact format for complex structures (:c):");
    printf!("HashMap of numbers: {map_of_nums:c}");
    printf!("HashMap of strings: {map_of_strings:c}");
    printf!("HashMap of HashMaps: {map_of_maps:c}");
    printf!("3-level nested structure: {cities_data:c}");

    // Access to deep nested elements
    printf!("\nAccess to deeply nested elements:");
    printf!("Population of Paris: {cities_data.get(\"France\").unwrap().get(\"Paris\").unwrap().get(\"population\").unwrap()}");
    printf!("Attractions in New York: {cities_data.get(\"USA\").unwrap().get(\"New York\").unwrap().get(\"attractions\").unwrap()}");
    printf!("------------------------------------------------");

    // 7. Advanced Turbofish and Complex Type Manipulation
    printf!("7. Advanced Turbofish and Complex Type Manipulation:");

    // Advanced turbofish with chained method calls
    printf!("\nTurbofish with chained method calls:");

    // Complex transformation with turbofish
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_squares = numbers.iter()
        .filter(|&n| n % 2 == 0)            // Keep only even numbers
        .map(|&n| n * n)                    // Square each number
        .collect::<Vec<_>>();               // Collect into a Vec<i32>

    printf!("Original numbers: {numbers:a}");
    printf!("Even numbers squared: {even_squares:a}");

    // Complex turbofish with multiple collections
    let nested_vec = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    let flattened_filtered = nested_vec.iter()
        .flat_map(|v| v.iter())             // Flatten nested vectors
        .filter(|&&x| x % 2 == 1)           // Keep only odd numbers
        .map(|&x| x * x)                    // Square each odd number
        .collect::<Vec<_>>();               // Collect into a Vec<i32>

    printf!("Nested vectors: \n{nested_vec:a}");
    printf!("Flattened and filtered (odd numbers squared): {flattened_filtered:a}");

    // Turbofish with custom data types
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    impl Person {
        fn new(name: &str, age: i32) -> Self {
            Person {
                name: name.to_string(),
                age,
            }
        }

        fn greet(&self) -> String {
            format!("Hello, my name is {} and I am {} years old", self.name, self.age)
        }

        fn birthday(&mut self) {
            self.age += 1;
        }
    }

    // Collection of custom structs
    let mut people = vec![
        Person::new("Alice", 30),
        Person::new("Bob", 25),
        Person::new("Charlie", 35),
        Person::new("Diana", 28),
    ];

    // Turbofish with custom struct fields
    let average_age = people.iter()
        .map(|p| p.age)                     // Extract ages
        .sum::<i32>() as f64                // Sum as i32, convert to f64
        / people.len() as f64;              // Divide by number of people

    printf!("\nOperations with custom structs:");
    printf!("Average age: {average_age:.1}");

    // Sort people by age using turbofish
    people.sort_by_key(|p| p.age);

    // Use turbofish to transform struct data
    let names = people.iter()
        .map(|p| p.name.clone())            // Extract names
        .collect::<Vec<String>>()           // Collect into a Vec<String>
        .join(", ");                        // Join with commas

    printf!("Names sorted by age: {names}");

    // Manipulate data using reference to self
    let mut alice = Person::new("Alice", 30);
    alice.birthday();  // Increment age

    printf!("After birthday: {alice.greet()}");

    // Advanced grouping with turbofish
    let grouped_by_age: HashMap<i32, Vec<String>> = people.iter()
        .map(|p| (p.age, p.name.clone()))  // Create (age, name) pairs
        .fold(HashMap::new(), |mut acc, (age, name)| {
            acc.entry(age)
                .or_insert_with(Vec::new)
                .push(name);
            acc
        });

    printf!("\nPeople grouped by age (:j):\n{grouped_by_age:j}");

    // Advanced HashMap creation with collect and turbofish
    let name_to_age: HashMap<String, i32> = people.iter()
        .map(|p| (p.name.clone(), p.age))
        .collect();

    printf!("\nName to age mapping (:j):\n{name_to_age:j}");

    // Enums in collections
    #[derive(Debug)]
    enum CityInfo {
        #[allow(dead_code)]
        Population(u32),
        #[allow(dead_code)]
        Attractions(Vec<String>),
        #[allow(dead_code)]
        Coordinates { lat: f64, lon: f64 },
    }

    let mut enum_map = HashMap::new();
    enum_map.insert("Paris", CityInfo::Population(2200000));
    enum_map.insert("Tokyo", CityInfo::Attractions(vec!["Tokyo Tower".to_string(), "Shibuya".to_string()]));
    enum_map.insert("New York", CityInfo::Coordinates { lat: 40.7128, lon: -74.0060 });

    printf!("\nHashMap with enum values (:j):\n{enum_map:j}");

    // Reverse wrapper for max/min heap with turbofish
    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(2));
    min_heap.push(Reverse(8));
    min_heap.push(Reverse(1));

    let sorted_values = min_heap.iter()
        .map(|r| r.0)                      // Extract the inner value from Reverse
        .collect::<Vec<_>>();              // Collect into a Vec<i32>

    printf!("\nBinaryHeap with Reverse wrapper:");
    printf!("Min-Heap: {min_heap:a}");
    printf!("Extracted values (still sorted): {sorted_values:a}");
    printf!("------------------------------------------------");

    // 8. Format recommendation summary
    printf!("8. Format recommendation summary:");
    printf!("- Format :a: Best for 1D arrays and matrices (adds indentation to matrices)");
    printf!("- Format :j: Best for maps and complex structures (pretty-printed)");
    printf!("- Format :c: Best for compact display (single-line for simple structures)");

    // Conclusion
    printf!("\n=== End of the demonstration ===");
}