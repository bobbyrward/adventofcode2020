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

    return row * 8 + col


def main():
    max_seat = 0

    for boarding_pass in get_input():
        seat = find_seat(boarding_pass)

        print(seat)

        max_seat = max((seat, max_seat))

    print(max_seat)


if __name__ == "__main__":
    main()
