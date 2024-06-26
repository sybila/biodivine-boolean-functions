use crate::traits::GatherLiterals;
use crate::utils::row_index_to_bool_point;
use std::fmt::Debug;

pub struct DomainIterator {
    variable_count: usize,
    index: usize,
}

impl DomainIterator {
    pub(crate) fn new<T: Debug + Clone + Ord>(value: &impl GatherLiterals<T>) -> Self {
        Self {
            variable_count: value.gather_literals().len(),
            index: 0,
        }
    }

    pub(crate) fn from_count(count: usize) -> Self {
        Self {
            variable_count: count,
            index: 0,
        }
    }
}

impl Iterator for DomainIterator {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 2_usize.pow(self.variable_count as u32) {
            return None;
        }

        let result = row_index_to_bool_point(self.index, self.variable_count);
        self.index += 1;

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::expressions::var;
    use crate::traits::BooleanFunction;
    use std::collections::BTreeMap;

    #[test]
    fn test_domain_ends() {
        let input = var("a") | var("b");
        let mut iterator = input.domain();

        assert!(iterator.next().is_some());
        assert!(iterator.next().is_some());
        assert!(iterator.next().is_some());
        assert!(iterator.next().is_some());
        assert!(iterator.next().is_none());
        assert!(iterator.next().is_none());
    }

    #[test]
    fn test_domain_ok() {
        let input = var("d") & var("b") | var("a");

        let actual = input.domain().collect::<Vec<_>>();

        let expected = vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_point_to_valuation_ok() -> Result<(), String> {
        let input = var("d") & var("b") | var("a");

        let actual = input
            .domain()
            .map(|point| input.boolean_point_to_valuation(point))
            .collect::<Option<Vec<_>>>()
            .ok_or("Failed to translate".to_string())?;

        let expected = vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ]
        .into_iter()
        .map(|point| {
            BTreeMap::from_iter(vec![
                ("a".to_string(), point[0]),
                ("b".to_string(), point[1]),
                ("d".to_string(), point[2]),
            ])
        })
        .collect::<Vec<_>>();

        assert_eq!(actual, expected);

        Ok(())
    }
}
