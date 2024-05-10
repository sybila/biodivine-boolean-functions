use biodivine_lib_bdd::{Bdd, OwnedBddSatisfyingValuations};

pub struct SupportIterator {
    iterator: OwnedBddSatisfyingValuations,
}

impl SupportIterator {
    pub(crate) fn new(bdd: &Bdd) -> Self {
        Self {
            iterator: bdd.clone().into_sat_valuations(),
        }
    }
}

impl Iterator for SupportIterator {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|it| it.vector())
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
