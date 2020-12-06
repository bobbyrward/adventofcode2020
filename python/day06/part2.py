def get_input():
    with open("../../inputs/day06.txt") as fd:
        return (x.strip() for x in fd.readlines())


def main():
    lines = get_input()

    groups = []
    current_group = {}
    group_count = 0

    for line in lines:
        print(line)
        if line == "":
            count = 0

            for k, v in current_group.items():
                if v == group_count:
                    count += 1

            groups.append(count)

            current_group = {}
            group_count = 0
        else:
            group_count += 1
            for c in line:
                current_group[c] = current_group.setdefault(c, 0) + 1

    if current_group:
        count = 0

        for k, v in current_group.items():
            if v == group_count:
                count += 1

        groups.append(count)

        current_group = {}
        group_count = 0

    print(sum(groups))


if __name__ == "__main__":
    main()
