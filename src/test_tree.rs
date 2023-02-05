#[cfg(test)]
mod tests {
    use crate::Tree;
    use rand::Rng;

    /* The constructed AVL Tree would be
         *                 5(H:2)(F: 0)
         *         /                       \
         *    1(H:1)(F:1)              42(H:1)(F:0)
         *               \            /            \
         *          2(H:0)(F:0)   21(H:0)(0)    83(H:0)(F:0)
         */

    #[test]
    fn test_basic() {
        let mut tree = Tree::new();
        tree.insert(42, "Peter".to_string());
        tree.insert(21, "Joanna".to_string());
        tree.insert(83, "Margaretha".to_string());
        tree.insert(5, "Frank".to_string());
        tree.insert(1, "Peter".to_string());
        tree.insert(2, "Mark".to_string());

        tree.print();
    }

    #[test]
    fn test_find() {
        let mut tree = Tree::new();
        tree.insert(42, "Peter".to_string());
        tree.insert(21, "Joanna".to_string());
        tree.insert(83, "Margaretha".to_string());
        tree.insert(5, "Frank".to_string());
        tree.insert(1, "Peter".to_string());
        tree.insert(2, "Mark".to_string());

        if !tree.find(42, "Peter".to_string()) {
            panic!("Could not find an item that was recently inserted");
        }
    }

    #[test]
    fn test_erase() {
        let mut tree = Tree::new();
        tree.insert(42, "Peter".to_string());
        tree.insert(21, "Joanna".to_string());
        tree.insert(83, "Margaretha".to_string());
        tree.insert(5, "Frank".to_string());
        tree.insert(1, "Peter".to_string());
        tree.insert(2, "Mark".to_string());

        tree.erase(2, "Mark".to_string());
        if tree.find(2, "Mark".to_string()) {
            panic!("Found an item that was supposed to be removed (leaf node)");
        }
    }

    #[test]
    fn test_insert() {
        let mut tree = Tree::new();
        tree.insert(42, "Peter".to_string());
        tree.insert(21, "Joanna".to_string());
        tree.insert(83, "Margaretha".to_string());
        tree.insert(5, "Frank".to_string());
        tree.insert(1, "Peter".to_string());
        tree.insert(2, "Mark".to_string());

        tree.erase(2, "Mark".to_string());
        if tree.find(2, "Mark".to_string()) {
            panic!("Found an item that was supposed to be removed (leaf node)");
        }
    }

    #[test]
    fn test_delete() {
        let mut tree = Tree::new();
        tree.insert(42, "Peter".to_string());
        tree.insert(21, "Joanna".to_string());
        tree.insert(83, "Margaretha".to_string());
        tree.insert(5, "Frank".to_string());
        tree.insert(1, "Peter".to_string());
        tree.insert(2, "Mark".to_string());

        tree.delete();

        if tree.find(2, "Mark".to_string()) {
            panic!("Found an item that was supposed to be removed (leaf node)");
        }
    }

    #[test]
    fn test_stress() {
        let mut tree = Tree::new();
        let mut rng = rand::thread_rng();

        for _ in 0..100000 {
            let age: i32 = rng.gen::<i32>() % 1000;
            let name = "Name".to_string();

            if tree.find(age, name.clone()) {
                tree.erase(age,name);
            }else{
                tree.insert(age,name);
            }
        }

        tree.delete();

        println!("Stress test succeeded\n")

    }
}