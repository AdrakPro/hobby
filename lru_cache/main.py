from lru_cache import Cache


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


if __name__ == "__main__":
    test_basic()
