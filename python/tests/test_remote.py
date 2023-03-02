from arbeider import remote


def intro(name, age):
    print(f'Hello {name}, you are {age} years old.')


def plus(*args):
    print(f'typeof args: {type(args)}')
    print(f'{args}')


def multi(**kwargs):
    print(f'typeof kwargs: {type(kwargs)}')
    print(f'{kwargs}')


@remote
def adder(x, y, z):
    return x + y + z


def test_remote():
    adder(x=1, y=2, z=1)
    adder(1, 2, 3)
    adder(1, 2, z=3)
    # intro('John', 18)
    # plus(1, 2, 3, 4, 5)
    # multi(a=1, b=2, c=3, d=4, e=5)