/*
Vec, VecDeque, LinkedList
HashMap, BTreeMap
HashSet, BTreeSet
BinaryHeap

new() -> Self
with_capacity(n: usize) -> Self
len(&self) -> usize
is_empty(&self) -> bool
clear(&mut self)
get(), get_mut()
*/

/*
Vec
 Динамический массив с быстрой вставкой в конец
 Может работать как стек: push(&mut self, T), pop(&mut self) -> T с конца
 insert(&mut self, index: usize, element: T)
 remove(&mut self, index: usize)
 swap_remove(&mut self, index: usize)
 sort(&mut self), sort_unstable(&mut self) - faster than sort
 Быстрое конструирование с помощью vec![1, 2, 3]
 Реализует Deref<Target = [T]>
*/

#[test]
fn test_vec() {
    let mut v = vec![1, 2, 3];
    let r = v.swap_remove(1);
    assert_eq!(r, 2);
    assert_eq!(v, [1, 3]);

    let s: &Vec<i32> = &v;
    let slice: &[i32] = v.as_slice();
}

/*
HashMap
 Словарь (ассоциативный массив)
 Тип ключа должен реализовывать Eq и Hash
 insert(&mut self, key: K, value: V)
 remove(&mut self, key: &K)
 Можно менять используемый хеш: with_hasher(hasher: H) для роста производительности
 Entry API
*/

#[test]
fn test_hashmap() {
    use std::collections::HashMap;

    let string = "Let's count character in this string";
    let mut map = HashMap::new();
    for c in string.chars() {
        let mut count = map.entry(c).or_insert(0);
        *count += 1;
    }

    let mut distance0: HashMap<&str, f64> = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
    {
        let jupiter = String::from("Jupiter");
        // distance0.insert(&jupiter, 5.2); // ??? won't compile
    }
    println!("{:?}", distance0);

    let mut distance: HashMap<String, f64> = HashMap::from([
        ("Mercury".to_string(), 0.4),
        ("Venus".to_string(), 0.7),
        ("Earth".to_string(), 1.0),
        ("Mars".to_string(), 1.5),
    ]);
    {
        distance.insert("Jupiter".to_string(), 5.2);
    }
    let earth_distance = distance["Earth"]; // можно использовать &str!
}

#[test]
fn test_index() {
    /*trait Index<Idx: ?Sized> {
        type Output: ?Sized;
        fn index(&self, index: Idx) -> &Self::Output;
    }
    trait IndexMut<Idx>: Index<Idx> where Idx: ?Sized {
        fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
    }*/

    let vec = vec![1, 2, 3, 4, 5];
    //let num_ref: &i32 = vec[0]; // expects &i32 found i32
    //let num_ref: &i32 = *vec[0]; // expects &i32 found i32
    let num_ref: i32 = vec[0];
    let num_ref: &i32 = &vec[0];

    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(&vec[..], &[1, 2, 3, 4, 5]);
    assert_eq!(&vec[1..], &[2, 3, 4, 5]);
    assert_eq!(&vec[..4], &[1, 2, 3, 4]);
    assert_eq!(&vec[1..4], &[2, 3, 4]);

    let v = vec.into_iter().inspect(|v| println!("{}", v));
}

/*
type HashSet<T> = HashMap<T, ()>;
type BTreeSet<T> = BTreeMap<T, ()>;
*/

/*
.into_iter() Iterator<Item = T>
.iter()      Iterator<Item = &T>
.iter_mut()  Iterator<Item = &mut T>
*/

/*
Implement custom iterator
*/

struct MyType {
    items: Vec<String>,
}
impl MyType {
    fn iter(&self) -> impl Iterator<Item = &String> {
        MyTypeIterator {
            index: 0,
            items: &self.items,
        }
    }
}
// impl Trait - it's a way we don't want to define specific type and we can call methods of that type using trait only
// We can't define Iterator<Item = &String> + ?Sized only Iterator<Item = &String> + Sized allowed

struct MyTypeIterator<'a> {
    index: usize,
    items: &'a Vec<String>,
}
impl<'a> Iterator for MyTypeIterator<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.items.len() {
            None
        } else {
            let item = &self.items[self.index];
            self.index += 1;
            Some(item)
        }
    }
}
/*
used into for
trait IntoIterator
where
    <Self::IntoIter as Iterator>::Item == Self::Item,
{
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}

for v in vec {} desugared to for v in vec.into_iter() {}
for v in &vec {} desugared to for v in (&vec).into_iter() {}
for v in &mut vec {} desugared to for v in (&mut vec).into_iter() {}
for
desugared to
let mut iter = <iterator>.into_iter(); while let Some(<value>) = iter.next() { <body> }
*/

/*
allow build a collection from iterator elements
trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = A>;
}
// Iterator::collect
fn collect<B>(self) -> B where B: FromIterator<Self::Item>;

*/

#[test]
fn test_iter() {
    let c: usize = None::<i32>.into_iter().count();
    let v: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
    let n: Vec<i32> = None::<i32>.into_iter().collect::<Vec<i32>>();
    let iter_of_iter = v.into_iter().map(|e| e.into_iter());
    let res = iter_of_iter.flatten().collect::<Vec<i32>>();
    assert_eq!(res, [1, 3])
}

// crate https://docs.rs/itertools/latest/itertools extends std collections
