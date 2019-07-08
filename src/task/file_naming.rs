use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::hash_map::{self, HashMap};
use std::fmt;
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker;
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;

/*
You are given an array of desired filenames in the order of their creation. Since two files cannot
have equal names, the one which comes later will have an addition to its name in a form of (k),
where k is the smallest positive integer such that the obtained name is not used yet.

Return an array of names that will be given to the files.
*/
pub fn fileNaming(names: Vec<String>) -> Vec<String> {
    let mut result = LinkedHashMap::new();

    for x in names {
        add_filename(x, 0, &mut result);
    }

    result.keys().cloned().into_iter().collect()
}

fn add_filename(file_name: String, incrementer: i32, result: &mut LinkedHashMap<String, u8>) {
    let mut nef_file_name = String::from(file_name.clone());
    if incrementer != 0 {
        nef_file_name.push_str("(");
        nef_file_name.push_str(&incrementer.to_string());
        nef_file_name.push_str(")");
    }

    let possible_value = result.contains_key(&nef_file_name);

    if possible_value {
        add_filename(file_name, incrementer + 1, result);
    } else {
        result.insert(nef_file_name, 0);
    }
}

// Solution better to use with LinkedHashSet (here LinkedHashMap instead)
struct KeyRef<K> {
    k: *const K,
}

struct Node<K, V> {
    next: *mut Node<K, V>,
    prev: *mut Node<K, V>,
    key: K,
    value: V,
}

pub struct LinkedHashMap<K, V, S = hash_map::RandomState> {
    map: HashMap<KeyRef<K>, *mut Node<K, V>, S>,
    head: *mut Node<K, V>,
    free: *mut Node<K, V>,
}

impl<K: Hash> Hash for KeyRef<K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { (*self.k).hash(state) }
    }
}

impl<K: PartialEq> PartialEq for KeyRef<K> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.k).eq(&*other.k) }
    }
}

impl<K: Eq> Eq for KeyRef<K> {}

#[derive(Hash, PartialEq, Eq)]
struct Qey<Q: ?Sized>(Q);

impl<Q: ?Sized> Qey<Q> {
    fn from_ref(q: &Q) -> &Self {
        unsafe { mem::transmute(q) }
    }
}

impl<K, Q: ?Sized> Borrow<Qey<Q>> for KeyRef<K>
    where
        K: Borrow<Q>,
{
    fn borrow(&self) -> &Qey<Q> {
        Qey::from_ref(unsafe { (*self.k).borrow() })
    }
}

impl<K, V> Node<K, V> {
    fn new(k: K, v: V) -> Self {
        Node {
            key: k,
            value: v,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }
    }
}

unsafe fn drop_empty_node<K, V>(the_box: *mut Node<K, V>) {
    let Node { key, value, .. } = *Box::from_raw(the_box);
    mem::forget(key);
    mem::forget(value);
}

impl<K: Hash + Eq, V> LinkedHashMap<K, V> {
    pub fn new() -> Self {
        Self::with_map(HashMap::new())
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_map(HashMap::with_capacity(capacity))
    }
}

impl<K, V, S> LinkedHashMap<K, V, S> {
    #[inline]
    fn detach(&mut self, node: *mut Node<K, V>) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }

    #[inline]
    fn attach(&mut self, node: *mut Node<K, V>) {
        unsafe {
            (*node).next = (*self.head).next;
            (*node).prev = self.head;
            (*self.head).next = node;
            (*(*node).next).prev = node;
        }
    }

    unsafe fn drop_entries(&mut self) {
        let mut cur = (*self.head).next;
        while cur != self.head {
            let next = (*cur).next;
            Box::from_raw(cur);
            cur = next;
        }
    }

    fn clear_free_list(&mut self) {
        unsafe {
            let mut free = self.free;
            while !free.is_null() {
                let next_free = (*free).next;
                drop_empty_node(free);
                free = next_free;
            }
            self.free = ptr::null_mut();
        }
    }

    fn ensure_guard_node(&mut self) {
        if self.head.is_null() {
            unsafe {
                self.head = Box::into_raw(Box::new(mem::uninitialized()));
                (*self.head).next = self.head;
                (*self.head).prev = self.head;
            }
        }
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> LinkedHashMap<K, V, S> {
    fn with_map(map: HashMap<KeyRef<K>, *mut Node<K, V>, S>) -> Self {
        LinkedHashMap {
            map: map,
            head: ptr::null_mut(),
            free: ptr::null_mut(),
        }
    }
    pub fn with_hasher(hash_builder: S) -> Self {
        Self::with_map(HashMap::with_hasher(hash_builder))
    }

    pub fn reserve(&mut self, additional: usize) {
        self.map.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.map.shrink_to_fit();
        self.clear_free_list();
    }

    pub fn entry(&mut self, k: K) -> Entry<K, V, S> {
        let self_ptr: *mut Self = self;

        if let Some(entry) = self.map.get_mut(&KeyRef { k: &k }) {
            return Entry::Occupied(OccupiedEntry {
                entry: *entry,
                map: self_ptr,
                marker: marker::PhantomData,
            });
        }

        Entry::Vacant(VacantEntry { key: k, map: self })
    }
    pub fn entries(&mut self) -> Entries<K, V, S> {
        let head = if !self.head.is_null() {
            unsafe { (*self.head).prev }
        } else {
            ptr::null_mut()
        };
        Entries {
            map: self,
            head: head,
            remaining: self.len(),
            marker: marker::PhantomData,
        }
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.ensure_guard_node();

        let (node, old_val) = match self.map.get(&KeyRef { k: &k }) {
            Some(node) => {
                let old_val = unsafe { ptr::replace(&mut (**node).value, v) };
                (*node, Some(old_val))
            }
            None => {
                let node = if self.free.is_null() {
                    Box::into_raw(Box::new(Node::new(k, v)))
                } else {
                    unsafe {
                        let free = self.free;
                        self.free = (*free).next;
                        ptr::write(free, Node::new(k, v));
                        free
                    }
                };
                (node, None)
            }
        };
        match old_val {
            Some(_) => {
                self.detach(node);
                self.attach(node);
            }
            None => {
                let keyref = unsafe { &(*node).key };
                self.map.insert(KeyRef { k: keyref }, node);
                self.attach(node);
            }
        }
        old_val
    }

    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
        where
            K: Borrow<Q>,
            Q: Eq + Hash,
    {
        self.map.contains_key(Qey::from_ref(k))
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: Eq + Hash,
    {
        self.map
            .get(Qey::from_ref(k))
            .map(|e| unsafe { &(**e).value })
    }

    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: Eq + Hash,
    {
        self.map
            .get(Qey::from_ref(k))
            .map(|e| unsafe { &mut (**e).value })
    }
    pub fn get_refresh<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: Eq + Hash,
    {
        let (value, node_ptr_opt) = match self.map.get(Qey::from_ref(k)) {
            None => (None, None),
            Some(node) => (Some(unsafe { &mut (**node).value }), Some(*node)),
        };
        if let Some(node_ptr) = node_ptr_opt {
            self.detach(node_ptr);
            self.attach(node_ptr);
        }
        value
    }

    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: Eq + Hash,
    {
        let removed = self.map.remove(Qey::from_ref(k));
        removed.map(|node| {
            self.detach(node);
            unsafe {
                (*node).next = self.free;
                self.free = node;
                drop(ptr::read(&(*node).key));
                ptr::read(&(*node).value)
            }
        })
    }

    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    #[inline]
    pub fn pop_front(&mut self) -> Option<(K, V)> {
        if self.is_empty() {
            return None;
        }
        let lru = unsafe { (*self.head).prev };
        self.detach(lru);
        self.map
            .remove(&KeyRef {
                k: unsafe { &(*lru).key },
            })
            .map(|e| {
                let e = *unsafe { Box::from_raw(e) };
                (e.key, e.value)
            })
    }

    #[inline]
    pub fn front(&self) -> Option<(&K, &V)> {
        if self.is_empty() {
            return None;
        }
        let lru = unsafe { (*self.head).prev };
        self.map
            .get(&KeyRef {
                k: unsafe { &(*lru).key },
            })
            .map(|e| unsafe { (&(**e).key, &(**e).value) })
    }

    #[inline]
    pub fn pop_back(&mut self) -> Option<(K, V)> {
        if self.is_empty() {
            return None;
        }
        let mru = unsafe { (*self.head).next };
        self.detach(mru);
        self.map
            .remove(&KeyRef {
                k: unsafe { &(*mru).key },
            })
            .map(|e| {
                let e = *unsafe { Box::from_raw(e) };
                (e.key, e.value)
            })
    }

    #[inline]
    pub fn back(&mut self) -> Option<(&K, &V)> {
        if self.is_empty() {
            return None;
        }
        let mru = unsafe { (*self.head).next };
        self.map
            .get(&KeyRef {
                k: unsafe { &(*mru).key },
            })
            .map(|e| unsafe { (&(**e).key, &(**e).value) })
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn hasher(&self) -> &S {
        self.map.hasher()
    }

    pub fn clear(&mut self) {
        self.map.clear();
        if !self.head.is_null() {
            unsafe {
                self.drop_entries();
                (*self.head).prev = self.head;
                (*self.head).next = self.head;
            }
        }
    }

    pub fn iter(&self) -> Iter<K, V> {
        let head = if self.head.is_null() {
            ptr::null_mut()
        } else {
            unsafe { (*self.head).prev }
        };
        Iter {
            head: head,
            tail: self.head,
            remaining: self.len(),
            marker: marker::PhantomData,
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        let head = if self.head.is_null() {
            ptr::null_mut()
        } else {
            unsafe { (*self.head).prev }
        };
        IterMut {
            head: head,
            tail: self.head,
            remaining: self.len(),
            marker: marker::PhantomData,
        }
    }

    pub fn keys(&self) -> Keys<K, V> {
        Keys { inner: self.iter() }
    }
    pub fn values(&self) -> Values<K, V> {
        Values { inner: self.iter() }
    }
}

impl<'a, K, V, S, Q: ?Sized> Index<&'a Q> for LinkedHashMap<K, V, S>
    where
        K: Hash + Eq + Borrow<Q>,
        S: BuildHasher,
        Q: Eq + Hash,
{
    type Output = V;

    fn index(&self, index: &'a Q) -> &V {
        self.get(index).expect("no entry found for key")
    }
}

impl<'a, K, V, S, Q: ?Sized> IndexMut<&'a Q> for LinkedHashMap<K, V, S>
    where
        K: Hash + Eq + Borrow<Q>,
        S: BuildHasher,
        Q: Eq + Hash,
{
    fn index_mut(&mut self, index: &'a Q) -> &mut V {
        self.get_mut(index).expect("no entry found for key")
    }
}

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher + Clone> Clone for LinkedHashMap<K, V, S> {
    fn clone(&self) -> Self {
        let mut map = Self::with_hasher(self.map.hasher().clone());
        map.extend(self.iter().map(|(k, v)| (k.clone(), v.clone())));
        map
    }
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> Default for LinkedHashMap<K, V, S> {
    fn default() -> Self {
        Self::with_hasher(S::default())
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> Extend<(K, V)> for LinkedHashMap<K, V, S> {
    fn extend<I: IntoIterator<Item=(K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<'a, K, V, S> Extend<(&'a K, &'a V)> for LinkedHashMap<K, V, S>
    where
        K: 'a + Hash + Eq + Copy,
        V: 'a + Copy,
        S: BuildHasher,
{
    fn extend<I: IntoIterator<Item=(&'a K, &'a V)>>(&mut self, iter: I) {
        for (&k, &v) in iter {
            self.insert(k, v);
        }
    }
}

impl<A: fmt::Debug + Hash + Eq, B: fmt::Debug, S: BuildHasher> fmt::Debug
for LinkedHashMap<A, B, S>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self).finish()
    }
}

impl<K: Hash + Eq, V: PartialEq, S: BuildHasher> PartialEq for LinkedHashMap<K, V, S> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<K: Hash + Eq, V: Eq, S: BuildHasher> Eq for LinkedHashMap<K, V, S> {}

impl<K: Hash + Eq + PartialOrd, V: PartialOrd, S: BuildHasher> PartialOrd
for LinkedHashMap<K, V, S>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }

    fn lt(&self, other: &Self) -> bool {
        self.iter().lt(other)
    }

    fn le(&self, other: &Self) -> bool {
        self.iter().le(other)
    }

    fn ge(&self, other: &Self) -> bool {
        self.iter().ge(other)
    }

    fn gt(&self, other: &Self) -> bool {
        self.iter().gt(other)
    }
}

impl<K: Hash + Eq + Ord, V: Ord, S: BuildHasher> Ord for LinkedHashMap<K, V, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<K: Hash + Eq, V: Hash, S: BuildHasher> Hash for LinkedHashMap<K, V, S> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        for e in self.iter() {
            e.hash(h);
        }
    }
}

unsafe impl<K: Send, V: Send, S: Send> Send for LinkedHashMap<K, V, S> {}

unsafe impl<K: Sync, V: Sync, S: Sync> Sync for LinkedHashMap<K, V, S> {}

impl<K, V, S> Drop for LinkedHashMap<K, V, S> {
    fn drop(&mut self) {
        if !self.head.is_null() {
            unsafe {
                self.drop_entries();
                drop_empty_node(self.head);
            }
        }
        self.clear_free_list();
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    head: *const Node<K, V>,
    tail: *const Node<K, V>,
    remaining: usize,
    marker: marker::PhantomData<(&'a K, &'a V)>,
}

pub struct IterMut<'a, K: 'a, V: 'a> {
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
    remaining: usize,
    marker: marker::PhantomData<(&'a K, &'a mut V)>,
}

pub struct IntoIter<K, V> {
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
    remaining: usize,
    marker: marker::PhantomData<(K, V)>,
}

pub struct Entries<'a, K: 'a, V: 'a, S: 'a = hash_map::RandomState> {
    map: *mut LinkedHashMap<K, V, S>,
    head: *mut Node<K, V>,
    remaining: usize,
    marker: marker::PhantomData<(&'a K, &'a mut V, &'a S)>,
}

unsafe impl<'a, K, V> Send for Iter<'a, K, V>
    where
        K: Send,
        V: Send,
{}

unsafe impl<'a, K, V> Send for IterMut<'a, K, V>
    where
        K: Send,
        V: Send,
{}

unsafe impl<K, V> Send for IntoIter<K, V>
    where
        K: Send,
        V: Send,
{}

unsafe impl<'a, K, V, S> Send for Entries<'a, K, V, S>
    where
        K: Send,
        V: Send,
        S: Send,
{}

unsafe impl<'a, K, V> Sync for Iter<'a, K, V>
    where
        K: Sync,
        V: Sync,
{}

unsafe impl<'a, K, V> Sync for IterMut<'a, K, V>
    where
        K: Sync,
        V: Sync,
{}

unsafe impl<K, V> Sync for IntoIter<K, V>
    where
        K: Sync,
        V: Sync,
{}

unsafe impl<'a, K, V, S> Sync for Entries<'a, K, V, S>
    where
        K: Sync,
        V: Sync,
        S: Sync,
{}

impl<'a, K, V> Clone for Iter<'a, K, V> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}

impl<K, V> Clone for IntoIter<K, V>
    where
        K: Clone,
        V: Clone,
{
    fn clone(&self) -> Self {
        if self.remaining == 0 {
            return IntoIter { ..*self };
        }

        fn clone_node<K, V>(e: *mut Node<K, V>) -> *mut Node<K, V>
            where
                K: Clone,
                V: Clone,
        {
            Box::into_raw(Box::new(Node::new(unsafe { (*e).key.clone() }, unsafe {
                (*e).value.clone()
            })))
        }

        let mut cur = self.head;
        let head = clone_node(cur);
        let mut tail = head;
        for _ in 1..self.remaining {
            unsafe {
                (*tail).prev = clone_node((*cur).prev);
                (*(*tail).prev).next = tail;
                tail = (*tail).prev;
                cur = (*cur).prev;
            }
        }

        IntoIter {
            head: head,
            tail: tail,
            remaining: self.remaining,
            marker: marker::PhantomData,
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        if self.head == self.tail {
            None
        } else {
            self.remaining -= 1;
            unsafe {
                let r = Some((&(*self.head).key, &(*self.head).value));
                self.head = (*self.head).prev;
                r
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        if self.head == self.tail {
            None
        } else {
            self.remaining -= 1;
            unsafe {
                let r = Some((&(*self.head).key, &mut (*self.head).value));
                self.head = (*self.head).prev;
                r
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        unsafe {
            let prev = (*self.head).prev;
            let e = *Box::from_raw(self.head);
            self.head = prev;
            Some((e.key, e.value))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, K, V, S: BuildHasher> Iterator for Entries<'a, K, V, S> {
    type Item = OccupiedEntry<'a, K, V, S>;

    fn next(&mut self) -> Option<OccupiedEntry<'a, K, V, S>> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            unsafe {
                let r = Some(OccupiedEntry {
                    map: self.map,
                    entry: self.head,
                    marker: marker::PhantomData,
                });

                self.head = (*self.head).prev;
                r
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a, K, V> DoubleEndedIterator for Iter<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a V)> {
        if self.head == self.tail {
            None
        } else {
            self.remaining -= 1;
            unsafe {
                self.tail = (*self.tail).next;
                Some((&(*self.tail).key, &(*self.tail).value))
            }
        }
    }
}

impl<'a, K, V> DoubleEndedIterator for IterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a mut V)> {
        if self.head == self.tail {
            None
        } else {
            self.remaining -= 1;
            unsafe {
                self.tail = (*self.tail).next;
                Some((&(*self.tail).key, &mut (*self.tail).value))
            }
        }
    }
}

impl<K, V> DoubleEndedIterator for IntoIter<K, V> {
    fn next_back(&mut self) -> Option<(K, V)> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        unsafe {
            let next = (*self.tail).next;
            let e = *Box::from_raw(self.tail);
            self.tail = next;
            Some((e.key, e.value))
        }
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<K, V> ExactSizeIterator for IntoIter<K, V> {
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<K, V> Drop for IntoIter<K, V> {
    fn drop(&mut self) {
        for _ in 0..self.remaining {
            unsafe {
                let next = (*self.tail).next;
                Box::from_raw(self.tail);
                self.tail = next;
            }
        }
    }
}

pub struct Keys<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K, V> Clone for Keys<'a, K, V> {
    fn clone(&self) -> Self {
        Keys {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, K, V> Iterator for Keys<'a, K, V> {
    type Item = &'a K;

    #[inline]
    fn next(&mut self) -> Option<&'a K> {
        self.inner.next().map(|e| e.0)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Keys<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a K> {
        self.inner.next_back().map(|e| e.0)
    }
}

impl<'a, K, V> ExactSizeIterator for Keys<'a, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

/// An insertion-order iterator over a `LinkedHashMap`'s values.
pub struct Values<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K, V> Clone for Values<'a, K, V> {
    fn clone(&self) -> Self {
        Values {
            inner: self.inner.clone(),
        }
    }
}

impl<'a, K, V> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|e| e.1)
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, K, V> DoubleEndedIterator for Values<'a, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a V> {
        self.inner.next_back().map(|e| e.1)
    }
}

impl<'a, K, V> ExactSizeIterator for Values<'a, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a, K: Hash + Eq, V, S: BuildHasher> IntoIterator for &'a LinkedHashMap<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    fn into_iter(self) -> Iter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K: Hash + Eq, V, S: BuildHasher> IntoIterator for &'a mut LinkedHashMap<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;
    fn into_iter(self) -> IterMut<'a, K, V> {
        self.iter_mut()
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> IntoIterator for LinkedHashMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;
    fn into_iter(mut self) -> IntoIter<K, V> {
        let (head, tail) = if !self.head.is_null() {
            unsafe { ((*self.head).prev, (*self.head).next) }
        } else {
            (ptr::null_mut(), ptr::null_mut())
        };
        let len = self.len();

        if !self.head.is_null() {
            unsafe { drop_empty_node(self.head) }
        }
        self.clear_free_list();
        // drop the HashMap but not the LinkedHashMap
        self.map = unsafe { mem::uninitialized() };
        mem::forget(self);

        IntoIter {
            head: head,
            tail: tail,
            remaining: len,
            marker: marker::PhantomData,
        }
    }
}

pub enum Entry<'a, K: 'a, V: 'a, S: 'a = hash_map::RandomState> {
    /// An occupied Entry.
    Occupied(OccupiedEntry<'a, K, V, S>),
    /// A vacant Entry.
    Vacant(VacantEntry<'a, K, V, S>),
}

pub struct OccupiedEntry<'a, K: 'a, V: 'a, S: 'a = hash_map::RandomState> {
    entry: *mut Node<K, V>,
    map: *mut LinkedHashMap<K, V, S>,
    marker: marker::PhantomData<&'a K>,
}

pub struct VacantEntry<'a, K: 'a, V: 'a, S: 'a = hash_map::RandomState> {
    key: K,
    map: &'a mut LinkedHashMap<K, V, S>,
}

impl<'a, K: Hash + Eq, V, S: BuildHasher> Entry<'a, K, V, S> {
    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref e) => e.key(),
            Entry::Vacant(ref e) => e.key(),
        }
    }
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }
}

impl<'a, K: Hash + Eq, V, S: BuildHasher> OccupiedEntry<'a, K, V, S> {
    pub fn key(&self) -> &K {
        unsafe { &(*self.entry).key }
    }

    pub fn get(&self) -> &V {
        unsafe { &(*self.entry).value }
    }

    pub fn get_mut(&mut self) -> &mut V {
        unsafe { &mut (*self.entry).value }
    }

    pub fn into_mut(self) -> &'a mut V {
        unsafe { &mut (*self.entry).value }
    }

    pub fn insert(&mut self, value: V) -> V {
        unsafe {
            (*self.map).ensure_guard_node();

            let old_val = mem::replace(&mut (*self.entry).value, value);
            let node_ptr: *mut Node<K, V> = self.entry;

            (*self.map).detach(node_ptr);
            (*self.map).attach(node_ptr);

            old_val
        }
    }

    pub fn remove(self) -> V {
        unsafe { (*self.map).remove(&(*self.entry).key) }.unwrap()
    }
}

impl<'a, K: 'a + Hash + Eq, V: 'a, S: BuildHasher> VacantEntry<'a, K, V, S> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn insert(self, value: V) -> &'a mut V {
        self.map.ensure_guard_node();

        let node = if self.map.free.is_null() {
            Box::into_raw(Box::new(Node::new(self.key, value)))
        } else {
            // use a recycled box
            unsafe {
                let free = self.map.free;
                self.map.free = (*free).next;
                ptr::write(free, Node::new(self.key, value));
                free
            }
        };

        let keyref = unsafe { &(*node).key };

        self.map.attach(node);

        let ret = self.map.map.entry(KeyRef { k: keyref }).or_insert(node);
        unsafe { &mut (**ret).value }
    }
}
