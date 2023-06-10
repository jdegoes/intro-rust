#![allow(dead_code, unreachable_code, unused_variables)]

// 05 - ITERATORS
//
// Rust has a built-in trait named Iterator that is used in for loops. Iterators
// are lazy and provide a way to produce a sequence of items. There is a rich API
// defined in the standard library to consume iterators and perform complex
// operations. The Iterator trait only requires implementors to define a method
// to get the next value. This makes it possible to define iterators that are
// infinite or that have a size that is not known in advance.
//
// In this section, you will learn how to use and create iterators.

mod basics {
    #[test]
    fn basic_for_over_vec() {
        let v = vec![1, 2, 3];
        #[allow(unused_mut)]
        let mut sum = 0;

        for i in v {
            todo!("Add i to sum")
        }

        assert_eq!(sum, 6);
    }

    #[test]
    fn basic_for_over_array() {
        let v = [1, 2, 3];
        #[allow(unused_mut)]
        let mut sum = 0;

        for i in v {
            todo!("Add i to sum")
        }

        assert_eq!(sum, 6);
    }

    #[test]
    fn custom_iterator() {
        enum Tree<A> {
            Leaf(A),
            Branch(Box<Tree<A>>, Box<Tree<A>>),
        }

        struct TreeIterator<'a, A> {
            current: Option<&'a Tree<A>>,
            todo: Vec<&'a Tree<A>>,
        }

        impl<'a, A> Iterator for TreeIterator<'a, A> {
            type Item = &'a A;

            fn next(&mut self) -> Option<Self::Item> {
                todo!("Implement next for TreeIterator")
            }
        }

        impl<'a, A> IntoIterator for &'a Tree<A> {
            type Item = &'a A;
            type IntoIter = TreeIterator<'a, A>;

            fn into_iter(self) -> Self::IntoIter {
                TreeIterator {
                    current: Some(&self),
                    todo: Vec::new(),
                }
            }
        }

        impl<A> Tree<A> {
            fn iter(&self) -> TreeIterator<'_, A> {
                TreeIterator {
                    current: Some(&self),
                    todo: Vec::new(),
                }
            }
        }

        let tree: Tree<i32> = Tree::Branch(
            Box::new(Tree::Branch(
                Box::new(Tree::Leaf(1)),
                Box::new(Tree::Leaf(2)),
            )),
            Box::new(Tree::Leaf(3)),
        );

        let mut sum = 0;

        for i in &tree {
            sum += *i;
        }

        assert_eq!(sum, 6);
    }
}

mod operators {
    #[test]
    fn map() {
        let v: Vec<i32> = vec![1, 2, 3];

        let into_iter = v.into_iter();

        let mapped = todo!("use into_iter.map to map each element to itself plus one");

        let collected: Vec<i32> = todo!("mapped.collect::<Vec<_>>()");

        assert_eq!(collected, vec![2, 3, 4]);
    }

    #[test]
    fn filter() {
        let v: Vec<i32> = vec![1, 2, 3];

        let into_iter = v.into_iter();

        let filtered = todo!("use into_iter.filter to filter out odd numbers");

        let collected: Vec<i32> = todo!("filtered.collect::<Vec<_>>()");

        assert_eq!(collected, vec![2]);
    }

    #[test]
    fn flat_map() {
        let v: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];

        let into_iter = v.into_iter();

        let flat_mapped = todo!("use into_iter.flat_map to flatten the vector");

        let collected: Vec<i32> = todo!("flat_mapped.collect::<Vec<_>>()");

        assert_eq!(collected, vec![1, 2, 3, 4]);
    }

    #[test]
    fn fold() {
        let v: Vec<i32> = vec![1, 2, 3];

        let into_iter = v.into_iter();

        let folded: i32 = todo!("use into_iter.fold to sum the elements");

        assert_eq!(folded, 6);
    }

    #[test]
    fn all() {
        let v: Vec<i32> = vec![1, 2, 3];

        #[allow(unused_mut)]
        let mut into_iter = v.into_iter();

        let all_even: bool = todo!("use into_iter.all to check if all elements are even");

        assert_eq!(all_even, false);
    }

    #[test]
    fn any() {
        let v: Vec<i32> = vec![1, 2, 3];

        #[allow(unused_mut)]
        let mut into_iter = v.into_iter();

        let any_even: bool = todo!("use into_iter.any to check if any element is even");

        assert_eq!(any_even, true);
    }

    #[test]
    fn find() {
        let v: Vec<i32> = vec![1, 2, 3];

        #[allow(unused_mut)]
        let mut into_iter = v.into_iter();

        let found: Option<i32> = todo!("use into_iter.find to find the first even element");

        assert_eq!(found, Some(2));
    }

    #[test]
    fn max() {
        let v: Vec<i32> = vec![1, 2, 3];

        #[allow(unused_mut)]
        let mut into_iter = v.into_iter();

        let max: Option<i32> = todo!("use into_iter.max to find the maximum element");

        assert_eq!(max, Some(3));
    }

    #[test]
    fn min() {
        let v: Vec<i32> = vec![1, 2, 3];

        #[allow(unused_mut)]
        let mut into_iter = v.into_iter();

        let min: Option<i32> = todo!("use into_iter.min to find the minimum element");

        assert_eq!(min, Some(1));
    }

    #[test]
    fn mutable_iteration() {
        let mut numbers = vec![1, 2, 3];

        // Find out what's wrong with this code and make it work:
        let incremented = numbers
            .iter_mut()
            .map(|x| todo!("x + 1") as i32)
            .collect::<Vec<_>>();

        assert_eq!(incremented, vec![2, 3, 4]);
    }
}
