

def adder(x, y, z):
    print(f'{x} + {y} + {z} = {x+y+z}')


def intro(name, age):
    print(f'Hello {name}, you are {age} years old.')


def plus(*args):
    print(f'typeof args: {type(args)}')


def multi(**kwargs):
    print(f'typeof kwargs: {type(kwargs)}')
