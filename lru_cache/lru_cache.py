# Task: Implement an LRU cache
# Thinking process:
# What is LRU? It's Least Recently Used. Criteria of eviction from cache:
# 1. Expiration time -> if time expired, evict
# 2. Priority from 0, lower priority will first get evicted then higher
# 3. Least recently used -> evict item which was least recently used
# Naive, minimal plausible solution with future options to optimize.
# Implement Cache and Item:

import time
from typing import NamedTuple


class Item(NamedTuple):
    key: str
    value: object
    expires: int
    priority: int


class Cache:
    def __init__(self, max_size: int, cache_time=time.monotonic):
        self.max_size = max_size
        self.time = cache_time
        # Dict provides average O(1) search/insert/delete
        self.cache = {}
        self.expires = PriorityQueue()

    # Functions needed in cache:
    # get(key: String)
    # set(key: String, value: Object, max_age: Int, priority=0)
    # evict(now: time)
    def get(self, key: str):
        # Check if the key is in cache and not expired
        item = self.cache.get(key)
        if not item:
            return None
        if self.time() >= item.expires:
            return None

        return item.value

    def set(self, key: str, value: object, *, max_age=10, priority=0):
        now = self.time()

        # If the same element is in cache remove and insert again
        # Tuples (expires, key) compare lexicographically, it will be like comparing by expires alone, but with key along
        if key in self.cache:
            item = self.cache.pop(key)
            self.expires.remove((item.expires, key))
            del item
        # Evict if the max cache size is exceeded
        elif len(self.cache) >= self.max_size:
            self.evict(now)

        expires = int(now + max_age)

        self.cache[key] = Item(key, value, expires, priority)
        self.expires.insert((expires, key))

    # First point of LRU - Expiration time
    def evict(self, now):
        if not self.cache:
            return

        initial_size = len(self.cache)

        while self.cache:
            expires, key = self.expires.peek()

            if expires > now:
                break

            self.expires.pop()
            del self.cache[key]

        if len(self.cache) == initial_size:
            _, key = self.expires.pop()
            del self.cache[key]


# Need a data structure which will efficiently remove the smallest element
# https://en.wikipedia.org/wiki/Priority_queue
class PriorityQueue:
    def __init__(self):
        self.data = []

    # O(1)
    def is_empty(self):
        return len(self.data) == 0

    # O(1)
    def peek(self):
        return self.data[0]

    # O(n), shift all the items left by one position
    def pop(self):
        first_element = self.data[0]
        self.data[:1] = []
        return first_element

    # O(n)
    def remove(self, item: tuple):
        self.data.remove(item)

    # O(n log n), can be optimized to O(n) with Timsort for example
    def insert(self, item: tuple):
        self.data.append(item)
        self.data.sort()
