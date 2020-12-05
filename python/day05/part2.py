import itertools


def get_input():
    with open("../../inputs/day05.txt") as fd:
        return fd.readlines()


def high(range_min, range_max):
    size = range_max - range_min + 1
    return (range_min + (size // 2), range_max)


def low(range_min, range_max):
    size = range_max - range_min + 1
    return (range_min, range_max - (size // 2))


def do_the_splits(actions, range_min, range_max):
    for char in actions:
        if char == "F" or char == "L":
            (range_min, range_max) = low(range_min, range_max)
        elif char == "B" or char == "R":
            (range_min, range_max) = high(range_min, range_max)

        if range_min == range_max:
            break

    return range_min


def find_seat(boarding_pass):
    row = do_the_splits(boarding_pass[:7], 0, 127)
    col = do_the_splits(boarding_pass[7:], 0, 7)

    return (row, col)


def main():
    seats = [False] * (128 * 8)

    for boarding_pass in get_input():
        row, col = find_seat(boarding_pass)

        seats[row * 8 + col] = True

    # black likes this formatting.  I question it's taste
    idx, _ = next(itertools.dropwhile(lambda x: x[1], itertools.dropwhile(lambda x: not x[1], enumerate(seats))))

    row, col = (idx // 8, idx % 8)

    print(row * 8 + col)


if __name__ == "__main__":
    main()
