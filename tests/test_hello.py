import skook


def test_hello_default():
    assert skook.hello() == "Hello, world! skook is running."


def test_hello_name():
    assert skook.hello("Kurt") == "Hello, Kurt! skook is running."
