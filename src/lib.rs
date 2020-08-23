pub mod first;

#[cfg(test)]
mod tests {

    use crate::first;

    #[test]
    fn test_first() {

        let mut list = first::List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        list.push(6);
        list.push(7);
        list.push(8);
        list.push(9);
        list.push(10);
        list.push(11);
        list.push(12);
        list.push(13);
        list.push(14);

        println!("{:#?}", list);
        println!("{:?}", list.pop_node());
        println!("{:#?}", list);

    }


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
