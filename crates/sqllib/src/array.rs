// Array operations

use crate::{some_function2, some_generic_function2, Variant, Weight};
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Index;

#[doc(hidden)]
pub fn array_map__<T, S, F>(vec: &[T], f: F) -> Vec<S>
where
    F: Fn(&T) -> S,
{
    vec.iter().map(f).collect()
}

#[doc(hidden)]
pub fn array_mapN_<T, S, F>(vec: &Option<Vec<T>>, f: F) -> Option<Vec<S>>
where
    F: Fn(&T) -> S,
{
    vec.as_ref().map(|vec| array_map__(vec, f))
}

#[doc(hidden)]
pub fn element__<T>(array: Vec<T>) -> Option<T>
where
    T: Clone,
{
    if array.is_empty() {
        None
    } else if array.len() == 1 {
        Some(array[0].clone())
    } else {
        panic!("'ELEMENT()' called on array that does not have exactly 1 element");
    }
}

#[doc(hidden)]
pub fn elementN_<T>(array: Option<Vec<T>>) -> Option<T>
where
    T: Clone,
{
    let array = array?;
    element__(array)
}

#[doc(hidden)]
pub fn element_N<T>(array: Vec<Option<T>>) -> Option<T>
where
    T: Clone,
{
    if array.is_empty() {
        None
    } else if array.len() == 1 {
        array[0].clone()
    } else {
        panic!("'ELEMENT()' called on array that does not have exactly 1 element");
    }
}

#[doc(hidden)]
pub fn elementNN<T>(array: Option<Vec<Option<T>>>) -> Option<T>
where
    T: Clone,
{
    let array = array?;
    element_N(array)
}

#[doc(hidden)]
pub fn cardinalityVec<T>(value: Vec<T>) -> i32 {
    value.len() as i32
}

#[doc(hidden)]
pub fn cardinalityVecN<T>(value: Option<Vec<T>>) -> Option<i32> {
    let value = value?;
    Some(value.len() as i32)
}

// 8 versions of index, depending on
// nullability of vector
// nullability of vector element
// nullability of index

#[doc(hidden)]
pub fn index___<T>(value: Vec<T>, index: isize) -> Option<T>
where
    T: Clone,
{
    if index < 0 {
        return None;
    };
    let index: usize = index as usize;
    if index >= value.len() {
        None
    } else {
        Some(value.index(index).clone())
    }
}

#[doc(hidden)]
pub fn index__N<T>(value: Vec<T>, index: Option<isize>) -> Option<T>
where
    T: Clone,
{
    let index = index?;
    index___(value, index)
}

#[doc(hidden)]
pub fn index_N_<T>(value: Vec<Option<T>>, index: isize) -> Option<T>
where
    T: Clone,
{
    if index < 0 {
        return None;
    };
    let index: usize = index as usize;
    if index >= value.len() {
        None
    } else {
        value.index(index).clone()
    }
}

#[doc(hidden)]
pub fn index_NN<T>(value: Vec<Option<T>>, index: Option<isize>) -> Option<T>
where
    T: Clone,
{
    let index = index?;
    if index < 0 {
        return None;
    };
    let index: usize = index as usize;
    if index >= value.len() {
        None
    } else {
        value.index(index).clone()
    }
}

#[doc(hidden)]
pub fn indexN__<T>(value: Option<Vec<T>>, index: isize) -> Option<T>
where
    T: Clone,
{
    match value {
        None => None,
        Some(value) => index___(value, index),
    }
}

#[doc(hidden)]
pub fn indexN_N<T>(value: Option<Vec<T>>, index: Option<isize>) -> Option<T>
where
    T: Clone,
{
    let index = index?;
    match value {
        None => None,
        Some(value) => index___(value, index),
    }
}

#[doc(hidden)]
pub fn indexNN_<T>(value: Option<Vec<Option<T>>>, index: isize) -> Option<T>
where
    T: Clone,
{
    match value {
        None => None,
        Some(value) => index_N_(value, index),
    }
}

#[doc(hidden)]
pub fn indexNNN<T>(value: Option<Vec<Option<T>>>, index: Option<isize>) -> Option<T>
where
    T: Clone,
{
    let index = index?;
    match value {
        None => None,
        Some(value) => index_N_(value, index),
    }
}

#[doc(hidden)]
pub fn array<T>() -> Vec<T> {
    vec![]
}

#[doc(hidden)]
pub fn limit<T>(vector: &[T], limit: usize) -> Vec<T>
where
    T: Clone,
{
    vector[0..limit].to_vec()
}

#[doc(hidden)]
pub fn map<T, S, F>(vector: &[T], func: F) -> Vec<S>
where
    F: FnMut(&T) -> S,
{
    vector.iter().map(func).collect()
}

#[doc(hidden)]
pub fn array_append<T>(mut vector: Vec<T>, value: T) -> Vec<T> {
    vector.push(value);
    vector
}

#[doc(hidden)]
pub fn array_appendN<T>(vector: Option<Vec<T>>, value: T) -> Option<Vec<T>> {
    Some(array_append(vector?, value))
}

#[doc(hidden)]
pub fn array_repeat__<T>(element: T, count: i32) -> Vec<T>
where
    T: Clone,
{
    std::iter::repeat(element)
        .take(usize::try_from(count).unwrap_or(0))
        .collect()
}

#[doc(hidden)]
pub fn array_repeatN_<T>(element: Option<T>, count: i32) -> Option<Vec<Option<T>>>
where
    T: Clone,
{
    Some(array_repeat__(element, count))
}

#[doc(hidden)]
pub fn array_repeat_N<T>(element: T, count: Option<i32>) -> Option<Vec<T>>
where
    T: Clone,
{
    Some(array_repeat__(element, count?))
}

#[doc(hidden)]
pub fn array_repeatNN<T>(element: Option<T>, count: Option<i32>) -> Option<Vec<Option<T>>>
where
    T: Clone,
{
    Some(array_repeat__(element, count?))
}

#[doc(hidden)]
pub fn array_remove__<T>(mut vector: Vec<T>, element: T) -> Vec<T>
where
    T: Eq,
{
    vector.retain(|v| v != &element);
    vector
}

some_generic_function2!(array_remove, T, Vec<T>, T, Eq, Vec<T>);

#[doc(hidden)]
pub fn array_position__<T>(vector: Vec<T>, element: T) -> i64
where
    T: Eq,
{
    vector
        .into_iter()
        .position(|x| x == element)
        .map(|v| v + 1)
        .unwrap_or(0) as i64
}

some_generic_function2!(array_position, T, Vec<T>, T, Eq, i64);

#[doc(hidden)]
pub fn array_reverse_<T>(vector: Vec<T>) -> Vec<T> {
    vector.into_iter().rev().collect()
}

#[doc(hidden)]
pub fn array_reverseN<T>(vector: Option<Vec<T>>) -> Option<Vec<T>> {
    Some(array_reverse_(vector?))
}

#[doc(hidden)]
pub fn sort_array<T>(mut vector: Vec<T>, ascending: bool) -> Vec<T>
where
    T: Ord,
{
    if ascending {
        vector.sort()
    } else {
        vector.sort_by(|a, b| b.cmp(a))
    };

    vector
}

#[doc(hidden)]
pub fn sort_arrayN<T>(vector: Option<Vec<T>>, ascending: bool) -> Option<Vec<T>>
where
    T: Ord,
{
    Some(sort_array(vector?, ascending))
}

#[doc(hidden)]
pub fn array_max__<T>(vector: Vec<T>) -> Option<T>
where
    T: Ord,
{
    vector.into_iter().max()
}

#[doc(hidden)]
pub fn array_maxN_<T>(vector: Option<Vec<T>>) -> Option<T>
where
    T: Ord,
{
    array_max__(vector?)
}

#[doc(hidden)]
pub fn array_max_N<T>(vector: Vec<Option<T>>) -> Option<T>
where
    T: Ord,
{
    vector.into_iter().flatten().max()
}

#[doc(hidden)]
pub fn array_maxNN<T>(vector: Option<Vec<Option<T>>>) -> Option<T>
where
    T: Ord,
{
    array_max_N(vector?)
}

#[doc(hidden)]
pub fn array_min__<T>(vector: Vec<T>) -> Option<T>
where
    T: Ord,
{
    vector.into_iter().min()
}

#[doc(hidden)]
pub fn array_minN_<T>(vector: Option<Vec<T>>) -> Option<T>
where
    T: Ord,
{
    array_min__(vector?)
}

#[doc(hidden)]
pub fn array_min_N<T>(vector: Vec<Option<T>>) -> Option<T>
where
    T: Ord,
{
    vector.into_iter().flatten().min()
}

#[doc(hidden)]
pub fn array_minNN<T>(vector: Option<Vec<Option<T>>>) -> Option<T>
where
    T: Ord,
{
    array_min_N(vector?)
}

#[doc(hidden)]
pub fn array_compact_<T>(vector: Vec<Option<T>>) -> Vec<T> {
    vector.into_iter().flatten().collect()
}

#[doc(hidden)]
pub fn array_compact_N<T>(vector: Option<Vec<Option<T>>>) -> Option<Vec<T>> {
    Some(array_compact_(vector?))
}

#[doc(hidden)]
pub fn array_prepend<T>(mut vector: Vec<T>, value: T) -> Vec<T> {
    vector.insert(0, value);
    vector
}

#[doc(hidden)]
pub fn array_prependN<T>(vector: Option<Vec<T>>, value: T) -> Option<Vec<T>> {
    Some(array_prepend(vector?, value))
}

#[doc(hidden)]
pub fn array_contains__<T>(vector: Vec<T>, element: T) -> bool
where
    T: Eq,
{
    vector.contains(&element)
}

some_generic_function2!(array_contains, T, Vec<T>, T, Eq, bool);

#[doc(hidden)]
pub fn array_distinct<T>(mut vector: Vec<T>) -> Vec<T>
where
    T: Eq + Hash + Clone,
{
    let mut hset: HashSet<T> = HashSet::new();
    vector.retain(|v| hset.insert(v.clone()));
    vector
}

#[doc(hidden)]
pub fn array_distinctN<T>(vector: Option<Vec<T>>) -> Option<Vec<T>>
where
    T: Eq + Hash + Clone,
{
    Some(array_distinct(vector?))
}

#[doc(hidden)]
pub fn sequence__(start: i32, end: i32) -> Vec<i32> {
    (start..=end).collect()
}

some_function2!(sequence, i32, i32, Vec<i32>);

// translated from the Calcite implementation to match the behavior
#[doc(hidden)]
pub fn arrays_overlapNvec_Nvec_<T>(first: Vec<Option<T>>, second: Vec<Option<T>>) -> Option<bool>
where
    T: Eq + Hash,
{
    if first.len() > second.len() {
        return arrays_overlapNvec_Nvec_(second, first);
    }

    let (smaller, bigger) = (first, second);
    let mut has_null = false;

    if !smaller.is_empty() && !bigger.is_empty() {
        let mut shset: HashSet<Option<T>> = HashSet::from_iter(smaller);
        has_null = shset.remove(&None);

        for element in bigger {
            if element.is_none() {
                has_null = true;
            } else if shset.contains(&element) {
                return Some(true);
            }
        }
    }
    if has_null {
        None
    } else {
        Some(true)
    }
}

#[doc(hidden)]
pub fn arrays_overlapNvec_NvecN<T>(
    first: Vec<Option<T>>,
    second: Option<Vec<Option<T>>>,
) -> Option<bool>
where
    T: Eq + Hash,
{
    arrays_overlapNvec_Nvec_(first, second?)
}

#[doc(hidden)]
pub fn arrays_overlapNvecNNvecN<T>(
    first: Option<Vec<Option<T>>>,
    second: Option<Vec<Option<T>>>,
) -> Option<bool>
where
    T: Eq + Hash,
{
    arrays_overlapNvec_Nvec_(first?, second?)
}

#[doc(hidden)]
pub fn arrays_overlapNvecNNvec_<T>(
    first: Option<Vec<Option<T>>>,
    second: Vec<Option<T>>,
) -> Option<bool>
where
    T: Eq + Hash,
{
    arrays_overlapNvec_Nvec_(first?, second)
}

#[doc(hidden)]
pub fn arrays_overlap_vec__vec_<T>(first: Vec<T>, second: Vec<T>) -> bool
where
    T: Eq + Hash,
{
    let first: HashSet<T> = HashSet::from_iter(first);
    let second: HashSet<T> = HashSet::from_iter(second);

    first.intersection(&second).count() != 0
}

#[doc(hidden)]
pub fn arrays_overlap_vecN_vecN<T>(first: Option<Vec<T>>, second: Option<Vec<T>>) -> Option<bool>
where
    T: Eq + Hash,
{
    Some(arrays_overlap_vec__vec_(first?, second?))
}

#[doc(hidden)]
pub fn arrays_overlap_vec__vecN<T>(first: Vec<T>, second: Option<Vec<T>>) -> Option<bool>
where
    T: Eq + Hash,
{
    Some(arrays_overlap_vec__vec_(first, second?))
}

#[doc(hidden)]
pub fn arrays_overlap_vecN_vec_<T>(first: Option<Vec<T>>, second: Vec<T>) -> Option<bool>
where
    T: Eq + Hash,
{
    Some(arrays_overlap_vec__vec_(first?, second))
}

#[doc(hidden)]
pub fn array_agg<T>(
    accumulator: &mut Vec<T>,
    value: T,
    weight: Weight,
    distinct: bool,
    keep: bool,
) -> Vec<T>
where
    T: Clone,
{
    if weight < 0 {
        panic!("Negative weight {:?}", weight);
    }
    if distinct && keep {
        accumulator.push(value.clone())
    } else if keep {
        for _i in 0..weight {
            accumulator.push(value.clone())
        }
    }
    accumulator.to_vec()
}

#[doc(hidden)]
pub fn array_aggN<T>(
    accumulator: &mut Option<Vec<T>>,
    value: T,
    weight: Weight,
    distinct: bool,
    keep: bool,
) -> Option<Vec<T>>
where
    T: Clone,
{
    accumulator
        .as_mut()
        .map(|accumulator| array_agg(accumulator, value, weight, distinct, keep))
}

#[doc(hidden)]
pub fn array_agg_opt<T>(
    accumulator: &mut Vec<Option<T>>,
    value: Option<T>,
    weight: Weight,
    distinct: bool,
    keep: bool,
    ignore_nulls: bool,
) -> Vec<Option<T>>
where
    T: Clone,
{
    if ignore_nulls && value.is_none() {
        accumulator.to_vec()
    } else {
        array_agg(accumulator, value, weight, distinct, keep)
    }
}

#[doc(hidden)]
pub fn array_agg_optN<T>(
    accumulator: &mut Option<Vec<Option<T>>>,
    value: Option<T>,
    weight: Weight,
    distinct: bool,
    keep: bool,
    ignore_nulls: bool,
) -> Option<Vec<Option<T>>>
where
    T: Clone,
{
    accumulator
        .as_mut()
        .map(|accumulator| array_agg_opt(accumulator, value, weight, distinct, keep, ignore_nulls))
}

/////////////// Variant index

// Return type is always Option<Variant>, but result is never None, always a Variant
#[doc(hidden)]
pub fn indexV__<T>(value: Variant, index: T) -> Option<Variant>
where
    T: Into<Variant>,
{
    value.index(index.into())
}

#[doc(hidden)]
pub fn indexV_N<T>(value: Variant, index: Option<T>) -> Option<Variant>
where
    T: Into<Variant>,
{
    let index = index?;
    indexV__(value, index)
}

#[doc(hidden)]
pub fn indexVN_<T>(value: Option<Variant>, index: T) -> Option<Variant>
where
    T: Into<Variant>,
{
    let value = value?;
    indexV__(value, index)
}

#[doc(hidden)]
pub fn indexVNN<T>(value: Option<Variant>, index: Option<T>) -> Option<Variant>
where
    T: Into<Variant>,
{
    let value = value?;
    let index = index?;
    indexV__(value, index)
}
