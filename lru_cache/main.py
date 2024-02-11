import pytest

from lru_cache import Cache, PriorityQueue


# First write tests, later implementation

class FakeTime:
    def __init__(self, now=0):
        self.now = now

    def __call__(self):
        return self.now


def test_basic():
    cache = Cache(2, FakeTime())

    assert cache.get("a") is None

    cache.set("a", "A")
    assert cache.get("a") == "A"

    cache.set("b", "B")
    assert cache.get("a") == "A"
    assert cache.get("b") == "B"

    cache.set("c", "C")
    assert cache.get("a") is None
    assert cache.get("b") == "B"
    assert cache.get("c") == "C"


def test_priority_queue():
    pq = PriorityQueue()
    pq.insert(1)
    pq.insert(3)
    pq.insert(2)

    assert pq
    assert pq.peek() == 1
    assert pq.pop() == 1
    assert pq.peek() == 2

    assert pq.remove(3) is None

    assert pq
    assert pq.peek() == 2
    with pytest.raises(ValueError):
        pq.remove(3)

    assert pq.pop() == 2

    assert pq.is_empty()
    with pytest.raises(IndexError):
        pq.peek()
    with pytest.raises(IndexError):
        pq.pop()


def test_expires():
    cache = Cache(2, FakeTime())
    cache.set("a", "A", max_age=10)
    cache.set("b", "B", max_age=5)

    assert cache.get("a") == "A"
    assert cache.get("b") == "B"

    cache.time.now = 6

    assert cache.get("a") == "A"
    assert cache.get("b") is None

    cache.set("c", "C", max_age=15)
    assert cache.get("a") == "A"
    assert cache.get("b") is None
    assert cache.get("c") == "C"


def test_update_expires():
    cache = Cache(2, FakeTime())
    cache.set("a", "A", max_age=10)
    cache.set("b", "B", max_age=10)

    assert cache.get("a") == "A"
    assert cache.get("b") == "B"

    cache.time.now = 6

    cache.set("a", "X", max_age=4)
    cache.set("b", "Y", max_age=6)

    assert cache.get("a") == "X"
    assert cache.get("b") == "Y"

    cache.time.now = 10
    assert cache.get("a") is None
    assert cache.get("b") == "Y"


def test_priority():
    cache = Cache(2, FakeTime())

    cache.set('a', 'A', priority=1)
    cache.set('b', 'B', priority=0)
    assert cache.get('a') == 'A'
    assert cache.get('b') == 'B'

    cache.set('c', 'C')
    assert cache.get('a') == 'A'
    assert cache.get('b') == None
    assert cache.get('c') == 'C'

def test_update_priorities():
    cache = Cache(2, FakeTime())

    cache.set('a', 'A', priority=1)
    cache.set('b', 'B', priority=0)
    cache.set('b', 'Y', priority=2)

    cache.set('c', 'C')
    assert cache.get('a') == None
    assert cache.get('b') == 'Y'
    assert cache.get('c') == 'C'


if __name__ == "__main__":
    test_basic()
    test_expires()
    test_priority_queue()
    test_update_expires()
    test_priority()
    test_update_priorities()
