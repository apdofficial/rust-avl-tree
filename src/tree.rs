use std::borrow::Borrow;
use std::cmp;
use std::cmp::{Ord, Ordering};

/// Node representation
pub struct Node{
    /// Left child of this node
    left: Option<Box<Node>>,
    /// Right child of this node
    right: Option<Box<Node>>,
    /// The age of the data in this node
    age: i32,
    /// The name of the data in this node
    name: String,
    /// Height of this node
    height: i32,
}

/// AVL self balanced binary tree representation
pub struct Tree{
    /// Root node of the tree
    root: Option<Box<Node>>
}

impl Tree {
    /// Create a new tree.
    /// Returns empty tree.
    pub fn new() -> Tree {
        Tree { root: None }
    }

    /// Print a tree in the following format:
    /// [<data>, <left>, <right>]
    /// where the elements above have the following format:
    ///     <data>             {<age:int>: "<name:string>"}
    ///     <left>, <right>:   The same format as the root node. When a child node is NULL, the string NULL is to be printed.
    pub fn print(&self) {
        match self.root.borrow() {
            Some(node) => node.print(),
            None => print!("null")
        }
        println!()
    }

    /// Find an item in the tree. Return true if found, or false if not.
    ///
    /// # Argument
    ///
    /// * `age` - Age of the data of the newly inserted node
    /// * `name` - Name of the data of the newly inserted node
    pub fn find(&self, age: i32, name: String) -> bool {
        match &self.root{
            &Some(ref node) => {
                node.find(age, name)
            }
            &None => false
        }
    }

    /// Insert a new data point into the tree
    ///
    /// # Argument
    ///
    /// * `age` - Age of the data of the newly to be inserted node
    /// * `name` - Name of the data of the newly to be inserted node
    pub fn insert(&mut self, age: i32, name: String) {
        match self.root.take(){
            Some(node) => self.root = node_insert(node, age, name),
            None => self.root = Some(Box::new(Node::new(age, name.clone())))
        }
    }

    ///Remove a data point from a tree
    ///
    /// # Argument
    ///
    /// * `age` - Age of the data of the node to be deleted
    /// * `name` - Name of the data of the node to be deleted
    pub fn erase(&mut self, age: i32, name: String) {
        match self.root.take(){
            Some(root) => {
                self.root = node_delete(root, age, name);
            }
            None => return
        }
    }

    /// Delete an entire tree. This will delete the passed Node and all children below it
    pub fn delete(&mut self) {
        match self.root.take() {
            Some(node) => {
                self.root = tree_delete(node)
            }
            None => return
        }
    }
}

impl Node {

    /// Create a new node.
    /// Returns newly created node.
    pub fn new(age: i32, name: String) -> Node {
        Node {age, name, height : 1, left : None, right : None }
    }

    /// Return immutable reference to the age data of the node
    pub fn age(&self) -> &i32 {
        &self.age
    }

    /// Return immutable reference to the name data of the node
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Update height of this node
    ///
    /// # Discussion
    ///
    /// The height is updated by picking the maximum height of ots child and incrementing it by 1.
    pub fn update_height(&mut self) {
        self.height = cmp::max(height(&self.right),height(&self.left)) + 1
    }

    /// Find an item in the tree of this node. Return true if found, or false if not.
    ///
    /// # Argument
    ///
    /// * `age` - Age of the data of the node to be searched
    /// * `name` - Name of the data of the node to be searched
    pub fn find(&self, age: i32, name: String) -> bool {
        match age.cmp(self.age()) {
            Ordering::Equal => {
                match age.cmp(self.age()) {
                    Ordering::Equal => true,
                    Ordering::Less | Ordering::Greater => {
                        match &self.left {
                            &Some(ref node) => node.find(age, name),
                            &None => false
                        }
                    }
                }
            }
            Ordering::Less  => {
                match &self.left {
                    &Some(ref node) => node.find(age, name),
                    &None => false
                }
            },
            Ordering::Greater => {
                match &self.right {
                    &Some(ref node) => node.find(age, name),
                    &None => false
                }
            }
        }
    }

    /// Print a node and its child in the following format:
    /// [<data>, <left>, <right>]
    /// where the elements above have the following format:
    ///     <data>             {<age:int>: "<name:string>"}
    ///     <left>, <right>:   The same format as the root node. When a child node is NULL, the string NULL is to be printed.
    pub fn print(&self) {
        print!("[");
        print!("{{\"{}\":\"{}\"}},", self.age, self.name);
        if let &Some(ref left) = &self.left {
            left.print();
        }else {
            print!("null");
        }
        print!(",");
        if let &Some(ref right) = &self.right {
            right.print();
        }else {
            print!("null");
        }
        print!("]");
    }
}


/// Delete an entire tree. This will delete the passed Node and all children below it
///
/// # Argument
///
/// * `node` - A node for which tree should be deleted
fn tree_delete (mut node: Box<Node>) -> Option<Box<Node>>{
    if let Some(left) = node.left.take() {
        return tree_delete(left);
    }
    if let Some(right) = node.right.take() {
        return tree_delete(right);
    }
    None
}

/// Delete node(with the given age and name) from the tree of the given node.
/// Returns the root node of the tree.
///
/// # Arguments
///
/// * `node` - A node which is the root of the tree where the node(with the given age and name) should be deleted
/// * `age` - Age of the data of the node to be deleted
/// * `name` - Name of the data of the node to be deleted
///
/// # Discussion
///
/// The balance of the tree is automatically maintained after the deletion.
/// Thus, returned root node of the tree is already correctly balanced.
fn node_delete (mut node: Box<Node>, age: i32, name: String) -> Option<Box<Node>>{
    match age.cmp(node.age()){
        Ordering::Less => {
            if let Some(left) = node.left.take() {
                node.left = node_delete(left, age, name);
            }
        },
        Ordering::Greater => {
            if let Some(right) = node.right.take() {
                node.right = node_delete(right, age, name);
            }
        },
        Ordering::Equal => {
            if node.name != name {
                if let Some(left) = node.left.take() {
                    node.left = node_delete(left, age, name);
                    return Some(node);
                }
            }else{
                if node.right.is_none() && node.left.is_none(){
                    return None;
                }else if node.right.is_some() && node.left.is_none(){
                    node = node.right.unwrap();
                }else if node.left.is_some() && node.right.is_none(){
                    node = node.left.unwrap();
                }else{
                    if let Some(successor) = find_successor(&node) {
                        let s_age = successor.age().clone();
                        let s_name = successor.name().clone();
                        node.age = s_age.clone();
                        node.name = s_name.clone();
                        node.right = node_delete(node.right.unwrap(), s_age, s_name);
                    }
                }
            }
        }
    }
    node.update_height();
    balance(node)
}

/// Insert new node(with the given age and name) into appropriate place in the tree of the given node.
/// Returns the root node of the tree.
///
/// # Arguments
///
/// * `node` - A node which is the root of the tree into which we want to insert new node
/// * `age` - Age of the data of the newly to be inserted node
/// * `name` - Name of the data of the newly to be inserted node
///
/// # Discussion
///
/// The balance of the tree is automatically maintained after the insertion.
/// Thus, returned root node of the tree is already correctly balanced.
fn node_insert (mut node: Box<Node>, age: i32, name: String) -> Option<Box<Node>> {
    match age.cmp(node.age()) {
        Ordering::Equal => {
            match name.cmp(node.name()) {
                Ordering::Equal => {
                    return Some(node)
                },
                Ordering::Less | Ordering:: Greater => {
                    match node.left.take() {
                        Some(n) => node.left = node_insert(n, age, name.clone()),
                        None =>  node.left = Some(Box::new(Node::new(age, name.clone())))
                    }
                }
            }
        },
        Ordering::Less => {
            match node.left.take() {
                Some(n) => node.left = node_insert(n, age, name.clone()),
                None =>  node.left = Some(Box::new(Node::new(age, name.clone())))
            }
        }
        Ordering::Greater => {
            match node.right.take() {
                Some(n) => node.right = node_insert(n, age, name.clone()),
                None => node.right = Some(Box::new(Node::new(age, name.clone())))
            }
        }
    }
    node.update_height();
    balance(node)
}

/// Returns successor for the given node.
///
/// # Argument
///
/// * `node` - A node for which successor needs to be searched.
///
/// # Discussion
///
/// If no successor is found, the node itself is returned.
fn find_successor (node: &Box<Node>) -> Option<&Box<Node>> {
    return match node.right {
        Some(ref right) => {
            find_leftmost(right)
        },
        None => Some(node)
    }
}

/// Returns left most child node
///
/// # Argument
///
/// * `node` - A node for which leftmost child needs tobe searched
///
/// # Discussion
///
/// If no left child is found, the node itself is returned.
fn find_leftmost (node: &Box<Node>) -> Option<&Box<Node>> {
    match node.left {
        Some(ref left) => find_leftmost(left),
        None => Some(node)
    }
}

/// Rebalanced the subtree if needed and return the new/old balanced sub tree.
///
/// # Argument
///
/// * `node` - A node for which balancing should be performed
fn balance (mut node: Box<Node>) -> Option<Box<Node>> {
    let balance_factor = calc_balance(&node);

    if balance_factor > 1 {
        if get_balance(&node.left) >= 0 {
            return rotate_right(node);
        }else{
            if let Some(left) = node.left.take(){
                node.left = rotate_left(left);
                return rotate_right(node);
            }
        }
    }else if balance_factor < -1  {
        if get_balance(&node.right) <= 0 {
            return rotate_left(node);
        }else{
            if let Some(right) = node.right.take(){
                node.right = rotate_right(right);
                return rotate_left(node);
            }
        }
    }

    Some(node)
}

/// If given node is Some, it returns height of the node, otherwise 0 is returned.
///
/// # Argument
///
/// * `node` - A node for which its height should be returned.
pub fn height (node: &Option<Box<Node>>) -> i32{
    if node.is_none() {
        return 0;
    }else{
        return node.as_ref().unwrap().height;
    }
}

/// It returns balance factor of the node.
///
/// # Argument
///
/// * `node` - A node for which its balance factor should be returned.
pub fn calc_balance (node: &Box<Node>) -> i32{
    height(&node.left) - height(&node.right)
}

/// If given node is Some, it returns balance factor of the node, otherwise o is returned.
///
/// # Argument
///
/// * `node` - A node for which its balance factor should be returned.
pub fn get_balance (node: &Option<Box<Node>>) -> i32{
    if node.is_none() {
        0
    }else{
        calc_balance(node.as_ref().unwrap())
    }
}

/// Returns node rotated to the right.
///
/// # Arguments
///
/// * `y` - A node on which the rotation should be performed
///
/// # Visual illustration
///
/// ```
///     (y)           (x)
///     / \           / \
///   (x)  c   ==>   a  (y)
///   / \               / \
///  a   b             b   c
/// ```
fn rotate_right (mut y : Box<Node>) -> Option<Box<Node>>{
    let mut x = y.left.take().expect("error in rotate_right");
    let b = x.right.take();

    y.left = b;
    y.update_height(); // due to the borrow checker we have to update it before we assign it
    x.right = Some(y);
    x.update_height();

    Some(x)
}

/// Returns node rotated to the left.
///
/// # Arguments
/// * 'y' - A node on which the rotation should be performed
///
/// # Visual illustration
///
/// ```
///    (x)            (y)
///    / \            / \
///   a  (y)   ==>  (x)  c
///      / \        / \
///     b   c      a   b
/// ```
fn rotate_left (mut x : Box<Node>) -> Option<Box<Node>>{
    let mut y = x.right.take().expect("error in rotate_left");
    let b = y.left.take();

    x.right = b;
    x.update_height(); // due to the borrow checker we have to update it before we assign it
    y.left = Some(x);
    y.update_height();

    Some(y)
}