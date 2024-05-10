use crate::iterators::DomainIterator;
use biodivine_lib_bdd::Bdd;
use biodivine_lib_bdd::BddValuation;

pub struct ImageIterator {
    domain_iterator: Box<dyn Iterator<Item = Vec<bool>>>,
    bdd: Bdd,
}

impl ImageIterator {
    pub(crate) fn new(input_count: usize, bdd: &Bdd) -> Self {
        Self {
            domain_iterator: Box::new(DomainIterator::from_count(input_count)),
            bdd: bdd.clone(),
        }
    }
}

impl Iterator for ImageIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.domain_iterator
            .next()
            .map(|it| self.bdd.eval_in(&BddValuation::new(it)))
    }
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::expressions::var;
//     use crate::traits::BooleanFunction;
//     use std::collections::BTreeMap;
//
//     #[test]
//     fn test_image_ok() {
//         let input = var("d") & var("b") | var("a");
//
//         let mut actual = input.image();
//         let expected = [
//             Some(input.evaluate(&BTreeMap::from([
//                 ("a".to_string(), false),
//                 ("b".to_string(), false),
//                 ("d".to_string(), false),
//             ]))),
//             Some(input.evaluate(&BTreeMap::from([
//                 ("a".to_string(), false),
//                 ("b".to_string(), false),
//                 ("d".to_string(), true),
//             ]))),
//             Some(input.evaluate(&BTreeMap::from([
//                 ("a".to_string(), false),
//                 ("b".to_string(), true),
//                 ("d".to_string(), false),
//             ]))),
//             Some(input.evaluate(&BTreeMap::from([
//                 ("a".to_string(), false),
//                 ("b".to_string(), true),
//                 ("d".to_string(), true),
//             ]))),
//             Some(input.evaluate(&BTreeMap::from([
//                 ("a".to_string(), true),
//                 ("b".to_string(), false),
//                 ("d".to_string(), false),
//             ]))),
//             Some(input.evaluate(&BTreeMap::from([
//                 ("a".to_string(), true),
//                 ("b".to_string(), false),
//                 ("d".to_string(), true),
//             ]))),
//             Some(input.evaluate(&BTreeMap::from([
//                 ("a".to_string(), true),
//                 ("b".to_string(), true),
//                 ("d".to_string(), false),
//             ]))),
//             Some(input.evaluate(&BTreeMap::from([
//                 ("a".to_string(), true),
//                 ("b".to_string(), true),
//                 ("d".to_string(), true),
//             ]))),
//         ];
//
//         assert_eq!(actual.next(), expected[0]);
//         assert_eq!(actual.next(), expected[1]);
//         assert_eq!(actual.next(), expected[2]);
//         assert_eq!(actual.next(), expected[3]);
//         assert_eq!(actual.next(), expected[4]);
//         assert_eq!(actual.next(), expected[5]);
//         assert_eq!(actual.next(), expected[6]);
//         assert_eq!(actual.next(), expected[7]);
//
//         assert_eq!(actual.next(), None);
//         assert_eq!(actual.next(), None);
//     }
// }
