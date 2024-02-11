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
        if key in self.cache:
            self.cache.pop(key)
        # Evict if the max cache size is exceeded
        elif len(self.cache) >= self.max_size:
            self.evict(now)

        expires = int(now + max_age)

        self.cache[key] = Item(key, value, expires, priority)

    def evict(self, now):
        if not self.cache:
            return

        # The oldest key is the first (dicts preserve insertion order)
        key = next(iter(self.cache))
        del self.cache[key]
