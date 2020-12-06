def get_input():
    with open("../../inputs/day06.txt") as fd:
        return (x.strip() for x in fd.readlines())


def main():
    lines = get_input()

    groups = []
    current_group = set()

    for line in lines:
        if line == "":
            groups.append(current_group)
            current_group = set()
        else:
            for c in line:
                current_group.add(c)

    if current_group:
        groups.append(current_group)

    print(sum(len(s) for s in groups))


if __name__ == "__main__":
    main()
